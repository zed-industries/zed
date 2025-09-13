use gpui::App;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};

/// Settings for configuring REPL display and behavior.
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

impl Settings for ReplSettings {
    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _cx: &mut App) -> anyhow::Result<Self> {
        let settings: ReplSettings = sources.json_merge()?;
        settings.validate()?;
        Ok(settings)
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}

const DEFAULT_NUM_LINES: usize = 32;
const DEFAULT_NUM_COLUMNS: usize = 128;
const DEFAULT_MIN_NUM_LINES: usize = 4;
const DEFAULT_MIN_NUM_COLUMNS: usize = 20;
const DEFAULT_MAX_NUM_LINES: usize = 256;
const DEFAULT_MAX_NUM_COLUMNS: usize = 512;

fn default_max_number_of_lines() -> usize {
    DEFAULT_NUM_LINES
}

fn default_max_number_of_columns() -> usize {
    DEFAULT_NUM_COLUMNS
}

impl Default for ReplSettings {
    fn default() -> Self {
        ReplSettings {
            max_number_of_lines: DEFAULT_NUM_LINES,
            max_number_of_columns: DEFAULT_NUM_COLUMNS,
        }
    }
}

impl ReplSettings {
    /// Validates the settings to ensure no unreasonable values are set.
    pub fn validate(&mut self) -> anyhow::Result<()> {
        self.max_number_of_lines = self.validate_range(
            self.max_number_of_lines,
            DEFAULT_MIN_NUM_LINES,
            DEFAULT_MAX_NUM_LINES,
            "max_number_of_lines",
        );

        self.max_number_of_columns = self.validate_range(
            self.max_number_of_columns,
            DEFAULT_MIN_NUM_COLUMNS,
            DEFAULT_MAX_NUM_COLUMNS,
            "max_number_of_columns",
        );

        Ok(())
    }

    /// Helper function to validate and adjust a value within a range.
    fn validate_range(&self, value: usize, min: usize, max: usize, field_name: &str) -> usize {
        if value < min {
            log::warn!(
                "{} too small: {}. Minimum recommended value is {}. Defaulting to {}.",
                field_name,
                value,
                min,
                min
            );
            min
        } else if value > max {
            log::warn!(
                "{} too large: {}. Maximum allowed value is {}. Defaulting to {}.",
                field_name,
                value,
                max,
                max
            );
            max
        } else {
            value
        }
    }
}
