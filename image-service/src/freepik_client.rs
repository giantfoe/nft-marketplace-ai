use reqwest::Client;
use serde::{Deserialize, Serialize};
use shared::GenerateImageResponse;
use std::error::Error;

#[derive(Deserialize)]
struct FreepikResponse {
    #[serde(default)]
    data: Vec<FreepikImage>,
    #[serde(default)]
    images: Vec<FreepikImage>,
}

#[derive(Deserialize)]
struct FreepikImage {
    url: Option<String>,
    image_url: Option<String>,
}

#[derive(Clone)]
pub struct FreepikClient {
    client: Client,
    api_key: String,
}

impl FreepikClient {
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

        // Try the correct Freepik API endpoint
        let response = self.client
            .post("https://api.freepik.com/v1/ai/text-to-image")
            .header("x-freepik-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "prompt": full_prompt,
                "negative_prompt": "",
                "style": style.unwrap_or("default"),
                "image_size": "1024x1024",
                "num_images": 1
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Freepik API error: {} - {}", status, body).into());
        }

        let response_text = response.text().await?;
        eprintln!("Freepik API response: {}", &response_text[..std::cmp::min(500, response_text.len())]);

        // Try to parse the response - be flexible with the format
        let image_url = if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&response_text) {
            // Try different possible paths for the image URL
            if let Some(url) = parsed.get("data").and_then(|d| d.as_array()).and_then(|arr| arr.get(0)).and_then(|img| img.get("url")).and_then(|u| u.as_str()) {
                url.to_string()
            } else if let Some(url) = parsed.get("images").and_then(|d| d.as_array()).and_then(|arr| arr.get(0)).and_then(|img| img.get("url")).and_then(|u| u.as_str()) {
                url.to_string()
            } else if let Some(url) = parsed.get("url").and_then(|u| u.as_str()) {
                url.to_string()
            } else if let Some(url) = parsed.get("image_url").and_then(|u| u.as_str()) {
                url.to_string()
            } else {
                return Err(format!("No image URL found in response: {}", response_text).into());
            }
        } else {
            return Err(format!("Invalid JSON response: {}", response_text).into());
        };

        Ok(GenerateImageResponse {
            image_url,
            image_data: None,
        })
    }
}