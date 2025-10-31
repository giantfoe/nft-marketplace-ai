// Wallet utilities and signature validation
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WalletBalanceRequest {
    pub wallet_address: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WalletBalanceResponse {
    pub balance: f64,
    pub wallet_address: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WalletNftsRequest {
    pub wallet_address: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WalletNftsResponse {
    pub nfts: Vec<serde_json::Value>,
    pub wallet_address: String,
    pub count: usize,
}

pub fn validate_signature(message: &str, signature: &str, pubkey: &str) -> bool {
    use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;
    use ed25519_dalek::{Signature, PublicKey, Verifier};
    
    println!("Validating signature for message: {}", message);
    println!("Signature: {}", signature);
    println!("Public key: {}", pubkey);
    
    // Parse the public key
    let pubkey = match Pubkey::from_str(pubkey) {
        Ok(pk) => pk,
        Err(e) => {
            println!("Failed to parse public key: {}", e);
            return false;
        }
    };
    
    // Decode the base58 signature
    let signature_bytes = match bs58::decode(signature).into_vec() {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("Failed to decode signature: {}", e);
            return false;
        }
    };
    
    // Ensure signature is 64 bytes (ed25519 signature length)
    if signature_bytes.len() != 64 {
        println!("Invalid signature length: {} (expected 64)", signature_bytes.len());
        return false;
    }
    
    // Convert to ed25519 signature
    let signature = match Signature::from_bytes(&signature_bytes) {
        Ok(sig) => sig,
        Err(e) => {
            println!("Failed to create ed25519 signature: {}", e);
            return false;
        }
    };
    
    // Convert public key to ed25519 public key
    let public_key = match PublicKey::from_bytes(&pubkey.to_bytes()) {
        Ok(key) => key,
        Err(e) => {
            println!("Failed to create public key: {}", e);
            return false;
        }
    };
    
    // Verify the signature against the message bytes
    match public_key.verify(message.as_bytes(), &signature) {
        Ok(_) => {
            println!("Signature verification successful");
            true
        }
        Err(e) => {
            println!("Signature verification failed: {}", e);
            false
        }
    }
}

pub async fn get_wallet_balance(
    client: std::sync::Arc<RpcClient>,
    wallet_address: &str,
) -> Result<WalletBalanceResponse, String> {
    let pubkey = Pubkey::from_str(wallet_address)
        .map_err(|e| format!("Invalid wallet address: {}", e))?;
    
   let balance = client.get_balance(&pubkey)
        .map_err(|e| format!("Failed to get balance: {}", e))?;
    // Convert lamports to SOL
    let balance_sol = balance as f64 / 1_000_000_000_f64;
    
    Ok(WalletBalanceResponse {
        balance: balance_sol,
        wallet_address: wallet_address.to_string(),
    })
}

pub async fn get_wallet_nfts(
    client: std::sync::Arc<RpcClient>,
    wallet_address: &str,
) -> Result<WalletNftsResponse, String> {
    let pubkey = Pubkey::from_str(wallet_address)
        .map_err(|e| format!("Invalid wallet address: {}", e))?;
    
    // TODO: Implement proper NFT fetching logic
    // For now, return empty list
    let nfts = Vec::new();
    
    Ok(WalletNftsResponse {
        nfts: nfts.clone(),
        wallet_address: wallet_address.to_string(),
        count: nfts.len(),
    })
}