use anyhow;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::Setting;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ChannelsPanelDockPosition {
    Left,
    Right,
}

#[derive(Deserialize, Debug)]
pub struct ChannelsPanelSettings {
    pub dock: ChannelsPanelDockPosition,
    pub default_width: f32,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct ChannelsPanelSettingsContent {
    pub dock: Option<ChannelsPanelDockPosition>,
    pub default_width: Option<f32>,
}

impl Setting for ChannelsPanelSettings {
    const KEY: Option<&'static str> = Some("channels_panel");

    type FileContent = ChannelsPanelSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
