pub mod assistant_panel;
pub mod assistant_settings;
pub mod completion_provider;
mod context_store;
mod inline_assistant;
mod model_selector;
mod prompt_library;
mod prompts;
mod search;
mod slash_command;
mod streaming_diff;
mod terminal_inline_assistant;

pub use assistant_panel::{AssistantPanel, AssistantPanelEvent};
use assistant_settings::AssistantSettings;
use assistant_slash_command::SlashCommandRegistry;
use client::Client;
use command_palette_hooks::CommandPaletteFilter;
pub(crate) use context_store::*;
use fs::Fs;
use gpui::{actions, AppContext, Global, SharedString, UpdateGlobal};
use indexed_docs::IndexedDocsRegistry;
pub(crate) use inline_assistant::*;
use language_model::Role;
pub(crate) use model_selector::*;
use semantic_index::{CloudEmbeddingProvider, SemanticIndex};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use slash_command::{
    active_command, default_command, diagnostics_command, docs_command, fetch_command,
    file_command, now_command, project_command, prompt_command, search_command, tabs_command,
    term_command,
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
        InlineAssist,
        InsertActivePrompt,
        DeployHistory,
        DeployPromptLibrary,
        ApplyEdit,
        ConfirmCommand,
        ToggleModelSelector
    ]
);

#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
struct MessageId(usize);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LanguageModelResponseMessage {
    pub role: Option<Role>,
    pub content: Option<String>,
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

    completion_provider::init(cx);
    prompt_library::init(cx);
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

fn register_slash_commands(cx: &mut AppContext) {
    let slash_command_registry = SlashCommandRegistry::global(cx);
    slash_command_registry.register_command(file_command::FileSlashCommand, true);
    slash_command_registry.register_command(active_command::ActiveSlashCommand, true);
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
