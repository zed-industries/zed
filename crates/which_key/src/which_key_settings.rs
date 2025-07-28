use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WhichKeyLocation {
    Buffer,
    LeftPanel,
}

#[derive(Deserialize, Debug)]
pub struct WhichKeySettings {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_delay_ms")]
    pub delay_ms: u64,
    #[serde(default = "default_group")]
    pub group: bool,
    #[serde(default = "default_location")]
    pub location: WhichKeyLocation,
}

fn default_enabled() -> bool {
    true
}

fn default_delay_ms() -> u64 {
    600
}

fn default_group() -> bool {
    true
}

fn default_location() -> WhichKeyLocation {
    WhichKeyLocation::Buffer
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct WhichKeySettingsContent {
    /// Whether to show the which-key popup when holding down key combinations.
    ///
    /// Default: true
    pub enabled: Option<bool>,
    /// Delay in milliseconds before showing the which-key popup.
    ///
    /// Default: 600
    pub delay_ms: Option<u64>,
    /// Whether to group key bindings with the same first keystroke.
    ///
    /// Default: true
    pub group: Option<bool>,
    /// Where to show the which-key popup.
    ///
    /// Default: buffer
    pub location: Option<WhichKeyLocation>,
}

impl Settings for WhichKeySettings {
    const KEY: Option<&'static str> = Some("which_key");

    type FileContent = WhichKeySettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {
        // No equivalent setting in VSCode
    }
}
