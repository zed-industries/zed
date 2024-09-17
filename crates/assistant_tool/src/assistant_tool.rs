mod tool_registry;

use std::sync::Arc;

use anyhow::Result;
use gpui::{AppContext, Task, WeakView, WindowContext};
use workspace::Workspace;

pub use tool_registry::*;

pub fn init(cx: &mut AppContext) {
    ToolRegistry::default_global(cx);
}

/// A tool that can be used by a language model.
pub trait Tool: 'static + Send + Sync {
    /// Returns the name of the tool.
    fn name(&self) -> String;

    /// Returns the description of the tool.
    fn description(&self) -> String;

    /// Returns the JSON schema that describes the tool's input.
    fn input_schema(&self) -> serde_json::Value {
        serde_json::Value::Object(serde_json::Map::default())
    }

    /// Runs the tool with the provided input.
    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        workspace: WeakView<Workspace>,
        cx: &mut WindowContext,
    ) -> Task<Result<String>>;
}
