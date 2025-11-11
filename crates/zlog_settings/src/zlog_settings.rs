//! # zlog_settings
use collections::HashMap;

use gpui::App;
use settings::{RegisterSetting, Settings, SettingsStore};

pub fn init(cx: &mut App) {
    cx.observe_global::<SettingsStore>(|cx| {
        let zlog_settings = ZlogSettings::get_global(cx);
        zlog::filter::refresh_from_settings(&zlog_settings.scopes);
    })
    .detach();
}

#[derive(Clone, Debug, RegisterSetting)]
pub struct ZlogSettings {
    /// A map of log scopes to the desired log level.
    /// Useful for filtering out noisy logs or enabling more verbose logging.
    ///
    /// Example: {"log": {"client": "warn"}}
    pub scopes: HashMap<String, String>,
}

impl Settings for ZlogSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        ZlogSettings {
            scopes: content.log.clone().unwrap(),
        }
    }
}
