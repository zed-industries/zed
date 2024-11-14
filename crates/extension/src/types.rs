mod lsp;
mod slash_command;

pub use lsp::*;
pub use slash_command::*;

/// A list of environment variables.
pub type EnvVars = Vec<(String, String)>;

/// A command.
pub struct Command {
    /// The command to execute.
    pub command: String,
    /// The arguments to pass to the command.
    pub args: Vec<String>,
    /// The environment variables to set for the command.
    pub env: EnvVars,
}
