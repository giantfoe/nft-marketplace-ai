use axum::{routing::{get, post}, Router, Json};
use shared::{GenerateImageRequest, GenerateImageResponse, MintNftRequest, MintNftResponse};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let http_client = reqwest::Client::new();

    let app = Router::new()
        .route("/", get(health_check))
        .route("/generate-image", post(generate_image))
        .route("/mint-nft", post(mint_nft))
        .layer(CorsLayer::permissive())
        .with_state(http_client);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("API gateway listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "API Gateway - Healthy"
}

async fn generate_image(
    axum::extract::State(client): axum::extract::State<reqwest::Client>,
    Json(req): Json<GenerateImageRequest>,
) -> Result<Json<GenerateImageResponse>, String> {
    let response = client
        .post("http://localhost:3002/generate")
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Image service error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Image service returned {}", response.status()));
    }

    response.json().await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

async fn mint_nft(
    axum::extract::State(_client): axum::extract::State<reqwest::Client>,
    Json(_req): Json<MintNftRequest>,
) -> Result<Json<MintNftResponse>, String> {
    // TODO: Call contract service
    Ok(Json(MintNftResponse {
        nft_address: "placeholder".to_string(),
        transaction_signature: "placeholder".to_string(),
    }))
}