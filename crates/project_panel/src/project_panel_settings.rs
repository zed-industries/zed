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
    pub git_status: bool,
    pub file_icons: bool,
    pub dock: ProjectPanelDockPosition,
    pub default_width: f32,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct ProjectPanelSettingsContent {
    pub git_status: Option<bool>,
    pub file_icons: Option<bool>,
    pub dock: Option<ProjectPanelDockPosition>,
    pub default_width: Option<f32>,
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
