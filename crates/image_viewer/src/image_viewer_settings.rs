use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

/// The settings for the image viewer.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Default)]
pub struct ImageViewerSettings {
    /// The unit to use for displaying image file sizes.
    ///
    /// Default: "binary"
    #[serde(default)]
    pub unit: ImageFileSizeUnit,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum ImageFileSizeUnit {
    /// Displays file size in binary units (e.g., KiB, MiB).
    #[default]
    Binary,
    /// Displays file size in decimal units (e.g., KB, MB).
    Decimal,
}

impl Settings for ImageViewerSettings {
    const KEY: Option<&'static str> = Some("image_viewer");

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut App,
    ) -> Result<Self, anyhow::Error> {
        SettingsSources::<Self::FileContent>::json_merge_with(
            [sources.default]
                .into_iter()
                .chain(sources.user)
                .chain(sources.server),
        )
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}
