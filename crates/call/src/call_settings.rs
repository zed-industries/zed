use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};

#[derive(Deserialize, Debug)]
pub struct CallSettings {
    pub mute_on_join: bool,
    pub share_on_join: bool,
}

/// Configuration of voice calls in Zed.
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug, SettingsUi, SettingsKey)]
#[settings_key(key = "calls")]
pub struct CallSettingsContent {
    /// Whether the microphone should be muted when joining a channel or a call.
    ///
    /// Default: false
    pub mute_on_join: Option<bool>,

    /// Whether your current project should be shared when joining an empty channel.
    ///
    /// Default: false
    pub share_on_join: Option<bool>,
}

impl Settings for CallSettings {
    type FileContent = CallSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}
