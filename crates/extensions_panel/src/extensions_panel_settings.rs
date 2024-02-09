use anyhow;
use gpui::Pixels;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::Settings;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionsPanelDockPosition {
    Left,
    Right,
}

#[derive(Deserialize, Debug)]
pub struct ExtensionsPanelSettings {
    pub default_width: Pixels,
    pub dock: ExtensionsPanelDockPosition,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct ExtensionsPanelSettingsContent {
    /// Customise default width (in pixels) taken by project panel
    ///
    /// Default: 240
    pub default_width: Option<f32>,
    /// The position of project panel
    ///
    /// Default: left
    pub dock: Option<ExtensionsPanelDockPosition>,
}

impl Settings for ExtensionsPanelSettings {
    const KEY: Option<&'static str> = Some("extensions_panel");

    type FileContent = ExtensionsPanelSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
