use shared::{MintNftRequest, MintNftResponse};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::sync::Arc;

pub struct ContractClient {
    client: Arc<solana_client::rpc_client::RpcClient>,
}

impl ContractClient {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            client: Arc::new(solana_client::rpc_client::RpcClient::new(rpc_url)),
        }
    }

    pub async fn mint_nft(&self, req: MintNftRequest) -> Result<MintNftResponse, String> {
        // TODO: Implement actual contract call
        // For now, return mock response

        let nft_mint = Keypair::new();
        Ok(MintNftResponse {
            nft_address: nft_mint.pubkey().to_string(),
            transaction_signature: "mock_signature".to_string(),
        })
    }
}