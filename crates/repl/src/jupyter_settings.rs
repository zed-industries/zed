use std::collections::HashMap;

use editor::EditorSettings;
use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Debug, Default)]
pub struct JupyterSettings {
    pub kernel_selections: HashMap<String, String>,
}

impl JupyterSettings {
    pub fn enabled(cx: &AppContext) -> bool {
        // In order to avoid a circular dependency between `editor` and `repl` crates,
        // we put the `enable` flag on its settings.
        // This allows the editor to set up context for key bindings/actions.
        EditorSettings::get_global(cx).jupyter.enabled
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
pub struct JupyterSettingsContent {
    /// Default kernels to select for each language.
    ///
    /// Default: `{}`
    pub kernel_selections: Option<HashMap<String, String>>,
}

impl Default for JupyterSettingsContent {
    fn default() -> Self {
        JupyterSettingsContent {
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

        EditorSettings::register(cx);
        JupyterSettings::register(cx);

        assert_eq!(JupyterSettings::enabled(cx), false);

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

        assert_eq!(JupyterSettings::enabled(cx), true);
    }
}
