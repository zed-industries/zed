use anyhow::Result;
use collections::HashMap;
use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
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

    fn load(sources: SettingsSources<Self::FileContent>, _cx: &mut AppContext) -> Result<Self> {
        Ok(sources.user.cloned().unwrap_or_default())
    }
}
