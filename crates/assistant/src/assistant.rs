pub mod assistant_panel;
pub mod assistant_settings;
mod codegen;
mod completion_provider;
mod model_selector;
mod prompts;
mod saved_conversation;
mod search;
mod slash_command;
mod streaming_diff;

pub use assistant_panel::AssistantPanel;

use assistant_settings::{AnthropicModel, AssistantSettings, OpenAiModel, ZedDotDevModel};
use client::{proto, Client};
use command_palette_hooks::CommandPaletteFilter;
pub(crate) use completion_provider::*;
use gpui::{actions, AppContext, Global, SharedString, UpdateGlobal};
pub(crate) use model_selector::*;
pub(crate) use saved_conversation::*;
use semantic_index::{CloudEmbeddingProvider, SemanticIndex};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::{
    fmt::{self, Display},
    sync::Arc,
};
use util::paths::EMBEDDINGS_DIR;

actions!(
    assistant,
    [
        Assist,
        Split,
        CycleMessageRole,
        QuoteSelection,
        ToggleFocus,
        ResetKey,
        InlineAssist,
        InsertActivePrompt,
        ToggleHistory,
        ApplyEdit,
        ConfirmCommand,
        ToggleModelSelector
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
    Anthropic(AnthropicModel),
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
            LanguageModel::Anthropic(model) => format!("anthropic/{}", model.id()),
            LanguageModel::ZedDotDev(model) => format!("zed.dev/{}", model.id()),
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            LanguageModel::OpenAi(model) => model.display_name().into(),
            LanguageModel::Anthropic(model) => model.display_name().into(),
            LanguageModel::ZedDotDev(model) => model.display_name().into(),
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            LanguageModel::OpenAi(model) => model.max_token_count(),
            LanguageModel::Anthropic(model) => model.max_token_count(),
            LanguageModel::ZedDotDev(model) => model.max_token_count(),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            LanguageModel::OpenAi(model) => model.id(),
            LanguageModel::Anthropic(model) => model.id(),
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
            tool_calls: Vec::new(),
            tool_call_id: None,
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
            tool_choice: None,
            tools: Vec::new(),
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
    status: MessageStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum MessageStatus {
    Pending,
    Done,
    Error(SharedString),
}

/// The state pertaining to the Assistant.
#[derive(Default)]
struct Assistant {
    /// Whether the Assistant is enabled.
    enabled: bool,
}

impl Global for Assistant {}

impl Assistant {
    const NAMESPACE: &'static str = "assistant";

    fn set_enabled(&mut self, enabled: bool, cx: &mut AppContext) {
        if self.enabled == enabled {
            return;
        }

        self.enabled = enabled;

        if !enabled {
            CommandPaletteFilter::update_global(cx, |filter, _cx| {
                filter.hide_namespace(Self::NAMESPACE);
            });

            return;
        }

        CommandPaletteFilter::update_global(cx, |filter, _cx| {
            filter.show_namespace(Self::NAMESPACE);
        });
    }
}

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    cx.set_global(Assistant::default());
    AssistantSettings::register(cx);

    cx.spawn(|mut cx| {
        let client = client.clone();
        async move {
            let embedding_provider = CloudEmbeddingProvider::new(client.clone());
            let semantic_index = SemanticIndex::new(
                EMBEDDINGS_DIR.join("semantic-index-db.0.mdb"),
                Arc::new(embedding_provider),
                &mut cx,
            )
            .await?;
            cx.update(|cx| cx.set_global(semantic_index))
        }
    })
    .detach();
    completion_provider::init(client, cx);
    assistant_slash_command::init(cx);
    assistant_panel::init(cx);

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_namespace(Assistant::NAMESPACE);
    });
    Assistant::update_global(cx, |assistant, cx| {
        let settings = AssistantSettings::get_global(cx);

        assistant.set_enabled(settings.enabled, cx);
    });
    cx.observe_global::<SettingsStore>(|cx| {
        Assistant::update_global(cx, |assistant, cx| {
            let settings = AssistantSettings::get_global(cx);

            assistant.set_enabled(settings.enabled, cx);
        });
    })
    .detach();
}

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}
