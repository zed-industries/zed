use anyhow::Result;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use std::cmp;
use ui::Pixels;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct FileFinderSettings {
    pub file_icons: bool,
    pub modal_width: FileFinderWidth,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct FileFinderSettingsContent {
    /// Whether to show file icons in the file finder.
    ///
    /// Default: true
    pub file_icons: Option<bool>,
    /// The width of the file finder modal.
    ///
    /// Default: "medium"
    pub modal_width: Option<FileFinderWidth>,
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
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
    Full,
}

impl FileFinderWidth {
    const MIN_MODAL_WIDTH_PX: f32 = 384.;

    pub fn padding_px(&self) -> Pixels {
        let padding_val = match self {
            FileFinderWidth::Small => 1280.,
            FileFinderWidth::Medium => 1024.,
            FileFinderWidth::Large => 768.,
            FileFinderWidth::XLarge => 512.,
            FileFinderWidth::Full => 0.,
        };

        Pixels(padding_val)
    }

    pub fn calc_width(&self, window_width: Pixels) -> Pixels {
        if self == &FileFinderWidth::Full {
            return window_width;
        }

        let min_modal_width_px = Pixels(FileFinderWidth::MIN_MODAL_WIDTH_PX);

        let padding_px = self.padding_px();
        let width_val = window_width - padding_px;
        let finder_width = cmp::max(min_modal_width_px, width_val);

        finder_width
    }
}
