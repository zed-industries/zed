use anyhow;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;

#[derive(Deserialize, Debug)]
pub struct SemanticIndexSettings {
    pub enabled: bool,
}

/// Configuration of semantic index, an alternate search engine available in
/// project search.
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct SemanticIndexSettingsContent {
    /// Whether or not to display the Semantic mode in project search.
    ///
    /// Default: true
    pub enabled: Option<bool>,
}

impl Settings for SemanticIndexSettings {
    const KEY: Option<&'static str> = Some("semantic_index");

    type FileContent = SemanticIndexSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
