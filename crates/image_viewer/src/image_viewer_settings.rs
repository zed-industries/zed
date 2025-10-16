pub use settings::ImageFileSizeUnit;
use settings::Settings;

/// The settings for the image viewer.
#[derive(Clone, Debug, Default)]
pub struct ImageViewerSettings {
    /// The unit to use for displaying image file sizes.
    ///
    /// Default: "binary"
    pub unit: ImageFileSizeUnit,
}

impl Settings for ImageViewerSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        Self {
            unit: content.image_viewer.clone().unwrap().unit.unwrap(),
        }
    }
}
