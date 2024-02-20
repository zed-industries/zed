pub mod assistant_panel;
pub mod assistant_settings;
mod codegen;
mod completion_provider;
mod prompts;
mod saved_conversation;
mod streaming_diff;

pub use assistant_panel::AssistantPanel;
use assistant_settings::{AssistantSettings, OpenAiModel, ZedDotDevModel};
use chrono::{DateTime, Local};
use client::{proto, Client};
pub(crate) use completion_provider::*;
use gpui::{actions, AppContext, SharedString};
pub(crate) use saved_conversation::*;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{
    fmt::{self, Display},
    sync::Arc,
};

actions!(
    assistant,
    [
        NewConversation,
        Assist,
        Split,
        CycleMessageRole,
        QuoteSelection,
        ToggleFocus,
        ResetKey,
        InlineAssist,
        ToggleIncludeConversation,
    ]
);

#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
struct MessageId(usize);

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
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
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
            Role::System => write!(f, "system"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum LanguageModel {
    ZedDotDev(ZedDotDevModel),
    OpenAi(OpenAiModel),
}

impl Default for LanguageModel {
    fn default() -> Self {
        LanguageModel::ZedDotDev(ZedDotDevModel::default())
    }
}

impl LanguageModel {
    pub fn telemetry_id(&self) -> String {
        match self {
            LanguageModel::OpenAi(model) => format!("openai/{}", model.id()),
            LanguageModel::ZedDotDev(model) => format!("zed.dev/{}", model.id()),
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            LanguageModel::OpenAi(model) => format!("openai/{}", model.display_name()),
            LanguageModel::ZedDotDev(model) => format!("zed.dev/{}", model.display_name()),
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            LanguageModel::OpenAi(model) => tiktoken_rs::model::get_context_size(model.id()),
            LanguageModel::ZedDotDev(model) => match model {
                ZedDotDevModel::GptThreePointFiveTurbo
                | ZedDotDevModel::GptFour
                | ZedDotDevModel::GptFourTurbo => tiktoken_rs::model::get_context_size(model.id()),
                ZedDotDevModel::Custom(_) => 30720, // TODO: Base this on the selected model.
            },
        }
    }

    pub fn id(&self) -> &str {
        match self {
            LanguageModel::OpenAi(model) => model.id(),
            LanguageModel::ZedDotDev(model) => model.id(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LanguageModelRequestMessage {
    pub role: Role,
    pub content: String,
}

impl LanguageModelRequestMessage {
    pub fn to_proto(&self) -> proto::LanguageModelRequestMessage {
        proto::LanguageModelRequestMessage {
            role: match self.role {
                Role::User => proto::LanguageModelRole::LanguageModelUser,
                Role::Assistant => proto::LanguageModelRole::LanguageModelAssistant,
                Role::System => proto::LanguageModelRole::LanguageModelSystem,
            } as i32,
            content: self.content.clone(),
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct LanguageModelRequest {
    pub model: LanguageModel,
    pub messages: Vec<LanguageModelRequestMessage>,
    pub stop: Vec<String>,
    pub temperature: f32,
}

impl LanguageModelRequest {
    pub fn to_proto(&self) -> proto::CompleteWithLanguageModel {
        proto::CompleteWithLanguageModel {
            model: self.model.id().to_string(),
            messages: self.messages.iter().map(|m| m.to_proto()).collect(),
            stop: self.stop.clone(),
            temperature: self.temperature,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LanguageModelResponseMessage {
    pub role: Option<Role>,
    pub content: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LanguageModelUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct LanguageModelChoiceDelta {
    pub index: u32,
    pub delta: LanguageModelResponseMessage,
    pub finish_reason: Option<String>,
}

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
    Error(SharedString),
}

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    AssistantSettings::register(cx);
    completion_provider::init(client, cx);
    assistant_panel::init(cx);
}

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}
