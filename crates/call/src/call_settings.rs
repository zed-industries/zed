use anyhow::Result;
use gpui::AppContext;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Deserialize, Debug)]
pub struct CallSettings {
    pub mute_on_join: bool,
    pub share_on_join: bool,
}

/// Configuration of voice calls in Zed.
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct CallSettingsContent {
    /// Whether the microphone should be muted when joining a channel or a call.
    ///
    /// Default: false
    pub mute_on_join: Option<bool>,

    /// Whether your current project should be shared when joining an empty channel.
    ///
    /// Default: true
    pub share_on_join: Option<bool>,
}

impl Settings for CallSettings {
    const KEY: Option<&'static str> = Some("calls");

    type FileContent = CallSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}
