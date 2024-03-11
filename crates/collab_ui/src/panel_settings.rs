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
    /// Whether to show the panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Where to dock the panel.
    ///
    /// Default: left
    pub dock: Option<DockPosition>,
    /// Default width of the panel in pixels.
    ///
    /// Default: 240
    pub default_width: Option<f32>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct MessageEditorSettings {
    /// Whether to automatically replace emoji shortcodes with emoji characters.
    /// For example: typing `:wave:` gets replaced with `👋`.
    ///
    /// Default: false
    pub auto_replace_emoji_shortcode: Option<bool>,
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

impl Settings for MessageEditorSettings {
    const KEY: Option<&'static str> = Some("message_editor");
    type FileContent = MessageEditorSettings;
    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
