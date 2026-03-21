use anyhow::{Context as _, Result};
use futures::AsyncReadExt as _;
use gpui::{
    App, SharedString,
    http_client::{self, HttpClient},
};
use language::language_settings::OpenAiCompatibleEditPredictionSettings;
use language_model::{LanguageModelProviderId, LanguageModelRegistry};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub(crate) struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    raw: bool,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaGenerateOptions>,
}

#[derive(Debug, Serialize)]
pub(crate) struct OllamaGenerateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OllamaGenerateResponse {
    pub created_at: String,
    pub response: String,
}

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("ollama");

pub fn is_available(cx: &App) -> bool {
    LanguageModelRegistry::read_global(cx)
        .provider(&PROVIDER_ID)
        .is_some_and(|provider| provider.is_authenticated(cx))
}

pub fn ensure_authenticated(cx: &mut App) {
    if let Some(provider) = LanguageModelRegistry::read_global(cx).provider(&PROVIDER_ID) {
        provider.authenticate(cx).detach_and_log_err(cx);
    }
}

pub fn fetch_models(cx: &mut App) -> Vec<SharedString> {
    let Some(provider) = LanguageModelRegistry::read_global(cx).provider(&PROVIDER_ID) else {
        return Vec::new();
    };
    provider.authenticate(cx).detach_and_log_err(cx);
    let mut models: Vec<SharedString> = provider
        .provided_models(cx)
        .into_iter()
        .map(|model| SharedString::from(model.id().0.to_string()))
        .collect();
    models.sort();
    models
}

pub(crate) async fn make_request(
    settings: OpenAiCompatibleEditPredictionSettings,
    prompt: String,
    stop_tokens: Vec<String>,
    http_client: Arc<dyn HttpClient>,
) -> Result<OllamaGenerateResponse> {
    let request = OllamaGenerateRequest {
        model: settings.model.clone(),
        prompt,
        raw: true,
        stream: false,
        options: Some(OllamaGenerateOptions {
            num_predict: Some(settings.max_output_tokens),
            temperature: Some(0.2),
            stop: Some(stop_tokens),
        }),
    };

    let request_body = serde_json::to_string(&request)?;
    let http_request = http_client::Request::builder()
        .method(http_client::Method::POST)
        .uri(format!("{}/api/generate", settings.api_url))
        .header("Content-Type", "application/json")
        .body(http_client::AsyncBody::from(request_body))?;

    let mut response = http_client.send(http_request).await?;
    let status = response.status();

    log::debug!("Ollama: Response status: {}", status);

    if !status.is_success() {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        return Err(anyhow::anyhow!("Ollama API error: {} - {}", status, body));
    }

    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    let ollama_response: OllamaGenerateResponse =
        serde_json::from_str(&body).context("Failed to parse Ollama response")?;
    Ok(ollama_response)
}
