use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use crate::browser::manager::{BrowserManager, Platform};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidtransConfig {
    pub merchant_id: String,
    pub server_key: String,
    pub client_key: String,
    pub environment: String,  // "sandbox" or "production"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidtransTransaction {
    pub transaction_id: String,
    pub order_id: String,
    pub transaction_status: String,
    pub transaction_time: String,
    pub gross_amount: f64,
    pub currency: String,
    pub payment_type: String,
}

pub struct MidtransClient {
    manager: Arc<Mutex<BrowserManager>>,
}

impl MidtransClient {
    pub fn new(manager: Arc<Mutex<BrowserManager>>) -> Self {
        Self { manager }
    }
    
    pub fn is_configured(&self) -> bool {
        true
    }
    
    /// Get the API base URL based on environment
    pub fn get_base_url(environment: &str) -> &'static str {
        match environment.to_lowercase().as_str() {
            "production" => "https://api.midtrans.com/v1",
            _ => "https://api.sandbox.midtrans.com/v1",
        }
    }
    
    /// Generate Authorization header using Base64
    fn generate_auth_header(server_key: &str) -> String {
        let auth_string = format!("{}:", server_key);
        base64::encode(auth_string.as_bytes())
    }
    
    /// Make authenticated API request to Midtrans
    async fn api_request(
        &self,
        method: &str,
        endpoint: &str,
        config: &MidtransConfig,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let base_url = Self::get_base_url(&config.environment);
        let url = format!("{}{}", base_url, endpoint);
        let auth_header = Self::generate_auth_header(&config.server_key);
        
        let client = reqwest::Client::new();
        
        let response = match method.to_uppercase().as_str() {
            "GET" => {
                client.get(&url)
                    .header("Authorization", format!("Basic {}", auth_header))
                    .header("Content-Type", "application/json")
                    .send()
                    .await
            },
            "POST" => {
                let mut request = client.post(&url)
                    .header("Authorization", format!("Basic {}", auth_header))
                    .header("Content-Type", "application/json");
                
                if let Some(body) = body {
                    request = request.json(&body);
                }
                
                request.send().await
            },
            _ => return Err("Invalid HTTP method".to_string()),
        }.map_err(|e| format!("Request failed: {}", e))?;
        
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Parse response failed: {}", e))?;
        
        Ok(json)
    }
    
    /// Get transaction status
    pub async fn get_transaction_status(&self, config: &MidtransConfig, order_id: &str) -> Result<MidtransTransaction, String> {
        let response = self.api_request("GET", &format!("/transaction/{}", order_id), config, None).await?;
        
        let transaction = MidtransTransaction {
            transaction_id: response["transaction_id"].as_str().unwrap_or("").to_string(),
            order_id: response["order_id"].as_str().unwrap_or("").to_string(),
            transaction_status: response["transaction_status"].as_str().unwrap_or("").to_string(),
            transaction_time: response["transaction_time"].as_str().unwrap_or("").to_string(),
            gross_amount: response["gross_amount"].as_f64().unwrap_or(0.0),
            currency: response["currency"].as_str().unwrap_or("IDR").to_string(),
            payment_type: response["payment_type"].as_str().unwrap_or("").to_string(),
        };
        
        Ok(transaction)
    }
    
    /// Get transaction status via BVA (Bank Virtual Account) notification
    pub async fn get_transaction_status_bva(&self, config: &MidtransConfig, order_id: &str) -> Result<MidtransTransaction, String> {
        let response = self.api_request("GET", &format!("/transaction/{}", order_id), config, None).await?;
        
        Ok(MidtransTransaction {
            transaction_id: response["transaction_id"].as_str().unwrap_or("").to_string(),
            order_id: response["order_id"].as_str().unwrap_or("").to_string(),
            transaction_status: response["transaction_status"].as_str().unwrap_or("").to_string(),
            transaction_time: response["transaction_time"].as_str().unwrap_or("").to_string(),
            gross_amount: response["gross_amount"].as_f64().unwrap_or(0.0),
            currency: response["currency"].as_str().unwrap_or("IDR").to_string(),
            payment_type: response["payment_type"].as_str().unwrap_or("").to_string(),
        })
    }
    
    /// Check if payment is completed (settlement/capture)
    pub async fn is_payment_settled(&self, config: &MidtransConfig, order_id: &str) -> Result<bool, String> {
        let transaction = self.get_transaction_status(config, order_id).await?;
        let settled = matches!(
            transaction.transaction_status.as_str(),
            "settlement" | "capture" | "success"
        );
        Ok(settled)
    }
    
    /// Approve transaction (for manual capture)
    pub async fn approve_transaction(&self, config: &MidtransConfig, order_id: &str) -> Result<(), String> {
        let response = self.api_request("POST", &format!("/transaction/{}", order_id), config, None).await?;
        
        if response["status_code"].as_str() == Some("200") || response["status_code"].as_i64() == Some(200) {
            Ok(())
        } else {
            Err(response["status_message"].as_str().unwrap_or("Failed").to_string())
        }
    }
    
    /// Get available payment methods
    pub async fn get_payment_methods(&self, config: &MidtransConfig) -> Result<Vec<PaymentMethod>, String> {
        // Static list of common payment methods
        let methods = vec![
            // Credit Card
            PaymentMethod {
                code: "credit_card".to_string(),
                name: "Credit Card".to_string(),
                active: true,
            },
            // Bank Transfer
            PaymentMethod {
                code: "bank_transfer".to_string(),
                name: "Bank Transfer".to_string(),
                active: true,
            },
            PaymentMethod {
                code: "bca_va".to_string(),
                name: "BCA Virtual Account".to_string(),
                active: true,
            },
            PaymentMethod {
                code: "bni_va".to_string(),
                name: "BNI Virtual Account".to_string(),
                active: true,
            },
            PaymentMethod {
                code: "bri_va".to_string(),
                name: "BRI Virtual Account".to_string(),
                active: true,
            },
            PaymentMethod {
                code: "mandiri_va".to_string(),
                name: "Mandiri Virtual Account".to_string(),
                active: true,
            },
            // E-Wallet
            PaymentMethod {
                code: "gopay".to_string(),
                name: "GoPay".to_string(),
                active: true,
            },
            PaymentMethod {
                code: "shopeepay".to_string(),
                name: "ShopeePay".to_string(),
                active: true,
            },
            PaymentMethod {
                code: "ovo".to_string(),
                name: "OVO".to_string(),
                active: true,
            },
            PaymentMethod {
                code: "dana".to_string(),
                name: "DANA".to_string(),
                active: true,
            },
            // QRIS
            PaymentMethod {
                code: "qris".to_string(),
                name: "QRIS".to_string(),
                active: true,
            },
            // Retail
            PaymentMethod {
                code: "alfamart".to_string(),
                name: "Alfamart".to_string(),
                active: true,
            },
            PaymentMethod {
                code: "indomaret".to_string(),
                name: "Indomaret".to_string(),
                active: true,
            },
        ];
        
        Ok(methods)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub code: String,
    pub name: String,
    pub active: bool,
}
