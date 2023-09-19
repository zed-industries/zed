use anyhow;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::Setting;
use workspace::dock::DockPosition;

#[derive(Deserialize, Debug)]
pub struct CollaborationPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: f32,
}

#[derive(Deserialize, Debug)]
pub struct ChatPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: f32,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct PanelSettingsContent {
    pub button: Option<bool>,
    pub dock: Option<DockPosition>,
    pub default_width: Option<f32>,
}

impl Setting for CollaborationPanelSettings {
    const KEY: Option<&'static str> = Some("collaboration_panel");

    type FileContent = PanelSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}

impl Setting for ChatPanelSettings {
    const KEY: Option<&'static str> = Some("chat_panel");

    type FileContent = PanelSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
