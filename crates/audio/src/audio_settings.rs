use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug, SettingsUi)]
pub struct AudioSettings {
    /// Opt into the new audio system.
    #[serde(rename = "experimental.rodio_audio", default)]
    pub rodio_audio: bool, // default is false
    /// Opt into the new audio systems automatic gain control
    #[serde(rename = "experimental.automatic_volume", default)]
    pub automatic_volume: bool,
}

/// Configuration of audio in Zed.
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug, SettingsUi, SettingsKey)]
#[serde(default)]
#[settings_key(key = "audio")]
pub struct AudioSettingsContent {
    /// Whether to use the experimental audio system
    #[serde(rename = "experimental.rodio_audio", default)]
    pub rodio_audio: bool,
    /// Whether the experimental audio systems should automatically
    /// manage the volume of calls
    #[serde(rename = "experimental.automatic_volume", default)]
    pub automatic_volume: bool,
}

impl Settings for AudioSettings {
    type FileContent = AudioSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _cx: &mut App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}
