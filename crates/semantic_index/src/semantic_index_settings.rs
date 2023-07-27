use anyhow;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Setting;

#[derive(Deserialize, Debug)]
pub struct SemanticIndexSettings {
    pub enabled: bool,
    pub reindexing_delay_seconds: usize,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct SemanticIndexSettingsContent {
    pub enabled: Option<bool>,
    pub reindexing_delay_seconds: Option<usize>,
}

impl Setting for SemanticIndexSettings {
    const KEY: Option<&'static str> = Some("semantic_index");

    type FileContent = SemanticIndexSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
