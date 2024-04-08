use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

impl settings::Settings for TaskSettings {
    const KEY: Option<&'static str> = Some("task");

    type FileContent = TaskSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> gpui::Result<Self>
    where
        Self: Sized,
    {
        let this = Self::json_merge(default_value, user_values)?;
        Ok(Self {
            show_status_indicator: this.show_status_indicator.unwrap_or(true),
        })
    }
}
