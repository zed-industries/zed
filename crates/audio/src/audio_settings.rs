use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources, SettingsUi};

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct AudioSettings {
    /// Opt into the new audio system.
    #[serde(rename = "experimental.rodio_audio", default)]
    pub rodio_audio: bool, // default is false
}

/// Configuration of audio in Zed.
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug, SettingsUi)]
#[serde(default)]
pub struct AudioSettingsContent {
    /// Whether to use the experimental audio system
    #[serde(rename = "experimental.rodio_audio", default)]
    pub rodio_audio: bool,
}

impl Settings for AudioSettings {
    const KEY: Option<&'static str> = Some("audio");

    type FileContent = AudioSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _cx: &mut App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}
