use anyhow;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::Setting;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationPanelDockPosition {
    Left,
    Right,
}

#[derive(Deserialize, Debug)]
pub struct CollaborationPanelSettings {
    pub button: bool,
    pub dock: CollaborationPanelDockPosition,
    pub default_width: f32,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct CollaborationPanelSettingsContent {
    pub button: Option<bool>,
    pub dock: Option<CollaborationPanelDockPosition>,
    pub default_width: Option<f32>,
}

impl Setting for CollaborationPanelSettings {
    const KEY: Option<&'static str> = Some("collaboration_panel");

    type FileContent = CollaborationPanelSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
