use settings::{RegisterSetting, Settings};

/// Settings for configuring REPL display and behavior.
#[derive(Clone, Debug, RegisterSetting)]
pub struct ReplSettings {
    /// Maximum number of lines to keep in REPL's scrollback buffer.
    /// Clamped with [4, 256] range.
    ///
    /// Default: 32
    pub max_lines: usize,
    /// Maximum number of columns to keep in REPL's scrollback buffer.
    /// Clamped with [20, 512] range.
    ///
    /// Default: 128
    pub max_columns: usize,
    /// Whether to show small single-line outputs inline instead of in a block.
    ///
    /// Default: true
    pub inline_output: bool,
    /// Maximum number of characters for an output to be shown inline.
    /// Only applies when `inline_output` is true.
    ///
    /// Default: 50
    pub inline_output_max_length: usize,
}

impl Settings for ReplSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let repl = content.repl.as_ref().unwrap();

        Self {
            max_lines: repl.max_lines.unwrap(),
            max_columns: repl.max_columns.unwrap(),
            inline_output: repl.inline_output.unwrap_or(true),
            inline_output_max_length: repl.inline_output_max_length.unwrap_or(50),
        }
    }
}
