use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

use crate::realtime::types::{CreateResponse, Item, Session};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    pub session: Session,
    pub r#type: String,
}

impl Default for SessionUpdate {
    fn default() -> Self {
        Self {
            event_id: None,
            session: Session::default(),
            r#type: "session.update".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InputAudioBufferAppend {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    pub audio: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InputAudioBufferCommit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InputAudioBufferClear {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConversationItemCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,
    pub item: Item,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConversationItemTruncate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    pub item_id: String,
    pub content_index: u32,
    pub audio_end_ms: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConversationItemDelete {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ResponseCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    pub response: Option<CreateResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ResponseCancel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientEvent {
    #[serde(rename = "session.update")]
    SessionUpdate(SessionUpdate),
    #[serde(rename = "input_audio_buffer.append")]
    InputAudioBufferAppend(InputAudioBufferAppend),
    #[serde(rename = "input_audio_buffer.commit")]
    InputAudioBufferCommit(InputAudioBufferCommit),
    #[serde(rename = "input_audio_buffer.clear")]
    InputAudioBufferClear(InputAudioBufferClear),
    #[serde(rename = "conversation.item.create")]
    ConversationItemCreate(ConversationItemCreate),
    #[serde(rename = "conversation.item.truncate")]
    ConversationItemTruncate(ConversationItemTruncate),
    #[serde(rename = "conversation.item.delete")]
    ConversationItemDelete(ConversationItemDelete),
    #[serde(rename = "response.create")]
    ResponseCreate(ResponseCreate),
    #[serde(rename = "response.cancel")]
    ResponseCancel(ResponseCancel),
}

impl From<ClientEvent> for Message {
    fn from(value: ClientEvent) -> Self {
        Message::text(String::from(&value))
    }
}

impl From<&ClientEvent> for String {
    fn from(value: &ClientEvent) -> Self {
        serde_json::to_string(value).unwrap()
    }
}

impl From<ConversationItemCreate> for Message {
    fn from(value: ConversationItemCreate) -> Self {
        Self::from(ClientEvent::ConversationItemCreate(value))
    }
}

impl From<InputAudioBufferAppend> for Message {
    fn from(value: InputAudioBufferAppend) -> Self {
        Self::from(ClientEvent::InputAudioBufferAppend(value))
    }
}

impl From<InputAudioBufferCommit> for Message {
    fn from(value: InputAudioBufferCommit) -> Self {
        Self::from(ClientEvent::InputAudioBufferCommit(value))
    }
}

impl From<InputAudioBufferClear> for Message {
    fn from(value: InputAudioBufferClear) -> Self {
        Self::from(ClientEvent::InputAudioBufferClear(value))
    }
}

impl From<SessionUpdate> for Message {
    fn from(value: SessionUpdate) -> Self {
        Self::from(ClientEvent::SessionUpdate(value))
    }
}

impl From<ConversationItemTruncate> for Message {
    fn from(value: ConversationItemTruncate) -> Self {
        Self::from(ClientEvent::ConversationItemTruncate(value))
    }
}

impl From<ConversationItemDelete> for Message {
    fn from(value: ConversationItemDelete) -> Self {
        Self::from(ClientEvent::ConversationItemDelete(value))
    }
}

impl From<ResponseCreate> for Message {
    fn from(value: ResponseCreate) -> Self {
        Self::from(ClientEvent::ResponseCreate(value))
    }
}

impl From<ResponseCancel> for Message {
    fn from(value: ResponseCancel) -> Self {
        Self::from(ClientEvent::ResponseCancel(value))
    }
}
