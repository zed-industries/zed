use anyhow::Result;
use collections::HashMap;
use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct WorkspaceSettings {
    /// Scale by which to zoom the active pane.
    /// When set to 1.0, the active pane has the same size as others,
    /// but when set to a larger value, the active pane takes up more space.
    pub active_pane_magnification: f32,
    /// Direction to split horizontally.
    pub pane_split_direction_horizontal: PaneSplitDirectionHorizontal,
    /// Direction to split vertically.
    pub pane_split_direction_vertical: PaneSplitDirectionVertical,
    /// Centered layout related settings.
    pub centered_layout: CenteredLayoutSettings,
    /// Whether or not to prompt the user to confirm before closing the application.
    pub confirm_quit: bool,
    /// Whether or not to show the call status icon in the status bar.
    pub show_call_status_icon: bool,
    /// When to automatically save edited buffers.
    pub autosave: AutosaveSetting,
    /// Controls previous session restoration in freshly launched Zed instance.
    pub restore_on_startup: RestoreOnStartupBehavior,
    /// The size of the workspace split drop targets on the outer edges.
    /// Given as a fraction that will be multiplied by the smaller dimension of the workspace.
    pub drop_target_size: f32,
    /// Whether to close the window when using 'close active item' on a workspace with no tabs
    pub when_closing_with_no_tabs: CloseWindowWhenNoItems,
    /// Whether to use the system provided dialogs for Open and Save As.
    /// When set to false, Zed will use the built-in keyboard-first pickers.
    pub use_system_path_prompts: bool,
    /// Aliases for the command palette. When you type a key in this map,
    /// it will be assumed to equal the value.
    pub command_aliases: HashMap<String, String>,
}

impl Default for WorkspaceSettings {
    fn default() -> Self {
        Self {
            active_pane_magnification: 1.0,
            pane_split_direction_horizontal: PaneSplitDirectionHorizontal::Up,
            pane_split_direction_vertical: PaneSplitDirectionVertical::Left,
            centered_layout: CenteredLayoutSettings::default(),
            confirm_quit: false,
            show_call_status_icon: true,
            autosave: AutosaveSetting::Off,
            restore_on_startup: RestoreOnStartupBehavior::default(),
            drop_target_size: 0.2,
            when_closing_with_no_tabs: CloseWindowWhenNoItems::default(),
            use_system_path_prompts: true,
            command_aliases: HashMap::default(),
        }
    }
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

#[derive(Copy, Clone, Default, Serialize, Deserialize, JsonSchema)]
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

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct TabBarSettings {
    /// Whether or not to show the tab bar in the editor.
    pub show: bool,
    /// Whether or not to show the navigation history buttons in the tab bar.
    pub show_nav_history_buttons: bool,
}

impl Default for TabBarSettings {
    fn default() -> Self {
        Self {
            show_nav_history_buttons: true,
            show: true,
        }
    }
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
    /// The relative width of the right padding of the central pane from the
    /// workspace when the centered layout is used.
    ///
    /// Default: 0.2
    pub right_padding: Option<f32>,
}

impl Default for CenteredLayoutSettings {
    fn default() -> Self {
        Self {
            left_padding: Some(0.2),
            right_padding: Some(0.2),
        }
    }
}

impl Settings for WorkspaceSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}

impl Settings for TabBarSettings {
    const KEY: Option<&'static str> = Some("tab_bar");

    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}
