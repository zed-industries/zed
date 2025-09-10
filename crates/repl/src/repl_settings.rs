use gpui::App;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};

use schemars::JsonSchema;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, SettingsUi, SettingsKey)]
#[settings_key(key = "repl")]
pub struct ReplSettings {
    /// Maximum number of lines in the REPL.
    ///
    /// Default: 32
    #[serde(default = "default_max_number_of_lines")]
    pub max_number_of_lines: usize,

    /// Maximum number of columns in the REPL.
    ///
    /// Default: 128
    #[serde(default = "default_max_number_of_columns")]
    pub max_number_of_columns: usize,
}

impl Settings for REPLSettings {
    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _cx: &mut App) -> anyhow::Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}

const DEFAULT_NUM_LINES: usize = 32;
const DEFAULT_NUM_COLUMNS: usize = 128;

fn default_max_number_of_lines() -> usize {
    DEFAULT_NUM_LINES
}

fn default_max_number_of_columns() -> usize {
    DEFAULT_NUM_COLUMNS
}

fn max_number_of_columns() -> usize {
    DEFAULT_NUM_COLUMNS
}

// Optional: implement Default for programmatic instantiation
impl Default for REPLSettings {
    fn default() -> Self {
        REPLSettings {
            max_number_of_lines: DEFAULT_NUM_LINES,
            max_number_of_columns: DEFAULT_NUM_COLUMNS,
        }
    }
}
