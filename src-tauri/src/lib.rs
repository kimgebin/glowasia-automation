pub mod browser;
pub mod automation;
pub mod storage;
pub mod update;
pub mod cred_db;
pub mod commands;

use std::sync::Arc;
use tauri::{Manager, State};
use serde::{Deserialize, Serialize};
use crate::cred_db::{CredentialsDB, CredentialData, SavedCredential};

fn init_credentials_db(app_handle: &tauri::AppHandle) -> Result<(), String> {
    CREDENTIALS_DB.set(CredentialsDB::new(app_handle)?)
        .map_err(|_| "Already initialized".to_string())
}

use crate::browser::manager::BrowserManager;
use crate::browser::shopify::{ShopifyBrowser, ShopifyConfig};
use crate::browser::shopee::{ShopeeBrowser};
use crate::browser::lazada::{LazadaBrowser, LazadaConfig};
use crate::browser::tokopedia::{TokopediaBrowser, TokopediaConfig};
use crate::browser::tiktok::{TiktokBrowser, TiktokConfig};
use crate::browser::cj::CjBrowser;
use crate::automation::monitor::Monitor;
use crate::automation::notifier::TelegramNotifier;
use crate::automation::fulfillment::FulfillmentEngine;
use crate::storage::db::{Order, ActivityEntry, self};
use crate::storage::credentials as enc;

pub struct AppState {
    pub shopify: Arc<ShopifyBrowser>,
    pub shopee: Arc<ShopeeBrowser>,
    pub lazada: Arc<LazadaBrowser>,
    pub tokopedia: Arc<TokopediaBrowser>,
    pub tiktok: Arc<TiktokBrowser>,
    pub cj: Arc<CjBrowser>,
    pub monitor: Arc<Monitor>,
    pub telegram: Arc<TelegramNotifier>,
    pub fulfillment: Arc<FulfillmentEngine>,
}

impl AppState {
    pub fn new() -> Self {
        let browser_manager = Arc::new(BrowserManager::new().expect("Failed to create browser manager"));

        let shopify = Arc::new(ShopifyBrowser::new(browser_manager.clone()));
        let shopee = Arc::new(ShopeeBrowser::new(browser_manager.clone()));
        let lazada = Arc::new(LazadaBrowser::new(browser_manager.clone()));
        let tokopedia = Arc::new(TokopediaBrowser::new(browser_manager.clone()));
        let tiktok = Arc::new(TiktokBrowser::new(browser_manager.clone()));
        let cj = Arc::new(CjBrowser::new(browser_manager.clone()));
        let telegram = Arc::new(TelegramNotifier::new());
        let monitor = Arc::new(Monitor::new());
        let fulfillment = Arc::new(FulfillmentEngine::new(
            shopify.clone(),
            shopee.clone(),
            lazada.clone(),
            tokopedia.clone(),
            tiktok.clone(),
            cj.clone(),
            telegram.clone(),
        ));

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
fn get_system_status(state: State<AppState>) -> Result<SystemStatus, String> {
    Ok(SystemStatus {
        shopify_connected: state.shopify.is_configured(),
        shopee_connected: state.shopee.is_configured(),
        lazada_connected: state.lazada.is_configured(),
        tokopedia_connected: state.tokopedia.is_configured(),
        tiktok_connected: state.tiktok.is_configured(),
        cj_connected: state.cj.is_configured(),
        telegram_connected: state.telegram.is_configured(),
        automation_running: state.monitor.is_running(),
        automation_state: format!("{:?}", state.monitor.get_state()),
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
fn save_shopify_credentials(state: State<AppState>, credentials: ShopifyCredentials) -> Result<(), String> {
    let json = serde_json::to_string(&credentials).map_err(|e| e.to_string())?;
    let encrypted = enc::encrypt(&json).map_err(|e| e.to_string())?;
    db::set_credential("shopify", &encrypted).map_err(|e| e.to_string())?;
    state.shopify.set_config(ShopifyConfig {
        shop_url: credentials.shop_url,
        access_token: credentials.access_token,
    });
    Ok(())
}

#[tauri::command]
fn save_cj_credentials(state: State<AppState>, credentials: CJCredentials) -> Result<(), String> {
    let json = serde_json::to_string(&credentials).map_err(|e| e.to_string())?;
    let encrypted = enc::encrypt(&json).map_err(|e| e.to_string())?;
    db::set_credential("cj", &encrypted).map_err(|e| e.to_string())?;
    state.cj.set_config(&credentials.email, &credentials.password);
    Ok(())
}

#[tauri::command]
fn save_shopee_credentials(state: State<AppState>, credentials: ShopeeCredentials) -> Result<(), String> {
    let json = serde_json::to_string(&credentials).map_err(|e| e.to_string())?;
    let encrypted = enc::encrypt(&json).map_err(|e| e.to_string())?;
    db::set_credential("shopee", &encrypted).map_err(|e| e.to_string())?;
    state.shopee.set_config(crate::browser::shopee::ShopeeConfig {
        email: credentials.email,
        password: credentials.password,
        country: credentials.country,
    });
    Ok(())
}

#[tauri::command]
fn save_lazada_credentials(state: State<AppState>, credentials: LazadaCredentials) -> Result<(), String> {
    let json = serde_json::to_string(&credentials).map_err(|e| e.to_string())?;
    let encrypted = enc::encrypt(&json).map_err(|e| e.to_string())?;
    db::set_credential("lazada", &encrypted).map_err(|e| e.to_string())?;
    state.lazada.set_config(LazadaConfig {
        email: credentials.email,
        password: credentials.password,
        country: credentials.country,
    });
    Ok(())
}

#[tauri::command]
fn save_tokopedia_credentials(state: State<AppState>, credentials: TokopediaCredentials) -> Result<(), String> {
    let json = serde_json::to_string(&credentials).map_err(|e| e.to_string())?;
    let encrypted = enc::encrypt(&json).map_err(|e| e.to_string())?;
    db::set_credential("tokopedia", &encrypted).map_err(|e| e.to_string())?;
    state.tokopedia.set_config(TokopediaConfig {
        email: credentials.email,
        password: credentials.password,
        country: "id".to_string(),
    });
    Ok(())
}

#[tauri::command]
fn save_tiktok_credentials(state: State<AppState>, credentials: TiktokCredentials) -> Result<(), String> {
    let json = serde_json::to_string(&credentials).map_err(|e| e.to_string())?;
    let encrypted = enc::encrypt(&json).map_err(|e| e.to_string())?;
    db::set_credential("tiktok", &encrypted).map_err(|e| e.to_string())?;
    state.tiktok.set_config(TiktokConfig {
        email: credentials.email,
        password: credentials.password,
        country: credentials.country,
    });
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
fn test_shopee_login(state: State<AppState>, email: String, password: String, country: String) -> Result<(), String> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(state.shopee.login(&email, &password, &country))
}

#[tauri::command]
fn get_tokopedia_orders(limit: i64) -> Result<Vec<crate::storage::db::Order>, String> {
    db::get_tokopedia_orders(limit).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_tiktok_orders(limit: i64) -> Result<Vec<crate::storage::db::Order>, String> {
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

#[tauri::command]
fn test_lazada_login(state: State<AppState>, email: String, password: String, country: String) -> Result<(), String> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(state.lazada.login(&email, &password, &country))
}

#[tauri::command]
fn get_shopee_orders(limit: i64) -> Result<Vec<crate::storage::db::Order>, String> {
    db::get_shopee_orders(limit).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_shopee_status(state: State<AppState>) -> Result<bool, String> {
    Ok(state.shopee.is_configured())
}

#[tauri::command]
fn get_lazada_orders(limit: i64) -> Result<Vec<crate::storage::db::Order>, String> {
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
    Ok(format!("{:?}", state.monitor.get_state()))
}

#[tauri::command]
fn test_shopify_login(state: State<AppState>, email: String, password: String) -> Result<(), String> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(state.shopify.login(&email, &password))
}

#[tauri::command]
fn test_cj_login(state: State<AppState>) -> Result<(), String> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(state.cj.login())
}

#[tauri::command]
fn test_telegram(state: State<AppState>) -> Result<(), String> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(state.telegram.send_message("🔔 GLOWASIA Copilot is connected!"))
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
    Ok(state.monitor.get_interval())
}

#[tauri::command]
fn set_monitor_interval(state: State<AppState>, seconds: u64) -> Result<(), String> {
    state.monitor.set_interval(seconds);
    Ok(())
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

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::new())
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
            get_tiktok_orders,
            get_tiktok_status,
            save_credential,
            load_credential,
            delete_credential,
            list_saved_platforms,
            save_app_setting,
            load_app_setting,
            export_credentials,
            import_credentials,
            cmd_save_credential,
            cmd_load_credential,
            cmd_delete_credential,
            cmd_list_saved_platforms,
            cmd_save_app_setting,
            cmd_load_app_setting,
            cmd_export_credentials,
            cmd_import_credentials,
        ])
        .setup(|app| {
            // Initialize credentials database
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