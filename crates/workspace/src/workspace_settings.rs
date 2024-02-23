use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;

#[derive(Deserialize)]
pub struct WorkspaceSettings {
    pub active_pane_magnification: f32,
    pub confirm_quit: bool,
    pub status_bar: StatusBarSettings,
    pub autosave: AutosaveSetting,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct WorkspaceSettingsContent {
    /// Scale by which to zoom the active pane.
    /// When set to 1.0, the active pane has the same size as others,
    /// but when set to a larger value, the active pane takes up more space.
    ///
    /// Default: `1.0`
    pub active_pane_magnification: Option<f32>,
    /// Whether or not to prompt the user to confirm before closing the application.
    ///
    /// Default: false
    pub confirm_quit: Option<bool>,
    /// Status bar settings.
    ///
    pub status_bar: StatusBarSettings,
    /// Whether or not to show the call status icon in the status bar.
    ///
    /// Default: true
    pub show_call_status_icon: Option<bool>,
    /// When to automatically save edited buffers.
    ///
    /// Default: off
    pub autosave: Option<AutosaveSetting>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct StatusBarSettings {
    /// Settings for the elements in the status bar.
    ///
    /// Default: all elements are visible
    pub elements: Option<StatusBarElementSettings>,
    /// Whether or not to show the status bar.
    ///
    /// Default: true
    pub visible: Option<bool>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct StatusBarElementSettings {
    /// Whether or not to show the assistant icon in the status bar.
    ///
    /// Default: true
    pub assistant: Option<bool>,
    /// Whether or not to show the chat icon in the status bar.
    ///
    /// Default: true
    pub chat: Option<bool>,
    /// Whether or not to show the collaboration panel icon in the status bar.
    ///
    /// Default: true
    pub collaboration: Option<bool>,
    /// Whether or not to show the copilot icon in the status bar.
    ///
    /// Default: true
    pub copilot: Option<bool>,
    /// Whether or not to show the cursor position in the status bar.
    ///
    /// Default: true
    pub cursor_position: Option<bool>,
    /// Whether or not to show the diagnostics in the status bar.
    ///
    /// Default: true
    pub diagnostics: Option<bool>,
    /// Whether or not to show the feedback icon in the status bar.
    ///
    /// Default: true
    pub feedback: Option<bool>,
    /// Whether or not to show the language selector in the status bar.
    ///
    /// Default: true
    pub language_selector: Option<bool>,
    /// Whether or not to show the notification icon in the status bar.
    ///
    /// Default: true
    pub notification: Option<bool>,
    /// Whether or not to show the project panel icon in the status bar.
    ///
    /// Default: true
    pub project: Option<bool>,
    /// Whether or not to show the terminal icon in the status bar.
    ///
    /// Default: true
    pub terminal: Option<bool>,
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

impl Settings for WorkspaceSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = WorkspaceSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
