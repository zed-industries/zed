use std::num::NonZeroUsize;

use collections::HashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct WorkspaceSettingsContent {
    /// Active pane styling settings.
    pub active_pane_modifiers: Option<ActivePanelModifiers>,
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
    /// Values: none, last_workspace, last_session
    /// Default: last_session
    pub restore_on_startup: Option<RestoreOnStartupBehavior>,
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
    pub drop_target_size: Option<f32>,
    /// Whether to close the window when using 'close active item' on a workspace with no tabs
    ///
    /// Default: auto ("on" on macOS, "off" otherwise)
    pub when_closing_with_no_tabs: Option<CloseWindowWhenNoItems>,
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
    /// Default: true
    #[serde(default)]
    pub command_aliases: HashMap<String, String>,
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
    #[serde(default)]
    pub resize_all_panels_in_dock: Vec<DockPosition>,
    /// Whether to automatically close files that have been deleted on disk.
    ///
    /// Default: false
    pub close_on_file_delete: Option<bool>,
    /// Whether to allow windows to tab together based on the user’s tabbing preference (macOS only).
    ///
    /// Default: false
    pub use_system_window_tabs: Option<bool>,
    /// Whether to show padding for zoomed panels.
    /// When enabled, zoomed bottom panels will have some top padding,
    /// while zoomed left/right panels will have padding to the right/left (respectively).
    ///
    /// Default: true
    pub zoomed_padding: Option<bool>,

    // Settings related to the editor's tab bar.
    pub tab_bar: Option<TabBarSettingsContent>,

    pub tabs: Option<ItemSettingsContent>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PreviewTabsSettingsContent {
    /// Whether to show opened editors as preview tabs.
    /// Preview tabs do not stay open, are reused until explicitly set to be kept open opened (via double-click or editing) and show file names in italic.
    ///
    /// Default: true
    pub enabled: Option<bool>,
    /// Whether to open tabs in preview mode when selected from the file finder.
    ///
    /// Default: false
    pub enable_preview_from_file_finder: Option<bool>,
    /// Whether a preview tab gets replaced when code navigation is used to navigate away from the tab.
    ///
    /// Default: false
    pub enable_preview_from_code_navigation: Option<bool>,
}

#[derive(Copy, Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ClosePosition {
    Left,
    #[default]
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ShowCloseButton {
    Always,
    #[default]
    Hover,
    Hidden,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ShowDiagnostics {
    #[default]
    Off,
    Errors,
    All,
}

#[derive(Copy, Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActivateOnClose {
    #[default]
    History,
    Neighbour,
    LeftNeighbour,
}

#[derive(Copy, Clone, PartialEq, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ActivePanelModifiers {
    /// Size of the border surrounding the active pane.
    /// When set to 0, the active pane doesn't have any border.
    /// The border is drawn inset.
    ///
    /// Default: `0.0`
    pub border_size: Option<f32>,
    /// Opacity of inactive panels.
    /// When set to 1.0, the inactive panes have the same opacity as the active one.
    /// If set to 0, the inactive panes content will not be visible at all.
    /// Values are clamped to the [0.0, 1.0] range.
    ///
    /// Default: `1.0`
    pub inactive_opacity: Option<f32>,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq, JsonSchema)]
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

#[derive(Copy, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, Debug)]
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
    pub fn should_close(&self) -> bool {
        match self {
            CloseWindowWhenNoItems::PlatformDefault => cfg!(target_os = "macos"),
            CloseWindowWhenNoItems::CloseWindow => true,
            CloseWindowWhenNoItems::KeepWindowOpen => false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RestoreOnStartupBehavior {
    /// Always start with an empty editor
    None,
    /// Restore the workspace that was closed last.
    LastWorkspace,
    /// Restore all workspaces that were open when quitting Zed.
    #[default]
    LastSession,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug, PartialEq)]
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
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AutosaveSetting {
    /// Disable autosave.
    Off,
    /// Save after inactivity period of `milliseconds`.
    AfterDelay { milliseconds: u64 },
    /// Autosave when focus changes.
    OnFocusChange,
    /// Autosave when the active window changes.
    OnWindowChange,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PaneSplitDirectionHorizontal {
    Up,
    Down,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PaneSplitDirectionVertical {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct CenteredLayoutSettings {
    /// The relative width of the left padding of the central pane from the
    /// workspace when the centered layout is used.
    ///
    /// Default: 0.2
    pub left_padding: Option<f32>,
    // The relative width of the right padding of the central pane from the
    // workspace when the centered layout is used.
    ///
    /// Default: 0.2
    pub right_padding: Option<f32>,
}

#[derive(Copy, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OnLastWindowClosed {
    /// Match platform conventions by default, so don't quit on macOS, and quit on other platforms
    #[default]
    PlatformDefault,
    /// Quit the application the last window is closed
    QuitApp,
}

impl OnLastWindowClosed {
    pub fn is_quit_app(&self) -> bool {
        match self {
            OnLastWindowClosed::PlatformDefault => false,
            OnLastWindowClosed::QuitApp => true,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DockPosition {
    Left,
    Bottom,
    Right,
}
