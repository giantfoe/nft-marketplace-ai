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

pub fn validate_signature(_message: &str, _signature: &str, _pubkey: &str) -> bool {
    // TODO: Implement proper signature validation
    // For now, return true
    true
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