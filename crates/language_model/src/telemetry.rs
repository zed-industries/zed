use anthropic::{ANTHROPIC_API_URL, AnthropicError};
use anyhow::{Context as _, Result, anyhow};
use client::telemetry::Telemetry;
use gpui::BackgroundExecutor;
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use std::env;
use std::sync::Arc;
use telemetry_events::{AssistantEventData, AssistantKind, AssistantPhase};
use util::ResultExt;

pub const ANTHROPIC_PROVIDER_ID: &str = "anthropic";

pub fn report_assistant_event(
    event: AssistantEventData,
    telemetry: Option<Arc<Telemetry>>,
    client: Arc<dyn HttpClient>,
    model_api_key: Option<String>,
    executor: &BackgroundExecutor,
) {
    if let Some(telemetry) = telemetry.as_ref() {
        telemetry.report_assistant_event(event.clone());
        if telemetry.metrics_enabled() && event.model_provider == ANTHROPIC_PROVIDER_ID {
            executor
                .spawn(async move {
                    report_anthropic_event(event, client, model_api_key)
                        .await
                        .log_err();
                })
                .detach();
        }
    }
}

async fn report_anthropic_event(
    event: AssistantEventData,
    client: Arc<dyn HttpClient>,
    model_api_key: Option<String>,
) -> Result<(), AnthropicError> {
    let api_key = match model_api_key {
        Some(key) => key,
        None => {
            return Err(AnthropicError::Other(anyhow!(
                "Anthropic API key is not set"
            )));
        }
    };

    let uri = format!("{ANTHROPIC_API_URL}/v1/log/zed");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("X-Api-Key", api_key)
        .header("Content-Type", "application/json");
    let serialized_event: serde_json::Value = serde_json::json!({
        "completion_type": match event.kind {
            AssistantKind::Inline => "natural_language_completion_in_editor",
            AssistantKind::InlineTerminal => "natural_language_completion_in_terminal",
            AssistantKind::Panel => "conversation_message",
        },
        "event": match event.phase {
            AssistantPhase::Response => "response",
            AssistantPhase::Invoked => "invoke",
            AssistantPhase::Accepted => "accept",
            AssistantPhase::Rejected => "reject",
        },
        "metadata": {
            "language_name": event.language_name,
            "message_id": event.message_id,
            "platform": env::consts::OS,
        }
    });

    let request = request_builder
        .body(AsyncBody::from(serialized_event.to_string()))
        .context("failed to construct request body")?;

    let response = client
        .send(request)
        .await
        .context("failed to send request to Anthropic")?;

    if response.status().is_success() {
        return Ok(());
    }

    return Err(AnthropicError::Other(anyhow!(
        "Failed to log: {}",
        response.status(),
    )));
}
