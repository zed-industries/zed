use std::collections::HashMap;

use editor::EditorSettings;
use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

use std::fmt;

#[derive(Default)]
pub struct JupyterSettings {
    pub kernel_selections: HashMap<String, String>,
    pub jupyter_servers: Vec<JupyterServer>,
}
impl fmt::Debug for JupyterSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JupyterSettings")
            .field("kernel_selections", &self.kernel_selections)
            .field("jupyter_servers", &self.jupyter_servers)
            .finish()
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
pub struct JupyterServer {
    pub url: String,
    pub nickname: Option<String>,
}

impl JupyterSettings {
    pub fn enabled(cx: &AppContext) -> bool {
        // In order to avoid a circular dependency between `editor` and `repl` crates,
        // we put the `enable` flag on its settings.
        // This allows the editor to set up context for key bindings/actions.
        EditorSettings::jupyter_enabled(cx)
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
pub struct JupyterSettingsContent {
    /// Default kernels to select for each language.
    ///
    /// Default: `{}`
    pub kernel_selections: Option<HashMap<String, String>>,

    pub jupyter_servers: Option<Vec<JupyterServer>>,
}

impl Default for JupyterSettingsContent {
    fn default() -> Self {
        JupyterSettingsContent {
            kernel_selections: Some(HashMap::new()),
            jupyter_servers: Some(Vec::new()),
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
            if let Some(servers) = &value.jupyter_servers {
                settings.jupyter_servers.append(&mut servers.clone());
            }
        }

        Ok(settings)
    }
}
