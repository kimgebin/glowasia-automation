use std::sync::Arc;
use crate::browser::shopify::{ShopifyBrowser, ShopifyConfig};
use crate::browser::shopee::ShopeeBrowser;
use crate::browser::lazada::LazadaBrowser;
use crate::browser::tokopedia::TokopediaBrowser;
use crate::browser::tiktok::TikTokBrowser;
use crate::browser::cj::{CjBrowser, CJOrderData};
use crate::automation::notifier::TelegramNotifier;
use crate::storage::db::Order;

pub struct FulfillmentEngine {
    shopify: Arc<ShopifyBrowser>,
    shopee: Arc<ShopeeBrowser>,
    lazada: Arc<LazadaBrowser>,
    tokopedia: Arc<TokopediaBrowser>,
    tiktok: Arc<TikTokBrowser>,
    cj: Arc<CjBrowser>,
    telegram: Arc<TelegramNotifier>,
}

impl FulfillmentEngine {
    pub fn new(
        shopify: Arc<ShopifyBrowser>,
        shopee: Arc<ShopeeBrowser>,
        lazada: Arc<LazadaBrowser>,
        tokopedia: Arc<TokopediaBrowser>,
        tiktok: Arc<TikTokBrowser>,
        cj: Arc<CjBrowser>,
        telegram: Arc<TelegramNotifier>,
    ) -> Self {
        Self {
            shopify,
            shopee,
            lazada,
            tokopedia,
            tiktok,
            cj,
            telegram,
        }
    }
    
    /// Get Shopify config from database
    async fn get_shopify_config(&self) -> Option<ShopifyConfig> {
        // For now, return a config with placeholder token
        // In production, this would load from database and use Dev Dashboard token
        Some(ShopifyConfig {
            shop_url: "glowasia-2.myshopify.com".to_string(),
            access_token: None,
            admin_api_key: None,
            admin_api_secret: None,
        })
    }
    
    /// Convert ShopifyOrder to Order for processing
    fn shopify_order_to_order(shopify_order: &crate::browser::shopify::ShopifyOrder) -> Order {
        Order {
            id: 0,
            order_id: shopify_order.order_id.to_string(),
            platform: Some("Shopify".to_string()),
            status: shopify_order.status.clone(),
            customer_name: None,
            customer_address: None,
            customer_phone: None,
            customer_email: Some(shopify_order.email.clone()),
            product_url: None,
            quantity: shopify_order.line_items.first().map(|i| i.quantity as i64),
            price: shopify_order.total_price.parse::<f64>().ok(),
            tracking_number: None,
            cj_order_id: None,
            shopify_order_id: Some(shopify_order.order_id.to_string()),
            created_at: None,
            updated_at: None,
            shipped_at: None,
            delivered_at: None,
            customer_zip: None,
            customer_country: None,
            customer_city: None,
            customer_state: None,
        }
    }
    
    /// Check all platforms for new orders and process them
    pub async fn check_all_platforms(&self) -> Result<Vec<String>, String> {
        let mut processed = Vec::new();
        
        // Check Shopify (requires config from database)
        if let Some(config) = self.get_shopify_config().await {
            match self.shopify.get_unfulfilled_orders(&config).await {
                Ok(orders) => {
                    for order in orders {
                        let order_id = order.order_id.to_string();
                        log::info!("Processing Shopify order {}", order_id);
                        processed.push(order_id);
                    }
                }
                Err(e) => log::warn!("Shopify check failed: {}", e),
            }
        } else {
            log::warn!("Shopify not configured - skipping order check");
        }
        
        // Check Shopee (requires config - skipped for now)
        // match self.shopee.get_orders(50).await {
        //     Ok(orders) => {
        //         for order in orders {
        //             if let Err(e) = self.process_order(&order, "Shopee").await {
        //                 log::error!("Failed to process Shopee order {}: {}", order.order_id, e);
        //             } else {
        //                 processed.push(order.order_id.clone());
        //             }
        //         }
        //     }
        //     Err(e) => log::warn!("Shopee check failed: {}", e),
        // }
        
        // Check Lazada (requires config - skipped for now)
        // match self.lazada.get_orders(50).await {
        //     Ok(orders) => {
        //         for order in orders {
        //             if let Err(e) = self.process_order(&order, "Lazada").await {
        //                 log::error!("Failed to process Lazada order {}: {}", order.order_id, e);
        //             } else {
        //                 processed.push(order.order_id.clone());
        //             }
        //         }
        //     }
        //     Err(e) => log::warn!("Lazada check failed: {}", e),
        // }
        
        // Check Tokopedia (requires config - skipped for now)
        // match self.tokopedia.get_orders(50).await {
        //     Ok(orders) => {
        //         for order in orders {
        //             if let Err(e) = self.process_order(&order, "Tokopedia").await {
        //                 log::error!("Failed to process Tokopedia order {}: {}", order.order_id, e);
        //             } else {
        //                 processed.push(order.order_id.clone());
        //             }
        //         }
        //     }
        //     Err(e) => log::warn!("Tokopedia check failed: {}", e),
        // }
        
        // Check TikTok (requires config - skipped for now)
        // match self.tiktok.get_orders(config, params).await {
        //     Ok(orders) => {
        //         for order in orders {
        //             if let Err(e) = self.process_order(&order, "TikTok").await {
        //                 log::error!("Failed to process TikTok order {}: {}", order.order_id, e);
        //             } else {
        //                 processed.push(order.order_id.clone());
        //             }
        //         }
        //     }
        //     Err(e) => log::warn!("TikTok check failed: {}", e),
        // }
        
        Ok(processed)
    }
    
    /// Process a single order: forward to CJ and send notification
    async fn process_order(&self, order: &Order, platform: &str) -> Result<(), String> {
        log::info!("Processing order {} from {}", order.order_id, platform);
        
        // Build CJ order data from the order
        let cj_order = CJOrderData {
            product_url: order.product_url.clone().unwrap_or_default(),
            quantity: order.quantity.unwrap_or(1) as u32,
            customer_name: order.customer_name.clone().unwrap_or_default(),
            customer_address: order.customer_address.clone().unwrap_or_default(),
            customer_phone: order.customer_phone.clone().unwrap_or_default(),
            customer_zip: order.customer_zip.clone().unwrap_or_default(),
            customer_country: order.customer_country.clone().unwrap_or("ID".to_string()),
            customer_city: order.customer_city.clone().unwrap_or_default(),
            customer_state: order.customer_state.clone().unwrap_or_default(),
        };
        
        // Submit to CJ Dropshipping
        let tracking = self.cj.create_order(&cj_order).await?;
        
        // Update the original platform with tracking number
        match platform {
            "Shopify" => {
                // Shopify tracking update requires config - logged for manual update
                log::info!("Order {} tracking {} - Shopify requires config for auto-update", order.order_id, tracking);
            }
            _ => {
                // For other platforms, log the tracking for manual update
                log::info!("Order {} tracking {} - manual update may be required", order.order_id, tracking);
            }
        }
        
        // Send Telegram notification
        self.telegram.send_order_notification(&order.order_id, &order.customer_name.clone().unwrap_or_default(), platform, None).await?;
        
        // Log to database
        if let Err(e) = crate::storage::db::create_order(&order, &tracking) {
            log::warn!("Failed to log order to database: {}", e);
        }
        
        Ok(())
    }
    
    /// Manually process a specific order from a platform
    pub async fn manual_process(&self, order_id: &str, platform: &str) -> Result<String, String> {
        log::info!("Manual processing order {} from {}", order_id, platform);
        
        let order = Order {
            id: 0,
            order_id: order_id.to_string(),
            platform: Some(platform.to_string()),
            status: "pending".to_string(),
            customer_name: Some("Manual Order".to_string()),
            customer_address: None,
            customer_phone: None,
            customer_email: None,
            product_url: None,
            quantity: Some(1),
            price: None,
            tracking_number: None,
            cj_order_id: None,
            shopify_order_id: None,
            created_at: None,
            updated_at: None,
            shipped_at: None,
            delivered_at: None,
            customer_zip: None,
            customer_country: None,
            customer_city: None,
            customer_state: None,
        };
        
        self.process_order(&order, platform).await?;
        
        Ok("Order processed successfully".to_string())
    }
}
