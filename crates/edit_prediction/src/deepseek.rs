use anyhow::{Context as _, Result};
use cloud_llm_client::predict_edits_v3::RawCompletionResponse;
use futures::AsyncReadExt as _;
use gpui::{App, AppContext as _, Entity, Global, SharedString, Task, http_client};
use language::language_settings::{OpenAiCompatibleEditPredictionSettings, all_language_settings};
use language_model::{ApiKeyState, AuthenticateError, EnvVar, env_var};
use serde::Serialize;
use std::sync::Arc;

pub static DEEPSEEK_API_KEY_ENV_VAR: std::sync::LazyLock<EnvVar> = env_var!("DEEPSEEK_API_KEY");

/// The configured Deepseek completions endpoint, used as the credential storage key.
pub fn deepseek_api_url(cx: &App) -> SharedString {
    all_language_settings(None, cx)
        .edit_predictions
        .deepseek
        .as_ref()
        .map(|settings| settings.api_url.clone())
        .unwrap_or_default()
        .into()
}

struct GlobalDeepseekApiKey(Entity<ApiKeyState>);

impl Global for GlobalDeepseekApiKey {}

/// Returns the shared [`ApiKeyState`] for Deepseek, creating it on first use. The
/// state resolves the key from the `DEEPSEEK_API_KEY` environment variable or,
/// failing that, the credential store (where keys entered in settings are saved).
pub fn deepseek_api_token(cx: &mut App) -> Entity<ApiKeyState> {
    if let Some(global) = cx.try_global::<GlobalDeepseekApiKey>() {
        return global.0.clone();
    }
    let entity =
        cx.new(|cx| ApiKeyState::new(deepseek_api_url(cx), DEEPSEEK_API_KEY_ENV_VAR.clone()));
    cx.set_global(GlobalDeepseekApiKey(entity.clone()));
    entity
}

pub fn load_deepseek_api_token(cx: &mut App) -> Task<Result<(), AuthenticateError>> {
    let credentials_provider = zed_credentials_provider::global(cx);
    let api_url = deepseek_api_url(cx);
    deepseek_api_token(cx).update(cx, |key_state, cx| {
        key_state.load_if_needed(api_url, |s| s, credentials_provider, cx)
    })
}

/// Kicks off a credential load if needed and returns the currently known key.
pub fn load_deepseek_api_key_if_needed(cx: &mut App) -> Option<Arc<str>> {
    _ = load_deepseek_api_token(cx);
    let url = deepseek_api_url(cx);
    deepseek_api_token(cx).read(cx).key(&url)
}

/// Returns the Deepseek API key if one is already resolved, without triggering a load.
pub fn deepseek_api_key(cx: &App) -> Option<Arc<str>> {
    let url = deepseek_api_url(cx);
    cx.try_global::<GlobalDeepseekApiKey>()?.0.read(cx).key(&url)
}

#[derive(Debug, Serialize)]
struct DeepseekCompletionRequest {
    model: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

/// Sends a fill-in-the-middle request to Deepseek's beta completion API.
///
/// Unlike the other FIM providers, Deepseek accepts the prefix (`prompt`) and
/// `suffix` as separate fields and assembles the FIM prompt server-side, so no
/// FIM sentinel tokens are formatted here. `settings.api_url` is used verbatim
/// as the completions endpoint (e.g. `https://api.deepseek.com/beta/completions`,
/// see https://api-docs.deepseek.com/guides/fim_completion).
pub(crate) async fn make_request(
    settings: &OpenAiCompatibleEditPredictionSettings,
    prompt: String,
    suffix: String,
    max_tokens: u32,
    api_key: Option<Arc<str>>,
    http_client: &Arc<dyn http_client::HttpClient>,
) -> Result<(String, String)> {
    let request = DeepseekCompletionRequest {
        model: settings.model.clone(),
        prompt,
        suffix: (!suffix.is_empty()).then_some(suffix),
        max_tokens: Some(max_tokens),
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

    let http_request = http_request_builder.body(http_client::AsyncBody::from(request_body))?;

    let mut response = http_client.send(http_request).await?;
    let status = response.status();

    if !status.is_success() {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        anyhow::bail!("Deepseek API error: {} - {}", status, body);
    }

    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    let parsed: RawCompletionResponse =
        serde_json::from_str(&body).context("Failed to parse Deepseek completion response")?;
    let text = parsed
        .choices
        .into_iter()
        .next()
        .map(|choice| choice.text)
        .unwrap_or_default();
    Ok((text, parsed.id))
}
