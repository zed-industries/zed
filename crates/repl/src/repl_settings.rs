use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};

/// Settings for configuring REPL display and behavior.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, SettingsUi, SettingsKey)]
#[settings_key(key = "repl")]
pub struct ReplSettings {
    /// Maximum number of lines to keep in REPL's scrollback buffer.
    /// Clamped with [4, 256] range.
    ///
    /// Default: 32
    #[serde(default = "default_max_lines")]
    pub max_lines: usize,
    /// Maximum number of columns to keep in REPL's scrollback buffer.
    /// Clamped with [20, 512] range.
    ///
    /// Default: 128
    #[serde(default = "default_max_columns")]
    pub max_columns: usize,
}

impl Settings for ReplSettings {
    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _cx: &mut App) -> anyhow::Result<Self> {
        let mut settings: ReplSettings = sources.json_merge()?;
        settings.max_columns = settings.max_columns.clamp(20, 512);
        settings.max_lines = settings.max_lines.clamp(4, 256);
        Ok(settings)
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}

const DEFAULT_NUM_LINES: usize = 32;
const DEFAULT_NUM_COLUMNS: usize = 128;

fn default_max_lines() -> usize {
    DEFAULT_NUM_LINES
}

fn default_max_columns() -> usize {
    DEFAULT_NUM_COLUMNS
}

impl Default for ReplSettings {
    fn default() -> Self {
        ReplSettings {
            max_lines: DEFAULT_NUM_LINES,
            max_columns: DEFAULT_NUM_COLUMNS,
        }
    }
}
