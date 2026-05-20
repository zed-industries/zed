//! OpenAI-compatible HTTP client for local Atomic Chat servers.
//!
//! See <https://atomic.chat/> for the upstream application.

use anyhow::{Context as _, Result};
use futures::AsyncReadExt;
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::Deserialize;

/// Default base URL for the Atomic Chat OpenAI-compatible API (`/v1` prefix included).
pub const ATOMIC_CHAT_API_URL: &str = "http://localhost:1337/v1";

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Model {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: u64,
    pub supports_tool_calls: bool,
    pub supports_images: bool,
}

impl Model {
    pub fn new(
        name: &str,
        display_name: Option<&str>,
        max_tokens: Option<u64>,
        supports_tool_calls: bool,
        supports_images: bool,
    ) -> Self {
        Self {
            name: name.to_owned(),
            display_name: display_name.map(str::to_owned),
            max_tokens: max_tokens.unwrap_or(32_768),
            supports_tool_calls,
            supports_images,
        }
    }

    pub fn id(&self) -> &str {
        &self.name
    }

    pub fn display_name(&self) -> &str {
        self.display_name.as_deref().unwrap_or(&self.name)
    }

    pub fn max_token_count(&self) -> u64 {
        self.max_tokens
    }

    pub fn supports_tool_calls(&self) -> bool {
        self.supports_tool_calls
    }
}

#[derive(Deserialize)]
struct ListModelsResponse {
    data: Vec<ModelEntry>,
}

#[derive(Deserialize)]
struct ModelEntry {
    id: String,
}

/// Fetches models from `GET {api_url}/models` (OpenAI-compatible list format).
pub async fn get_models(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: Option<&str>,
) -> Result<Vec<Model>> {
    let uri = format!("{api_url}/models");
    let mut request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json");

    if let Some(api_key) = api_key {
        if !api_key.is_empty() {
            request_builder =
                request_builder.header("Authorization", format!("Bearer {}", api_key.trim()));
        }
    }

    let request = request_builder.body(AsyncBody::default())?;
    let mut response = client.send(request).await?;

    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    anyhow::ensure!(
        response.status().is_success(),
        "Failed to connect to Atomic Chat API: {} {}",
        response.status(),
        body,
    );

    let parsed: ListModelsResponse =
        serde_json::from_str(&body).context("Unable to parse Atomic Chat models response")?;

    let mut models: Vec<Model> = parsed
        .data
        .into_iter()
        .filter(|entry| {
            let id = entry.id.to_ascii_lowercase();
            !id.contains("embedding") && !id.contains("embed")
        })
        .map(|entry| {
            Model::new(
                &entry.id,
                None,
                None,
                true,
                true, // opt into vision/tool assumptions; users can override in settings
            )
        })
        .collect();

    models.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(models)
}
