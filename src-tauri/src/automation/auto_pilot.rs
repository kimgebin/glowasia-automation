use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Instant;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::automation::notifier::TelegramNotifier;
use crate::storage::db;

static CREDENTIALS_DB: OnceLock<crate::cred_db::CredentialsDB> = OnceLock::new();

// ============ AUTO-PILOT ENGINE ============

pub struct AutoPilotEngine {
    // Config
    enabled: AtomicBool,
    order_polling_interval_sec: AtomicU64,
    inventory_sync_interval_sec: AtomicU64,
    health_check_interval_sec: AtomicU64,
    
    // Stats
    last_health_check: Mutex<Instant>,
    last_order_check: Mutex<Instant>,
    last_inventory_sync: Mutex<Instant>,
    last_daily_report: Mutex<Instant>,
    
    // Notifications
    telegram: Arc<TelegramNotifier>,
}

impl AutoPilotEngine {
    pub fn new(telegram: Arc<TelegramNotifier>) -> Self {
        Self {
            enabled: AtomicBool::new(false),
            order_polling_interval_sec: AtomicU64::new(300), // 5 min default
            inventory_sync_interval_sec: AtomicU64::new(3600), // 1 hour default
            health_check_interval_sec: AtomicU64::new(60), // 1 min default
            last_health_check: Mutex::new(Instant::now()),
            last_order_check: Mutex::new(Instant::now()),
            last_inventory_sync: Mutex::new(Instant::now()),
            last_daily_report: Mutex::new(Instant::now()),
            telegram,
        }
    }
    
    // ============ CONFIGURATION ============
    
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::SeqCst);
        log::info!("AutoPilot Engine enabled");
    }
    
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::SeqCst);
        log::info!("AutoPilot Engine disabled");
    }
    
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }
    
    pub fn set_order_polling_interval(&self, seconds: u64) {
        self.order_polling_interval_sec.store(seconds, Ordering::SeqCst);
        log::info!("Order polling interval set to {} seconds", seconds);
    }
    
    pub fn set_inventory_sync_interval(&self, seconds: u64) {
        self.inventory_sync_interval_sec.store(seconds, Ordering::SeqCst);
        log::info!("Inventory sync interval set to {} seconds", seconds);
    }
    
    pub fn set_health_check_interval(&self, seconds: u64) {
        self.health_check_interval_sec.store(seconds, Ordering::SeqCst);
        log::info!("Health check interval set to {} seconds", seconds);
    }
    
    // ============ HEALTH CHECK ============
    
    pub async fn health_check(&self) -> Result<HealthStatus, String> {
        let mut status = HealthStatus::default();
        status.timestamp = chrono::Utc::now().to_rfc3339();
        
        // Check database
        match db::get_recent_activity(1) {
            Ok(_) => status.database = "healthy".to_string(),
            Err(e) => status.database = format!("error: {}", e),
        }
        
        // Check credentials - use cred_db module
        match CREDENTIALS_DB.get() {
            Some(db) => {
                match db.list_platforms() {
                    Ok(platforms) => {
                        let mut connected = Vec::new();
                        for p in platforms {
                            if p.status == "active" {
                                connected.push(p.platform);
                            }
                        }
                        status.credentials_loaded = connected;
                        status.credentials_status = "ok".to_string();
                    }
                    Err(e) => status.credentials_status = format!("error: {}", e),
                }
            }
            None => status.credentials_status = "not_initialized".to_string(),
        }
        
        // Update last check time
        {
            let mut last = self.last_health_check.lock().unwrap();
            *last = Instant::now();
        }
        
        Ok(status)
    }
    
    // ============ DAILY REPORT ============
    
    pub async fn send_daily_report(&self) -> Result<(), String> {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        
        // Get today's stats
        let (order_count, revenue, processed, failed) = db::get_today_stats()
            .map_err(|e| e.to_string())?;
        
        // Get recent activity
        let recent = db::get_recent_activity(5).map_err(|e| e.to_string())?;
        
        let mut report = format!(
            "📊 *GLOWASIA Daily Report*\n\n📅 Date: {}\n\n",
            today
        );
        
        report.push_str(&format!(
            "🛒 Orders Today: {}\n💰 Revenue: Rp {:.0}\n✅ Processed: {}\n❌ Failed: {}\n\n",
            order_count, revenue, processed, failed
        ));
        
        if !recent.is_empty() {
            report.push_str("*Recent Activity:*\n");
            for activity in recent.iter().take(5) {
                report.push_str(&format!(
                    "• {}\n",
                    activity.action
                ));
            }
        }
        
        report.push_str("\n_Auto-piloted by GLOWASIA Copilot_");
        
        self.telegram.send_message(&report).await?;
        
        // Update last report time
        {
            let mut last = self.last_daily_report.lock().unwrap();
            *last = Instant::now();
        }
        
        Ok(())
    }
    
    // ============ GET STATUS ============
    
    pub fn get_status(&self) -> AutoPilotStatus {
        AutoPilotStatus {
            enabled: self.is_enabled(),
            order_polling_sec: self.order_polling_interval_sec.load(Ordering::SeqCst),
            inventory_sync_sec: self.inventory_sync_interval_sec.load(Ordering::SeqCst),
            health_check_sec: self.health_check_interval_sec.load(Ordering::SeqCst),
            last_health_check_secs: {
                let last = self.last_health_check.lock().unwrap();
                last.elapsed().as_secs()
            },
            last_order_check_secs: {
                let last = self.last_order_check.lock().unwrap();
                last.elapsed().as_secs()
            },
            last_inventory_sync_secs: {
                let last = self.last_inventory_sync.lock().unwrap();
                last.elapsed().as_secs()
            },
        }
    }
}

// ============ STATUS TYPES ============

#[derive(Debug, Default, serde::Serialize)]
pub struct HealthStatus {
    pub timestamp: String,
    pub database: String,
    pub credentials_status: String,
    pub credentials_loaded: Vec<String>,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct AutoPilotStatus {
    pub enabled: bool,
    pub order_polling_sec: u64,
    pub inventory_sync_sec: u64,
    pub health_check_sec: u64,
    pub last_health_check_secs: u64,
    pub last_order_check_secs: u64,
    pub last_inventory_sync_secs: u64,
}

// ============ ACTIVITY LOGGING ============

pub fn log_activity(action: &str, platform: &str, status: &str, details: Option<&str>) -> Result<(), String> {
    let full_action = format!("{} [{}] - {}", action, platform, status);
    db::log_activity(&full_action, details).map_err(|e| e.to_string())
}
