use anyhow::Result;
use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Deserialize, Debug)]
pub struct ProjectDiagnosticsSettings {
    pub include_warnings: bool,
}

/// Diagnostics configuration.
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct ProjectDiagnosticsSettingsContent {
    /// Whether to show warnings or not by default.
    ///
    /// Default: true
    include_warnings: Option<bool>,
}

impl Settings for ProjectDiagnosticsSettings {
    const KEY: Option<&'static str> = Some("diagnostics");
    type FileContent = ProjectDiagnosticsSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}
