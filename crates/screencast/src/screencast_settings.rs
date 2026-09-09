use std::time::Duration;

use settings::{RegisterSetting, ScreencastSettingsContent, Settings, SettingsContent};

#[derive(Debug, Clone, Copy, RegisterSetting)]
pub struct ScreencastSettings {
    pub font_size: f32,
    pub vertical_offset: f32,
    pub keyboard_overlay_timeout: Duration,
}

impl Settings for ScreencastSettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let screencast = content.screencast.as_ref().unwrap();

        let vertical_offset_percent = screencast
            .vertical_offset
            .unwrap_or(ScreencastSettingsContent::DEFAULT_VERTICAL_OFFSET_PERCENT)
            .clamp(0.0, ScreencastSettingsContent::MAX_VERTICAL_OFFSET_PERCENT);

        let keyboard_overlay_timeout = screencast
            .keyboard_overlay_timeout
            .unwrap_or(ScreencastSettingsContent::DEFAULT_KEYBOARD_OVERLAY_TIMEOUT)
            .clamp(
                ScreencastSettingsContent::MIN_KEYBOARD_OVERLAY_TIMEOUT,
                ScreencastSettingsContent::MAX_KEYBOARD_OVERLAY_TIMEOUT,
            );

        Self {
            font_size: screencast.font_size.as_ref().unwrap().0,
            vertical_offset: vertical_offset_percent / 100.0,
            keyboard_overlay_timeout: Duration::from_millis(keyboard_overlay_timeout),
        }
    }
}
