use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use ui::Pixels;

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RuntimesDockPosition {
    Left,
    #[default]
    Right,
    Bottom,
}

#[derive(Debug, Default)]
pub struct JupyterSettings {
    pub enabled: bool,
    pub dock: RuntimesDockPosition,
    pub default_width: Pixels,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
pub struct JupyterSettingsContent {
    /// Whether the Runtimes feature is enabled.
    ///
    /// Default: `false`
    enabled: Option<bool>,
    /// Where to dock the runtimes panel.
    ///
    /// Default: `right`
    dock: Option<RuntimesDockPosition>,
    /// Default width in pixels when the runtimes panel is docked to the left or right.
    ///
    /// Default: 640
    pub default_width: Option<f32>,
}

impl Default for JupyterSettingsContent {
    fn default() -> Self {
        JupyterSettingsContent {
            enabled: Some(false),
            dock: Some(RuntimesDockPosition::Right),
            default_width: Some(640.0),
        }
    }
}

impl Settings for JupyterSettings {
    const KEY: Option<&'static str> = Some("jupyter");

    type FileContent = JupyterSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _cx: &mut gpui::AppContext,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut settings = JupyterSettings::default();

        for value in sources.defaults_and_customizations() {
            if let Some(enabled) = value.enabled {
                settings.enabled = enabled;
            }
            if let Some(dock) = value.dock {
                settings.dock = dock;
            }

            if let Some(default_width) = value.default_width {
                settings.default_width = Pixels::from(default_width);
            }
        }

        Ok(settings)
    }
}
