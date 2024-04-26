use anyhow::Result;
use collections::HashMap;
use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug, Default, Clone, JsonSchema)]
pub struct ExtensionSettings {
    /// The extensions that should be automatically installed by Zed.
    ///
    /// This is used to make functionality provided by extensions (e.g., language support)
    /// available out-of-the-box.
    #[serde(default)]
    pub auto_install_extensions: HashMap<Arc<str>, bool>,
    #[serde(default)]
    pub auto_update_extensions: HashMap<Arc<str>, bool>,
}

impl ExtensionSettings {
    /// Returns whether the given extension should be auto-installed.
    pub fn should_auto_install(&self, extension_id: &str) -> bool {
        self.auto_install_extensions
            .get(extension_id)
            .copied()
            .unwrap_or(true)
    }

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
        SettingsSources::<Self::FileContent>::json_merge_with(
            [sources.default].into_iter().chain(sources.user),
        )
    }
}
