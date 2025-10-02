use collections::HashMap;

use editor::EditorSettings;
use gpui::App;
use settings::Settings;

#[derive(Debug, Default)]
pub struct JupyterSettings {
    pub kernel_selections: HashMap<String, String>,
}

impl JupyterSettings {
    pub fn enabled(cx: &App) -> bool {
        // In order to avoid a circular dependency between `editor` and `repl` crates,
        // we put the `enable` flag on its settings.
        // This allows the editor to set up context for key bindings/actions.
        EditorSettings::jupyter_enabled(cx)
    }
}

impl Settings for JupyterSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let jupyter = content.editor.jupyter.clone().unwrap();
        Self {
            kernel_selections: jupyter.kernel_selections.unwrap_or_default(),
        }
    }
}
