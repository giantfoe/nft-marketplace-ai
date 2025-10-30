use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct GenerateImageRequest {
    pub prompt: String,
    pub style: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct GenerateImageResponse {
    pub image_url: String,
    pub image_data: Option<String>, // Base64 if available
}

#[derive(Deserialize)]
struct FreepikResponse {
    data: TaskDetail,
}

#[derive(Deserialize)]
struct TaskDetail {
    generated: Vec<String>,
    task_id: String,
    status: String,
}

#[derive(Clone)]
pub struct FreepikApiClient {
    client: Client,
    api_key: String,
}

impl FreepikApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn generate_image(
        &self,
        prompt: &str,
        style: Option<&str>,
    ) -> Result<GenerateImageResponse, Box<dyn Error>> {
        if prompt.is_empty() || prompt.len() > 500 {
            return Err("Prompt must be 1-500 characters".into());
        }

        let full_prompt = match style {
            Some(s) => format!("{} in {} style", prompt, s),
            None => prompt.to_string(),
        };

        let response = self.client
            .post("https://api.freepik.com/v1/ai/mystic")
            .header("x-freepik-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "prompt": full_prompt
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("Freepik API error: {} - {}", status, text).into());
        }

        let mut freepik_resp: FreepikResponse = response.json().await?;

        // Poll until completed
        let task_id = freepik_resp.data.task_id.clone();
        loop {
            if freepik_resp.data.status == "COMPLETED" {
                if freepik_resp.data.generated.is_empty() {
                    return Err("No image generated".into());
                }
                return Ok(GenerateImageResponse {
                    image_url: freepik_resp.data.generated[0].clone(),
                    image_data: None,
                });
            } else if freepik_resp.data.status == "FAILED" {
                return Err("Image generation failed".into());
            }

            // Wait and poll
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            let poll_response = self.client
                .get(&format!("https://api.freepik.com/v1/ai/mystic/{}", task_id))
                .header("x-freepik-api-key", &self.api_key)
                .send()
                .await?;

            if !poll_response.status().is_success() {
                return Err(format!("Poll failed: {}", poll_response.status()).into());
            }

            freepik_resp = poll_response.json().await?;
        }
    }
}