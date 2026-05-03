use std::sync::Arc;

use agent_client_protocol::schema as acp;
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::symbol_locator::SymbolLocator;
use crate::{AgentTool, ToolCallEventStream, ToolInput};

/// Renames a symbol across the project using the language server.
///
/// This performs a semantic rename, updating all references to the symbol
/// across all files in the project. The language server determines which
/// occurrences to rename based on the symbol's type and scope.
///
/// Before using this tool, use read_file or grep to find the exact symbol
/// name and line number.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct RenameToolInput {
    /// The symbol to rename.
    pub symbol: SymbolLocator,

    /// The new name for the symbol.
    pub new_name: String,
}

pub struct RenameTool {
    project: Entity<Project>,
}

impl RenameTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for RenameTool {
    type Input = RenameToolInput;
    type Output = String;

    const NAME: &'static str = "rename_symbol";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Edit
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!(
                "Rename `{}` to `{}`",
                input.symbol.symbol_name, input.new_name
            )
            .into()
        } else {
            "Rename symbol".into()
        }
    }

    fn run(
        self: Arc<Self>,
        _input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Task<Result<String, String>> {
        // TODO: Implement LSP rename
        Task::ready(Err("Rename tool is not yet implemented".into()))
    }
}
