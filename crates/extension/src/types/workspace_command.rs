/// A workspace command that can be invoked from the command palette or via a keybinding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceCommand {
    /// The unique identifier for this command within the extension.
    pub id: String,
    /// The display name shown in the command palette.
    pub name: String,
    /// An optional description shown in the command palette.
    pub description: Option<String>,
}

/// The result of executing a workspace command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceCommandResult {
    /// Open the file at the given absolute path in the editor.
    OpenFile(String),
    /// Show a picker with multiple candidate file paths for the user to choose from.
    PickAndOpen(Vec<String>),
    /// No action is needed.
    None,
}
