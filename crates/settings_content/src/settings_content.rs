//! Data types describing the content of Zed's settings files.
//!
//! The rustdocs on these types are the single source of truth for
//! user-facing settings documentation: field docs become the comments in the
//! generated `assets/settings/default.json`, and enum variant docs become the
//! option lists in the generated settings reference docs.

#![warn(missing_docs)]

mod action;
mod agent;
pub mod default_settings_json;
mod editor;
mod extension;
mod fallible_options;
mod language;
mod language_model;
/// Recursive merging of settings structures.
pub mod merge_from;
mod project;
mod serde_helper;
mod terminal;
mod theme;
mod title_bar;
mod workspace;

pub use action::{ActionName, ActionWithArguments, CommandAliasTarget};
pub use agent::*;
use anyhow::Context;
pub use editor::*;
pub use extension::*;
pub use fallible_options::*;
pub use language::*;
pub use language_model::*;
pub use merge_from::MergeFrom as MergeFromTrait;
pub use project::*;
use serde::de::DeserializeOwned;
pub use serde_helper::{
    serialize_f32_with_two_decimal_places, serialize_optional_f32_with_two_decimal_places,
};
use settings_json::parse_json_with_comments;
pub use terminal::*;
pub use theme::*;
pub use title_bar::*;
pub use workspace::*;

use collections::{HashMap, IndexMap, IndexSet};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

/// A non-negative size in pixels.
///
/// Valid range: 0.0 and up
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    PartialOrd,
    derive_more::FromStr,
    derive_more::Deref,
    derive_more::From,
)]
#[serde(transparent)]
pub struct PixelSetting(
    #[serde(serialize_with = "crate::serialize_f32_with_two_decimal_places")] pub f32,
);

impl std::fmt::Display for PixelSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rounded = (self.0 * 100.0).round() / 100.0;
        write!(f, "{rounded}")
    }
}

/// Defines a settings override struct where each field is
/// `Option<Box<SettingsContent>>`, along with:
/// - `OVERRIDE_KEYS`: a `&[&str]` of the field names (the JSON keys)
/// - `get_by_key(&self, key) -> Option<&SettingsContent>`: accessor by key
///
/// The field list is the single source of truth for the override key strings.
macro_rules! settings_overrides {
    (
        $(#[$attr:meta])*
        pub struct $name:ident { $($field:ident),* $(,)? }
    ) => {
        $(#[$attr])*
        pub struct $name {
            $(
                #[doc = concat!("Settings overrides applied under the `", stringify!($field), "` key.")]
                pub $field: Option<Box<SettingsContent>>,
            )*
        }

        impl $name {
            /// The JSON override keys, derived from the field names on this struct.
            pub const OVERRIDE_KEYS: &[&str] = &[$(stringify!($field)),*];

            /// Look up an override by its JSON key name.
            pub fn get_by_key(&self, key: &str) -> Option<&SettingsContent> {
                match key {
                    $(stringify!($field) => self.$field.as_deref(),)*
                    _ => None,
                }
            }
        }
    }
}
use std::collections::{BTreeMap, BTreeSet};
use std::hash::Hash;
use std::sync::Arc;
pub use util::serde::default_true;

/// The result of parsing a settings file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseStatus {
    /// Settings were parsed successfully
    Success,
    /// Settings file was not changed, so no parsing was performed
    Unchanged,
    /// Settings failed to parse
    Failed {
        /// The parse error message.
        error: String,
    },
}

/// Determines when the mouse cursor should be hidden in response to keyboard
/// input.
///
/// Default: on_typing_and_action
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum HideMouseMode {
    /// Never hide the mouse cursor
    Never,
    /// Hide only when typing
    OnTyping,
    /// Hide on typing and on key bindings that resolve to an action
    #[default]
    OnTypingAndAction,
}

/// Determines whether to reduce non-essential motion in the UI, such as
/// loading spinners and pulsating labels, by rendering them in a static state.
///
/// Default: off
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum ReduceMotionMode {
    /// Always reduce motion
    On,
    /// Never reduce motion
    #[default]
    Off,
}

/// The content of Zed's settings, with every setting optional.
#[with_fallible_options]
#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct SettingsContent {
    /// Project-related settings, flattened into the root object.
    #[serde(flatten)]
    pub project: ProjectSettingsContent,

    /// Theme and appearance settings, flattened into the root object.
    #[serde(flatten)]
    pub theme: Box<ThemeSettingsContent>,

    /// Extension settings, flattened into the root object.
    #[serde(flatten)]
    pub extension: ExtensionSettingsContent,

    /// Workspace settings, flattened into the root object.
    #[serde(flatten)]
    pub workspace: WorkspaceSettingsContent,

    /// Editor settings, flattened into the root object.
    #[serde(flatten)]
    pub editor: EditorSettingsContent,

    /// Remote development settings, flattened into the root object.
    #[serde(flatten)]
    pub remote: RemoteSettingsContent,

    /// Settings related to the file finder.
    pub file_finder: Option<FileFinderSettingsContent>,

    /// Setting to customize the behavior of the git panel.
    pub git_panel: Option<GitPanelSettingsContent>,

    /// Settings related to the editor's tabs.
    pub tabs: Option<ItemSettingsContent>,
    /// Settings related to the editor's tab bar.
    pub tab_bar: Option<TabBarSettingsContent>,
    /// Status bar-related settings.
    pub status_bar: Option<StatusBarSettingsContent>,

    /// Settings related to preview tabs.
    pub preview_tabs: Option<PreviewTabsSettingsContent>,

    /// Settings related to the agent panel.
    pub agent: Option<AgentSettingsContent>,
    /// Configures agent servers available in the agent panel.
    pub agent_servers: Option<AllAgentServersSettings>,

    /// Configuration of audio in Zed.
    pub audio: Option<AudioSettingsContent>,

    /// Whether or not to automatically check for updates.
    ///
    /// Default: true
    pub auto_update: Option<bool>,

    /// This base keymap settings adjusts the default keybindings in Zed to be similar
    /// to other common code editors. By default, Zed's keymap closely follows VSCode's
    /// keymap, with minor adjustments, this corresponds to the "VSCode" setting.
    ///
    /// Default: VSCode
    pub base_keymap: Option<BaseKeymapContent>,

    /// Configuration for the collab panel visual settings.
    pub collaboration_panel: Option<PanelSettingsContent>,

    /// Configuration for debugger panel and settings.
    pub debugger: Option<DebuggerSettingsContent>,

    /// Configuration for Diagnostics-related features.
    pub diagnostics: Option<DiagnosticsSettingsContent>,

    /// Configuration for Git-related features
    pub git: Option<GitSettings>,

    /// Common language server settings.
    pub global_lsp_settings: Option<GlobalLspSettingsContent>,

    /// The settings for the image viewer.
    pub image_viewer: Option<ImageViewerSettingsContent>,

    /// The settings for the markdown preview.
    pub markdown_preview: Option<MarkdownPreviewSettingsContent>,

    /// REPL settings.
    pub repl: Option<ReplSettingsContent>,

    /// Whether or not to enable Helix mode.
    ///
    /// Default: false
    pub helix_mode: Option<bool>,

    /// Determines when the mouse cursor should be hidden in response to
    /// keyboard input. Applies globally across all input surfaces (editors,
    /// terminals, palettes, etc.).
    ///
    /// Default: on_typing_and_action
    pub hide_mouse: Option<HideMouseMode>,

    /// Settings specific to journaling.
    pub journal: Option<JournalSettingsContent>,

    /// A map of log scopes to the desired log level.
    /// Useful for filtering out noisy logs or enabling more verbose logging.
    ///
    /// Example: {"log": {"client": "warn"}}
    pub log: Option<HashMap<String, String>>,

    /// Whether to show full labels in line indicator or short ones
    ///
    /// Values:
    ///   - `short`: "2 s, 15 l, 32 c"
    ///   - `long`: "2 selections, 15 lines, 32 characters"
    ///
    /// Default: long
    pub line_indicator_format: Option<LineIndicatorFormat>,

    /// Different settings for specific language models.
    pub language_models: Option<AllLanguageModelSettingsContent>,

    /// Customize outline Panel.
    pub outline_panel: Option<OutlinePanelSettingsContent>,

    /// Customize project panel.
    pub project_panel: Option<ProjectPanelSettingsContent>,

    /// Configuration for Node-related features
    pub node: Option<NodeBinarySettings>,

    /// Set a proxy to use. The proxy protocol is specified by the URI scheme.
    ///
    /// Supported URI scheme: `http`, `https`, `socks4`, `socks4a`, `socks5`,
    /// `socks5h`. `http` will be used when no scheme is specified.
    ///
    /// By default no proxy will be used, or Zed will try get proxy settings from
    /// environment variables. If certain hosts should not be proxied,
    /// set the `no_proxy` environment variable and provide a comma-separated list.
    ///
    /// Examples:
    ///   - "proxy": "socks5h://localhost:10808"
    ///   - "proxy": "http://127.0.0.1:10809"
    pub proxy: Option<String>,

    /// Whether to reduce non-essential motion in the UI, such as loading
    /// spinners and pulsating labels, by rendering them in a static state.
    ///
    /// Default: off
    pub reduce_motion: Option<ReduceMotionMode>,

    /// The URL of the Zed server to connect to.
    pub server_url: Option<String>,

    /// The URL used as the key for credential storage.
    ///
    /// When set, credentials are stored under this URL instead of `server_url`.
    /// This allows running multiple Zed instances side by side without them
    /// overwriting each other's keychain entries.
    pub credentials_url: Option<String>,

    /// Configuration for session-related features
    pub session: Option<SessionSettingsContent>,
    /// Control what info is collected by Zed.
    pub telemetry: Option<TelemetrySettingsContent>,

    /// Configuration of the terminal in Zed.
    pub terminal: Option<TerminalSettingsContent>,

    /// Titlebar related settings
    pub title_bar: Option<TitleBarSettingsContent>,

    /// Whether or not to enable Vim mode.
    ///
    /// Default: false
    pub vim_mode: Option<bool>,

    /// Settings related to calls in Zed
    pub calls: Option<CallSettingsContent>,

    /// Settings for the which-key popup.
    pub which_key: Option<WhichKeySettingsContent>,

    /// Settings related to Vim mode in Zed.
    pub vim: Option<VimSettingsContent>,

    /// Number of lines to search for modelines at the beginning and end of files.
    /// Modelines contain editor directives (e.g., vim/emacs settings) that configure
    /// the editor behavior for specific files.
    ///
    /// Default: 5
    pub modeline_lines: Option<usize>,

    /// Local overrides for feature flags, keyed by flag name.
    pub feature_flags: Option<FeatureFlagsMap>,

    /// Settings for developer-oriented instrumentation tools (profilers,
    /// tracers, etc.) that can be toggled at runtime.
    pub instrumentation: Option<InstrumentationSettingsContent>,
}

/// Configuration for developer-oriented instrumentation tools that collect
/// diagnostic data about a running Zed instance.
#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct InstrumentationSettingsContent {
    /// Configuration for the performance profiler, accessed via the
    /// `zed: open performance profiler` action.
    pub performance_profiler: Option<PerformanceProfilerSettingsContent>,
}

/// Configuration for the performance profiler which collects timing data
/// for foreground and background executor tasks.
#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct PerformanceProfilerSettingsContent {
    /// Whether to collect timing data for foreground and background executor
    /// tasks. Enabling this may lead to increased memory usage, hence it's
    /// disabled by default for regular builds.
    ///
    /// Default: false
    pub enabled: Option<bool>,
}

/// Local overrides for feature flags, keyed by flag name.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, MergeFrom)]
#[serde(transparent)]
pub struct FeatureFlagsMap(pub HashMap<String, String>);

// A manual `JsonSchema` impl keeps this type's schema registered under a
// unique name. The derived impl on a `#[serde(transparent)]` newtype around
// `HashMap<String, String>` would inline to the map's own schema name (`Map_of_string`),
// which is shared with every other `HashMap<String, String>` setting field in
// `SettingsContent`. A named placeholder lets `json_schema_store` find and
// replace just this field's schema at runtime without clobbering the others.
impl JsonSchema for FeatureFlagsMap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FeatureFlagsMap".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "object",
            "additionalProperties": { "type": "string" }
        })
    }
}

impl std::ops::Deref for FeatureFlagsMap {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for FeatureFlagsMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SettingsContent {
    /// Returns a mutable reference to the per-language settings map.
    pub fn languages_mut(&mut self) -> &mut HashMap<String, LanguageSettingsContent> {
        &mut self.project.all_languages.languages.0
    }
}

// These impls are there to optimize builds by avoiding monomorphization downstream. Yes, they're repetitive, but using default impls
// break the optimization, for whatever reason.
/// Parsing entry points for types that can be the root of a settings file.
pub trait RootUserSettings: Sized + DeserializeOwned {
    /// Parses settings JSON, returning the parsed value (if any) and the parse status.
    fn parse_json(json: &str) -> (Option<Self>, ParseStatus);
    /// Parses settings JSON that may contain comments and trailing commas.
    fn parse_json_with_comments(json: &str) -> anyhow::Result<Self>;
}

impl RootUserSettings for SettingsContent {
    fn parse_json(json: &str) -> (Option<Self>, ParseStatus) {
        fallible_options::parse_json(json)
    }
    fn parse_json_with_comments(json: &str) -> anyhow::Result<Self> {
        parse_json_with_comments(json)
    }
}
// Explicit opt-in instead of blanket impl to avoid monomorphizing downstream. Just a hunch though.
impl RootUserSettings for Option<SettingsContent> {
    fn parse_json(json: &str) -> (Option<Self>, ParseStatus) {
        fallible_options::parse_json(json)
    }
    fn parse_json_with_comments(json: &str) -> anyhow::Result<Self> {
        parse_json_with_comments(json)
    }
}
impl RootUserSettings for UserSettingsContent {
    fn parse_json(json: &str) -> (Option<Self>, ParseStatus) {
        fallible_options::parse_json(json)
    }
    fn parse_json_with_comments(json: &str) -> anyhow::Result<Self> {
        parse_json_with_comments(json)
    }
}

settings_overrides! {
    /// Per-release-channel settings overrides, applied when running the matching Zed release channel.
    #[with_fallible_options]
    #[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
    pub struct ReleaseChannelOverrides { dev, nightly, preview, stable }
}

settings_overrides! {
    /// Per-platform settings overrides, applied when running on the matching operating system.
    #[with_fallible_options]
    #[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
    pub struct PlatformOverrides { macos, linux, windows }
}

/// Determines what settings a profile starts from before applying its overrides.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom,
)]
#[serde(rename_all = "snake_case")]
pub enum ProfileBase {
    /// Apply profile settings on top of the user's current settings.
    #[default]
    User,
    /// Apply profile settings on top of Zed's default settings, ignoring user customizations.
    Default,
}

/// A named settings profile that can temporarily override settings.
#[with_fallible_options]
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct SettingsProfile {
    /// What base settings to start from before applying this profile's overrides.
    ///
    /// - `user`: Apply on top of user's settings (default)
    /// - `default`: Apply on top of Zed's default settings, ignoring user customizations
    #[serde(default)]
    pub base: ProfileBase,

    /// The settings overrides for this profile.
    #[serde(default)]
    pub settings: Box<SettingsContent>,
}

/// The content of the user's settings file.
#[with_fallible_options]
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct UserSettingsContent {
    /// The base settings, flattened into the root object.
    #[serde(flatten)]
    pub content: Box<SettingsContent>,

    /// Per-release-channel settings overrides, flattened into the root object.
    #[serde(flatten)]
    pub release_channel_overrides: ReleaseChannelOverrides,

    /// Per-platform settings overrides, flattened into the root object.
    #[serde(flatten)]
    pub platform_overrides: PlatformOverrides,

    /// Named settings profiles that can temporarily override settings.
    #[serde(default)]
    pub profiles: IndexMap<String, SettingsProfile>,
}

/// Settings contributed by installed extensions.
pub struct ExtensionsSettingsContent {
    /// Default language settings provided by installed extensions.
    pub all_languages: AllLanguageSettingsContent,
}

/// Base key bindings scheme. Base keymaps can be overridden with user keymaps.
///
/// Default: Zed
#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    Default,
    strum::VariantArray,
)]
pub enum BaseKeymapContent {
    /// Zed's default keymap.
    #[default]
    Zed,
    /// Keymap similar to VS Code.
    VSCode,
    /// Keymap similar to JetBrains IDEs.
    JetBrains,
    /// Keymap similar to Sublime Text.
    SublimeText,
    /// Keymap similar to Atom.
    Atom,
    /// Keymap similar to TextMate.
    TextMate,
    /// Keymap similar to Emacs.
    Emacs,
    /// Keymap similar to Cursor.
    Cursor,
    /// Disables the base keymap.
    None,
}

impl strum::VariantNames for BaseKeymapContent {
    const VARIANTS: &'static [&'static str] = &[
        "Zed",
        "VSCode",
        "JetBrains",
        "Sublime Text",
        "Atom",
        "TextMate",
        "Emacs",
        "Cursor",
        "None",
    ];
}

/// Configuration of audio in Zed.
#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct AudioSettingsContent {
    /// Select specific output audio device.
    #[serde(rename = "experimental.output_audio_device")]
    pub output_audio_device: Option<AudioOutputDeviceName>,
    /// Select specific input audio device.
    #[serde(rename = "experimental.input_audio_device")]
    pub input_audio_device: Option<AudioInputDeviceName>,
}

/// The name of the output audio device to use.
#[derive(Clone, Default, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq)]
#[serde(transparent)]
pub struct AudioOutputDeviceName(pub Option<String>);

impl AsRef<Option<String>> for AudioInputDeviceName {
    fn as_ref(&self) -> &Option<String> {
        &self.0
    }
}

impl From<Option<String>> for AudioInputDeviceName {
    fn from(value: Option<String>) -> Self {
        Self(value)
    }
}

/// The name of the input audio device to use.
#[derive(Clone, Default, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq)]
#[serde(transparent)]
pub struct AudioInputDeviceName(pub Option<String>);

impl AsRef<Option<String>> for AudioOutputDeviceName {
    fn as_ref(&self) -> &Option<String> {
        &self.0
    }
}

impl From<Option<String>> for AudioOutputDeviceName {
    fn from(value: Option<String>) -> Self {
        Self(value)
    }
}

/// Control what info is collected by Zed.
#[with_fallible_options]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Debug, MergeFrom)]
pub struct TelemetrySettingsContent {
    /// Send debug info like crash reports.
    ///
    /// Default: true
    pub diagnostics: Option<bool>,
    /// Send anonymized usage data like what languages you're using Zed with.
    ///
    /// Default: true
    pub metrics: Option<bool>,
    /// Allow sending requests to Anthropic models that cannot be offered with
    /// Zero Data Retention.
    ///
    /// Default: false
    pub anthropic_retention: Option<bool>,
}

impl Default for TelemetrySettingsContent {
    fn default() -> Self {
        Self {
            diagnostics: Some(true),
            metrics: Some(true),
            anthropic_retention: Some(false),
        }
    }
}

/// Configuration for debugger panel and settings.
#[with_fallible_options]
#[derive(Default, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Clone, MergeFrom)]
pub struct DebuggerSettingsContent {
    /// Determines the stepping granularity.
    ///
    /// Default: line
    pub stepping_granularity: Option<SteppingGranularity>,
    /// Whether the breakpoints should be reused across Zed sessions.
    ///
    /// Default: true
    pub save_breakpoints: Option<bool>,
    /// Whether to show the debug button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Time in milliseconds until timeout error when connecting to a TCP debug adapter
    ///
    /// Default: 2000ms
    pub timeout: Option<u64>,
    /// Whether to log messages between active debug adapters and Zed
    ///
    /// Default: true
    pub log_dap_communications: Option<bool>,
    /// Whether to format dap messages in when adding them to debug adapter logger
    ///
    /// Default: true
    pub format_dap_log_messages: Option<bool>,
    /// The dock position of the debug panel
    ///
    /// Default: Bottom
    pub dock: Option<DockPosition>,
}

/// The granularity of one 'step' in the stepping requests `next`, `stepIn`, `stepOut`, and `stepBack`.
#[derive(
    PartialEq,
    Eq,
    Debug,
    Hash,
    Clone,
    Copy,
    Deserialize,
    Serialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum SteppingGranularity {
    /// The step should allow the program to run until the current statement has finished executing.
    /// The meaning of a statement is determined by the adapter and it may be considered equivalent to a line.
    /// For example 'for(int i = 0; i < 10; i++)' could be considered to have 3 statements 'int i = 0', 'i < 10', and 'i++'.
    Statement,
    /// The step should allow the program to run until the current source line has executed.
    Line,
    /// The step should allow one instruction to execute (e.g. one x86 instruction).
    Instruction,
}

/// The position at which a panel is docked.
#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum DockPosition {
    /// Dock the panel on the left.
    Left,
    /// Dock the panel at the bottom.
    Bottom,
    /// Dock the panel on the right.
    Right,
}

/// Configuration of voice calls in Zed.
#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct CallSettingsContent {
    /// Whether the microphone should be muted when joining a channel or a call.
    ///
    /// Default: false
    pub mute_on_join: Option<bool>,

    /// Whether your current project should be shared when joining an empty channel.
    ///
    /// Default: false
    pub share_on_join: Option<bool>,
}

/// Setting to customize the behavior of the git panel.
#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct GitPanelSettingsContent {
    /// Whether to show the panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Where to dock the panel.
    ///
    /// Default: right (Agentic layout), left (Classic layout)
    pub dock: Option<DockPosition>,
    /// Default width of the panel in pixels.
    ///
    /// Default: 360
    pub default_width: Option<PixelSetting>,
    /// How entry statuses are displayed.
    ///
    /// Default: icon
    pub status_style: Option<StatusStyle>,

    /// Whether to show file icons in the git panel.
    ///
    /// Default: false
    pub file_icons: Option<bool>,

    /// Whether to show folder icons or chevrons for directories in the git panel.
    ///
    /// Default: true
    pub folder_icons: Option<bool>,

    /// How and when the scrollbar should be displayed.
    ///
    /// Default: inherits editor scrollbar settings
    pub scrollbar: Option<ScrollbarSettings>,

    /// What the default branch name should be when
    /// `init.defaultBranch` is not set in git
    ///
    /// Default: main
    pub fallback_branch_name: Option<String>,

    /// How to sort entries in the git panel.
    ///
    /// Default: path
    pub sort_by: Option<GitPanelSortBy>,

    /// How to group entries in the git panel.
    ///
    /// Default: status
    pub group_by: Option<GitPanelGroupBy>,

    /// Whether to collapse untracked files in the diff panel.
    ///
    /// Default: false
    pub collapse_untracked_diff: Option<bool>,

    /// Whether to show entries with tree or flat view in the panel
    ///
    /// Default: false
    pub tree_view: Option<bool>,

    /// Whether to show the addition/deletion change count next to each file in the Git panel.
    ///
    /// Default: true
    pub diff_stats: Option<bool>,

    /// Whether to show a badge on the git panel icon with the count of uncommitted changes.
    ///
    /// Default: false
    pub show_count_badge: Option<bool>,

    /// Whether the git panel should open on startup.
    ///
    /// Default: false
    pub starts_open: Option<bool>,

    /// Maximum length of the commit message title before a warning is shown.
    /// Set to 0 to disable.
    ///
    /// Default: 0
    pub commit_title_max_length: Option<usize>,

    /// Default action when clicking a changed file in the Git panel.
    ///
    /// Default: project_diff
    pub entry_primary_click_action: Option<GitPanelClickBehavior>,
}

/// The action performed when clicking a changed file in the Git panel.
#[derive(
    Default,
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum GitPanelClickBehavior {
    /// Open the project diff, showing all changed files.
    #[default]
    ProjectDiff,
    /// Open a single-file diff view.
    FileDiff,
    /// Open the file in the editor without a diff view.
    ViewFile,
}

/// How to sort entries in the git panel.
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum GitPanelSortBy {
    /// Sort entries by file path.
    #[default]
    Path,
    /// Sort entries by file name.
    Name,
}

/// How to group entries in the git panel.
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum GitPanelGroupBy {
    /// Do not group entries.
    None,
    /// Group entries by git status.
    #[default]
    Status,
    /// Group entries by whether they are staged.
    Staging,
}

/// How entry statuses are displayed in the git panel.
#[derive(
    Default,
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum StatusStyle {
    /// Show the git status as an icon.
    #[default]
    Icon,
    /// Show the git status by coloring the entry label.
    LabelColor,
}

/// Scrollbar-related settings for the git panel.
#[with_fallible_options]
#[derive(
    Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq,
)]
pub struct ScrollbarSettings {
    /// When to show the scrollbar in the git panel.
    pub show: Option<ShowScrollbar>,
}

/// Visual settings shared by dockable panels, such as the collaboration panel.
#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct PanelSettingsContent {
    /// Whether to show the panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Where to dock the panel.
    ///
    /// Default: right (Agentic layout), left (Classic layout)
    pub dock: Option<DockPosition>,
    /// Default width of the panel in pixels.
    ///
    /// Default: 240
    pub default_width: Option<PixelSetting>,
}

/// Settings related to the file finder.
#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct FileFinderSettingsContent {
    /// Whether to show file icons in the file finder.
    ///
    /// Default: true
    pub file_icons: Option<bool>,
    /// Determines how much space the file finder can take up in relation to the available window width.
    ///
    /// Default: small
    pub modal_max_width: Option<FileFinderWidthContent>,
    /// Determines whether the file finder should skip focus for the active file in search results.
    ///
    /// Default: true
    pub skip_focus_for_active_in_search: Option<bool>,
    /// Whether to use gitignored files when searching.
    /// Only the file Zed had indexed will be used, not necessary all the gitignored files.
    ///
    /// Default: Smart
    pub include_ignored: Option<IncludeIgnoredContent>,
    /// Whether to include text channels in file finder results.
    ///
    /// Default: false
    pub include_channels: Option<bool>,
}

/// Whether to include gitignored files when searching in the file finder.
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum IncludeIgnoredContent {
    /// Use all gitignored files
    All,
    /// Use only the files Zed had indexed
    Indexed,
    /// Be smart and search for ignored when called from a gitignored worktree
    #[default]
    Smart,
}

/// Max-width of the file finder modal in relation to the available window width.
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
pub enum FileFinderWidthContent {
    /// Small max-width.
    #[default]
    Small,
    /// Medium max-width.
    Medium,
    /// Large max-width.
    Large,
    /// Extra-large max-width.
    XLarge,
    /// Take up the full window width.
    Full,
}

/// Settings related to Vim mode in Zed.
#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug, JsonSchema, MergeFrom)]
pub struct VimSettingsContent {
    /// The default mode to start in.
    ///
    /// Default: normal
    pub default_mode: Option<ModeContent>,
    /// If `true`, line numbers are relative in normal mode and absolute in
    /// insert mode, giving you the best of both options.
    ///
    /// Default: false
    pub toggle_relative_line_numbers: Option<bool>,
    /// Determines how the system clipboard is used.
    ///
    /// Default: always
    pub use_system_clipboard: Option<UseSystemClipboard>,
    /// If `true`, `f` and `t` motions are case-insensitive when the target
    /// letter is lowercase.
    ///
    /// Default: false
    pub use_smartcase_find: Option<bool>,
    /// If `true`, then vim search will use regex mode.
    ///
    /// Default: true
    pub use_regex_search: Option<bool>,
    /// When enabled, the `:substitute` command replaces all matches in a line
    /// by default. The 'g' flag then toggles this behavior.,
    pub gdefault: Option<bool>,
    /// An object that allows you to add custom digraphs.
    ///
    /// Default: {}
    pub custom_digraphs: Option<HashMap<String, Arc<str>>>,
    /// The duration of the highlight animation (in ms). Set to `0` to disable.
    ///
    /// Default: 200
    pub highlight_on_yank_duration: Option<u64>,
    /// Cursor shape for each mode.
    pub cursor_shape: Option<CursorShapeSettings>,
    /// When enabled, edit predictions are shown in Vim normal mode.
    /// By default, edit predictions are only shown in insert and replace modes.
    pub show_edit_predictions_in_normal_mode: Option<bool>,
}

/// The Vim mode to start in.
#[derive(
    Copy,
    Clone,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Debug,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum ModeContent {
    /// Normal mode.
    #[default]
    Normal,
    /// Insert mode.
    Insert,
}

/// Controls when to use system clipboard.
#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum UseSystemClipboard {
    /// Don't use system clipboard.
    Never,
    /// Use system clipboard.
    Always,
    /// Use system clipboard for yank operations.
    OnYank,
}

/// Cursor shape configuration for insert mode in Vim.
#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum VimInsertModeCursorShape {
    /// Inherit cursor shape from the editor's base cursor_shape setting.
    Inherit,
    /// Vertical bar cursor.
    Bar,
    /// Block cursor that surrounds the character.
    Block,
    /// Underline cursor.
    Underline,
    /// Hollow box cursor.
    Hollow,
}

/// The settings for cursor shape.
#[with_fallible_options]
#[derive(
    Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom,
)]
pub struct CursorShapeSettings {
    /// Cursor shape for the normal mode.
    ///
    /// Default: block
    pub normal: Option<CursorShape>,
    /// Cursor shape for the replace mode.
    ///
    /// Default: underline
    pub replace: Option<CursorShape>,
    /// Cursor shape for the visual mode.
    ///
    /// Default: block
    pub visual: Option<CursorShape>,
    /// Cursor shape for the insert mode.
    ///
    /// The default value follows the primary cursor_shape.
    pub insert: Option<VimInsertModeCursorShape>,
}

/// Settings specific to journaling
#[with_fallible_options]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct JournalSettingsContent {
    /// The path of the directory where journal entries are stored.
    ///
    /// Default: `~`
    pub path: Option<String>,
    /// What format to display the hours in.
    ///
    /// Default: hour12
    pub hour_format: Option<HourFormat>,
}

/// The format to use for displaying hours in the journal.
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HourFormat {
    /// 12-hour format.
    #[default]
    Hour12,
    /// 24-hour format.
    Hour24,
}

/// Customize outline Panel.
#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct OutlinePanelSettingsContent {
    /// Whether to show the outline panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Customize default width (in pixels) taken by outline panel
    ///
    /// Default: 240
    pub default_width: Option<PixelSetting>,
    /// The position of outline panel
    ///
    /// Default: right (Agentic layout), left (Classic layout)
    pub dock: Option<DockSide>,
    /// Whether to show file icons in the outline panel.
    ///
    /// Default: true
    pub file_icons: Option<bool>,
    /// Whether to show folder icons or chevrons for directories in the outline panel.
    ///
    /// Default: true
    pub folder_icons: Option<bool>,
    /// Whether to show the git status in the outline panel.
    ///
    /// Default: true
    pub git_status: Option<bool>,
    /// Amount of indentation (in pixels) for nested items.
    ///
    /// Default: 20
    pub indent_size: Option<PixelSetting>,
    /// Whether to reveal it in the outline panel automatically,
    /// when a corresponding project entry becomes active.
    /// Gitignored entries are never auto revealed.
    ///
    /// Default: true
    pub auto_reveal_entries: Option<bool>,
    /// Whether to fold directories automatically
    /// when directory has only one directory inside.
    ///
    /// Default: true
    pub auto_fold_dirs: Option<bool>,
    /// Settings related to indent guides in the outline panel.
    pub indent_guides: Option<IndentGuidesSettingsContent>,
    /// Scrollbar-related settings
    pub scrollbar: Option<ScrollbarSettingsContent>,
    /// Default depth to expand outline items in the current file.
    /// The default depth to which outline entries are expanded on reveal.
    /// - Set to 0 to collapse all items that have children
    /// - Set to 1 or higher to collapse items at that depth or deeper
    ///
    /// Default: 100
    pub expand_outlines_with_depth: Option<usize>,
}

/// The side of the workspace a panel is docked to.
#[derive(
    Clone,
    Copy,
    Debug,
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
pub enum DockSide {
    /// Dock the panel on the left.
    Left,
    /// Dock the panel on the right.
    Right,
}

/// When to show indent guides in the outline panel.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Deserialize,
    Serialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum ShowIndentGuides {
    /// Always show indent guides.
    Always,
    /// Never show indent guides.
    Never,
}

/// Settings related to indent guides in the outline panel.
#[with_fallible_options]
#[derive(
    Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq, Default,
)]
pub struct IndentGuidesSettingsContent {
    /// When to show the scrollbar in the outline panel.
    pub show: Option<ShowIndentGuides>,
}

/// Whether to show full labels in line indicator or short ones.
#[derive(Clone, Copy, Default, PartialEq, Debug, JsonSchema, MergeFrom, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LineIndicatorFormat {
    /// A short format, e.g. "2 s, 15 l, 32 c".
    Short,
    /// A long format, e.g. "2 selections, 15 lines, 32 characters".
    #[default]
    Long,
}

/// The settings for the markdown preview.
#[with_fallible_options]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, Default, PartialEq)]
pub struct MarkdownPreviewSettingsContent {
    /// Whether to limit the width of the rendered markdown content. When
    /// enabled, content is constrained to `max_width` and centered
    /// horizontally within the preview pane, for optimal readability.
    ///
    /// Default: true
    pub limit_content_width: Option<bool>,
    /// The maximum width, in pixels, of the rendered markdown content when
    /// `limit_content_width` is enabled.
    ///
    /// Default: 800
    pub max_width: Option<PixelSetting>,
}

/// The settings for the image viewer.
#[with_fallible_options]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, Default, PartialEq)]
pub struct ImageViewerSettingsContent {
    /// The unit to use for displaying image file sizes.
    ///
    /// Default: "binary"
    pub unit: Option<ImageFileSizeUnit>,
}

/// The unit for image file sizes.
#[with_fallible_options]
#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    Default,
    PartialEq,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum ImageFileSizeUnit {
    /// Displays file size in binary units (e.g., KiB, MiB).
    #[default]
    Binary,
    /// Displays file size in decimal units (e.g., KB, MB).
    Decimal,
}

/// Settings for connecting to remote servers.
#[with_fallible_options]
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct RemoteSettingsContent {
    /// ssh_connections is an array of ssh connections.
    /// You can configure these from `project: Open Remote` in the command palette.
    /// Zed's ssh support will pull configuration from your ~/.ssh too.
    pub ssh_connections: Option<Vec<SshConnection>>,
    /// A list of WSL distributions to connect to.
    pub wsl_connections: Option<Vec<WslConnection>>,
    /// A list of dev container connections.
    pub dev_container_connections: Option<Vec<DevContainerConnection>>,
    /// Whether to read ~/.ssh/config for ssh connection sources.
    ///
    /// Default: true
    pub read_ssh_config: Option<bool>,
    /// Whether to use Podman instead of Docker for dev containers.
    ///
    /// Default: false
    pub use_podman: Option<bool>,
    /// Whether to build dev container images with BuildKit.
    ///
    /// When unset, Zed auto-detects BuildKit by probing for the `buildx` CLI
    /// plugin. Set to `false` to force the classic Docker builder, which is
    /// required for Docker-compatible engines that lack an integrated BuildKit
    /// (e.g. Apple Container via a Docker-API bridge), where BuildKit builds
    /// cannot resolve locally-built images.
    ///
    /// Default: null (auto-detect)
    pub dev_container_use_buildkit: Option<bool>,
}

/// A connection to a dev container.
#[with_fallible_options]
#[derive(
    Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom, Hash,
)]
pub struct DevContainerConnection {
    /// The name of the dev container.
    pub name: String,
    /// The user to connect as inside the container.
    pub remote_user: String,
    /// The identifier of the container.
    pub container_id: String,
    /// Whether this connection uses Podman instead of Docker.
    pub use_podman: bool,
    /// The IDs of Zed extensions to install in the dev container.
    pub extension_ids: Vec<String>,
    /// Environment variables to set in the dev container.
    pub remote_env: BTreeMap<String, String>,
}

/// A remote server accessed over SSH.
#[with_fallible_options]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct SshConnection {
    /// The host name or address to connect to.
    pub host: String,
    /// The user name to connect as.
    pub username: Option<String>,
    /// The port to connect to.
    pub port: Option<u16>,
    /// Additional arguments to pass to the ssh command.
    #[serde(default)]
    pub args: Vec<String>,
    /// The projects that have been opened on this server.
    #[serde(default)]
    pub projects: collections::BTreeSet<RemoteProject>,
    /// Name to use for this server in UI.
    pub nickname: Option<String>,
    /// By default Zed will download the binary to the host directly.
    /// If this is set to true, Zed will download the binary to your local machine,
    /// and then upload it over the SSH connection. Useful if your SSH server has
    /// limited outbound internet access.
    pub upload_binary_over_ssh: Option<bool>,

    /// Port forwards to establish for this connection.
    pub port_forwards: Option<Vec<SshPortForwardOption>>,
    /// Timeout in seconds for SSH connection and downloading the remote server binary.
    /// Defaults to 10 seconds if not specified.
    pub connection_timeout: Option<u16>,
}

/// A WSL distribution to connect to.
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom, Debug)]
pub struct WslConnection {
    /// The name of the WSL distribution.
    pub distro_name: String,
    /// The user to connect as.
    pub user: Option<String>,
    /// The projects that have been opened in this distribution.
    #[serde(default)]
    pub projects: BTreeSet<RemoteProject>,
}

/// A project that has been opened on a remote server.
#[with_fallible_options]
#[derive(
    Clone, Debug, Default, Serialize, PartialEq, Eq, PartialOrd, Ord, Deserialize, JsonSchema,
)]
pub struct RemoteProject {
    /// The paths to open on the remote server.
    pub paths: Vec<String>,
}

/// A port forwarding configuration for an SSH connection.
#[with_fallible_options]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, JsonSchema, MergeFrom)]
pub struct SshPortForwardOption {
    /// The local host to bind.
    pub local_host: Option<String>,
    /// The local port to bind.
    pub local_port: u16,
    /// The remote host to forward to.
    pub remote_host: Option<String>,
    /// The remote port to forward to.
    pub remote_port: u16,
}

/// Settings for configuring REPL display and behavior.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ReplSettingsContent {
    /// Maximum number of lines to keep in REPL's scrollback buffer.
    /// Clamped with [4, 256] range.
    ///
    /// Default: 32
    pub max_lines: Option<usize>,
    /// Maximum number of columns to keep in REPL's scrollback buffer.
    /// Clamped with [20, 512] range.
    ///
    /// Default: 128
    pub max_columns: Option<usize>,
    /// Whether to show small single-line outputs inline instead of in a block.
    ///
    /// Default: true
    pub inline_output: Option<bool>,
    /// Maximum number of characters for an output to be shown inline.
    /// Only applies when `inline_output` is true.
    ///
    /// Default: 50
    pub inline_output_max_length: Option<usize>,
    /// Maximum number of lines of output to display before scrolling.
    /// Set to 0 to disable output height limits.
    ///
    /// Default: 0
    pub output_max_height_lines: Option<usize>,
}

/// Settings for configuring the which-key popup behaviour.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct WhichKeySettingsContent {
    /// Whether to show the which-key popup when holding down key combinations
    ///
    /// Default: false
    pub enabled: Option<bool>,
    /// Delay in milliseconds before showing the which-key popup.
    ///
    /// Default: 700
    pub delay_ms: Option<u64>,
}

/// An ExtendingVec in the settings can only accumulate new values.
///
/// This is useful for things like private files where you only want
/// to allow new values to be added.
///
/// Consider using a HashMap<String, bool> instead of this type
/// (like auto_install_extensions) so that user settings files can both add
/// and remove values from the set.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ExtendingVec<T>(pub Vec<T>);

impl<T> Into<Vec<T>> for ExtendingVec<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}
impl<T> From<Vec<T>> for ExtendingVec<T> {
    fn from(vec: Vec<T>) -> Self {
        ExtendingVec(vec)
    }
}

impl<T: Clone> merge_from::MergeFrom for ExtendingVec<T> {
    fn merge_from(&mut self, other: &Self) {
        self.0.extend_from_slice(other.0.as_slice());
    }
}

/// The placeholder entry that expands to the inherited list when merging a [`SplicingVec`].
pub const REST_OF_FILE_SCAN_EXCLUSIONS: &str = "...";

/// A SplicingVec in the settings replaces the value it merges over, except that
/// a `...` entry expands to that previous value.
///
/// This lets a settings file add to a list without restating what it inherits,
/// while omitting `...` still replaces the list outright. Unlike ExtendingVec,
/// entries can be dropped by leaving `...` out and listing what to keep.
///
/// Entries collapse to their first occurrence, so naming a value that `...`
/// already covers keeps it at the position it was written in rather than
/// repeating it.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SplicingVec(pub Vec<String>);

impl From<Vec<String>> for SplicingVec {
    fn from(vec: Vec<String>) -> Self {
        SplicingVec(vec)
    }
}

impl merge_from::MergeFrom for SplicingVec {
    fn merge_from(&mut self, other: &Self) {
        let inherited = std::mem::take(&mut self.0);
        self.0 = other
            .0
            .iter()
            .flat_map(|entry| {
                if entry == REST_OF_FILE_SCAN_EXCLUSIONS {
                    inherited.clone()
                } else {
                    vec![entry.clone()]
                }
            })
            .collect::<IndexSet<_>>()
            .into_iter()
            .collect();
    }
}

/// An ExtendingSet in the settings can only accumulate new values, and ignores
/// values that are already present, so merging the same source more than once
/// (e.g. re-importing VS Code settings) is idempotent.
///
/// Insertion order is preserved, so it round-trips through the user's settings
/// file without reordering their entries.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ExtendingSet<T: std::hash::Hash + Eq>(pub IndexSet<T>);

impl<T: std::hash::Hash + Eq> From<Vec<T>> for ExtendingSet<T> {
    fn from(vec: Vec<T>) -> Self {
        ExtendingSet(vec.into_iter().collect())
    }
}

impl<T: Clone + std::hash::Hash + Eq> merge_from::MergeFrom for ExtendingSet<T> {
    fn merge_from(&mut self, other: &Self) {
        self.0.extend(other.0.iter().cloned());
    }
}

/// A SaturatingBool in the settings can only ever be set to true,
/// later attempts to set it to false will be ignored.
///
/// Used by `disable_ai`.
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SaturatingBool(pub bool);

impl From<bool> for SaturatingBool {
    fn from(value: bool) -> Self {
        SaturatingBool(value)
    }
}

impl From<SaturatingBool> for bool {
    fn from(value: SaturatingBool) -> bool {
        value.0
    }
}

impl merge_from::MergeFrom for SaturatingBool {
    fn merge_from(&mut self, other: &Self) {
        self.0 |= other.0
    }
}

/// A delay duration in milliseconds.
#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    MergeFrom,
    JsonSchema,
)]
#[serde(transparent)]
pub struct DelayMs(pub u64);

impl From<u64> for DelayMs {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl std::fmt::Display for DelayMs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}ms", self.0)
    }
}

impl std::str::FromStr for DelayMs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .strip_suffix("ms")
            .unwrap_or(s.trim())
            .parse::<u64>()
            .map(DelayMs)
            .with_context(|| format!("failed to parse delay duration: {s}"))
    }
}

impl SettingsContent {
    /// The default values for every Zed setting, as shipped in
    /// `assets/settings/default.json` (which is generated from this function).
    ///
    /// Every field must be assigned explicitly here, so that adding a new
    /// setting is a compile error until it is given a default value.
    pub fn defaults() -> Self {
        Self {
            project: ProjectSettingsContent::defaults(),
            theme: Box::new(ThemeSettingsContent::defaults()),
            extension: ExtensionSettingsContent::defaults(),
            workspace: WorkspaceSettingsContent::defaults(),
            editor: EditorSettingsContent::defaults(),
            remote: RemoteSettingsContent::defaults(),
            file_finder: Some(FileFinderSettingsContent::defaults()),
            git_panel: Some(GitPanelSettingsContent::defaults()),
            tabs: Some(ItemSettingsContent::defaults()),
            tab_bar: Some(TabBarSettingsContent::defaults()),
            status_bar: Some(StatusBarSettingsContent::defaults()),
            preview_tabs: Some(PreviewTabsSettingsContent::defaults()),
            agent: Some(AgentSettingsContent::defaults()),
            agent_servers: Some(AllAgentServersSettings::defaults()),
            audio: Some(AudioSettingsContent::defaults()),
            auto_update: Some(true),
            base_keymap: Some(BaseKeymapContent::Zed),
            collaboration_panel: Some(PanelSettingsContent::defaults()),
            debugger: Some(DebuggerSettingsContent::defaults()),
            diagnostics: Some(DiagnosticsSettingsContent::defaults()),
            git: Some(GitSettings::defaults()),
            global_lsp_settings: Some(GlobalLspSettingsContent::defaults()),
            image_viewer: Some(ImageViewerSettingsContent::defaults()),
            markdown_preview: Some(MarkdownPreviewSettingsContent::defaults()),
            repl: Some(ReplSettingsContent::defaults()),
            helix_mode: Some(false),
            hide_mouse: Some(HideMouseMode::OnTypingAndAction),
            journal: Some(JournalSettingsContent::defaults()),
            log: Some(HashMap::default()),
            line_indicator_format: Some(LineIndicatorFormat::Long),
            language_models: Some(AllLanguageModelSettingsContent::defaults()),
            outline_panel: Some(OutlinePanelSettingsContent::defaults()),
            project_panel: Some(ProjectPanelSettingsContent::defaults()),
            node: Some(NodeBinarySettings::defaults()),
            proxy: Some(String::new()),
            reduce_motion: Some(ReduceMotionMode::Off),
            server_url: Some(String::from("https://zed.dev")),
            credentials_url: None,
            session: Some(SessionSettingsContent::defaults()),
            telemetry: Some(TelemetrySettingsContent::defaults()),
            terminal: Some(TerminalSettingsContent::defaults()),
            title_bar: Some(TitleBarSettingsContent::defaults()),
            vim_mode: Some(false),
            calls: Some(CallSettingsContent::defaults()),
            which_key: Some(WhichKeySettingsContent::defaults()),
            vim: Some(VimSettingsContent::defaults()),
            modeline_lines: Some(5),
            feature_flags: None,
            instrumentation: Some(InstrumentationSettingsContent::defaults()),
        }
    }
}

fn instrumentation_enabled_override() -> SettingsContent {
    SettingsContent {
        instrumentation: Some(InstrumentationSettingsContent {
            performance_profiler: Some(PerformanceProfilerSettingsContent {
                enabled: Some(true),
            }),
        }),
        ..Default::default()
    }
}

fn windows_platform_override() -> SettingsContent {
    SettingsContent {
        project: ProjectSettingsContent {
            all_languages: AllLanguageSettingsContent::windows_defaults_override(),
            ..Default::default()
        },
        ..Default::default()
    }
}

impl UserSettingsContent {
    /// The full contents of the generated `assets/settings/default.json`,
    /// including the release-channel and platform override sections.
    pub fn defaults() -> Self {
        Self {
            content: Box::new(SettingsContent::defaults()),
            release_channel_overrides: ReleaseChannelOverrides {
                dev: Some(Box::new(instrumentation_enabled_override())),
                nightly: Some(Box::new(instrumentation_enabled_override())),
                preview: Some(Box::default()),
                stable: Some(Box::default()),
            },
            platform_overrides: PlatformOverrides {
                macos: Some(Box::default()),
                linux: Some(Box::default()),
                windows: Some(Box::new(windows_platform_override())),
            },
            profiles: IndexMap::default(),
        }
    }
}

impl InstrumentationSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            performance_profiler: Some(PerformanceProfilerSettingsContent {
                enabled: Some(false),
            }),
        }
    }
}

impl AudioSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            output_audio_device: None,
            input_audio_device: None,
        }
    }
}

impl TelemetrySettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            diagnostics: Some(true),
            metrics: Some(true),
            anthropic_retention: Some(false),
        }
    }
}

impl DebuggerSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            stepping_granularity: Some(SteppingGranularity::Line),
            save_breakpoints: Some(true),
            button: Some(true),
            timeout: Some(2000),
            log_dap_communications: Some(true),
            format_dap_log_messages: Some(true),
            dock: Some(DockPosition::Bottom),
        }
    }
}

impl CallSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            mute_on_join: Some(false),
            share_on_join: Some(false),
        }
    }
}

impl GitPanelSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            button: Some(true),
            dock: Some(DockPosition::Right),
            default_width: Some(PixelSetting(360.0)),
            status_style: Some(StatusStyle::Icon),
            file_icons: Some(false),
            folder_icons: Some(true),
            scrollbar: Some(ScrollbarSettings { show: None }),
            fallback_branch_name: Some(String::from("main")),
            sort_by: Some(GitPanelSortBy::Path),
            group_by: Some(GitPanelGroupBy::Status),
            collapse_untracked_diff: Some(false),
            tree_view: Some(false),
            diff_stats: Some(true),
            show_count_badge: Some(false),
            starts_open: Some(false),
            commit_title_max_length: Some(0),
            entry_primary_click_action: Some(GitPanelClickBehavior::ProjectDiff),
        }
    }
}

impl PanelSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            button: Some(true),
            dock: Some(DockPosition::Right),
            default_width: Some(PixelSetting(240.0)),
        }
    }
}

impl FileFinderSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            file_icons: Some(true),
            modal_max_width: Some(FileFinderWidthContent::Small),
            skip_focus_for_active_in_search: Some(true),
            include_ignored: Some(IncludeIgnoredContent::Smart),
            include_channels: Some(false),
        }
    }
}

impl VimSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            default_mode: Some(ModeContent::Normal),
            toggle_relative_line_numbers: Some(false),
            use_system_clipboard: Some(UseSystemClipboard::Always),
            use_smartcase_find: Some(false),
            use_regex_search: Some(true),
            gdefault: Some(false),
            custom_digraphs: Some(HashMap::default()),
            highlight_on_yank_duration: Some(200),
            cursor_shape: Some(CursorShapeSettings {
                normal: Some(CursorShape::Block),
                replace: Some(CursorShape::Underline),
                visual: Some(CursorShape::Block),
                insert: Some(VimInsertModeCursorShape::Inherit),
            }),
            show_edit_predictions_in_normal_mode: Some(false),
        }
    }
}

impl JournalSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            path: Some(String::from("~")),
            hour_format: Some(HourFormat::Hour12),
        }
    }
}

impl OutlinePanelSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            button: Some(true),
            default_width: Some(PixelSetting(300.0)),
            dock: Some(DockSide::Right),
            file_icons: Some(true),
            folder_icons: Some(true),
            git_status: Some(true),
            indent_size: Some(PixelSetting(20.0)),
            auto_reveal_entries: Some(true),
            auto_fold_dirs: Some(true),
            indent_guides: Some(IndentGuidesSettingsContent {
                show: Some(ShowIndentGuides::Always),
            }),
            scrollbar: Some(ScrollbarSettingsContent { show: None }),
            expand_outlines_with_depth: Some(100),
        }
    }
}

impl MarkdownPreviewSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            limit_content_width: Some(true),
            max_width: Some(PixelSetting(800.0)),
        }
    }
}

impl ImageViewerSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            unit: Some(ImageFileSizeUnit::Binary),
        }
    }
}

impl RemoteSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            ssh_connections: Some(Vec::new()),
            wsl_connections: None,
            dev_container_connections: None,
            read_ssh_config: Some(true),
            use_podman: None,
            dev_container_use_buildkit: None,
        }
    }
}

impl ReplSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            max_lines: Some(32),
            max_columns: Some(128),
            inline_output: None,
            inline_output_max_length: None,
            output_max_height_lines: Some(0),
        }
    }
}

impl WhichKeySettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            enabled: Some(false),
            delay_ms: Some(1000),
        }
    }
}

#[cfg(test)]
mod default_settings_tests {
    use super::*;

    fn diff_values(
        path: &str,
        json: &serde_json::Value,
        rust: &serde_json::Value,
        out: &mut Vec<String>,
    ) {
        use serde_json::Value;
        match (json, rust) {
            (Value::Object(json), Value::Object(rust)) => {
                for (key, json_value) in json {
                    let sub_path = format!("{path}/{key}");
                    match rust.get(key) {
                        Some(rust_value) => diff_values(&sub_path, json_value, rust_value, out),
                        None => out.push(format!(
                            "missing from defaults(): {sub_path} = {json_value}"
                        )),
                    }
                }
                for (key, rust_value) in rust {
                    if !json.contains_key(key) {
                        out.push(format!("extra in defaults(): {path}/{key} = {rust_value}"));
                    }
                }
            }
            _ => {
                if json != rust {
                    out.push(format!("differs at {path}: json={json} rust={rust}"));
                }
            }
        }
    }

    #[test]
    fn defaults_match_default_json() {
        let text = include_str!("../../../assets/settings/default.json");
        let parsed: UserSettingsContent = parse_json_with_comments(text).unwrap();
        let constructed = UserSettingsContent::defaults();
        if parsed == constructed {
            return;
        }
        let parsed = serde_json::to_value(&parsed).unwrap();
        let constructed = serde_json::to_value(&constructed).unwrap();
        let mut out = Vec::new();
        diff_values("", &parsed, &constructed, &mut out);
        panic!(
            "UserSettingsContent::defaults() diverges from assets/settings/default.json ({} differences):\n{}",
            out.len(),
            out.join("\n")
        );
    }
}
