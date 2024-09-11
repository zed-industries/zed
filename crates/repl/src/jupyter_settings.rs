use std::collections::HashMap;

use editor::EditorSettings;
use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct JupyterSettings {
    /// Default kernels to select for each language.
    pub kernel_selections: HashMap<String, String>,
}

impl JupyterSettings {
    pub fn enabled(cx: &AppContext) -> bool {
        // In order to avoid a circular dependency between `editor` and `repl` crates,
        // we put the `enable` flag on its settings.
        // This allows the editor to set up context for key bindings/actions.
        EditorSettings::jupyter_enabled(cx)
    }
}

impl Settings for JupyterSettings {
    const KEY: Option<&'static str> = Some("jupyter");

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _cx: &mut gpui::AppContext,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut settings = JupyterSettings::default();

        for value in sources.defaults_and_customizations() {
            for (k, v) in &value.kernel_selections {
                settings.kernel_selections.insert(k.clone(), v.clone());
            }
        }

        Ok(settings)
    }
}
