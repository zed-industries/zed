use gpui::WindowButtonLayout;
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

/// The layout of window control buttons as represented by user settings.
///
/// Custom layout strings use the GNOME `button-layout` format (e.g.
/// `"close:minimize,maximize"`).
#[derive(
    Clone,
    PartialEq,
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    Default,
    strum::EnumDiscriminants,
)]
#[strum_discriminants(derive(strum::VariantArray, strum::VariantNames, strum::FromRepr))]
#[schemars(schema_with = "window_button_layout_schema")]
#[serde(from = "String", into = "String")]
pub enum WindowButtonLayoutContent {
    /// Follow the system/desktop configuration.
    #[default]
    PlatformDefault,
    /// Use Zed's built-in standard layout, regardless of system config.
    Standard,
    /// A raw GNOME-style layout string.
    Custom(String),
}

impl WindowButtonLayoutContent {
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn into_layout(self) -> Option<WindowButtonLayout> {
        use util::ResultExt;

        match self {
            Self::PlatformDefault => None,
            Self::Standard => Some(WindowButtonLayout::linux_default()),
            Self::Custom(layout) => WindowButtonLayout::parse(&layout).log_err(),
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    pub fn into_layout(self) -> Option<WindowButtonLayout> {
        None
    }
}

fn window_button_layout_schema(_: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "anyOf": [
            { "enum": ["platform_default", "standard"] },
            { "type": "string" }
        ]
    })
}

impl From<WindowButtonLayoutContent> for String {
    fn from(value: WindowButtonLayoutContent) -> Self {
        match value {
            WindowButtonLayoutContent::PlatformDefault => "platform_default".to_string(),
            WindowButtonLayoutContent::Standard => "standard".to_string(),
            WindowButtonLayoutContent::Custom(s) => s,
        }
    }
}

impl From<String> for WindowButtonLayoutContent {
    fn from(layout_string: String) -> Self {
        match layout_string.as_str() {
            "platform_default" => Self::PlatformDefault,
            "standard" => Self::Standard,
            _ => Self::Custom(layout_string),
        }
    }
}

#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct TitleBarSettingsContent {
    /// Whether to show the branch icon beside branch switcher in the title bar.
    ///
    /// Default: false
    pub show_branch_icon: Option<bool>,
    /// Whether to show onboarding banners in the title bar.
    ///
    /// Default: true
    pub show_onboarding_banner: Option<bool>,
    /// Whether to show user avatar in the title bar.
    ///
    /// Default: true
    pub show_user_picture: Option<bool>,
    /// Whether to show the branch name button in the titlebar.
    ///
    /// Default: true
    pub show_branch_name: Option<bool>,
    /// Whether to show the project host and name in the titlebar.
    ///
    /// Default: true
    pub show_project_items: Option<bool>,
    /// Whether to show the sign in button in the title bar.
    ///
    /// Default: true
    pub show_sign_in: Option<bool>,
    /// Whether to show the user menu button in the title bar.
    ///
    /// Default: true
    pub show_user_menu: Option<bool>,
    /// Whether to show the menus in the title bar.
    ///
    /// Default: false
    pub show_menus: Option<bool>,
    /// The layout of window control buttons in the title bar (Linux only).
    ///
    /// This can be set to "platform_default" to follow the system configuration, or
    /// "standard" to use Zed's built-in layout. For custom layouts, use a
    /// GNOME-style layout string like "close:minimize,maximize".
    ///
    /// Default: "platform_default"
    pub button_layout: Option<WindowButtonLayoutContent>,
}
