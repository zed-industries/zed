use anyhow::Result;
use collections::HashMap;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug, Default, Clone, JsonSchema, SettingsUi, SettingsKey)]
#[settings_key(None)]
pub struct ExtensionSettings {
    /// The extensions that should be automatically installed by Zed.
    ///
    /// This is used to make functionality provided by extensions (e.g., language support)
    /// available out-of-the-box.
    ///
    /// Default: { "html": true }
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
    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _cx: &mut App) -> Result<Self> {
        SettingsSources::<Self::FileContent>::json_merge_with(
            [sources.default]
                .into_iter()
                .chain(sources.user)
                .chain(sources.server),
        )
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {
        // settingsSync.ignoredExtensions controls autoupdate for vscode extensions, but we
        // don't have a mapping to zed-extensions. there's also extensions.autoCheckUpdates
        // and extensions.autoUpdate which are global switches, we don't support those yet
    }
}
