use serde::Deserialize;
use settings::{Settings, SettingsContent, SettingsKey, WhichKeySettingsContent};

#[derive(Deserialize)]
pub struct WhichKeySettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_delay_ms")]
    pub delay_ms: u64,
}

fn default_delay_ms() -> u64 {
    700
}

impl Settings for WhichKeySettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let which_key: WhichKeySettingsContent = content.which_key.clone().unwrap_or_default();

        Self {
            enabled: which_key.enabled.unwrap_or(false),
            delay_ms: which_key.delay_ms.unwrap_or_else(default_delay_ms),
        }
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut SettingsContent) {
        // No equivalent setting in VScode
    }
}

impl SettingsKey for WhichKeySettings {
    const KEY: Option<&'static str> = Some("which_key");
}
