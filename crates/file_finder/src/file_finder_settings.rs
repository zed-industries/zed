use anyhow::Result;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct FileFinderSettings {
    pub file_icons: bool,
    pub modal_max_width: Option<FileFinderWidth>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct FileFinderSettingsContent {
    /// Whether to show file icons in the file finder.
    ///
    /// Default: true
    pub file_icons: Option<bool>,
    /// Determines how much space the file finder can take up in relation to the available window width.
    ///
    /// Default: small
    pub modal_max_width: Option<FileFinderWidth>,
}

impl Settings for FileFinderSettings {
    const KEY: Option<&'static str> = Some("file_finder");

    type FileContent = FileFinderSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut gpui::App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VSCodeSettings, _old: &mut Self::FileContent) {}
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
