mod lsp;
mod slash_command;

use std::ops::Range;

pub use lsp::*;
pub use slash_command::*;

/// A list of environment variables.
pub type EnvVars = Vec<(String, String)>;

/// A command.
#[derive(Debug)]
pub struct Command {
    /// The command to execute.
    pub command: String,
    /// The arguments to pass to the command.
    pub args: Vec<String>,
    /// The environment variables to set for the command.
    pub env: EnvVars,
}

/// A label containing some code.
#[derive(Debug, Clone)]
pub struct CodeLabel {
    /// The source code to parse with Tree-sitter.
    pub code: String,
    /// The spans to display in the label.
    pub spans: Vec<CodeLabelSpan>,
    /// The range of the displayed label to include when filtering.
    pub filter_range: Range<usize>,
}

/// A span within a code label.
#[derive(Debug, Clone)]
pub enum CodeLabelSpan {
    /// A range into the parsed code.
    CodeRange(Range<usize>),
    /// A span containing a code literal.
    Literal(CodeLabelSpanLiteral),
}

/// A span containing a code literal.
#[derive(Debug, Clone)]
pub struct CodeLabelSpanLiteral {
    /// The literal text.
    pub text: String,
    /// The name of the highlight to use for this literal.
    pub highlight_name: Option<String>,
}
