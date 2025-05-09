mod active_thread;
mod agent_configuration;
mod agent_diff;
mod agent_model_selector;
mod agent_panel;
mod buffer_codegen;
mod context;
mod context_picker;
mod context_server_configuration;
mod context_server_tool;
mod context_store;
mod context_strip;
mod debug;
mod history_store;
mod inline_assistant;
mod inline_prompt_editor;
mod message_editor;
mod profile_selector;
mod slash_command_settings;
mod terminal_codegen;
mod terminal_inline_assistant;
mod thread;
mod thread_history;
mod thread_store;
mod tool_compatibility;
mod tool_use;
mod ui;

use std::sync::Arc;

use assistant_settings::{AgentProfileId, AssistantSettings, LanguageModelSelection};
use assistant_slash_command::SlashCommandRegistry;
use client::Client;
use feature_flags::FeatureFlagAppExt as _;
use fs::Fs;
use gpui::{App, actions, impl_actions};
use language::LanguageRegistry;
use language_model::{LanguageModelId, LanguageModelProviderId, LanguageModelRegistry};
use prompt_store::PromptBuilder;
use schemars::JsonSchema;
use serde::Deserialize;
use settings::{Settings as _, SettingsStore};
use thread::ThreadId;

pub use crate::active_thread::ActiveThread;
use crate::agent_configuration::{AddContextServerModal, ManageProfilesModal};
pub use crate::agent_panel::{AgentPanel, ConcreteAssistantPanelDelegate};
pub use crate::context::{ContextLoadResult, LoadedContext};
pub use crate::inline_assistant::InlineAssistant;
use crate::slash_command_settings::SlashCommandSettings;
pub use crate::thread::{Message, MessageSegment, Thread, ThreadEvent};
pub use crate::thread_store::{TextThreadStore, ThreadStore};
pub use agent_diff::{AgentDiffPane, AgentDiffToolbar};
pub use context_store::ContextStore;
pub use ui::preview::{all_agent_previews, get_agent_preview};

actions!(
    agent,
    [
        NewTextThread,
        ToggleContextPicker,
        ToggleNavigationMenu,
        ToggleOptionsMenu,
        DeleteRecentlyOpenThread,
        ToggleProfileSelector,
        RemoveAllContext,
        ExpandMessageEditor,
        OpenHistory,
        AddContextServer,
        RemoveSelectedThread,
        Chat,
        CycleNextInlineAssist,
        CyclePreviousInlineAssist,
        FocusUp,
        FocusDown,
        FocusLeft,
        FocusRight,
        RemoveFocusedContext,
        AcceptSuggestedContext,
        OpenActiveThreadAsMarkdown,
        OpenAgentDiff,
        Keep,
        Reject,
        RejectAll,
        KeepAll,
        Follow,
        ResetTrialUpsell,
        Close,
    ]
);

#[derive(Default, Clone, PartialEq, Deserialize, JsonSchema)]
pub struct NewThread {
    #[serde(default)]
    from_thread_id: Option<ThreadId>,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema)]
pub struct ManageProfiles {
    #[serde(default)]
    pub customize_tools: Option<AgentProfileId>,
}

impl ManageProfiles {
    pub fn customize_tools(profile_id: AgentProfileId) -> Self {
        Self {
            customize_tools: Some(profile_id),
        }
    }
}

impl_actions!(agent, [NewThread, ManageProfiles]);

/// Initializes the `agent` crate.
pub fn init(
    fs: Arc<dyn Fs>,
    client: Arc<Client>,
    prompt_builder: Arc<PromptBuilder>,
    language_registry: Arc<LanguageRegistry>,
    cx: &mut App,
) {
    AssistantSettings::register(cx);
    SlashCommandSettings::register(cx);

    assistant_context_editor::init(client.clone(), cx);
    rules_library::init(cx);
    init_language_model_settings(cx);
    assistant_slash_command::init(cx);
    thread_store::init(cx);
    agent_panel::init(cx);
    context_server_configuration::init(language_registry, cx);

    register_slash_commands(cx);
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
    cx.observe_new(AddContextServerModal::register).detach();
    cx.observe_new(ManageProfilesModal::register).detach();
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

    fn to_selected_model(selection: &LanguageModelSelection) -> language_model::SelectedModel {
        language_model::SelectedModel {
            provider: LanguageModelProviderId::from(selection.provider.0.clone()),
            model: LanguageModelId::from(selection.model.clone()),
        }
    }

    let default = to_selected_model(&settings.default_model);
    let inline_assistant = settings
        .inline_assistant_model
        .as_ref()
        .map(to_selected_model);
    let commit_message = settings
        .commit_message_model
        .as_ref()
        .map(to_selected_model);
    let thread_summary = settings
        .thread_summary_model
        .as_ref()
        .map(to_selected_model);
    let inline_alternatives = settings
        .inline_alternatives
        .iter()
        .map(to_selected_model)
        .collect::<Vec<_>>();

    LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
        registry.select_default_model(Some(&default), cx);
        registry.select_inline_assistant_model(inline_assistant.as_ref(), cx);
        registry.select_commit_message_model(commit_message.as_ref(), cx);
        registry.select_thread_summary_model(thread_summary.as_ref(), cx);
        registry.select_inline_alternative_models(inline_alternatives, cx);
    });
}

fn register_slash_commands(cx: &mut App) {
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
