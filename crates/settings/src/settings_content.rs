mod agent;
mod editor;
mod extension;
mod language;
mod language_model;
mod project;
mod terminal;
mod theme;
mod workspace;

pub use agent::*;
pub use editor::*;
pub use extension::*;
pub use language::*;
pub use language_model::*;
pub use project::*;
pub use terminal::*;
pub use theme::*;
pub use workspace::*;

use collections::{HashMap, IndexMap};
use gpui::{App, SharedString};
use release_channel::ReleaseChannel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};
use std::collections::BTreeSet;
use std::env;
use std::sync::Arc;
pub use util::serde::default_true;

use crate::{ActiveSettingsProfileName, merge_from};

#[with_fallible_options]
#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct SettingsContent {
    #[serde(flatten)]
    pub project: ProjectSettingsContent,

    #[serde(flatten)]
    pub theme: Box<ThemeSettingsContent>,

    #[serde(flatten)]
    pub extension: ExtensionSettingsContent,

    #[serde(flatten)]
    pub workspace: WorkspaceSettingsContent,

    #[serde(flatten)]
    pub editor: EditorSettingsContent,

    #[serde(flatten)]
    pub remote: RemoteSettingsContent,

    /// Settings related to the file finder.
    pub file_finder: Option<FileFinderSettingsContent>,

    pub git_panel: Option<GitPanelSettingsContent>,

    pub tabs: Option<ItemSettingsContent>,
    pub tab_bar: Option<TabBarSettingsContent>,
    pub status_bar: Option<StatusBarSettingsContent>,

    pub preview_tabs: Option<PreviewTabsSettingsContent>,

    pub agent: Option<AgentSettingsContent>,
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

    pub debugger: Option<DebuggerSettingsContent>,

    /// Configuration for Diagnostics-related features.
    pub diagnostics: Option<DiagnosticsSettingsContent>,

    /// Configuration for Git-related features
    pub git: Option<GitSettings>,

    /// Common language server settings.
    pub global_lsp_settings: Option<GlobalLspSettingsContent>,

    /// The settings for the image viewer.
    pub image_viewer: Option<ImageViewerSettingsContent>,

    pub repl: Option<ReplSettingsContent>,

    /// Whether or not to enable Helix mode.
    ///
    /// Default: false
    pub helix_mode: Option<bool>,

    pub journal: Option<JournalSettingsContent>,

    /// A map of log scopes to the desired log level.
    /// Useful for filtering out noisy logs or enabling more verbose logging.
    ///
    /// Example: {"log": {"client": "warn"}}
    pub log: Option<HashMap<String, String>>,

    pub line_indicator_format: Option<LineIndicatorFormat>,

    pub language_models: Option<AllLanguageModelSettingsContent>,

    pub outline_panel: Option<OutlinePanelSettingsContent>,

    pub project_panel: Option<ProjectPanelSettingsContent>,

    /// Configuration for the Message Editor
    pub message_editor: Option<MessageEditorSettings>,

    /// Configuration for Node-related features
    pub node: Option<NodeBinarySettings>,

    /// Configuration for the Notification Panel
    pub notification_panel: Option<NotificationPanelSettingsContent>,

    pub proxy: Option<String>,

    /// The URL of the Zed server to connect to.
    pub server_url: Option<String>,

    /// Configuration for session-related features
    pub session: Option<SessionSettingsContent>,
    /// Control what info is collected by Zed.
    pub telemetry: Option<TelemetrySettingsContent>,

    /// Configuration of the terminal in Zed.
    pub terminal: Option<TerminalSettingsContent>,

    pub title_bar: Option<TitleBarSettingsContent>,

    /// Whether or not to enable Vim mode.
    ///
    /// Default: false
    pub vim_mode: Option<bool>,

    // Settings related to calls in Zed
    pub calls: Option<CallSettingsContent>,

    /// Whether to disable all AI features in Zed.
    ///
    /// Default: false
    pub disable_ai: Option<SaturatingBool>,

    /// Settings for the which-key popup.
    pub which_key: Option<WhichKeySettingsContent>,

    /// Settings related to Vim mode in Zed.
    pub vim: Option<VimSettingsContent>,
}

impl SettingsContent {
    pub fn languages_mut(&mut self) -> &mut HashMap<SharedString, LanguageSettingsContent> {
        &mut self.project.all_languages.languages.0
    }
}

#[with_fallible_options]
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct UserSettingsContent {
    #[serde(flatten)]
    pub content: Box<SettingsContent>,

    pub dev: Option<Box<SettingsContent>>,
    pub nightly: Option<Box<SettingsContent>>,
    pub preview: Option<Box<SettingsContent>>,
    pub stable: Option<Box<SettingsContent>>,

    pub macos: Option<Box<SettingsContent>>,
    pub windows: Option<Box<SettingsContent>>,
    pub linux: Option<Box<SettingsContent>>,

    #[serde(default)]
    pub profiles: IndexMap<String, SettingsContent>,
}

pub struct ExtensionsSettingsContent {
    pub all_languages: AllLanguageSettingsContent,
}

impl UserSettingsContent {
    pub fn for_release_channel(&self) -> Option<&SettingsContent> {
        match *release_channel::RELEASE_CHANNEL {
            ReleaseChannel::Dev => self.dev.as_deref(),
            ReleaseChannel::Nightly => self.nightly.as_deref(),
            ReleaseChannel::Preview => self.preview.as_deref(),
            ReleaseChannel::Stable => self.stable.as_deref(),
        }
    }

    pub fn for_os(&self) -> Option<&SettingsContent> {
        match env::consts::OS {
            "macos" => self.macos.as_deref(),
            "linux" => self.linux.as_deref(),
            "windows" => self.windows.as_deref(),
            _ => None,
        }
    }

    pub fn for_profile(&self, cx: &App) -> Option<&SettingsContent> {
        let Some(active_profile) = cx.try_global::<ActiveSettingsProfileName>() else {
            return None;
        };
        self.profiles.get(&active_profile.0)
    }
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
    #[default]
    VSCode,
    JetBrains,
    SublimeText,
    Atom,
    TextMate,
    Emacs,
    Cursor,
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

#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct TitleBarSettingsContent {
    /// Whether to show the branch icon beside branch switcher in the title bar.
    ///
    /// Default: false
    pub show_branch_icon: Option<bool>,
    /// Whether to show onboarding banners in the title bar.
    ///
    /// Default: true
    pub show_onboarding_banner: Option<bool>,
    /// Whether to show user avatar in the title bar.
    ///
    /// Default: true
    pub show_user_picture: Option<bool>,
    /// Whether to show the branch name button in the titlebar.
    ///
    /// Default: true
    pub show_branch_name: Option<bool>,
    /// Whether to show the project host and name in the titlebar.
    ///
    /// Default: true
    pub show_project_items: Option<bool>,
    /// Whether to show the sign in button in the title bar.
    ///
    /// Default: true
    pub show_sign_in: Option<bool>,
    /// Whether to show the user menu button in the title bar.
    ///
    /// Default: true
    pub show_user_menu: Option<bool>,
    /// Whether to show the menus in the title bar.
    ///
    /// Default: false
    pub show_menus: Option<bool>,
}

/// Configuration of audio in Zed.
#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct AudioSettingsContent {
    /// Opt into the new audio system.
    ///
    /// You need to rejoin a call for this setting to apply
    #[serde(rename = "experimental.rodio_audio")]
    pub rodio_audio: Option<bool>, // default is false
    /// Requires 'rodio_audio: true'
    ///
    /// Automatically increase or decrease you microphone's volume. This affects how
    /// loud you sound to others.
    ///
    /// Recommended: off (default)
    /// Microphones are too quite in zed, until everyone is on experimental
    /// audio and has auto speaker volume on this will make you very loud
    /// compared to other speakers.
    #[serde(rename = "experimental.auto_microphone_volume")]
    pub auto_microphone_volume: Option<bool>,
    /// Requires 'rodio_audio: true'
    ///
    /// Automatically increate or decrease the volume of other call members.
    /// This only affects how things sound for you.
    #[serde(rename = "experimental.auto_speaker_volume")]
    pub auto_speaker_volume: Option<bool>,
    /// Requires 'rodio_audio: true'
    ///
    /// Remove background noises. Works great for typing, cars, dogs, AC. Does
    /// not work well on music.
    #[serde(rename = "experimental.denoise")]
    pub denoise: Option<bool>,
    /// Requires 'rodio_audio: true'
    ///
    /// Use audio parameters compatible with the previous versions of
    /// experimental audio and non-experimental audio. When this is false you
    /// will sound strange to anyone not on the latest experimental audio. In
    /// the future we will migrate by setting this to false
    ///
    /// You need to rejoin a call for this setting to apply
    #[serde(rename = "experimental.legacy_audio_compatible")]
    pub legacy_audio_compatible: Option<bool>,
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
#[serde(rename_all = "snake_case")]
pub enum DockPosition {
    Left,
    Bottom,
    Right,
}

/// Settings for slash commands.
#[with_fallible_options]
#[derive(Deserialize, Serialize, Debug, Default, Clone, JsonSchema, MergeFrom, PartialEq, Eq)]
pub struct SlashCommandSettings {
    /// Settings for the `/cargo-workspace` slash command.
    pub cargo_workspace: Option<CargoWorkspaceCommandSettings>,
}

/// Settings for the `/cargo-workspace` slash command.
#[with_fallible_options]
#[derive(Deserialize, Serialize, Debug, Default, Clone, JsonSchema, MergeFrom, PartialEq, Eq)]
pub struct CargoWorkspaceCommandSettings {
    /// Whether `/cargo-workspace` is enabled.
    pub enabled: Option<bool>,
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
#[serde(rename_all = "snake_case")]
pub enum StatusStyle {
    #[default]
    Icon,
    LabelColor,
}

#[with_fallible_options]
#[derive(
    Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq,
)]
pub struct ScrollbarSettings {
    pub show: Option<ShowScrollbar>,
}

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
    /// Default: 300
    #[serde(serialize_with = "crate::serialize_optional_f32_with_two_decimal_places")]
    pub default_width: Option<f32>,
}

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

#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct MessageEditorSettings {
    /// Whether to automatically replace emoji shortcodes with emoji characters.
    /// For example: typing `:wave:` gets replaced with `👋`.
    ///
    /// Default: false
    pub auto_replace_emoji_shortcode: Option<bool>,
}

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
    /// Determines whether to show the git status in the file finder
    ///
    /// Default: true
    pub git_status: Option<bool>,
    /// Whether to use gitignored files when searching.
    /// Only the file Zed had indexed will be used, not necessary all the gitignored files.
    ///
    /// Default: Smart
    pub include_ignored: Option<IncludeIgnoredContent>,
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
#[serde(rename_all = "lowercase")]
pub enum FileFinderWidthContent {
    #[default]
    Small,
    Medium,
    Large,
    XLarge,
    Full,
}

#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug, JsonSchema, MergeFrom)]
pub struct VimSettingsContent {
    pub default_mode: Option<ModeContent>,
    pub toggle_relative_line_numbers: Option<bool>,
    pub use_system_clipboard: Option<UseSystemClipboard>,
    pub use_smartcase_find: Option<bool>,
    pub custom_digraphs: Option<HashMap<String, Arc<str>>>,
    pub highlight_on_yank_duration: Option<u64>,
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
#[serde(rename_all = "snake_case")]
pub enum ModeContent {
    #[default]
    Normal,
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

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HourFormat {
    #[default]
    Hour12,
    Hour24,
}

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
#[serde(rename_all = "snake_case")]
pub enum DockSide {
    Left,
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
#[serde(rename_all = "snake_case")]
pub enum ShowIndentGuides {
    Always,
    Never,
}

#[with_fallible_options]
#[derive(
    Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq, Default,
)]
pub struct IndentGuidesSettingsContent {
    /// When to show the scrollbar in the outline panel.
    pub show: Option<ShowIndentGuides>,
}

#[derive(Clone, Copy, Default, PartialEq, Debug, JsonSchema, MergeFrom, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LineIndicatorFormat {
    Short,
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

#[with_fallible_options]
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct RemoteSettingsContent {
    pub ssh_connections: Option<Vec<SshConnection>>,
    pub wsl_connections: Option<Vec<WslConnection>>,
    pub dev_container_connections: Option<Vec<DevContainerConnection>>,
    pub read_ssh_config: Option<bool>,
}

#[with_fallible_options]
#[derive(
    Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom, Hash,
)]
pub struct DevContainerConnection {
    pub name: SharedString,
    pub container_id: SharedString,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct SshConnection {
    pub host: SharedString,
    pub username: Option<String>,
    pub port: Option<u16>,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub projects: collections::BTreeSet<RemoteProject>,
    /// Name to use for this server in UI.
    pub nickname: Option<String>,
    // By default Zed will download the binary to the host directly.
    // If this is set to true, Zed will download the binary to your local machine,
    // and then upload it over the SSH connection. Useful if your SSH server has
    // limited outbound internet access.
    pub upload_binary_over_ssh: Option<bool>,

    pub port_forwards: Option<Vec<SshPortForwardOption>>,
    /// Timeout in seconds for SSH connection and downloading the remote server binary.
    /// Defaults to 10 seconds if not specified.
    pub connection_timeout: Option<u16>,
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom, Debug)]
pub struct WslConnection {
    pub distro_name: SharedString,
    pub user: Option<String>,
    #[serde(default)]
    pub projects: BTreeSet<RemoteProject>,
}

#[with_fallible_options]
#[derive(
    Clone, Debug, Default, Serialize, PartialEq, Eq, PartialOrd, Ord, Deserialize, JsonSchema,
)]
pub struct RemoteProject {
    pub paths: Vec<String>,
}

#[with_fallible_options]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, JsonSchema, MergeFrom)]
pub struct SshPortForwardOption {
    pub local_host: Option<String>,
    pub local_port: u16,
    pub remote_host: Option<String>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
/// An ExtendingVec in the settings can only accumulate new values.
///
/// This is useful for things like private files where you only want
/// to allow new values to be added.
///
/// Consider using a HashMap<String, bool> instead of this type
/// (like auto_install_extensions) so that user settings files can both add
/// and remove values from the set.
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
