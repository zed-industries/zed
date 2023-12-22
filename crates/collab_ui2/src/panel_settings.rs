use anyhow;
use gpui::Pixels;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::Settings;
use workspace::dock::DockPosition;

#[derive(Deserialize, Debug)]
pub struct CollaborationPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: Pixels,
}

#[derive(Deserialize, Debug)]
pub struct ChatPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: Pixels,
}

#[derive(Deserialize, Debug)]
pub struct NotificationPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: Pixels,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct PanelSettingsContent {
    pub button: Option<bool>,
    pub dock: Option<DockPosition>,
    pub default_width: Option<f32>,
}

impl Settings for CollaborationPanelSettings {
    const KEY: Option<&'static str> = Some("collaboration_panel");
    type FileContent = PanelSettingsContent;
    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}

impl Settings for ChatPanelSettings {
    const KEY: Option<&'static str> = Some("chat_panel");
    type FileContent = PanelSettingsContent;
    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}

impl Settings for NotificationPanelSettings {
    const KEY: Option<&'static str> = Some("notification_panel");
    type FileContent = PanelSettingsContent;
    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
