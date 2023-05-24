use collections::HashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Setting;
use std::sync::Arc;

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ProjectSettings {
    #[serde(default)]
    pub lsp: HashMap<Arc<str>, LspSettings>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LspSettings {
    pub initialization_options: Option<serde_json::Value>,
}

impl Setting for ProjectSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = Self;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
