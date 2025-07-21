use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Deserialize, Debug)]
pub struct WhichKeySettings {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_delay_ms")]
    pub delay_ms: u64,
}

fn default_enabled() -> bool {
    true
}

fn default_delay_ms() -> u64 {
    600
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
