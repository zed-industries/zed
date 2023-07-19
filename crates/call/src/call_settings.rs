use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::Setting;

#[derive(Deserialize, Debug)]
pub struct CallSettings {
    pub mute_on_join: bool,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct CallSettingsContent {
    pub mute_on_join: Option<bool>,
}

impl Setting for CallSettings {
    const KEY: Option<&'static str> = Some("calls");

    type FileContent = CallSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
