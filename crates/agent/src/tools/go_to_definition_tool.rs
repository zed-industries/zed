use std::sync::Arc;

use agent_client_protocol::schema as acp;
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::symbol_locator::SymbolLocator;
use crate::{AgentTool, ToolCallEventStream, ToolInput};

/// Jumps to the definition of a symbol using the language server.
///
/// Returns the file path and line number of the symbol's definition,
/// along with a snippet of the source code at that location.
///
/// Before using this tool, use read_file or grep to find the exact symbol
/// name and line number of a usage you want to navigate from.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct GoToDefinitionToolInput {
    /// The symbol to find the definition of.
    pub symbol: SymbolLocator,
}

pub struct GoToDefinitionTool {
    project: Entity<Project>,
}

impl GoToDefinitionTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for GoToDefinitionTool {
    type Input = GoToDefinitionToolInput;
    type Output = String;

    const NAME: &'static str = "go_to_definition";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Search
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Go to definition of `{}`", input.symbol.symbol_name).into()
        } else {
            "Go to definition".into()
        }
    }

    fn run(
        self: Arc<Self>,
        _input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Task<Result<String, String>> {
        // TODO: Implement LSP go-to-definition
        Task::ready(Err("Go to definition tool is not yet implemented".into()))
    }
}
