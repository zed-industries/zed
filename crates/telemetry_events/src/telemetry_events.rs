//! See [Telemetry in Zed](https://zed.dev/docs/telemetry) for additional information.

use semantic_version::SemanticVersion;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, time::Duration};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventRequestBody {
    /// Identifier unique to each system Zed is installed on
    pub system_id: Option<String>,
    /// Identifier unique to each Zed installation (differs for stable, preview, dev)
    pub installation_id: Option<String>,
    /// Identifier unique to each logged in Zed user (randomly generated on first sign in)
    /// Identifier unique to each Zed session (differs for each time you open Zed)
    pub session_id: Option<String>,
    pub metrics_id: Option<String>,
    /// True for Zed staff, otherwise false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_staff: Option<bool>,
    /// Zed version number
    pub app_version: String,
    pub os_name: String,
    pub os_version: Option<String>,
    pub architecture: String,
    /// Zed release channel (stable, preview, dev)
    pub release_channel: Option<String>,
    pub events: Vec<EventWrapper>,
}

impl EventRequestBody {
    pub fn semver(&self) -> Option<SemanticVersion> {
        self.app_version.parse().ok()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventWrapper {
    pub signed_in: bool,
    /// Duration between this event's timestamp and the timestamp of the first event in the current batch
    pub milliseconds_since_first_event: i64,
    /// The event itself
    #[serde(flatten)]
    pub event: Event,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantKind {
    Panel,
    Inline,
    InlineTerminal,
}
impl Display for AssistantKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Panel => "panel",
                Self::Inline => "inline",
                Self::InlineTerminal => "inline_terminal",
            }
        )
    }
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantPhase {
    #[default]
    Response,
    Invoked,
    Accepted,
    Rejected,
}

impl Display for AssistantPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Response => "response",
                Self::Invoked => "invoked",
                Self::Accepted => "accepted",
                Self::Rejected => "rejected",
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    Flexible(FlexibleEvent),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlexibleEvent {
    pub event_type: String,
    pub event_properties: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InlineCompletionRating {
    Positive,
    Negative,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantEventData {
    /// Unique random identifier for each assistant tab (None for inline assist)
    pub conversation_id: Option<String>,
    /// Server-generated message ID (only supported for some providers)
    pub message_id: Option<String>,
    /// The kind of assistant (Panel, Inline)
    pub kind: AssistantKind,
    #[serde(default)]
    pub phase: AssistantPhase,
    /// Name of the AI model used (gpt-4o, claude-3-5-sonnet, etc)
    pub model: String,
    pub model_provider: String,
    pub response_latency: Option<Duration>,
    pub error_message: Option<String>,
    pub language_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BacktraceFrame {
    pub ip: usize,
    pub symbol_addr: usize,
    pub base: Option<usize>,
    pub symbols: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HangReport {
    pub backtrace: Vec<BacktraceFrame>,
    pub app_version: Option<SemanticVersion>,
    pub os_name: String,
    pub os_version: Option<String>,
    pub architecture: String,
    /// Identifier unique to each Zed installation (differs for stable, preview, dev)
    pub installation_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocationData {
    pub file: String,
    pub line: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Panic {
    /// The name of the thread that panicked
    pub thread: String,
    /// The panic message
    pub payload: String,
    /// The location of the panic (file, line number)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_data: Option<LocationData>,
    pub backtrace: Vec<String>,
    /// Zed version number
    pub app_version: String,
    /// The Git commit SHA that Zed was built at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_commit_sha: Option<String>,
    /// Zed release channel (stable, preview, dev)
    pub release_channel: String,
    pub target: Option<String>,
    pub os_name: String,
    pub os_version: Option<String>,
    pub architecture: String,
    /// The time the panic occurred (UNIX millisecond timestamp)
    pub panicked_on: i64,
    /// Identifier unique to each system Zed is installed on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_id: Option<String>,
    /// Identifier unique to each Zed installation (differs for stable, preview, dev)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_id: Option<String>,
    /// Identifier unique to each Zed session (differs for each time you open Zed)
    pub session_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct PanicRequest {
    pub panic: Panic,
}
