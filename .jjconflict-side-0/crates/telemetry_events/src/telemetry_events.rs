//! See [Telemetry in Zed](https://zed.dev/docs/telemetry) for additional information.

use semantic_version::SemanticVersion;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, sync::Arc, time::Duration};

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
    Editor(EditorEvent),
    InlineCompletion(InlineCompletionEvent),
    InlineCompletionRating(InlineCompletionRatingEvent),
    Call(CallEvent),
    Assistant(AssistantEventData),
    Cpu(CpuEvent),
    Memory(MemoryEvent),
    App(AppEvent),
    Setting(SettingEvent),
    Extension(ExtensionEvent),
    Edit(EditEvent),
    Action(ActionEvent),
    Repl(ReplEvent),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlexibleEvent {
    pub event_type: String,
    pub event_properties: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditorEvent {
    /// The editor operation performed (open, save)
    pub operation: String,
    /// The extension of the file that was opened or saved
    pub file_extension: Option<String>,
    /// Whether the user is in vim mode or not
    pub vim_mode: bool,
    /// Whether the user has copilot enabled or not
    pub copilot_enabled: bool,
    /// Whether the user has copilot enabled for the language of the file opened or saved
    pub copilot_enabled_for_language: bool,
    /// Whether the client is opening/saving a local file or a remote file via SSH
    #[serde(default)]
    pub is_via_ssh: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineCompletionEvent {
    /// Provider of the completion suggestion (e.g. copilot, supermaven)
    pub provider: String,
    pub suggestion_accepted: bool,
    pub file_extension: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InlineCompletionRating {
    Positive,
    Negative,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineCompletionRatingEvent {
    pub rating: InlineCompletionRating,
    pub input_events: Arc<str>,
    pub input_excerpt: Arc<str>,
    pub output_excerpt: Arc<str>,
    pub feedback: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CallEvent {
    /// Operation performed: invite/join call; begin/end screenshare; share/unshare project; etc
    pub operation: String,
    pub room_id: Option<u64>,
    pub channel_id: Option<u64>,
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
pub struct CpuEvent {
    pub usage_as_percentage: f32,
    pub core_count: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemoryEvent {
    pub memory_in_bytes: u64,
    pub virtual_memory_in_bytes: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionEvent {
    pub source: String,
    pub action: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditEvent {
    pub duration: i64,
    pub environment: String,
    /// Whether the edits occurred locally or remotely via SSH
    #[serde(default)]
    pub is_via_ssh: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingEvent {
    pub setting: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionEvent {
    pub extension_id: Arc<str>,
    pub version: Arc<str>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppEvent {
    pub operation: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplEvent {
    pub kernel_language: String,
    pub kernel_status: String,
    pub repl_session_id: String,
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
