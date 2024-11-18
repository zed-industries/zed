use anyhow::Result;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct FileFinderSettings {
    pub file_icons: bool,
    pub modal_width: Option<FileFinderWidth>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct FileFinderSettingsContent {
    /// Whether to show file icons in the file finder.
    ///
    /// Default: true
    pub file_icons: Option<bool>,
    /// Max-width of the file finder modal. If no value is specified, the file finder takes a min-width.
    /// There are 5 possible width values:
    ///
    /// 0. Take the min-width:
    ///    "modal_width": "null",
    /// 1. "modal_width": "small"
    /// 2. "modal_width": "medium"
    /// 3. "modal_width": "large"
    /// 4. "modal_width": "xlarge"
    /// 5. Take the whole, fullscreen width:
    ///    "modal_width": "full"
    ///
    /// Default: null
    pub modal_width: Option<FileFinderWidth>,
}

impl Settings for FileFinderSettings {
    const KEY: Option<&'static str> = Some("file_finder");

    type FileContent = FileFinderSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut gpui::AppContext) -> Result<Self> {
        sources.json_merge()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum FileFinderWidth {
    Small,
    Medium,
    Large,
    XLarge,
    Full,
}
