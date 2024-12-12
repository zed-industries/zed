use std::ops::Range;

/// A slash command for use in the Assistant.
#[derive(Debug, Clone)]
pub struct SlashCommand {
    /// The name of the slash command.
    pub name: String,
    /// The description of the slash command.
    pub description: String,
    /// The tooltip text to display for the run button.
    pub tooltip_text: String,
    /// Whether this slash command requires an argument.
    pub requires_argument: bool,
}

/// The output of a slash command.
#[derive(Debug, Clone)]
pub struct SlashCommandOutput {
    /// The text produced by the slash command.
    pub text: String,
    /// The list of sections to show in the slash command placeholder.
    pub sections: Vec<SlashCommandOutputSection>,
}

/// A section in the slash command output.
#[derive(Debug, Clone)]
pub struct SlashCommandOutputSection {
    /// The range this section occupies.
    pub range: Range<usize>,
    /// The label to display in the placeholder for this section.
    pub label: String,
}

/// A completion for a slash command argument.
#[derive(Debug, Clone)]
pub struct SlashCommandArgumentCompletion {
    /// The label to display for this completion.
    pub label: String,
    /// The new text that should be inserted into the command when this completion is accepted.
    pub new_text: String,
    /// Whether the command should be run when accepting this completion.
    pub run_command: bool,
}
