use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct AssistantSettings {
    pub enabled: bool,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct AssistantSettingsContent {
    pub enabled: Option<bool>,
}

impl Settings for AssistantSettings {
    const KEY: Option<&'static str> = Some("assistant_v2");

    type FileContent = AssistantSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Ok(sources.json_merge().unwrap_or_else(|_| Default::default()))
    }
}
