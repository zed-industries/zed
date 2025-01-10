use anyhow::{anyhow, Result};
use futures::AsyncReadExt;
use http_client::{http::HeaderMap, AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};

pub const FIREWORKS_API_URL: &str = "https://api.openai.com/v1";

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: String,
    pub max_tokens: u32,
    pub temperature: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prediction: Option<Prediction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite_speculation: Option<bool>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Prediction {
    Content { content: String },
}

#[derive(Debug)]
pub struct Response {
    pub completion: CompletionResponse,
    pub headers: Headers,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionChoice {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Headers {
    pub server_processing_time: Option<f64>,
    pub request_id: Option<String>,
    pub prompt_tokens: Option<u32>,
    pub speculation_generated_tokens: Option<u32>,
    pub cached_prompt_tokens: Option<u32>,
    pub backend_host: Option<String>,
    pub num_concurrent_requests: Option<u32>,
    pub deployment: Option<String>,
    pub tokenizer_queue_duration: Option<f64>,
    pub tokenizer_duration: Option<f64>,
    pub prefill_queue_duration: Option<f64>,
    pub prefill_duration: Option<f64>,
    pub generation_queue_duration: Option<f64>,
}

impl Headers {
    pub fn parse(headers: &HeaderMap) -> Self {
        Headers {
            request_id: headers
                .get("x-request-id")
                .and_then(|v| v.to_str().ok())
                .map(String::from),
            server_processing_time: headers
                .get("fireworks-server-processing-time")
                .and_then(|v| v.to_str().ok()?.parse().ok()),
            prompt_tokens: headers
                .get("fireworks-prompt-tokens")
                .and_then(|v| v.to_str().ok()?.parse().ok()),
            speculation_generated_tokens: headers
                .get("fireworks-speculation-generated-tokens")
                .and_then(|v| v.to_str().ok()?.parse().ok()),
            cached_prompt_tokens: headers
                .get("fireworks-cached-prompt-tokens")
                .and_then(|v| v.to_str().ok()?.parse().ok()),
            backend_host: headers
                .get("fireworks-backend-host")
                .and_then(|v| v.to_str().ok())
                .map(String::from),
            num_concurrent_requests: headers
                .get("fireworks-num-concurrent-requests")
                .and_then(|v| v.to_str().ok()?.parse().ok()),
            deployment: headers
                .get("fireworks-deployment")
                .and_then(|v| v.to_str().ok())
                .map(String::from),
            tokenizer_queue_duration: headers
                .get("fireworks-tokenizer-queue-duration")
                .and_then(|v| v.to_str().ok()?.parse().ok()),
            tokenizer_duration: headers
                .get("fireworks-tokenizer-duration")
                .and_then(|v| v.to_str().ok()?.parse().ok()),
            prefill_queue_duration: headers
                .get("fireworks-prefill-queue-duration")
                .and_then(|v| v.to_str().ok()?.parse().ok()),
            prefill_duration: headers
                .get("fireworks-prefill-duration")
                .and_then(|v| v.to_str().ok()?.parse().ok()),
            generation_queue_duration: headers
                .get("fireworks-generation-queue-duration")
                .and_then(|v| v.to_str().ok()?.parse().ok()),
        }
    }
}

pub async fn complete(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: CompletionRequest,
) -> Result<Response> {
    let uri = format!("{api_url}/completions");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key));

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;

    if response.status().is_success() {
        let headers = Headers::parse(response.headers());

        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        Ok(Response {
            completion: serde_json::from_str(&body)?,
            headers,
        })
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        #[derive(Deserialize)]
        struct FireworksResponse {
            error: FireworksError,
        }

        #[derive(Deserialize)]
        struct FireworksError {
            message: String,
        }

        match serde_json::from_str::<FireworksResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => Err(anyhow!(
                "Failed to connect to Fireworks API: {}",
                response.error.message,
            )),

            _ => Err(anyhow!(
                "Failed to connect to Fireworks API: {} {}",
                response.status(),
                body,
            )),
        }
    }
}
