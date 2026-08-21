use std::num::NonZeroUsize;

use collections::HashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

use crate::{
    CenteredPaddingSettings, CommandAliasTarget, DelayMs, DockPosition, DockSide, InactiveOpacity,
    ShowIndentGuides, ShowScrollbar, serialize_optional_f32_with_two_decimal_places,
};

/// Settings related to the workspace.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct WorkspaceSettingsContent {
    /// Active pane styling settings.
    pub active_pane_modifiers: Option<ActivePaneModifiers>,
    /// The text rendering mode to use.
    ///
    /// Default: platform_default
    pub text_rendering_mode: Option<TextRenderingMode>,
    /// Layout mode for the bottom dock
    ///
    /// Default: contained
    pub bottom_dock_layout: Option<BottomDockLayout>,
    /// Direction to split horizontally.
    ///
    /// Default: "up"
    pub pane_split_direction_horizontal: Option<PaneSplitDirectionHorizontal>,
    /// Direction to split vertically.
    ///
    /// Default: "left"
    pub pane_split_direction_vertical: Option<PaneSplitDirectionVertical>,
    /// Centered layout related settings.
    pub centered_layout: Option<CenteredLayoutSettings>,
    /// Whether or not to prompt the user to confirm before closing the application.
    ///
    /// Default: false
    pub confirm_quit: Option<bool>,
    /// Whether or not to show the call status icon in the status bar.
    ///
    /// Default: true
    pub show_call_status_icon: Option<bool>,
    /// When to automatically save edited buffers.
    ///
    /// Default: off
    pub autosave: Option<AutosaveSetting>,
    /// Controls previous session restoration in freshly launched Zed instance.
    /// Values: empty_tab, last_workspace, last_session, launchpad
    /// Default: last_session
    pub restore_on_startup: Option<RestoreOnStartupBehavior>,
    /// The default behavior when opening paths from the CLI without
    /// an explicit `-e` or `-n` flag.
    ///
    /// Default: existing_window
    pub cli_default_open_behavior: Option<CliDefaultOpenBehavior>,
    /// The default behavior when opening projects from the UI.
    ///
    /// Default: existing_window
    pub default_open_behavior: Option<DefaultOpenBehavior>,
    /// Whether to attempt to restore previous file's state when opening it again.
    /// The state is stored per pane.
    /// When disabled, defaults are applied instead of the state restoration.
    ///
    /// E.g. for editors, selections, folds and scroll positions are restored, if the same file is closed and, later, opened again in the same pane.
    /// When disabled, a single selection in the very beginning of the file, zero scroll position and no folds state is used as a default.
    ///
    /// Default: true
    pub restore_on_file_reopen: Option<bool>,
    /// The size of the workspace split drop targets on the outer edges.
    /// Given as a fraction that will be multiplied by the smaller dimension of the workspace.
    ///
    /// Default: `0.2` (20% of the smaller dimension of the workspace)
    #[serde(serialize_with = "serialize_optional_f32_with_two_decimal_places")]
    pub drop_target_size: Option<f32>,
    /// Whether to close the window when using 'close active item' on a workspace with no tabs
    ///
    /// Default: auto ("on" on macOS, "off" otherwise)
    pub when_closing_with_no_tabs: Option<CloseWindowWhenNoItems>,
    /// Whether to optimize Zed's interface for assistive technology such as
    /// screen readers.
    ///
    /// Default: false
    pub accessible_mode: Option<bool>,
    /// Whether to use the system provided dialogs for Open and Save As.
    /// When set to false, Zed will use the built-in keyboard-first pickers.
    ///
    /// Default: true
    pub use_system_path_prompts: Option<bool>,
    /// Whether to use the system provided prompts.
    /// When set to false, Zed will use the built-in prompts.
    /// Note that this setting has no effect on Linux, where Zed will always
    /// use the built-in prompts.
    ///
    /// Default: true
    pub use_system_prompts: Option<bool>,
    /// Aliases for the command palette. When you type a key in this map,
    /// it will be assumed to equal the value.
    ///
    /// Default: {}
    #[serde(default)]
    pub command_aliases: HashMap<String, CommandAliasTarget>,
    /// Maximum open tabs in a pane. Will not close an unsaved
    /// tab. Set to `None` for unlimited tabs.
    ///
    /// Default: none
    pub max_tabs: Option<NonZeroUsize>,
    /// What to do when the last window is closed
    ///
    /// Default: auto (nothing on macOS, "app quit" otherwise)
    pub on_last_window_closed: Option<OnLastWindowClosed>,
    /// Whether to resize all the panels in a dock when resizing the dock.
    ///
    /// Default: ["left"]
    pub resize_all_panels_in_dock: Option<Vec<DockPosition>>,
    /// Whether to automatically close files that have been deleted on disk.
    ///
    /// Default: false
    pub close_on_file_delete: Option<bool>,
    /// Whether to allow windows to tab together based on the user’s tabbing preference (macOS only).
    ///
    /// Default: false
    pub use_system_window_tabs: Option<bool>,
    /// Which fullscreen mode the `zed::ToggleFullScreen` action enters (macOS only).
    ///
    /// Default: native
    pub fullscreen_mode: Option<FullscreenMode>,
    /// Whether to show padding for zoomed panels.
    /// When enabled, zoomed bottom panels will have some top padding,
    /// while zoomed left/right panels will have padding to the right/left (respectively).
    ///
    /// Default: true
    pub zoomed_padding: Option<bool>,
    /// Whether toggling a panel (e.g. with its keyboard shortcut) also closes
    /// the panel when it is already focused, instead of just moving focus back
    /// to the editor.
    ///
    /// Default: false
    pub close_panel_on_toggle: Option<bool>,
    /// Controls whether Zed or the window manager or compositor draws window decorations on Linux.
    ///
    /// Default: client
    pub window_decorations: Option<WindowDecorations>,
    /// Whether the focused panel follows the mouse location
    /// Default: false
    pub focus_follows_mouse: Option<FocusFollowsMouse>,
}

/// Configuration for the editor tabs.
#[with_fallible_options]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ItemSettingsContent {
    /// Whether to show the Git file status on a tab item.
    ///
    /// Default: false
    pub git_status: Option<bool>,
    /// Position of the close button in a tab.
    ///
    /// Default: right
    pub close_position: Option<ClosePosition>,
    /// Whether to show the file icon for a tab.
    ///
    /// Default: false
    pub file_icons: Option<bool>,
    /// What to do after closing the current tab.
    ///
    /// Default: history
    pub activate_on_close: Option<ActivateOnClose>,
    /// Which files containing diagnostic errors/warnings to mark in the tabs.
    /// This setting can take the following three values:
    ///
    /// Default: off
    pub show_diagnostics: Option<ShowDiagnostics>,
    /// Whether to always show the close button on tabs.
    ///
    /// Default: false
    pub show_close_button: Option<ShowCloseButton>,
}

/// Settings related to preview tabs.
#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct PreviewTabsSettingsContent {
    /// Whether to show opened editors as preview tabs.
    /// Preview tabs do not stay open, are reused until explicitly set to be kept open opened (via double-click or editing) and show file names in italic.
    ///
    /// Default: true
    pub enabled: Option<bool>,
    /// Whether to open tabs in preview mode when opened from the project panel with a single click.
    ///
    /// Default: true
    pub enable_preview_from_project_panel: Option<bool>,
    /// Whether to open tabs in preview mode when selected from the file finder.
    ///
    /// Default: false
    pub enable_preview_from_file_finder: Option<bool>,
    /// Whether to open tabs in preview mode when opened from a multibuffer.
    ///
    /// Default: true
    pub enable_preview_from_multibuffer: Option<bool>,
    /// Whether to open tabs in preview mode when code navigation is used to open a multibuffer.
    ///
    /// Default: false
    pub enable_preview_multibuffer_from_code_navigation: Option<bool>,
    /// Whether to open tabs in preview mode when code navigation is used to open a single file.
    ///
    /// Default: true
    pub enable_preview_file_from_code_navigation: Option<bool>,
    /// Whether to keep tabs in preview mode when code navigation is used to navigate away from them.
    /// If `enable_preview_file_from_code_navigation` or `enable_preview_multibuffer_from_code_navigation` is also true, the new tab may replace the existing one.
    ///
    /// Default: false
    pub enable_keep_preview_on_code_navigation: Option<bool>,
}

/// Where to display close button within a tab.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
pub enum ClosePosition {
    /// Display the close button on the left.
    Left,
    /// Display the close button on the right.
    #[default]
    Right,
}

/// Controls the appearance behavior of the tab's close button.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
pub enum ShowCloseButton {
    /// Show it persistently.
    Always,
    /// Show it just upon hovering the tab.
    #[default]
    Hover,
    /// Never show it, even if hovering it.
    Hidden,
}

/// Whether to show diagnostics indicators in tabs. This setting only works
/// when file icons are active and controls which files with diagnostic issues to mark.
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
pub enum ShowDiagnostics {
    /// Do not mark any files.
    #[default]
    Off,
    /// Only mark files with errors.
    Errors,
    /// Mark files with errors and warnings.
    All,
}

/// What to do after closing the current tab.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum ActivateOnClose {
    /// Activate the tab that was open previously.
    #[default]
    History,
    /// Activate the right neighbour tab if present.
    Neighbour,
    /// Activate the left neighbour tab if present.
    LeftNeighbour,
}

/// Styling settings applied to the active pane.
#[with_fallible_options]
#[derive(Copy, Clone, PartialEq, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub struct ActivePaneModifiers {
    /// Size of the border surrounding the active pane.
    /// When set to 0, the active pane doesn't have any border.
    /// The border is drawn inset.
    ///
    /// Default: `0.0`
    pub border_size: Option<crate::PixelSetting>,
    /// Opacity of inactive panels.
    /// When set to 1.0, the inactive panes have the same opacity as the active one.
    /// If set to 0, the inactive panes content will not be visible at all.
    /// Values are clamped to the [0.0, 1.0] range.
    ///
    /// Default: `1.0`
    #[schemars(range(min = 0.0, max = 1.0))]
    pub inactive_opacity: Option<InactiveOpacity>,
}

/// Control the layout of the bottom dock, relative to the left and right docks.
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum BottomDockLayout {
    /// Contained between the left and right docks
    #[default]
    Contained,
    /// Takes up the full width of the window
    Full,
    /// Extends under the left dock while snapping to the right dock
    LeftAligned,
    /// Extends under the right dock while snapping to the left dock
    RightAligned,
}

/// Which fullscreen mode the `zed::ToggleFullScreen` action enters (macOS only).
#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum FullscreenMode {
    /// Use macOS's native fullscreen, which moves the window into its own
    /// Mission Control space.
    #[default]
    Native,
    /// Resize the window to cover the entire screen, including the menu bar and,
    /// on notched displays, the area around the notch.
    Simple,
}

/// Configures what draws Zed's window decorations on Linux.
/// This setting has no effect on other platforms.
#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum WindowDecorations {
    /// Zed draws its own window decorations/titlebar (client-side decoration).
    #[default]
    Client,
    /// The window manager or compositor draws the server-side window
    /// decorations (not supported by GNOME Wayland).
    Server,
}

/// Whether to close the window when using 'close active item' on a workspace with no tabs.
#[derive(
    Copy,
    Clone,
    PartialEq,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    Debug,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum CloseWindowWhenNoItems {
    /// Match platform conventions by default, so "on" on macOS and "off" everywhere else
    #[default]
    PlatformDefault,
    /// Close the window when there are no tabs
    CloseWindow,
    /// Leave the window open when there are no tabs
    KeepWindowOpen,
}

impl CloseWindowWhenNoItems {
    /// Returns true if the window should close when it contains no tabs.
    pub fn should_close(&self) -> bool {
        match self {
            CloseWindowWhenNoItems::PlatformDefault => cfg!(target_os = "macos"),
            CloseWindowWhenNoItems::CloseWindow => true,
            CloseWindowWhenNoItems::KeepWindowOpen => false,
        }
    }
}

/// The default behavior when opening paths from the CLI without an explicit `-e` or `-n` flag.
#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    Debug,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum CliDefaultOpenBehavior {
    /// Open directories as a new workspace in the current Zed window's sidebar.
    #[default]
    #[strum(serialize = "Add to Existing Window")]
    ExistingWindow,
    /// Open paths in a new window unless they are subpaths of an existing project.
    #[strum(serialize = "Open a New Window")]
    NewWindow,
}

/// The default behavior when opening projects from the UI.
#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    Debug,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum DefaultOpenBehavior {
    /// Open projects in the current Zed window.
    #[default]
    #[strum(serialize = "Add to Existing Window")]
    ExistingWindow,
    /// Open projects in a new window.
    #[strum(serialize = "Open a New Window")]
    NewWindow,
}

/// Controls session restoration on startup.
#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    Debug,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum RestoreOnStartupBehavior {
    /// Always start with an empty editor tab
    #[serde(alias = "none")]
    EmptyTab,
    /// Restore the workspace that was closed last.
    LastWorkspace,
    /// Restore all workspaces that were open when quitting Zed.
    #[default]
    LastSession,
    /// Show the launchpad with recent projects (no tabs).
    Launchpad,
}

/// Settings related to the editor's tab bar.
#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct TabBarSettingsContent {
    /// Whether or not to show the tab bar in the editor.
    ///
    /// Default: true
    pub show: Option<bool>,
    /// Whether or not to show the navigation history buttons in the tab bar.
    ///
    /// Default: true
    pub show_nav_history_buttons: Option<bool>,
    /// Whether or not to show the tab bar buttons.
    ///
    /// Default: true
    pub show_tab_bar_buttons: Option<bool>,
    /// Whether or not to show pinned tabs in a separate row.
    /// When enabled, pinned tabs appear in a top row and unpinned tabs in a bottom row.
    ///
    /// Default: false
    pub show_pinned_tabs_in_separate_row: Option<bool>,
}

/// Control various elements in the status bar.
/// Note that some items in the status bar have their own settings set elsewhere.
#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq, Eq)]
pub struct StatusBarSettingsContent {
    /// Whether to show the status bar.
    ///
    /// Default: true
    #[serde(rename = "experimental.show")]
    pub show: Option<bool>,
    /// Whether to show the name of the active file in the status bar.
    ///
    /// Default: false
    pub show_active_file: Option<bool>,
    /// Whether to display the active language button in the status bar.
    ///
    /// Default: true
    pub active_language_button: Option<bool>,
    /// Whether to show the cursor position button in the status bar.
    ///
    /// Default: true
    pub cursor_position_button: Option<bool>,
    /// Whether to show active line endings button in the status bar.
    ///
    /// Default: false
    pub line_endings_button: Option<bool>,
    /// Whether to show the active encoding button in the status bar.
    ///
    /// Default: non_utf8
    pub active_encoding_button: Option<EncodingDisplayOptions>,
}

/// Control when to show the active encoding in the status bar.
#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantNames,
    strum::VariantArray,
)]
#[serde(rename_all = "snake_case")]
pub enum EncodingDisplayOptions {
    /// Always show the active encoding.
    Enabled,
    /// Never show the active encoding.
    Disabled,
    /// Show the active encoding only when it is not UTF-8 without a BOM.
    #[default]
    NonUtf8,
}
impl EncodingDisplayOptions {
    /// Returns true if the encoding button should be shown for a file with the given encoding properties.
    pub fn should_show(&self, is_utf8: bool, has_bom: bool) -> bool {
        match self {
            Self::Disabled => false,
            Self::Enabled => true,
            Self::NonUtf8 => {
                let is_standard_utf8 = is_utf8 && !has_bom;
                !is_standard_utf8
            }
        }
    }
}

/// When to automatically save edited buffers.
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
    strum::EnumDiscriminants,
)]
#[strum_discriminants(derive(strum::VariantArray, strum::VariantNames, strum::FromRepr))]
#[strum_discriminants(doc = "The kind of an autosave setting.")]
#[serde(rename_all = "snake_case")]
pub enum AutosaveSetting {
    /// Disable autosave.
    Off,
    /// Save after inactivity period of `milliseconds`.
    AfterDelay {
        /// The inactivity period in milliseconds.
        milliseconds: DelayMs,
    },
    /// Autosave when focus changes.
    OnFocusChange,
    /// Autosave when the active window changes.
    OnWindowChange,
}

impl AutosaveSetting {
    /// Returns true if the buffer should be saved when it is closed.
    pub fn should_save_on_close(&self) -> bool {
        matches!(
            &self,
            AutosaveSetting::OnFocusChange
                | AutosaveSetting::OnWindowChange
                | AutosaveSetting::AfterDelay { .. }
        )
    }
}

/// The direction that you want to split panes horizontally.
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
pub enum PaneSplitDirectionHorizontal {
    /// Split up.
    Up,
    /// Split down.
    Down,
}

/// The direction that you want to split panes vertically.
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
pub enum PaneSplitDirectionVertical {
    /// Split left.
    Left,
    /// Split right.
    Right,
}

/// Configuration for the centered layout mode.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
#[with_fallible_options]
pub struct CenteredLayoutSettings {
    /// The relative width of the left padding of the central pane from the
    /// workspace when the centered layout is used.
    ///
    /// Default: 0.2
    pub left_padding: Option<CenteredPaddingSettings>,
    /// The relative width of the right padding of the central pane from the
    /// workspace when the centered layout is used.
    ///
    /// Default: 0.2
    pub right_padding: Option<CenteredPaddingSettings>,
}

/// What to do when the last window is closed.
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
pub enum OnLastWindowClosed {
    /// Match platform conventions by default, so don't quit on macOS, and quit on other platforms
    #[default]
    PlatformDefault,
    /// Quit the application the last window is closed
    QuitApp,
}

/// The text rendering mode to use.
#[derive(
    Copy,
    Clone,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    PartialEq,
    Eq,
    Debug,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum TextRenderingMode {
    /// Use platform default behavior.
    #[default]
    PlatformDefault,
    /// Use subpixel (ClearType-style) text rendering.
    Subpixel,
    /// Use grayscale text rendering.
    Grayscale,
}

impl OnLastWindowClosed {
    /// Returns true if the application should quit when the last window is closed.
    pub fn is_quit_app(&self) -> bool {
        match self {
            OnLastWindowClosed::PlatformDefault => false,
            OnLastWindowClosed::QuitApp => true,
        }
    }
}

/// Settings for automatically opening files from the project panel.
#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct ProjectPanelAutoOpenSettings {
    /// Whether to automatically open newly created files in the editor.
    ///
    /// Default: true
    pub on_create: Option<bool>,
    /// Whether to automatically open files after pasting or duplicating them.
    ///
    /// Default: true
    pub on_paste: Option<bool>,
    /// Whether to automatically open files dropped from external sources.
    ///
    /// Default: true
    pub on_drop: Option<bool>,
}

/// Settings related to the project panel.
#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct ProjectPanelSettingsContent {
    /// Whether to show the project panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Whether to hide gitignore files in the project panel.
    ///
    /// Default: false
    pub hide_gitignore: Option<bool>,
    /// Customize default width (in pixels) taken by project panel
    ///
    /// Default: 240
    pub default_width: Option<crate::PixelSetting>,
    /// The position of project panel
    ///
    /// Default: right (Agentic layout), left (Classic layout)
    pub dock: Option<DockSide>,
    /// Spacing between worktree entries in the project panel.
    ///
    /// Default: comfortable
    pub entry_spacing: Option<ProjectPanelEntrySpacing>,
    /// Whether to show file icons in the project panel.
    ///
    /// Default: true
    pub file_icons: Option<bool>,
    /// Whether to show folder icons or chevrons for directories in the project panel.
    ///
    /// Default: true
    pub folder_icons: Option<bool>,
    /// Whether to show the git status in the project panel.
    ///
    /// Default: true
    pub git_status: Option<bool>,
    /// Amount of indentation (in pixels) for nested items.
    ///
    /// Default: 20
    pub indent_size: Option<crate::PixelSetting>,
    /// Whether to reveal it in the project panel automatically,
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
    /// Whether to show folder names with bold text in the project panel.
    ///
    /// Default: false
    pub bold_folder_labels: Option<bool>,
    /// Whether the project panel should open on startup.
    ///
    /// Default: true
    pub starts_open: Option<bool>,
    /// Scrollbar-related settings
    pub scrollbar: Option<ProjectPanelScrollbarSettingsContent>,
    /// Which files containing diagnostic errors/warnings to mark in the project panel.
    ///
    /// Default: all
    pub show_diagnostics: Option<ShowDiagnostics>,
    /// Settings related to indent guides in the project panel.
    pub indent_guides: Option<ProjectPanelIndentGuidesSettings>,
    /// Whether to hide the root entry when only one folder is open in the window.
    ///
    /// Default: false
    pub hide_root: Option<bool>,
    /// Whether to hide the hidden entries in the project panel.
    ///
    /// Default: false
    pub hide_hidden: Option<bool>,
    /// Whether to stick parent directories at top of the project panel.
    ///
    /// Default: true
    pub sticky_scroll: Option<bool>,
    /// Whether to enable drag-and-drop operations in the project panel.
    ///
    /// Default: true
    pub drag_and_drop: Option<bool>,
    /// Settings for automatically opening files.
    pub auto_open: Option<ProjectPanelAutoOpenSettings>,
    /// How to order sibling entries in the project panel.
    ///
    /// Default: directories_first
    pub sort_mode: Option<ProjectPanelSortMode>,
    /// Whether to sort file and folder names case-sensitively in the project panel.
    /// This works in combination with `sort_mode`. `sort_mode` controls how files and
    /// directories are grouped, while this setting controls how names are compared.
    ///
    /// Default: default
    pub sort_order: Option<ProjectPanelSortOrder>,
    /// Whether to show error and warning count badges next to file names in the project panel.
    ///
    /// Default: false
    pub diagnostic_badges: Option<bool>,
    /// Whether to show a git status indicator next to file names in the project panel.
    ///
    /// Default: false
    pub git_status_indicator: Option<bool>,
}

/// Spacing between worktree entries in the project panel.
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
pub enum ProjectPanelEntrySpacing {
    /// Comfortable spacing of entries.
    #[default]
    Comfortable,
    /// The standard spacing of entries.
    Standard,
}

/// How to order sibling entries in the project panel.
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
pub enum ProjectPanelSortMode {
    /// Show directories first, then files
    #[default]
    DirectoriesFirst,
    /// Mix directories and files together
    Mixed,
    /// Show files first, then directories
    FilesFirst,
}

/// Whether to sort file and folder names case-sensitively in the project panel.
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
pub enum ProjectPanelSortOrder {
    /// Case-insensitive natural sort with lowercase preferred in ties.
    /// Numbers in file names are compared by value (e.g., `file2` before `file10`).
    #[default]
    Default,
    /// Uppercase names are grouped before lowercase names, with case-insensitive
    /// natural sort within each group. Dot-prefixed names sort before both groups.
    Upper,
    /// Lowercase names are grouped before uppercase names, with case-insensitive
    /// natural sort within each group. Dot-prefixed names sort before both groups.
    Lower,
    /// Pure Unicode codepoint comparison. No case folding, no natural number sorting.
    /// Uppercase ASCII sorts before lowercase. Accented characters sort after ASCII.
    Unicode,
}

impl From<ProjectPanelSortMode> for util::paths::SortMode {
    fn from(mode: ProjectPanelSortMode) -> Self {
        match mode {
            ProjectPanelSortMode::DirectoriesFirst => Self::DirectoriesFirst,
            ProjectPanelSortMode::Mixed => Self::Mixed,
            ProjectPanelSortMode::FilesFirst => Self::FilesFirst,
        }
    }
}

impl From<ProjectPanelSortOrder> for util::paths::SortOrder {
    fn from(order: ProjectPanelSortOrder) -> Self {
        match order {
            ProjectPanelSortOrder::Default => Self::Default,
            ProjectPanelSortOrder::Upper => Self::Upper,
            ProjectPanelSortOrder::Lower => Self::Lower,
            ProjectPanelSortOrder::Unicode => Self::Unicode,
        }
    }
}

/// Scrollbar-related settings for the project panel.
#[with_fallible_options]
#[derive(
    Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq, Default,
)]
pub struct ProjectPanelScrollbarSettingsContent {
    /// When to show the scrollbar in the project panel.
    ///
    /// Default: inherits editor scrollbar settings
    pub show: Option<ShowScrollbar>,
    /// Whether to allow horizontal scrolling in the project panel.
    /// When false, the view is locked to the leftmost position and
    /// long file names are clipped.
    ///
    /// Default: true
    pub horizontal_scroll: Option<bool>,
}

/// Settings related to indent guides in the project panel.
#[with_fallible_options]
#[derive(
    Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq, Default,
)]
pub struct ProjectPanelIndentGuidesSettings {
    /// When to show indent guides in the project panel.
    ///
    /// Default: always
    pub show: Option<ShowIndentGuides>,
}

/// Controls how semantic tokens from language servers are used for syntax highlighting.
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
    strum::EnumMessage,
)]
#[serde(rename_all = "snake_case")]
pub enum SemanticTokens {
    /// Do not request semantic tokens from language servers.
    #[default]
    Off,
    /// Use LSP semantic tokens together with tree-sitter highlighting.
    Combined,
    /// Use LSP semantic tokens exclusively, replacing tree-sitter highlighting.
    Full,
}

impl SemanticTokens {
    /// Returns true if semantic tokens should be requested from language servers.
    pub fn enabled(&self) -> bool {
        self != &Self::Off
    }

    /// Returns true if tree-sitter syntax highlighting should be used.
    /// In `full` mode, tree-sitter is disabled in favor of LSP semantic tokens.
    pub fn use_tree_sitter(&self) -> bool {
        self != &Self::Full
    }
}

/// Controls whether folding ranges from language servers are used instead of
/// tree-sitter and indent-based folding.
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
pub enum DocumentFoldingRanges {
    /// Do not request folding ranges from language servers; use tree-sitter and indent-based folding.
    #[default]
    Off,
    /// Use LSP folding wherever possible, falling back to tree-sitter and indent-based folding when no results were returned by the server.
    On,
}

impl DocumentFoldingRanges {
    /// Returns true if LSP folding ranges should be requested from language servers.
    pub fn enabled(&self) -> bool {
        self != &Self::Off
    }
}

/// Controls the source of document symbols used for outlines and breadcrumbs.
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
pub enum DocumentSymbols {
    /// Use tree-sitter queries to compute document symbols for outlines and breadcrumbs (default).
    #[default]
    #[serde(alias = "tree_sitter")]
    Off,
    /// Use the language server's `textDocument/documentSymbol` LSP response for outlines and
    /// breadcrumbs. When enabled, tree-sitter is not used for document symbols.
    #[serde(alias = "language_server")]
    On,
}

impl DocumentSymbols {
    /// Returns true if LSP document symbols should be used instead of tree-sitter.
    pub fn lsp_enabled(&self) -> bool {
        self == &Self::On
    }
}

/// Determines whether the focused panel follows the mouse location.
#[with_fallible_options]
#[derive(Copy, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct FocusFollowsMouse {
    /// Whether the focused panel follows the mouse location.
    ///
    /// Default: false
    pub enabled: Option<bool>,
    /// How long the mouse must stay over a panel before it is focused, in milliseconds.
    ///
    /// Default: 250
    pub debounce_ms: Option<u64>,
}

impl WorkspaceSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            active_pane_modifiers: Some(ActivePaneModifiers::defaults()),
            text_rendering_mode: Some(TextRenderingMode::PlatformDefault),
            bottom_dock_layout: Some(BottomDockLayout::Contained),
            pane_split_direction_horizontal: Some(PaneSplitDirectionHorizontal::Down),
            pane_split_direction_vertical: Some(PaneSplitDirectionVertical::Right),
            centered_layout: Some(CenteredLayoutSettings::defaults()),
            confirm_quit: Some(false),
            show_call_status_icon: Some(true),
            autosave: Some(AutosaveSetting::Off),
            restore_on_startup: Some(RestoreOnStartupBehavior::LastSession),
            cli_default_open_behavior: Some(CliDefaultOpenBehavior::ExistingWindow),
            default_open_behavior: Some(DefaultOpenBehavior::ExistingWindow),
            restore_on_file_reopen: Some(true),
            drop_target_size: Some(0.2),
            when_closing_with_no_tabs: Some(CloseWindowWhenNoItems::PlatformDefault),
            accessible_mode: Some(false),
            use_system_path_prompts: Some(true),
            use_system_prompts: Some(true),
            command_aliases: HashMap::default(),
            max_tabs: None,
            on_last_window_closed: Some(OnLastWindowClosed::PlatformDefault),
            resize_all_panels_in_dock: Some(vec![DockPosition::Left]),
            close_on_file_delete: Some(false),
            use_system_window_tabs: Some(false),
            fullscreen_mode: Some(FullscreenMode::Native),
            zoomed_padding: Some(true),
            close_panel_on_toggle: Some(false),
            window_decorations: Some(WindowDecorations::Client),
            focus_follows_mouse: Some(FocusFollowsMouse::defaults()),
        }
    }
}

impl ActivePaneModifiers {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            border_size: Some(crate::PixelSetting(0.0)),
            inactive_opacity: Some(InactiveOpacity(1.0)),
        }
    }
}

impl CenteredLayoutSettings {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            left_padding: Some(CenteredPaddingSettings(0.2)),
            right_padding: Some(CenteredPaddingSettings(0.2)),
        }
    }
}

impl FocusFollowsMouse {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            enabled: Some(false),
            debounce_ms: Some(250),
        }
    }
}

impl ItemSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            git_status: Some(false),
            close_position: Some(ClosePosition::Right),
            file_icons: Some(false),
            activate_on_close: Some(ActivateOnClose::History),
            show_diagnostics: Some(ShowDiagnostics::Off),
            show_close_button: Some(ShowCloseButton::Hover),
        }
    }
}

impl TabBarSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            show: Some(true),
            show_nav_history_buttons: Some(true),
            show_tab_bar_buttons: Some(true),
            show_pinned_tabs_in_separate_row: Some(false),
        }
    }
}

impl StatusBarSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            show: Some(true),
            show_active_file: Some(false),
            active_language_button: Some(true),
            cursor_position_button: Some(true),
            line_endings_button: Some(false),
            active_encoding_button: Some(EncodingDisplayOptions::NonUtf8),
        }
    }
}

impl PreviewTabsSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            enabled: Some(true),
            enable_preview_from_project_panel: Some(true),
            enable_preview_from_file_finder: Some(false),
            enable_preview_from_multibuffer: Some(true),
            enable_preview_multibuffer_from_code_navigation: Some(false),
            enable_preview_file_from_code_navigation: Some(true),
            enable_keep_preview_on_code_navigation: Some(false),
        }
    }
}

impl ProjectPanelSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            button: Some(true),
            hide_gitignore: Some(false),
            default_width: Some(crate::PixelSetting(240.0)),
            dock: Some(DockSide::Right),
            entry_spacing: Some(ProjectPanelEntrySpacing::Comfortable),
            file_icons: Some(true),
            folder_icons: Some(true),
            git_status: Some(true),
            indent_size: Some(crate::PixelSetting(20.0)),
            auto_reveal_entries: Some(true),
            auto_fold_dirs: Some(true),
            bold_folder_labels: Some(false),
            starts_open: Some(true),
            scrollbar: Some(ProjectPanelScrollbarSettingsContent::defaults()),
            show_diagnostics: Some(ShowDiagnostics::All),
            indent_guides: Some(ProjectPanelIndentGuidesSettings::defaults()),
            hide_root: Some(false),
            hide_hidden: Some(false),
            sticky_scroll: Some(true),
            drag_and_drop: Some(true),
            auto_open: Some(ProjectPanelAutoOpenSettings::defaults()),
            sort_mode: Some(ProjectPanelSortMode::DirectoriesFirst),
            sort_order: Some(ProjectPanelSortOrder::Default),
            diagnostic_badges: Some(false),
            git_status_indicator: Some(false),
        }
    }
}

impl ProjectPanelScrollbarSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            show: None,
            horizontal_scroll: Some(true),
        }
    }
}

impl ProjectPanelIndentGuidesSettings {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            show: Some(ShowIndentGuides::Always),
        }
    }
}

impl ProjectPanelAutoOpenSettings {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            on_create: Some(true),
            on_paste: Some(true),
            on_drop: Some(true),
        }
    }
}
