use gpui::Pixels;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use workspace::dock::DockPosition;

#[derive(Deserialize, Debug)]
pub struct CollaborationPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: Pixels,
}

#[derive(Clone, Copy, Default, Serialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ChatPanelButton {
    Never,
    Always,
    #[default]
    WhenInCall,
}

impl<'de> Deserialize<'de> for ChatPanelButton {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = ChatPanelButton;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    r#"a boolean or one of "never", "always", "when_in_call""#
                )
            }

            fn visit_bool<E>(self, b: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match b {
                    false => Ok(ChatPanelButton::Never),
                    true => Ok(ChatPanelButton::Always),
                }
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match s {
                    "never" => Ok(ChatPanelButton::Never),
                    "always" => Ok(ChatPanelButton::Always),
                    "when_in_call" => Ok(ChatPanelButton::WhenInCall),
                    _ => Err(E::unknown_variant(s, &["never", "always", "when_in_call"])),
                }
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Deserialize, Debug)]
pub struct ChatPanelSettings {
    pub button: ChatPanelButton,
    pub dock: DockPosition,
    pub default_width: Pixels,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct ChatPanelSettingsContent {
    /// When to show the panel button in the status bar.
    ///
    /// Default: only when in a call
    pub button: Option<ChatPanelButton>,
    /// Where to dock the panel.
    ///
    /// Default: right
    pub dock: Option<DockPosition>,
    /// Default width of the panel in pixels.
    ///
    /// Default: 240
    pub default_width: Option<f32>,
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
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::App,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}

impl Settings for ChatPanelSettings {
    const KEY: Option<&'static str> = Some("chat_panel");

    type FileContent = ChatPanelSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::App,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}

impl Settings for NotificationPanelSettings {
    const KEY: Option<&'static str> = Some("notification_panel");

    type FileContent = PanelSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::App,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}

impl Settings for MessageEditorSettings {
    const KEY: Option<&'static str> = Some("message_editor");

    type FileContent = MessageEditorSettings;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::App,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}
