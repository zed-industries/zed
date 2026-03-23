use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

/// The layout of window control buttons as represented by user settings.
///
/// This matches the string format used by GNOME `button-layout` settings (e.g.
/// "close:minimize,maximize").
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, Default)]
#[schemars(schema_with = "window_button_layout_schema")]
#[serde(from = "String", into = "String")]
pub enum WindowButtonLayoutContent {
    /// Follow the system/desktop configuration.
    #[default]
    Auto,
    /// Use Zed's own hardcoded default layout, regardless of system config.
    Default,
    /// A user-specified layout string.
    Custom(gpui::WindowButtonLayout),
}

fn window_button_layout_schema(_: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "anyOf": [
            { "enum": ["auto", "default"] },
            { "type": "string" }
        ]
    })
}

impl WindowButtonLayoutContent {
    pub fn into_layout(self) -> Option<gpui::WindowButtonLayout> {
        match self {
            Self::Auto => None,
            Self::Default => Some(gpui::WindowButtonLayout::default()),
            Self::Custom(layout) => Some(layout),
        }
    }
}

impl From<WindowButtonLayoutContent> for String {
    fn from(value: WindowButtonLayoutContent) -> Self {
        match value {
            WindowButtonLayoutContent::Auto => "auto".to_string(),
            WindowButtonLayoutContent::Default => "default".to_string(),
            WindowButtonLayoutContent::Custom(layout) => layout.format(),
        }
    }
}

impl From<String> for WindowButtonLayoutContent {
    fn from(layout_string: String) -> Self {
        match layout_string.as_str() {
            "auto" => Self::Auto,
            "default" => Self::Default,
            other => match gpui::WindowButtonLayout::parse(other) {
                Ok(layout) => Self::Custom(layout),
                Err(error) => {
                    log::warn!("Invalid button layout string {other:?}: {error:#}");
                    Self::Default
                }
            },
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
    /// This can be set to "auto" to follow the system configuration, or
    /// "default" to use Zed's hardcoded layout. For custom layouts, use a
    /// GNOME-style layout string like "close:minimize,maximize".
    ///
    /// Default: "auto"
    pub button_layout: Option<WindowButtonLayoutContent>,
}
