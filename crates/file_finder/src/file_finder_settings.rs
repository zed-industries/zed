use anyhow::Result;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct FileFinderSettings {
    pub file_icons: bool,
    pub window_width: f32,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct FileFinderSettingsContent {
    /// Whether to show file icons in the file finder.
    ///
    /// Default: true
    pub file_icons: Option<bool>,
    /// The width of the file finder window in rem.
    ///
    /// Default: 34
    pub window_width: Option<f32>,
}

impl Settings for FileFinderSettings {
    const KEY: Option<&'static str> = Some("file_finder");

    type FileContent = FileFinderSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut gpui::AppContext) -> Result<Self> {
        let defaults = sources.default;

        let mut this = Self {
            file_icons: defaults.file_icons.unwrap().into(),
            window_width: defaults.window_width.unwrap().into(),
        };

        for value in sources.user.into_iter().chain(sources.release_channel) {
            if let Some(value) = value.file_icons {
                this.file_icons = value.into();
            }

            if let Some(value) = value.window_width {
                this.window_width = value.into();
            }

            this.window_width = this.window_width.clamp(16., 128.);
        }

        Ok(this)
    }
}
