use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc};
use util::SemanticVersion;

#[derive(Serialize, Deserialize, Debug)]
pub struct EventRequestBody {
    pub installation_id: Option<String>,
    pub session_id: Option<String>,
    pub is_staff: Option<bool>,
    pub app_version: String,
    pub os_name: String,
    pub os_version: Option<String>,
    pub architecture: String,
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
    Copilot(CopilotEvent),
    Call(CallEvent),
    Assistant(AssistantEvent),
    Cpu(CpuEvent),
    Memory(MemoryEvent),
    App(AppEvent),
    Setting(SettingEvent),
    Extension(ExtensionEvent),
    Edit(EditEvent),
    Action(ActionEvent),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditorEvent {
    pub operation: String,
    pub file_extension: Option<String>,
    pub vim_mode: bool,
    pub copilot_enabled: bool,
    pub copilot_enabled_for_language: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CopilotEvent {
    pub suggestion_id: Option<String>,
    pub suggestion_accepted: bool,
    pub file_extension: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CallEvent {
    pub operation: String,
    pub room_id: Option<u64>,
    pub channel_id: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantEvent {
    pub conversation_id: Option<String>,
    pub kind: AssistantKind,
    pub model: String,
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
