use crate::cred_db::{CredentialData, SavedCredential};
use crate::CREDENTIALS_DB;

#[tauri::command]
#[allow(dead_code)]
pub fn save_credential(platform: String, data: CredentialData) -> Result<(), String> {
    let db = CREDENTIALS_DB.get().ok_or("Database not initialized")?;
    db.save_credential(&platform, &data)
}

#[tauri::command]
#[allow(dead_code)]
pub fn load_credential(platform: String) -> Result<Option<CredentialData>, String> {
    let db = CREDENTIALS_DB.get().ok_or("Database not initialized")?;
    db.load_credential(&platform)
}

#[tauri::command]
#[allow(dead_code)]
pub fn delete_credential(platform: String) -> Result<(), String> {
    let db = CREDENTIALS_DB.get().ok_or("Database not initialized")?;
    db.delete_credential(&platform)
}

#[tauri::command]
#[allow(dead_code)]
pub fn list_saved_platforms() -> Result<Vec<SavedCredential>, String> {
    let db = CREDENTIALS_DB.get().ok_or("Database not initialized")?;
    db.list_platforms()
}

#[tauri::command]
#[allow(dead_code)]
fn save_app_setting(key: String, value: String) -> Result<(), String> {
    let db = CREDENTIALS_DB.get().ok_or("Database not initialized")?;
    db.save_setting(&key, &value)
}

#[tauri::command]
#[allow(dead_code)]
fn load_app_setting(key: String) -> Result<Option<String>, String> {
    let db = CREDENTIALS_DB.get().ok_or("Database not initialized")?;
    db.load_setting(&key)
}

#[tauri::command]
#[allow(dead_code)]
fn export_credentials() -> Result<String, String> {
    let db = CREDENTIALS_DB.get().ok_or("Database not initialized")?;
    db.export_credentials()
}

#[tauri::command]
#[allow(dead_code)]
fn import_credentials(json_data: String) -> Result<usize, String> {
    let db = CREDENTIALS_DB.get().ok_or("Database not initialized")?;
    db.import_credentials(&json_data)
}

pub fn init_credentials_db(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let db = crate::cred_db::CredentialsDB::new(app_handle).map_err(|e| e.to_string())?;
    CREDENTIALS_DB.set(db).map_err(|_| "Already initialized".to_string())
}
