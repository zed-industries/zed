use settings::{CellMarkerStyle, RegisterSetting, Settings};

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
    /// How cell markers are interpreted in source files.
    ///
    /// Default: Jupytext
    pub cell_marker_style: CellMarkerStyle,
}

impl Settings for ReplSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let repl = content.repl.as_ref().unwrap();

        Self {
            max_lines: repl.max_lines.unwrap(),
            max_columns: repl.max_columns.unwrap(),
            cell_marker_style: repl.cell_marker_style.unwrap_or_default(),
        }
    }
}
