use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

impl settings::Settings for ProjectDiagnosticsSettings {
    const KEY: Option<&'static str> = Some("diagnostics");
    type FileContent = ProjectDiagnosticsSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _cx: &mut gpui::AppContext,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Self::load_via_json_merge(default_value, user_values)
    }
}
