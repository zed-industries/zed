use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Default)]
pub struct ImageViewerSettings {
    #[serde(default)]
    pub unit_type: ImageFileSizeUnitType,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum ImageFileSizeUnitType {
    #[default]
    Binary,
    Decimal,
}

impl Settings for ImageViewerSettings {
    const KEY: Option<&'static str> = Some("image_viewer");

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut App,
    ) -> Result<Self, anyhow::Error> {
        sources.json_merge().or_else(|_| Ok(Self::default()))
    }
}
