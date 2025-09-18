use gpui::App;
pub use settings::ImageFileSizeUnit;
use settings::Settings;
use util::MergeFrom;

/// The settings for the image viewer.
#[derive(Clone, Debug, Default)]
pub struct ImageViewerSettings {
    /// The unit to use for displaying image file sizes.
    ///
    /// Default: "binary"
    pub unit: ImageFileSizeUnit,
}

impl Settings for ImageViewerSettings {
    fn from_settings(content: &settings::SettingsContent, _cx: &mut App) -> Self {
        Self {
            unit: content.image_viewer.clone().unwrap().unit.unwrap(),
        }
    }

    fn refine(&mut self, content: &settings::SettingsContent, _cx: &mut App) {
        self.unit.merge_from(
            &content
                .image_viewer
                .as_ref()
                .and_then(|image_viewer| image_viewer.unit),
        );
    }
}
