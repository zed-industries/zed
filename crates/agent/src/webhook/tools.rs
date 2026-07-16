//! Agent tools for managing webhook subscriptions.
//! Provides `webhook_subscribe`, `webhook_unsubscribe`, `webhook_list`.

use std::sync::Arc;

use crate::webhook::{WebhookEventType, WebhookSubscription, global_store};
use crate::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema::v1 as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use language_model::LanguageModelToolResultContent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// webhook_subscribe
// ---------------------------------------------------------------------------

/// Subscribe to an event that will trigger the agent automatically.
/// Choose the event type and provide a prompt to run when it fires.
///
/// Event types:
/// - `http` — listen for POST requests on a localhost port. Provide a port number as the filter.
/// - `file_change` — watch for file changes matching a glob (e.g. "**/*.rs"). Provide the glob as filter.
/// - `git_hook` — run on git events. Provide the hook name as filter (e.g. "pre-commit").
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WebhookSubscribeToolInput {
    /// Event type: "http", "file_change", or "git_hook".
    pub event_type: String,
    /// Filter: port for http, glob pattern for file_change, hook name for git_hook.
    #[serde(default)]
    pub filter: String,
    /// The prompt to run when the webhook fires.
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebhookSubscribeToolOutput {
    Success { id: String },
    Error { error: String },
}

impl From<WebhookSubscribeToolOutput> for LanguageModelToolResultContent {
    fn from(output: WebhookSubscribeToolOutput) -> Self {
        match output {
            WebhookSubscribeToolOutput::Success { id } => {
                format!("Webhook '{id}' created. It will fire when the event occurs.").into()
            }
            WebhookSubscribeToolOutput::Error { error } => error.into(),
        }
    }
}

pub struct WebhookSubscribeTool;

impl AgentTool for WebhookSubscribeTool {
    type Input = WebhookSubscribeToolInput;
    type Output = WebhookSubscribeToolOutput;

    const NAME: &'static str = "webhook_subscribe";

    fn kind() -> acp::ToolKind { acp::ToolKind::Write }
    fn initial_title(&self, _: Result<Self::Input, _>, _: &mut App) -> SharedString {
        "Creating webhook subscription…".into()
    }
    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _: ToolCallEventStream,
        _: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        Task::ready(async move {
            let input = input.recv().await.map_err(|e| WebhookSubscribeToolOutput::Error {
                error: format!("Failed to receive input: {e}"),
            })?;

            let event_type = match input.event_type.as_str() {
                "http" => WebhookEventType::Http,
                "file_change" | "file" => WebhookEventType::FileChange,
                "git_hook" | "git" => WebhookEventType::GitHook,
                other => return Ok(WebhookSubscribeToolOutput::Error {
                    error: format!("Unknown event type '{other}'. Use: http, file_change, or git_hook"),
                }),
            };

            let id = slugify(&format!("{}-{}", input.event_type, &input.prompt));
            let store = global_store();
            let sub = WebhookSubscription {
                id,
                event_type,
                filter: input.filter,
                prompt: input.prompt,
                active: true,
                fire_count: 0,
                last_fired_at: 0,
            };
            let id = sub.id.clone();
            store.add(sub);
            Ok(WebhookSubscribeToolOutput::Success { id })
        })
    }
}

// ---------------------------------------------------------------------------
// webhook_unsubscribe
// ---------------------------------------------------------------------------

/// Remove a webhook subscription by its ID.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WebhookUnsubscribeToolInput {
    /// The ID of the webhook to remove.
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebhookUnsubscribeToolOutput {
    Success { removed: bool },
    Error { error: String },
}

impl From<WebhookUnsubscribeToolOutput> for LanguageModelToolResultContent {
    fn from(output: WebhookUnsubscribeToolOutput) -> Self {
        match output {
            WebhookUnsubscribeToolOutput::Success { removed } => {
                if removed { "Webhook removed.".into() } else { "No webhook found with that ID.".into() }
            }
            WebhookUnsubscribeToolOutput::Error { error } => error.into(),
        }
    }
}

pub struct WebhookUnsubscribeTool;

impl AgentTool for WebhookUnsubscribeTool {
    type Input = WebhookUnsubscribeToolInput;
    type Output = WebhookUnsubscribeToolOutput;

    const NAME: &'static str = "webhook_unsubscribe";
    fn kind() -> acp::ToolKind { acp::ToolKind::Write }
    fn initial_title(&self, _: Result<Self::Input, _>, _: &mut App) -> SharedString {
        "Removing webhook…".into()
    }
    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _: ToolCallEventStream,
        _: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        Task::ready(async move {
            let input = input.recv().await.map_err(|e| WebhookUnsubscribeToolOutput::Error {
                error: format!("Failed to receive input: {e}"),
            })?;
            let store = global_store();
            let removed = store.remove(&input.id);
            Ok(WebhookUnsubscribeToolOutput::Success { removed })
        })
    }
}

// ---------------------------------------------------------------------------
// webhook_list
// ---------------------------------------------------------------------------

/// List all active webhook subscriptions.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WebhookListToolInput;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebhookListToolOutput {
    Success { subscriptions: Vec<serde_json::Value>, total: usize },
    Error { error: String },
}

impl From<WebhookListToolOutput> for LanguageModelToolResultContent {
    fn from(output: WebhookListToolOutput) -> Self {
        match output {
            WebhookListToolOutput::Success { subscriptions, total } => {
                if subscriptions.is_empty() {
                    "No webhook subscriptions.".into()
                } else {
                    let mut lines = format!("**{total} webhook(s):**\n\n");
                    for (i, sub) in subscriptions.iter().enumerate() {
                        let id = sub.get("id").and_then(|v| v.as_str()).unwrap_or("?");
                        let et = sub.get("event_type").and_then(|v| v.as_str()).unwrap_or("?");
                        let active = sub.get("active").and_then(|v| v.as_bool()).unwrap_or(false);
                        let fires = sub.get("fire_count").and_then(|v| v.as_u64()).unwrap_or(0);
                        let status = if active { "▶ active" } else { "⏸ paused" };
                        lines.push_str(&format!("{i}. **{id}** — {et} ({status}, {fires} fires)\n"));
                    }
                    lines.into()
                }
            }
            WebhookListToolOutput::Error { error } => error.into(),
        }
    }
}

pub struct WebhookListTool;

impl AgentTool for WebhookListTool {
    type Input = WebhookListToolInput;
    type Output = WebhookListToolOutput;

    const NAME: &'static str = "webhook_list";
    fn kind() -> acp::ToolKind { acp::ToolKind::Read }
    fn initial_title(&self, _: Result<Self::Input, _>, _: &mut App) -> SharedString {
        "Listing webhooks…".into()
    }
    fn run(
        self: Arc<Self>,
        _: ToolInput<Self::Input>,
        _: ToolCallEventStream,
        _: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        Task::ready(async move {
            let store = global_store();
            let subs: Vec<serde_json::Value> = store
                .all()
                .into_iter()
                .filter_map(|s| serde_json::to_value(s).ok())
                .collect();
            let total = subs.len();
            Ok(WebhookListToolOutput::Success { subscriptions: subs, total })
        })
    }
}

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("http-run tests"), "http-run-tests");
        assert_eq!(slugify("file_change!!"), "file-change");
    }
}
