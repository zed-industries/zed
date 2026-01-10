use serde::{Deserialize, Serialize};

// Outbound messages
#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OutboundMessage {
    StateUpdate(StateUpdateMessage),
    #[allow(dead_code)]
    UseFreeVersion,
    Logout,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateUpdateMessage {
    pub new_id: String,
    pub updates: Vec<StateUpdate>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StateUpdate {
    FileUpdate(FileUpdateMessage),
    CursorUpdate(CursorPositionUpdateMessage),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileUpdateMessage {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CursorPositionUpdateMessage {
    pub path: String,
    pub offset: usize,
}

// Inbound messages coming in on stdout

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ResponseItem {
    // A completion
    Text { text: String },
    // Vestigial message type from old versions -- safe to ignore
    Del { text: String },
    // Be able to delete whitespace prior to the cursor, likely for the rest of the completion
    Dedent { text: String },
    // When the completion is over
    End,
    // Got the closing parentheses and shouldn't show any more after
    Barrier,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupermavenResponse {
    pub state_id: String,
    pub items: Vec<ResponseItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupermavenMetadataMessage {
    pub dust_strings: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupermavenTaskUpdateMessage {
    pub task: String,
    pub status: TaskStatus,
    pub percent_complete: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    InProgress,
    Complete,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupermavenActiveRepoMessage {
    pub repo_simple_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SupermavenPopupAction {
    OpenUrl { label: String, url: String },
    NoOp { label: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SupermavenPopupMessage {
    pub message: String,
    pub actions: Vec<SupermavenPopupAction>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub struct ActivationRequest {
    pub activate_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupermavenSetMessage {
    pub key: String,
    pub value: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServiceTier {
    FreeNoLicense,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SupermavenMessage {
    Response(SupermavenResponse),
    Metadata(SupermavenMetadataMessage),
    Apology {
        message: Option<String>,
    },
    ActivationRequest(ActivationRequest),
    ActivationSuccess,
    Passthrough {
        passthrough: Box<SupermavenMessage>,
    },
    Popup(SupermavenPopupMessage),
    TaskStatus(SupermavenTaskUpdateMessage),
    ActiveRepo(SupermavenActiveRepoMessage),
    ServiceTier {
        service_tier: ServiceTier,
    },

    Set(SupermavenSetMessage),
    #[serde(other)]
    Unknown,
}
