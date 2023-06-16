pub mod assistant;
mod assistant_settings;

pub use assistant::AssistantPanel;
use gpui::AppContext;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

// Data types for chat completion requests
#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<RequestMessage>,
    stream: bool,
}

#[derive(Serialize, Deserialize)]
struct SavedConversation {
    zed: String,
    version: String,
    messages: Vec<RequestMessage>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct RequestMessage {
    role: Role,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct ResponseMessage {
    role: Option<Role>,
    content: Option<String>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Role {
    User,
    Assistant,
    System,
}

impl Role {
    pub fn cycle(&mut self) {
        *self = match self {
            Role::User => Role::Assistant,
            Role::Assistant => Role::System,
            Role::System => Role::User,
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Assistant => write!(f, "Assistant"),
            Role::System => write!(f, "System"),
        }
    }
}

#[derive(Deserialize, Debug)]
struct OpenAIResponseStreamEvent {
    pub id: Option<String>,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChatChoiceDelta>,
    pub usage: Option<Usage>,
}

#[derive(Deserialize, Debug)]
struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct ChatChoiceDelta {
    pub index: u32,
    pub delta: ResponseMessage,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
struct OpenAIUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[derive(Deserialize, Debug)]
struct OpenAIChoice {
    text: String,
    index: u32,
    logprobs: Option<serde_json::Value>,
    finish_reason: Option<String>,
}

pub fn init(cx: &mut AppContext) {
    assistant::init(cx);
}
