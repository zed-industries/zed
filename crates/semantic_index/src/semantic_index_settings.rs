use anyhow;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;

#[derive(Deserialize, Debug)]
pub struct SemanticIndexSettings {
    pub enabled: bool,
    pub openai_embedding_api_url: String,
    pub default_openai_embedding_model: String,
}

/// Configuration of semantic index, an alternate search engine available in
/// project search.
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct SemanticIndexSettingsContent {
    /// Whether or not to display the Semantic mode in project search.
    ///
    /// Default: true
    pub enabled: Option<bool>,

    /// openai_embedding_api_url
    ///
    /// Default: https://YOUR_RESOURCE_NAME.openai.azure.com/openai/deployments/YOUR_DEPLOYMENT_NAME/chat/completions?api-version=2023-12-01-preview
    pub openai_embedding_api_url: Option<String>,

    /// default_openai_embedding_model
    ///
    /// Default: text-embedding-ada-002
    pub default_openai_embedding_model: Option<String>,
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
