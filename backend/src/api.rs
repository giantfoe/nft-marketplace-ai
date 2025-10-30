use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

use crate::{nft, freepik_api::FreepikApiClient};

// Import required crates
extern crate md5;

// Shared state for the API
#[derive(Clone)]
pub struct ApiState {
    pub solana_client: Arc<solana_client::rpc_client::RpcClient>,
    pub freepik_client: Option<FreepikApiClient>,
    pub keypair: Arc<solana_sdk::signature::Keypair>,
    pub url_mappings: Arc<tokio::sync::RwLock<std::collections::HashMap<String, String>>>,
}

// Standard API Response wrapper
#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub message: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

// Helper function to create success responses
fn success_response<T>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        success: true,
        data: Some(data),
        error: None,
        message: None,
    })
}

// Helper function to create error responses
fn error_response<T>(code: &str, message: &str) -> Result<Json<ApiResponse<T>>, StatusCode> {
    Ok(Json(ApiResponse {
        success: false,
        data: None,
        error: Some(ApiError {
            code: code.to_string(),
            message: message.to_string(),
            details: None,
        }),
        message: Some(message.to_string()),
    }))
}

// ==================== IMAGE GENERATION APIs ====================

/// Generate AI images from text prompts
#[derive(Deserialize, ToSchema)]
pub struct GenerateImageRequest {
    pub prompt: String,
    pub style: Option<String>,
    pub count: Option<u32>, // Number of images to generate (1-4)
}

#[derive(Serialize, ToSchema)]
pub struct GenerateImageResponse {
    pub images: Vec<GeneratedImage>,
    pub request_id: String,
}

#[derive(Serialize, ToSchema)]
pub struct GeneratedImage {
    pub id: String,
    pub url: String,
    pub prompt: String,
    pub style: Option<String>,
    pub created_at: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/images/generate",
    request_body = GenerateImageRequest,
    responses(
        (status = 200, description = "Images generated successfully", body = ApiResponse<GenerateImageResponse>),
        (status = 400, description = "Invalid request", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    tag = "images"
)]
pub async fn generate_images(
    State(state): State<super::AppState>,
    Json(req): Json<GenerateImageRequest>,
) -> Result<Json<ApiResponse<GenerateImageResponse>>, StatusCode> {
    // Validate input
    if req.prompt.trim().is_empty() {
        return error_response("INVALID_PROMPT", "Prompt cannot be empty");
    }

    if req.prompt.len() > 1000 {
        return error_response("PROMPT_TOO_LONG", "Prompt must be less than 1000 characters");
    }

    let count = req.count.unwrap_or(1).clamp(1, 4);

    let client = match &state.api_state.freepik_client {
        Some(client) => client,
        None => return error_response("SERVICE_UNAVAILABLE", "Image generation service is not available"),
    };

    let mut images = Vec::new();
    let request_id = format!("req_{}", chrono::Utc::now().timestamp());

    // Generate multiple images
    for i in 0..count {
        match client.generate_image(&req.prompt, req.style.as_deref()).await {
            Ok(response) => {
                let image = GeneratedImage {
                    id: format!("{}_{}", request_id, i),
                    url: response.image_url,
                    prompt: req.prompt.clone(),
                    style: req.style.clone(),
                    created_at: chrono::Utc::now().to_rfc3339(),
                };
                images.push(image);
            }
            Err(e) => {
                return error_response("GENERATION_FAILED", &format!("Failed to generate image: {}", e));
            }
        }
    }

    let response = GenerateImageResponse {
        images,
        request_id,
    };

    Ok(success_response(response))
}

// ==================== NFT MINTING APIs ====================

/// Mint an NFT with a pre-generated image
#[derive(Deserialize, ToSchema)]
pub struct MintNftRequest {
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub image_url: String,
    pub attributes: Option<Vec<NftAttribute>>,
    pub creator_address: String,
    pub signature: String,
    pub message: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct NftAttribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Serialize, ToSchema)]
pub struct MintNftResponse {
    pub nft_address: String,
    pub transaction_signature: String,
    pub image_short_url: String,
    pub metadata_url: String,
    pub fee_breakdown: nft::FeeBreakdown,
    pub minted_at: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/nfts/mint",
    request_body = MintNftRequest,
    responses(
        (status = 200, description = "NFT minted successfully", body = ApiResponse<MintNftResponse>),
        (status = 400, description = "Invalid request", body = ApiResponse<()>),
        (status = 401, description = "Unauthorized", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    tag = "nfts"
)]
pub async fn mint_nft(
    State(state): State<super::AppState>,
    Json(req): Json<MintNftRequest>,
) -> Result<Json<ApiResponse<MintNftResponse>>, StatusCode> {
    // Validate input
    if req.name.trim().is_empty() || req.symbol.trim().is_empty() {
        return error_response("INVALID_INPUT", "Name and symbol are required");
    }

    if req.name.len() > 32 || req.symbol.len() > 10 {
        return error_response("INVALID_INPUT", "Name max 32 chars, symbol max 10 chars");
    }

    // Create short URL for the image first
    let short_id = format!("{:x}", md5::compute(&req.image_url));
    let image_short_url = format!("http://localhost:3001/image/{}", short_id);

    // Convert to the backend MintNftRequest format
    let backend_req = nft::MintNftRequest {
        name: req.name,
        symbol: req.symbol,
        uri: image_short_url.clone(),
        creator_pubkey: req.creator_address,
        signature: req.signature,
        message: req.message,
        fee_payment_signature: None,
    };

    // Call the existing mint_nft function
    match nft::mint_nft(state.api_state.solana_client, &*state.api_state.keypair, backend_req, state.api_state.url_mappings).await {
        Ok(result) => {

            let response = MintNftResponse {
                nft_address: result.nft_address,
                transaction_signature: result.transaction_signature,
                image_short_url,
                metadata_url: format!("http://localhost:3001/image/{}", short_id), // Same as image for now
                fee_breakdown: result.fee_breakdown,
                minted_at: chrono::Utc::now().to_rfc3339(),
            };

            Ok(success_response(response))
        }
        Err(e) => error_response("MINT_FAILED", &e),
    }
}

// ==================== NFT MANAGEMENT APIs ====================

/// Get NFTs owned by a wallet
#[derive(Deserialize, ToSchema)]
pub struct GetWalletNftsRequest {
    pub wallet_address: String,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Serialize, ToSchema)]
pub struct GetWalletNftsResponse {
    pub nfts: Vec<NftInfo>,
    pub total_count: u32,
    pub limit: u32,
    pub offset: u32,
}

#[derive(Serialize, ToSchema)]
pub struct NftInfo {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub image_url: String,
    pub metadata_url: String,
    pub owner: String,
    pub created_at: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/v1/wallet/{address}/nfts",
    params(
        ("address" = String, Path, description = "Wallet address"),
        ("limit" = Option<u32>, Query, description = "Number of NFTs to return"),
        ("offset" = Option<u32>, Query, description = "Offset for pagination")
    ),
    responses(
        (status = 200, description = "NFTs retrieved successfully", body = ApiResponse<GetWalletNftsResponse>),
        (status = 400, description = "Invalid request", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    tag = "wallet"
)]
pub async fn get_wallet_nfts(
    State(state): State<super::AppState>,
    Path(wallet_address): Path<String>,
    Query(params): Query<GetWalletNftsRequest>,
) -> Result<Json<ApiResponse<GetWalletNftsResponse>>, StatusCode> {
    // For now, return empty list as we don't have on-chain NFT querying implemented
    let response = GetWalletNftsResponse {
        nfts: vec![],
        total_count: 0,
        limit: params.limit.unwrap_or(20),
        offset: params.offset.unwrap_or(0),
    };

    Ok(success_response(response))
}

// ==================== MARKETPLACE APIs ====================

/// List an NFT for sale
#[derive(Deserialize, ToSchema)]
pub struct ListNftRequest {
    pub nft_address: String,
    pub price: u64, // Price in lamports
    pub seller_address: String,
    pub signature: String,
    pub message: String,
}

#[derive(Serialize, ToSchema)]
pub struct ListNftResponse {
    pub listing_address: String,
    pub transaction_signature: String,
    pub listed_at: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/marketplace/list",
    request_body = ListNftRequest,
    responses(
        (status = 200, description = "NFT listed successfully", body = ApiResponse<ListNftResponse>),
        (status = 400, description = "Invalid request", body = ApiResponse<()>),
        (status = 401, description = "Unauthorized", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    tag = "marketplace"
)]
pub async fn list_nft(
    State(state): State<super::AppState>,
    Json(req): Json<ListNftRequest>,
) -> Result<Json<ApiResponse<ListNftResponse>>, StatusCode> {
    let nft_req = nft::ListNftRequest {
        nft_address: req.nft_address,
        price: req.price,
        seller_pubkey: req.seller_address,
    };

    match nft::list_nft(state.api_state.solana_client, &*state.api_state.keypair, nft_req).await {
        Ok(result) => {
            let response = ListNftResponse {
                listing_address: result["listing_address"].as_str().unwrap_or("").to_string(),
                transaction_signature: result["transaction_signature"].as_str().unwrap_or("").to_string(),
                listed_at: chrono::Utc::now().to_rfc3339(),
            };
            Ok(success_response(response))
        }
        Err(e) => error_response("LIST_FAILED", &e),
    }
}

/// Get marketplace listings
#[derive(Deserialize, ToSchema)]
pub struct GetListingsRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub sort_by: Option<String>, // "price_asc", "price_desc", "recent"
}

#[derive(Serialize, ToSchema)]
pub struct GetListingsResponse {
    pub listings: Vec<NftListing>,
    pub total_count: u32,
    pub limit: u32,
    pub offset: u32,
}

#[derive(Serialize, ToSchema)]
pub struct NftListing {
    pub listing_address: String,
    pub nft_address: String,
    pub name: String,
    pub image_url: String,
    pub price: u64,
    pub seller: String,
    pub listed_at: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/marketplace/listings",
    params(
        ("limit" = Option<u32>, Query, description = "Number of listings to return"),
        ("offset" = Option<u32>, Query, description = "Offset for pagination"),
        ("sort_by" = Option<String>, Query, description = "Sort order")
    ),
    responses(
        (status = 200, description = "Listings retrieved successfully", body = ApiResponse<GetListingsResponse>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    tag = "marketplace"
)]
pub async fn get_listings(
    State(_state): State<super::AppState>,
    Query(params): Query<GetListingsRequest>,
) -> Result<Json<ApiResponse<GetListingsResponse>>, StatusCode> {
    // For now, return empty list as we don't have on-chain listing querying implemented
    let response = GetListingsResponse {
        listings: vec![],
        total_count: 0,
        limit: params.limit.unwrap_or(20),
        offset: params.offset.unwrap_or(0),
    };

    Ok(success_response(response))
}

// ==================== UTILITY APIs ====================

/// Get fee estimates for operations
#[derive(Serialize, ToSchema)]
pub struct FeeEstimateResponse {
    pub mint_fee: nft::FeeBreakdown,
    pub list_fee: u64, // Estimated listing fee in lamports
    pub buy_fee: u64,  // Estimated buy fee in lamports
}

#[utoipa::path(
    get,
    path = "/api/v1/fees/estimate",
    responses(
        (status = 200, description = "Fee estimates retrieved successfully", body = ApiResponse<FeeEstimateResponse>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    tag = "utilities"
)]
pub async fn get_fee_estimates(
    State(state): State<super::AppState>,
) -> Result<Json<ApiResponse<FeeEstimateResponse>>, StatusCode> {
    match nft::get_fee_estimate(state.api_state.solana_client, &*state.api_state.keypair).await {
        Ok(fee_estimate) => {
            let response = FeeEstimateResponse {
                mint_fee: fee_estimate.fee_breakdown,
                list_fee: 5000, // Estimated transaction fee
                buy_fee: 5000,  // Estimated transaction fee
            };
            Ok(success_response(response))
        }
        Err(e) => error_response("FEE_ESTIMATE_FAILED", &e),
    }
}

/// Health check
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy", body = ApiResponse<HealthResponse>),
    ),
    tag = "utilities"
)]
pub async fn health_check() -> Json<ApiResponse<HealthResponse>> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    success_response(response)
}

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
}