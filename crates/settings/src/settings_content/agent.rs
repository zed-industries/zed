use collections::{HashMap, IndexMap};
use gpui::SharedString;
use schemars::{JsonSchema, json_schema};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use settings_macros::MergeFrom;
use std::{borrow::Cow, path::PathBuf, sync::Arc};

use crate::DockPosition;

#[skip_serializing_none]
#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, Default)]
pub struct AgentSettingsContent {
    /// Whether the Agent is enabled.
    ///
    /// Default: true
    pub enabled: Option<bool>,
    /// Whether to show the agent panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Where to dock the agent panel.
    ///
    /// Default: right
    pub dock: Option<DockPosition>,
    /// Default width in pixels when the agent panel is docked to the left or right.
    ///
    /// Default: 640
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub default_width: Option<f32>,
    /// Default height in pixels when the agent panel is docked to the bottom.
    ///
    /// Default: 320
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub default_height: Option<f32>,
    /// The default model to use when creating new chats and for other features when a specific model is not specified.
    pub default_model: Option<LanguageModelSelection>,
    /// Model to use for the inline assistant. Defaults to default_model when not specified.
    pub inline_assistant_model: Option<LanguageModelSelection>,
    /// Model to use for generating git commit messages. Defaults to default_model when not specified.
    pub commit_message_model: Option<LanguageModelSelection>,
    /// Model to use for generating thread summaries. Defaults to default_model when not specified.
    pub thread_summary_model: Option<LanguageModelSelection>,
    /// Additional models with which to generate alternatives when performing inline assists.
    pub inline_alternatives: Option<Vec<LanguageModelSelection>>,
    /// The default profile to use in the Agent.
    ///
    /// Default: write
    pub default_profile: Option<Arc<str>>,
    /// Which view type to show by default in the agent panel.
    ///
    /// Default: "thread"
    pub default_view: Option<DefaultAgentView>,
    /// The available agent profiles.
    pub profiles: Option<IndexMap<Arc<str>, AgentProfileContent>>,
    /// Whenever a tool action would normally wait for your confirmation
    /// that you allow it, always choose to allow it.
    ///
    /// This setting has no effect on external agents that support permission modes, such as Claude Code.
    ///
    /// Set `agent_servers.claude.default_mode` to `bypassPermissions`, to disable all permission requests when using Claude Code.
    ///
    /// Default: false
    pub always_allow_tool_actions: Option<bool>,
    /// Where to show a popup notification when the agent is waiting for user input.
    ///
    /// Default: "primary_screen"
    pub notify_when_agent_waiting: Option<NotifyWhenAgentWaiting>,
    /// Whether to play a sound when the agent has either completed its response, or needs user input.
    ///
    /// Default: false
    pub play_sound_when_agent_done: Option<bool>,
    /// Whether to display agent edits in single-file editors in addition to the review multibuffer pane.
    ///
    /// Default: true
    pub single_file_review: Option<bool>,
    /// Additional parameters for language model requests. When making a request
    /// to a model, parameters will be taken from the last entry in this list
    /// that matches the model's provider and name. In each entry, both provider
    /// and model are optional, so that you can specify parameters for either
    /// one.
    ///
    /// Default: []
    #[serde(default)]
    pub model_parameters: Vec<LanguageModelParameters>,
    /// What completion mode to enable for new threads
    ///
    /// Default: normal
    pub preferred_completion_mode: Option<CompletionMode>,
    /// Whether to show thumb buttons for feedback in the agent panel.
    ///
    /// Default: true
    pub enable_feedback: Option<bool>,
    /// Whether to have edit cards in the agent panel expanded, showing a preview of the full diff.
    ///
    /// Default: true
    pub expand_edit_card: Option<bool>,
    /// Whether to have terminal cards in the agent panel expanded, showing the whole command output.
    ///
    /// Default: true
    pub expand_terminal_card: Option<bool>,
    /// Whether to always use cmd-enter (or ctrl-enter on Linux or Windows) to send messages in the agent panel.
    ///
    /// Default: false
    pub use_modifier_to_send: Option<bool>,
    /// Minimum number of lines of height the agent message editor should have.
    ///
    /// Default: 4
    pub message_editor_min_lines: Option<usize>,
}

impl AgentSettingsContent {
    pub fn set_dock(&mut self, dock: DockPosition) {
        self.dock = Some(dock);
    }

    pub fn set_model(&mut self, language_model: LanguageModelSelection) {
        // let model = language_model.id().0.to_string();
        // let provider = language_model.provider_id().0.to_string();
        // self.default_model = Some(LanguageModelSelection {
        //     provider: provider.into(),
        //     model,
        // });
        self.default_model = Some(language_model)
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

    pub fn set_profile(&mut self, profile_id: Arc<str>) {
        self.default_profile = Some(profile_id);
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct AgentProfileContent {
    pub name: Arc<str>,
    #[serde(default)]
    pub tools: IndexMap<Arc<str>, bool>,
    /// Whether all context servers are enabled by default.
    pub enable_all_context_servers: Option<bool>,
    #[serde(default)]
    pub context_servers: IndexMap<Arc<str>, ContextServerPresetContent>,
    /// The default language model selected when using this profile.
    pub default_model: Option<LanguageModelSelection>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ContextServerPresetContent {
    pub tools: IndexMap<Arc<str>, bool>,
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum DefaultAgentView {
    #[default]
    Thread,
    TextThread,
}

#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum NotifyWhenAgentWaiting {
    #[default]
    PrimaryScreen,
    AllScreens,
    Never,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct LanguageModelSelection {
    pub provider: LanguageModelProviderSetting,
    pub model: String,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum CompletionMode {
    #[default]
    Normal,
    #[serde(alias = "max")]
    Burn,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct LanguageModelParameters {
    pub provider: Option<LanguageModelProviderSetting>,
    pub model: Option<SharedString>,
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub temperature: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, MergeFrom)]
pub struct LanguageModelProviderSetting(pub String);

impl JsonSchema for LanguageModelProviderSetting {
    fn schema_name() -> Cow<'static, str> {
        "LanguageModelProviderSetting".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        // list the builtin providers as a subset so that we still auto complete them in the settings
        json_schema!({
            "anyOf": [
                {
                    "type": "string",
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
                },
                {
                    "type": "string",
                }
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

#[skip_serializing_none]
#[derive(Default, PartialEq, Deserialize, Serialize, Clone, JsonSchema, MergeFrom, Debug)]
pub struct AllAgentServersSettings {
    pub gemini: Option<BuiltinAgentServerSettings>,
    pub claude: Option<BuiltinAgentServerSettings>,
    pub codex: Option<BuiltinAgentServerSettings>,

    /// Custom agent servers configured by the user
    #[serde(flatten)]
    pub custom: HashMap<SharedString, CustomAgentServerSettings>,
}

#[skip_serializing_none]
#[derive(Default, Deserialize, Serialize, Clone, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct BuiltinAgentServerSettings {
    /// Absolute path to a binary to be used when launching this agent.
    ///
    /// This can be used to run a specific binary without automatic downloads or searching `$PATH`.
    #[serde(rename = "command")]
    pub path: Option<PathBuf>,
    /// If a binary is specified in `command`, it will be passed these arguments.
    pub args: Option<Vec<String>>,
    /// If a binary is specified in `command`, it will be passed these environment variables.
    pub env: Option<HashMap<String, String>>,
    /// Whether to skip searching `$PATH` for an agent server binary when
    /// launching this agent.
    ///
    /// This has no effect if a `command` is specified. Otherwise, when this is
    /// `false`, Zed will search `$PATH` for an agent server binary and, if one
    /// is found, use it for threads with this agent. If no agent binary is
    /// found on `$PATH`, Zed will automatically install and use its own binary.
    /// When this is `true`, Zed will not search `$PATH`, and will always use
    /// its own binary.
    ///
    /// Default: true
    pub ignore_system_version: Option<bool>,
    /// The default mode to use for this agent.
    ///
    /// Note: Not only all agents support modes.
    ///
    /// Default: None
    pub default_mode: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Clone, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct CustomAgentServerSettings {
    #[serde(rename = "command")]
    pub path: PathBuf,
    #[serde(default)]
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
    /// The default mode to use for this agent.
    ///
    /// Note: Not only all agents support modes.
    ///
    /// Default: None
    pub default_mode: Option<String>,
}
