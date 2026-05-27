use gpui::App;
use settings::{RegisterSetting, Settings};

#[derive(Debug, Clone, RegisterSetting)]
pub struct SendCodeSettings {
    pub enabled: bool,
    pub bracketed_paste: bool,
}

impl Default for SendCodeSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            bracketed_paste: true,
        }
    }
}

impl SendCodeSettings {
    pub fn enabled(cx: &App) -> bool {
        Self::get_global(cx).enabled
    }
}

impl Settings for SendCodeSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let Some(sc) = content.send_code.as_ref() else {
            return Self::default();
        };
        Self {
            enabled: sc.enabled.unwrap_or(true),
            bracketed_paste: sc.bracketed_paste.unwrap_or(true),
        }
    }
}

pub use settings::settings_content::SendCodeSettingsContent;
