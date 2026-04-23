use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use crate::browser::manager::{BrowserManager, Platform};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopeeConfig {
    pub partner_id: i64,
    pub partner_key: String,
    pub shop_id: i64,
    pub access_token: String,
    pub country: String,  // SINGAPORE, MALAYSIA, THAILAND, INDONESIA, PHILIPPINES, VIETNAM, TAIWAN
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopeeOrder {
    pub order_id: i64,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub total_amount: f64,
    pub currency: String,
    pub items: Vec<ShopeeOrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopeeOrderItem {
    pub order_item_id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub quantity: i32,
    pub price: f64,
}

pub struct ShopeeBrowser {
    manager: Arc<Mutex<BrowserManager>>,
}

impl ShopeeBrowser {
    pub fn new(manager: Arc<Mutex<BrowserManager>>) -> Self {
        Self { manager }
    }
    
    pub fn is_configured(&self) -> bool {
        true
    }
    
    /// Get the API base URL for a country
    pub fn get_base_url(country: &str) -> &'static str {
        match country.to_uppercase().as_str() {
            "SINGAPORE" => "https://partner.shopeemobile.com/api/v1",
            "MALAYSIA" => "https://partner.shopeemobile.com/api/v1",
            "THAILAND" => "https://partner.shopeemobile.com/api/v1",
            "INDONESIA" => "https://partner.shopeemobile.com/api/v1",
            "PHILIPPINES" => "https://partner.shopeemobile.com/api/v1",
            "VIETNAM" => "https://partner.shopeemobile.com/api/v1",
            "TAIWAN" => "https://partner.shopeemobile.com/api/v1",
            _ => "https://partner.shopeemobile.com/api/v1",
        }
    }
    
    /// Generate signature for Shopee API request
    /// Base string: partner_id + api_path + timestamp + access_token + shop_id + partner_key
    fn generate_signature(partner_id: i64, api_path: &str, timestamp: i64, access_token: &str, shop_id: i64, partner_key: &str) -> String {
        let sign_string = format!(
            "{}{}{}{}{}",
            partner_id,
            api_path,
            timestamp,
            access_token,
            shop_id
        );
        
        let mut mac = HmacSha256::new_from_slice(partner_key.as_bytes()).unwrap();
        mac.update(sign_string.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }
    
    /// Make authenticated API request to Shopee
    async fn api_request(
        &self,
        method: &str,
        endpoint: &str,
        params: HashMap<String, serde_json::Value>,
        config: &ShopeeConfig,
    ) -> Result<serde_json::Value, String> {
        let base_url = Self::get_base_url(&config.country);
        let api_path = endpoint;
        let timestamp = chrono::Utc::now().timestamp();
        
        // Build sorted params string
        let mut sorted_params: Vec<(String, serde_json::Value)> = params.into_iter().collect();
        sorted_params.sort_by(|a, b| a.0.cmp(&b.0));
        
        // Convert params to JSON string
        let _params_json = serde_json::to_string(&sorted_params).map_err(|e| e.to_string())?;
        
        // Generate signature
        let signature = Self::generate_signature(
            config.partner_id,
            api_path,
            timestamp,
            &config.access_token,
            config.shop_id,
            &config.partner_key
        );
        
        // Build request body
        let mut body = serde_json::json!({
            "partner_id": config.partner_id,
            "shop_id": config.shop_id,
            "timestamp": timestamp,
            "access_token": config.access_token,
            "sign": signature,
        });
        
        // Add params
        for (key, value) in sorted_params {
            body[key] = value;
        }
        
        // Make HTTP request
        let client = reqwest::Client::new();
        let url = format!("{}{}", base_url, endpoint);
        
        let response = match method.to_uppercase().as_str() {
            "GET" => {
                client.get(&url)
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                    .await
            },
            "POST" => {
                client.post(&url)
                    .header("Content-Type", "application/json")
                    .json(&body)
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
    
    /// Login to Shopee (browser-based for OAuth flow)
    pub async fn login(&self, email: &str, password: &str, country: &str) -> Result<(), String> {
        let credentials = serde_json::json!({
            "email": email,
            "password": password,
            "country": country
        });
        
        let manager = self.manager.lock().map_err(|e| e.to_string())?;
        manager.login(Platform::Shopee, credentials)
    }
    
    /// Get orders from Shopee
    pub async fn get_orders(&self, config: &ShopeeConfig, params: ShopeeOrderParams) -> Result<Vec<ShopeeOrder>, String> {
        let mut query_params = HashMap::new();
        
        if let Some(orders_status) = params.orders_status {
            query_params.insert("orders_status".to_string(), serde_json::json!(orders_status));
        }
        if let Some(create_time_from) = params.create_time_from {
            query_params.insert("create_time_from".to_string(), serde_json::json!(create_time_from));
        }
        if let Some(create_time_to) = params.create_time_to {
            query_params.insert("create_time_to".to_string(), serde_json::json!(create_time_to));
        }
        query_params.insert("page_size".to_string(), serde_json::json!(params.page_size.unwrap_or(100)));
        query_params.insert("cursor".to_string(), serde_json::json!(params.cursor.unwrap_or(0)));
        
        let response = self.api_request("POST", "/order/get_orders", query_params, config).await?;
        
        // Parse response
        let orders = response["data"]["order_list"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|o| ShopeeOrder {
                        order_id: o["order_id"].as_i64().unwrap_or(0),
                        status: o["status"].as_str().unwrap_or("").to_string(),
                        created_at: o["create_time"].as_i64().unwrap_or(0),
                        updated_at: o["update_time"].as_i64().unwrap_or(0),
                        total_amount: o["total_amount"].as_f64().unwrap_or(0.0),
                        currency: o["currency"].as_str().unwrap_or("USD").to_string(),
                        items: vec![],
                    })
                    .collect()
            })
            .unwrap_or_default();
        
        Ok(orders)
    }
    
    /// Get order details
    pub async fn get_order_detail(&self, config: &ShopeeConfig, order_id: i64) -> Result<ShopeeOrder, String> {
        let mut params = HashMap::new();
        params.insert("order_id".to_string(), serde_json::json!(order_id));
        
        let response = self.api_request("POST", "/order/get_order_detail", params, config).await?;
        
        let order_data = &response["data"];
        let order = ShopeeOrder {
            order_id: order_data["order_id"].as_i64().unwrap_or(0),
            status: order_data["status"].as_str().unwrap_or("").to_string(),
            created_at: order_data["create_time"].as_i64().unwrap_or(0),
            updated_at: order_data["update_time"].as_i64().unwrap_or(0),
            total_amount: order_data["total_amount"].as_f64().unwrap_or(0.0),
            currency: order_data["currency"].as_str().unwrap_or("USD").to_string(),
            items: order_data["item_list"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .map(|item| ShopeeOrderItem {
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
    
    /// Set order shipping information (Ready to Ship)
    pub async fn set_rts(
        &self,
        config: &ShopeeConfig,
        order_id: i64,
        package_list: Vec<ShopeePackage>,
    ) -> Result<(), String> {
        let mut params = HashMap::new();
        params.insert("order_id".to_string(), serde_json::json!(order_id));
        params.insert("package_list".to_string(), serde_json::json!(package_list));
        
        let response = self.api_request("POST", "/order/rts", params, config).await?;
        
        if response["error"].is_null() || response["error"].as_i64() == Some(0) {
            Ok(())
        } else {
            Err(response["message"].as_str().unwrap_or("Failed").to_string())
        }
    }
    
    /// Get shipment providers
    pub async fn get_shipment_providers(&self, config: &ShopeeConfig) -> Result<Vec<ShipmentProvider>, String> {
        let response = self.api_request("POST", "/logistics/get_shipping_carrier_list", HashMap::new(), config).await?;
        
        let providers = response["data"]["shipping_carrier"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|p| ShipmentProvider {
                        id: p["id"].as_i64().unwrap_or(0),
                        name: p["name"].as_str().unwrap_or("").to_string(),
                        enabled: p["enabled"].as_bool().unwrap_or(false),
                    })
                    .collect()
            })
            .unwrap_or_default();
        
        Ok(providers)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopeeOrderParams {
    pub orders_status: Option<String>,  // BROKEN, CANCELLED, CANCELLING, CLOSED, COMPLETED, DISPUTED, DOING, INIT, MODIFIED, PAID, PENDING, READY_TO_SHIP, PROCESSING, TO_CONFIRM, TO_RETURN, UNPAID, WAITING_FOR_PAYMENT
    pub create_time_from: Option<i64>,
    pub create_time_to: Option<i64>,
    pub page_size: Option<i64>,
    pub cursor: Option<i64>,
}

impl Default for ShopeeOrderParams {
    fn default() -> Self {
        Self {
            orders_status: Some("COMPLETED".to_string()),
            create_time_from: None,
            create_time_to: None,
            page_size: Some(100),
            cursor: Some(0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopeePackage {
    pub package_id: Option<String>,
    pub shipment_provider: String,
    pub tracking_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentProvider {
    pub id: i64,
    pub name: String,
    pub enabled: bool,
}
