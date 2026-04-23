pub mod browser;
pub mod automation;
pub mod storage;
pub mod update;
pub mod cred_db;
pub mod commands;

use std::sync::Arc;
use tauri::{Manager, State};
use serde::{Deserialize, Serialize};
use crate::cred_db::CredentialsDB;

fn init_credentials_db(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let db = CredentialsDB::new(app_handle).map_err(|e| e.to_string())?;
    CREDENTIALS_DB.set(db).map_err(|_| "Already initialized".to_string())
}

#[allow(dead_code)]
fn load_telegram_from_db(state: &AppState) {
    if let Ok(Some(cred)) = crate::commands::load_credential("telegram".to_string()) {
        // Telegram stores: api_key = bot_token, api_secret = chat_id
        if let (Some(bot_token), Some(chat_id)) = (cred.api_key, cred.api_secret) {
            if !bot_token.is_empty() && !chat_id.is_empty() {
                state.telegram.configure(&bot_token, &chat_id);
            }
        }
    }
}

use crate::browser::manager::BrowserManager;
use crate::browser::shopify::ShopifyBrowser;
use crate::browser::shopee::ShopeeBrowser;
use crate::browser::lazada::LazadaBrowser;
use crate::browser::tokopedia::TokopediaBrowser;
use crate::browser::tiktok::TikTokBrowser;
use crate::browser::cj::CjBrowser;
use crate::automation::monitor::AutomationMonitor;
use crate::automation::notifier::TelegramNotifier;
use crate::automation::fulfillment::FulfillmentEngine;
use crate::automation::auto_pilot::{AutoPilotEngine, AutoPilotStatus, HealthStatus};
use crate::storage::db::{Order, ActivityEntry, self};
use crate::storage::credentials as enc;

pub struct AppState {
    pub shopify: Arc<ShopifyBrowser>,
    pub shopee: Arc<ShopeeBrowser>,
    pub lazada: Arc<LazadaBrowser>,
    pub tokopedia: Arc<TokopediaBrowser>,
    pub tiktok: Arc<TikTokBrowser>,
    pub cj: Arc<CjBrowser>,
    pub monitor: Arc<AutomationMonitor>,
    pub telegram: Arc<TelegramNotifier>,
    pub fulfillment: Arc<FulfillmentEngine>,
    pub auto_pilot: Arc<AutoPilotEngine>,
}

impl AppState {
    pub fn new() -> Self {
        let browser_manager = Arc::new(std::sync::Mutex::new(
            BrowserManager::new()
        ));

        let shopify = Arc::new(ShopifyBrowser::new(browser_manager.clone()));
        let shopee = Arc::new(ShopeeBrowser::new(browser_manager.clone()));
        let lazada = Arc::new(LazadaBrowser::new(browser_manager.clone()));
        let tokopedia = Arc::new(TokopediaBrowser::new(browser_manager.clone()));
        let tiktok = Arc::new(TikTokBrowser::new(browser_manager.clone()));
        let cj = Arc::new(CjBrowser::new(browser_manager.clone()));
        let telegram = Arc::new(TelegramNotifier::new());
        let monitor = Arc::new(AutomationMonitor::new());
        let fulfillment = Arc::new(FulfillmentEngine::new(
            shopify.clone(),
            shopee.clone(),
            lazada.clone(),
            tokopedia.clone(),
            tiktok.clone(),
            cj.clone(),
            telegram.clone(),
        ));
        let auto_pilot = Arc::new(AutoPilotEngine::new(telegram.clone()));

        Self {
            shopify,
            shopee,
            lazada,
            tokopedia,
            tiktok,
            cj,
            monitor,
            telegram,
            fulfillment,
            auto_pilot,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize)]
pub struct SystemStatus {
    pub shopify_connected: bool,
    pub shopee_connected: bool,
    pub lazada_connected: bool,
    pub tokopedia_connected: bool,
    pub tiktok_connected: bool,
    pub cj_connected: bool,
    pub telegram_connected: bool,
    pub automation_running: bool,
    pub automation_state: String,
}

#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub orders_today: i64,
    pub revenue_today: f64,
    pub shipped_today: i64,
    pub delivered_today: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShopifyCredentials {
    pub shop_url: String,
    pub access_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CJCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShopeeCredentials {
    pub email: String,
    pub password: String,
    pub country: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LazadaCredentials {
    pub email: String,
    pub password: String,
    pub country: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokopediaCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TiktokCredentials {
    pub email: String,
    pub password: String,
    pub country: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleSheetsConfig {
    pub spreadsheet_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TestResult {
    pub success: bool,
    pub message: String,
}

use std::sync::OnceLock;
static CREDENTIALS_DB: OnceLock<CredentialsDB> = OnceLock::new();

#[tauri::command]
fn check_for_updates() -> Result<Option<String>, String> {
    let updater = update::UpdateManager::new();
    updater.check_for_update()
}

#[tauri::command]
fn install_update() -> Result<(), String> {
    let updater = update::UpdateManager::new();
    updater.download_and_install()
}

#[tauri::command]
fn get_app_version() -> String {
    update::UpdateManager::new().get_current_version()
}

#[tauri::command]
fn get_changelog() -> String {
    // Changelog for each version
    r#"## 🆕 What's New in GLOWASIA Copilot

### v1.0.1 - Telegram Notifications
- ✅ Fixed app crash on launch (tokio panic)
- ✅ Telegram notification system improved
- 🔴 App crash alerts sent to Telegram
- 🟠 Automation failed notifications
- 🟡 Platform detection alerts
- 🟢 New order notifications
- 🔵 Order shipped notifications
- 🟣 Update available notifications
- ⏱️ Anti-spam rate limiting (10 sec)

### v1.0.0 - Initial Release
- 🚀 Core automation engine
- 🖥️ Dashboard with real-time status
- ⚙️ Platform credentials management
- 🔐 Secure credential storage
- 🌐 Multi-platform support ready

---
*Stay updated for more features!*
"#.to_string()
}

#[tauri::command]
fn mark_version_seen() -> Result<(), String> {
    let version = update::UpdateManager::new().get_current_version();
    db::set_setting("last_seen_version", &version).map_err(|e| e.to_string())
}

#[tauri::command]
fn should_show_whats_new() -> Result<bool, String> {
    let current = update::UpdateManager::new().get_current_version();
    let last_seen = db::get_setting("last_seen_version").map_err(|e| e.to_string())?;
    
    // Show if first run or version changed
    Ok(last_seen.map(|v| v != current).unwrap_or(true))
}

#[tauri::command]
fn get_system_status(state: State<AppState>) -> Result<SystemStatus, String> {
    // Load Telegram credentials on-demand before checking status
    if let Ok(Some(cred)) = crate::commands::load_credential("telegram".to_string()) {
        if let (Some(bot_token), Some(chat_id)) = (cred.api_key, cred.api_secret) {
            if !bot_token.is_empty() && !chat_id.is_empty() {
                state.telegram.configure(&bot_token, &chat_id);
            }
        }
    }
    
    Ok(SystemStatus {
        shopify_connected: state.shopify.is_configured(),
        shopee_connected: state.shopee.is_configured(),
        lazada_connected: state.lazada.is_configured(),
        tokopedia_connected: state.tokopedia.is_configured(),
        tiktok_connected: state.tiktok.is_configured(),
        cj_connected: state.cj.is_configured(),
        telegram_connected: state.telegram.is_configured(),
        automation_running: state.monitor.is_running(),
        automation_state: if state.monitor.is_running() { "Running" } else { "Stopped" }.to_string(),
    })
}

#[tauri::command]
fn get_dashboard_stats() -> Result<DashboardStats, String> {
    let (orders, revenue, shipped, delivered) = db::get_today_stats()
        .map_err(|e| e.to_string())?;
    Ok(DashboardStats {
        orders_today: orders,
        revenue_today: revenue,
        shipped_today: shipped,
        delivered_today: delivered,
    })
}

#[tauri::command]
fn get_recent_activity(limit: i64) -> Result<Vec<ActivityEntry>, String> {
    db::get_recent_activity(limit).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_recent_orders(limit: i64) -> Result<Vec<Order>, String> {
    db::get_recent_orders(limit).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_shopify_credentials(_state: State<AppState>, credentials: ShopifyCredentials) -> Result<(), String> {
    let json = serde_json::to_string(&credentials).map_err(|e| e.to_string())?;
    let encrypted = enc::encrypt(&json).map_err(|e| e.to_string())?;
    db::set_credential("shopify", &encrypted).map_err(|e| e.to_string())?;
    // Store shop_url for later use during login
    db::set_setting("shopify_shop_url", &credentials.shop_url).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn save_cj_credentials(_state: State<AppState>, credentials: CJCredentials) -> Result<(), String> {
    let json = serde_json::to_string(&credentials).map_err(|e| e.to_string())?;
    let encrypted = enc::encrypt(&json).map_err(|e| e.to_string())?;
    db::set_credential("cj", &encrypted).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn save_shopee_credentials(_state: State<AppState>, credentials: ShopeeCredentials) -> Result<(), String> {
    let json = serde_json::to_string(&credentials).map_err(|e| e.to_string())?;
    let encrypted = enc::encrypt(&json).map_err(|e| e.to_string())?;
    db::set_credential("shopee", &encrypted).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn save_lazada_credentials(_state: State<AppState>, credentials: LazadaCredentials) -> Result<(), String> {
    let json = serde_json::to_string(&credentials).map_err(|e| e.to_string())?;
    let encrypted = enc::encrypt(&json).map_err(|e| e.to_string())?;
    db::set_credential("lazada", &encrypted).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn save_tokopedia_credentials(_state: State<AppState>, credentials: TokopediaCredentials) -> Result<(), String> {
    let json = serde_json::to_string(&credentials).map_err(|e| e.to_string())?;
    let encrypted = enc::encrypt(&json).map_err(|e| e.to_string())?;
    db::set_credential("tokopedia", &encrypted).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn save_tiktok_credentials(_state: State<AppState>, credentials: TiktokCredentials) -> Result<(), String> {
    let json = serde_json::to_string(&credentials).map_err(|e| e.to_string())?;
    let encrypted = enc::encrypt(&json).map_err(|e| e.to_string())?;
    db::set_credential("tiktok", &encrypted).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn test_tiktok_login(state: State<AppState>, email: String, password: String, country: String) -> Result<(), String> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(state.tiktok.login(&email, &password, &country))
}

#[tauri::command]
fn test_tokopedia_login(state: State<AppState>, email: String, password: String) -> Result<(), String> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(state.tokopedia.login(&email, &password, "id"))
}

#[tauri::command]
fn get_tokopedia_orders(limit: i64) -> Result<Vec<Order>, String> {
    db::get_tokopedia_orders(limit).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_tiktok_orders(limit: i64) -> Result<Vec<Order>, String> {
    db::get_tiktok_orders(limit).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_tokopedia_status(state: State<AppState>) -> Result<bool, String> {
    Ok(state.tokopedia.is_configured())
}

#[tauri::command]
fn get_tiktok_status(state: State<AppState>) -> Result<bool, String> {
    Ok(state.tiktok.is_configured())
}

// ============ PRODUCT MANAGEMENT COMMANDS ============

#[tauri::command]
fn get_products() -> Result<Vec<db::Product>, String> {
    db::get_all_products().map_err(|e| e.to_string())
}

#[tauri::command]
fn add_product(product: db::ProductInput) -> Result<i64, String> {
    db::add_product(&product).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_product(id: i64, product: db::ProductInput) -> Result<(), String> {
    db::update_product(id, &product).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_product(id: i64) -> Result<(), String> {
    db::delete_product(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_product_status(id: i64, status: String) -> Result<(), String> {
    db::update_product_status(id, &status).map_err(|e| e.to_string())
}

#[tauri::command]
fn bulk_update_product_prices(prices: Vec<(i64, f64)>) -> Result<(), String> {
    for (id, price) in prices {
        db::update_product_price(id, price).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn bulk_update_product_status(ids: Vec<i64>, status: String) -> Result<(), String> {
    for id in ids {
        db::update_product_status(id, &status).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn bulk_delete_products(ids: Vec<i64>) -> Result<(), String> {
    for id in ids {
        db::delete_product(id).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn import_products_csv(csv_content: String) -> Result<db::CsvImportResult, String> {
    db::import_products_from_csv(&csv_content).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_product_count() -> Result<i64, String> {
    db::get_product_count().map_err(|e| e.to_string())
}

#[tauri::command]
fn sync_product_to_platform(product_id: i64, platform: String, link_id: String) -> Result<(), String> {
    db::update_platform_link(product_id, &platform, &link_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_markup_percentage() -> Result<f64, String> {
    db::get_setting("markup_percentage").map_err(|e| e.to_string())
        .map(|opt| opt.and_then(|s| s.parse().ok()).unwrap_or(30.0))
}

#[tauri::command]
fn set_markup_percentage(percentage: f64) -> Result<(), String> {
    db::set_setting("markup_percentage", &percentage.to_string()).map_err(|e| e.to_string())
}


#[tauri::command]
fn get_shopee_orders(limit: i64) -> Result<Vec<Order>, String> {
    db::get_shopee_orders(limit).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_shopee_status(state: State<AppState>) -> Result<bool, String> {
    Ok(state.shopee.is_configured())
}

#[tauri::command]
fn get_lazada_orders(limit: i64) -> Result<Vec<Order>, String> {
    db::get_lazada_orders(limit).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_lazada_status(state: State<AppState>) -> Result<bool, String> {
    Ok(state.lazada.is_configured())
}

#[tauri::command]
fn save_telegram_config(state: State<AppState>, config: TelegramConfig) -> Result<(), String> {
    let json = serde_json::to_string(&config).map_err(|e| e.to_string())?;
    let encrypted = enc::encrypt(&json).map_err(|e| e.to_string())?;
    db::set_credential("telegram", &encrypted).map_err(|e| e.to_string())?;
    state.telegram.configure(&config.bot_token, &config.chat_id);
    Ok(())
}

#[tauri::command]
fn save_google_sheets_config(config: GoogleSheetsConfig) -> Result<(), String> {
    db::set_setting("google_sheets_id", &config.spreadsheet_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_credentials_masked(service: &str) -> Result<bool, String> {
    let encrypted = db::get_credential(service).map_err(|e| e.to_string())?;
    Ok(encrypted.is_some())
}

#[tauri::command]
fn start_automation(state: State<AppState>) -> Result<(), String> {
    log::info!("Starting automation");
    db::log_activity("automation_started", Some("User started automation")).map_err(|e| e.to_string())?;
    state.monitor.start();
    Ok(())
}

#[tauri::command]
fn stop_automation(state: State<AppState>) -> Result<(), String> {
    log::info!("Stopping automation");
    db::log_activity("automation_stopped", Some("User stopped automation")).map_err(|e| e.to_string())?;
    state.monitor.stop();
    Ok(())
}

#[tauri::command]
fn get_automation_state(state: State<AppState>) -> Result<String, String> {
    Ok(if state.monitor.is_running() { "Running" } else { "Stopped" }.to_string())
}

#[tauri::command]
fn test_shopify_login(state: State<AppState>) -> Result<TestResult, String> {
    match state.shopify.is_configured() {
        true => Ok(TestResult { success: true, message: "Shopify configured".to_string() }),
        false => Ok(TestResult { success: false, message: "Shopify not configured".to_string() }),
    }
}

#[tauri::command]
fn test_cj_login(state: State<AppState>) -> Result<TestResult, String> {
    match state.cj.is_configured() {
        true => Ok(TestResult { success: true, message: "CJ configured".to_string() }),
        false => Ok(TestResult { success: false, message: "CJ not configured".to_string() }),
    }
}

#[tauri::command]
fn test_shopee_login(state: State<AppState>) -> Result<TestResult, String> {
    match state.shopee.is_configured() {
        true => Ok(TestResult { success: true, message: "Shopee configured".to_string() }),
        false => Ok(TestResult { success: false, message: "Shopee not configured".to_string() }),
    }
}

#[tauri::command]
fn test_lazada_login(state: State<AppState>) -> Result<TestResult, String> {
    match state.lazada.is_configured() {
        true => Ok(TestResult { success: true, message: "Lazada configured".to_string() }),
        false => Ok(TestResult { success: false, message: "Lazada not configured".to_string() }),
    }
}

#[tauri::command]
fn test_telegram(state: State<AppState>) -> Result<TestResult, String> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    match rt.block_on(state.telegram.send_message("🔔 GLOWASIA Copilot is connected!")) {
        Ok(()) => Ok(TestResult { success: true, message: "Connected successfully".to_string() }),
        Err(e) => Ok(TestResult { success: false, message: e }),
    }
}

#[tauri::command]
fn send_test_notification(state: State<AppState>, message: String) -> Result<(), String> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(state.telegram.send_message(&message))
}

#[tauri::command]
fn get_setting(key: &str) -> Result<Option<String>, String> {
    db::get_setting(key).map_err(|e| e.to_string())
}

#[tauri::command]
fn set_setting(key: &str, value: &str) -> Result<(), String> {
    db::set_setting(key, value).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_monitor_interval(state: State<AppState>) -> Result<u64, String> {
    Ok(state.monitor.get_interval_ms() / 1000)
}

#[tauri::command]
fn set_monitor_interval(state: State<AppState>, seconds: u64) -> Result<(), String> {
    state.monitor.set_interval(seconds);
    Ok(())
}

// ============ AUTO-PILOT COMMANDS ============

#[tauri::command]
fn get_auto_pilot_status(state: State<AppState>) -> Result<AutoPilotStatus, String> {
    Ok(state.auto_pilot.get_status())
}

#[tauri::command]
fn enable_auto_pilot(state: State<AppState>) -> Result<(), String> {
    state.auto_pilot.enable();
    Ok(())
}

#[tauri::command]
fn disable_auto_pilot(state: State<AppState>) -> Result<(), String> {
    state.auto_pilot.disable();
    Ok(())
}

#[tauri::command]
fn set_order_polling_interval(state: State<AppState>, seconds: u64) -> Result<(), String> {
    state.auto_pilot.set_order_polling_interval(seconds);
    Ok(())
}

#[tauri::command]
fn set_inventory_sync_interval(state: State<AppState>, seconds: u64) -> Result<(), String> {
    state.auto_pilot.set_inventory_sync_interval(seconds);
    Ok(())
}

#[tauri::command]
async fn health_check(state: State<'_, AppState>) -> Result<HealthStatus, String> {
    state.auto_pilot.health_check().await
}

#[tauri::command]
async fn send_daily_report(state: State<'_, AppState>) -> Result<(), String> {
    state.auto_pilot.send_daily_report().await
}

pub fn run() {
    std::panic::set_hook(Box::new(|panic_info| {
        log::error!("Application panic: {}", panic_info);
    }));

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    db::init_db().expect("Failed to initialize database");

    let app_state = AppState::new();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_system_status,
            get_dashboard_stats,
            get_recent_activity,
            get_recent_orders,
            save_shopify_credentials,
            save_cj_credentials,
            save_shopee_credentials,
            save_lazada_credentials,
            save_telegram_config,
            save_google_sheets_config,
            get_credentials_masked,
            crate::commands::save_credential,
            crate::commands::delete_credential,
            crate::commands::list_saved_platforms,
            start_automation,
            stop_automation,
            get_automation_state,
            test_shopify_login,
            test_cj_login,
            test_shopee_login,
            test_lazada_login,
            test_telegram,
            send_test_notification,
            get_setting,
            set_setting,
            get_monitor_interval,
            set_monitor_interval,
            get_auto_pilot_status,
            enable_auto_pilot,
            disable_auto_pilot,
            set_order_polling_interval,
            set_inventory_sync_interval,
            health_check,
            send_daily_report,
            get_shopee_orders,
            get_shopee_status,
            get_lazada_orders,
            get_lazada_status,
            save_tokopedia_credentials,
            test_tokopedia_login,
            get_tokopedia_orders,
            get_tokopedia_status,
            save_tiktok_credentials,
            test_tiktok_login,
            get_app_version,
            check_for_updates,
            install_update,
            get_changelog,
            mark_version_seen,
            should_show_whats_new,
            get_tiktok_orders,
            get_tiktok_status,
            // Product Management
            get_products,
            add_product,
            update_product,
            delete_product,
            update_product_status,
            bulk_update_product_prices,
            bulk_update_product_status,
            bulk_delete_products,
            import_products_csv,
            get_product_count,
            sync_product_to_platform,
            get_markup_percentage,
            set_markup_percentage,
        ])
        .setup(|app| {
            if let Err(e) = init_credentials_db(app.handle()) {
                log::error!("Failed to initialize credentials DB: {}", e);
            }

            #[cfg(desktop)]
            {
                use tauri::menu::{MenuBuilder, MenuItemBuilder};
                use tauri::tray::TrayIconBuilder;

                let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
                let show_item = MenuItemBuilder::with_id("show", "Show Window").build(app)?;
                let menu = MenuBuilder::new(app)
                    .items(&[&show_item, &quit_item])
                    .build()?;

                let _tray = TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .on_menu_event(|app, event| {
                        match event.id().as_ref() {
                            "quit" => {
                                log::info!("Quit requested from tray");
                                app.exit(0);
                            }
                            "show" => {
                                if let Some(window) = app.get_webview_window("main") {
                                    window.show().ok();
                                    window.set_focus().ok();
                                }
                            }
                            _ => {}
                        }
                    })
                    .build(app)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
