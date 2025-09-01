mod agent_profile;

use std::sync::Arc;

use anyhow::{Result, bail};
use collections::IndexMap;
use gpui::{App, Pixels, SharedString};
use language_model::LanguageModel;
use schemars::{JsonSchema, json_schema};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources, SettingsUi};
use std::borrow::Cow;

pub use crate::agent_profile::*;

pub const SUMMARIZE_THREAD_PROMPT: &str =
    include_str!("../../agent/src/prompts/summarize_thread_prompt.txt");
pub const SUMMARIZE_THREAD_DETAILED_PROMPT: &str =
    include_str!("../../agent/src/prompts/summarize_thread_detailed_prompt.txt");

pub fn init(cx: &mut App) {
    AgentSettings::register(cx);
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AgentDockPosition {
    Left,
    #[default]
    Right,
    Bottom,
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DefaultView {
    #[default]
    Thread,
    TextThread,
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NotifyWhenAgentWaiting {
    #[default]
    PrimaryScreen,
    AllScreens,
    Never,
}

#[derive(Default, Clone, Debug, SettingsUi)]
pub struct AgentSettings {
    pub enabled: bool,
    pub button: bool,
    pub dock: AgentDockPosition,
    pub default_width: Pixels,
    pub default_height: Pixels,
    pub default_model: Option<LanguageModelSelection>,
    pub inline_assistant_model: Option<LanguageModelSelection>,
    pub commit_message_model: Option<LanguageModelSelection>,
    pub thread_summary_model: Option<LanguageModelSelection>,
    pub inline_alternatives: Vec<LanguageModelSelection>,
    pub using_outdated_settings_version: bool,
    pub default_profile: AgentProfileId,
    pub default_view: DefaultView,
    pub profiles: IndexMap<AgentProfileId, AgentProfileSettings>,
    pub always_allow_tool_actions: bool,
    pub notify_when_agent_waiting: NotifyWhenAgentWaiting,
    pub play_sound_when_agent_done: bool,
    pub stream_edits: bool,
    pub single_file_review: bool,
    pub model_parameters: Vec<LanguageModelParameters>,
    pub preferred_completion_mode: CompletionMode,
    pub enable_feedback: bool,
    pub expand_edit_card: bool,
    pub expand_terminal_card: bool,
    pub use_modifier_to_send: bool,
}

impl AgentSettings {
    pub fn temperature_for_model(model: &Arc<dyn LanguageModel>, cx: &App) -> Option<f32> {
        let settings = Self::get_global(cx);
        settings
            .model_parameters
            .iter()
            .rfind(|setting| setting.matches(model))
            .and_then(|m| m.temperature)
    }

    pub fn set_inline_assistant_model(&mut self, provider: String, model: String) {
        self.inline_assistant_model = Some(LanguageModelSelection {
            provider: provider.into(),
            model,
        });
    }

    pub fn set_commit_message_model(&mut self, provider: String, model: String) {
        self.commit_message_model = Some(LanguageModelSelection {
            provider: provider.into(),
            model,
        });
    }

    pub fn set_thread_summary_model(&mut self, provider: String, model: String) {
        self.thread_summary_model = Some(LanguageModelSelection {
            provider: provider.into(),
            model,
        });
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct LanguageModelParameters {
    pub provider: Option<LanguageModelProviderSetting>,
    pub model: Option<SharedString>,
    pub temperature: Option<f32>,
}

impl LanguageModelParameters {
    pub fn matches(&self, model: &Arc<dyn LanguageModel>) -> bool {
        if let Some(provider) = &self.provider
            && provider.0 != model.provider_id().0
        {
            return false;
        }
        if let Some(setting_model) = &self.model
            && *setting_model != model.id().0
        {
            return false;
        }
        true
    }
}

impl AgentSettingsContent {
    pub fn set_dock(&mut self, dock: AgentDockPosition) {
        self.dock = Some(dock);
    }

    pub fn set_model(&mut self, language_model: Arc<dyn LanguageModel>) {
        let model = language_model.id().0.to_string();
        let provider = language_model.provider_id().0.to_string();

        self.default_model = Some(LanguageModelSelection {
            provider: provider.into(),
            model,
        });
    }

    pub fn set_inline_assistant_model(&mut self, provider: String, model: String) {
        self.inline_assistant_model = Some(LanguageModelSelection {
            provider: provider.into(),
            model,
        });
    }

    pub fn set_commit_message_model(&mut self, provider: String, model: String) {
        self.commit_message_model = Some(LanguageModelSelection {
            provider: provider.into(),
            model,
        });
    }

    pub fn set_thread_summary_model(&mut self, provider: String, model: String) {
        self.thread_summary_model = Some(LanguageModelSelection {
            provider: provider.into(),
            model,
        });
    }

    pub fn set_always_allow_tool_actions(&mut self, allow: bool) {
        self.always_allow_tool_actions = Some(allow);
    }

    pub fn set_play_sound_when_agent_done(&mut self, allow: bool) {
        self.play_sound_when_agent_done = Some(allow);
    }

    pub fn set_single_file_review(&mut self, allow: bool) {
        self.single_file_review = Some(allow);
    }

    pub fn set_use_modifier_to_send(&mut self, always_use: bool) {
        self.use_modifier_to_send = Some(always_use);
    }

    pub fn set_profile(&mut self, profile_id: AgentProfileId) {
        self.default_profile = Some(profile_id);
    }

    pub fn create_profile(
        &mut self,
        profile_id: AgentProfileId,
        profile_settings: AgentProfileSettings,
    ) -> Result<()> {
        let profiles = self.profiles.get_or_insert_default();
        if profiles.contains_key(&profile_id) {
            bail!("profile with ID '{profile_id}' already exists");
        }

        profiles.insert(
            profile_id,
            AgentProfileContent {
                name: profile_settings.name.into(),
                tools: profile_settings.tools,
                enable_all_context_servers: Some(profile_settings.enable_all_context_servers),
                context_servers: profile_settings
                    .context_servers
                    .into_iter()
                    .map(|(server_id, preset)| {
                        (
                            server_id,
                            ContextServerPresetContent {
                                tools: preset.tools,
                            },
                        )
                    })
                    .collect(),
            },
        );

        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug, Default)]
pub struct AgentSettingsContent {
    /// Whether the Agent is enabled.
    ///
    /// Default: true
    enabled: Option<bool>,
    /// Whether to show the agent panel button in the status bar.
    ///
    /// Default: true
    button: Option<bool>,
    /// Where to dock the agent panel.
    ///
    /// Default: right
    dock: Option<AgentDockPosition>,
    /// Default width in pixels when the agent panel is docked to the left or right.
    ///
    /// Default: 640
    default_width: Option<f32>,
    /// Default height in pixels when the agent panel is docked to the bottom.
    ///
    /// Default: 320
    default_height: Option<f32>,
    /// The default model to use when creating new chats and for other features when a specific model is not specified.
    default_model: Option<LanguageModelSelection>,
    /// Model to use for the inline assistant. Defaults to default_model when not specified.
    inline_assistant_model: Option<LanguageModelSelection>,
    /// Model to use for generating git commit messages. Defaults to default_model when not specified.
    commit_message_model: Option<LanguageModelSelection>,
    /// Model to use for generating thread summaries. Defaults to default_model when not specified.
    thread_summary_model: Option<LanguageModelSelection>,
    /// Additional models with which to generate alternatives when performing inline assists.
    inline_alternatives: Option<Vec<LanguageModelSelection>>,
    /// The default profile to use in the Agent.
    ///
    /// Default: write
    default_profile: Option<AgentProfileId>,
    /// Which view type to show by default in the agent panel.
    ///
    /// Default: "thread"
    default_view: Option<DefaultView>,
    /// The available agent profiles.
    pub profiles: Option<IndexMap<AgentProfileId, AgentProfileContent>>,
    /// Whenever a tool action would normally wait for your confirmation
    /// that you allow it, always choose to allow it.
    ///
    /// Default: false
    always_allow_tool_actions: Option<bool>,
    /// Where to show a popup notification when the agent is waiting for user input.
    ///
    /// Default: "primary_screen"
    notify_when_agent_waiting: Option<NotifyWhenAgentWaiting>,
    /// Whether to play a sound when the agent has either completed its response, or needs user input.
    ///
    /// Default: false
    play_sound_when_agent_done: Option<bool>,
    /// Whether to stream edits from the agent as they are received.
    ///
    /// Default: false
    stream_edits: Option<bool>,
    /// Whether to display agent edits in single-file editors in addition to the review multibuffer pane.
    ///
    /// Default: true
    single_file_review: Option<bool>,
    /// Additional parameters for language model requests. When making a request
    /// to a model, parameters will be taken from the last entry in this list
    /// that matches the model's provider and name. In each entry, both provider
    /// and model are optional, so that you can specify parameters for either
    /// one.
    ///
    /// Default: []
    #[serde(default)]
    model_parameters: Vec<LanguageModelParameters>,
    /// What completion mode to enable for new threads
    ///
    /// Default: normal
    preferred_completion_mode: Option<CompletionMode>,
    /// Whether to show thumb buttons for feedback in the agent panel.
    ///
    /// Default: true
    enable_feedback: Option<bool>,
    /// Whether to have edit cards in the agent panel expanded, showing a preview of the full diff.
    ///
    /// Default: true
    expand_edit_card: Option<bool>,
    /// Whether to have terminal cards in the agent panel expanded, showing the whole command output.
    ///
    /// Default: true
    expand_terminal_card: Option<bool>,
    /// Whether to always use cmd-enter (or ctrl-enter on Linux or Windows) to send messages in the agent panel.
    ///
    /// Default: false
    use_modifier_to_send: Option<bool>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum CompletionMode {
    #[default]
    Normal,
    #[serde(alias = "max")]
    Burn,
}

impl From<CompletionMode> for cloud_llm_client::CompletionMode {
    fn from(value: CompletionMode) -> Self {
        match value {
            CompletionMode::Normal => cloud_llm_client::CompletionMode::Normal,
            CompletionMode::Burn => cloud_llm_client::CompletionMode::Max,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct LanguageModelSelection {
    pub provider: LanguageModelProviderSetting,
    pub model: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LanguageModelProviderSetting(pub String);

impl JsonSchema for LanguageModelProviderSetting {
    fn schema_name() -> Cow<'static, str> {
        "LanguageModelProviderSetting".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "enum": [
                "amazon-bedrock",
                "anthropic",
                "copilot_chat",
                "deepseek",
                "google",
                "lmstudio",
                "mistral",
                "ollama",
                "openai",
                "openrouter",
                "vercel",
                "x_ai",
                "zed.dev"
            ]
        })
    }
}

impl From<String> for LanguageModelProviderSetting {
    fn from(provider: String) -> Self {
        Self(provider)
    }
}

impl From<&str> for LanguageModelProviderSetting {
    fn from(provider: &str) -> Self {
        Self(provider.to_string())
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentProfileContent {
    pub name: Arc<str>,
    #[serde(default)]
    pub tools: IndexMap<Arc<str>, bool>,
    /// Whether all context servers are enabled by default.
    pub enable_all_context_servers: Option<bool>,
    #[serde(default)]
    pub context_servers: IndexMap<Arc<str>, ContextServerPresetContent>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ContextServerPresetContent {
    pub tools: IndexMap<Arc<str>, bool>,
}

impl Settings for AgentSettings {
    const KEY: Option<&'static str> = Some("agent");

    const FALLBACK_KEY: Option<&'static str> = Some("assistant");

    const PRESERVED_KEYS: Option<&'static [&'static str]> = Some(&["version"]);

    type FileContent = AgentSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::App,
    ) -> anyhow::Result<Self> {
        let mut settings = AgentSettings::default();

        for value in sources.defaults_and_customizations() {
            merge(&mut settings.enabled, value.enabled);
            merge(&mut settings.button, value.button);
            merge(&mut settings.dock, value.dock);
            merge(
                &mut settings.default_width,
                value.default_width.map(Into::into),
            );
            merge(
                &mut settings.default_height,
                value.default_height.map(Into::into),
            );
            settings.default_model = value
                .default_model
                .clone()
                .or(settings.default_model.take());
            settings.inline_assistant_model = value
                .inline_assistant_model
                .clone()
                .or(settings.inline_assistant_model.take());
            settings.commit_message_model = value
                .clone()
                .commit_message_model
                .or(settings.commit_message_model.take());
            settings.thread_summary_model = value
                .clone()
                .thread_summary_model
                .or(settings.thread_summary_model.take());
            merge(
                &mut settings.inline_alternatives,
                value.inline_alternatives.clone(),
            );
            merge(
                &mut settings.notify_when_agent_waiting,
                value.notify_when_agent_waiting,
            );
            merge(
                &mut settings.play_sound_when_agent_done,
                value.play_sound_when_agent_done,
            );
            merge(&mut settings.stream_edits, value.stream_edits);
            merge(&mut settings.single_file_review, value.single_file_review);
            merge(&mut settings.default_profile, value.default_profile.clone());
            merge(&mut settings.default_view, value.default_view);
            merge(
                &mut settings.preferred_completion_mode,
                value.preferred_completion_mode,
            );
            merge(&mut settings.enable_feedback, value.enable_feedback);
            merge(&mut settings.expand_edit_card, value.expand_edit_card);
            merge(
                &mut settings.expand_terminal_card,
                value.expand_terminal_card,
            );
            merge(
                &mut settings.use_modifier_to_send,
                value.use_modifier_to_send,
            );

            settings
                .model_parameters
                .extend_from_slice(&value.model_parameters);

            if let Some(profiles) = value.profiles.as_ref() {
                settings
                    .profiles
                    .extend(profiles.into_iter().map(|(id, profile)| {
                        (
                            id.clone(),
                            AgentProfileSettings {
                                name: profile.name.clone().into(),
                                tools: profile.tools.clone(),
                                enable_all_context_servers: profile
                                    .enable_all_context_servers
                                    .unwrap_or_default(),
                                context_servers: profile
                                    .context_servers
                                    .iter()
                                    .map(|(context_server_id, preset)| {
                                        (
                                            context_server_id.clone(),
                                            ContextServerPreset {
                                                tools: preset.tools.clone(),
                                            },
                                        )
                                    })
                                    .collect(),
                            },
                        )
                    }));
            }
        }

        debug_assert!(
            !sources.default.always_allow_tool_actions.unwrap_or(false),
            "For security, agent.always_allow_tool_actions should always be false in default.json. If it's true, that is a bug that should be fixed!"
        );

        // For security reasons, only trust the user's global settings for whether to always allow tool actions.
        // If this could be overridden locally, an attacker could (e.g. by committing to source control and
        // convincing you to switch branches) modify your project-local settings to disable the agent's safety checks.
        settings.always_allow_tool_actions = sources
            .user
            .and_then(|setting| setting.always_allow_tool_actions)
            .unwrap_or(false);

        Ok(settings)
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut Self::FileContent) {
        if let Some(b) = vscode
            .read_value("chat.agent.enabled")
            .and_then(|b| b.as_bool())
        {
            current.enabled = Some(b);
            current.button = Some(b);
        }
    }
}

fn merge<T>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}
