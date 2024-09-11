use gpui::Pixels;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use ui::px;
use workspace::dock::DockPosition;

#[derive(Clone, Deserialize, Debug, JsonSchema, Serialize)]
#[serde(default)]
pub struct CollaborationPanelSettings {
    /// Whether to show the panel button in the status bar.
    pub button: bool,
    /// Where to dock the panel.
    pub dock: DockPosition,
    /// Default width of the panel in pixels.
    pub default_width: Pixels,
}

impl Default for CollaborationPanelSettings {
    fn default() -> Self {
        Self {
            button: true,
            dock: DockPosition::Left,
            default_width: px(240.),
        }
    }
}

#[derive(Clone, Deserialize, Debug, JsonSchema, Serialize)]
#[serde(default)]
pub struct ChatPanelSettings {
    /// Whether to show the panel button in the status bar.
    pub button: bool,
    /// Where to dock the panel.
    pub dock: DockPosition,
    /// Default width of the panel in pixels.
    pub default_width: Pixels,
}

impl Default for ChatPanelSettings {
    fn default() -> Self {
        Self {
            button: true,
            dock: DockPosition::Right,
            default_width: px(240.),
        }
    }
}

#[derive(Clone, Deserialize, Debug, JsonSchema, Serialize)]
#[serde(default)]
pub struct NotificationPanelSettings {
    /// Whether to show the panel button in the status bar.
    pub button: bool,
    /// Where to dock the panel.
    pub dock: DockPosition,
    /// Default width of the panel in pixels.
    pub default_width: Pixels,
}

impl Default for NotificationPanelSettings {
    fn default() -> Self {
        Self {
            button: true,
            dock: DockPosition::Right,
            default_width: px(380.),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(default)]
pub struct MessageEditorSettings {
    /// Whether to automatically replace emoji shortcodes with emoji characters.
    /// For example: typing `:wave:` gets replaced with `👋`.
    pub auto_replace_emoji_shortcode: bool,
}

impl Settings for CollaborationPanelSettings {
    const KEY: Option<&'static str> = Some("collaboration_panel");

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}

impl Settings for ChatPanelSettings {
    const KEY: Option<&'static str> = Some("chat_panel");

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}

impl Settings for NotificationPanelSettings {
    const KEY: Option<&'static str> = Some("notification_panel");

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}

impl Settings for MessageEditorSettings {
    const KEY: Option<&'static str> = Some("message_editor");

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}
