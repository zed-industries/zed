use collections::{HashMap, IndexMap};
use schemars::{JsonSchema, json_schema};
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};
use std::sync::Arc;
use std::{borrow::Cow, path::PathBuf};

use crate::ExtendingVec;

use crate::DockPosition;

/// Where to position the threads sidebar.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum SidebarDockPosition {
    /// Always show the sidebar on the left side.
    #[default]
    Left,
    /// Always show the sidebar on the right side.
    Right,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

/// How thinking blocks should be displayed by default in the agent panel.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum ThinkingBlockDisplay {
    /// Thinking blocks fully expand during streaming, then auto-collapse
    /// when the model finishes thinking. Users can re-expand after collapse.
    #[default]
    Auto,
    /// Thinking blocks auto-expand with a height constraint during streaming,
    /// then remain in their constrained state when complete. Users can click
    /// to fully expand or collapse.
    Preview,
    /// Thinking blocks are always fully expanded by default (no height constraint).
    AlwaysExpanded,
    /// Thinking blocks are always collapsed by default.
    AlwaysCollapsed,
}

/// Threshold at which agent auto-compaction runs. See
/// [`AutoCompactSettingsContent::threshold`] for the accepted formats.
///
/// The canonical textual form is stored verbatim so it can round-trip through
/// the settings UI; it is serialized back as a JSON string for percentages and
/// as a JSON integer for token counts.
#[derive(Clone, Debug, PartialEq, Eq, MergeFrom)]
pub struct AutoCompactThreshold(pub String);

impl From<String> for AutoCompactThreshold {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<AutoCompactThreshold> for String {
    fn from(value: AutoCompactThreshold) -> Self {
        value.0
    }
}

impl AsRef<str> for AutoCompactThreshold {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Serialize for AutoCompactThreshold {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.0.ends_with('%') {
            serializer.serialize_str(&self.0)
        } else if let Ok(tokens) = self.0.parse::<i64>() {
            serializer.serialize_i64(tokens)
        } else {
            serializer.serialize_str(&self.0)
        }
    }
}

impl<'de> Deserialize<'de> for AutoCompactThreshold {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ThresholdVisitor;

        impl serde::de::Visitor<'_> for ThresholdVisitor {
            type Value = AutoCompactThreshold;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter
                    .write_str("a percentage string like \"90%\" or an integer number of tokens")
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                Ok(AutoCompactThreshold(value.to_owned()))
            }

            fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
                Ok(AutoCompactThreshold(value.to_string()))
            }

            fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                Ok(AutoCompactThreshold(value.to_string()))
            }

            fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> {
                Ok(AutoCompactThreshold(value.to_string()))
            }
        }

        deserializer.deserialize_any(ThresholdVisitor)
    }
}

impl JsonSchema for AutoCompactThreshold {
    fn schema_name() -> Cow<'static, str> {
        "AutoCompactThreshold".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "oneOf": [
                {
                    "type": "string",
                    "pattern": "^\\d+(\\.\\d+)?%$"
                },
                {
                    "type": "integer"
                }
            ]
        })
    }
}

#[with_fallible_options]
#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, Default)]
pub struct AutoCompactSettingsContent {
    /// Whether to automatically compact the agent's context when it grows too
    /// large, summarizing earlier messages to free up room in the model's
    /// context window.
    ///
    /// Default: true
    pub enabled: Option<bool>,
    /// The threshold at which auto-compaction runs. This is one of:
    ///
    /// - A percentage string ending in `%`, e.g. `"90%"`, measured against the
    ///   model's context window. `"90%"` compacts once the context is 90% full.
    /// - A positive integer: compaction runs once that many tokens have been
    ///   used. For example, `100000` compacts after 100,000 tokens are used.
    /// - A negative integer: compaction runs once that many tokens remain in
    ///   the context window. For example, `-20000` compacts once there are
    ///   fewer than 20,000 tokens of headroom left in the context window.
    ///
    /// `0` is not a valid threshold.
    ///
    /// Default: "90%"
    pub threshold: Option<AutoCompactThreshold>,
}

#[with_fallible_options]
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
    /// Default: left (Agentic layout), right (Classic layout)
    pub dock: Option<DockPosition>,
    /// Whether the agent panel should use flexible (proportional) sizing.
    ///
    /// Default: true
    pub flexible: Option<bool>,
    /// Where to position the threads sidebar.
    ///
    /// Default: left
    pub sidebar_side: Option<SidebarDockPosition>,
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
    /// Whether to limit the content width in the agent panel. When enabled,
    /// content will be constrained to `max_content_width` and centered when
    /// the panel is wider than that value, for optimal readability.
    ///
    /// Default: true
    pub limit_content_width: Option<bool>,
    /// Maximum content width in pixels for the agent panel. Content will be
    /// centered when the panel is wider than this value.
    ///
    /// Default: 850
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub max_content_width: Option<f32>,
    /// The default model to use when creating new chats and for other features when a specific model is not specified.
    pub default_model: Option<LanguageModelSelection>,
    /// The model to use for subagents spawned via the `spawn_agent` tool. Defaults to the parent agent's model when not specified.
    pub subagent_model: Option<LanguageModelSelection>,
    /// Favorite models to show at the top of the model selector.
    #[serde(default)]
    pub favorite_models: Vec<LanguageModelSelection>,
    /// Model to use for the inline assistant. Defaults to default_model when not specified.
    pub inline_assistant_model: Option<LanguageModelSelection>,
    /// Model to use for the inline assistant when streaming tools are enabled.
    ///
    /// Default: true
    pub inline_assistant_use_streaming_tools: Option<bool>,
    /// Model to use for generating git commit messages. Defaults to default_model when not specified.
    pub commit_message_model: Option<LanguageModelSelection>,
    /// Whether to include project rules files (AGENTS.md, CLAUDE.md, .rules, etc.)
    /// in the prompt when generating git commit messages.
    ///
    /// Default: true
    pub commit_message_include_project_rules: Option<bool>,
    /// Custom instructions to include in the prompt when generating git commit messages.
    /// Applied in addition to any project rules files (such as `.rules` or `AGENTS.md`).
    pub commit_message_instructions: Option<String>,
    /// Model to use for generating thread summaries. Defaults to default_model when not specified.
    pub thread_summary_model: Option<LanguageModelSelection>,
    /// Additional models with which to generate alternatives when performing inline assists.
    pub inline_alternatives: Option<Vec<LanguageModelSelection>>,
    /// The default profile to use in the Agent.
    ///
    /// Default: write
    pub default_profile: Option<Arc<str>>,
    /// The available agent profiles.
    pub profiles: Option<IndexMap<Arc<str>, AgentProfileContent>>,
    /// Where to show a popup notification when the agent is waiting for user input.
    ///
    /// Default: "primary_screen"
    pub notify_when_agent_waiting: Option<NotifyWhenAgentWaiting>,
    /// When to play a sound when the agent has either completed its response, or needs user input.
    ///
    /// Default: never
    pub play_sound_when_agent_done: Option<PlaySoundWhenAgentDone>,
    /// Whether to display agent edits in single-file editors in addition to the review multibuffer pane.
    ///
    /// Default: false
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
    /// Settings for automatic agent context compaction, which summarizes
    /// earlier messages to free up room in the model's context window once the
    /// context grows too large.
    pub auto_compact: Option<AutoCompactSettingsContent>,
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
    /// Command to automatically run when Zed creates a Terminal Thread shell in the agent panel.
    /// The command is sent to the shell as if typed, so it is interpreted by your
    /// configured shell (including on Windows and remote/WSL projects).
    /// An empty string disables this behavior.
    ///
    /// Default: ""
    pub terminal_init_command: Option<String>,
    /// How thinking blocks should be displayed by default in the agent panel.
    ///
    /// Default: automatic
    pub thinking_display: Option<ThinkingBlockDisplay>,
    /// Whether clicking the stop button on a running terminal tool should also cancel the agent's generation.
    /// Note that this only applies to the stop button, not to ctrl+c inside the terminal.
    ///
    /// Default: true
    pub cancel_generation_on_terminal_stop: Option<bool>,
    /// Whether to always use cmd-enter (or ctrl-enter on Linux or Windows) to send messages in the agent panel.
    ///
    /// Default: false
    pub use_modifier_to_send: Option<bool>,
    /// Minimum number of lines of height the agent message editor should have.
    ///
    /// Default: 4
    pub message_editor_min_lines: Option<usize>,
    /// Whether to show turn statistics (elapsed time during generation, final turn duration).
    ///
    /// Default: false
    pub show_turn_stats: Option<bool>,
    /// Whether to show the merge conflict indicator in the status bar
    /// that offers to resolve conflicts using the agent.
    ///
    /// Default: true
    pub show_merge_conflict_indicator: Option<bool>,
    /// Per-tool permission rules for granular control over which tool actions
    /// require confirmation.
    ///
    /// The global `default` applies when no tool-specific rules match.
    /// For external agent servers (e.g. Claude Agent) that define their own
    /// permission modes, "deny" and "confirm" still take precedence — the
    /// external agent's permission system is only used when Zed would allow
    /// the action. Per-tool regex patterns (`always_allow`, `always_deny`,
    /// `always_confirm`) match against the tool's text input (command, path,
    /// URL, etc.).
    pub tool_permissions: Option<ToolPermissionsContent>,

    /// Persistent sandbox permission grants for agent-run terminal commands.
    /// These are populated when choosing "Allow always" from a sandbox
    /// escalation prompt.
    pub sandbox_permissions: Option<SandboxPermissionsContent>,
}

impl AgentSettingsContent {
    pub fn set_dock(&mut self, dock: DockPosition) {
        self.dock = Some(dock);
    }

    pub fn set_sidebar_side(&mut self, position: SidebarDockPosition) {
        self.sidebar_side = Some(position);
    }

    pub fn set_flexible_size(&mut self, flexible: bool) {
        self.flexible = Some(flexible);
    }

    pub fn set_model(&mut self, language_model: LanguageModelSelection) {
        self.default_model = Some(language_model)
    }

    pub fn set_inline_assistant_model(&mut self, provider: String, model: String) {
        self.inline_assistant_model = Some(LanguageModelSelection {
            provider: provider.into(),
            model,
            enable_thinking: false,
            effort: None,
            speed: None,
        });
    }

    pub fn set_profile(&mut self, profile_id: Arc<str>) {
        self.default_profile = Some(profile_id);
    }

    pub fn add_favorite_model(&mut self, model: LanguageModelSelection) {
        // Note: this is intentional to not compare using `PartialEq`here.
        // Full equality would treat entries that differ just in thinking/effort/speed
        // as distinct and silently produce duplicates.
        if !self
            .favorite_models
            .iter()
            .any(|m| m.provider == model.provider && m.model == model.model)
        {
            self.favorite_models.push(model);
        }
    }

    pub fn remove_favorite_model(&mut self, model: &LanguageModelSelection) {
        self.favorite_models
            .retain(|m| !(m.provider == model.provider && m.model == model.model));
    }

    pub fn update_favorite_model<F>(&mut self, provider: &str, model: &str, f: F)
    where
        F: FnOnce(&mut LanguageModelSelection),
    {
        if let Some(entry) = self
            .favorite_models
            .iter_mut()
            .find(|m| m.provider.0 == provider && m.model == model)
        {
            f(entry);
        }
    }

    pub fn set_tool_default_permission(&mut self, tool_id: &str, mode: ToolPermissionMode) {
        let tool_permissions = self.tool_permissions.get_or_insert_default();
        let tool_rules = tool_permissions
            .tools
            .entry(Arc::from(tool_id))
            .or_default();
        tool_rules.default = Some(mode);
    }

    pub fn add_tool_allow_pattern(&mut self, tool_name: &str, pattern: String) {
        let tool_permissions = self.tool_permissions.get_or_insert_default();
        let tool_rules = tool_permissions
            .tools
            .entry(Arc::from(tool_name))
            .or_default();
        let always_allow = tool_rules.always_allow.get_or_insert_default();
        if !always_allow.0.iter().any(|r| r.pattern == pattern) {
            always_allow.0.push(ToolRegexRule {
                pattern,
                case_sensitive: None,
            });
        }
    }

    pub fn add_tool_deny_pattern(&mut self, tool_name: &str, pattern: String) {
        let tool_permissions = self.tool_permissions.get_or_insert_default();
        let tool_rules = tool_permissions
            .tools
            .entry(Arc::from(tool_name))
            .or_default();
        let always_deny = tool_rules.always_deny.get_or_insert_default();
        if !always_deny.0.iter().any(|r| r.pattern == pattern) {
            always_deny.0.push(ToolRegexRule {
                pattern,
                case_sensitive: None,
            });
        }
    }

    pub fn allow_sandbox_all_hosts(&mut self) {
        self.sandbox_permissions
            .get_or_insert_default()
            .allow_all_hosts = Some(true);
    }

    /// The persisted sandbox network host patterns, as written (callers own
    /// parsing/validation).
    pub fn sandbox_network_hosts(&self) -> &[String] {
        self.sandbox_permissions
            .as_ref()
            .and_then(|permissions| permissions.network_hosts.as_ref())
            .map(|hosts| hosts.0.as_slice())
            .unwrap_or_default()
    }

    /// Replace the persisted sandbox network host patterns. Callers compute
    /// the new list (typically the old list plus newly granted hosts, pruned
    /// of entries subsumed by wildcards) rather than appending blindly.
    pub fn set_sandbox_network_hosts(&mut self, hosts: Vec<String>) {
        self.sandbox_permissions
            .get_or_insert_default()
            .network_hosts = Some(ExtendingVec(hosts));
    }

    pub fn allow_sandbox_fs_write_all(&mut self) {
        self.sandbox_permissions
            .get_or_insert_default()
            .allow_fs_write_all = Some(true);
    }

    pub fn allow_sandbox_unsandboxed(&mut self) {
        self.sandbox_permissions
            .get_or_insert_default()
            .allow_unsandboxed = Some(true);
    }

    pub fn add_sandbox_write_path(&mut self, path: PathBuf) {
        let write_paths = &mut self
            .sandbox_permissions
            .get_or_insert_default()
            .write_paths
            .get_or_insert_default()
            .0;

        util::paths::insert_subtree(write_paths, path);
    }
}

#[with_fallible_options]
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

#[with_fallible_options]
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ContextServerPresetContent {
    pub tools: IndexMap<Arc<str>, bool>,
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
pub enum PlaySoundWhenAgentDone {
    #[default]
    Never,
    WhenHidden,
    Always,
}

impl PlaySoundWhenAgentDone {
    pub fn should_play(&self, visible: bool) -> bool {
        match self {
            PlaySoundWhenAgentDone::Never => false,
            PlaySoundWhenAgentDone::WhenHidden => !visible,
            PlaySoundWhenAgentDone::Always => true,
        }
    }
}

#[with_fallible_options]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct LanguageModelSelection {
    pub provider: LanguageModelProviderSetting,
    pub model: String,
    #[serde(default)]
    pub enable_thinking: bool,
    pub effort: Option<String>,
    pub speed: Option<language_model_core::Speed>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct LanguageModelParameters {
    pub provider: Option<LanguageModelProviderSetting>,
    pub model: Option<String>,
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
                        "opencode",
                        "openrouter",
                        "vercel_ai_gateway",
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

#[with_fallible_options]
#[derive(Default, PartialEq, Deserialize, Serialize, Clone, JsonSchema, MergeFrom, Debug)]
#[serde(transparent)]
pub struct AllAgentServersSettings(pub HashMap<String, CustomAgentServerSettings>);

impl std::ops::Deref for AllAgentServersSettings {
    type Target = HashMap<String, CustomAgentServerSettings>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for AllAgentServersSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Deserialize, Serialize, Clone, JsonSchema, MergeFrom, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum AgentConfigOptionValue {
    ValueId(String),
    Boolean(bool),
}

impl AgentConfigOptionValue {
    pub fn as_value_id(&self) -> Option<&str> {
        match self {
            Self::ValueId(value) => Some(value),
            Self::Boolean(_) => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            Self::ValueId(_) => None,
        }
    }
}

impl std::fmt::Display for AgentConfigOptionValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValueId(value) => formatter.write_str(value),
            Self::Boolean(value) => value.fmt(formatter),
        }
    }
}

impl From<String> for AgentConfigOptionValue {
    fn from(value: String) -> Self {
        Self::ValueId(value)
    }
}

impl From<&str> for AgentConfigOptionValue {
    fn from(value: &str) -> Self {
        Self::ValueId(value.to_string())
    }
}

impl From<bool> for AgentConfigOptionValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

#[with_fallible_options]
#[derive(Deserialize, Serialize, Clone, JsonSchema, MergeFrom, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CustomAgentServerSettings {
    Custom {
        #[serde(rename = "command")]
        path: PathBuf,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        args: Vec<String>,
        /// Default: {}
        #[serde(default, skip_serializing_if = "HashMap::is_empty")]
        env: HashMap<String, String>,
        /// The default mode to use for this agent.
        ///
        /// Note: Not only all agents support modes.
        ///
        /// Default: None
        default_mode: Option<String>,
        /// Default values for session config options.
        ///
        /// This is a map from config option ID to the default value for that option.
        ///
        /// Default: {}
        #[serde(default, skip_serializing_if = "HashMap::is_empty")]
        default_config_options: HashMap<String, AgentConfigOptionValue>,
        /// Favorited values for session config options.
        ///
        /// This is a map from config option ID to a list of favorited value IDs.
        ///
        /// Default: {}
        #[serde(default, skip_serializing_if = "HashMap::is_empty")]
        favorite_config_option_values: HashMap<String, Vec<String>>,
    },
    // Used for the ACP extension migration
    #[serde(alias = "extension")]
    Registry {
        /// Additional environment variables to pass to the agent.
        ///
        /// Default: {}
        #[serde(default, skip_serializing_if = "HashMap::is_empty")]
        env: HashMap<String, String>,
        /// The default mode to use for this agent.
        ///
        /// Note: Not only all agents support modes.
        ///
        /// Default: None
        default_mode: Option<String>,
        /// Default values for session config options.
        ///
        /// This is a map from config option ID to the default value for that option.
        ///
        /// Default: {}
        #[serde(default, skip_serializing_if = "HashMap::is_empty")]
        default_config_options: HashMap<String, AgentConfigOptionValue>,
        /// Favorited values for session config options.
        ///
        /// This is a map from config option ID to a list of favorited value IDs.
        ///
        /// Default: {}
        #[serde(default, skip_serializing_if = "HashMap::is_empty")]
        favorite_config_option_values: HashMap<String, Vec<String>>,
    },
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct SandboxPermissionsContent {
    /// Whether sandboxed terminal commands may always reach any host over the
    /// network without prompting.
    /// Default: false
    pub allow_all_hosts: Option<bool>,

    /// Hosts that sandboxed terminal commands may always reach over the
    /// network without prompting. Each entry is an exact hostname
    /// (`github.com`) or a leading-`*.` subdomain wildcard (`*.npmjs.org`).
    /// Default: []
    pub network_hosts: Option<ExtendingVec<String>>,

    /// Whether sandboxed terminal commands may always write anywhere on the
    /// filesystem without prompting.
    /// Default: false
    pub allow_fs_write_all: Option<bool>,

    /// Whether to persistently run agent terminal commands outside the OS
    /// sandbox. This is the model-facing "off switch": when true, the sandboxed
    /// terminal tool is not exposed and the system prompt omits the sandbox
    /// section, so the model uses the plain `terminal` tool. On Windows, WSL
    /// sandbox setup is skipped. Distinct from the model-requested
    /// `unsandboxed: true` escape approved "once" or "for this thread".
    /// Default: false
    pub allow_unsandboxed: Option<bool>,

    /// Directory subtrees that sandboxed terminal commands may always write
    /// to without prompting. Paths written by Zed are absolute.
    /// Default: []
    pub write_paths: Option<ExtendingVec<PathBuf>>,

    /// Whether to warn when a sandbox escalation prompt requests a domain or
    /// write path that contains potentially confusable Unicode characters
    /// (homoglyphs, invisible characters, or bidirectional overrides). When
    /// enabled, such prompts show a warning that must be acknowledged before
    /// the request can be allowed.
    /// Default: true
    pub warn_confusable_unicode: Option<bool>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ToolPermissionsContent {
    /// Global default permission when no tool-specific rules match.
    /// Individual tools can override this with their own default.
    /// Default: confirm
    #[serde(alias = "default_mode")]
    pub default: Option<ToolPermissionMode>,

    /// Per-tool permission rules.
    /// Keys are tool names (e.g. terminal, edit_file, fetch) including MCP
    /// tools (e.g. mcp:server_name:tool_name). Any tool name is accepted;
    /// even tools without meaningful text input can have a `default` set.
    #[serde(default)]
    pub tools: HashMap<Arc<str>, ToolRulesContent>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ToolRulesContent {
    /// Default mode when no regex rules match.
    /// When unset, inherits from the global `tool_permissions.default`.
    #[serde(alias = "default_mode")]
    pub default: Option<ToolPermissionMode>,

    /// Regexes for inputs to auto-approve.
    /// For terminal: matches command. For file tools: matches path. For fetch: matches URL.
    /// For `copy_path` and `move_path`, patterns are matched independently against each
    /// path (source and destination).
    /// Patterns accumulate across settings layers (user, project, profile) and cannot be
    /// removed by a higher-priority layer—only new patterns can be added.
    /// Default: []
    pub always_allow: Option<ExtendingVec<ToolRegexRule>>,

    /// Regexes for inputs to auto-reject.
    /// **SECURITY**: These take precedence over ALL other rules, across ALL settings layers.
    /// For `copy_path` and `move_path`, patterns are matched independently against each
    /// path (source and destination).
    /// Patterns accumulate across settings layers (user, project, profile) and cannot be
    /// removed by a higher-priority layer—only new patterns can be added.
    /// Default: []
    pub always_deny: Option<ExtendingVec<ToolRegexRule>>,

    /// Regexes for inputs that must always prompt.
    /// Takes precedence over always_allow but not always_deny.
    /// For `copy_path` and `move_path`, patterns are matched independently against each
    /// path (source and destination).
    /// Patterns accumulate across settings layers (user, project, profile) and cannot be
    /// removed by a higher-priority layer—only new patterns can be added.
    /// Default: []
    pub always_confirm: Option<ExtendingVec<ToolRegexRule>>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ToolRegexRule {
    /// The regex pattern to match.
    #[serde(default)]
    pub pattern: String,

    /// Whether the regex is case-sensitive.
    /// Default: false (case-insensitive)
    pub case_sensitive: Option<bool>,
}

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom,
)]
#[serde(rename_all = "snake_case")]
pub enum ToolPermissionMode {
    /// Auto-approve without prompting.
    Allow,
    /// Auto-reject with an error.
    Deny,
    /// Always prompt for confirmation (default behavior).
    #[default]
    Confirm,
}

impl std::fmt::Display for ToolPermissionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolPermissionMode::Allow => write!(f, "Allow"),
            ToolPermissionMode::Deny => write!(f, "Deny"),
            ToolPermissionMode::Confirm => write!(f, "Confirm"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_config_option_value_serializes_value_id_as_string() {
        let value = AgentConfigOptionValue::from("manual");

        assert_eq!(
            serde_json::to_value(&value).expect("serialize value id"),
            serde_json::json!("manual")
        );
        assert_eq!(
            serde_json::from_value::<AgentConfigOptionValue>(serde_json::json!("manual"))
                .expect("deserialize value id"),
            AgentConfigOptionValue::ValueId("manual".to_string())
        );
    }

    #[test]
    fn agent_config_option_value_serializes_boolean_as_boolean() {
        let value = AgentConfigOptionValue::Boolean(true);

        assert_eq!(
            serde_json::to_value(&value).expect("serialize boolean"),
            serde_json::json!(true)
        );
        assert_eq!(
            serde_json::from_value::<AgentConfigOptionValue>(serde_json::json!(true))
                .expect("deserialize boolean"),
            AgentConfigOptionValue::Boolean(true)
        );
    }

    #[test]
    fn agent_config_option_value_merge_replaces_existing_value() {
        use crate::merge_from::MergeFrom as _;

        let mut value = AgentConfigOptionValue::ValueId("manual".to_string());
        value.merge_from(&AgentConfigOptionValue::Boolean(true));

        assert_eq!(value, AgentConfigOptionValue::Boolean(true));
    }

    #[test]
    fn test_set_tool_default_permission_creates_structure() {
        let mut settings = AgentSettingsContent::default();
        assert!(settings.tool_permissions.is_none());

        settings.set_tool_default_permission("terminal", ToolPermissionMode::Allow);

        let tool_permissions = settings.tool_permissions.as_ref().unwrap();
        let terminal_rules = tool_permissions.tools.get("terminal").unwrap();
        assert_eq!(terminal_rules.default, Some(ToolPermissionMode::Allow));
    }

    #[test]
    fn test_set_tool_default_permission_updates_existing() {
        let mut settings = AgentSettingsContent::default();

        settings.set_tool_default_permission("terminal", ToolPermissionMode::Confirm);
        settings.set_tool_default_permission("terminal", ToolPermissionMode::Allow);

        let tool_permissions = settings.tool_permissions.as_ref().unwrap();
        let terminal_rules = tool_permissions.tools.get("terminal").unwrap();
        assert_eq!(terminal_rules.default, Some(ToolPermissionMode::Allow));
    }

    #[test]
    fn test_set_tool_default_permission_for_mcp_tool() {
        let mut settings = AgentSettingsContent::default();

        settings.set_tool_default_permission("mcp:github:create_issue", ToolPermissionMode::Allow);

        let tool_permissions = settings.tool_permissions.as_ref().unwrap();
        let mcp_rules = tool_permissions
            .tools
            .get("mcp:github:create_issue")
            .unwrap();
        assert_eq!(mcp_rules.default, Some(ToolPermissionMode::Allow));
    }

    #[test]
    fn test_add_tool_allow_pattern_creates_structure() {
        let mut settings = AgentSettingsContent::default();
        assert!(settings.tool_permissions.is_none());

        settings.add_tool_allow_pattern("terminal", "^cargo\\s".to_string());

        let tool_permissions = settings.tool_permissions.as_ref().unwrap();
        let terminal_rules = tool_permissions.tools.get("terminal").unwrap();
        let always_allow = terminal_rules.always_allow.as_ref().unwrap();
        assert_eq!(always_allow.0.len(), 1);
        assert_eq!(always_allow.0[0].pattern, "^cargo\\s");
    }

    #[test]
    fn test_add_tool_allow_pattern_appends_to_existing() {
        let mut settings = AgentSettingsContent::default();

        settings.add_tool_allow_pattern("terminal", "^cargo\\s".to_string());
        settings.add_tool_allow_pattern("terminal", "^npm\\s".to_string());

        let tool_permissions = settings.tool_permissions.as_ref().unwrap();
        let terminal_rules = tool_permissions.tools.get("terminal").unwrap();
        let always_allow = terminal_rules.always_allow.as_ref().unwrap();
        assert_eq!(always_allow.0.len(), 2);
        assert_eq!(always_allow.0[0].pattern, "^cargo\\s");
        assert_eq!(always_allow.0[1].pattern, "^npm\\s");
    }

    #[test]
    fn test_add_tool_allow_pattern_does_not_duplicate() {
        let mut settings = AgentSettingsContent::default();

        settings.add_tool_allow_pattern("terminal", "^cargo\\s".to_string());
        settings.add_tool_allow_pattern("terminal", "^cargo\\s".to_string());
        settings.add_tool_allow_pattern("terminal", "^cargo\\s".to_string());

        let tool_permissions = settings.tool_permissions.as_ref().unwrap();
        let terminal_rules = tool_permissions.tools.get("terminal").unwrap();
        let always_allow = terminal_rules.always_allow.as_ref().unwrap();
        assert_eq!(
            always_allow.0.len(),
            1,
            "Duplicate patterns should not be added"
        );
    }

    #[test]
    fn test_add_tool_allow_pattern_for_different_tools() {
        let mut settings = AgentSettingsContent::default();

        settings.add_tool_allow_pattern("terminal", "^cargo\\s".to_string());
        settings.add_tool_allow_pattern("fetch", "^https?://github\\.com".to_string());

        let tool_permissions = settings.tool_permissions.as_ref().unwrap();

        let terminal_rules = tool_permissions.tools.get("terminal").unwrap();
        assert_eq!(
            terminal_rules.always_allow.as_ref().unwrap().0[0].pattern,
            "^cargo\\s"
        );

        let fetch_rules = tool_permissions.tools.get("fetch").unwrap();
        assert_eq!(
            fetch_rules.always_allow.as_ref().unwrap().0[0].pattern,
            "^https?://github\\.com"
        );
    }

    #[test]
    fn test_add_tool_deny_pattern_creates_structure() {
        let mut settings = AgentSettingsContent::default();
        assert!(settings.tool_permissions.is_none());

        settings.add_tool_deny_pattern("terminal", "^rm\\s".to_string());

        let tool_permissions = settings.tool_permissions.as_ref().unwrap();
        let terminal_rules = tool_permissions.tools.get("terminal").unwrap();
        let always_deny = terminal_rules.always_deny.as_ref().unwrap();
        assert_eq!(always_deny.0.len(), 1);
        assert_eq!(always_deny.0[0].pattern, "^rm\\s");
    }

    #[test]
    fn test_add_tool_deny_pattern_appends_to_existing() {
        let mut settings = AgentSettingsContent::default();

        settings.add_tool_deny_pattern("terminal", "^rm\\s".to_string());
        settings.add_tool_deny_pattern("terminal", "^sudo\\s".to_string());

        let tool_permissions = settings.tool_permissions.as_ref().unwrap();
        let terminal_rules = tool_permissions.tools.get("terminal").unwrap();
        let always_deny = terminal_rules.always_deny.as_ref().unwrap();
        assert_eq!(always_deny.0.len(), 2);
        assert_eq!(always_deny.0[0].pattern, "^rm\\s");
        assert_eq!(always_deny.0[1].pattern, "^sudo\\s");
    }

    #[test]
    fn test_add_tool_deny_pattern_does_not_duplicate() {
        let mut settings = AgentSettingsContent::default();

        settings.add_tool_deny_pattern("terminal", "^rm\\s".to_string());
        settings.add_tool_deny_pattern("terminal", "^rm\\s".to_string());
        settings.add_tool_deny_pattern("terminal", "^rm\\s".to_string());

        let tool_permissions = settings.tool_permissions.as_ref().unwrap();
        let terminal_rules = tool_permissions.tools.get("terminal").unwrap();
        let always_deny = terminal_rules.always_deny.as_ref().unwrap();
        assert_eq!(
            always_deny.0.len(),
            1,
            "Duplicate patterns should not be added"
        );
    }

    #[test]
    fn test_add_tool_deny_and_allow_patterns_separate() {
        let mut settings = AgentSettingsContent::default();

        settings.add_tool_allow_pattern("terminal", "^cargo\\s".to_string());
        settings.add_tool_deny_pattern("terminal", "^rm\\s".to_string());

        let tool_permissions = settings.tool_permissions.as_ref().unwrap();
        let terminal_rules = tool_permissions.tools.get("terminal").unwrap();

        let always_allow = terminal_rules.always_allow.as_ref().unwrap();
        assert_eq!(always_allow.0.len(), 1);
        assert_eq!(always_allow.0[0].pattern, "^cargo\\s");

        let always_deny = terminal_rules.always_deny.as_ref().unwrap();
        assert_eq!(always_deny.0.len(), 1);
        assert_eq!(always_deny.0[0].pattern, "^rm\\s");
    }

    #[test]
    fn test_allow_sandbox_permissions_create_structure() {
        let mut settings = AgentSettingsContent::default();
        assert!(settings.sandbox_permissions.is_none());

        settings.allow_sandbox_all_hosts();
        assert_eq!(settings.sandbox_network_hosts(), &[] as &[String]);
        settings
            .set_sandbox_network_hosts(vec!["github.com".to_string(), "*.npmjs.org".to_string()]);
        assert_eq!(
            settings.sandbox_network_hosts(),
            &["github.com".to_string(), "*.npmjs.org".to_string()]
        );
        settings.allow_sandbox_fs_write_all();
        settings.allow_sandbox_unsandboxed();
        settings.add_sandbox_write_path(PathBuf::from("/tmp/build"));

        let sandbox_permissions = settings.sandbox_permissions.as_ref().unwrap();
        assert_eq!(sandbox_permissions.allow_all_hosts, Some(true));
        assert_eq!(
            sandbox_permissions
                .network_hosts
                .as_ref()
                .unwrap()
                .0
                .as_slice(),
            &["github.com".to_string(), "*.npmjs.org".to_string()]
        );
        assert_eq!(sandbox_permissions.allow_fs_write_all, Some(true));
        assert_eq!(sandbox_permissions.allow_unsandboxed, Some(true));
        assert_eq!(
            sandbox_permissions
                .write_paths
                .as_ref()
                .unwrap()
                .0
                .as_slice(),
            &[PathBuf::from("/tmp/build")]
        );
    }

    #[test]
    fn test_add_sandbox_write_path_prunes_redundant_paths() {
        let mut settings = AgentSettingsContent::default();

        settings.add_sandbox_write_path(PathBuf::from("/tmp/build/cache"));
        settings.add_sandbox_write_path(PathBuf::from("/tmp/build"));
        settings.add_sandbox_write_path(PathBuf::from("/tmp/build/output"));

        let write_paths = settings
            .sandbox_permissions
            .as_ref()
            .unwrap()
            .write_paths
            .as_ref()
            .unwrap()
            .0
            .as_slice();
        assert_eq!(write_paths, &[PathBuf::from("/tmp/build")]);
    }
}
