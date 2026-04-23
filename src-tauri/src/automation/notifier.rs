use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use std::sync::Mutex as StdMutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TelegramNotifier {
    bot_token: Arc<AsyncMutex<Option<String>>>,
    chat_id: Arc<AsyncMutex<Option<String>>>,
    configured: Arc<AtomicBool>,
    last_sent: Arc<StdMutex<u64>>, // Unix timestamp for rate limiting
}

impl TelegramNotifier {
    pub fn new() -> Self {
        Self {
            bot_token: Arc::new(AsyncMutex::new(None)),
            chat_id: Arc::new(AsyncMutex::new(None)),
            configured: Arc::new(AtomicBool::new(false)),
            last_sent: Arc::new(StdMutex::new(0)),
        }
    }
    
    pub fn configure(&self, bot_token: &str, chat_id: &str) {
        // Configure is called during setup, which may not have tokio runtime
        // We use try_lock to avoid blocking, and the value will be set properly
        if let Ok(mut bt) = self.bot_token.try_lock() {
            *bt = Some(bot_token.to_string());
        }
        if let Ok(mut ci) = self.chat_id.try_lock() {
            *ci = Some(chat_id.to_string());
        }
        self.configured.store(true, Ordering::SeqCst);
    }
    
    pub fn is_configured(&self) -> bool {
        self.configured.load(Ordering::SeqCst)
    }
    
    // Rate limit: minimum 10 seconds between messages to avoid spam
    fn can_send(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let mut last = self.last_sent.lock().unwrap();
        if now - *last >= 10 {
            *last = now;
            true
        } else {
            false
        }
    }
    
    pub async fn send_message(&self, message: &str) -> Result<(), String> {
        if !self.can_send() {
            log::debug!("Telegram rate limited, skipping message");
            return Ok(());
        }
        
        let bot_token = self.bot_token.lock().await;
        let chat_id = self.chat_id.lock().await;
        
        let token = bot_token.as_ref().ok_or("Bot token not set")?;
        let chat = chat_id.as_ref().ok_or("Chat ID not set")?;
        
        let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
        
        let client = reqwest::Client::new();
        let response = client.post(&url)
            .json(&serde_json::json!({
                "chat_id": chat,
                "text": message,
                "parse_mode": "Markdown"
            }))
            .send()
            .await
            .map_err(|e| format!("Failed to send message: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("Telegram API error: {}", response.status()));
        }
        
        log::info!("Telegram message sent: {}", message);
        Ok(())
    }
    
    // ============ NOTIFICATION METHODS ============
    
    /// 🔴 App crash/fatal error notification
    pub async fn send_crash_notification(&self, error: &str, context: &str) -> Result<(), String> {
        let message = format!(
            "🔴 *APP CRASH*\n\n❌ {}\n📍 Context: {}\n\nGLOWASIA Copilot needs attention!",
            error, context
        );
        self.send_message(&message).await
    }
    
    /// 🟠 Automation failed notification  
    pub async fn send_automation_failed(&self, action: &str, reason: &str, order_id: Option<&str>) -> Result<(), String> {
        let order_info = order_id.map_or(String::new(), |id| format!("\n📦 Order: `{}`", id));
        let message = format!(
            "🟠 *Automation Failed*\n\n❌ Action: {}\n📝 Reason: {}\n⏰ Time: <current>{} \n\nAuto-piloted by GLOWASIA Copilot",
            action, reason, order_info
        );
        self.send_message(&message).await
    }
    
    /// 🟡 Platform detection alert
    pub async fn send_detection_alert(&self, platform: &str, current_stealth: u8, new_stealth: u8) -> Result<(), String> {
        let message = format!(
            "🟡 *Detection Alert*\n\n⚠️ {} detected automation!\n🔒 Stealth: Level {} → Level {}\n\nAuto-escalating defense...",
            platform, current_stealth, new_stealth
        );
        self.send_message(&message).await
    }
    
    /// 🟢 New order notification
    pub async fn send_order_notification(&self, order_id: &str, customer: &str, platform: &str, amount: Option<&str>) -> Result<(), String> {
        let amount_info = amount.map_or(String::new(), |a| format!("\n💰 Amount: {}", a));
        let message = format!(
            "🟢 *New Order*\n\n📦 Order: `{}`\n👤 Customer: {}\n📍 Platform: {} \n{} \n\nAuto-piloted by GLOWASIA Copilot",
            order_id, customer, platform, amount_info
        );
        self.send_message(&message).await
    }
    
    /// 🔵 Order shipped notification
    pub async fn send_shipped_notification(&self, order_id: &str, tracking: &str, courier: Option<&str>) -> Result<(), String> {
        let courier_info = courier.map_or(String::new(), |c| format!("\n🚚 Courier: {}", c));
        let message = format!(
            "🔵 *Order Shipped*\n\n📦 Order: `{}`\n🔢 Tracking: `{}` \n{} \n\nAuto-piloted by GLOWASIA Copilot",
            order_id, tracking, courier_info
        );
        self.send_message(&message).await
    }
    
    /// 🟣 Update available notification
    pub async fn send_update_notification(&self, version: &str, notes: &str) -> Result<(), String> {
        let message = format!(
            "🟣 *Update Available*\n\n📦 Version `{}` ready\n📝 {}\n\nUse /update to install",
            version, notes
        );
        self.send_message(&message).await
    }
}

impl Default for TelegramNotifier {
    fn default() -> Self {
        Self::new()
    }
}
