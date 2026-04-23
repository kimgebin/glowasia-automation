use std::sync::Arc;
use std::sync::Mutex;
use crate::browser::manager::{BrowserManager, Platform};

pub struct CjBrowser {
    manager: Arc<Mutex<BrowserManager>>,
}

impl CjBrowser {
    pub fn new(manager: Arc<Mutex<BrowserManager>>) -> Self {
        Self { manager }
    }
    
    pub fn is_configured(&self) -> bool {
        true
    }
    
    pub async fn login(&self, email: &str, password: &str) -> Result<(), String> {
        let credentials = serde_json::json!({
            "email": email,
            "password": password
        });
        
        let manager = self.manager.lock().map_err(|e| e.to_string())?;
        manager.login(Platform::CJ, credentials)
    }
    
    pub async fn create_order(&self, order_data: &CJOrderData) -> Result<String, String> {
        let order_json = serde_json::json!({
            "productUrl": order_data.product_url,
            "quantity": order_data.quantity,
            "customerName": order_data.customer_name,
            "customerPhone": order_data.customer_phone,
            "customerAddress": order_data.customer_address,
            "customerZip": order_data.customer_zip,
            "customerCountry": order_data.customer_country,
            "customerCity": order_data.customer_city,
            "customerState": order_data.customer_state,
        });
        
        let manager = self.manager.lock().map_err(|e| e.to_string())?;
        manager.create_cj_order(order_json)
    }
    
    pub async fn get_orders(&self) -> Result<Vec<crate::storage::db::Order>, String> {
        let manager = self.manager.lock().map_err(|e| e.to_string())?;
        let orders_json = manager.get_orders(Platform::CJ)?;
        
        let orders: Vec<crate::storage::db::Order> = orders_json
            .as_array()
            .map(|arr| arr.iter().map(|v| crate::storage::db::Order::from_json(v)).collect())
            .unwrap_or_default();
        
        Ok(orders)
    }
}

#[derive(Debug, Clone)]
pub struct CJOrderData {
    pub product_url: String,
    pub quantity: u32,
    pub customer_name: String,
    pub customer_address: String,
    pub customer_phone: String,
    pub customer_zip: String,
    pub customer_country: String,
    pub customer_city: String,
    pub customer_state: String,
}
