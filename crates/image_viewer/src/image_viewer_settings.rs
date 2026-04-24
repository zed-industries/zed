pub use settings::{ImageFileSizeUnit, ImageSmoothing};
use settings::{RegisterSetting, Settings};

/// The settings for the image viewer.
#[derive(Clone, Debug, Default, RegisterSetting)]
pub struct ImageViewerSettings {
    /// The unit to use for displaying image file sizes.
    ///
    /// Default: "binary"
    pub unit: ImageFileSizeUnit,
    /// How to interpolate scaled images.
    ///
    /// Default: "linear"
    pub image_smoothing: ImageSmoothing,
}

impl Settings for ImageViewerSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let image_viewer = content.image_viewer.clone().unwrap();
        Self {
            unit: image_viewer.unit.unwrap(),
            image_smoothing: image_viewer.image_smoothing.unwrap(),
        }
    }
}
