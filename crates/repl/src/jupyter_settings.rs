use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use ui::Pixels;

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum JupyterDockPosition {
    Left,
    #[default]
    Right,
    Bottom,
}

#[derive(Debug, Default)]
pub struct JupyterSettings {
    pub enabled: bool,
    pub dock: JupyterDockPosition,
    pub default_width: Pixels,
    pub kernel_selections: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
pub struct JupyterSettingsContent {
    /// Whether the Jupyter feature is enabled.
    ///
    /// Default: `false`
    enabled: Option<bool>,
    /// Where to dock the Jupyter panel.
    ///
    /// Default: `right`
    dock: Option<JupyterDockPosition>,
    /// Default width in pixels when the jupyter panel is docked to the left or right.
    ///
    /// Default: 640
    pub default_width: Option<f32>,
    /// Default kernels to select for each language.
    ///
    /// Default: `{}`
    pub kernel_selections: Option<HashMap<String, String>>,
}

impl JupyterSettingsContent {
    pub fn set_dock(&mut self, dock: JupyterDockPosition) {
        self.dock = Some(dock);
    }
}

impl Default for JupyterSettingsContent {
    fn default() -> Self {
        JupyterSettingsContent {
            enabled: Some(false),
            dock: Some(JupyterDockPosition::Right),
            default_width: Some(640.0),
            kernel_selections: Some(HashMap::new()),
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

            if let Some(source) = &value.kernel_selections {
                for (k, v) in source {
                    settings.kernel_selections.insert(k.clone(), v.clone());
                }
            }
        }

        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use gpui::{AppContext, UpdateGlobal};
    use settings::SettingsStore;

    use super::*;

    #[gpui::test]
    fn test_deserialize_jupyter_settings(cx: &mut AppContext) {
        let store = settings::SettingsStore::test(cx);
        cx.set_global(store);

        JupyterSettings::register(cx);

        assert_eq!(JupyterSettings::get_global(cx).enabled, false);
        assert_eq!(
            JupyterSettings::get_global(cx).dock,
            JupyterDockPosition::Right
        );
        assert_eq!(
            JupyterSettings::get_global(cx).default_width,
            Pixels::from(640.0)
        );

        // Setting a custom setting through user settings
        SettingsStore::update_global(cx, |store, cx| {
            store
                .set_user_settings(
                    r#"{
                        "jupyter": {
                            "enabled": true,
                            "dock": "left",
                            "default_width": 800.0
                        }
                    }"#,
                    cx,
                )
                .unwrap();
        });

        assert_eq!(JupyterSettings::get_global(cx).enabled, true);
        assert_eq!(
            JupyterSettings::get_global(cx).dock,
            JupyterDockPosition::Left
        );
        assert_eq!(
            JupyterSettings::get_global(cx).default_width,
            Pixels::from(800.0)
        );
    }
}
