use std::sync::Arc;
use tauri::{Manager, State};
use serde::{Deserialize, Serialize};
use crate::cred_db::CredentialsDB;

pub fn init_credentials_db(app_handle: &tauri::AppHandle) -> Result<(), String> {
    CREDENTIALS_DB.set(CredentialsDB::new(app_handle)?)
        .map_err(|_| "Already initialized".to_string())
}

#[tauri::command]
fn cmd_save_credential(platform: String, data: CredentialData) -> Result<(), String> {
    CREDENTIALS_DB.get().ok_or("Database not initialized")?
        .save_credential(&platform, &data)
}

#[tauri::command]
fn cmd_load_credential(platform: String) -> Result<Option<CredentialData>, String> {
    CREDENTIALS_DB.get().ok_or("Database not initialized")?
        .load_credential(&platform)
}

#[tauri::command]
fn cmd_delete_credential(platform: String) -> Result<(), String> {
    CREDENTIALS_DB.get().ok_or("Database not initialized")?
        .delete_credential(&platform)
}

#[tauri::command]
fn cmd_list_saved_platforms() -> Result<Vec<SavedCredential>, String> {
    CREDENTIALS_DB.get().ok_or("Database not initialized")?
        .list_platforms()
}

#[tauri::command]
fn cmd_save_app_setting(key: String, value: String) -> Result<(), String> {
    CREDENTIALS_DB.get().ok_or("Database not initialized")?
        .save_setting(&key, &value)
}

#[tauri::command]
fn cmd_load_app_setting(key: String) -> Result<Option<String>, String> {
    CREDENTIALS_DB.get().ok_or("Database not initialized")?
        .load_setting(&key)
}

#[tauri::command]
fn cmd_export_credentials() -> Result<String, String> {
    CREDENTIALS_DB.get().ok_or("Database not initialized")?
        .export_credentials()
}

#[tauri::command]
fn cmd_import_credentials(json_data: String) -> Result<usize, String> {
    CREDENTIALS_DB.get().ok_or("Database not initialized")?
        .import_credentials(&json_data)
}
