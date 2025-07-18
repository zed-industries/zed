use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

/// General settings that apply globally to Zed
#[derive(Clone, Debug)]
pub struct GeneralSettings {
    /// Whether to disable all AI features
    pub disable_ai: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct GeneralSettingsContent {
    /// Whether to disable all AI features in Zed.
    ///
    /// When enabled:
    /// - All AI-related commands are hidden from the command palette
    /// - Language model completions instantly return errors
    /// - Edit predictions are disabled
    /// - The agent panel button is removed from the UI
    /// - AI-related UI elements are not rendered
    ///
    /// Default: false
    #[serde(default)]
    pub disable_ai: Option<bool>,
}

impl Settings for GeneralSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = GeneralSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        let content: GeneralSettingsContent = sources.json_merge()?;
        Ok(Self {
            disable_ai: content.disable_ai.unwrap_or(false),
        })
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {
        // No VSCode equivalent for disable_ai
    }
}
