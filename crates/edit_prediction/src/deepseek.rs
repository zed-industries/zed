use anyhow::{Context as _, Result};
use cloud_llm_client::predict_edits_v3::RawCompletionResponse;
use futures::AsyncReadExt as _;
use gpui::http_client;
use language::language_settings::OpenAiCompatibleEditPredictionSettings;
use language_model::{EnvVar, env_var};
use serde::Serialize;
use std::sync::Arc;

static DEEPSEEK_API_KEY_ENV_VAR: std::sync::LazyLock<EnvVar> = env_var!("DEEPSEEK_API_KEY");

/// Reads the Deepseek API key from the `DEEPSEEK_API_KEY` environment variable.
pub fn api_key() -> Option<Arc<str>> {
    DEEPSEEK_API_KEY_ENV_VAR.value.as_deref().map(Arc::from)
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
