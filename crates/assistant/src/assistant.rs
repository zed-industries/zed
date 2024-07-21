pub mod assistant_panel;
pub mod assistant_settings;
mod context;
pub mod context_store;
mod inline_assistant;
mod model_selector;
mod prompt_library;
mod prompts;
mod slash_command;
mod streaming_diff;
mod terminal_inline_assistant;

pub use assistant_panel::{AssistantPanel, AssistantPanelEvent};
use assistant_settings::AssistantSettings;
use assistant_slash_command::SlashCommandRegistry;
use client::{proto, Client};
use command_palette_hooks::CommandPaletteFilter;
use completion::CompletionProvider;
pub use context::*;
pub use context_store::*;
use fs::Fs;
use gpui::{
    actions, impl_actions, AppContext, BorrowAppContext, Global, SharedString, UpdateGlobal,
};
use indexed_docs::IndexedDocsRegistry;
pub(crate) use inline_assistant::*;
use language_model::LanguageModelResponseMessage;
pub(crate) use model_selector::*;
use semantic_index::{CloudEmbeddingProvider, SemanticIndex};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use slash_command::{
    active_command, default_command, diagnostics_command, docs_command, fetch_command,
    file_command, now_command, project_command, prompt_command, search_command, symbols_command,
    tabs_command, term_command,
};
use std::sync::Arc;
pub(crate) use streaming_diff::*;

actions!(
    assistant,
    [
        Assist,
        Split,
        CycleMessageRole,
        QuoteSelection,
        InsertIntoEditor,
        ToggleFocus,
        ResetKey,
        InsertActivePrompt,
        DeployHistory,
        DeployPromptLibrary,
        ConfirmCommand,
        ToggleModelSelector,
        DebugEditSteps
    ]
);

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct InlineAssist {
    prompt: Option<String>,
}

impl_actions!(assistant, [InlineAssist]);

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MessageId(clock::Lamport);

impl MessageId {
    pub fn as_u64(self) -> u64 {
        self.0.as_u64()
    }
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MessageStatus {
    Pending,
    Done,
    Error(SharedString),
}

impl MessageStatus {
    pub fn from_proto(status: proto::ContextMessageStatus) -> MessageStatus {
        match status.variant {
            Some(proto::context_message_status::Variant::Pending(_)) => MessageStatus::Pending,
            Some(proto::context_message_status::Variant::Done(_)) => MessageStatus::Done,
            Some(proto::context_message_status::Variant::Error(error)) => {
                MessageStatus::Error(error.message.into())
            }
            None => MessageStatus::Pending,
        }
    }

    pub fn to_proto(&self) -> proto::ContextMessageStatus {
        match self {
            MessageStatus::Pending => proto::ContextMessageStatus {
                variant: Some(proto::context_message_status::Variant::Pending(
                    proto::context_message_status::Pending {},
                )),
            },
            MessageStatus::Done => proto::ContextMessageStatus {
                variant: Some(proto::context_message_status::Variant::Done(
                    proto::context_message_status::Done {},
                )),
            },
            MessageStatus::Error(message) => proto::ContextMessageStatus {
                variant: Some(proto::context_message_status::Variant::Error(
                    proto::context_message_status::Error {
                        message: message.to_string(),
                    },
                )),
            },
        }
    }
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

pub fn init(fs: Arc<dyn Fs>, client: Arc<Client>, cx: &mut AppContext) {
    cx.set_global(Assistant::default());
    AssistantSettings::register(cx);

    cx.spawn(|mut cx| {
        let client = client.clone();
        async move {
            let embedding_provider = CloudEmbeddingProvider::new(client.clone());
            let semantic_index = SemanticIndex::new(
                paths::embeddings_dir().join("semantic-index-db.0.mdb"),
                Arc::new(embedding_provider),
                &mut cx,
            )
            .await?;
            cx.update(|cx| cx.set_global(semantic_index))
        }
    })
    .detach();

    context_store::init(&client);
    prompt_library::init(cx);
    init_completion_provider(Arc::clone(&client), cx);
    assistant_slash_command::init(cx);
    register_slash_commands(cx);
    assistant_panel::init(cx);
    inline_assistant::init(fs.clone(), client.telemetry().clone(), cx);
    terminal_inline_assistant::init(fs.clone(), client.telemetry().clone(), cx);
    IndexedDocsRegistry::init_global(cx);

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

fn init_completion_provider(client: Arc<Client>, cx: &mut AppContext) {
    let provider = assistant_settings::create_provider_from_settings(client.clone(), 0, cx);
    cx.set_global(CompletionProvider::new(provider, Some(client)));

    let mut settings_version = 0;
    cx.observe_global::<SettingsStore>(move |cx| {
        settings_version += 1;
        cx.update_global::<CompletionProvider, _>(|provider, cx| {
            assistant_settings::update_completion_provider_settings(provider, settings_version, cx);
        })
    })
    .detach();
}

fn register_slash_commands(cx: &mut AppContext) {
    let slash_command_registry = SlashCommandRegistry::global(cx);
    slash_command_registry.register_command(file_command::FileSlashCommand, true);
    slash_command_registry.register_command(active_command::ActiveSlashCommand, true);
    slash_command_registry.register_command(symbols_command::OutlineSlashCommand, true);
    slash_command_registry.register_command(tabs_command::TabsSlashCommand, true);
    slash_command_registry.register_command(project_command::ProjectSlashCommand, true);
    slash_command_registry.register_command(search_command::SearchSlashCommand, true);
    slash_command_registry.register_command(prompt_command::PromptSlashCommand, true);
    slash_command_registry.register_command(default_command::DefaultSlashCommand, true);
    slash_command_registry.register_command(term_command::TermSlashCommand, true);
    slash_command_registry.register_command(now_command::NowSlashCommand, true);
    slash_command_registry.register_command(diagnostics_command::DiagnosticsSlashCommand, true);
    slash_command_registry.register_command(docs_command::DocsSlashCommand, true);
    slash_command_registry.register_command(fetch_command::FetchSlashCommand, false);
}

pub fn humanize_token_count(count: usize) -> String {
    match count {
        0..=999 => count.to_string(),
        1000..=9999 => {
            let thousands = count / 1000;
            let hundreds = (count % 1000 + 50) / 100;
            if hundreds == 0 {
                format!("{}k", thousands)
            } else if hundreds == 10 {
                format!("{}k", thousands + 1)
            } else {
                format!("{}.{}k", thousands, hundreds)
            }
        }
        _ => format!("{}k", (count + 500) / 1000),
    }
}

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}
