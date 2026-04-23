use rusqlite::{Connection, params, Result};
use std::sync::Mutex;
use std::path::PathBuf;
use once_cell::sync::Lazy;
use directories::ProjectDirs;

pub struct DbPool {
    #[allow(dead_code)]
    conn: Mutex<Connection>,
}

impl DbPool {
    pub fn new(path: PathBuf) -> Result<Self> {
        let conn = Connection::open(&path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS orders (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                order_id TEXT NOT NULL,
                platform TEXT,
                status TEXT DEFAULT 'pending',
                customer_name TEXT,
                customer_address TEXT,
                customer_phone TEXT,
                customer_email TEXT,
                product_url TEXT,
                quantity INTEGER DEFAULT 1,
                price REAL,
                tracking_number TEXT,
                cj_order_id TEXT,
                shopify_order_id TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                shipped_at TIMESTAMP,
                delivered_at TIMESTAMP,
                customer_zip TEXT,
                customer_country TEXT,
                customer_city TEXT,
                customer_state TEXT
            );
            CREATE TABLE IF NOT EXISTS activity_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                action TEXT NOT NULL,
                details TEXT,
                timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS products (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                description TEXT,
                price REAL NOT NULL,
                cost REAL,
                images TEXT,
                category TEXT,
                sku TEXT,
                status TEXT DEFAULT 'active',
                platform_links TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );"
        )?;
        Ok(Self { conn: Mutex::new(conn) })
    }
}

impl DbPool {
    pub fn get() -> &'static Mutex<Connection> {
        static POOL: Lazy<Mutex<Connection>> = Lazy::new(|| {
            let proj_dirs = ProjectDirs::from("com", "glowasia", "automation")
                .expect("Failed to get project dirs");
            let data_dir = proj_dirs.data_dir();
            std::fs::create_dir_all(data_dir).expect("Failed to create data dir");
            let db_path = data_dir.join("glowasia.db");
            let conn = Connection::open(db_path).expect("Failed to open db");
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS orders (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    order_id TEXT NOT NULL,
                    platform TEXT,
                    status TEXT DEFAULT 'pending',
                    customer_name TEXT,
                    customer_address TEXT,
                    customer_phone TEXT,
                    customer_email TEXT,
                    product_url TEXT,
                    quantity INTEGER DEFAULT 1,
                    price REAL,
                    tracking_number TEXT,
                    cj_order_id TEXT,
                    shopify_order_id TEXT,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    shipped_at TIMESTAMP,
                    delivered_at TIMESTAMP,
                    customer_zip TEXT,
                    customer_country TEXT,
                    customer_city TEXT,
                    customer_state TEXT
                );
                CREATE TABLE IF NOT EXISTS activity_log (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    action TEXT NOT NULL,
                    details TEXT,
                    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                );
                CREATE TABLE IF NOT EXISTS settings (
                    key TEXT PRIMARY KEY,
                    value TEXT,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                );
                CREATE TABLE IF NOT EXISTS products (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL,
                    description TEXT,
                    price REAL NOT NULL,
                    cost REAL,
                    images TEXT,
                    category TEXT,
                    sku TEXT,
                    status TEXT DEFAULT 'active',
                    platform_links TEXT,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                );"
            ).expect("Failed to init db tables");
            Mutex::new(conn)
        });
        &POOL
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Order {
    pub id: i64,
    pub order_id: String,
    pub platform: Option<String>,
    pub status: String,
    pub customer_name: Option<String>,
    pub customer_address: Option<String>,
    pub customer_phone: Option<String>,
    pub customer_email: Option<String>,
    pub product_url: Option<String>,
    pub quantity: Option<i64>,
    pub price: Option<f64>,
    pub tracking_number: Option<String>,
    pub cj_order_id: Option<String>,
    pub shopify_order_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub shipped_at: Option<String>,
    pub delivered_at: Option<String>,
    pub customer_zip: Option<String>,
    pub customer_country: Option<String>,
    pub customer_city: Option<String>,
    pub customer_state: Option<String>,
}

impl Order {
    pub fn from_json(value: &serde_json::Value) -> Self {
        Order {
            id: value.get("id").and_then(|v| v.as_i64()).unwrap_or(0),
            order_id: value.get("order_id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            platform: value.get("platform").and_then(|v| v.as_str()).map(|s| s.to_string()),
            status: value.get("status").and_then(|v| v.as_str()).unwrap_or("pending").to_string(),
            customer_name: value.get("customer_name").and_then(|v| v.as_str()).map(|s| s.to_string()),
            customer_address: value.get("customer_address").and_then(|v| v.as_str()).map(|s| s.to_string()),
            customer_phone: value.get("customer_phone").and_then(|v| v.as_str()).map(|s| s.to_string()),
            customer_email: value.get("customer_email").and_then(|v| v.as_str()).map(|s| s.to_string()),
            product_url: value.get("product_url").and_then(|v| v.as_str()).map(|s| s.to_string()),
            quantity: value.get("quantity").and_then(|v| v.as_i64()),
            price: value.get("price").and_then(|v| v.as_f64()),
            tracking_number: value.get("tracking_number").and_then(|v| v.as_str()).map(|s| s.to_string()),
            cj_order_id: value.get("cj_order_id").and_then(|v| v.as_str()).map(|s| s.to_string()),
            shopify_order_id: value.get("shopify_order_id").and_then(|v| v.as_str()).map(|s| s.to_string()),
            created_at: value.get("created_at").and_then(|v| v.as_str()).map(|s| s.to_string()),
            updated_at: value.get("updated_at").and_then(|v| v.as_str()).map(|s| s.to_string()),
            shipped_at: value.get("shipped_at").and_then(|v| v.as_str()).map(|s| s.to_string()),
            delivered_at: value.get("delivered_at").and_then(|v| v.as_str()).map(|s| s.to_string()),
            customer_zip: value.get("customer_zip").and_then(|v| v.as_str()).map(|s| s.to_string()),
            customer_country: value.get("customer_country").and_then(|v| v.as_str()).map(|s| s.to_string()),
            customer_city: value.get("customer_city").and_then(|v| v.as_str()).map(|s| s.to_string()),
            customer_state: value.get("customer_state").and_then(|v| v.as_str()).map(|s| s.to_string()),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ActivityEntry {
    pub id: i64,
    pub action: String,
    pub details: Option<String>,
    pub timestamp: String,
}

pub fn init_db() -> Result<(), String> {
    let _ = DbPool::get();
    Ok(())
}

pub fn create_order(order: &Order, tracking: &str) -> Result<i64, String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO orders (order_id, platform, status, customer_name, customer_address, customer_phone, customer_email, product_url, quantity, price, tracking_number, created_at)
         VALUES (?1, ?2, 'processing', ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, CURRENT_TIMESTAMP)",
        params![
            order.order_id,
            order.platform,
            order.customer_name,
            order.customer_address,
            order.customer_phone,
            order.customer_email,
            order.product_url,
            order.quantity,
            order.price,
            tracking
        ]
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn get_today_stats() -> Result<(i64, f64, i64, i64), String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT COUNT(*), COALESCE(SUM(price), 0), 
         COUNT(CASE WHEN status = 'shipped' THEN 1 END),
         COUNT(CASE WHEN status = 'delivered' THEN 1 END)
         FROM orders WHERE DATE(created_at) = DATE('now')"
    ).map_err(|e| e.to_string())?;
    
    let result = stmt.query_row([], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    }).map_err(|e| e.to_string())?;
    
    Ok(result)
}

pub fn get_recent_activity(_limit: i64) -> Result<Vec<ActivityEntry>, String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    let _stmt = conn.prepare(
        "SELECT id, action, details, timestamp FROM activity_log ORDER BY timestamp DESC LIMIT ?1"
    ).map_err(|e| e.to_string())?;
    
    let entries = Vec::<ActivityEntry>::new();
    Ok(entries)
}

pub fn get_recent_orders(_limit: i64) -> Result<Vec<Order>, String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    let _stmt = conn.prepare(
        "SELECT id, order_id, platform, status, customer_name, customer_address, customer_phone, customer_email, product_url, quantity, price, tracking_number, cj_order_id, shopify_order_id, created_at, updated_at, shipped_at, delivered_at, customer_zip, customer_country, customer_city, customer_state
         FROM orders ORDER BY created_at DESC LIMIT ?1"
    ).map_err(|e| e.to_string())?;
    
    Ok(Vec::new())
}

pub fn get_shopee_orders(limit: i64) -> Result<Vec<Order>, String> {
    get_platform_orders("shopee", limit)
}

pub fn get_lazada_orders(limit: i64) -> Result<Vec<Order>, String> {
    get_platform_orders("lazada", limit)
}

pub fn get_tokopedia_orders(limit: i64) -> Result<Vec<Order>, String> {
    get_platform_orders("tokopedia", limit)
}

pub fn get_tiktok_orders(limit: i64) -> Result<Vec<Order>, String> {
    get_platform_orders("tiktok", limit)
}

fn get_platform_orders(_platform: &str, _limit: i64) -> Result<Vec<Order>, String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    let _stmt = conn.prepare(
        "SELECT id, order_id, platform, status, customer_name, customer_address, customer_phone, customer_email, product_url, quantity, price, tracking_number, cj_order_id, shopify_order_id, created_at, updated_at, shipped_at, delivered_at, customer_zip, customer_country, customer_city, customer_state
         FROM orders WHERE platform = ?1 ORDER BY created_at DESC LIMIT ?2"
    ).map_err(|e| e.to_string())?;
    
    let orders = Vec::<Order>::new();
    Ok(orders)
}

pub fn log_activity(action: &str, details: Option<&str>) -> Result<(), String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO activity_log (action, details) VALUES (?1, ?2)",
        params![action, details]
    ).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_credential(service: &str) -> Result<Option<String>, String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT value FROM settings WHERE key = ?1"
    ).map_err(|e| e.to_string())?;
    
    let result = stmt.query_row(params![format!("cred_{}", service)], |row| row.get(0));
    
    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

pub fn set_credential(service: &str, encrypted: &str) -> Result<(), String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?1, ?2, CURRENT_TIMESTAMP)",
        params![format!("cred_{}", service), encrypted]
    ).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_setting(key: &str) -> Result<Option<String>, String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT value FROM settings WHERE key = ?1"
    ).map_err(|e| e.to_string())?;
    
    let result = stmt.query_row(params![key], |row| row.get(0));
    
    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

pub fn set_setting(key: &str, value: &str) -> Result<(), String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?1, ?2, CURRENT_TIMESTAMP)",
        params![key, value]
    ).map_err(|e| e.to_string())?;
    Ok(())
}

// ============ PRODUCT MANAGEMENT ============

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Product {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub price: f64,
    pub cost: Option<f64>,
    pub images: Option<String>,
    pub category: Option<String>,
    pub sku: Option<String>,
    pub status: String,
    pub platform_links: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ProductInput {
    pub title: String,
    pub description: Option<String>,
    pub price: f64,
    pub cost: Option<f64>,
    pub images: Option<String>,
    pub category: Option<String>,
    pub sku: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CsvImportResult {
    pub imported: i64,
    pub errors: Vec<String>,
}

pub fn get_all_products() -> Result<Vec<Product>, String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, title, description, price, cost, images, category, sku, status, platform_links, created_at, updated_at FROM products ORDER BY created_at DESC"
    ).map_err(|e| e.to_string())?;
    
    let products = stmt.query_map([], |row| {
        Ok(Product {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            price: row.get(3)?,
            cost: row.get(4)?,
            images: row.get(5)?,
            category: row.get(6)?,
            sku: row.get(7)?,
            status: row.get(8)?,
            platform_links: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    
    Ok(products)
}

pub fn add_product(input: &ProductInput) -> Result<i64, String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO products (title, description, price, cost, images, category, sku, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            input.title,
            input.description,
            input.price,
            input.cost,
            input.images,
            input.category,
            input.sku,
            input.status.as_deref().unwrap_or("active")
        ]
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn update_product(id: i64, input: &ProductInput) -> Result<(), String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE products SET title = ?1, description = ?2, price = ?3, cost = ?4, images = ?5, category = ?6, sku = ?7, status = ?8, updated_at = CURRENT_TIMESTAMP WHERE id = ?9",
        params![
            input.title,
            input.description,
            input.price,
            input.cost,
            input.images,
            input.category,
            input.sku,
            input.status.as_deref().unwrap_or("active"),
            id
        ]
    ).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn delete_product(id: i64) -> Result<(), String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM products WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn update_product_status(id: i64, status: &str) -> Result<(), String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE products SET status = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
        params![status, id]
    ).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn update_product_price(id: i64, price: f64) -> Result<(), String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE products SET price = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
        params![price, id]
    ).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn update_platform_link(product_id: i64, platform: &str, link_id: &str) -> Result<(), String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    
    let current: Option<String> = conn.query_row(
        "SELECT platform_links FROM products WHERE id = ?1",
        params![product_id],
        |row| row.get(0)
    ).map_err(|e| e.to_string())?;
    
    let mut links: serde_json::Map<String, serde_json::Value> = current
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    
    links.insert(platform.to_string(), serde_json::Value::String(link_id.to_string()));
    let links_json = serde_json::to_string(&links).map_err(|e| e.to_string())?;
    
    conn.execute(
        "UPDATE products SET platform_links = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
        params![links_json, product_id]
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}

pub fn get_product_count() -> Result<i64, String> {
    let conn = DbPool::get().lock().map_err(|e| e.to_string())?;
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM products", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    Ok(count)
}

pub fn import_products_from_csv(csv_content: &str) -> Result<CsvImportResult, String> {
    let mut imported: i64 = 0;
    let mut errors: Vec<String> = Vec::new();
    
    let lines: Vec<&str> = csv_content.lines().collect();
    if lines.is_empty() {
        return Err("CSV file is empty".to_string());
    }
    
    let header = lines[0].to_lowercase();
    if !header.contains("title") || !header.contains("price") {
        return Err("CSV must have 'title' and 'price' columns".to_string());
    }
    
    let get_col = |line: &str, col_name: &str| -> Option<String> {
        let cols: Vec<&str> = line.split(',').collect();
        let headers_lower: Vec<&str> = header.split(',').collect();
        for (i, h) in headers_lower.iter().enumerate() {
            if h.trim().contains(col_name) && i < cols.len() {
                return Some(cols[i].trim().trim_matches('"').to_string());
            }
        }
        None
    };
    
    for (idx, line) in lines.iter().enumerate().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        
        let title = get_col(line, "title").unwrap_or_default();
        let price_str = get_col(line, "price").unwrap_or_default();
        
        if title.is_empty() {
            errors.push(format!("Row {}: Missing title", idx + 1));
            continue;
        }
        
        let price: f64 = match price_str.parse() {
            Ok(p) => p,
            Err(_) => {
                errors.push(format!("Row {}: Invalid price '{}'", idx + 1, price_str));
                continue;
            }
        };
        
        let description = get_col(line, "description");
        let cost: Option<f64> = get_col(line, "cost").and_then(|s| s.parse().ok());
        let images = get_col(line, "image_url");
        let category = get_col(line, "category");
        let sku = get_col(line, "sku");
        
        let input = ProductInput {
            title,
            description,
            price,
            cost,
            images,
            category,
            sku,
            status: Some("active".to_string()),
        };
        
        match add_product(&input) {
            Ok(_) => imported += 1,
            Err(e) => errors.push(format!("Row {}: {}", idx + 1, e)),
        }
    }
    
    Ok(CsvImportResult { imported, errors })
}
