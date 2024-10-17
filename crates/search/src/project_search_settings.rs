use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct ProjectSearchSettings {
    pub button: bool,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct ProjectSearchSettingsContent {
    /// Whether to show the project search button in the status bar.
    ///
    /// Default: false
    pub button: Option<bool>,
}

impl Settings for ProjectSearchSettings {
    const KEY: Option<&'static str> = Some("project_search");

    type FileContent = ProjectSearchSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}
