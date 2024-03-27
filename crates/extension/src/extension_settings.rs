use anyhow::Result;
use collections::HashMap;
use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug, Default, Clone, JsonSchema)]
pub struct ExtensionSettings {
    #[serde(default)]
    pub auto_update_extensions: HashMap<Arc<str>, bool>,
}

impl ExtensionSettings {
    pub fn should_auto_update(&self, extension_id: &str) -> bool {
        self.auto_update_extensions
            .get(extension_id)
            .copied()
            .unwrap_or(true)
    }
}

impl Settings for ExtensionSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = Self;

    fn load(
        _default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _cx: &mut AppContext,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(user_values.get(0).copied().cloned().unwrap_or_default())
    }
}
