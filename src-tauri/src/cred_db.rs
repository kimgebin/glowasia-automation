use rusqlite::{Connection, params};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CredentialData {
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub shop_url: Option<String>,
    pub additional_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedCredential {
    pub platform: String,
    pub data: CredentialData,
    pub status: String,
    pub updated_at: String,
}

pub struct CredentialsDB {
    conn: Mutex<Connection>,
}

impl CredentialsDB {
    pub fn new(app_handle: &AppHandle) -> Result<Self, String> {
        let app_dir = app_handle.path().app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {}", e))?;
        
        std::fs::create_dir_all(&app_dir)
            .map_err(|e| format!("Failed to create app dir: {}", e))?;
        
        let db_path = app_dir.join("credentials.db");
        let conn = Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;
        
        // Initialize tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS api_credentials (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                platform TEXT NOT NULL UNIQUE,
                api_key TEXT,
                api_secret TEXT,
                access_token TEXT,
                refresh_token TEXT,
                shop_url TEXT,
                additional_data TEXT,
                status TEXT DEFAULT 'active',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )", []
        ).map_err(|e| format!("Failed to create table: {}", e))?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS app_settings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key TEXT NOT NULL UNIQUE,
                value TEXT,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )", []
        ).map_err(|e| format!("Failed to create settings table: {}", e))?;
        
        log::info!("Credentials database initialized at {:?}", db_path);
        Ok(Self { conn: Mutex::new(conn) })
    }
    
    pub fn save_credential(&self, platform: &str, data: &CredentialData) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        
        conn.execute(
            "INSERT OR REPLACE INTO api_credentials 
             (platform, api_key, api_secret, access_token, refresh_token, shop_url, additional_data, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, CURRENT_TIMESTAMP)",
            params![
                platform,
                data.api_key,
                data.api_secret,
                data.access_token,
                data.refresh_token,
                data.shop_url,
                data.additional_data
            ]
        ).map_err(|e| format!("Failed to save credential: {}", e))?;
        
        log::info!("Saved credential for platform: {}", platform);
        Ok(())
    }
    
    pub fn load_credential(&self, platform: &str) -> Result<Option<CredentialData>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        
        let mut stmt = conn.prepare(
            "SELECT api_key, api_secret, access_token, refresh_token, shop_url, additional_data 
             FROM api_credentials WHERE platform = ?1 AND status = 'active'"
        ).map_err(|e| e.to_string())?;
        
        let result = stmt.query_row(params![platform], |row| {
            Ok(CredentialData {
                api_key: row.get(0)?,
                api_secret: row.get(1)?,
                access_token: row.get(2)?,
                refresh_token: row.get(3)?,
                shop_url: row.get(4)?,
                additional_data: row.get(5)?,
            })
        });
        
        match result {
            Ok(data) => Ok(Some(data)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }
    
    pub fn delete_credential(&self, platform: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        
        conn.execute(
            "UPDATE api_credentials SET status = 'deleted', updated_at = CURRENT_TIMESTAMP WHERE platform = ?1",
            params![platform]
        ).map_err(|e| format!("Failed to delete credential: {}", e))?;
        
        log::info!("Deleted credential for platform: {}", platform);
        Ok(())
    }
    
    pub fn list_platforms(&self) -> Result<Vec<SavedCredential>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        
        let mut stmt = conn.prepare(
            "SELECT platform, api_key, api_secret, access_token, refresh_token, shop_url, additional_data, status, updated_at 
             FROM api_credentials WHERE status = 'active'"
        ).map_err(|e| e.to_string())?;
        
        let credentials = stmt.query_map([], |row| {
            Ok(SavedCredential {
                platform: row.get(0)?,
                data: CredentialData {
                    api_key: row.get(1)?,
                    api_secret: row.get(2)?,
                    access_token: row.get(3)?,
                    refresh_token: row.get(4)?,
                    shop_url: row.get(5)?,
                    additional_data: row.get(6)?,
                },
                status: row.get(7)?,
                updated_at: row.get(8)?,
            })
        }).map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
        
        Ok(credentials)
    }
    
    pub fn save_setting(&self, key: &str, value: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        
        conn.execute(
            "INSERT OR REPLACE INTO app_settings (key, value, updated_at) VALUES (?1, ?2, CURRENT_TIMESTAMP)",
            params![key, value]
        ).map_err(|e| format!("Failed to save setting: {}", e))?;
        
        Ok(())
    }
    
    pub fn load_setting(&self, key: &str) -> Result<Option<String>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        
        let mut stmt = conn.prepare("SELECT value FROM app_settings WHERE key = ?1")
            .map_err(|e| e.to_string())?;
        
        let result = stmt.query_row(params![key], |row| row.get(0));
        
        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }
    
    pub fn export_credentials(&self) -> Result<String, String> {
        let credentials = self.list_platforms()?;
        serde_json::to_string(&credentials).map_err(|e| e.to_string())
    }
    
    pub fn import_credentials(&self, json_data: &str) -> Result<usize, String> {
        let credentials: Vec<SavedCredential> = serde_json::from_str(json_data)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;
        
        let mut count = 0;
        for cred in credentials {
            self.save_credential(&cred.platform, &cred.data)?;
            count += 1;
        }
        
        log::info!("Imported {} credentials", count);
        Ok(count)
    }
}
