use anyhow::Result;
use gpui::AppContext;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::Settings;

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

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _cx: &mut AppContext,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        Self::load_via_json_merge(default_value, user_values)
    }
}
