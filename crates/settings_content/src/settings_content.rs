#![warn(missing_docs)]
//! Settings content types for Zed's JSON settings files.
//!
//! This crate defines the structures that represent all user-configurable settings in Zed.

mod agent;
mod editor;
mod extension;
mod fallible_options;
mod language;
mod language_model;
/// Trait and helpers for recursively merging settings structures.
pub mod merge_from;
mod project;
mod serde_helper;
mod terminal;
mod theme;
mod title_bar;
mod workspace;

pub use agent::*;
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

use collections::{HashMap, IndexMap};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

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
                #[doc = concat!("Settings overrides for the `", stringify!($field), "` context.")]
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
use std::collections::BTreeSet;
use std::sync::Arc;
pub use util::serde::default_true;

/// The result of parsing a settings JSON file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseStatus {
    /// Settings were parsed successfully
    Success,
    /// Settings failed to parse
    Failed {
        /// The error message describing why parsing failed.
        error: String,
    },
}

/// The top-level settings content structure that holds all configurable settings for Zed.
#[with_fallible_options]
#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct SettingsContent {
    /// Project-related settings such as file types and LSP configuration.
    #[serde(flatten)]
    pub project: ProjectSettingsContent,

    /// Theme and appearance settings.
    #[serde(flatten)]
    pub theme: Box<ThemeSettingsContent>,

    /// Extension-related settings such as auto-install preferences.
    #[serde(flatten)]
    pub extension: ExtensionSettingsContent,

    /// Workspace layout and panel settings.
    #[serde(flatten)]
    pub workspace: WorkspaceSettingsContent,

    /// Editor behavior and display settings.
    #[serde(flatten)]
    pub editor: EditorSettingsContent,

    /// Remote development connection settings.
    #[serde(flatten)]
    pub remote: RemoteSettingsContent,

    /// Settings related to the file finder.
    pub file_finder: Option<FileFinderSettingsContent>,

    /// Configuration for the Git Panel.
    pub git_panel: Option<GitPanelSettingsContent>,

    /// Configuration for tab items in the editor.
    pub tabs: Option<ItemSettingsContent>,
    /// Configuration for the tab bar.
    pub tab_bar: Option<TabBarSettingsContent>,
    /// Configuration for the status bar.
    pub status_bar: Option<StatusBarSettingsContent>,

    /// Configuration for preview tab behavior.
    pub preview_tabs: Option<PreviewTabsSettingsContent>,

    /// Configuration for the AI Agent panel.
    pub agent: Option<AgentSettingsContent>,
    /// Configuration for AI agent servers.
    pub agent_servers: Option<AllAgentServersSettings>,

    /// Configuration of audio in Zed.
    pub audio: Option<AudioSettingsContent>,

    /// Whether or not to automatically check for updates.
    ///
    /// This setting may be ignored on Linux if Zed was installed through a package manager.
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

    /// Configuration for the debugger.
    pub debugger: Option<DebuggerSettingsContent>,

    /// Configuration for Diagnostics-related features.
    pub diagnostics: Option<DiagnosticsSettingsContent>,

    /// Configuration for Git-related features
    pub git: Option<GitSettings>,

    /// Common language server settings.
    pub global_lsp_settings: Option<GlobalLspSettingsContent>,

    /// The settings for the image viewer.
    pub image_viewer: Option<ImageViewerSettingsContent>,

    /// Configuration for the REPL (Read-Eval-Print Loop).
    pub repl: Option<ReplSettingsContent>,

    /// Whether or not to enable Helix mode and key bindings.
    ///
    /// Enabling this mode will automatically enable vim mode.
    ///
    /// Default: false
    pub helix_mode: Option<bool>,

    /// Configuration for the journal feature.
    pub journal: Option<JournalSettingsContent>,

    /// A map of log scopes to the desired log level.
    /// Useful for filtering out noisy logs or enabling more verbose logging.
    ///
    /// Example: {"log": {"client": "warn"}}
    pub log: Option<HashMap<String, String>>,

    /// Whether to show full labels or abbreviated labels in the line indicator.
    ///
    /// - `short`: e.g., "2 s, 15 l, 32 c"
    /// - `long`: e.g., "2 selections, 15 lines, 32 characters"
    ///
    /// Default: long
    pub line_indicator_format: Option<LineIndicatorFormat>,

    /// Configuration for language model providers.
    pub language_models: Option<AllLanguageModelSettingsContent>,

    /// Configuration for the Outline Panel.
    pub outline_panel: Option<OutlinePanelSettingsContent>,

    /// Configuration for the Project Panel.
    pub project_panel: Option<ProjectPanelSettingsContent>,

    /// Configuration for the Message Editor
    pub message_editor: Option<MessageEditorSettings>,

    /// Configuration for Node-related features
    pub node: Option<NodeBinarySettings>,

    /// Configuration for the Notification Panel
    pub notification_panel: Option<NotificationPanelSettingsContent>,

    /// HTTP proxy URL for outbound network requests.
    ///
    /// The proxy protocol is specified by the URI scheme.
    /// Supported schemes: `http`, `https`, `socks4`, `socks4a`, `socks5`, `socks5h`.
    /// When unset, Zed falls back to proxy settings from environment variables.
    ///
    /// Default: ""
    pub proxy: Option<String>,

    /// The URL of the Zed server to connect to.
    ///
    /// If the environment variable `ZED_SERVER_URL` is set, it will override this setting.
    ///
    /// Default: "https://zed.dev"
    pub server_url: Option<String>,

    /// Configuration for session-related features
    pub session: Option<SessionSettingsContent>,
    /// Control what info is collected by Zed.
    pub telemetry: Option<TelemetrySettingsContent>,

    /// Configuration of the terminal in Zed.
    pub terminal: Option<TerminalSettingsContent>,

    /// Configuration for the title bar.
    pub title_bar: Option<TitleBarSettingsContent>,

    /// Whether or not to enable Vim mode.
    ///
    /// Default: false
    pub vim_mode: Option<bool>,

    /// Configuration for voice calls in Zed.
    pub calls: Option<CallSettingsContent>,

    /// Settings for the which-key popup.
    pub which_key: Option<WhichKeySettingsContent>,

    /// Settings related to Vim mode in Zed.
    pub vim: Option<VimSettingsContent>,

    /// Number of lines to search for modelines at the beginning and end of files.
    ///
    /// Modelines contain editor directives (e.g., vim/emacs settings) that configure
    /// the editor behavior for specific files. A value of 0 disables modeline support.
    ///
    /// Default: 5
    pub modeline_lines: Option<usize>,
}

impl SettingsContent {
    /// Returns a mutable reference to the per-language settings map.
    pub fn languages_mut(&mut self) -> &mut HashMap<String, LanguageSettingsContent> {
        &mut self.project.all_languages.languages.0
    }
}

// These impls are there to optimize builds by avoiding monomorphization downstream. Yes, they're repetitive, but using default impls
// break the optimization, for whatever reason.
/// Trait for types that can be parsed from a JSON settings file.
///
/// Explicit impls are provided instead of a blanket impl to avoid monomorphizing
/// parse logic in downstream crates.
pub trait RootUserSettings: Sized + DeserializeOwned {
    /// Parse settings from a JSON string, returning any parse errors as a [`ParseStatus`].
    fn parse_json(json: &str) -> (Option<Self>, ParseStatus);
    /// Parse settings from a JSON string that may contain comments, returning an error on failure.
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
    /// Settings overrides applied only when running a specific release channel.
    #[with_fallible_options]
    #[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
    pub struct ReleaseChannelOverrides { dev, nightly, preview, stable }
}

settings_overrides! {
    /// Settings overrides applied only when running on a specific operating system.
    #[with_fallible_options]
    #[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
    pub struct PlatformOverrides { macos, linux, windows }
}

/// The full settings content as read from a user's settings file, including
/// per-release-channel and per-platform overrides and named profiles.
#[with_fallible_options]
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct UserSettingsContent {
    #[serde(flatten)]
    /// The base settings content.
    pub content: Box<SettingsContent>,

    #[serde(flatten)]
    /// Settings overrides applied based on the current release channel.
    pub release_channel_overrides: ReleaseChannelOverrides,

    #[serde(flatten)]
    /// Settings overrides applied based on the current operating system.
    pub platform_overrides: PlatformOverrides,

    #[serde(default)]
    /// Named settings profiles that can be activated on demand.
    pub profiles: IndexMap<String, SettingsContent>,
}

/// Settings content provided by extensions, limited to language-related settings.
pub struct ExtensionsSettingsContent {
    /// Language-specific settings from installed extensions.
    pub all_languages: AllLanguageSettingsContent,
}

/// Base key bindings scheme. Base keymaps can be overridden with user keymaps.
///
/// Default: VSCode
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
    /// VSCode-style keybindings (default).
    #[default]
    VSCode,
    /// JetBrains IDE-style keybindings.
    JetBrains,
    /// Sublime Text-style keybindings.
    SublimeText,
    /// Atom-style keybindings.
    Atom,
    /// TextMate-style keybindings.
    TextMate,
    /// Emacs-style keybindings.
    Emacs,
    /// Cursor-style keybindings.
    Cursor,
    /// No base keymap; use only user-defined keybindings.
    None,
}

impl strum::VariantNames for BaseKeymapContent {
    const VARIANTS: &'static [&'static str] = &[
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
    /// Automatically increase or decrease you microphone's volume. This affects how
    /// loud you sound to others.
    ///
    /// Recommended: off (default)
    /// Microphones are too quite in zed, until everyone is on experimental
    /// audio and has auto speaker volume on this will make you very loud
    /// compared to other speakers.
    #[serde(rename = "experimental.auto_microphone_volume")]
    pub auto_microphone_volume: Option<bool>,
    /// Remove background noises. Works great for typing, cars, dogs, AC. Does
    /// not work well on music.
    /// Select specific output audio device.
    #[serde(rename = "experimental.output_audio_device")]
    pub output_audio_device: Option<AudioOutputDeviceName>,
    /// Select specific input audio device.
    #[serde(rename = "experimental.input_audio_device")]
    pub input_audio_device: Option<AudioInputDeviceName>,
}

/// The name of the audio output device to use.
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

/// The name of the audio input device to use.
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
}

impl Default for TelemetrySettingsContent {
    fn default() -> Self {
        Self {
            diagnostics: Some(true),
            metrics: Some(true),
        }
    }
}

/// Configuration for the debugger.
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
/// The position of a dockable panel in the editor.
#[serde(rename_all = "snake_case")]
pub enum DockPosition {
    /// Dock the panel on the left side.
    Left,
    /// Dock the panel on the bottom.
    Bottom,
    /// Dock the panel on the right side.
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

/// Configuration for the Git Panel.
#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct GitPanelSettingsContent {
    /// Whether to show the panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Where to dock the panel.
    ///
    /// Default: left
    pub dock: Option<DockPosition>,
    /// Default width of the panel in pixels.
    ///
    /// Default: 360
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub default_width: Option<f32>,
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

    /// Whether to sort entries in the panel by path
    /// or by status (the default).
    ///
    /// Default: false
    pub sort_by_path: Option<bool>,

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
}

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
/// How Git entry statuses are displayed in the panel.
#[serde(rename_all = "snake_case")]
pub enum StatusStyle {
    /// Show status as a colored icon.
    #[default]
    Icon,
    /// Show status as a colored label.
    LabelColor,
}

/// How and when the scrollbar should be displayed.
#[with_fallible_options]
#[derive(
    Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq,
)]
pub struct ScrollbarSettings {
    /// When to show the scrollbar.
    pub show: Option<ShowScrollbar>,
}

/// Configuration for the Notification Panel.
#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct NotificationPanelSettingsContent {
    /// Whether to show the panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Where to dock the panel.
    ///
    /// Default: right
    pub dock: Option<DockPosition>,
    /// Default width of the panel in pixels.
    ///
    /// Default: 380
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub default_width: Option<f32>,
    /// Whether to show a badge on the notification panel icon with the count of unread notifications.
    ///
    /// Default: false
    pub show_count_badge: Option<bool>,
}

/// Configuration for a collapsible side panel.
#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct PanelSettingsContent {
    /// Whether to show the panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Where to dock the panel.
    ///
    /// Default: left
    pub dock: Option<DockPosition>,
    /// Default width of the panel in pixels.
    ///
    /// Default: 240
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub default_width: Option<f32>,
}

/// Configuration for the Message Editor used in collaboration features.
#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct MessageEditorSettings {
    /// Whether to automatically replace emoji shortcodes with emoji characters.
    /// For example: typing `:wave:` gets replaced with `👋`.
    ///
    /// Default: true
    pub auto_replace_emoji_shortcode: Option<bool>,
}

/// Configuration for the File Finder dialog.
#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct FileFinderSettingsContent {
    /// Whether to show file icons in the file finder.
    ///
    /// Default: true
    pub file_icons: Option<bool>,
    /// Determines how much space the file finder can take up in relation to the available window width.
    ///
    /// - `small`: Essentially a fixed width.
    /// - `medium`: Medium relative width.
    /// - `large`: Large relative width.
    /// - `xlarge`: Extra-large relative width.
    /// - `full`: Removes any horizontal padding, consuming the full viewport width.
    ///
    /// Default: small
    pub modal_max_width: Option<FileFinderWidthContent>,
    /// Whether the file finder should skip focus for the currently active file in search results.
    ///
    /// When `true`, if the active file appears as the first result, auto-focus skips it
    /// and focuses the second result instead.
    /// When `false`, the first result always receives focus, even if it is the active file.
    ///
    /// Default: true
    pub skip_focus_for_active_in_search: Option<bool>,
    /// Whether to include gitignored files when searching.
    ///
    /// - `all`: Use all gitignored files.
    /// - `indexed`: Use only files Zed had already indexed.
    /// - `smart`: Include ignored files only when searching from within a gitignored worktree.
    ///
    /// Default: smart
    pub include_ignored: Option<IncludeIgnoredContent>,
    /// Whether to include text channels in file finder results.
    ///
    /// Default: false
    pub include_channels: Option<bool>,
}

/// Whether and how to include gitignored files in the file finder.
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
/// The maximum width of the file finder modal relative to the window width.
#[serde(rename_all = "lowercase")]
pub enum FileFinderWidthContent {
    /// Small width (~30% of window).
    #[default]
    Small,
    /// Medium width (~50% of window).
    Medium,
    /// Large width (~70% of window).
    Large,
    /// Extra-large width (~90% of window).
    XLarge,
    /// Full window width.
    Full,
}

/// Settings related to Vim emulation in Zed.
#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug, JsonSchema, MergeFrom)]
pub struct VimSettingsContent {
    /// The default Vim mode to use when opening a buffer.
    ///
    /// Default: normal
    pub default_mode: Option<ModeContent>,
    /// Whether to toggle relative line numbers when entering or leaving insert mode.
    ///
    /// Default: false
    pub toggle_relative_line_numbers: Option<bool>,
    /// When to use the system clipboard in Vim operations.
    ///
    /// Default: always
    pub use_system_clipboard: Option<UseSystemClipboard>,
    /// Whether to use smartcase matching for find operations.
    ///
    /// Default: false
    pub use_smartcase_find: Option<bool>,
    /// When enabled, the `:substitute` command replaces all matches in a line
    /// by default. The 'g' flag then toggles this behavior.
    ///
    /// Default: false
    pub gdefault: Option<bool>,
    /// Custom digraph mappings. A digraph is a two-character sequence that inserts
    /// a special character (e.g., `"a` for `ä`).
    pub custom_digraphs: Option<HashMap<String, Arc<str>>>,
    /// Duration in milliseconds to highlight yanked text. Set to 0 to disable.
    ///
    /// Default: 200
    pub highlight_on_yank_duration: Option<u64>,
    /// Cursor shape for each Vim mode.
    ///
    /// The shape can be one of: `block`, `bar`, `underline`, `hollow`.
    pub cursor_shape: Option<CursorShapeSettings>,
}

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
/// The default Vim mode to use when opening a buffer.
#[serde(rename_all = "snake_case")]
pub enum ModeContent {
    /// Normal mode for navigation and text manipulation commands.
    #[default]
    Normal,
    /// Insert mode for typing text directly.
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
    /// Set to `inherit` to use the editor's `cursor_shape` setting.
    ///
    /// Default: inherit
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
    /// - `hour12`: 12-hour clock (e.g., 3:00 PM)
    /// - `hour24`: 24-hour clock (e.g., 15:00)
    ///
    /// Default: hour12
    pub hour_format: Option<HourFormat>,
}

/// The format to use when displaying hours in journal entries.
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HourFormat {
    /// Use 12-hour clock format (e.g., 3:00 PM).
    #[default]
    Hour12,
    /// Use 24-hour clock format (e.g., 15:00).
    Hour24,
}

/// Configuration for the Outline Panel.
#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct OutlinePanelSettingsContent {
    /// Whether to show the outline panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Default width of the outline panel in pixels.
    ///
    /// Default: 300
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub default_width: Option<f32>,
    /// The position of outline panel
    ///
    /// Default: left
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
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub indent_size: Option<f32>,
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
/// The side of the editor where a panel can be docked.
#[serde(rename_all = "snake_case")]
pub enum DockSide {
    /// Dock the panel on the left side.
    Left,
    /// Dock the panel on the right side.
    Right,
}

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
/// When to display indent guides in a panel.
#[serde(rename_all = "snake_case")]
pub enum ShowIndentGuides {
    /// Always show indent guides.
    Always,
    /// Never show indent guides.
    Never,
}

/// Settings related to indent guides in panel views.
#[with_fallible_options]
#[derive(
    Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq, Default,
)]
pub struct IndentGuidesSettingsContent {
    /// When to show indent guides.
    pub show: Option<ShowIndentGuides>,
}

/// Format for the cursor position indicator shown in the status bar.
#[derive(Clone, Copy, Default, PartialEq, Debug, JsonSchema, MergeFrom, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LineIndicatorFormat {
    /// Abbreviated format, e.g., "2 s, 15 l, 32 c".
    Short,
    /// Full format, e.g., "2 selections, 15 lines, 32 characters".
    #[default]
    Long,
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

/// The unit system used when displaying image file sizes.
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

/// Configuration for remote development connections.
#[with_fallible_options]
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct RemoteSettingsContent {
    /// List of configured SSH remote server connections.
    ///
    /// You can also configure these from `project: Open Remote` in the command palette.
    /// Zed will also pull connection settings from `~/.ssh/config`.
    pub ssh_connections: Option<Vec<SshConnection>>,
    /// List of configured WSL (Windows Subsystem for Linux) connections.
    pub wsl_connections: Option<Vec<WslConnection>>,
    /// List of configured dev container connections.
    pub dev_container_connections: Option<Vec<DevContainerConnection>>,
    /// Whether to read SSH host entries from `~/.ssh/config` automatically.
    ///
    /// Default: true
    pub read_ssh_config: Option<bool>,
    /// Whether to use Podman instead of Docker for dev container connections.
    ///
    /// Default: false
    pub use_podman: Option<bool>,
}

/// A connection to a running dev container.
#[with_fallible_options]
#[derive(
    Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom, Hash,
)]
pub struct DevContainerConnection {
    /// Display name for this dev container connection.
    pub name: String,
    /// Username to connect as inside the container.
    pub remote_user: String,
    /// The container ID or name to connect to.
    pub container_id: String,
    /// Whether to use Podman instead of Docker for this connection.
    pub use_podman: bool,
}

/// A connection to a remote server over SSH.
#[with_fallible_options]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct SshConnection {
    /// The hostname or IP address of the remote server.
    pub host: String,
    /// Username for the SSH connection. Defaults to the current user if not specified.
    pub username: Option<String>,
    /// Port number for the SSH connection.
    ///
    /// Default: 22
    pub port: Option<u16>,
    /// Additional arguments to pass to the SSH command.
    #[serde(default)]
    pub args: Vec<String>,
    /// List of projects to open on this server.
    #[serde(default)]
    pub projects: collections::BTreeSet<RemoteProject>,
    /// Name to use for this server in UI.
    pub nickname: Option<String>,
    /// By default Zed will download the binary to the host directly.
    /// If this is set to true, Zed will download the binary to your local machine,
    /// and then upload it over the SSH connection. Useful if your SSH server has
    /// limited outbound internet access.
    pub upload_binary_over_ssh: Option<bool>,
    /// Port forwarding configurations for this connection.
    pub port_forwards: Option<Vec<SshPortForwardOption>>,
    /// Timeout in seconds for SSH connection and downloading the remote server binary.
    /// Defaults to 10 seconds if not specified.
    pub connection_timeout: Option<u16>,
}

/// A connection to a WSL (Windows Subsystem for Linux) distribution.
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom, Debug)]
pub struct WslConnection {
    /// Name of the WSL distribution to connect to.
    pub distro_name: String,
    /// Username to connect as. Defaults to the distribution's default user if not specified.
    pub user: Option<String>,
    /// List of projects to open in this distribution.
    #[serde(default)]
    pub projects: BTreeSet<RemoteProject>,
}

/// A project to open on a remote server.
#[with_fallible_options]
#[derive(
    Clone, Debug, Default, Serialize, PartialEq, Eq, PartialOrd, Ord, Deserialize, JsonSchema,
)]
pub struct RemoteProject {
    /// Paths to open in the remote project.
    pub paths: Vec<String>,
}

/// A port forwarding configuration for an SSH connection.
#[with_fallible_options]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, JsonSchema, MergeFrom)]
pub struct SshPortForwardOption {
    /// The local host address to bind to. Defaults to `localhost` if not specified.
    pub local_host: Option<String>,
    /// The local port to forward traffic from.
    pub local_port: u16,
    /// The remote host to forward traffic to. Defaults to `localhost` if not specified.
    pub remote_host: Option<String>,
    /// The remote port to forward traffic to.
    pub remote_port: u16,
}

/// Configuration for the REPL (Read-Eval-Print Loop) display and behavior.
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
    /// Default: 1000
    pub delay_ms: Option<u64>,
}

/// A settings vector that can only accumulate new values when merging; existing values are never removed.
///
/// This is useful for things like private files where you only want
/// to allow new values to be added.
///
/// Consider using a `HashMap<String, bool>` instead of this type
/// (like `auto_install_extensions`) so that user settings files can both add
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

/// A boolean setting that can only ever be set to `true`; later attempts to set it to `false` are ignored.
///
/// Used by `disable_ai` to ensure the setting cannot be reversed by a lower-priority settings file.
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

/// A duration value in milliseconds used in settings.
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
    derive_more::FromStr,
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
