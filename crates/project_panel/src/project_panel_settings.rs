use anyhow;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::Setting;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProjectPanelDockPosition {
    Left,
    Right,
}

#[derive(Deserialize, Debug)]
pub struct ProjectPanelSettings {
    pub default_width: f32,
    pub dock: ProjectPanelDockPosition,
    pub file_icons: bool,
    pub folder_icons: bool,
    pub git_status: bool,
    pub indent_size: f32,
    pub auto_reveal_entries: bool,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct ProjectPanelSettingsContent {
    pub default_width: Option<f32>,
    pub dock: Option<ProjectPanelDockPosition>,
    pub file_icons: Option<bool>,
    pub folder_icons: Option<bool>,
    pub git_status: Option<bool>,
    pub indent_size: Option<f32>,
    pub auto_reveal_entries: Option<bool>,
}

impl Setting for ProjectPanelSettings {
    const KEY: Option<&'static str> = Some("project_panel");

    type FileContent = ProjectPanelSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
