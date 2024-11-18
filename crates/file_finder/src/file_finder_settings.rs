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
    /// There are 5 possible width values:
    ///
    /// 1. Small: This value is essentially a fixed width.
    ///    "modal_width": "small"
    /// 2. Medium:
    ///    "modal_width": "medium"
    /// 3. Large:
    ///    "modal_width": "large"
    /// 4. Extra Large:
    ///    "modal_width": "xlarge"
    /// 5. Fullscreen: This value removes any horizontal padding, as it consumes the whole viewport width.
    ///    "modal_width": "full"
    ///
    /// Default: small
    pub modal_max_width: Option<FileFinderWidth>,
}

impl Settings for FileFinderSettings {
    const KEY: Option<&'static str> = Some("file_finder");

    type FileContent = FileFinderSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut gpui::AppContext) -> Result<Self> {
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
