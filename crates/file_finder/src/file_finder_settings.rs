use anyhow::Result;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct FileFinderSettings {
    pub file_icons: bool,
    pub modal_max_width: Option<FileFinderWidth>,
    pub auto_select: FileFinderAutoSelect,
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
    /// Determines the initial selection behavior when opening the file finder.
    ///
    /// Default: skip_active
    pub auto_select: Option<FileFinderAutoSelect>,
}

impl Settings for FileFinderSettings {
    const KEY: Option<&'static str> = Some("file_finder");

    type FileContent = FileFinderSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut gpui::App) -> Result<Self> {
        sources.json_merge()
    }
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FileFinderAutoSelect {
    #[default]
    SkipActive,
    First,
}
