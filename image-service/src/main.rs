use axum::{routing::post, Router, Json};
use shared::{GenerateImageRequest, GenerateImageResponse};
use std::net::SocketAddr;

mod freepik_client;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let freepik_api_key = std::env::var("FREEPIK_API_KEY")
        .expect("FREEPIK_API_KEY required");

    let client = freepik_client::FreepikClient::new(freepik_api_key);

    let app = Router::new()
        .route("/generate", post(generate_image))
        .with_state(client);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    println!("Image service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn generate_image(
    axum::extract::State(client): axum::extract::State<freepik_client::FreepikClient>,
    Json(req): Json<GenerateImageRequest>,
) -> Result<Json<GenerateImageResponse>, String> {
    client.generate_image(&req.prompt, req.style.as_deref())
        .await
        .map(Json)
        .map_err(|e| format!("Image generation failed: {}", e))
}