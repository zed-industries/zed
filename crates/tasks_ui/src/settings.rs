use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Serialize, Deserialize, PartialEq, Default)]
pub(crate) struct TaskSettings {
    pub(crate) show_status_indicator: bool,
}

/// Task-related settings.
#[derive(Serialize, Deserialize, PartialEq, Default, Clone, JsonSchema)]
pub(crate) struct TaskSettingsContent {
    /// Whether to show task status indicator in the status bar. Default: true
    show_status_indicator: Option<bool>,
}

impl Settings for TaskSettings {
    const KEY: Option<&'static str> = Some("task");

    type FileContent = TaskSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> gpui::Result<Self> {
        sources.json_merge()
    }
}
