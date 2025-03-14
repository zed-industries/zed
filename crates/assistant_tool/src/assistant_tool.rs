mod tool_registry;
mod tool_working_set;

use std::sync::Arc;

use anyhow::Result;
use collections::HashSet;
use gpui::{App, Entity, SharedString, Task};
use language::Buffer;
use language_model::LanguageModelRequestMessage;
use project::Project;

pub use crate::tool_registry::*;
pub use crate::tool_working_set::*;

pub fn init(cx: &mut App) {
    ToolRegistry::default_global(cx);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum ToolSource {
    /// A native tool built-in to Zed.
    Native,
    /// A tool provided by a context server.
    ContextServer { id: SharedString },
}

/// A tool that can be used by a language model.
pub trait Tool: 'static + Send + Sync {
    /// Returns the name of the tool.
    fn name(&self) -> String;

    /// Returns the description of the tool.
    fn description(&self) -> String;

    /// Returns the source of the tool.
    fn source(&self) -> ToolSource {
        ToolSource::Native
    }

    /// Returns the JSON schema that describes the tool's input.
    fn input_schema(&self) -> serde_json::Value {
        serde_json::Value::Object(serde_json::Map::default())
    }

    /// Runs the tool with the provided input.
    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<ToolResult>>;
}

/// Represents the result of a tool execution, including the output text and any buffers affected.
pub struct ToolResult {
    /// The textual output produced by the tool
    pub output: String,
    /// Set of buffers that were modified during tool execution
    pub affected_buffers: HashSet<Entity<Buffer>>,
}

impl ToolResult {
    /// Creates a new tool result with the given output
    pub fn new(output: String) -> Self {
        Self {
            output,
            affected_buffers: HashSet::default(),
        }
    }

    /// Adds a set of affected buffers to this result
    pub fn with_buffers(mut self, buffers: HashSet<Entity<Buffer>>) -> Self {
        self.affected_buffers = buffers;
        self
    }
}

impl From<String> for ToolResult {
    fn from(output: String) -> Self {
        Self::new(output)
    }
}
