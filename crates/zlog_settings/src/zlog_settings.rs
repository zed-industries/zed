//! # zlog_settings
use collections::HashMap;

use gpui::App;
use settings::{Settings, SettingsStore};

pub fn init(cx: &mut App) {
    ZlogSettings::register(cx);

    cx.observe_global::<SettingsStore>(|cx| {
        let zlog_settings = ZlogSettings::get_global(cx);
        zlog::filter::refresh_from_settings(&zlog_settings.scopes);
    })
    .detach();
}

#[derive(Clone, Debug)]
pub struct ZlogSettings {
    /// A map of log scopes to the desired log level.
    /// Useful for filtering out noisy logs or enabling more verbose logging.
    ///
    /// Example: {"log": {"client": "warn"}}
    pub scopes: HashMap<String, String>,
}

impl Settings for ZlogSettings {
    fn from_settings(content: &settings::SettingsContent, _: &mut App) -> Self {
        ZlogSettings {
            scopes: content.log.clone().unwrap(),
        }
    }

    fn import_from_vscode(_: &settings::VsCodeSettings, _: &mut settings::SettingsContent) {}
}
