//! A module for working with terminals.

use crate::wit::zed::extension::terminal;

pub use crate::wit::zed::extension::terminal::{
    SplitDirection, TerminalContent, TerminalHandle, TerminalOptions,
};

/// A builder for constructing terminal options.
#[derive(Default)]
pub struct TerminalOptionsBuilder {
    cwd: Option<String>,
    env: Vec<(String, String)>,
    command: Option<String>,
    args: Vec<String>,
}

impl TerminalOptionsBuilder {
    /// Creates a new terminal options builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the working directory for the terminal.
    pub fn cwd(mut self, cwd: impl Into<String>) -> Self {
        self.cwd = Some(cwd.into());
        self
    }

    /// Adds an environment variable.
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.push((key.into(), value.into()));
        self
    }

    /// Adds multiple environment variables.
    pub fn envs(
        mut self,
        envs: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        self.env.extend(
            envs.into_iter()
                .map(|(key, value)| (key.into(), value.into())),
        );
        self
    }

    /// Sets the command to run instead of the default shell.
    pub fn command(mut self, command: impl Into<String>) -> Self {
        self.command = Some(command.into());
        self
    }

    /// Adds an argument for the command.
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Adds multiple arguments for the command.
    pub fn args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    /// Builds the terminal options.
    pub fn build(self) -> TerminalOptions {
        TerminalOptions {
            cwd: self.cwd,
            env: self.env,
            command: self.command,
            args: self.args,
        }
    }
}

/// Creates a new terminal in the workspace.
///
/// Requires the `terminal:create` capability.
pub fn create_terminal(options: TerminalOptions) -> Result<TerminalHandle, String> {
    terminal::create_terminal(&options)
}

/// Sends text input to a terminal.
///
/// Requires the `terminal:input` capability.
pub fn send_text(terminal: &TerminalHandle, text: impl Into<String>) -> Result<(), String> {
    terminal::send_text(*terminal, &text.into())
}

/// Sends a special key to a terminal (e.g., "enter", "ctrl-c", "escape").
///
/// Requires the `terminal:input` capability.
pub fn send_key(terminal: &TerminalHandle, key: impl Into<String>) -> Result<(), String> {
    terminal::send_key(*terminal, &key.into())
}

/// Reads the current screen content of a terminal.
///
/// Requires the `terminal:read` capability.
pub fn read_screen(terminal: &TerminalHandle) -> Result<TerminalContent, String> {
    terminal::read_screen(*terminal)
}

/// Splits the terminal pane in the given direction, creating a new terminal.
///
/// Requires the `terminal:create` capability.
pub fn split_terminal(
    terminal: &TerminalHandle,
    direction: SplitDirection,
    options: TerminalOptions,
) -> Result<TerminalHandle, String> {
    terminal::split_terminal(*terminal, direction, &options)
}

/// Closes a terminal.
///
/// Requires the `terminal:close` capability.
pub fn close_terminal(terminal: TerminalHandle) -> Result<(), String> {
    terminal::close_terminal(terminal)
}

/// Lists all terminals in the workspace.
///
/// Requires the `terminal:read` capability.
pub fn list_terminals() -> Result<Vec<TerminalHandle>, String> {
    terminal::list_terminals()
}

/// Gets the terminal's current working directory.
pub fn get_cwd(terminal: &TerminalHandle) -> Result<Option<String>, String> {
    terminal::get_cwd(*terminal)
}

/// Checks if the terminal is idle (waiting for input).
pub fn is_idle(terminal: &TerminalHandle) -> Result<bool, String> {
    terminal::is_idle(*terminal)
}
