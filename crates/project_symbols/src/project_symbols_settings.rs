use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Clone, settings::SettingsUi)]
pub struct ProjectSymbolsSettings {
    pub width: Option<f32>,
    pub ellipsis_type: EllipsisKind,
}

#[derive(Clone, Copy, Default, Serialize, Deserialize, JsonSchema)]
pub enum EllipsisKind {
    Start,
    End,
    #[default]
    None,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ProjectSymbolsSettingsContent {
    /// The width, in Rems, of this panel.
    ///
    /// Default: 34 rems
    #[serde(default)]
    pub width: Option<f32>,

    /// Determines if paths to files will be ellipsized.
    ///
    /// Default: None
    #[serde(default)]
    pub ellipsis_type: Option<EllipsisKind>,
}

impl Settings for ProjectSymbolsSettings {
    const KEY: Option<&'static str> = Some("project_symbols");

    type FileContent = ProjectSymbolsSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        let result: ProjectSymbolsSettingsContent = sources.json_merge()?;

        Ok(Self {
            width: result.width,
            ellipsis_type: match result.ellipsis_type {
                Some(value) => value,
                None => EllipsisKind::None,
            },
        })
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}
