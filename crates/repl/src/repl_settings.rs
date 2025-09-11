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
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.max_number_of_lines == 0 || self.max_number_of_lines > 256 {
            anyhow::bail!(
                "Invalid max_number_of_lines: {}. It must be between 1 and 256.",
                self.max_number_of_lines
            );
        }

        if self.max_number_of_columns == 0 || self.max_number_of_columns > 512 {
            anyhow::bail!(
                "Invalid max_number_of_columns: {}. It must be between 1 and 512.",
                self.max_number_of_columns
            );
        }

        // Ensure minimum usable sizes for a functional REPL
        if self.max_number_of_lines < 4 {
            anyhow::bail!(
                "max_number_of_lines too small: {}. Minimum recommended value is 4.",
                self.max_number_of_lines
            );
        }

        if self.max_number_of_columns < 20 {
            anyhow::bail!(
                "max_number_of_columns too small: {}. Minimum recommended value is 20.",
                self.max_number_of_columns
            );
        }

        Ok(())
    }
}
