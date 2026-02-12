use settings::{RegisterSetting, Settings, SettingsContent, WhichKeySettingsContent};

#[derive(Debug, Clone, Copy, RegisterSetting)]
pub struct WhichKeySettings {
    pub enabled: bool,
    pub delay_ms: u64,
}

impl Settings for WhichKeySettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let which_key: &WhichKeySettingsContent = content.which_key.as_ref().unwrap();

        Self {
            enabled: which_key.enabled.unwrap(),
            delay_ms: which_key.delay_ms.unwrap(),
        }
    }
}
