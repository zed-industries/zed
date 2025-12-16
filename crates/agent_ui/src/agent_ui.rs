pub mod acp;
mod agent_configuration;
mod agent_diff;
mod agent_model_selector;
mod agent_panel;
mod buffer_codegen;
mod completion_provider;
mod context;
mod context_server_configuration;
mod favorite_models;
mod inline_assistant;
mod inline_prompt_editor;
mod language_model_selector;
mod mention_set;
mod profile_selector;
mod slash_command;
mod slash_command_picker;
mod terminal_codegen;
mod terminal_inline_assistant;
mod text_thread_editor;
mod ui;

use std::rc::Rc;
use std::sync::Arc;

use agent_settings::{AgentProfileId, AgentSettings};
use assistant_slash_command::SlashCommandRegistry;
use client::Client;
use command_palette_hooks::CommandPaletteFilter;
use feature_flags::{AgentV2FeatureFlag, FeatureFlagAppExt as _};
use fs::Fs;
use gpui::{Action, App, Entity, SharedString, actions};
use language::{
    LanguageRegistry,
    language_settings::{AllLanguageSettings, EditPredictionProvider},
};
use language_model::{
    ConfiguredModel, LanguageModelId, LanguageModelProviderId, LanguageModelRegistry,
};
use project::DisableAiSettings;
use prompt_store::PromptBuilder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{LanguageModelSelection, Settings as _, SettingsStore};
use std::any::TypeId;

use crate::agent_configuration::{ConfigureContextServerModal, ManageProfilesModal};
pub use crate::agent_panel::{AgentPanel, ConcreteAssistantPanelDelegate};
pub use crate::inline_assistant::InlineAssistant;
pub use agent_diff::{AgentDiffPane, AgentDiffToolbar};
pub use text_thread_editor::{AgentPanelDelegate, TextThreadEditor};
use zed_actions;

actions!(
    agent,
    [
        /// Creates a new text-based conversation thread.
        NewTextThread,
        /// Toggles the menu to create new agent threads.
        ToggleNewThreadMenu,
        /// Toggles the navigation menu for switching between threads and views.
        ToggleNavigationMenu,
        /// Toggles the options menu for agent settings and preferences.
        ToggleOptionsMenu,
        /// Deletes the recently opened thread from history.
        DeleteRecentlyOpenThread,
        /// Toggles the profile or mode selector for switching between agent profiles.
        ToggleProfileSelector,
        /// Cycles through available session modes.
        CycleModeSelector,
        /// Cycles through favorited models in the ACP model selector.
        CycleFavoriteModels,
        /// Expands the message editor to full size.
        ExpandMessageEditor,
        /// Removes all thread history.
        RemoveHistory,
        /// Opens the conversation history view.
        OpenHistory,
        /// Adds a context server to the configuration.
        AddContextServer,
        /// Removes the currently selected thread.
        RemoveSelectedThread,
        /// Starts a chat conversation with follow-up enabled.
        ChatWithFollow,
        /// Cycles to the next inline assist suggestion.
        CycleNextInlineAssist,
        /// Cycles to the previous inline assist suggestion.
        CyclePreviousInlineAssist,
        /// Moves focus up in the interface.
        FocusUp,
        /// Moves focus down in the interface.
        FocusDown,
        /// Moves focus left in the interface.
        FocusLeft,
        /// Moves focus right in the interface.
        FocusRight,
        /// Opens the active thread as a markdown file.
        OpenActiveThreadAsMarkdown,
        /// Opens the agent diff view to review changes.
        OpenAgentDiff,
        /// Keeps the current suggestion or change.
        Keep,
        /// Rejects the current suggestion or change.
        Reject,
        /// Rejects all suggestions or changes.
        RejectAll,
        /// Keeps all suggestions or changes.
        KeepAll,
        /// Allow this operation only this time.
        AllowOnce,
        /// Allow this operation and remember the choice.
        AllowAlways,
        /// Reject this operation only this time.
        RejectOnce,
        /// Follows the agent's suggestions.
        Follow,
        /// Resets the trial upsell notification.
        ResetTrialUpsell,
        /// Resets the trial end upsell notification.
        ResetTrialEndUpsell,
        /// Continues the current thread.
        ContinueThread,
        /// Continues the thread with burn mode enabled.
        ContinueWithBurnMode,
        /// Toggles burn mode for faster responses.
        ToggleBurnMode,
    ]
);

/// Creates a new conversation thread, optionally based on an existing thread.
#[derive(Default, Clone, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = agent)]
#[serde(deny_unknown_fields)]
pub struct NewThread;

/// Creates a new external agent conversation thread.
#[derive(Default, Clone, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = agent)]
#[serde(deny_unknown_fields)]
pub struct NewExternalAgentThread {
    /// Which agent to use for the conversation.
    agent: Option<ExternalAgent>,
}

#[derive(Clone, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = agent)]
#[serde(deny_unknown_fields)]
pub struct NewNativeAgentThreadFromSummary {
    from_session_id: agent_client_protocol::SessionId,
}

// TODO unify this with AgentType
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExternalAgent {
    Gemini,
    ClaudeCode,
    Codex,
    NativeAgent,
    Custom { name: SharedString },
}

impl ExternalAgent {
    pub fn server(
        &self,
        fs: Arc<dyn fs::Fs>,
        history: Entity<agent::HistoryStore>,
    ) -> Rc<dyn agent_servers::AgentServer> {
        match self {
            Self::Gemini => Rc::new(agent_servers::Gemini),
            Self::ClaudeCode => Rc::new(agent_servers::ClaudeCode),
            Self::Codex => Rc::new(agent_servers::Codex),
            Self::NativeAgent => Rc::new(agent::NativeAgentServer::new(fs, history)),
            Self::Custom { name } => Rc::new(agent_servers::CustomAgentServer::new(name.clone())),
        }
    }

    pub fn is_mcp(&self) -> bool {
        match self {
            Self::Gemini => true,
            Self::ClaudeCode => true,
            Self::Codex => true,
            Self::NativeAgent => false,
            Self::Custom { .. } => false,
        }
    }
}

/// Opens the profile management interface for configuring agent tools and settings.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = agent)]
#[serde(deny_unknown_fields)]
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

#[derive(Clone)]
pub(crate) enum ModelUsageContext {
    InlineAssistant,
}

impl ModelUsageContext {
    pub fn configured_model(&self, cx: &App) -> Option<ConfiguredModel> {
        match self {
            Self::InlineAssistant => {
                LanguageModelRegistry::read_global(cx).inline_assistant_model()
            }
        }
    }
}

/// Initializes the `agent` crate.
pub fn init(
    fs: Arc<dyn Fs>,
    client: Arc<Client>,
    prompt_builder: Arc<PromptBuilder>,
    language_registry: Arc<LanguageRegistry>,
    is_eval: bool,
    cx: &mut App,
) {
    assistant_text_thread::init(client, cx);
    rules_library::init(cx);
    if !is_eval {
        // Initializing the language model from the user settings messes with the eval, so we only initialize them when
        // we're not running inside of the eval.
        init_language_model_settings(cx);
    }
    assistant_slash_command::init(cx);
    agent_panel::init(cx);
    context_server_configuration::init(language_registry.clone(), fs.clone(), cx);
    TextThreadEditor::init(cx);

    register_slash_commands(cx);
    inline_assistant::init(fs.clone(), prompt_builder.clone(), cx);
    terminal_inline_assistant::init(fs.clone(), prompt_builder, cx);
    cx.observe_new(move |workspace, window, cx| {
        ConfigureContextServerModal::register(workspace, language_registry.clone(), window, cx)
    })
    .detach();
    cx.observe_new(ManageProfilesModal::register).detach();

    // Update command palette filter based on AI settings
    update_command_palette_filter(cx);

    // Watch for settings changes
    cx.observe_global::<SettingsStore>(|app_cx| {
        // When settings change, update the command palette filter
        update_command_palette_filter(app_cx);
    })
    .detach();

    cx.on_flags_ready(|_, cx| {
        update_command_palette_filter(cx);
    })
    .detach();
}

fn update_command_palette_filter(cx: &mut App) {
    let disable_ai = DisableAiSettings::get_global(cx).disable_ai;
    let agent_enabled = AgentSettings::get_global(cx).enabled;
    let agent_v2_enabled = cx.has_flag::<AgentV2FeatureFlag>();
    let edit_prediction_provider = AllLanguageSettings::get_global(cx)
        .edit_predictions
        .provider;

    CommandPaletteFilter::update_global(cx, |filter, _| {
        use editor::actions::{
            AcceptEditPrediction, AcceptNextLineEditPrediction, AcceptNextWordEditPrediction,
            NextEditPrediction, PreviousEditPrediction, ShowEditPrediction, ToggleEditPrediction,
        };
        let edit_prediction_actions = [
            TypeId::of::<AcceptEditPrediction>(),
            TypeId::of::<AcceptNextWordEditPrediction>(),
            TypeId::of::<AcceptNextLineEditPrediction>(),
            TypeId::of::<AcceptEditPrediction>(),
            TypeId::of::<ShowEditPrediction>(),
            TypeId::of::<NextEditPrediction>(),
            TypeId::of::<PreviousEditPrediction>(),
            TypeId::of::<ToggleEditPrediction>(),
        ];

        if disable_ai {
            filter.hide_namespace("agent");
            filter.hide_namespace("agents");
            filter.hide_namespace("assistant");
            filter.hide_namespace("copilot");
            filter.hide_namespace("supermaven");
            filter.hide_namespace("zed_predict_onboarding");
            filter.hide_namespace("edit_prediction");

            filter.hide_action_types(&edit_prediction_actions);
            filter.hide_action_types(&[TypeId::of::<zed_actions::OpenZedPredictOnboarding>()]);
        } else {
            if agent_enabled {
                filter.show_namespace("agent");
                filter.show_namespace("agents");
            } else {
                filter.hide_namespace("agent");
                filter.hide_namespace("agents");
            }

            filter.show_namespace("assistant");

            match edit_prediction_provider {
                EditPredictionProvider::None => {
                    filter.hide_namespace("edit_prediction");
                    filter.hide_namespace("copilot");
                    filter.hide_namespace("supermaven");
                    filter.hide_action_types(&edit_prediction_actions);
                }
                EditPredictionProvider::Copilot => {
                    filter.show_namespace("edit_prediction");
                    filter.show_namespace("copilot");
                    filter.hide_namespace("supermaven");
                    filter.show_action_types(edit_prediction_actions.iter());
                }
                EditPredictionProvider::Supermaven => {
                    filter.show_namespace("edit_prediction");
                    filter.hide_namespace("copilot");
                    filter.show_namespace("supermaven");
                    filter.show_action_types(edit_prediction_actions.iter());
                }
                EditPredictionProvider::Zed
                | EditPredictionProvider::Codestral
                | EditPredictionProvider::Experimental(_) => {
                    filter.show_namespace("edit_prediction");
                    filter.hide_namespace("copilot");
                    filter.hide_namespace("supermaven");
                    filter.show_action_types(edit_prediction_actions.iter());
                }
            }

            filter.show_namespace("zed_predict_onboarding");
            filter.show_action_types(&[TypeId::of::<zed_actions::OpenZedPredictOnboarding>()]);
            if !agent_v2_enabled {
                filter.hide_action_types(&[TypeId::of::<zed_actions::agent::ToggleAgentPane>()]);
            }
        }
    });
}

fn init_language_model_settings(cx: &mut App) {
    update_active_language_model_from_settings(cx);

    cx.observe_global::<SettingsStore>(update_active_language_model_from_settings)
        .detach();
    cx.subscribe(
        &LanguageModelRegistry::global(cx),
        |_, event: &language_model::Event, cx| match event {
            language_model::Event::ProviderStateChanged(_)
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
    let settings = AgentSettings::get_global(cx);

    fn to_selected_model(selection: &LanguageModelSelection) -> language_model::SelectedModel {
        language_model::SelectedModel {
            provider: LanguageModelProviderId::from(selection.provider.0.clone()),
            model: LanguageModelId::from(selection.model.clone()),
        }
    }

    let default = settings.default_model.as_ref().map(to_selected_model);
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
        registry.select_default_model(default.as_ref(), cx);
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
    slash_command_registry.register_command(assistant_slash_commands::PromptSlashCommand, true);
    slash_command_registry.register_command(assistant_slash_commands::SelectionCommand, true);
    slash_command_registry.register_command(assistant_slash_commands::DefaultSlashCommand, false);
    slash_command_registry.register_command(assistant_slash_commands::NowSlashCommand, false);
    slash_command_registry
        .register_command(assistant_slash_commands::DiagnosticsSlashCommand, true);
    slash_command_registry.register_command(assistant_slash_commands::FetchSlashCommand, true);

    cx.observe_flag::<assistant_slash_commands::StreamingExampleSlashCommandFeatureFlag, _>({
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent_settings::{AgentProfileId, AgentSettings, CompletionMode};
    use command_palette_hooks::CommandPaletteFilter;
    use editor::actions::AcceptEditPrediction;
    use gpui::{BorrowAppContext, TestAppContext, px};
    use project::DisableAiSettings;
    use settings::{
        DefaultAgentView, DockPosition, DockSide, NotifyWhenAgentWaiting, Settings, SettingsStore,
    };

    #[gpui::test]
    fn test_agent_command_palette_visibility(cx: &mut TestAppContext) {
        // Init settings
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            command_palette_hooks::init(cx);
            AgentSettings::register(cx);
            DisableAiSettings::register(cx);
            AllLanguageSettings::register(cx);
        });

        let agent_settings = AgentSettings {
            enabled: true,
            button: true,
            dock: DockPosition::Right,
            agents_panel_dock: DockSide::Left,
            default_width: px(300.),
            default_height: px(600.),
            default_model: None,
            inline_assistant_model: None,
            inline_assistant_use_streaming_tools: false,
            commit_message_model: None,
            thread_summary_model: None,
            inline_alternatives: vec![],
            favorite_models: vec![],
            default_profile: AgentProfileId::default(),
            default_view: DefaultAgentView::Thread,
            profiles: Default::default(),
            always_allow_tool_actions: false,
            notify_when_agent_waiting: NotifyWhenAgentWaiting::default(),
            play_sound_when_agent_done: false,
            single_file_review: false,
            model_parameters: vec![],
            preferred_completion_mode: CompletionMode::Normal,
            enable_feedback: false,
            expand_edit_card: true,
            expand_terminal_card: true,
            use_modifier_to_send: true,
            message_editor_min_lines: 1,
        };

        cx.update(|cx| {
            AgentSettings::override_global(agent_settings.clone(), cx);
            DisableAiSettings::override_global(DisableAiSettings { disable_ai: false }, cx);

            // Initial update
            update_command_palette_filter(cx);
        });

        // Assert visible
        cx.update(|cx| {
            let filter = CommandPaletteFilter::try_global(cx).unwrap();
            assert!(
                !filter.is_hidden(&NewThread),
                "NewThread should be visible by default"
            );
        });

        // Disable agent
        cx.update(|cx| {
            let mut new_settings = agent_settings.clone();
            new_settings.enabled = false;
            AgentSettings::override_global(new_settings, cx);

            // Trigger update
            update_command_palette_filter(cx);
        });

        // Assert hidden
        cx.update(|cx| {
            let filter = CommandPaletteFilter::try_global(cx).unwrap();
            assert!(
                filter.is_hidden(&NewThread),
                "NewThread should be hidden when agent is disabled"
            );
        });

        // Test EditPredictionProvider
        // Enable EditPredictionProvider::Copilot
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.project
                        .all_languages
                        .features
                        .get_or_insert(Default::default())
                        .edit_prediction_provider = Some(EditPredictionProvider::Copilot);
                });
            });
            update_command_palette_filter(cx);
        });

        cx.update(|cx| {
            let filter = CommandPaletteFilter::try_global(cx).unwrap();
            assert!(
                !filter.is_hidden(&AcceptEditPrediction),
                "EditPrediction should be visible when provider is Copilot"
            );
        });

        // Disable EditPredictionProvider (None)
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.project
                        .all_languages
                        .features
                        .get_or_insert(Default::default())
                        .edit_prediction_provider = Some(EditPredictionProvider::None);
                });
            });
            update_command_palette_filter(cx);
        });

        cx.update(|cx| {
            let filter = CommandPaletteFilter::try_global(cx).unwrap();
            assert!(
                filter.is_hidden(&AcceptEditPrediction),
                "EditPrediction should be hidden when provider is None"
            );
        });
    }
}
