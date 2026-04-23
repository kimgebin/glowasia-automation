use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex;
use crate::browser::manager::{BrowserManager, Platform};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopifyConfig {
    pub shop_url: String,        // e.g., "glowasia-2.myshopify.com"
    pub access_token: Option<String>,  // API access token (from Dev Dashboard or client credentials)
    pub admin_api_key: Option<String>, // Client ID from Dev Dashboard
    pub admin_api_secret: Option<String>, // Client Secret from Dev Dashboard
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopifyOrder {
    pub order_id: i64,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
    pub total_price: String,
    pub currency: String,
    pub financial_status: String,
    pub fulfillment_status: Option<String>,
    pub status: String,
    pub line_items: Vec<ShopifyLineItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopifyLineItem {
    pub id: i64,
    pub product_id: i64,
    pub title: String,
    pub quantity: i32,
    pub price: String,
    pub sku: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TokenCache {
    token: Option<String>,
    expires_at: i64,
}

pub struct ShopifyBrowser {
    manager: Arc<Mutex<BrowserManager>>,
    token_cache: std::sync::Mutex<TokenCache>,
}

impl ShopifyBrowser {
    pub fn new(manager: Arc<Mutex<BrowserManager>>) -> Self {
        Self { 
            manager, 
            token_cache: std::sync::Mutex::new(TokenCache {
                token: None,
                expires_at: 0,
            }),
        }
    }
    
    pub fn is_configured(&self) -> bool {
        true
    }
    
    /// Get access token using client credentials grant from Dev Dashboard
    /// This exchanges client_id and client_secret for an access_token
    pub async fn get_access_token(&self, config: &ShopifyConfig) -> Result<String, String> {
        // Check cache first (refresh 60 seconds before expiry)
        {
            let cache = self.token_cache.lock().map_err(|e| e.to_string())?;
            if let Some(token) = &cache.token {
                if chrono::Utc::now().timestamp() < cache.expires_at - 60 {
                    return Ok(token.clone());
                }
            }
        }
        
        // Need client credentials for Dev Dashboard auth
        let client_id = config.admin_api_key.as_ref()
            .ok_or("No admin_api_key configured - need Client ID from Dev Dashboard")?;
        let client_secret = config.admin_api_secret.as_ref()
            .ok_or("No admin_api_secret configured - need Client Secret from Dev Dashboard")?;
        
        // Request token using client credentials grant
        let client = reqwest::Client::new();
        let url = format!("https://{}/admin/oauth/access_token", config.shop_url);
        
        let params = serde_json::json!({
            "grant_type": "client_credentials",
            "client_id": client_id,
            "client_secret": client_secret
        });
        
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&params)
            .send()
            .await
            .map_err(|e| format!("Token request failed: {}", e))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("Token request failed with {}: {}", status, text));
        }
        
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Parse token response failed: {}", e))?;
        
        let access_token = json["access_token"]
            .as_str()
            .ok_or("No access_token in response")?
            .to_string();
        
        let expires_in = json["expires_in"]
            .as_i64()
            .unwrap_or(86399);
        
        // Cache the token
        {
            let mut cache = self.token_cache.lock().map_err(|e| e.to_string())?;
            cache.token = Some(access_token.clone());
            cache.expires_at = chrono::Utc::now().timestamp() + expires_in;
        }
        
        Ok(access_token)
    }
    
    /// Get access token - uses provided token, falls back to Dev Dashboard credentials
    pub async fn get_token(&self, config: &ShopifyConfig) -> Result<String, String> {
        // If we have a direct access token, use it
        if let Some(token) = &config.access_token {
            return Ok(token.clone());
        }
        
        // Otherwise try client credentials grant
        self.get_access_token(config).await
    }
    
    /// Make authenticated API request to Shopify
    async fn api_request(
        &self,
        method: &str,
        endpoint: &str,
        config: &ShopifyConfig,
    ) -> Result<serde_json::Value, String> {
        let token = self.get_token(config).await?;
        
        let client = reqwest::Client::new();
        let base_url = format!("https://{}/admin/api/2024-01", config.shop_url);
        let url = format!("{}{}", base_url, endpoint);
        
        let response = match method.to_uppercase().as_str() {
            "GET" => {
                client.get(&url)
                    .header("X-Shopify-Access-Token", &token)
                    .header("Content-Type", "application/json")
                    .send()
                    .await
            },
            "POST" => {
                client.post(&url)
                    .header("X-Shopify-Access-Token", &token)
                    .header("Content-Type", "application/json")
                    .send()
                    .await
            },
            _ => return Err("Invalid HTTP method".to_string()),
        }.map_err(|e| format!("Request failed: {}", e))?;
        
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Parse response failed: {}", e))?;
        
        Ok(json)
    }
    
    /// Login to Shopify (browser-based for OAuth flow)
    pub async fn login(&self, shop_url: &str, email: &str, password: &str) -> Result<(), String> {
        let credentials = serde_json::json!({
            "shopUrl": shop_url,
            "email": email,
            "password": password
        });
        
        let manager = self.manager.lock().map_err(|e| e.to_string())?;
        manager.login(Platform::Shopify, credentials)
    }
    
    /// Get orders from Shopify
    pub async fn get_orders(&self, config: &ShopifyConfig, limit: i64) -> Result<Vec<ShopifyOrder>, String> {
        let response = self.api_request("GET", &format!("/orders.json?limit={}", limit), config).await?;
        
        let orders = response["orders"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|o| ShopifyOrder {
                        order_id: o["id"].as_i64().unwrap_or(0),
                        name: o["name"].as_str().unwrap_or("").to_string(),
                        email: o["email"].as_str().unwrap_or("").to_string(),
                        created_at: o["created_at"].as_str().unwrap_or("").to_string(),
                        updated_at: o["updated_at"].as_str().unwrap_or("").to_string(),
                        total_price: o["total_price"].as_str().unwrap_or("0").to_string(),
                        currency: o["currency"].as_str().unwrap_or("USD").to_string(),
                        financial_status: o["financial_status"].as_str().unwrap_or("").to_string(),
                        fulfillment_status: o["fulfillment_status"].as_str().map(|s| s.to_string()),
                        status: o["status"].as_str().unwrap_or("").to_string(),
                        line_items: o["line_items"]
                            .as_array()
                            .map(|items| {
                                items.iter()
                                    .map(|item| ShopifyLineItem {
                                        id: item["id"].as_i64().unwrap_or(0),
                                        product_id: item["product_id"].as_i64().unwrap_or(0),
                                        title: item["title"].as_str().unwrap_or("").to_string(),
                                        quantity: item["quantity"].as_i64().unwrap_or(0) as i32,
                                        price: item["price"].as_str().unwrap_or("0").to_string(),
                                        sku: item["sku"].as_str().map(|s| s.to_string()),
                                    })
                                    .collect()
                            })
                            .unwrap_or_default(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        
        Ok(orders)
    }
    
    /// Get unfulfilled orders
    pub async fn get_unfulfilled_orders(&self, config: &ShopifyConfig) -> Result<Vec<ShopifyOrder>, String> {
        let response = self.api_request(
            "GET", 
            "/orders.json?status=any&fulfillment_status=unfulfilled", 
            config
        ).await?;
        
        let orders = response["orders"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|o| ShopifyOrder {
                        order_id: o["id"].as_i64().unwrap_or(0),
                        name: o["name"].as_str().unwrap_or("").to_string(),
                        email: o["email"].as_str().unwrap_or("").to_string(),
                        created_at: o["created_at"].as_str().unwrap_or("").to_string(),
                        updated_at: o["updated_at"].as_str().unwrap_or("").to_string(),
                        total_price: o["total_price"].as_str().unwrap_or("0").to_string(),
                        currency: o["currency"].as_str().unwrap_or("USD").to_string(),
                        financial_status: o["financial_status"].as_str().unwrap_or("").to_string(),
                        fulfillment_status: o["fulfillment_status"].as_str().map(|s| s.to_string()),
                        status: o["status"].as_str().unwrap_or("").to_string(),
                        line_items: vec![],
                    })
                    .collect()
            })
            .unwrap_or_default();
        
        Ok(orders)
    }
    
    /// Update tracking information
    pub async fn update_tracking(
        &self,
        config: &ShopifyConfig,
        order_id: i64,
        tracking_number: &str,
        carrier: &str,
    ) -> Result<(), String> {
        let body = serde_json::json!({
            "fulfillment": {
                "tracking_number": tracking_number,
                "tracking_company": carrier,
                "notify_customer": true
            }
        });
        
        let token = self.get_token(config).await?;
        let client = reqwest::Client::new();
        let base_url = format!("https://{}/admin/api/2024-01", config.shop_url);
        let url = format!("{}/orders/{}/fulfillments.json", base_url, order_id);
        
        let response = client
            .post(&url)
            .header("X-Shopify-Access-Token", &token)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Tracking update failed: {}", e))?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            Err("Failed to update tracking".to_string())
        }
    }
    
    /// Get products
    pub async fn get_products(&self, config: &ShopifyConfig, limit: i64) -> Result<Vec<serde_json::Value>, String> {
        let response = self.api_request("GET", &format!("/products.json?limit={}", limit), config).await?;
        
        let products = response["products"]
            .as_array()
            .map(|arr| arr.to_owned())
            .unwrap_or_default();
        
        Ok(products)
    }
}