mod tool_registry;
mod tool_working_set;

use std::sync::Arc;

use anyhow::Result;
use collections::HashSet;
use gpui::Context;
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
        action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>>;
}

/// Tracks actions performed by tools in a thread
#[derive(Debug)]
pub struct ActionLog {
    changed_buffers: HashSet<Entity<Buffer>>,
    pending_refresh: HashSet<Entity<Buffer>>,
}

impl ActionLog {
    /// Creates a new, empty action log.
    pub fn new() -> Self {
        Self {
            changed_buffers: HashSet::default(),
            pending_refresh: HashSet::default(),
        }
    }

    /// Registers buffers that have changed and need refreshing.
    pub fn notify_buffers_changed(
        &mut self,
        buffers: HashSet<Entity<Buffer>>,
        _cx: &mut Context<Self>,
    ) {
        self.changed_buffers.extend(buffers.clone());
        self.pending_refresh.extend(buffers);
    }

    /// Takes and returns the set of buffers pending refresh, clearing internal state.
    pub fn take_pending_refresh_buffers(&mut self) -> HashSet<Entity<Buffer>> {
        std::mem::take(&mut self.pending_refresh)
    }
}
