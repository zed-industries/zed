use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct FileFinderSettings {
    pub file_icons: bool,
    pub modal_max_width: Option<FileFinderWidth>,
    pub skip_focus_for_active_in_search: bool,
    pub include_ignored: Option<bool>,
}

impl Settings for FileFinderSettings {
    type FileContent = FileFinderSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut gpui::App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum FileFinderWidth {
    #[default]
    Small,
    Medium,
    Large,
    XLarge,
    Full,
}
