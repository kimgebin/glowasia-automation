use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use crate::browser::manager::{BrowserManager, Platform};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokopediaConfig {
    pub app_id: String,        // formerly fs_id
    pub client_id: String,
    pub client_secret: String,
    pub access_token: Option<String>,
    pub shop_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokopediaOrder {
    pub order_id: i64,
    pub order_status: String,
    pub created_at: String,
    pub updated_at: String,
    pub total_amount: f64,
    pub currency: String,
    pub items: Vec<TokopediaOrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokopediaOrderItem {
    pub order_item_id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub quantity: i32,
    pub price: f64,
}

pub struct TokopediaBrowser {
    manager: Arc<Mutex<BrowserManager>>,
}

impl TokopediaBrowser {
    pub fn new(manager: Arc<Mutex<BrowserManager>>) -> Self {
        Self { manager }
    }
    
    pub fn is_configured(&self) -> bool {
        true
    }
    
    /// Get the API base URL
    pub fn get_base_url() -> &'static str {
        "https://fs.tokopedia.net/v1/fs"
    }
    
    /// Get authentication token using client credentials
    pub async fn get_access_token(&self, config: &TokopediaConfig) -> Result<String, String> {
        let auth_string = format!("{}:{}", config.client_id, config.client_secret);
        let auth_base64 = base64::encode(auth_string.as_bytes());
        
        let client = reqwest::Client::new();
        let url = "https://accounts.tokopedia.com/token?grant_type=client_credentials";
        
        let response = client
            .post(url)
            .header("Authorization", format!("Basic {}", auth_base64))
            .header("Content-Length", "0")
            .send()
            .await
            .map_err(|e| format!("Auth request failed: {}", e))?;
        
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Parse auth response failed: {}", e))?;
        
        json["access_token"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "No access_token in response".to_string())
    }
    
    /// Make authenticated API request to Tokopedia
    async fn api_request(
        &self,
        method: &str,
        endpoint: &str,
        access_token: &str,
        app_id: &str,
    ) -> Result<serde_json::Value, String> {
        let client = reqwest::Client::new();
        let url = format!("https://fs.tokopedia.net/v1/fs/{}{}", app_id, endpoint);
        
        let response = match method.to_uppercase().as_str() {
            "GET" => {
                client.get(&url)
                    .header("Authorization", format!("Bearer {}", access_token))
                    .send()
                    .await
            },
            "POST" => {
                client.post(&url)
                    .header("Authorization", format!("Bearer {}", access_token))
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
    
    /// Login to Tokopedia (browser-based for OAuth flow)
    pub async fn login(&self, email: &str, password: &str, _country: &str) -> Result<(), String> {
        let credentials = serde_json::json!({
            "email": email,
            "password": password
        });
        
        let manager = self.manager.lock().map_err(|e| e.to_string())?;
        manager.login(Platform::Tokopedia, credentials)
    }
    
    /// Get orders from Tokopedia
    pub async fn get_orders(&self, config: &TokopediaConfig, params: TokopediaOrderParams) -> Result<Vec<TokopediaOrder>, String> {
        let access_token = match &config.access_token {
            Some(token) => token.clone(),
            None => self.get_access_token(config).await?,
        };
        
        // Build query string
        let mut query_parts = vec![];
        if let Some(page_size) = params.page_size {
            query_parts.push(format!("page_size={}", page_size));
        }
        if let Some(page) = params.page {
            query_parts.push(format!("page={}", page));
        }
        if let Some(eta_start) = params.eta_start {
            query_parts.push(format!("eta_start={}", eta_start));
        }
        if let Some(eta_end) = params.eta_end {
            query_parts.push(format!("eta_end={}", eta_end));
        }
        
        let query_string = if query_parts.is_empty() {
            String::new()
        } else {
            format!("?{}", query_parts.join("&"))
        };
        
        let endpoint = format!("/order/{}", query_string);
        let response = self.api_request("GET", &endpoint, &access_token, &config.app_id).await?;
        
        // Parse response
        let orders = response["data"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|o| TokopediaOrder {
                        order_id: o["order_id"].as_i64().unwrap_or(0),
                        order_status: o["status"].as_str().unwrap_or("").to_string(),
                        created_at: o["create_time"].as_str().unwrap_or("").to_string(),
                        updated_at: o["update_time"].as_str().unwrap_or("").to_string(),
                        total_amount: o["total_amount"].as_f64().unwrap_or(0.0),
                        currency: o["currency"].as_str().unwrap_or("IDR").to_string(),
                        items: vec![],
                    })
                    .collect()
            })
            .unwrap_or_default();
        
        Ok(orders)
    }
    
    /// Get order details
    pub async fn get_order_detail(&self, config: &TokopediaConfig, order_id: i64) -> Result<TokopediaOrder, String> {
        let access_token = match &config.access_token {
            Some(token) => token.clone(),
            None => self.get_access_token(config).await?,
        };
        
        let endpoint = format!("/order/{}/detail", order_id);
        let response = self.api_request("GET", &endpoint, &access_token, &config.app_id).await?;
        
        let order_data = &response["data"];
        let order = TokopediaOrder {
            order_id: order_data["order_id"].as_i64().unwrap_or(0),
            order_status: order_data["status"].as_str().unwrap_or("").to_string(),
            created_at: order_data["create_time"].as_str().unwrap_or("").to_string(),
            updated_at: order_data["update_time"].as_str().unwrap_or("").to_string(),
            total_amount: order_data["total_amount"].as_f64().unwrap_or(0.0),
            currency: order_data["currency"].as_str().unwrap_or("IDR").to_string(),
            items: order_data["items"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .map(|item| TokopediaOrderItem {
                            order_item_id: item["order_item_id"].as_i64().unwrap_or(0),
                            product_id: item["product_id"].as_i64().unwrap_or(0),
                            product_name: item["product_name"].as_str().unwrap_or("").to_string(),
                            quantity: item["quantity"].as_i64().unwrap_or(0) as i32,
                            price: item["price"].as_f64().unwrap_or(0.0),
                        })
                        .collect()
                })
                .unwrap_or_default(),
        };
        
        Ok(order)
    }
    
    /// Update order status (Ready to Ship)
    pub async fn set_rts(&self, config: &TokopediaConfig, order_id: i64, courier_id: i64, tracking_number: &str) -> Result<(), String> {
        let access_token = match &config.access_token {
            Some(token) => token.clone(),
            None => self.get_access_token(config).await?,
        };
        
        let body = serde_json::json!({
            "order_id": order_id,
            "courier_id": courier_id,
            "tracking_number": tracking_number
        });
        
        let client = reqwest::Client::new();
        let url = format!("https://fs.tokopedia.net/v1/fs/{}/order/rts", config.app_id);
        
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("RTS request failed: {}", e))?;
        
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Parse RTS response failed: {}", e))?;
        
        if json["status"] == "200 Ok" || json["status"] == "200" {
            Ok(())
        } else {
            Err(json["error_message"].as_str().unwrap_or("Failed").to_string())
        }
    }
    
    /// Get shipment providers
    pub async fn get_shipment_providers(&self, config: &TokopediaConfig) -> Result<Vec<TokopediaShipmentProvider>, String> {
        let access_token = match &config.access_token {
            Some(token) => token.clone(),
            None => self.get_access_token(config).await?,
        };
        
        let response = self.api_request("GET", "/logistics", &access_token, &config.app_id).await?;
        
        let providers = response["data"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|p| TokopediaShipmentProvider {
                        id: p["courier_id"].as_i64().unwrap_or(0),
                        name: p["courier_name"].as_str().unwrap_or("").to_string(),
                        enabled: p["enabled"].as_bool().unwrap_or(false),
                    })
                    .collect()
            })
            .unwrap_or_default();
        
        Ok(providers)
    }
    
    /// Get shop info
    pub async fn get_shop_info(&self, config: &TokopediaConfig) -> Result<serde_json::Value, String> {
        let access_token = match &config.access_token {
            Some(token) => token.clone(),
            None => self.get_access_token(config).await?,
        };
        
        self.api_request("GET", &format!("/{}", config.app_id), &access_token, &config.app_id).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokopediaOrderParams {
    pub page_size: Option<i64>,
    pub page: Option<i64>,
    pub eta_start: Option<String>,
    pub eta_end: Option<String>,
}

impl Default for TokopediaOrderParams {
    fn default() -> Self {
        Self {
            page_size: Some(20),
            page: Some(1),
            eta_start: None,
            eta_end: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokopediaShipmentProvider {
    pub id: i64,
    pub name: String,
    pub enabled: bool,
}
