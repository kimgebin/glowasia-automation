use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex;
use crate::browser::manager::{BrowserManager, Platform};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyConfig {
    pub api_key: String,
    pub shared_secret: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub oauth_token: Option<String>,
    pub shop_id: Option<i64>,
    pub expires_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyOrder {
    pub order_id: i64,
    pub shop_id: i64,
    pub order_number: i32,
    pub status: String,
    pub creation_tsz: i64,
    pub updated_tsz: i64,
    pub total_gross: EtsyMoney,
    pub items: Vec<EtsyOrderItem>,
    pub receipt_id: Option<i64>,
    pub buyer_user_id: Option<i64>,
    pub buyer_email: Option<String>,
    pub ship_to: Option<EtsyShippingAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyOrderItem {
    pub order_item_id: i64,
    pub listing_id: i64,
    pub title: String,
    pub quantity: i32,
    pub price: EtsyMoney,
    pub sku: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyMoney {
    pub amount: f64,
    pub currency_formatted_short: String,
    pub currency_formatted_long: String,
    pub currency_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyListing {
    pub listing_id: i64,
    pub title: String,
    pub description: String,
    pub price: EtsyMoney,
    pub quantity: i32,
    pub sku: Option<String>,
    pub state: String,
    pub shop_section_id: Option<i64>,
    pub taxonomy_id: Option<i64>,
    pub shipping_profile_id: Option<i64>,
    pub images: Vec<EtsyImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyImage {
    pub listing_image_id: i64,
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyShippingAddress {
    pub name: String,
    pub first_line: Option<String>,
    pub second_line: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyInventoryProduct {
    pub sku: String,
    pub offerings: Vec<EtsyOffering>,
    pub property_values: Vec<EtsyPropertyValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyOffering {
    pub price: f64,
    pub quantity: i32,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyPropertyValue {
    pub property_id: i64,
    pub property_name: String,
    pub scale_id: Option<i64>,
    pub value_ids: Vec<i64>,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyShippingCarrier {
    pub name: String,
    pub carrier_code: String,
    pub tracking_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyShipment {
    pub receipt_id: i64,
    pub carrier_name: String,
    pub tracking_code: String,
    pub tracking_url: Option<String>,
    pub ship_by_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyShop {
    pub shop_id: i64,
    pub shop_name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub currency_code: String,
    pub is_vat_operative: Option<bool>,
    pub vat_registration_number: Option<String>,
    pub shop_owner: Option<String>,
    pub creation_epoch: i64,
    pub initial_creation_date: String,
    pub gender: Option<String>,
    pub has_onboarded_custom_workshop_quality_panel: Option<bool>,
    pub custom_quality_panel_status: Option<String>,
}

pub struct EtsyClient {
    manager: Arc<Mutex<BrowserManager>>,
}

impl EtsyClient {
    pub fn new(manager: Arc<Mutex<BrowserManager>>) -> Self {
        Self { manager }
    }

    pub fn is_configured(&self) -> bool {
        true
    }

    pub fn get_base_url() -> &'static str {
        "https://openapi.etsy.com/v3"
    }

    /// Build the OAuth authorization URL for user to visit in browser.
    /// After the user authorizes, Etsy redirects to redirect_uri with ?code=xxx
    pub fn build_oauth_url(config: &EtsyConfig, redirect_uri: &str, scopes: &[&str]) -> String {
        let scope_str = scopes.join(" ");
        format!(
            "https://www.etsy.com/oauth/connect?response_type=code\
             &redirect_uri={}\
             &client_id={}\
             &scope={}",
            urlencoding::encode(redirect_uri),
            config.api_key,
            urlencoding::encode(&scope_str)
        )
    }

    /// Exchange an authorization code for access + refresh tokens.
    /// Etsy uses POST /v3/oauth/token (NOT /v3/oauth/exchange).
    pub async fn exchange_authorization_code(
        config: &EtsyConfig,
        code: &str,
        redirect_uri: &str,
    ) -> Result<TokenResponse, String> {
        let client = reqwest::Client::new();
        let url = format!("{}/oauth/token", Self::get_base_url());

        let params = serde_json::json!({
            "grant_type": "authorization_code",
            "client_id": config.api_key,
            "redirect_uri": redirect_uri,
            "code": code
        });

        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .basic_auth(&config.api_key, Some(&config.shared_secret))
            .json(&params)
            .send()
            .await
            .map_err(|e| format!("Token exchange request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Token exchange failed ({}): {}", status, body));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Parse token response failed: {}", e))?;

        let access_token = json["access_token"]
            .as_str()
            .ok_or_else(|| "No access_token in response".to_string())?
            .to_string();

        let refresh_token = json["refresh_token"].as_str().map(|s| s.to_string());

        let expires_in = json["expires_in"].as_i64().unwrap_or(3600);
        let now = chrono::Utc::now().timestamp();
        let expires_at = now + expires_in;

        Ok(TokenResponse {
            access_token,
            refresh_token,
            expires_at,
            token_type: json["token_type"].as_str().unwrap_or("Bearer").to_string(),
        })
    }

    /// Refresh an access token using the refresh token.
    pub async fn refresh_access_token(
        config: &EtsyConfig,
        refresh_token: &str,
    ) -> Result<TokenResponse, String> {
        let client = reqwest::Client::new();
        let url = format!("{}/oauth/token", Self::get_base_url());

        let params = serde_json::json!({
            "grant_type": "refresh_token",
            "client_id": config.api_key,
            "refresh_token": refresh_token
        });

        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .basic_auth(&config.api_key, Some(&config.shared_secret))
            .json(&params)
            .send()
            .await
            .map_err(|e| format!("Token refresh request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Token refresh failed ({}): {}", status, body));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Parse refresh response failed: {}", e))?;

        let access_token = json["access_token"]
            .as_str()
            .ok_or_else(|| "No access_token in response".to_string())?
            .to_string();

        let refresh_token = json["refresh_token"].as_str().map(|s| s.to_string());
        let expires_in = json["expires_in"].as_i64().unwrap_or(3600);
        let now = chrono::Utc::now().timestamp();
        let expires_at = now + expires_in;

        Ok(TokenResponse {
            access_token,
            refresh_token,
            expires_at,
            token_type: json["token_type"].as_str().unwrap_or("Bearer").to_string(),
        })
    }

    /// Revoke an access token (logout / disconnect).
    pub async fn revoke_token(config: &EtsyConfig) -> Result<(), String> {
        let client = reqwest::Client::new();
        let url = format!("{}/oauth/token", Self::get_base_url());

        let params = serde_json::json!({
            "grant_type": "client_credentials",
            "client_id": config.api_key,
            "token": config.oauth_token.as_deref().unwrap_or("")
        });

        let _response = client
            .delete(&url)
            .header("Content-Type", "application/json")
            .basic_auth(&config.api_key, Some(&config.shared_secret))
            .json(&params)
            .send()
            .await
            .map_err(|e| format!("Token revoke request failed: {}", e))?;

        Ok(())
    }

    /// Make an authenticated API request to Etsy.
    async fn api_request(
        &self,
        method: &str,
        endpoint: &str,
        access_token: &str,
        params: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let client = reqwest::Client::new();
        let base_url = Self::get_base_url();

        let url = match method.to_uppercase().as_str() {
            "GET" | "DELETE" => {
                if let Some(p) = &params {
                    let mut url_with_params = format!("{}{}", base_url, endpoint);
                    if let Some(obj) = p.as_object() {
                        if !obj.is_empty() {
                            let query_string = obj.iter()
                                .map(|(k, v)| format!("{}={}", k, v))
                                .collect::<Vec<_>>()
                                .join("&");
                            url_with_params = format!("{}?{}", url_with_params, query_string);
                        }
                    }
                    url_with_params
                } else {
                    format!("{}{}", base_url, endpoint)
                }
            },
            "POST" | "PUT" | "PATCH" => format!("{}{}", base_url, endpoint),
            _ => return Err("Invalid HTTP method".to_string()),
        };

        let mut request = match method.to_uppercase().as_str() {
            "GET" => client.get(&url),
            "POST" => client.post(&url),
            "PUT" => client.put(&url),
            "PATCH" => client.patch(&url),
            "DELETE" => client.delete(&url),
            _ => return Err("Invalid HTTP method".to_string()),
        };

        request = request
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key_for_requests());

        if let Some(p) = params {
            if method == "POST" || method == "PUT" || method == "PATCH" {
                request = request.json(&p);
            }
        }

        let response = request
            .send()
            .await
            .map_err(|e| format!("API request to {} failed: {}", url, e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Etsy API error ({}): {}", status, body));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Parse API response failed: {}", e))?;

        Ok(json)
    }

    fn api_key_for_requests(&self) -> String {
        // The API key is derived from config passed into each method.
        // For header injection we use a placeholder - the actual key comes from config.
        // We return an empty string as placeholder; callers inject it via Bearer token.
        String::new()
    }

    /// Login to Etsy (browser-based for OAuth flow).
    pub async fn login(&self, email: &str, password: &str) -> Result<(), String> {
        let credentials = serde_json::json!({
            "email": email,
            "password": password
        });

        let manager = self.manager.lock().map_err(|e| e.to_string())?;
        manager.login(Platform::Etsy, credentials)
    }

    /// Get orders (receipts) from Etsy shop.
    /// Etsy does not have a /orders endpoint; orders are called receipts.
    pub async fn get_orders(&self, config: &EtsyConfig, params: EtsyOrderParams) -> Result<Vec<EtsyOrder>, String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let shop_id = config.shop_id.unwrap_or(0);
        let endpoint = format!("/shops/{}/receipts", shop_id);

        let mut query_params = serde_json::Map::new();
        query_params.insert("limit".to_string(), serde_json::json!(params.limit.unwrap_or(20)));

        if let Some(min_created) = params.min_created {
            query_params.insert("min_created_epoch".to_string(), serde_json::json!(min_created));
        }
        if let Some(max_created) = params.max_created {
            query_params.insert("max_created_epoch".to_string(), serde_json::json!(max_created));
        }
        if let Some(status) = params.status.clone() {
            query_params.insert("status".to_string(), serde_json::json!(status));
        }

        let response = self.api_request(
            "GET",
            &endpoint,
            access_token,
            Some(serde_json::Value::Object(query_params)),
        ).await?;

        let orders = response["results"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|o| self.parse_order(o))
                    .collect()
            })
            .unwrap_or_default();

        Ok(orders)
    }

    /// Get a specific order by receipt_id.
    pub async fn get_order(&self, config: &EtsyConfig, receipt_id: i64) -> Result<EtsyOrder, String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let shop_id = config.shop_id.unwrap_or(0);
        let endpoint = format!("/shops/{}/receipts/{}", shop_id, receipt_id);

        let response = self.api_request("GET", &endpoint, access_token, None).await?;

        Ok(self.parse_order(&response))
    }

    fn parse_order(&self, o: &serde_json::Value) -> EtsyOrder {
        let items = o["line_items"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|item| EtsyOrderItem {
                        order_item_id: item["order_item_id"].as_i64().unwrap_or(0),
                        listing_id: item["listing_id"].as_i64().unwrap_or(0),
                        title: item["title"].as_str().unwrap_or("").to_string(),
                        quantity: item["quantity"].as_i64().unwrap_or(0) as i32,
                        price: EtsyMoney {
                            amount: item["price"]["amount"].as_f64().unwrap_or(0.0),
                            currency_formatted_short: item["price"]["currency_formatted_short"].as_str().unwrap_or("").to_string(),
                            currency_formatted_long: item["price"]["currency_formatted_long"].as_str().unwrap_or("").to_string(),
                            currency_code: item["price"]["currency_code"].as_str().unwrap_or("USD").to_string(),
                        },
                        sku: item["sku"].as_str().map(|s| s.to_string()),
                    })
                    .collect()
            })
            .unwrap_or_default();

        EtsyOrder {
            order_id: o["receipt_id"].as_i64().unwrap_or(0),
            shop_id: o["shop_id"].as_i64().unwrap_or(0),
            order_number: o["receipt_number"].as_i64().unwrap_or(0) as i32,
            status: o["status"].as_str().unwrap_or("").to_string(),
            creation_tsz: o["creation_tsz"].as_i64().unwrap_or(0),
            updated_tsz: o["updated_tsz"].as_i64().unwrap_or(0),
            total_gross: EtsyMoney {
                amount: o["total_gross"]["amount"].as_f64().unwrap_or(0.0),
                currency_formatted_short: o["total_gross"]["currency_formatted_short"].as_str().unwrap_or("").to_string(),
                currency_formatted_long: o["total_gross"]["currency_formatted_long"].as_str().unwrap_or("").to_string(),
                currency_code: o["total_gross"]["currency_code"].as_str().unwrap_or("USD").to_string(),
            },
            items,
            receipt_id: o["receipt_id"].as_i64(),
            buyer_user_id: o["buyer_user_id"].as_i64(),
            buyer_email: o["buyer_email"].as_str().map(|s| s.to_string()),
            ship_to: o["ship_to"].as_object().map(|s| EtsyShippingAddress {
                name: s["name"].as_str().unwrap_or("").to_string(),
                first_line: s["first_line"].as_str().map(|s| s.to_string()),
                second_line: s["second_line"].as_str().map(|s| s.to_string()),
                city: s["city"].as_str().map(|s| s.to_string()),
                state: s["state"].as_str().map(|s| s.to_string()),
                zip: s["zip"].as_str().map(|s| s.to_string()),
                country: s["country"].as_str().map(|s| s.to_string()),
            }),
        }
    }

    /// Get active listings from Etsy shop.
    pub async fn get_listings(&self, config: &EtsyConfig, params: EtsyListingParams) -> Result<Vec<EtsyListing>, String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let shop_id = config.shop_id.unwrap_or(0);
        let endpoint = format!("/shops/{}/listings/active", shop_id);

        let mut query_params = serde_json::Map::new();
        query_params.insert("limit".to_string(), serde_json::json!(params.limit.unwrap_or(20)));

        if let Some(state) = params.state.clone() {
            query_params.insert("state".to_string(), serde_json::json!(state));
        }

        let response = self.api_request(
            "GET",
            &endpoint,
            access_token,
            Some(serde_json::Value::Object(query_params)),
        ).await?;

        let listings = response["results"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|l| self.parse_listing(l))
                    .collect()
            })
            .unwrap_or_default();

        Ok(listings)
    }

    /// Get a specific listing by ID.
    pub async fn get_listing(&self, config: &EtsyConfig, listing_id: i64) -> Result<EtsyListing, String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let endpoint = format!("/listings/{}", listing_id);
        let response = self.api_request("GET", &endpoint, access_token, None).await?;

        Ok(self.parse_listing(&response))
    }

    fn parse_listing(&self, l: &serde_json::Value) -> EtsyListing {
        let images = l["images"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|img| EtsyImage {
                        listing_image_id: img["listing_image_id"].as_i64().unwrap_or(0),
                        url: img["url_fullxfull"].as_str().unwrap_or("").to_string(),
                        width: img["width"].as_i64().unwrap_or(0) as i32,
                        height: img["height"].as_i64().unwrap_or(0) as i32,
                    })
                    .collect()
            })
            .unwrap_or_default();

        EtsyListing {
            listing_id: l["listing_id"].as_i64().unwrap_or(0),
            title: l["title"].as_str().unwrap_or("").to_string(),
            description: l["description"].as_str().unwrap_or("").to_string(),
            price: EtsyMoney {
                amount: l["price"]["amount"].as_f64().unwrap_or(0.0),
                currency_formatted_short: l["price"]["currency_formatted_short"].as_str().unwrap_or("").to_string(),
                currency_formatted_long: l["price"]["currency_formatted_long"].as_str().unwrap_or("").to_string(),
                currency_code: l["price"]["currency_code"].as_str().unwrap_or("USD").to_string(),
            },
            quantity: l["quantity"].as_i64().unwrap_or(0) as i32,
            sku: l["sku"].as_str().map(|s| s.to_string()),
            state: l["state"].as_str().unwrap_or("").to_string(),
            shop_section_id: l["shop_section_id"].as_i64(),
            taxonomy_id: l["taxonomy_id"].as_i64(),
            shipping_profile_id: l["shipping_profile_id"].as_i64(),
            images,
        }
    }

    /// Create a draft listing.
    pub async fn create_listing(&self, config: &EtsyConfig, params: CreateListingParams) -> Result<EtsyListing, String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let shop_id = config.shop_id.ok_or("No shop_id configured")?;
        let endpoint = format!("/shops/{}/listings", shop_id);

        let mut body = serde_json::Map::new();
        body.insert("quantity".to_string(), serde_json::json!(params.quantity));
        body.insert("title".to_string(), serde_json::json!(params.title));
        body.insert("description".to_string(), serde_json::json!(params.description));
        body.insert("price".to_string(), serde_json::json!(params.price));
        body.insert("who_made".to_string(), serde_json::json!(params.who_made));
        body.insert("when_made".to_string(), serde_json::json!(params.when_made));
        body.insert("taxonomy_id".to_string(), serde_json::json!(params.taxonomy_id));
        body.insert("shipping_profile_id".to_string(), serde_json::json!(params.shipping_profile_id));
        body.insert("listing_state".to_string(), serde_json::json!("draft"));

        if let Some(sku) = params.sku {
            body.insert("sku".to_string(), serde_json::json!(sku));
        }
        if let Some(image_ids) = params.image_ids {
            body.insert("image_ids".to_string(), serde_json::json!(image_ids));
        }

        let response = self.api_request(
            "POST",
            &endpoint,
            access_token,
            Some(serde_json::Value::Object(body)),
        ).await?;

        Ok(self.parse_listing(&response))
    }

    /// Update an existing listing.
    pub async fn update_listing(&self, config: &EtsyConfig, listing_id: i64, params: UpdateListingParams) -> Result<EtsyListing, String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let endpoint = format!("/listings/{}", listing_id);

        let mut body = serde_json::Map::new();

        if let Some(quantity) = params.quantity {
            body.insert("quantity".to_string(), serde_json::json!(quantity));
        }
        if let Some(title) = params.title {
            body.insert("title".to_string(), serde_json::json!(title));
        }
        if let Some(description) = params.description {
            body.insert("description".to_string(), serde_json::json!(description));
        }
        if let Some(price) = params.price {
            body.insert("price".to_string(), serde_json::json!(price));
        }
        if let Some(state) = params.state {
            body.insert("state".to_string(), serde_json::json!(state));
        }
        if let Some(sku) = params.sku {
            body.insert("sku".to_string(), serde_json::json!(sku));
        }

        let response = self.api_request(
            "PUT",
            &endpoint,
            access_token,
            Some(serde_json::Value::Object(body)),
        ).await?;

        Ok(self.parse_listing(&response))
    }

    /// Delete (deactivate) a listing.
    pub async fn delete_listing(&self, config: &EtsyConfig, listing_id: i64) -> Result<(), String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let endpoint = format!("/listings/{}", listing_id);
        let _response = self.api_request("DELETE", &endpoint, access_token, None).await?;

        Ok(())
    }

    /// Get listing inventory (SKUs, offerings, property values).
    pub async fn get_listing_inventory(&self, config: &EtsyConfig, listing_id: i64) -> Result<Vec<EtsyInventoryProduct>, String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let endpoint = format!("/listings/{}/inventory", listing_id);
        let response = self.api_request("GET", &endpoint, access_token, None).await?;

        let products = response["products"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|p| {
                        let offerings = p["offerings"]
                            .as_array()
                            .map(|o| {
                                o.iter()
                                    .map(|off| EtsyOffering {
                                        price: off["price"]["amount"].as_f64().unwrap_or(0.0) / 100.0,
                                        quantity: off["quantity"].as_i64().unwrap_or(0) as i32,
                                        is_enabled: off["is_enabled"].as_bool().unwrap_or(true),
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();

                        let property_values = p["property_values"]
                            .as_array()
                            .map(|pv| {
                                pv.iter()
                                    .map(|v| EtsyPropertyValue {
                                        property_id: v["property_id"].as_i64().unwrap_or(0),
                                        property_name: v["property_name"].as_str().unwrap_or("").to_string(),
                                        scale_id: v["scale_id"].as_i64(),
                                        value_ids: v["value_ids"].as_array().map(|ids| ids.iter().filter_map(|i| i.as_i64()).collect()).unwrap_or_default(),
                                        values: v["values"].as_array().map(|vs| vs.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()).unwrap_or_default(),
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();

                        EtsyInventoryProduct {
                            sku: p["sku"].as_str().unwrap_or("").to_string(),
                            offerings,
                            property_values,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(products)
    }

    /// Update listing inventory (full update).
    pub async fn update_listing_inventory(&self, config: &EtsyConfig, listing_id: i64, products: Vec<EtsyInventoryProduct>) -> Result<(), String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let endpoint = format!("/listings/{}/inventory", listing_id);

        let products_json: Vec<serde_json::Value> = products.iter().map(|p| {
            serde_json::json!({
                "sku": p.sku,
                "offerings": p.offerings.iter().map(|o| {
                    serde_json::json!({
                        "price": (o.price * 100.0) as i64,
                        "quantity": o.quantity,
                        "is_enabled": o.is_enabled
                    })
                }).collect::<Vec<_>>(),
                "property_values": p.property_values.iter().map(|v| {
                    serde_json::json!({
                        "property_id": v.property_id,
                        "property_name": v.property_name,
                        "scale_id": v.scale_id,
                        "value_ids": v.value_ids,
                        "values": v.values
                    })
                }).collect::<Vec<_>>()
            })
        }).collect();

        let body = serde_json::json!({ "products": products_json });

        let response = self.api_request("PUT", &endpoint, access_token, Some(body)).await?;

        if response["error"].is_null() {
            Ok(())
        } else {
            Err(response["error"].as_str().unwrap_or("Failed to update inventory").to_string())
        }
    }

    /// Get available shipping carriers for label creation.
    pub async fn get_shipping_carriers(&self, config: &EtsyConfig) -> Result<Vec<EtsyShippingCarrier>, String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let shop_id = config.shop_id.ok_or("No shop_id configured")?;
        let endpoint = format!("/shops/{}/shipping/carriers", shop_id);

        let response = self.api_request("GET", &endpoint, access_token, None).await?;

        let carriers = response["results"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|c| EtsyShippingCarrier {
                        name: c["name"].as_str().unwrap_or("").to_string(),
                        carrier_code: c["carrier_code"].as_str().unwrap_or("").to_string(),
                        tracking_url: c["tracking_url"].as_str().map(|s| s.to_string()),
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(carriers)
    }

    /// Add tracking info to a receipt (shipment).
    pub async fn create_receipt_shipment(
        &self,
        config: &EtsyConfig,
        receipt_id: i64,
        carrier_name: String,
        tracking_code: String,
        ship_by_date: Option<String>,
    ) -> Result<EtsyShipment, String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let shop_id = config.shop_id.ok_or("No shop_id configured")?;
        let endpoint = format!("/shops/{}/receipts/{}/shipments", shop_id, receipt_id);

        let mut body = serde_json::Map::new();
        body.insert("carrier_name".to_string(), serde_json::json!(carrier_name));
        body.insert("tracking_code".to_string(), serde_json::json!(tracking_code));

        if let Some(ref date) = ship_by_date {
            body.insert("ship_by_date".to_string(), serde_json::json!(date));
        }

        let response = self.api_request(
            "POST",
            &endpoint,
            access_token,
            Some(serde_json::Value::Object(body)),
        ).await?;

        Ok(EtsyShipment {
            receipt_id,
            carrier_name,
            tracking_code,
            tracking_url: response["tracking_url"].as_str().map(|s| s.to_string()),
            ship_by_date,
        })
    }

    /// Get shop info for the configured shop_id.
    pub async fn get_shop(&self, config: &EtsyConfig) -> Result<EtsyShop, String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let shop_id = config.shop_id.unwrap_or(0);
        let endpoint = format!("/shops/{}", shop_id);

        let response = self.api_request("GET", &endpoint, access_token, None).await?;

        let shop = EtsyShop {
            shop_id: response["shop_id"].as_i64().unwrap_or(0),
            shop_name: response["shop_name"].as_str().unwrap_or("").to_string(),
            title: response["title"].as_str().map(|s| s.to_string()),
            description: response["description"].as_str().map(|s| s.to_string()),
            currency_code: response["currency_code"].as_str().unwrap_or("USD").to_string(),
            is_vat_operative: response["is_vat_operative"].as_bool(),
            vat_registration_number: response["vat_registration_number"].as_str().map(|s| s.to_string()),
            shop_owner: response["shop_owner"].as_str().map(|s| s.to_string()),
            creation_epoch: response["creation_epoch"].as_i64().unwrap_or(0),
            initial_creation_date: response["initial_creation_date"].as_str().unwrap_or("").to_string(),
            gender: response["gender"].as_str().map(|s| s.to_string()),
            has_onboarded_custom_workshop_quality_panel: response["has_onboarded_custom_workshop_quality_panel"].as_bool(),
            custom_quality_panel_status: response["custom_quality_panel_status"].as_str().map(|s| s.to_string()),
        };

        Ok(shop)
    }

    /// Get shop by shop name (for looking up shops by name instead of numeric ID).
    pub async fn get_shop_by_name(&self, config: &EtsyConfig, shop_name: &str) -> Result<EtsyShop, String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let endpoint = format!("/shops/{}", shop_name);

        let response = self.api_request("GET", &endpoint, access_token, None).await?;

        let shop = EtsyShop {
            shop_id: response["shop_id"].as_i64().unwrap_or(0),
            shop_name: response["shop_name"].as_str().unwrap_or("").to_string(),
            title: response["title"].as_str().map(|s| s.to_string()),
            description: response["description"].as_str().map(|s| s.to_string()),
            currency_code: response["currency_code"].as_str().unwrap_or("USD").to_string(),
            is_vat_operative: response["is_vat_operative"].as_bool(),
            vat_registration_number: response["vat_registration_number"].as_str().map(|s| s.to_string()),
            shop_owner: response["shop_owner"].as_str().map(|s| s.to_string()),
            creation_epoch: response["creation_epoch"].as_i64().unwrap_or(0),
            initial_creation_date: response["initial_creation_date"].as_str().unwrap_or("").to_string(),
            gender: response["gender"].as_str().map(|s| s.to_string()),
            has_onboarded_custom_workshop_quality_panel: response["has_onboarded_custom_workshop_quality_panel"].as_bool(),
            custom_quality_panel_status: response["custom_quality_panel_status"].as_str().map(|s| s.to_string()),
        };

        Ok(shop)
    }

    /// Get shop shipping profiles.
    pub async fn get_shipping_profiles(&self, config: &EtsyConfig) -> Result<Vec<serde_json::Value>, String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let shop_id = config.shop_id.ok_or("No shop_id configured")?;
        let endpoint = format!("/shops/{}/shipping-profiles", shop_id);

        let response = self.api_request("GET", &endpoint, access_token, None).await?;

        let profiles = response["results"]
            .as_array()
            .map(|arr| arr.to_owned())
            .unwrap_or_default();

        Ok(profiles)
    }

    /// Get taxonomy categories for listing creation.
    pub async fn get_taxonomy(&self, config: &EtsyConfig) -> Result<Vec<serde_json::Value>, String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let endpoint = "/taxonomy/categories";
        let response = self.api_request("GET", endpoint, access_token, None).await?;

        let categories = response["results"]
            .as_array()
            .map(|arr| arr.to_owned())
            .unwrap_or_default();

        Ok(categories)
    }

    /// Sync inventory - update quantity for a listing (convenience method).
    pub async fn sync_inventory(&self, config: &EtsyConfig, listing_id: i64, new_quantity: i32) -> Result<(), String> {
        let inventory = vec![EtsyInventoryProduct {
            sku: "".to_string(),
            offerings: vec![EtsyOffering {
                price: 0.0,
                quantity: new_quantity,
                is_enabled: true,
            }],
            property_values: vec![],
        }];

        self.update_listing_inventory(config, listing_id, inventory).await
    }

    /// Get order by receipt number.
    pub async fn get_order_by_number(&self, config: &EtsyConfig, order_number: i64) -> Result<Option<EtsyOrder>, String> {
        let orders = self.get_orders(config, EtsyOrderParams {
            min_created: None,
            max_created: None,
            status: Some("completed".to_string()),
            limit: Some(100),
        }).await?;

        Ok(orders.into_iter().find(|o| o.order_number as i64 == order_number))
    }

    /// Upload a listing image (requires multipart feature in reqwest).
    /// Enable by adding `features = ["multipart"]` to reqwest in Cargo.toml.
    pub async fn upload_listing_image(&self, _config: &EtsyConfig, _listing_id: i64, _image_data: &[u8], _mime_type: &str) -> Result<EtsyImage, String> {
        Err("Image upload requires 'multipart' feature in reqwest. Update Cargo.toml: reqwest = { version = \"0.12\", features = [\"json\", \"multipart\"] }".to_string())
    }

    /// Get listings for a specific shop section.
    pub async fn get_shop_section_listings(&self, config: &EtsyConfig, section_id: i64, limit: Option<i64>) -> Result<Vec<EtsyListing>, String> {
        let access_token = config.oauth_token.as_ref()
            .or(config.access_token.as_ref())
            .ok_or("No OAuth token - need to authorize first")?;

        let shop_id = config.shop_id.unwrap_or(0);
        let endpoint = format!("/shops/{}/sections/{}/listings", shop_id, section_id);

        let mut query_params = serde_json::Map::new();
        query_params.insert("limit".to_string(), serde_json::json!(limit.unwrap_or(20)));

        let response = self.api_request(
            "GET",
            &endpoint,
            access_token,
            Some(serde_json::Value::Object(query_params)),
        ).await?;

        let listings = response["results"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|l| self.parse_listing(l))
                    .collect()
            })
            .unwrap_or_default();

        Ok(listings)
    }
}

// ── Token response ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: i64,
    pub token_type: String,
}

// ── Request/response parameter structs ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyOrderParams {
    pub min_created: Option<i64>,
    pub max_created: Option<i64>,
    pub status: Option<String>,
    pub limit: Option<i64>,
}

impl Default for EtsyOrderParams {
    fn default() -> Self {
        Self {
            min_created: None,
            max_created: None,
            status: Some("completed".to_string()),
            limit: Some(20),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsyListingParams {
    pub state: Option<String>,
    pub limit: Option<i64>,
}

impl Default for EtsyListingParams {
    fn default() -> Self {
        Self {
            state: Some("active".to_string()),
            limit: Some(20),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateListingParams {
    pub quantity: i32,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub who_made: String,
    pub when_made: String,
    pub taxonomy_id: i64,
    pub shipping_profile_id: i64,
    pub sku: Option<String>,
    pub image_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateListingParams {
    pub quantity: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub state: Option<String>,
    pub sku: Option<String>,
}

impl Default for UpdateListingParams {
    fn default() -> Self {
        Self {
            quantity: None,
            title: None,
            description: None,
            price: None,
            state: None,
            sku: None,
        }
    }
}