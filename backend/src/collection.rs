use serde::Deserialize;
use utoipa::ToSchema;
use std::sync::Arc;

#[derive(Deserialize, ToSchema)]
pub struct CreateCollectionRequest {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator_pubkey: String,
}

pub async fn create_collection(
    _client: Arc<solana_client::rpc_client::RpcClient>,
    _req: CreateCollectionRequest,
) -> Result<serde_json::Value, String> {
    // TODO: Implement collection creation
    Ok(serde_json::json!({"status": "created"}))
}