use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;

#[derive(Deserialize)]
pub struct WorkspaceSettings {
    pub active_pane_magnification: f32,
    pub confirm_quit: bool,
    pub show_call_status_icon: bool,
    pub show_status_bar: bool,
    pub show_feedback_icon: bool,
    pub show_cursor_position: bool,
    pub show_active_langauge: bool,
    pub autosave: AutosaveSetting,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct WorkspaceSettingsContent {
    /// Scale by which to zoom the active pane.
    /// When set to 1.0, the active pane has the same size as others,
    /// but when set to a larger value, the active pane takes up more space.
    ///
    /// Default: `1.0`
    #[serde(default)]
    pub active_pane_magnification: Option<f32>,
    /// Whether or not to prompt the user to confirm before closing the application.
    ///
    /// Default: false
    #[serde(default)]
    pub confirm_quit: Option<bool>,
    /// Whether or not to show the call status icon in the status bar.
    ///
    /// Default: true
    #[serde(default)]
    pub show_call_status_icon: Option<bool>,
    /// Whether or not to show the status bar.
    ///
    /// Default: true
    #[serde(default)]
    pub show_status_bar: Option<bool>,
    /// Whether or not to show the share feedback icon in status bar.
    ///
    /// Default: true
    #[serde(default)]
    pub show_feedback_icon: Option<bool>,
    /// Whether or not to show the share feedback icon in status bar.
    ///
    /// Default: true
    #[serde(default)]
    pub show_cursor_position: Option<bool>,
    /// Whether or not to show the active language in status bar.
    ///
    /// Default: true
    #[serde(default)]
    pub show_active_langauge: Option<bool>,
    /// When to automatically save edited buffers.
    ///
    /// Default: off
    #[serde(default)]
    pub autosave: Option<AutosaveSetting>,
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
