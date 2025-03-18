mod tool_registry;
mod tool_working_set;

use anyhow::Result;
use buffer_diff::BufferDiff;
use collections::{HashMap, HashSet};
use gpui::{App, AppContext, Context, Entity, SharedString, Task};
use language::Buffer;
use language_model::LanguageModelRequestMessage;
use project::Project;
use std::sync::Arc;

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
    /// Buffers that user manually added to the context, and whose content has
    /// changed since the model last saw them.
    stale_buffers_in_context: HashSet<Entity<Buffer>>,
    /// Buffers that we want to notify the model about when they change.
    tracked_buffers: HashMap<Entity<Buffer>, TrackedBuffer>,
}

#[derive(Debug)]
struct TrackedBuffer {
    unreviewed_edit_ids: Vec<clock::Lamport>,
    diff: Entity<BufferDiff>,
    version: clock::Global,
}

impl ActionLog {
    /// Creates a new, empty action log.
    pub fn new() -> Self {
        Self {
            stale_buffers_in_context: HashSet::default(),
            tracked_buffers: HashMap::default(),
        }
    }

    fn track_buffer(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> &mut TrackedBuffer {
        let tracked_buffer = self
            .tracked_buffers
            .entry(buffer.clone())
            .or_insert_with(|| {
                let text_snapshot = buffer.read(cx).text_snapshot();
                TrackedBuffer {
                    unreviewed_edit_ids: Vec::new(),
                    diff: cx.new(|cx| BufferDiff::new(&text_snapshot, cx)),
                    version: buffer.read(cx).version(),
                }
            });
        tracked_buffer.version = buffer.read(cx).version();
        tracked_buffer
    }

    /// Track a buffer as read, so we can notify the model about user edits.
    pub fn buffer_read(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        self.track_buffer(buffer, cx);
    }

    /// Mark a buffer as edited, so we can refresh it in the context
    pub fn buffer_edited(
        &mut self,
        buffer: Entity<Buffer>,
        edit_ids: Vec<clock::Lamport>,
        cx: &mut Context<Self>,
    ) {
        self.stale_buffers_in_context.insert(buffer.clone());

        let tracked_buffer = self.track_buffer(buffer.clone(), cx);
        tracked_buffer
            .unreviewed_edit_ids
            .extend(edit_ids.iter().copied());

        let operations_to_undo = tracked_buffer
            .unreviewed_edit_ids
            .iter()
            .map(|edit_id| (*edit_id, u32::MAX))
            .collect::<HashMap<_, _>>();
        let buffer_without_changes = buffer.update(cx, |buffer, cx| buffer.branch(cx));
        buffer_without_changes.update(cx, |buffer, cx| {
            buffer.undo_operations(operations_to_undo, cx);
        });
        let _ = tracked_buffer.diff.update(cx, |diff, cx| {
            diff.set_base_text(buffer_without_changes, buffer.read(cx).text_snapshot(), cx)
        });
    }

    /// Returns the set of buffers that contain changes that haven't been reviewed by the user.
    pub fn unreviewed_buffers(&self) -> impl '_ + Iterator<Item = Entity<Buffer>> {
        self.tracked_buffers
            .iter()
            .filter(|(_, tracked)| !tracked.unreviewed_edit_ids.is_empty())
            .map(|(buffer, _)| buffer.clone())
    }

    /// Iterate over buffers changed since last read or edited by the model
    pub fn stale_buffers<'a>(&'a self, cx: &'a App) -> impl Iterator<Item = &'a Entity<Buffer>> {
        self.tracked_buffers
            .iter()
            .filter(|(buffer, tracked)| tracked.version != buffer.read(cx).version)
            .map(|(buffer, _)| buffer)
    }

    /// Takes and returns the set of buffers pending refresh, clearing internal state.
    pub fn take_stale_buffers_in_context(&mut self) -> HashSet<Entity<Buffer>> {
        std::mem::take(&mut self.stale_buffers_in_context)
    }
}
