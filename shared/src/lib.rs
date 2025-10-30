use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GenerateImageRequest {
    pub prompt: String,
    pub style: Option<String>,
}

#[derive(Serialize)]
pub struct GenerateImageResponse {
    pub image_url: String,
    pub image_data: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MintNftRequest {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator_pubkey: String,
}

#[derive(Serialize)]
pub struct MintNftResponse {
    pub nft_address: String,
    pub transaction_signature: String,
}