mod action_log;
mod string_tool_output;
mod tool_registry;
mod tool_working_set;

use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::Arc;

use anyhow::Result;
use gpui::{self, App, Entity, EntityId, SharedString, Task};
use icons::IconName;
use language_model::LanguageModelRequestMessage;
use language_model::LanguageModelToolSchemaFormat;
use project::Project;

pub use crate::action_log::*;
pub use crate::string_tool_output::*;
pub use crate::tool_registry::*;
pub use crate::tool_working_set::*;

/// A rendered tool use containing styled markdown elements for UI representation.
#[derive(Clone)]
pub struct RenderedToolUse {
    pub label: EntityId,
    pub input: EntityId,
    pub output: EntityId,
}

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

    /// Returns the icon for the tool.
    fn icon(&self) -> IconName;

    /// Returns the source of the tool.
    fn source(&self) -> ToolSource {
        ToolSource::Native
    }

    /// Returns true iff the tool needs the users's confirmation
    /// before having permission to run.
    fn needs_confirmation(&self) -> bool;

    /// Returns the JSON schema that describes the tool's input.
    fn input_schema(&self, _: LanguageModelToolSchemaFormat) -> serde_json::Value {
        serde_json::Value::Object(serde_json::Map::default())
    }

    /// Returns markdown to be displayed in the UI for this tool.
    fn ui_text(&self, input: &serde_json::Value) -> String;

    /// Runs the tool with the provided input.
    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<Arc<dyn ToolOutput>>>;
}

impl Debug for dyn Tool {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tool").field("name", &self.name()).finish()
    }
}

pub trait ToolOutput: Send + Sync + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + serde::Serialize + for<'de> serde::Deserialize<'de> {
    /// Returns a string that will be given to the model
    /// as the tool output.
    fn response_for_model(&self) -> SharedString;

    /// Returns a custom UI element to render the tool's output.
    /// Returns None by default to indicate that rendering has not yet been
    /// implemented for this tool, and the caller should do some default rendering.
    fn render(
        &self,
        _rendered_tool_use: &RenderedToolUse,
        _window: &mut gpui::Window,
        _cx: &gpui::App,
    ) -> Option<gpui::AnyElement> {
        None
    }
}

impl ToolOutput for SharedString {
    fn response_for_model(&self) -> SharedString {
        self.clone()
    }
}
