use axum::{
    routing::{get, post},
    Router,
    extract::{State, Path, Query},
    Json,
    response::Response,
    http::{StatusCode, header},
};
use std::{net::SocketAddr, sync::Arc, collections::HashMap};
use tower_http::cors::CorsLayer;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

mod nft;
mod collection;
mod wallet;
mod freepik_api;
mod marketplace;
mod api;

use freepik_api::{FreepikApiClient, GenerateImageRequest, GenerateImageResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Legacy endpoints
        health_check,
        mint_nft_handler,
        generate_and_mint_nft_handler,
        get_fee_estimate_handler,
        create_collection_handler,
        list_nft_handler,
        buy_nft_handler,
        get_nfts_handler,
        generate_image_handler,
        image_proxy_handler,
        get_wallet_balance_handler,
        get_wallet_nfts_handler,
        get_marketplace_stats_handler,
        get_listed_nfts_handler,
        search_nfts_handler,
        get_nft_details_handler,
        // New v1 API endpoints
        api::generate_images,
        api::mint_nft,
        api::get_wallet_nfts,
        api::list_nft,
        api::get_listings,
        api::get_fee_estimates,
        api::health_check,
    ),
    components(
        schemas(
            // Legacy schemas
            nft::MintNftRequest,
            nft::GenerateAndMintNftRequest,
            nft::MintNftResponse,
            nft::FeeBreakdown,
            nft::FeeEstimateResponse,
            nft::ListNftRequest,
            nft::BuyNftRequest,
            collection::CreateCollectionRequest,
            GenerateImageRequest,
            GenerateImageResponse,
            wallet::WalletBalanceRequest,
            wallet::WalletBalanceResponse,
            wallet::WalletNftsRequest,
            wallet::WalletNftsResponse,
            marketplace::MarketplaceStatsResponse,
            marketplace::GetListedNftsResponse,
            marketplace::SearchNftsRequest,
            marketplace::NftDetailsRequest,
            marketplace::NftDetailsResponse,
            // New v1 API schemas
            api::ApiResponse<api::GenerateImageResponse>,
            api::ApiResponse<api::MintNftResponse>,
            api::ApiResponse<api::GetWalletNftsResponse>,
            api::ApiResponse<api::ListNftResponse>,
            api::ApiResponse<api::GetListingsResponse>,
            api::ApiResponse<api::FeeEstimateResponse>,
            api::ApiResponse<api::HealthResponse>,
            api::GenerateImageRequest,
            api::GenerateImageResponse,
            api::GeneratedImage,
            api::MintNftRequest,
            api::NftAttribute,
            api::MintNftResponse,
            api::GetWalletNftsRequest,
            api::GetWalletNftsResponse,
            api::NftInfo,
            api::ListNftRequest,
            api::ListNftResponse,
            api::GetListingsRequest,
            api::GetListingsResponse,
            api::NftListing,
            api::FeeEstimateResponse,
            api::HealthResponse,
            api::ApiError,
        )
    ),
    tags(
        (name = "nft", description = "NFT operations"),
        (name = "wallet", description = "Wallet operations"),
        (name = "marketplace", description = "Marketplace operations"),
        (name = "image", description = "Image generation operations"),
        (name = "images", description = "AI image generation operations"),
        (name = "utilities", description = "Utility endpoints"),
    )
)]
struct ApiDoc;

#[derive(Clone)]
struct AppState {
    solana_client: Arc<solana_client::rpc_client::RpcClient>,
    freepik_client: Option<FreepikApiClient>,
    keypair: Arc<solana_sdk::signature::Keypair>,
    url_mappings: Arc<tokio::sync::RwLock<HashMap<String, String>>>,
    api_state: api::ApiState,
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    let dotenv_path = dotenv::dotenv().expect("Failed to load .env");
    println!("Loaded .env from {}", dotenv_path.display());

    // Validate required environment variables
    let rpc_url = std::env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());

    let private_key_str = std::env::var("SOLANA_PRIVATE_KEY")
        .expect("SOLANA_PRIVATE_KEY environment variable must be set");

    // Parse comma-separated bytes
    let bytes: Vec<u8> = private_key_str.split(',')
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect();

    let keypair = solana_sdk::signature::Keypair::from_bytes(&bytes)
        .expect("Invalid keypair bytes");

    // Initialize Solana client
    let solana_client = Arc::new(solana_client::rpc_client::RpcClient::new(rpc_url));

    // Initialize Freepik client
    let freepik_client = std::env::var("FREEPIK_API_KEY")
        .ok()
        .map(FreepikApiClient::new);

    let keypair_arc = Arc::new(keypair);

    let api_state = api::ApiState {
        solana_client: solana_client.clone(),
        freepik_client: freepik_client.clone(),
        keypair: keypair_arc.clone(),
        url_mappings: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
    };

    let state = AppState {
        solana_client,
        freepik_client,
        keypair: keypair_arc,
        url_mappings: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        api_state,
    };

    let app = Router::new()
        // Legacy endpoints (keeping for backward compatibility)
        .route("/", get(health_check))
        .route("/mint-nft", post(mint_nft_handler))
        .route("/generate-and-mint-nft", post(generate_and_mint_nft_handler))
        .route("/fee-estimate", get(get_fee_estimate_handler))
        .route("/create-collection", post(create_collection_handler))
        .route("/list-nft", post(list_nft_handler))
        .route("/buy-nft", post(buy_nft_handler))
        .route("/nfts", get(get_nfts_handler))
        .route("/generate-image", post(generate_image_handler))
        .route("/image/:id", get(image_proxy_handler))
        .route("/debug/url-mappings", get(debug_url_mappings_handler))
        // Wallet endpoints
        .route("/wallet/balance", post(get_wallet_balance_handler))
        .route("/wallet/nfts", post(get_wallet_nfts_handler))
        // Marketplace endpoints
        .route("/marketplace/stats", get(get_marketplace_stats_handler))
        .route("/marketplace/listings", get(get_listed_nfts_handler))
        .route("/marketplace/search", post(search_nfts_handler))
        .route("/marketplace/nft/:address", get(get_nft_details_handler))
        // New v1 API endpoints
        .route("/api/v1/images/generate", post(api::generate_images))
        .route("/api/v1/nfts/mint", post(api::mint_nft))
        .route("/api/v1/wallet/:address/nfts", get(api::get_wallet_nfts))
        .route("/api/v1/marketplace/list", post(api::list_nft))
        .route("/api/v1/marketplace/listings", get(api::get_listings))
        .route("/api/v1/fees/estimate", get(api::get_fee_estimates))
        .route("/api/v1/health", get(api::health_check))
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Get port from environment variable (Render provides PORT)
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>()
        .unwrap_or(3001);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("NFT Marketplace API server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Health check successful", body = String)
    ),
    tag = "health"
)]
async fn health_check() -> &'static str {
    "NFT Marketplace API - Healthy"
}

#[utoipa::path(
    post,
    path = "/mint-nft",
    request_body = nft::MintNftRequest,
    responses(
        (status = 200, description = "NFT minted successfully", body = nft::MintNftResponse)
    ),
    tag = "nft"
)]
async fn mint_nft_handler(
    State(state): State<AppState>,
    Json(req): Json<nft::MintNftRequest>,
) -> Result<Json<nft::MintNftResponse>, String> {
    let result = nft::mint_nft(state.solana_client, &*state.keypair, req, state.url_mappings.clone()).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/create-collection",
    request_body = collection::CreateCollectionRequest,
    responses(
        (status = 200, description = "Collection created successfully", body = serde_json::Value)
    ),
    tag = "nft"
)]
async fn create_collection_handler(
    State(state): State<AppState>,
    Json(req): Json<collection::CreateCollectionRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let result = collection::create_collection(state.solana_client, req).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/list-nft",
    request_body = nft::ListNftRequest,
    responses(
        (status = 200, description = "NFT listed successfully", body = serde_json::Value)
    ),
    tag = "nft"
)]
async fn list_nft_handler(
    State(state): State<AppState>,
    Json(req): Json<nft::ListNftRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let result = nft::list_nft(state.solana_client, &*state.keypair, req).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/buy-nft",
    request_body = nft::BuyNftRequest,
    responses(
        (status = 200, description = "NFT purchased successfully", body = serde_json::Value)
    ),
    tag = "nft"
)]
async fn buy_nft_handler(
    State(state): State<AppState>,
    Json(req): Json<nft::BuyNftRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let result = nft::buy_nft(state.solana_client, &*state.keypair, req).await?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/nfts",
    responses(
        (status = 200, description = "NFTs retrieved successfully", body = Vec<serde_json::Value>)
    ),
    tag = "nft"
)]
async fn get_nfts_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<serde_json::Value>>, String> {
    let result = nft::get_nfts(state.solana_client).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/generate-and-mint-nft",
    request_body = nft::GenerateAndMintNftRequest,
    responses(
        (status = 200, description = "NFT generated and minted successfully", body = nft::MintNftResponse)
    ),
    tag = "nft"
)]
async fn generate_and_mint_nft_handler(
    State(state): State<AppState>,
    Json(req): Json<nft::GenerateAndMintNftRequest>,
) -> Result<Json<nft::MintNftResponse>, String> {
    let result = nft::generate_and_mint_nft(
        state.solana_client, 
        &*state.keypair, 
        state.freepik_client.as_ref(), 
        state.url_mappings.clone(),
        req
    ).await?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/fee-estimate",
    responses(
        (status = 200, description = "Fee estimate retrieved successfully", body = nft::FeeEstimateResponse)
    ),
    tag = "nft"
)]
async fn get_fee_estimate_handler(
    State(state): State<AppState>,
) -> Result<Json<nft::FeeEstimateResponse>, String> {
    let result = nft::get_fee_estimate(state.solana_client, &*state.keypair).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/generate-image",
    request_body = GenerateImageRequest,
    responses(
        (status = 200, description = "Image generated successfully", body = GenerateImageResponse)
    ),
    tag = "image"
)]
async fn generate_image_handler(
    State(state): State<AppState>,
    Json(req): Json<GenerateImageRequest>,
) -> Result<Json<GenerateImageResponse>, String> {
    let client = state.freepik_client
        .ok_or("Freepik API not configured")?;

    client.generate_image(&req.prompt, req.style.as_deref())
        .await
        .map(Json)
        .map_err(|e| format!("Image generation failed: {}", e))
}

#[utoipa::path(
    get,
    path = "/image/{id}",
    params(
        ("id" = String, Path, description = "Image ID")
    ),
    responses(
        (status = 200, description = "Image retrieved successfully"),
        (status = 404, description = "Image not found")
    ),
    tag = "image"
)]
async fn image_proxy_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response, StatusCode> {
    // Get the original URL from the mapping
    let original_url = {
        let mappings = state.url_mappings.read().await;
        mappings.get(&id).cloned()
    };

    let original_url = match original_url {
        Some(url) => url,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // Fetch the image from the original URL
    let client = reqwest::Client::new();
    let response = match client.get(&original_url).send().await {
        Ok(resp) => resp,
        Err(_) => return Err(StatusCode::BAD_GATEWAY),
    };

    if !response.status().is_success() {
        return Err(StatusCode::BAD_GATEWAY);
    }

    let content_type = response.headers()
        .get("content-type")
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or("image/png")
        .to_string();

    let body = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CACHE_CONTROL, "public, max-age=3600")
        .body(body.into())
        .unwrap())
}

async fn debug_url_mappings_handler(
    State(state): State<AppState>,
) -> Json<HashMap<String, String>> {
    let mappings = state.url_mappings.read().await;
    Json(mappings.clone())
}

// Wallet handlers
#[utoipa::path(
    post,
    path = "/wallet/balance",
    request_body = wallet::WalletBalanceRequest,
    responses(
        (status = 200, description = "Wallet balance retrieved successfully", body = wallet::WalletBalanceResponse)
    ),
    tag = "wallet"
)]
async fn get_wallet_balance_handler(
    State(state): State<AppState>,
    Json(req): Json<wallet::WalletBalanceRequest>,
) -> Result<Json<wallet::WalletBalanceResponse>, StatusCode> {
    match wallet::get_wallet_balance(state.solana_client.clone(), &req.wallet_address).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/wallet/nfts",
    request_body = wallet::WalletNftsRequest,
    responses(
        (status = 200, description = "Wallet NFTs retrieved successfully", body = wallet::WalletNftsResponse)
    ),
    tag = "wallet"
)]
async fn get_wallet_nfts_handler(
    State(state): State<AppState>,
    Json(req): Json<wallet::WalletNftsRequest>,
) -> Result<Json<wallet::WalletNftsResponse>, StatusCode> {
    match wallet::get_wallet_nfts(state.solana_client.clone(), &req.wallet_address).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Marketplace handlers
#[utoipa::path(
    get,
    path = "/marketplace/stats",
    responses(
        (status = 200, description = "Marketplace stats retrieved successfully", body = marketplace::MarketplaceStatsResponse)
    ),
    tag = "marketplace"
)]
async fn get_marketplace_stats_handler(
    State(state): State<AppState>,
) -> Result<Json<marketplace::MarketplaceStatsResponse>, StatusCode> {
    match marketplace::get_marketplace_stats(state.solana_client.clone()).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    get,
    path = "/marketplace/listings",
    responses(
        (status = 200, description = "Listed NFTs retrieved successfully", body = marketplace::GetListedNftsResponse)
    ),
    tag = "marketplace"
)]
async fn get_listed_nfts_handler(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<marketplace::GetListedNftsResponse>, StatusCode> {
    let page = params.get("page").and_then(|p| p.parse().ok()).unwrap_or(1);
    let per_page = params.get("per_page").and_then(|p| p.parse().ok()).unwrap_or(20);
    
    match marketplace::get_listed_nfts(state.solana_client.clone(), page, per_page).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/marketplace/search",
    request_body = marketplace::SearchNftsRequest,
    responses(
        (status = 200, description = "NFT search completed successfully", body = marketplace::GetListedNftsResponse)
    ),
    tag = "marketplace"
)]
async fn search_nfts_handler(
    State(state): State<AppState>,
    Json(req): Json<marketplace::SearchNftsRequest>,
) -> Result<Json<marketplace::GetListedNftsResponse>, StatusCode> {
    match marketplace::search_nfts(state.solana_client.clone(), req).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    get,
    path = "/marketplace/nft/{address}",
    params(
        ("address" = String, Path, description = "NFT address")
    ),
    responses(
        (status = 200, description = "NFT details retrieved successfully", body = marketplace::NftDetailsResponse)
    ),
    tag = "marketplace"
)]
async fn get_nft_details_handler(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Result<Json<marketplace::NftDetailsResponse>, StatusCode> {
    match marketplace::get_nft_details(state.solana_client.clone(), &address).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}