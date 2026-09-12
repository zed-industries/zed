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

        let font_size = screencast
            .font_size
            .as_ref()
            .map(|font_size| font_size.0)
            .unwrap_or(ScreencastSettingsContent::DEFAULT_FONT_SIZE)
            .clamp(
                ScreencastSettingsContent::MIN_FONT_SIZE,
                ScreencastSettingsContent::MAX_FONT_SIZE,
            );

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
            font_size,
            vertical_offset: vertical_offset_percent / 100.0,
            keyboard_overlay_timeout: Duration::from_millis(keyboard_overlay_timeout),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn font_size_from_settings(font_size: Option<f32>) -> f32 {
        let content = SettingsContent {
            screencast: Some(ScreencastSettingsContent {
                font_size: font_size.map(Into::into),
                ..Default::default()
            }),
            ..Default::default()
        };

        ScreencastSettings::from_settings(&content).font_size
    }

    #[test]
    fn uses_default_font_size_when_missing() {
        assert_eq!(
            font_size_from_settings(None),
            ScreencastSettingsContent::DEFAULT_FONT_SIZE
        );
    }

    #[test]
    fn clamps_font_size_below_minimum() {
        assert_eq!(
            font_size_from_settings(Some(ScreencastSettingsContent::MIN_FONT_SIZE - 1.0)),
            ScreencastSettingsContent::MIN_FONT_SIZE
        );
    }

    #[test]
    fn preserves_font_size_within_range() {
        let font_size = 48.0;

        assert_eq!(font_size_from_settings(Some(font_size)), font_size);
    }

    #[test]
    fn clamps_font_size_above_maximum() {
        assert_eq!(
            font_size_from_settings(Some(ScreencastSettingsContent::MAX_FONT_SIZE + 1.0)),
            ScreencastSettingsContent::MAX_FONT_SIZE
        );
    }
}
