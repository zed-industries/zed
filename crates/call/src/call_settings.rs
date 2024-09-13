use anyhow::Result;
use gpui::AppContext;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

/// Configuration of voice calls in Zed.
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
#[serde(default)]
pub struct CallSettings {
    /// Whether the microphone should be muted when joining a channel or a call.
    pub mute_on_join: bool,
    /// Whether your current project should be shared when joining an empty channel.
    pub share_on_join: bool,
}

impl Settings for CallSettings {
    const KEY: Option<&'static str> = Some("calls");

    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}
