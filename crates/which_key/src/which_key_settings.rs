use gpui::Keystroke;
use settings::{RegisterSetting, Settings, SettingsContent, WhichKeySettingsContent};

#[derive(Debug, Clone, RegisterSetting)]
pub struct WhichKeySettings {
    pub enabled: bool,
    pub delay_ms: u64,
    pub filtered_keystrokes: Vec<Vec<Keystroke>>,
}

impl Settings for WhichKeySettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let which_key: &WhichKeySettingsContent = content.which_key.as_ref().unwrap();

        let filtered_keystrokes = which_key
            .filtered_keystrokes
            .as_ref()
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|s| {
                        s.split(' ')
                            .map(|keystroke_str| Keystroke::parse(keystroke_str))
                            .collect::<Result<Vec<_>, _>>()
                            .ok()
                    })
                    .collect()
            })
            .unwrap_or_default();

        Self {
            enabled: which_key.enabled.unwrap(),
            delay_ms: which_key.delay_ms.unwrap(),
            filtered_keystrokes,
        }
    }
}
