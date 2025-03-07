#![cfg_attr(target_os = "windows", allow(unused, dead_code))]

mod assistant_configuration;
pub mod assistant_panel;
mod inline_assistant;
pub mod slash_command_settings;
mod terminal_inline_assistant;

use std::sync::Arc;

use assistant_settings::AssistantSettings;
use assistant_slash_command::SlashCommandRegistry;
use assistant_slash_commands::{ProjectSlashCommandFeatureFlag, SearchSlashCommandFeatureFlag};
use client::Client;
use command_palette_hooks::CommandPaletteFilter;
use feature_flags::FeatureFlagAppExt;
use fs::Fs;
use gpui::{actions, App, Global, UpdateGlobal};
use language_model::{
    LanguageModelId, LanguageModelProviderId, LanguageModelRegistry, LanguageModelResponseMessage,
};
use prompt_store::PromptBuilder;
use semantic_index::{CloudEmbeddingProvider, SemanticDb};
use serde::Deserialize;
use settings::{Settings, SettingsStore};

pub use crate::assistant_panel::{AssistantPanel, AssistantPanelEvent};
pub(crate) use crate::inline_assistant::*;
use crate::slash_command_settings::SlashCommandSettings;

actions!(
    assistant,
    [
        InsertActivePrompt,
        DeployHistory,
        NewChat,
        CycleNextInlineAssist,
        CyclePreviousInlineAssist
    ]
);

const DEFAULT_CONTEXT_LINES: usize = 50;

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

/// The state pertaining to the Assistant.
#[derive(Default)]
struct Assistant {
    /// Whether the Assistant is enabled.
    enabled: bool,
}

impl Global for Assistant {}

impl Assistant {
    const NAMESPACE: &'static str = "assistant";

    fn set_enabled(&mut self, enabled: bool, cx: &mut App) {
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
    prompt_builder: Arc<PromptBuilder>,
    cx: &mut App,
) {
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

    assistant_context_editor::init(client.clone(), cx);
    prompt_library::init(cx);
    init_language_model_settings(cx);
    assistant_slash_command::init(cx);
    assistant_tool::init(cx);
    assistant_panel::init(cx);
    context_server::init(cx);

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
}

fn init_language_model_settings(cx: &mut App) {
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

fn update_active_language_model_from_settings(cx: &mut App) {
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

fn register_slash_commands(prompt_builder: Option<Arc<PromptBuilder>>, cx: &mut App) {
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

fn update_slash_commands_from_settings(cx: &mut App) {
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

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}
