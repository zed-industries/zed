//! # zlog_settings
use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore, SettingsUi};

pub fn init(cx: &mut App) {
    ZlogSettings::register(cx);

    cx.observe_global::<SettingsStore>(|cx| {
        let zlog_settings = ZlogSettings::get_global(cx);
        zlog::filter::refresh_from_settings(&zlog_settings.scopes);
    })
    .detach();
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, SettingsUi)]
pub struct ZlogSettings {
    #[serde(default, flatten)]
    pub scopes: std::collections::HashMap<String, String>,
}

impl Settings for ZlogSettings {
    const KEY: Option<&'static str> = Some("log");

    type FileContent = Self;

    fn load(sources: settings::SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self>
    where
        Self: Sized,
    {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}
