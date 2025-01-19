#![cfg_attr(target_os = "windows", allow(unused, dead_code))]

pub mod assistant_panel;
mod context;
pub mod context_store;
mod inline_assistant;
mod patch;
mod slash_command;
pub(crate) mod slash_command_picker;
pub mod slash_command_settings;
mod terminal_inline_assistant;

use std::path::PathBuf;
use std::sync::Arc;

use assistant_settings::AssistantSettings;
use assistant_slash_command::SlashCommandRegistry;
use assistant_slash_commands::{ProjectSlashCommandFeatureFlag, SearchSlashCommandFeatureFlag};
use client::{proto, Client};
use command_palette_hooks::CommandPaletteFilter;
use feature_flags::FeatureFlagAppExt;
use fs::Fs;
use gpui::impl_internal_actions;
use gpui::{actions, AppContext, Global, SharedString, UpdateGlobal};
use language_model::{
    LanguageModelId, LanguageModelProviderId, LanguageModelRegistry, LanguageModelResponseMessage,
};
use prompt_library::{PromptBuilder, PromptLoadingParams};
use semantic_index::{CloudEmbeddingProvider, SemanticDb};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use util::ResultExt;

pub use crate::assistant_panel::{AssistantPanel, AssistantPanelEvent};
pub use crate::context::*;
pub use crate::context_store::*;
pub(crate) use crate::inline_assistant::*;
pub use crate::patch::*;
use crate::slash_command_settings::SlashCommandSettings;

actions!(
    assistant,
    [
        Assist,
        Edit,
        Split,
        CopyCode,
        CycleMessageRole,
        QuoteSelection,
        InsertIntoEditor,
        ToggleFocus,
        InsertActivePrompt,
        DeployHistory,
        DeployPromptLibrary,
        ConfirmCommand,
        NewContext,
        ToggleModelSelector,
        CycleNextInlineAssist,
        CyclePreviousInlineAssist
    ]
);

#[derive(PartialEq, Clone)]
pub enum InsertDraggedFiles {
    ProjectPaths(Vec<PathBuf>),
    ExternalFiles(Vec<PathBuf>),
}

impl_internal_actions!(assistant, [InsertDraggedFiles]);

const DEFAULT_CONTEXT_LINES: usize = 50;

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
    Canceled,
}

impl MessageStatus {
    pub fn from_proto(status: proto::ContextMessageStatus) -> MessageStatus {
        match status.variant {
            Some(proto::context_message_status::Variant::Pending(_)) => MessageStatus::Pending,
            Some(proto::context_message_status::Variant::Done(_)) => MessageStatus::Done,
            Some(proto::context_message_status::Variant::Error(error)) => {
                MessageStatus::Error(error.message.into())
            }
            Some(proto::context_message_status::Variant::Canceled(_)) => MessageStatus::Canceled,
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
            MessageStatus::Canceled => proto::ContextMessageStatus {
                variant: Some(proto::context_message_status::Variant::Canceled(
                    proto::context_message_status::Canceled {},
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

pub fn init(
    fs: Arc<dyn Fs>,
    client: Arc<Client>,
    stdout_is_a_pty: bool,
    cx: &mut AppContext,
) -> Arc<PromptBuilder> {
    cx.set_global(Assistant::default());
    AssistantSettings::register(cx);
    SlashCommandSettings::register(cx);

    cx.spawn(|mut cx| {
        let client = client.clone();
        async move {
            let is_search_slash_command_enabled = cx
                .update(|cx| cx.wait_for_flag::<SearchSlashCommandFeatureFlag>())?
                .await;
            let is_project_slash_command_enabled = cx
                .update(|cx| cx.wait_for_flag::<ProjectSlashCommandFeatureFlag>())?
                .await;

            if !is_search_slash_command_enabled && !is_project_slash_command_enabled {
                return Ok(());
            }

            let embedding_provider = CloudEmbeddingProvider::new(client.clone());
            let semantic_index = SemanticDb::new(
                paths::embeddings_dir().join("semantic-index-db.0.mdb"),
                Arc::new(embedding_provider),
                &mut cx,
            )
            .await?;

            cx.update(|cx| cx.set_global(semantic_index))
        }
    })
    .detach();

    context_store::init(&client.clone().into());
    prompt_library::init(cx);
    init_language_model_settings(cx);
    assistant_slash_command::init(cx);
    assistant_tool::init(cx);
    assistant_panel::init(cx);
    context_server::init(cx);

    let prompt_builder = PromptBuilder::new(Some(PromptLoadingParams {
        fs: fs.clone(),
        repo_path: stdout_is_a_pty
            .then(|| std::env::current_dir().log_err())
            .flatten(),
        cx,
    }))
    .log_err()
    .map(Arc::new)
    .unwrap_or_else(|| Arc::new(PromptBuilder::new(None).unwrap()));
    register_slash_commands(Some(prompt_builder.clone()), cx);
    inline_assistant::init(
        fs.clone(),
        prompt_builder.clone(),
        client.telemetry().clone(),
        cx,
    );
    terminal_inline_assistant::init(
        fs.clone(),
        prompt_builder.clone(),
        client.telemetry().clone(),
        cx,
    );
    indexed_docs::init(cx);

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

    prompt_builder
}

fn init_language_model_settings(cx: &mut AppContext) {
    update_active_language_model_from_settings(cx);

    cx.observe_global::<SettingsStore>(update_active_language_model_from_settings)
        .detach();
    cx.subscribe(
        &LanguageModelRegistry::global(cx),
        |_, event: &language_model::Event, cx| match event {
            language_model::Event::ProviderStateChanged
            | language_model::Event::AddedProvider(_)
            | language_model::Event::RemovedProvider(_) => {
                update_active_language_model_from_settings(cx);
            }
            _ => {}
        },
    )
    .detach();
}

fn update_active_language_model_from_settings(cx: &mut AppContext) {
    let settings = AssistantSettings::get_global(cx);
    let provider_name = LanguageModelProviderId::from(settings.default_model.provider.clone());
    let model_id = LanguageModelId::from(settings.default_model.model.clone());
    let inline_alternatives = settings
        .inline_alternatives
        .iter()
        .map(|alternative| {
            (
                LanguageModelProviderId::from(alternative.provider.clone()),
                LanguageModelId::from(alternative.model.clone()),
            )
        })
        .collect::<Vec<_>>();
    LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
        registry.select_active_model(&provider_name, &model_id, cx);
        registry.select_inline_alternative_models(inline_alternatives, cx);
    });
}

fn register_slash_commands(prompt_builder: Option<Arc<PromptBuilder>>, cx: &mut AppContext) {
    let slash_command_registry = SlashCommandRegistry::global(cx);

    slash_command_registry.register_command(assistant_slash_commands::FileSlashCommand, true);
    slash_command_registry.register_command(assistant_slash_commands::DeltaSlashCommand, true);
    slash_command_registry.register_command(assistant_slash_commands::OutlineSlashCommand, true);
    slash_command_registry.register_command(assistant_slash_commands::TabSlashCommand, true);
    slash_command_registry
        .register_command(assistant_slash_commands::CargoWorkspaceSlashCommand, true);
    slash_command_registry.register_command(assistant_slash_commands::PromptSlashCommand, true);
    slash_command_registry.register_command(assistant_slash_commands::SelectionCommand, true);
    slash_command_registry.register_command(assistant_slash_commands::DefaultSlashCommand, false);
    slash_command_registry.register_command(assistant_slash_commands::TerminalSlashCommand, true);
    slash_command_registry.register_command(assistant_slash_commands::NowSlashCommand, false);
    slash_command_registry
        .register_command(assistant_slash_commands::DiagnosticsSlashCommand, true);
    slash_command_registry.register_command(assistant_slash_commands::FetchSlashCommand, true);

    if let Some(prompt_builder) = prompt_builder {
        cx.observe_flag::<assistant_slash_commands::ProjectSlashCommandFeatureFlag, _>({
            let slash_command_registry = slash_command_registry.clone();
            move |is_enabled, _cx| {
                if is_enabled {
                    slash_command_registry.register_command(
                        assistant_slash_commands::ProjectSlashCommand::new(prompt_builder.clone()),
                        true,
                    );
                }
            }
        })
        .detach();
    }

    cx.observe_flag::<assistant_slash_commands::AutoSlashCommandFeatureFlag, _>({
        let slash_command_registry = slash_command_registry.clone();
        move |is_enabled, _cx| {
            if is_enabled {
                // [#auto-staff-ship] TODO remove this when /auto is no longer staff-shipped
                slash_command_registry
                    .register_command(assistant_slash_commands::AutoCommand, true);
            }
        }
    })
    .detach();

    cx.observe_flag::<assistant_slash_commands::StreamingExampleSlashCommandFeatureFlag, _>({
        let slash_command_registry = slash_command_registry.clone();
        move |is_enabled, _cx| {
            if is_enabled {
                slash_command_registry.register_command(
                    assistant_slash_commands::StreamingExampleSlashCommand,
                    false,
                );
            }
        }
    })
    .detach();

    update_slash_commands_from_settings(cx);
    cx.observe_global::<SettingsStore>(update_slash_commands_from_settings)
        .detach();

    cx.observe_flag::<assistant_slash_commands::SearchSlashCommandFeatureFlag, _>({
        let slash_command_registry = slash_command_registry.clone();
        move |is_enabled, _cx| {
            if is_enabled {
                slash_command_registry
                    .register_command(assistant_slash_commands::SearchSlashCommand, true);
            }
        }
    })
    .detach();
}

fn update_slash_commands_from_settings(cx: &mut AppContext) {
    let slash_command_registry = SlashCommandRegistry::global(cx);
    let settings = SlashCommandSettings::get_global(cx);

    if settings.docs.enabled {
        slash_command_registry.register_command(assistant_slash_commands::DocsSlashCommand, true);
    } else {
        slash_command_registry.unregister_command(assistant_slash_commands::DocsSlashCommand);
    }

    if settings.cargo_workspace.enabled {
        slash_command_registry
            .register_command(assistant_slash_commands::CargoWorkspaceSlashCommand, true);
    } else {
        slash_command_registry
            .unregister_command(assistant_slash_commands::CargoWorkspaceSlashCommand);
    }
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
