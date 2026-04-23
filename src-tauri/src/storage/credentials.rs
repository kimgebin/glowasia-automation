use sha2::{Sha256, Digest};
use hex;

pub fn encrypt(data: &str) -> Result<String, String> {
    // Simple XOR-based encryption with machine-derived key
    // In production, use AES-256-GCM from a proper crypto library
    let machine_id = get_machine_id();
    let key = derive_key(&machine_id);
    
    let encrypted: Vec<u8> = data.as_bytes()
        .iter()
        .zip(key.as_bytes().iter())
        .map(|(b, k)| b ^ k)
        .collect();
    
    Ok(hex::encode(encrypted))
}

pub fn decrypt(encrypted: &str) -> Result<String, String> {
    let machine_id = get_machine_id();
    let key = derive_key(&machine_id);
    
    let data = hex::decode(encrypted)
        .map_err(|e| format!("Failed to decode hex: {}", e))?;
    
    let decrypted: Vec<u8> = data
        .iter()
        .zip(key.as_bytes().iter())
        .map(|(b, k)| b ^ k)
        .collect();
    
    String::from_utf8(decrypted)
        .map_err(|e| format!("Failed to decode string: {}", e))
}

fn get_machine_id() -> String {
    hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "default-machine-id".to_string())
}

fn derive_key(machine_id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(machine_id.as_bytes());
    hasher.update(b"glowasia-salt-v1");
    hex::encode(hasher.finalize())
}
