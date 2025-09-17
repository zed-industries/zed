use std::num::NonZeroUsize;

use crate::DockPosition;
use anyhow::Result;
use collections::HashMap;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};

#[derive(Deserialize)]
pub struct WorkspaceSettings {
    pub active_pane_modifiers: ActivePanelModifiers,
    pub bottom_dock_layout: BottomDockLayout,
    pub pane_split_direction_horizontal: PaneSplitDirectionHorizontal,
    pub pane_split_direction_vertical: PaneSplitDirectionVertical,
    pub centered_layout: CenteredLayoutSettings,
    pub confirm_quit: bool,
    pub show_call_status_icon: bool,
    pub autosave: AutosaveSetting,
    pub restore_on_startup: RestoreOnStartupBehavior,
    pub restore_on_file_reopen: bool,
    pub drop_target_size: f32,
    pub use_system_path_prompts: bool,
    pub use_system_prompts: bool,
    pub command_aliases: HashMap<String, String>,
    pub max_tabs: Option<NonZeroUsize>,
    pub when_closing_with_no_tabs: CloseWindowWhenNoItems,
    pub on_last_window_closed: OnLastWindowClosed,
    pub resize_all_panels_in_dock: Vec<DockPosition>,
    pub close_on_file_delete: bool,
    pub use_system_window_tabs: bool,
    pub zoomed_padding: bool,
}

#[derive(Copy, Clone, Default, Serialize, Deserialize, JsonSchema)]
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

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
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

#[derive(Copy, Clone, Default, Serialize, Deserialize, JsonSchema)]
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

#[derive(Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
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

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, SettingsUi, SettingsKey)]
#[settings_key(None)]
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
    pub command_aliases: Option<HashMap<String, String>>,
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
    /// Whether to show padding for zoomed panels.
    /// When enabled, zoomed bottom panels will have some top padding,
    /// while zoomed left/right panels will have padding to the right/left (respectively).
    ///
    /// Default: true
    pub zoomed_padding: Option<bool>,
}

#[derive(Deserialize)]
pub struct TabBarSettings {
    pub show: bool,
    pub show_nav_history_buttons: bool,
    pub show_tab_bar_buttons: bool,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, SettingsUi, SettingsKey)]
#[settings_key(key = "tab_bar")]
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

impl AutosaveSetting {
    pub fn should_save_on_close(&self) -> bool {
        matches!(
            &self,
            AutosaveSetting::OnFocusChange
                | AutosaveSetting::OnWindowChange
                | AutosaveSetting::AfterDelay { .. }
        )
    }
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, SettingsUi)]
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

impl Settings for WorkspaceSettings {
    type FileContent = WorkspaceSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut Self::FileContent) {
        if vscode
            .read_bool("accessibility.dimUnfocused.enabled")
            .unwrap_or_default()
            && let Some(opacity) = vscode
                .read_value("accessibility.dimUnfocused.opacity")
                .and_then(|v| v.as_f64())
        {
            if let Some(settings) = current.active_pane_modifiers.as_mut() {
                settings.inactive_opacity = Some(opacity as f32)
            } else {
                current.active_pane_modifiers = Some(ActivePanelModifiers {
                    inactive_opacity: Some(opacity as f32),
                    ..Default::default()
                })
            }
        }

        vscode.enum_setting(
            "window.confirmBeforeClose",
            &mut current.confirm_quit,
            |s| match s {
                "always" | "keyboardOnly" => Some(true),
                "never" => Some(false),
                _ => None,
            },
        );

        vscode.bool_setting(
            "workbench.editor.restoreViewState",
            &mut current.restore_on_file_reopen,
        );

        if let Some(b) = vscode.read_bool("window.closeWhenEmpty") {
            current.when_closing_with_no_tabs = Some(if b {
                CloseWindowWhenNoItems::CloseWindow
            } else {
                CloseWindowWhenNoItems::KeepWindowOpen
            })
        }

        if let Some(b) = vscode.read_bool("files.simpleDialog.enable") {
            current.use_system_path_prompts = Some(!b);
        }

        vscode.enum_setting("files.autoSave", &mut current.autosave, |s| match s {
            "off" => Some(AutosaveSetting::Off),
            "afterDelay" => Some(AutosaveSetting::AfterDelay {
                milliseconds: vscode
                    .read_value("files.autoSaveDelay")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1000),
            }),
            "onFocusChange" => Some(AutosaveSetting::OnFocusChange),
            "onWindowChange" => Some(AutosaveSetting::OnWindowChange),
            _ => None,
        });

        // workbench.editor.limit contains "enabled", "value", and "perEditorGroup"
        // our semantics match if those are set to true, some N, and true respectively.
        // we'll ignore "perEditorGroup" for now since we only support a global max
        if let Some(n) = vscode
            .read_value("workbench.editor.limit.value")
            .and_then(|v| v.as_u64())
            .and_then(|n| NonZeroUsize::new(n as usize))
            && vscode
                .read_bool("workbench.editor.limit.enabled")
                .unwrap_or_default()
        {
            current.max_tabs = Some(n)
        }

        vscode.bool_setting("window.nativeTabs", &mut current.use_system_window_tabs);

        // some combination of "window.restoreWindows" and "workbench.startupEditor" might
        // map to our "restore_on_startup"

        // there doesn't seem to be a way to read whether the bottom dock's "justified"
        // setting is enabled in vscode. that'd be our equivalent to "bottom_dock_layout"
    }
}

impl Settings for TabBarSettings {
    type FileContent = TabBarSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut Self::FileContent) {
        vscode.enum_setting(
            "workbench.editor.showTabs",
            &mut current.show,
            |s| match s {
                "multiple" => Some(true),
                "single" | "none" => Some(false),
                _ => None,
            },
        );
        if Some("hidden") == vscode.read_string("workbench.editor.editorActionsLocation") {
            current.show_tab_bar_buttons = Some(false)
        }
    }
}
