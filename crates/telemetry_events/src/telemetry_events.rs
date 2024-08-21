/// Please see: [Telemetry in Zed](https://zed.dev/docs/telemetry) for additional documentation.
use semantic_version::SemanticVersion;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc, time::Duration};

#[derive(Serialize, Deserialize, Debug)]
pub struct EventRequestBody {
    /// Identifier unique to each Zed installation (differs for stable, preview, dev)
    pub installation_id: Option<String>,
    /// Identifier unique to each logged in Zed user (randomly generated on first sign in)
    pub metrics_id: Option<String>,
    /// Identifier unique to each Zed session (differs for each time you open Zed)
    pub session_id: Option<String>,
    /// True for Zed staff, otherwise false
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

#[derive(Serialize, Deserialize, Debug)]
pub struct EventWrapper {
    pub signed_in: bool,
    /// Duration between this event's timestamp and the timestamp of the first event in the current batch
    pub milliseconds_since_first_event: i64,
    #[serde(flatten)]
    pub event: Event,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantKind {
    Panel,
    Inline,
}

impl Display for AssistantKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Panel => "panel",
                Self::Inline => "inline",
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    Editor(EditorEvent),
    Copilot(CopilotEvent), // Needed for clients sending old copilot_event types
    InlineCompletion(InlineCompletionEvent),
    Call(CallEvent),
    Assistant(AssistantEvent),
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
}

/// Deprecated since Zed v0.137.0 (2024-05-29). Replaced by InlineCompletionEvent.
// Needed for clients sending old copilot_event types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CopilotEvent {
    pub suggestion_id: Option<String>,
    pub suggestion_accepted: bool,
    pub file_extension: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineCompletionEvent {
    /// Provider of the completion suggestion (e.g. copilot, supermaven)
    pub provider: String,
    pub suggestion_accepted: bool,
    pub file_extension: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CallEvent {
    /// Operation performed: invite/join call; begin/end screenshare; share/unshare project; etc
    pub operation: String,
    pub room_id: Option<u64>,
    pub channel_id: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantEvent {
    /// Unique random identifier for each assistant tab (None for inline assist)
    pub conversation_id: Option<String>,
    /// The kind of assistant (Panel, Inline)
    pub kind: AssistantKind,
    /// Name of the AI model used (gpt-4o, claude-3-5-sonnet, etc)
    pub model: String,
    pub response_latency: Option<Duration>,
    pub error_message: Option<String>,
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

#[derive(Serialize, Deserialize)]
pub struct LocationData {
    pub file: String,
    pub line: u32,
}

#[derive(Serialize, Deserialize)]
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
    /// Zed release channel (stable, preview, dev)
    pub release_channel: String,
    pub os_name: String,
    pub os_version: Option<String>,
    pub architecture: String,
    /// The time the panic occurred (UNIX millisecond timestamp)
    pub panicked_on: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Identifier unique to each Zed installation (differs for stable, preview, dev)
    pub installation_id: Option<String>,
    /// Identifier unique to each Zed session (differs for each time you open Zed)
    pub session_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct PanicRequest {
    pub panic: Panic,
}
