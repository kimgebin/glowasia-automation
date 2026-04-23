use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use crate::browser::manager::{BrowserManager, Platform};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TikTokConfig {
    pub app_key: String,       // App ID
    pub app_secret: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub shop_cipher: Option<String>,  // Unique shop identifier
    pub shop_id: String,
    pub country: String,      // ID, MY, TH, SG, VN, PH, US, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TikTokOrder {
    pub order_id: String,
    pub status: String,  // UNPAID, PAID, CONFIRMED, SHIPPED, DELIVERED, COMPLETED, CANCELLED, REFUNDED
    pub created_at: i64,
    pub updated_at: i64,
    pub total_amount: f64,
    pub currency: String,
    pub items: Vec<TikTokOrderItem>,
    pub shipping_address: TikTokShippingAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TikTokOrderItem {
    pub order_item_id: String,
    pub product_id: String,
    pub product_name: String,
    pub quantity: i32,
    pub price: f64,
    pub sku_info: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TikTokShippingAddress {
    pub recipient_name: String,
    pub phone_number: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

pub struct TikTokBrowser {
    manager: Arc<Mutex<BrowserManager>>,
}

impl TikTokBrowser {
    pub fn new(manager: Arc<Mutex<BrowserManager>>) -> Self {
        Self { manager }
    }
    
    pub fn is_configured(&self) -> bool {
        true
    }
    
    /// Get the API base URL based on country
    pub fn get_base_url(country: &str) -> &'static str {
        match country.to_uppercase().as_str() {
            "ID" => "https://open.tiktokglobalshop.com/api/v2",
            "MY" => "https://open.tiktokglobalshop.com/api/v2",
            "TH" => "https://open.tiktokglobalshop.com/api/v2",
            "SG" => "https://open.tiktokglobalshop.com/api/v2",
            "VN" => "https://open.tiktokglobalshop.com/api/v2",
            "PH" => "https://open.tiktokglobalshop.com/api/v2",
            "US" => "https://open.tiktokglobalshop.com/api/v2",
            _ => "https://open.tiktokglobalshop.com/api/v2",
        }
    }
    
    /// Generate signature for TikTok API
    fn generate_signature(params: &str, secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(params.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }
    
    /// Get access token using authorization code
    pub async fn get_access_token(&self, config: &TikTokConfig, auth_code: &str) -> Result<(), String> {
        let client = reqwest::Client::new();
        let url = format!("https://auth.tiktok.com/oauth2/token");
        
        let body = serde_json::json!({
            "app_key": config.app_key,
            "app_secret": config.app_secret,
            "auth_code": auth_code,
            "grant_type": "authorization_code"
        });
        
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Token request failed: {}", e))?;
        
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Parse token response failed: {}", e))?;
        
        // Update config with tokens
        let _token = json["access_token"].as_str();
        println!("Access token obtained");
        
        Ok(())
    }
    
    /// Refresh access token
    pub async fn refresh_access_token(&self, config: &TikTokConfig) -> Result<(), String> {
        let client = reqwest::Client::new();
        let url = format!("https://auth.tiktok.com/oauth2/token");
        
        let body = serde_json::json!({
            "app_key": config.app_key,
            "app_secret": config.app_secret,
            "refresh_token": config.refresh_token.as_ref().unwrap_or(&String::new()),
            "grant_type": "refresh_token"
        });
        
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Refresh token request failed: {}", e))?;
        
        let _json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Parse refresh response failed: {}", e))?;
        
        Ok(())
    }
    
    /// Make authenticated API request to TikTok Shop
    async fn api_request(
        &self,
        method: &str,
        endpoint: &str,
        config: &TikTokConfig,
        params: Option<HashMap<String, String>>,
    ) -> Result<serde_json::Value, String> {
        let access_token = config.access_token.as_ref()
            .ok_or("No access token - need to authorize first")?;
        let shop_cipher = config.shop_cipher.as_ref()
            .ok_or("No shop cipher - need to authorize first")?;
            
        let client = reqwest::Client::new();
        
        // Build URL with query params if GET
        let base = Self::get_base_url(&config.country);
        let url = if method == "GET" {
            let mut url_with_params = format!("{}{}", base, endpoint);
            if let Some(ref p) = params {
                let query_string = p.iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&");
                url_with_params = format!("{}?{}", url_with_params, query_string);
            }
            url_with_params
        } else {
            format!("{}{}", base, endpoint)
        };
        
        let response = match method.to_uppercase().as_str() {
            "GET" => {
                client.get(&url)
                    .header("Authorization", format!("Bearer {}", access_token))
                    .header("Content-Type", "application/json")
                    .header("Shop-Cipher", shop_cipher)
                    .send()
                    .await
            },
            "POST" => {
                let mut request = client.post(&url)
                    .header("Authorization", format!("Bearer {}", access_token))
                    .header("Content-Type", "application/json")
                    .header("Shop-Cipher", shop_cipher);
                
                if let Some(p) = params {
                    request = request.json(&p);
                }
                
                request.send().await
            },
            _ => return Err("Invalid HTTP method".to_string()),
        }.map_err(|e| format!("Request failed: {}", e))?;
        
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Parse response failed: {}", e))?;
        
        Ok(json)
    }
    
    /// Login to TikTok (browser-based for OAuth flow)
    pub async fn login(&self, email: &str, password: &str, country: &str) -> Result<(), String> {
        let credentials = serde_json::json!({
            "email": email,
            "password": password,
            "country": country
        });
        
        let manager = self.manager.lock().map_err(|e| e.to_string())?;
        manager.login(Platform::TikTok, credentials)
    }
    
    /// Get orders from TikTok Shop
    pub async fn get_orders(&self, config: &TikTokConfig, params: TikTokOrderParams) -> Result<Vec<TikTokOrder>, String> {
        let mut query_params = HashMap::new();
        query_params.insert("page_size".to_string(), params.page_size.unwrap_or(20).to_string());
        query_params.insert("page".to_string(), params.page.unwrap_or(1).to_string());
        
        if let Some(status) = params.status {
            query_params.insert("order_status".to_string(), status);
        }
        if let Some(create_time_from) = params.create_time_from {
            query_params.insert("create_time_from".to_string(), create_time_from.to_string());
        }
        if let Some(create_time_to) = params.create_time_to {
            query_params.insert("create_time_to".to_string(), create_time_to.to_string());
        }
        
        let response = self.api_request("GET", "/order/search", config, Some(query_params)).await?;
        
        let orders = response["data"]["orders"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|o| self.parse_order(o))
                    .collect()
            })
            .unwrap_or_default();
        
        Ok(orders)
    }
    
    /// Get order details
    pub async fn get_order_detail(&self, config: &TikTokConfig, order_id: &str) -> Result<TikTokOrder, String> {
        let mut params = HashMap::new();
        params.insert("order_id".to_string(), order_id.to_string());
        
        let response = self.api_request("GET", "/order/detail", config, Some(params)).await?;
        
        let order = self.parse_order(&response["data"]);
        Ok(order)
    }
    
    /// Parse order from JSON
    fn parse_order(&self, o: &serde_json::Value) -> TikTokOrder {
        TikTokOrder {
            order_id: o["order_id"].as_str().unwrap_or("").to_string(),
            status: o["status"].as_str().unwrap_or("").to_string(),
            created_at: o["create_time"].as_i64().unwrap_or(0),
            updated_at: o["update_time"].as_i64().unwrap_or(0),
            total_amount: o["total_amount"].as_f64().unwrap_or(0.0),
            currency: o["currency"].as_str().unwrap_or("USD").to_string(),
            items: o["items"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .map(|item| TikTokOrderItem {
                            order_item_id: item["order_item_id"].as_str().unwrap_or("").to_string(),
                            product_id: item["product_id"].as_str().unwrap_or("").to_string(),
                            product_name: item["product_name"].as_str().unwrap_or("").to_string(),
                            quantity: item["quantity"].as_i64().unwrap_or(0) as i32,
                            price: item["price"].as_f64().unwrap_or(0.0),
                            sku_info: item["sku_info"].as_str().unwrap_or("").to_string(),
                        })
                        .collect()
                })
                .unwrap_or_default(),
            shipping_address: TikTokShippingAddress {
                recipient_name: o["shipping_address"]["recipient_name"].as_str().unwrap_or("").to_string(),
                phone_number: o["shipping_address"]["phone_number"].as_str().unwrap_or("").to_string(),
                address_line1: o["shipping_address"]["address_line1"].as_str().unwrap_or("").to_string(),
                address_line2: o["shipping_address"]["address_line2"].as_str().map(|s| s.to_string()),
                city: o["shipping_address"]["city"].as_str().unwrap_or("").to_string(),
                state: o["shipping_address"]["state"].as_str().unwrap_or("").to_string(),
                postal_code: o["shipping_address"]["postal_code"].as_str().unwrap_or("").to_string(),
                country: o["shipping_address"]["country"].as_str().unwrap_or("").to_string(),
            },
        }
    }
    
    /// Update shipping information (Ready to Ship)
    pub async fn update_shipping(
        &self,
        config: &TikTokConfig,
        order_id: &str,
        tracking_number: &str,
        courier_id: &str,
    ) -> Result<(), String> {
        let mut params = HashMap::new();
        params.insert("order_id".to_string(), order_id.to_string());
        params.insert("tracking_number".to_string(), tracking_number.to_string());
        params.insert("courier_id".to_string(), courier_id.to_string());
        
        let response = self.api_request("POST", "/order/rts", config, Some(params)).await?;
        
        if response["code"].as_i64() == Some(0) {
            Ok(())
        } else {
            Err(response["message"].as_str().unwrap_or("Failed").to_string())
        }
    }
    
    /// Get shipment providers
    pub async fn get_shipment_providers(&self, config: &TikTokConfig) -> Result<Vec<TikTokShipmentProvider>, String> {
        let response = self.api_request("GET", "/logistics/get_shipping_providers", config, None).await?;
        
        let providers = response["data"]["shipping_providers"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|p| TikTokShipmentProvider {
                        id: p["courier_id"].as_str().unwrap_or("").to_string(),
                        name: p["courier_name"].as_str().unwrap_or("").to_string(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        
        Ok(providers)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TikTokOrderParams {
    pub page_size: Option<i64>,
    pub page: Option<i64>,
    pub status: Option<String>,  // UNPAID, PAID, CONFIRMED, SHIPPED, DELIVERED, COMPLETED, CANCELLED
    pub create_time_from: Option<i64>,
    pub create_time_to: Option<i64>,
}

impl Default for TikTokOrderParams {
    fn default() -> Self {
        Self {
            page_size: Some(20),
            page: Some(1),
            status: Some("COMPLETED".to_string()),
            create_time_from: None,
            create_time_to: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TikTokShipmentProvider {
    pub id: String,
    pub name: String,
}
