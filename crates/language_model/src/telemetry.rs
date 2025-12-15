use crate::ANTHROPIC_PROVIDER_ID;
use anthropic::ANTHROPIC_API_URL;
use anyhow::{Context as _, anyhow};
use client::telemetry::Telemetry;
use gpui::BackgroundExecutor;
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use std::env;
use std::sync::Arc;
use telemetry_events::{AssistantEventData, AssistantKind, AssistantPhase};
use util::ResultExt;

pub fn report_assistant_event(
    event: AssistantEventData,
    telemetry: Option<Arc<Telemetry>>,
    client: Arc<dyn HttpClient>,
    model_api_key: Option<String>,
    executor: &BackgroundExecutor,
) {
    if let Some(telemetry) = telemetry.as_ref() {
        telemetry.report_assistant_event(event.clone());
        if telemetry.metrics_enabled() && event.model_provider == ANTHROPIC_PROVIDER_ID.0 {
            if let Some(api_key) = model_api_key {
                executor
                    .spawn(async move {
                        report_anthropic_event(event, client, api_key)
                            .await
                            .log_err();
                    })
                    .detach();
            } else {
                log::error!("Cannot send Anthropic telemetry because API key is missing");
            }
        }
    }
}

async fn report_anthropic_event(
    event: AssistantEventData,
    client: Arc<dyn HttpClient>,
    api_key: String,
) -> anyhow::Result<()> {
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
        .context("Failed to construct Anthropic telemetry HTTP request body")?;

    let response = client
        .send(request)
        .await
        .context("Failed to send telemetry HTTP request to Anthropic")?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow!(
            "Anthropic telemetry logging failed with HTTP status: {}",
            response.status()
        ))
    }
}

#[derive(Clone, Debug)]
pub struct AnthropicEventData {
    pub completion_type: AnthropicCompletionType,
    pub event: AnthropicEventType,
    pub language_name: Option<String>,
    pub message_id: Option<String>,
}

#[derive(Clone, Debug)]
pub enum AnthropicCompletionType {
    Editor,
    Terminal,
    Panel,
}

#[derive(Clone, Debug)]
pub enum AnthropicEventType {
    Invoked,
    Response,
    Accept,
    Reject,
}

impl AnthropicCompletionType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Editor => "natural_language_completion_in_editor",
            Self::Terminal => "natural_language_completion_in_terminal",
            Self::Panel => "conversation_message",
        }
    }
}

impl AnthropicEventType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Invoked => "invoke",
            Self::Response => "response",
            Self::Accept => "accept",
            Self::Reject => "reject",
        }
    }
}

pub fn report_anthropic_event_2(
    model: &Arc<dyn crate::LanguageModel>,
    event: AnthropicEventData,
    cx: &gpui::App,
) {
    let reporter = AnthropicEventReporter::new(model, cx);
    reporter.report(event);
}

#[derive(Clone)]
pub struct AnthropicEventReporter {
    http_client: Arc<dyn HttpClient>,
    executor: BackgroundExecutor,
    api_key: Option<String>,
    is_anthropic: bool,
}

impl AnthropicEventReporter {
    pub fn new(model: &Arc<dyn crate::LanguageModel>, cx: &gpui::App) -> Self {
        Self {
            http_client: cx.http_client(),
            executor: cx.background_executor().clone(),
            api_key: model.api_key(cx),
            is_anthropic: model.provider_id() == ANTHROPIC_PROVIDER_ID,
        }
    }

    pub fn report(&self, event: AnthropicEventData) {
        if !self.is_anthropic {
            return;
        }
        let Some(api_key) = self.api_key.clone() else {
            return;
        };
        let client = self.http_client.clone();
        self.executor
            .spawn(async move {
                send_anthropic_event(event, client, api_key).await.log_err();
            })
            .detach();
    }
}

async fn send_anthropic_event(
    event: AnthropicEventData,
    client: Arc<dyn HttpClient>,
    api_key: String,
) -> anyhow::Result<()> {
    let uri = format!("{ANTHROPIC_API_URL}/v1/log/zed");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("X-Api-Key", api_key)
        .header("Content-Type", "application/json");

    let serialized_event = serde_json::json!({
        "completion_type": event.completion_type.as_str(),
        "event": event.event.as_str(),
        "metadata": {
            "language_name": event.language_name,
            "message_id": event.message_id,
            "platform": env::consts::OS,
        }
    });

    let request = request_builder
        .body(AsyncBody::from(serialized_event.to_string()))
        .context("Failed to construct Anthropic telemetry HTTP request body")?;

    let response = client
        .send(request)
        .await
        .context("Failed to send telemetry HTTP request to Anthropic")?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow!(
            "Anthropic telemetry logging failed with HTTP status: {}",
            response.status()
        ))
    }
}
