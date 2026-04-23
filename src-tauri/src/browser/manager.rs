use std::collections::HashMap;
use std::process::Command;
use std::sync::Mutex;
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Shopify,
    Shopee,
    Lazada,
    Tokopedia,
    TikTok,
    Etsy,
    CJ,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Shopify => write!(f, "Shopify"),
            Platform::Shopee => write!(f, "Shopee"),
            Platform::Lazada => write!(f, "Lazada"),
            Platform::Tokopedia => write!(f, "Tokopedia"),
            Platform::TikTok => write!(f, "TikTok"),
            Platform::Etsy => write!(f, "Etsy"),
            Platform::CJ => write!(f, "CJ"),
        }
    }
}

impl Platform {
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Shopify => "shopify",
            Platform::Shopee => "shopee",
            Platform::Lazada => "lazada",
            Platform::Tokopedia => "tokopedia",
            Platform::TikTok => "tiktok",
            Platform::Etsy => "etsy",
            Platform::CJ => "cj",
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoginStatus {
    pub logged_in: bool,
    pub cookies: Option<String>,
}

pub struct BrowserManager {
    statuses: Mutex<HashMap<Platform, LoginStatus>>,
    scripts_dir: String,
}

impl BrowserManager {
    pub fn new() -> Self {
        Self {
            statuses: Mutex::new(HashMap::new()),
            scripts_dir: std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
        }
    }
    
    fn run_script(&self, platform: Platform, action: &str, args: Value) -> Result<Value, String> {
        let script_path = format!("{}/scripts/playwright-runner.js", self.scripts_dir);
        
        let args_json = serde_json::to_string(&args).map_err(|e| e.to_string())?;
        
        let output = Command::new("node")
            .arg(&script_path)
            .arg(platform.as_str())
            .arg(action)
            .arg(&args_json)
            .output()
            .map_err(|e| format!("Failed to run script: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Script failed: {}", stderr));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: Value = serde_json::from_str(&stdout)
            .map_err(|e| format!("Failed to parse result: {} - stdout: {}", e, stdout))?;
        
        if result.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
            Ok(result)
        } else {
            Err(result.get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error")
                .to_string())
        }
    }
    
    pub fn login(&self, platform: Platform, credentials: Value) -> Result<(), String> {
        let result = self.run_script(platform, "login", credentials)?;
        
        if result.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
            let mut statuses = self.statuses.lock().map_err(|e| e.to_string())?;
            statuses.insert(platform, LoginStatus { logged_in: true, cookies: None });
            log::info!("{} login successful", platform);
            Ok(())
        } else {
            Err(result.get("error").and_then(|v| v.as_str()).unwrap_or("Login failed").to_string())
        }
    }
    
    pub fn is_logged_in(&self, platform: Platform) -> bool {
        self.statuses.lock()
            .map(|s| s.get(&platform).map(|st| st.logged_in).unwrap_or(false))
            .unwrap_or(false)
    }
    
    pub fn get_orders(&self, platform: Platform) -> Result<Value, String> {
        if !self.is_logged_in(platform) {
            return Err(format!("{} not logged in", platform));
        }
        
        self.run_script(platform, "getOrders", serde_json::json!({}))
            .map(|r| r.get("orders").cloned().unwrap_or(Value::Array(vec![])))
    }
    
    pub fn create_cj_order(&self, order_data: Value) -> Result<String, String> {
        if !self.is_logged_in(Platform::CJ) {
            return Err("CJ not logged in".to_string());
        }
        
        let result = self.run_script(Platform::CJ, "createOrder", order_data)?;
        
        result.get("tracking")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "No tracking number returned".to_string())
    }
    
    pub fn update_tracking(&self, platform: Platform, order_id: &str, tracking: &str, _carrier: &str) -> Result<(), String> {
        if platform == Platform::Shopify {
            let args = serde_json::json!({
                "shopUrl": "",
                "orderId": order_id,
                "trackingNumber": tracking,
                "carrier": _carrier
            });
            self.run_script(platform, "updateTracking", args)?;
        }
        Ok(())
    }
    
    pub fn logout(&self, platform: Platform) {
        let mut statuses = self.statuses.lock().unwrap();
        statuses.remove(&platform);
        log::info!("{} logged out", platform);
    }
    
    pub fn logout_all(&self) {
        let mut statuses = self.statuses.lock().unwrap();
        statuses.clear();
    }
}
