use std::fmt::Write;
use std::sync::Arc;

use super::symbol_locator::{LocationDisplay, SymbolLocator};
use crate::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema as acp;
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Finds all references to a symbol across the project using the language server.
///
/// Returns a list of locations where the symbol is referenced, including file paths, line numbers, and code snippets for each reference.
///
/// Before using this tool, use read_file or grep to find the exact symbol name and line number.
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
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String, String>> {
        let project = self.project.clone();
        cx.spawn(async move |cx| {
            let input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;

            let resolved = input.symbol.resolve(&project, cx).await?;

            let references_task = project.update(cx, |project, cx| {
                project.references(&resolved.buffer, resolved.position, cx)
            });

            let references = references_task
                .await
                .map_err(|e| format!("Find references failed: {e}"))?
                .unwrap_or_default();

            if references.is_empty() {
                return Ok(format!(
                    "No references found for '{}'.",
                    input.symbol.symbol_name
                ));
            }

            let mut output = format!(
                "Found {} references to `{}`:\n",
                references.len(),
                input.symbol.symbol_name
            );

            for location in &references {
                let display = location
                    .buffer
                    .read_with(cx, |_, cx| LocationDisplay::from_location(location, cx));
                write!(output, "\n## {display}\n").ok();
            }

            Ok(output)
        })
    }
}
