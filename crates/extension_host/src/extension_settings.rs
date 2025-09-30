use collections::HashMap;
use gpui::App;
use settings::Settings;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct ExtensionSettings {
    /// The extensions that should be automatically installed by Zed.
    ///
    /// This is used to make functionality provided by extensions (e.g., language support)
    /// available out-of-the-box.
    ///
    /// Default: { "html": true }
    pub auto_install_extensions: HashMap<Arc<str>, bool>,
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
    fn from_settings(content: &settings::SettingsContent, _cx: &mut App) -> Self {
        Self {
            auto_install_extensions: content.extension.auto_install_extensions.clone(),
            auto_update_extensions: content.extension.auto_update_extensions.clone(),
        }
    }
}
