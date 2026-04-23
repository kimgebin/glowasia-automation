use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use crate::browser::manager::{BrowserManager, Platform};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LazadaConfig {
    pub api_key: String,
    pub api_secret: String,
    pub user_id: String,
    pub country: String,  // SINGAPORE, THAILAND, MALAYSIA, VIETNAM, PHILIPPINES, INDONESIA
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LazadaOrder {
    pub order_id: i64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub price: String,
    pub currency: String,
    pub items: Vec<LazadaOrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LazadaOrderItem {
    pub order_item_id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub quantity: i32,
    pub price: String,
}

pub struct LazadaBrowser {
    manager: Arc<Mutex<BrowserManager>>,
}

impl LazadaBrowser {
    pub fn new(manager: Arc<Mutex<BrowserManager>>) -> Self {
        Self { manager }
    }
    
    pub fn is_configured(&self) -> bool {
        true
    }
    
    /// Get the API gateway URL for a country
    pub fn get_gateway(country: &str) -> &'static str {
        match country.to_uppercase().as_str() {
            "SINGAPORE" => "api.lazada.sg/rest",
            "THAILAND" => "api.lazada.co.th/rest",
            "MALAYSIA" => "api.lazada.com.my/rest",
            "VIETNAM" => "api.lazada.vn/rest",
            "PHILIPPINES" => "api.lazada.com.ph/rest",
            "INDONESIA" => "api.lazada.co.id/rest",
            _ => "api.lazada.co.id/rest", // Default to Indonesia
        }
    }
    
    /// Generate signature for Lazada API request (HMAC-SHA256)
    fn generate_signature(params: &str, secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(params.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }
    
    /// Make authenticated API request to Lazada
    async fn api_request(
        &self,
        method: &str,
        endpoint: &str,
        mut params: HashMap<String, String>,
        config: &LazadaConfig,
    ) -> Result<serde_json::Value, String> {
        let gateway = Self::get_gateway(&config.country);
        let base_url = format!("https://{}", gateway);
        
        // Add required auth params
        params.insert("app_key".to_string(), config.api_key.clone());
        params.insert("access_token".to_string(), config.user_id.clone());
        params.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
        
        // Build sorted query string
        let mut sorted_params: Vec<(String, String)> = params.into_iter().collect();
        sorted_params.sort_by(|a, b| a.0.cmp(&b.0));
        
        let query_string = sorted_params
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        
        // Generate signature (use params without encoding for signing)
        let sign_string = sorted_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("");
        let signature = Self::generate_signature(&sign_string, &config.api_secret);
        
        // Full URL with signature
        let url = format!("{}/{}/?{}&sign={}", base_url, endpoint, query_string, signature);
        
        // Make HTTP request
        let client = reqwest::Client::new();
        
        let response = match method.to_uppercase().as_str() {
            "GET" => {
                client.get(&url)
                    .header("Content-Type", "application/json")
                    .send()
                    .await
            },
            "POST" => {
                client.post(&url)
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .send()
                    .await
            },
            _ => return Err("Invalid HTTP method".to_string()),
        }.map_err(|e| format!("Request failed: {}", e))?;
        
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        Ok(json)
    }
    
    /// Login to Lazada Seller Center (browser-based for OAuth flow)
    pub async fn login(&self, email: &str, password: &str, country: &str) -> Result<(), String> {
        let credentials = serde_json::json!({
            "email": email,
            "password": password,
            "country": country
        });
        
        let manager = self.manager.lock().map_err(|e| e.to_string())?;
        manager.login(Platform::Lazada, credentials)
    }
    
    /// Get orders from Lazada
    pub async fn get_orders(&self, config: &LazadaConfig, params: LazadaOrderParams) -> Result<Vec<LazadaOrder>, String> {
        let mut query_params = HashMap::new();
        
        if let Some(created_after) = params.created_after {
            query_params.insert("created_after".to_string(), created_after);
        }
        if let Some(updated_after) = params.updated_after {
            query_params.insert("updated_after".to_string(), updated_after);
        }
        if let Some(status) = params.status {
            query_params.insert("status".to_string(), status);
        }
        query_params.insert("offset".to_string(), params.offset.unwrap_or(0).to_string());
        query_params.insert("limit".to_string(), params.limit.unwrap_or(100).to_string());
        
        let response = self.api_request("GET", "/orders/get", query_params, config).await?;
        
        // Parse response
        let orders = response["data"]["orders"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|o| LazadaOrder {
                        order_id: o["order_id"].as_i64().unwrap_or(0),
                        status: o["status"].as_str().unwrap_or("").to_string(),
                        created_at: o["created_at"].as_str().unwrap_or("").to_string(),
                        updated_at: o["updated_at"].as_str().unwrap_or("").to_string(),
                        price: o["price"].as_str().unwrap_or("0").to_string(),
                        currency: o["currency"].as_str().unwrap_or("USD").to_string(),
                        items: vec![],
                    })
                    .collect()
            })
            .unwrap_or_default();
        
        Ok(orders)
    }
    
    /// Mark order as ready to ship
    pub async fn set_ready_to_ship(
        &self,
        config: &LazadaConfig,
        order_item_ids: Vec<i64>,
        shipment_provider: &str,
        tracking_number: &str,
    ) -> Result<(), String> {
        let mut params = HashMap::new();
        params.insert("delivery_type".to_string(), "dropship".to_string());
        params.insert("order_item_ids".to_string(), serde_json::to_string(&order_item_ids).unwrap());
        params.insert("shipment_provider".to_string(), shipment_provider.to_string());
        params.insert("tracking_number".to_string(), tracking_number.to_string());
        
        let response = self.api_request("POST", "/order/rts", params, config).await?;
        
        if response["code"].as_i64() == Some(0) {
            Ok(())
        } else {
            Err(response["message"].as_str().unwrap_or("Failed").to_string())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LazadaOrderParams {
    pub created_after: Option<String>,
    pub created_before: Option<String>,
    pub updated_after: Option<String>,
    pub updated_before: Option<String>,
    pub status: Option<String>,  // pending, canceled, ready_to_ship, delivered, returned, shipped, failed
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl Default for LazadaOrderParams {
    fn default() -> Self {
        Self {
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            status: None,
            sort_by: None,
            sort_direction: None,
            offset: Some(0),
            limit: Some(100),
        }
    }
}
