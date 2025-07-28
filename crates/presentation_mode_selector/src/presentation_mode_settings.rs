use anyhow::Result;
use gpui::{App, Global};
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use theme::FontFamilyName;
use ui::Pixels;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(transparent)]
pub struct PresentationModeSettings {
    pub presentation_modes: Vec<PresentationMode>,
}

impl Default for PresentationModeSettings {
    fn default() -> Self {
        Self {
            presentation_modes: Vec::new(),
        }
    }
}

/// Configuration for a presentation mode
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct PresentationMode {
    /// The name of your presentation mode.
    pub name: String,
    /// The setting overrides associated with your presentation mode.
    pub settings: PresentationModeConfiguration,
}

impl PresentationMode {
    pub fn display_name(presentation_mode: &Option<PresentationMode>) -> String {
        match presentation_mode {
            Some(presentation_mode) => presentation_mode.name.clone(),
            None => "Disabled (Default)".to_string(),
        }
    }
}

/// Settings configuration for presentation mode
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct PresentationModeConfiguration {
    /// The font family to use for buffer text in presentation mode.
    pub buffer_font_family: Option<FontFamilyName>,
    /// The font size to use for buffer text in presentation mode.
    pub buffer_font_size: Option<Pixels>,
    /// The theme to use in presentation mode.
    pub theme: Option<String>,
    /// Whether to enable full screen when using this presentation mode.
    pub full_screen: Option<bool>,
}

impl Default for PresentationModeConfiguration {
    fn default() -> Self {
        Self {
            buffer_font_family: None,
            buffer_font_size: None,
            theme: None,
            full_screen: None,
        }
    }
}

impl Settings for PresentationModeSettings {
    const KEY: Option<&'static str> = Some("presentation_modes");

    type FileContent = Vec<PresentationMode>;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            presentation_modes: sources.json_merge()?,
        })
    }

    fn import_from_vscode(_: &settings::VsCodeSettings, _: &mut Self::FileContent) {}
}

/// Global state for the currently active presentation mode
#[derive(Clone, Debug, PartialEq)]
pub struct ActivePresentationMode {
    pub presentation_mode: PresentationMode,
    pub disabled_mode_is_in_full_screen: bool,
}

impl Global for ActivePresentationMode {}
