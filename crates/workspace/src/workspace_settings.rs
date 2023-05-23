use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Setting;

#[derive(Deserialize)]
pub struct WorkspaceSettings {
    pub active_pane_magnification: f32,
    pub confirm_quit: bool,
    pub show_call_status_icon: bool,
    pub autosave: AutosaveSetting,
    pub git: GitSettings,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct WorkspaceSettingsContent {
    pub active_pane_magnification: Option<f32>,
    pub confirm_quit: Option<bool>,
    pub show_call_status_icon: Option<bool>,
    pub autosave: Option<AutosaveSetting>,
    pub git: Option<GitSettings>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AutosaveSetting {
    Off,
    AfterDelay { milliseconds: u64 },
    OnFocusChange,
    OnWindowChange,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct GitSettings {
    pub git_gutter: Option<GitGutterSetting>,
    pub gutter_debounce: Option<u64>,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GitGutterSetting {
    #[default]
    TrackedFiles,
    Hide,
}

impl Setting for WorkspaceSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = WorkspaceSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
