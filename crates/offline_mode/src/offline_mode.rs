//! Contains the [`OfflineModeSetting`] used to enable/disable offline mode.
//!
//! When offline mode is enabled, all network features are disabled. This allows
//! Zed to be used in air-gapped environments or when network access is restricted.

use settings::{RegisterSetting, Settings, SettingsContent};

#[derive(RegisterSetting)]
pub struct OfflineModeSetting(pub bool);

impl Settings for OfflineModeSetting {
    fn from_settings(content: &SettingsContent) -> Self {
        Self(content.offline.unwrap_or(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offline_mode_default() {
        let content = SettingsContent::default();
        let setting = OfflineModeSetting::from_settings(&content);
        assert!(!setting.0, "Offline mode should default to false");
    }

    #[test]
    fn test_offline_mode_true() {
        let mut content = SettingsContent::default();
        content.offline = Some(true);
        let setting = OfflineModeSetting::from_settings(&content);
        assert!(setting.0, "Offline mode should be true when set to true");
    }

    #[test]
    fn test_offline_mode_false() {
        let mut content = SettingsContent::default();
        content.offline = Some(false);
        let setting = OfflineModeSetting::from_settings(&content);
        assert!(!setting.0, "Offline mode should be false when set to false");
    }
}
