pub mod assistant;
mod assistant_settings;

use anyhow::Result;
pub use assistant::AssistantPanel;
use assistant_settings::OpenAIModel;
use chrono::{DateTime, Local};
use collections::HashMap;
use fs::Fs;
use futures::StreamExt;
use gpui::AppContext;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Reverse,
    ffi::OsStr,
    fmt::{self, Display},
    path::PathBuf,
    sync::Arc,
};
use util::paths::CONVERSATIONS_DIR;

// Data types for chat completion requests
#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<RequestMessage>,
    stream: bool,
}

#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
struct MessageId(usize);

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MessageMetadata {
    role: Role,
    sent_at: DateTime<Local>,
    status: MessageStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum MessageStatus {
    Pending,
    Done,
    Error(Arc<str>),
}

#[derive(Serialize, Deserialize)]
struct SavedMessage {
    id: MessageId,
    start: usize,
}

#[derive(Serialize, Deserialize)]
struct SavedConversation {
    zed: String,
    version: String,
    text: String,
    messages: Vec<SavedMessage>,
    message_metadata: HashMap<MessageId, MessageMetadata>,
    summary: String,
    model: OpenAIModel,
}

impl SavedConversation {
    const VERSION: &'static str = "0.1.0";
}

struct SavedConversationMetadata {
    title: String,
    path: PathBuf,
    mtime: chrono::DateTime<chrono::Local>,
}

impl SavedConversationMetadata {
    pub async fn list(fs: Arc<dyn Fs>) -> Result<Vec<Self>> {
        fs.create_dir(&CONVERSATIONS_DIR).await?;

        let mut paths = fs.read_dir(&CONVERSATIONS_DIR).await?;
        let mut conversations = Vec::<SavedConversationMetadata>::new();
        while let Some(path) = paths.next().await {
            let path = path?;
            if path.extension() != Some(OsStr::new("json")) {
                continue;
            }

            let pattern = r" - \d+.zed.json$";
            let re = Regex::new(pattern).unwrap();

            let metadata = fs.metadata(&path).await?;
            if let Some((file_name, metadata)) = path
                .file_name()
                .and_then(|name| name.to_str())
                .zip(metadata)
            {
                let title = re.replace(file_name, "");
                conversations.push(Self {
                    title: title.into_owned(),
                    path,
                    mtime: metadata.mtime.into(),
                });
            }
        }
        conversations.sort_unstable_by_key(|conversation| Reverse(conversation.mtime));

        Ok(conversations)
    }
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
