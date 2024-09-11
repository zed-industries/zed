use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(default)]
/// Task-related settings.
pub(crate) struct TaskSettings {
    /// Whether to show task status indicator in the status bar. Default: true
    pub(crate) show_status_indicator: bool,
}

impl Default for TaskSettings {
    fn default() -> Self {
        Self {
            show_status_indicator: true,
        }
    }
}

impl Settings for TaskSettings {
    const KEY: Option<&'static str> = Some("task");

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> gpui::Result<Self> {
        sources.json_merge()
    }
}
