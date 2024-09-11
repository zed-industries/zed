use anyhow::Result;
use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(default)]
/// Diagnostics configuration.
pub struct ProjectDiagnosticsSettings {
    /// Whether to show warnings or not by default.
    pub include_warnings: bool,
}

impl Default for ProjectDiagnosticsSettings {
    fn default() -> Self {
        Self {
            include_warnings: true,
        }
    }
}

impl Settings for ProjectDiagnosticsSettings {
    const KEY: Option<&'static str> = Some("diagnostics");
    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}
