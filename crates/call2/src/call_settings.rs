use anyhow::Result;
use gpui2::AppContext;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings2::Settings;

#[derive(Deserialize, Debug)]
pub struct CallSettings {
    pub mute_on_join: bool,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct CallSettingsContent {
    pub mute_on_join: Option<bool>,
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
