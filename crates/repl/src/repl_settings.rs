use collections::HashMap;
use settings::{RegisterSetting, Settings};
use task::RevealTarget;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminalReplRevealTarget {
    Dock,
    Center,
}

impl TerminalReplRevealTarget {
    fn from_settings_value(value: Option<&str>) -> Self {
        match value {
            Some(value) if value.eq_ignore_ascii_case("center") => Self::Center,
            _ => Self::Dock,
        }
    }

    pub fn reveal_target(self) -> RevealTarget {
        match self {
            Self::Dock => RevealTarget::Dock,
            Self::Center => RevealTarget::Center,
        }
    }
}

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
    /// Maximum number of lines of output to display before scrolling.
    /// Set to 0 to disable output height limits.
    ///
    /// Default: 0
    pub output_max_height_lines: usize,
    /// Language-specific commands used to start a persistent terminal REPL.
    ///
    /// Keys are normalized to lowercase for case-insensitive matching.
    pub terminal_repl_commands: HashMap<String, String>,
    /// Where to reveal terminal-backed inline REPL sessions.
    ///
    /// Accepted values: "dock", "center".
    ///
    /// Default: dock
    pub terminal_repl_reveal_target: TerminalReplRevealTarget,
}

impl ReplSettings {
    pub fn terminal_repl_command(&self, language_name: &str) -> Option<&str> {
        self.terminal_repl_commands
            .get(&language_name.to_ascii_lowercase())
            .map(|command| command.as_str())
    }
}

impl Settings for ReplSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let repl = content.repl.as_ref().unwrap();
        let terminal_repl_commands = repl
            .terminal_repl_commands
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|(language_name, command)| (language_name.to_ascii_lowercase(), command))
            .collect();

        Self {
            max_lines: repl.max_lines.unwrap(),
            max_columns: repl.max_columns.unwrap(),
            inline_output: repl.inline_output.unwrap_or(true),
            inline_output_max_length: repl.inline_output_max_length.unwrap_or(50),
            output_max_height_lines: repl.output_max_height_lines.unwrap_or(0),
            terminal_repl_commands,
            terminal_repl_reveal_target: TerminalReplRevealTarget::from_settings_value(
                repl.terminal_repl_reveal_target.as_deref(),
            ),
        }
    }
}
