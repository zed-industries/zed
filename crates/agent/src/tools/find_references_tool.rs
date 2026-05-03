use std::sync::Arc;

use agent_client_protocol::schema as acp;
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::symbol_locator::SymbolLocator;
use crate::{AgentTool, ToolCallEventStream, ToolInput};

/// Finds all references to a symbol across the project using the language server.
///
/// Returns a list of locations where the symbol is referenced, including file paths,
/// line numbers, and code snippets for each reference.
///
/// Before using this tool, use read_file or grep to find the exact symbol
/// name and line number.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct FindReferencesToolInput {
    /// The symbol to find references of.
    pub symbol: SymbolLocator,
}

pub struct FindReferencesTool {
    project: Entity<Project>,
}

impl FindReferencesTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for FindReferencesTool {
    type Input = FindReferencesToolInput;
    type Output = String;

    const NAME: &'static str = "find_references";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Search
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Find references to `{}`", input.symbol.symbol_name).into()
        } else {
            "Find references".into()
        }
    }

    fn run(
        self: Arc<Self>,
        _input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Task<Result<String, String>> {
        // TODO: Implement LSP find-all-references
        Task::ready(Err("Find references tool is not yet implemented".into()))
    }
}
