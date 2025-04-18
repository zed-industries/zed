use std::num::NonZeroUsize;

use anyhow::Result;
use collections::HashMap;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

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
    pub show_user_picture: bool,
    pub max_tabs: Option<NonZeroUsize>,
    pub when_closing_with_no_tabs: CloseWindowWhenNoItems,
    pub on_last_window_closed: OnLastWindowClosed,
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
    /// Scale by which to zoom the active pane.
    /// When set to 1.0, the active pane has the same size as others,
    /// but when set to a larger value, the active pane takes up more space.
    ///
    /// Default: `1.0`
    pub magnification: Option<f32>,
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

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
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
    /// Whether to show user avatar in the title bar.
    ///
    /// Default: true
    pub show_user_picture: Option<bool>,
    /// Maximum open tabs in a pane. Will not close an unsaved
    /// tab. Set to `None` for unlimited tabs.
    ///
    /// Default: none
    pub max_tabs: Option<NonZeroUsize>,
    /// What to do when the last window is closed
    ///
    /// Default: auto (nothing on macOS, "app quit" otherwise)
    pub on_last_window_closed: Option<OnLastWindowClosed>,
}

#[derive(Deserialize)]
pub struct TabBarSettings {
    pub show: bool,
    pub show_nav_history_buttons: bool,
    pub show_tab_bar_buttons: bool,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
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
    const KEY: Option<&'static str> = None;

    type FileContent = WorkspaceSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(vscode: &settings::VSCodeSettings, old: &mut Self::FileContent) {
        if vscode
            .read_value("accessibility.dimUnfocused.enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or_default()
        {
            if let Some(opacity) = vscode
                .read_value("accessibility.dimUnfocused.opacity")
                .and_then(|v| v.as_f64())
            {
                if let Some(settings) = old.active_pane_modifiers.as_mut() {
                    settings.inactive_opacity = Some(opacity as f32)
                } else {
                    old.active_pane_modifiers = Some(ActivePanelModifiers {
                        inactive_opacity: Some(opacity as f32),
                        ..Default::default()
                    })
                }
            }
        }

        vscode.enum_setting(
            "window.confirmBeforeClose",
            &mut old.confirm_quit,
            |s| match s {
                "always" | "keyboardOnly" => Some(true),
                "never" => Some(false),
                _ => None,
            },
        );

        vscode.bool_setting(
            "workbench.editor.restoreViewState",
            &mut old.restore_on_file_reopen,
        );

        if let Some(b) = vscode
            .read_value("window.closeWhenEmpty")
            .and_then(|v| v.as_bool())
        {
            old.when_closing_with_no_tabs = Some(if b {
                CloseWindowWhenNoItems::CloseWindow
            } else {
                CloseWindowWhenNoItems::KeepWindowOpen
            })
        }

        if let Some(b) = vscode
            .read_value("files.simpleDialog.enable")
            .and_then(|v| v.as_bool())
        {
            old.use_system_path_prompts = Some(!b);
        }

        vscode.enum_setting("files.autoSave", &mut old.autosave, |s| match s {
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
        {
            if vscode
                .read_value("workbench.editor.limit.enabled")
                .and_then(|v| v.as_bool())
                .unwrap_or_default()
            {
                old.max_tabs = Some(n)
            }
        }

        // some combination of "window.restoreWindows" and "workbench.startupEditor" might
        // map to our "restore_on_startup"

        // there doesn't seem to be a way to read whether the bottom dock's "justified"
        // setting is enabled in vscode. that'd be our equivilant to "bottom_dock_layout"
    }
}

impl Settings for TabBarSettings {
    const KEY: Option<&'static str> = Some("tab_bar");

    type FileContent = TabBarSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(vscode: &settings::VSCodeSettings, old: &mut Self::FileContent) {
        vscode.enum_setting("workbench.editor.showTabs", &mut old.show, |s| match s {
            "multiple" => Some(true),
            "single" | "none" => Some(false),
            _ => None,
        });
        if Some("hidden") == vscode.read_string("workbench.editor.editorActionsLocation") {
            old.show_tab_bar_buttons = Some(false)
        }
    }
}
