use anyhow::{Context as _, Result};
use cloud_llm_client::predict_edits_v3::{RawCompletionRequest, RawCompletionResponse};
use futures::AsyncReadExt as _;
use gpui::{App, AppContext as _, Entity, Global, SharedString, Task, http_client};
use language::language_settings::{OpenAiCompatibleEditPredictionSettings, all_language_settings};
use language_model::{ApiKeyState, EnvVar, env_var};
use std::sync::Arc;

pub fn open_ai_compatible_api_url(cx: &App) -> SharedString {
    all_language_settings(None, cx)
        .edit_predictions
        .open_ai_compatible_api
        .as_ref()
        .map(|settings| settings.api_url.clone())
        .unwrap_or_default()
        .into()
}

pub const OPEN_AI_COMPATIBLE_CREDENTIALS_USERNAME: &str = "openai-compatible-api-token";
pub static OPEN_AI_COMPATIBLE_TOKEN_ENV_VAR: std::sync::LazyLock<EnvVar> =
    env_var!("ZED_OPEN_AI_COMPATIBLE_EDIT_PREDICTION_API_KEY");

struct GlobalOpenAiCompatibleApiKey(Entity<ApiKeyState>);

impl Global for GlobalOpenAiCompatibleApiKey {}

pub fn open_ai_compatible_api_token(cx: &mut App) -> Entity<ApiKeyState> {
    if let Some(global) = cx.try_global::<GlobalOpenAiCompatibleApiKey>() {
        return global.0.clone();
    }

    let entity = cx.new(|cx| {
        ApiKeyState::new(
            open_ai_compatible_api_url(cx),
            OPEN_AI_COMPATIBLE_TOKEN_ENV_VAR.clone(),
        )
    });
    cx.set_global(GlobalOpenAiCompatibleApiKey(entity.clone()));
    entity
}

pub fn load_open_ai_compatible_api_token(
    cx: &mut App,
) -> Task<Result<(), language_model::AuthenticateError>> {
    let credentials_provider = zed_credentials_provider::global(cx);
    let api_url = open_ai_compatible_api_url(cx);
    open_ai_compatible_api_token(cx).update(cx, |key_state, cx| {
        key_state.load_if_needed(api_url, |s| s, credentials_provider, cx)
    })
}

pub fn load_open_ai_compatible_api_key_if_needed(
    provider: settings::EditPredictionProvider,
    cx: &mut App,
) -> Option<Arc<str>> {
    if provider != settings::EditPredictionProvider::OpenAiCompatibleApi {
        return None;
    }
    _ = load_open_ai_compatible_api_token(cx);
    let url = open_ai_compatible_api_url(cx);
    return open_ai_compatible_api_token(cx).read(cx).key(&url);
}

pub(crate) async fn send_custom_server_request(
    provider: settings::EditPredictionProvider,
    settings: &OpenAiCompatibleEditPredictionSettings,
    prompt: String,
    max_tokens: u32,
    stop_tokens: Vec<String>,
    api_key: Option<Arc<str>>,
    http_client: &Arc<dyn http_client::HttpClient>,
) -> Result<(String, String)> {
    match provider {
        settings::EditPredictionProvider::Ollama => {
            let response = crate::ollama::make_request(
                settings.clone(),
                prompt,
                stop_tokens,
                http_client.clone(),
            )
            .await?;
            Ok((response.response, response.created_at))
        }
        _ => {
            let request = RawCompletionRequest {
                model: settings.model.clone(),
                prompt,
                max_tokens: Some(max_tokens),
                temperature: None,
                stop: stop_tokens
                    .into_iter()
                    .map(std::borrow::Cow::Owned)
                    .collect(),
                environment: None,
            };

            let request_body = serde_json::to_string(&request)?;
            let mut http_request_builder = http_client::Request::builder()
                .method(http_client::Method::POST)
                .uri(settings.api_url.as_ref())
                .header("Content-Type", "application/json");

            if let Some(api_key) = api_key {
                http_request_builder =
                    http_request_builder.header("Authorization", format!("Bearer {}", api_key));
            }

            let http_request =
                http_request_builder.body(http_client::AsyncBody::from(request_body))?;

            let mut response = http_client.send(http_request).await?;
            let status = response.status();

            if !status.is_success() {
                let mut body = String::new();
                response.body_mut().read_to_string(&mut body).await?;
                anyhow::bail!("custom server error: {} - {}", status, body);
            }

            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;

            let parsed: RawCompletionResponse =
                serde_json::from_str(&body).context("Failed to parse completion response")?;
            let text = parsed
                .choices
                .into_iter()
                .next()
                .map(|choice| choice.text)
                .unwrap_or_default();
            Ok((text, parsed.id))
        }
    }
}
