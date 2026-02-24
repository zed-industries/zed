use super::edit_file_tool::EditFileTool;
use super::restore_file_from_disk_tool::RestoreFileFromDiskTool;
use super::save_file_tool::SaveFileTool;
use crate::{
    AgentTool, Templates, Thread, ToolCallEventStream, ToolInput,
    edit_agent::streaming_fuzzy_matcher::StreamingFuzzyMatcher,
};
use acp_thread::Diff;
use agent_client_protocol::{self as acp, ToolCallLocation, ToolCallUpdateFields};
use anyhow::{Context as _, Result, anyhow};
use collections::HashSet;
use futures::FutureExt as _;
use gpui::{App, AppContext, AsyncApp, Entity, Task, WeakEntity};
use language::LanguageRegistry;
use language::language_settings::{self, FormatOnSave};
use language_model::LanguageModelToolResultContent;
use project::lsp_store::{FormatTrigger, LspFormatTarget};
use project::{Project, ProjectPath};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;
use text::BufferSnapshot;
use ui::SharedString;
use util::ResultExt;
use util::rel_path::RelPath;

const DEFAULT_UI_TEXT: &str = "Editing file";

/// This is a tool for creating a new file or editing an existing file. For moving or renaming files, you should generally use the `move_path` tool instead.
///
/// Before using this tool:
///
/// 1. Use the `read_file` tool to understand the file's contents and context
///
/// 2. Verify the directory path is correct (only applicable when creating new files):
///    - Use the `list_directory` tool to verify the parent directory exists and is the correct location
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StreamingEditFileToolInput {
    /// A one-line, user-friendly markdown description of the edit. This will be shown in the UI.
    ///
    /// Be terse, but also descriptive in what you want to achieve with this edit. Avoid generic instructions.
    ///
    /// NEVER mention the file path in this description.
    ///
    /// <example>Fix API endpoint URLs</example>
    /// <example>Update copyright year in `page_footer`</example>
    ///
    /// Make sure to include this field before all the others in the input object so that we can display it immediately.
    pub display_description: String,

    /// The full path of the file to create or modify in the project.
    ///
    /// WARNING: When specifying which file path need changing, you MUST start each path with one of the project's root directories.
    ///
    /// The following examples assume we have two root directories in the project:
    /// - /a/b/backend
    /// - /c/d/frontend
    ///
    /// <example>
    /// `backend/src/main.rs`
    ///
    /// Notice how the file path starts with `backend`. Without that, the path would be ambiguous and the call would fail!
    /// </example>
    ///
    /// <example>
    /// `frontend/db.js`
    /// </example>
    pub path: PathBuf,

    /// The mode of operation on the file. Possible values:
    /// - 'create': Create a new file if it doesn't exist. Requires 'content' field.
    /// - 'overwrite': Replace the entire contents of an existing file. Requires 'content' field.
    /// - 'edit': Make granular edits to an existing file. Requires 'edits' field.
    ///
    /// When a file already exists or you just created it, prefer editing it as opposed to recreating it from scratch.
    pub mode: StreamingEditFileMode,

    /// The complete content for the new file (required for 'create' and 'overwrite' modes).
    /// This field should contain the entire file content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// List of edit operations to apply sequentially (required for 'edit' mode).
    /// Each edit finds `old_text` in the file and replaces it with `new_text`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edits: Option<Vec<EditOperation>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StreamingEditFileMode {
    /// Create a new file if it doesn't exist
    Create,
    /// Replace the entire contents of an existing file
    Overwrite,
    /// Make granular edits to an existing file
    Edit,
}

/// A single edit operation that replaces old text with new text
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditOperation {
    /// The exact text to find in the file. This will be matched using fuzzy matching
    /// to handle minor differences in whitespace or formatting.
    pub old_text: String,
    /// The text to replace it with
    pub new_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
struct StreamingEditFileToolPartialInput {
    #[serde(default)]
    path: String,
    #[serde(default)]
    display_description: String,
}

#[derive(Default, Deserialize)]
struct StreamingPartialInput {
    #[serde(default)]
    display_description: Option<String>,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    mode: Option<StreamingEditFileMode>,
    #[serde(default)]
    #[allow(dead_code)]
    content: Option<String>,
    #[serde(default)]
    edits: Option<Vec<PartialEditOperation>>,
}

#[derive(Default, Deserialize)]
struct PartialEditOperation {
    #[serde(default)]
    old_text: Option<String>,
    #[serde(default)]
    new_text: Option<String>,
}

struct StreamingEditState {
    title_set: bool,
    path_resolved: bool,
    diff_created: bool,
    dirty_check_done: bool,

    project_path: Option<ProjectPath>,
    abs_path: Option<PathBuf>,
    authorize_task: Option<Task<Result<()>>>,
    buffer_open_task: Option<Task<Result<Entity<language::Buffer>>>>,
    buffer: Option<Entity<language::Buffer>>,
    diff: Option<Entity<Diff>>,
    old_text: Option<Arc<String>>,

    finalize_diff_guard: Option<util::Deferred<Box<dyn FnOnce()>>>,

    edit_tracker: EditTracker,
}

struct EditTracker {
    applied_count: usize,
    in_progress_matcher: Option<StreamingFuzzyMatcher>,
    last_old_text_len: usize,
}

impl EditTracker {
    fn new() -> Self {
        Self {
            applied_count: 0,
            in_progress_matcher: None,
            last_old_text_len: 0,
        }
    }
}

impl StreamingEditState {
    fn new() -> Self {
        Self {
            title_set: false,
            path_resolved: false,
            diff_created: false,
            dirty_check_done: false,
            project_path: None,
            abs_path: None,
            authorize_task: None,
            buffer_open_task: None,
            buffer: None,
            diff: None,
            old_text: None,
            finalize_diff_guard: None,
            edit_tracker: EditTracker::new(),
        }
    }

    fn process(
        &mut self,
        partial: StreamingPartialInput,
        tool: &StreamingEditFileTool,
        event_stream: &ToolCallEventStream,
        cx: &mut AsyncApp,
    ) -> Result<(), StreamingEditFileToolOutput> {
        // Update title greedily from display_description
        if let Some(description) = &partial.display_description {
            let trimmed = description.trim();
            if !trimmed.is_empty() {
                event_stream.update_fields(ToolCallUpdateFields::new().title(trimmed));
                self.title_set = true;
            }
        }

        // When `mode` appears, `path` is guaranteed complete — kick off path-dependent work
        if !self.path_resolved && partial.mode.is_some() {
            if let Some(path_str) = &partial.path {
                let trimmed_path = path_str.trim();
                if !trimmed_path.is_empty() {
                    self.resolve_path_and_start_work(trimmed_path, tool, event_stream, cx);
                }
            }
        }

        // Try to resolve the buffer open task if it's pending
        self.try_resolve_buffer(tool, event_stream, cx)?;

        // Process edits incrementally if we have a buffer and edits are streaming
        if self.buffer.is_some() {
            if let Some(edits) = &partial.edits {
                self.process_streaming_edits(edits, tool, event_stream, cx)?;
            }
        }

        Ok(())
    }

    fn try_resolve_buffer(
        &mut self,
        tool: &StreamingEditFileTool,
        event_stream: &ToolCallEventStream,
        cx: &mut AsyncApp,
    ) -> Result<(), StreamingEditFileToolOutput> {
        if self.buffer.is_some() {
            return Ok(());
        }

        let task = match self.buffer_open_task.as_mut() {
            Some(task) => task,
            None => return Ok(()),
        };

        // Poll the buffer open task without blocking
        let poll_result = cx.update(|_cx| {
            task.poll_unpin(&mut std::task::Context::from_waker(
                futures::task::noop_waker_ref(),
            ))
        });
        if let std::task::Poll::Ready(result) = poll_result {
            self.buffer_open_task = None;
            let buffer = result.map_err(|e| StreamingEditFileToolOutput::Error {
                error: format!("Failed to open buffer: {e}"),
            })?;
            self.buffer = Some(buffer.clone());

            // Create the Diff entity now that the buffer is available
            if !self.diff_created {
                let diff = cx.new(|cx| Diff::new(buffer.clone(), cx));
                event_stream.update_diff(diff.clone());
                let finalize_guard = util::defer(Box::new({
                    let diff = diff.downgrade();
                    let mut cx = cx.clone();
                    move || {
                        diff.update(&mut cx, |diff, cx| diff.finalize(cx)).ok();
                    }
                }) as Box<dyn FnOnce()>);
                self.diff = Some(diff);
                self.finalize_diff_guard = Some(finalize_guard);
                self.diff_created = true;

                // Capture original buffer text before any edits are applied
                let old_text = buffer.read_with(cx, |buffer, _cx| Arc::new(buffer.text()));
                self.old_text = Some(old_text);
            }

            // Run dirty/mtime check before any edits are applied
            if !self.dirty_check_done {
                self.check_dirty_mtime(&buffer, tool, cx)?;
                self.dirty_check_done = true;
            }
        }

        Ok(())
    }

    fn check_dirty_mtime(
        &self,
        buffer: &Entity<language::Buffer>,
        tool: &StreamingEditFileTool,
        cx: &mut AsyncApp,
    ) -> Result<(), StreamingEditFileToolOutput> {
        if let Some(abs_path) = self.abs_path.as_ref() {
            let check_result = tool.thread.update(cx, |thread, cx| {
                let last_read = thread.file_read_times.get(abs_path).copied();
                let current = buffer
                    .read(cx)
                    .file()
                    .and_then(|file| file.disk_state().mtime());
                let dirty = buffer.read(cx).is_dirty();
                let has_save = thread.has_tool(SaveFileTool::NAME);
                let has_restore = thread.has_tool(RestoreFileFromDiskTool::NAME);
                (last_read, current, dirty, has_save, has_restore)
            });

            let Ok((last_read_mtime, current_mtime, is_dirty, has_save_tool, has_restore_tool)) =
                check_result
            else {
                return Ok(());
            };

            if is_dirty {
                let message = match (has_save_tool, has_restore_tool) {
                    (true, true) => {
                        "This file has unsaved changes. Ask the user whether they want to keep or discard those changes. \
                         If they want to keep them, ask for confirmation then use the save_file tool to save the file, then retry this edit. \
                         If they want to discard them, ask for confirmation then use the restore_file_from_disk tool to restore the on-disk contents, then retry this edit."
                    }
                    (true, false) => {
                        "This file has unsaved changes. Ask the user whether they want to keep or discard those changes. \
                         If they want to keep them, ask for confirmation then use the save_file tool to save the file, then retry this edit. \
                         If they want to discard them, ask the user to manually revert the file, then inform you when it's ok to proceed."
                    }
                    (false, true) => {
                        "This file has unsaved changes. Ask the user whether they want to keep or discard those changes. \
                         If they want to keep them, ask the user to manually save the file, then inform you when it's ok to proceed. \
                         If they want to discard them, ask for confirmation then use the restore_file_from_disk tool to restore the on-disk contents, then retry this edit."
                    }
                    (false, false) => {
                        "This file has unsaved changes. Ask the user whether they want to keep or discard those changes, \
                         then ask them to save or revert the file manually and inform you when it's ok to proceed."
                    }
                };
                return Err(StreamingEditFileToolOutput::Error {
                    error: message.to_string(),
                });
            }

            if let (Some(last_read), Some(current)) = (last_read_mtime, current_mtime) {
                if current != last_read {
                    return Err(StreamingEditFileToolOutput::Error {
                        error: "The file has been modified since you last read it. \
                             Please read the file again to get the current state before editing it."
                            .to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    fn process_streaming_edits(
        &mut self,
        edits: &[PartialEditOperation],
        tool: &StreamingEditFileTool,
        event_stream: &ToolCallEventStream,
        cx: &mut AsyncApp,
    ) -> Result<(), StreamingEditFileToolOutput> {
        let buffer = match &self.buffer {
            Some(b) => b.clone(),
            None => return Ok(()),
        };
        let diff = match &self.diff {
            Some(d) => d.clone(),
            None => return Ok(()),
        };

        if edits.is_empty() {
            return Ok(());
        }

        // Edits at indices applied_count..edits.len()-1 are newly complete
        // (a subsequent edit exists, proving the LLM moved on).
        // The last edit (edits.len()-1) is potentially still in progress.
        let completed_count = edits.len().saturating_sub(1);

        // Apply newly-complete edits
        while self.edit_tracker.applied_count < completed_count {
            let edit_index = self.edit_tracker.applied_count;
            let partial_edit = &edits[edit_index];

            let old_text = match &partial_edit.old_text {
                Some(t) => t.clone(),
                None => {
                    self.edit_tracker.applied_count += 1;
                    continue;
                }
            };
            let new_text = partial_edit.new_text.clone().unwrap_or_default();

            // Reset in-progress matcher since this edit is now complete
            self.edit_tracker.in_progress_matcher = None;
            self.edit_tracker.last_old_text_len = 0;

            // Take a fresh snapshot (reflects all previously-applied edits)
            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

            // Resolve the edit using StreamingFuzzyMatcher
            let edit_op = EditOperation {
                old_text: old_text.clone(),
                new_text: new_text.clone(),
            };
            let resolve_result = resolve_edit(&snapshot, &edit_op);

            match resolve_result {
                Ok(Some((range, new_text))) => {
                    // Reveal the range in the diff view
                    let (start_anchor, end_anchor) = buffer.read_with(cx, |buffer, _cx| {
                        (
                            buffer.anchor_before(range.start),
                            buffer.anchor_after(range.end),
                        )
                    });
                    diff.update(cx, |card, cx| {
                        card.reveal_range(start_anchor..end_anchor, cx)
                    });

                    // Emit location for this edit
                    if let Some(abs_path) = self.abs_path.clone() {
                        let line = snapshot.offset_to_point(range.start).row;
                        event_stream.update_fields(
                            ToolCallUpdateFields::new()
                                .locations(vec![ToolCallLocation::new(abs_path).line(Some(line))]),
                        );
                    }

                    // Apply the edit and report to action_log in the same effect cycle
                    let action_log_result = tool
                        .thread
                        .read_with(cx, |thread, _cx| thread.action_log().clone());
                    if let Ok(action_log) = action_log_result {
                        // On the first edit, mark the buffer as read
                        if self.edit_tracker.applied_count == 0 {
                            action_log.update(cx, |log, cx| {
                                log.buffer_read(buffer.clone(), cx);
                            });
                        }

                        cx.update(|cx| {
                            buffer.update(cx, |buffer, cx| {
                                buffer.edit([(range, new_text.as_str())], None, cx);
                            });
                            action_log.update(cx, |log, cx| {
                                log.buffer_edited(buffer.clone(), cx);
                            });
                        });
                    }
                }
                Ok(None) => {
                    return Err(StreamingEditFileToolOutput::Error {
                        error: format!(
                            "Could not find matching text for edit at index {}. \
                             The old_text did not match any content in the file. \
                             Please read the file again to get the current content.",
                            edit_index
                        ),
                    });
                }
                Err(ranges) => {
                    let snapshot_ref = &snapshot;
                    let lines = ranges
                        .iter()
                        .map(|r| (snapshot_ref.offset_to_point(r.start).row + 1).to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    return Err(StreamingEditFileToolOutput::Error {
                        error: format!(
                            "Edit {} matched multiple locations in the file at lines: {}. \
                             Please provide more context in old_text to uniquely identify the location.",
                            edit_index, lines
                        ),
                    });
                }
            }

            self.edit_tracker.applied_count += 1;
        }

        // Feed the in-progress last edit's old_text to the matcher for live preview
        let last_index = edits.len() - 1;
        if let Some(partial_edit) = edits.last() {
            if let Some(old_text) = &partial_edit.old_text {
                let old_text_len = old_text.len();
                if old_text_len > self.edit_tracker.last_old_text_len {
                    let new_chunk = &old_text[self.edit_tracker.last_old_text_len..];

                    let matcher = self
                        .edit_tracker
                        .in_progress_matcher
                        .get_or_insert_with(|| {
                            let snapshot =
                                buffer.read_with(cx, |buffer, _cx| buffer.text_snapshot());
                            StreamingFuzzyMatcher::new(snapshot)
                        });

                    if let Some(match_range) = matcher.push(new_chunk, None) {
                        // Show live match preview in diff view
                        let (start_anchor, end_anchor) = buffer.read_with(cx, |buffer, _cx| {
                            (
                                buffer.anchor_before(match_range.start),
                                buffer.anchor_after(match_range.end),
                            )
                        });
                        diff.update(cx, |card, cx| {
                            card.reveal_range(start_anchor..end_anchor, cx)
                        });
                    }

                    self.edit_tracker.last_old_text_len = old_text_len;
                }

                // If new_text has appeared on the last edit and there are no more
                // edits coming (this is still a partial, so we can't be sure this
                // is truly the last edit yet), don't apply — finalization handles it.
                let _ = last_index;
            }
        }

        Ok(())
    }

    fn resolve_path_and_start_work(
        &mut self,
        path_str: &str,
        tool: &StreamingEditFileTool,
        event_stream: &ToolCallEventStream,
        cx: &mut AsyncApp,
    ) {
        let path = PathBuf::from(path_str);

        let (project_path, abs_path, authorize, buffer_open) = cx.update(|cx| {
            let project = tool.project.clone();
            let project_ref = project.read(cx);

            let project_path = project_ref.find_project_path(&path, cx);

            let abs_path = project_path
                .as_ref()
                .and_then(|pp| project.read(cx).absolute_path(pp, cx));

            if let Some(abs_path) = abs_path.clone() {
                event_stream.update_fields(
                    ToolCallUpdateFields::new().locations(vec![ToolCallLocation::new(abs_path)]),
                );
            }

            // Kick off authorization
            let authorize = super::tool_permissions::authorize_file_edit(
                EditFileTool::NAME,
                &path,
                "",
                &tool.thread,
                event_stream,
                cx,
            );

            // Kick off buffer open if we have a project path
            let buffer_open = project_path
                .as_ref()
                .map(|pp| project.update(cx, |project, cx| project.open_buffer(pp.clone(), cx)));

            (project_path, abs_path, authorize, buffer_open)
        });

        self.project_path = project_path;
        self.abs_path = abs_path;
        self.authorize_task = Some(authorize);
        self.buffer_open_task = buffer_open;
        self.path_resolved = true;
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StreamingEditFileToolOutput {
    Success {
        #[serde(alias = "original_path")]
        input_path: PathBuf,
        new_text: String,
        old_text: Arc<String>,
        #[serde(default)]
        diff: String,
    },
    Error {
        error: String,
    },
}

impl std::fmt::Display for StreamingEditFileToolOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamingEditFileToolOutput::Success {
                diff, input_path, ..
            } => {
                if diff.is_empty() {
                    write!(f, "No edits were made.")
                } else {
                    write!(
                        f,
                        "Edited {}:\n\n```diff\n{diff}\n```",
                        input_path.display()
                    )
                }
            }
            StreamingEditFileToolOutput::Error { error } => write!(f, "{error}"),
        }
    }
}

impl From<StreamingEditFileToolOutput> for LanguageModelToolResultContent {
    fn from(output: StreamingEditFileToolOutput) -> Self {
        output.to_string().into()
    }
}

pub struct StreamingEditFileTool {
    thread: WeakEntity<Thread>,
    language_registry: Arc<LanguageRegistry>,
    project: Entity<Project>,
    #[allow(dead_code)]
    templates: Arc<Templates>,
}

impl StreamingEditFileTool {
    pub fn new(
        project: Entity<Project>,
        thread: WeakEntity<Thread>,
        language_registry: Arc<LanguageRegistry>,
        templates: Arc<Templates>,
    ) -> Self {
        Self {
            project,
            thread,
            language_registry,
            templates,
        }
    }

    pub fn with_thread(&self, new_thread: WeakEntity<Thread>) -> Self {
        Self {
            project: self.project.clone(),
            thread: new_thread,
            language_registry: self.language_registry.clone(),
            templates: self.templates.clone(),
        }
    }

    fn authorize(
        &self,
        input: &StreamingEditFileToolInput,
        event_stream: &ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<()>> {
        super::tool_permissions::authorize_file_edit(
            EditFileTool::NAME,
            &input.path,
            &input.display_description,
            &self.thread,
            event_stream,
            cx,
        )
    }
}

impl AgentTool for StreamingEditFileTool {
    type Input = StreamingEditFileToolInput;
    type Output = StreamingEditFileToolOutput;

    const NAME: &'static str = "streaming_edit_file";

    fn supports_input_streaming() -> bool {
        true
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Edit
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        cx: &mut App,
    ) -> SharedString {
        match input {
            Ok(input) => self
                .project
                .read(cx)
                .find_project_path(&input.path, cx)
                .and_then(|project_path| {
                    self.project
                        .read(cx)
                        .short_full_path_for_project_path(&project_path, cx)
                })
                .unwrap_or(input.path.to_string_lossy().into_owned())
                .into(),
            Err(raw_input) => {
                if let Some(input) =
                    serde_json::from_value::<StreamingEditFileToolPartialInput>(raw_input).ok()
                {
                    let path = input.path.trim();
                    if !path.is_empty() {
                        return self
                            .project
                            .read(cx)
                            .find_project_path(&input.path, cx)
                            .and_then(|project_path| {
                                self.project
                                    .read(cx)
                                    .short_full_path_for_project_path(&project_path, cx)
                            })
                            .unwrap_or(input.path)
                            .into();
                    }

                    let description = input.display_description.trim();
                    if !description.is_empty() {
                        return description.to_string().into();
                    }
                }

                DEFAULT_UI_TEXT.into()
            }
        }
    }

    fn run(
        self: Arc<Self>,
        mut input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        cx.spawn(async move |cx: &mut AsyncApp| {
            // === Phase 1: Process partials, progressively set up ===
            let mut state = StreamingEditState::new();

            loop {
                futures::select! {
                    partial = input.recv_partial().fuse() => {
                        let Some(partial_value) = partial else { break };
                        if let Ok(parsed) = serde_json::from_value::<StreamingPartialInput>(partial_value) {
                            state.process(parsed, &self, &event_stream, cx)?;
                        }
                    }
                    _ = event_stream.cancelled_by_user().fuse() => {
                        return Err(StreamingEditFileToolOutput::Error {
                            error: "Edit cancelled by user".to_string(),
                        });
                    }
                }
            }

            // === Phase 2: Final input arrived — finalize ===
            let final_input =
                input
                    .recv()
                    .await
                    .map_err(|e| StreamingEditFileToolOutput::Error {
                        error: format!("Failed to receive tool input: {e}"),
                    })?;

            self.finalize(state, final_input, &event_stream, cx).await
        })
    }

    fn replay(
        &self,
        _input: Self::Input,
        output: Self::Output,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Result<()> {
        match output {
            StreamingEditFileToolOutput::Success {
                input_path,
                old_text,
                new_text,
                ..
            } => {
                event_stream.update_diff(cx.new(|cx| {
                    Diff::finalized(
                        input_path.to_string_lossy().into_owned(),
                        Some(old_text.to_string()),
                        new_text,
                        self.language_registry.clone(),
                        cx,
                    )
                }));
                Ok(())
            }
            StreamingEditFileToolOutput::Error { .. } => Ok(()),
        }
    }
}

impl StreamingEditFileTool {
    async fn finalize(
        &self,
        mut state: StreamingEditState,
        input: StreamingEditFileToolInput,
        event_stream: &ToolCallEventStream,
        cx: &mut AsyncApp,
    ) -> Result<StreamingEditFileToolOutput, StreamingEditFileToolOutput> {
        let project = self
            .thread
            .read_with(cx, |thread, _cx| thread.project().clone())
            .map_err(|_| StreamingEditFileToolOutput::Error {
                error: "thread was dropped".to_string(),
            })?;

        // Resolve path if not already done during streaming.
        // During streaming we only did find_project_path (for location updates),
        // but finalization needs the full resolve_path validation (e.g. checking
        // the file exists for edit/overwrite mode, or parent exists for create mode).
        let (project_path, abs_path) = cx.update(|cx| {
            let project_path = resolve_path(&input, project.clone(), cx).map_err(|err| {
                StreamingEditFileToolOutput::Error {
                    error: err.to_string(),
                }
            })?;

            let abs_path = project.read(cx).absolute_path(&project_path, cx);
            if !state.path_resolved {
                if let Some(abs_path) = abs_path.clone() {
                    event_stream.update_fields(
                        ToolCallUpdateFields::new()
                            .locations(vec![ToolCallLocation::new(abs_path)]),
                    );
                }
            }

            Ok::<_, StreamingEditFileToolOutput>((project_path, abs_path))
        })?;

        // Await or start authorization
        let authorize = if let Some(task) = state.authorize_task.take() {
            task
        } else {
            cx.update(|cx| self.authorize(&input, event_stream, cx))
        };

        let result: anyhow::Result<StreamingEditFileToolOutput> = async {
            authorize.await?;

            // Await the already-started buffer open, or start a new one.
            // The buffer may already be resolved during streaming.
            let buffer = if let Some(buffer) = state.buffer.take() {
                buffer
            } else if let Some(task) = state.buffer_open_task.take() {
                task.await?
            } else {
                project
                    .update(cx, |project, cx| {
                        project.open_buffer(project_path.clone(), cx)
                    })
                    .await?
            };

            // Run dirty/mtime check if not already done during streaming
            if !state.dirty_check_done {
                if let Some(abs_path) = abs_path.as_ref() {
                    let (last_read_mtime, current_mtime, is_dirty, has_save_tool, has_restore_tool) =
                        self.thread.update(cx, |thread, cx| {
                            let last_read = thread.file_read_times.get(abs_path).copied();
                            let current = buffer
                                .read(cx)
                                .file()
                                .and_then(|file| file.disk_state().mtime());
                            let dirty = buffer.read(cx).is_dirty();
                            let has_save = thread.has_tool(SaveFileTool::NAME);
                            let has_restore = thread.has_tool(RestoreFileFromDiskTool::NAME);
                            (last_read, current, dirty, has_save, has_restore)
                        })?;

                    if is_dirty {
                        let message = match (has_save_tool, has_restore_tool) {
                            (true, true) => {
                                "This file has unsaved changes. Ask the user whether they want to keep or discard those changes. \
                                 If they want to keep them, ask for confirmation then use the save_file tool to save the file, then retry this edit. \
                                 If they want to discard them, ask for confirmation then use the restore_file_from_disk tool to restore the on-disk contents, then retry this edit."
                            }
                            (true, false) => {
                                "This file has unsaved changes. Ask the user whether they want to keep or discard those changes. \
                                 If they want to keep them, ask for confirmation then use the save_file tool to save the file, then retry this edit. \
                                 If they want to discard them, ask the user to manually revert the file, then inform you when it's ok to proceed."
                            }
                            (false, true) => {
                                "This file has unsaved changes. Ask the user whether they want to keep or discard those changes. \
                                 If they want to keep them, ask the user to manually save the file, then inform you when it's ok to proceed. \
                                 If they want to discard them, ask for confirmation then use the restore_file_from_disk tool to restore the on-disk contents, then retry this edit."
                            }
                            (false, false) => {
                                "This file has unsaved changes. Ask the user whether they want to keep or discard those changes, \
                                 then ask them to save or revert the file manually and inform you when it's ok to proceed."
                            }
                        };
                        anyhow::bail!("{}", message);
                    }

                    if let (Some(last_read), Some(current)) = (last_read_mtime, current_mtime) {
                        if current != last_read {
                            anyhow::bail!(
                                "The file {} has been modified since you last read it. \
                                 Please read the file again to get the current state before editing it.",
                                input.path.display()
                            );
                        }
                    }
                }
            }

            // Use existing diff from streaming, or create a new one
            let diff = if let Some(diff) = state.diff.take() {
                diff
            } else {
                let diff = cx.new(|cx| Diff::new(buffer.clone(), cx));
                event_stream.update_diff(diff.clone());
                diff
            };

            // Ensure the finalize guard exists. If created during streaming,
            // take ownership so it persists through finalization.
            let _finalize_diff = if let Some(guard) = state.finalize_diff_guard.take() {
                guard
            } else {
                util::defer(Box::new({
                    let diff = diff.downgrade();
                    let mut cx = cx.clone();
                    move || {
                        diff.update(&mut cx, |diff, cx| diff.finalize(cx)).ok();
                    }
                }) as Box<dyn FnOnce()>)
            };

            // Use the old_text captured during streaming (before any edits were
            // applied), or capture it now if no streaming edits occurred.
            let old_text = if let Some(old_text) = state.old_text.take() {
                old_text
            } else {
                let old_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
                cx.background_spawn({
                    let old_snapshot = old_snapshot.clone();
                    async move { Arc::new(old_snapshot.text()) }
                })
                .await
            };

            let action_log = self.thread.read_with(cx, |thread, _cx| thread.action_log().clone())?;

            match input.mode {
                StreamingEditFileMode::Create | StreamingEditFileMode::Overwrite => {
                    action_log.update(cx, |log, cx| {
                        log.buffer_created(buffer.clone(), cx);
                    });
                    let content = input.content.ok_or_else(|| {
                        anyhow!("'content' field is required for create and overwrite modes")
                    })?;
                    cx.update(|cx| {
                        buffer.update(cx, |buffer, cx| {
                            buffer.edit([(0..buffer.len(), content.as_str())], None, cx);
                        });
                        action_log.update(cx, |log, cx| {
                            log.buffer_edited(buffer.clone(), cx);
                        });
                    });
                }
                StreamingEditFileMode::Edit => {
                    let edits = input.edits.ok_or_else(|| {
                        anyhow!("'edits' field is required for edit mode")
                    })?;

                    if state.edit_tracker.applied_count > 0 {
                        // Some edits were already applied during streaming.
                        // Apply only the remaining edits (the last one that was
                        // still in-progress when streaming ended).
                        let remaining_edits = &edits[state.edit_tracker.applied_count..];
                        if !remaining_edits.is_empty() {
                            // Mark buffer as read if we haven't yet (shouldn't happen
                            // since streaming already did this, but be safe)
                            apply_edits(
                                &buffer,
                                &action_log,
                                remaining_edits,
                                &diff,
                                event_stream,
                                &abs_path,
                                cx,
                            )?;
                        }
                    } else {
                        // No edits applied during streaming — apply all at once
                        action_log.update(cx, |log, cx| {
                            log.buffer_read(buffer.clone(), cx);
                        });
                        apply_edits(&buffer, &action_log, &edits, &diff, event_stream, &abs_path, cx)?;
                    }
                }
            }

            let format_on_save_enabled = buffer.read_with(cx, |buffer, cx| {
                let settings = language_settings::language_settings(
                    buffer.language().map(|l| l.name()),
                    buffer.file(),
                    cx,
                );
                settings.format_on_save != FormatOnSave::Off
            });

            if format_on_save_enabled {
                action_log.update(cx, |log, cx| {
                    log.buffer_edited(buffer.clone(), cx);
                });

                let format_task = project.update(cx, |project, cx| {
                    project.format(
                        HashSet::from_iter([buffer.clone()]),
                        LspFormatTarget::Buffers,
                        false,
                        FormatTrigger::Save,
                        cx,
                    )
                });
                futures::select! {
                    result = format_task.fuse() => { result.log_err(); },
                    _ = event_stream.cancelled_by_user().fuse() => {
                        anyhow::bail!("Edit cancelled by user");
                    }
                };
            }

            let save_task = project
                .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx));
            futures::select! {
                result = save_task.fuse() => { result?; },
                _ = event_stream.cancelled_by_user().fuse() => {
                    anyhow::bail!("Edit cancelled by user");
                }
            };

            action_log.update(cx, |log, cx| {
                log.buffer_edited(buffer.clone(), cx);
            });

            if let Some(abs_path) = abs_path.as_ref() {
                if let Some(new_mtime) = buffer.read_with(cx, |buffer, _| {
                    buffer.file().and_then(|file| file.disk_state().mtime())
                }) {
                    self.thread.update(cx, |thread, _| {
                        thread.file_read_times.insert(abs_path.to_path_buf(), new_mtime);
                    })?;
                }
            }

            let new_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
            let (new_text, unified_diff) = cx
                .background_spawn({
                    let new_snapshot = new_snapshot.clone();
                    let old_text = old_text.clone();
                    async move {
                        let new_text = new_snapshot.text();
                        let diff = language::unified_diff(&old_text, &new_text);
                        (new_text, diff)
                    }
                })
                .await;

            let output = StreamingEditFileToolOutput::Success {
                input_path: input.path,
                new_text,
                old_text,
                diff: unified_diff,
            };

            Ok(output)
        }.await;
        result.map_err(|e| StreamingEditFileToolOutput::Error {
            error: e.to_string(),
        })
    }
}

fn apply_edits(
    buffer: &Entity<language::Buffer>,
    action_log: &Entity<action_log::ActionLog>,
    edits: &[EditOperation],
    diff: &Entity<Diff>,
    event_stream: &ToolCallEventStream,
    abs_path: &Option<PathBuf>,
    cx: &mut AsyncApp,
) -> Result<()> {
    let mut failed_edits = Vec::new();
    let mut ambiguous_edits = Vec::new();
    let mut resolved_edits: Vec<(Range<usize>, String)> = Vec::new();

    // First pass: resolve all edits without applying them
    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
    for (index, edit) in edits.iter().enumerate() {
        let result = resolve_edit(&snapshot, edit);

        match result {
            Ok(Some((range, new_text))) => {
                // Reveal the range in the diff view
                let (start_anchor, end_anchor) = buffer.read_with(cx, |buffer, _cx| {
                    (
                        buffer.anchor_before(range.start),
                        buffer.anchor_after(range.end),
                    )
                });
                diff.update(cx, |card, cx| {
                    card.reveal_range(start_anchor..end_anchor, cx)
                });
                resolved_edits.push((range, new_text));
            }
            Ok(None) => {
                failed_edits.push(index);
            }
            Err(ranges) => {
                ambiguous_edits.push((index, ranges));
            }
        }
    }

    // Check for errors before applying any edits
    if !failed_edits.is_empty() {
        let indices = failed_edits
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        anyhow::bail!(
            "Could not find matching text for edit(s) at index(es): {}. \
             The old_text did not match any content in the file. \
             Please read the file again to get the current content.",
            indices
        );
    }

    if !ambiguous_edits.is_empty() {
        let details: Vec<String> = ambiguous_edits
            .iter()
            .map(|(index, ranges)| {
                let lines = ranges
                    .iter()
                    .map(|r| (snapshot.offset_to_point(r.start).row + 1).to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("edit {}: matches at lines {}", index, lines)
            })
            .collect();
        anyhow::bail!(
            "Some edits matched multiple locations in the file:\n{}. \
             Please provide more context in old_text to uniquely identify the location.",
            details.join("\n")
        );
    }

    // Sort edits by position so buffer.edit() can handle offset translation
    let mut edits_sorted = resolved_edits;
    edits_sorted.sort_by(|a, b| a.0.start.cmp(&b.0.start));

    // Emit location for the earliest edit in the file
    if let Some((first_range, _)) = edits_sorted.first() {
        if let Some(abs_path) = abs_path.clone() {
            let line = snapshot.offset_to_point(first_range.start).row;
            event_stream.update_fields(
                ToolCallUpdateFields::new()
                    .locations(vec![ToolCallLocation::new(abs_path).line(Some(line))]),
            );
        }
    }

    // Validate no overlaps (sorted ascending by start)
    for window in edits_sorted.windows(2) {
        if let [(earlier_range, _), (later_range, _)] = window
            && (earlier_range.end > later_range.start || earlier_range.start == later_range.start)
        {
            let earlier_start_line = snapshot.offset_to_point(earlier_range.start).row + 1;
            let earlier_end_line = snapshot.offset_to_point(earlier_range.end).row + 1;
            let later_start_line = snapshot.offset_to_point(later_range.start).row + 1;
            let later_end_line = snapshot.offset_to_point(later_range.end).row + 1;
            anyhow::bail!(
                "Conflicting edit ranges detected: lines {}-{} conflicts with lines {}-{}. \
                 Conflicting edit ranges are not allowed, as they would overwrite each other.",
                earlier_start_line,
                earlier_end_line,
                later_start_line,
                later_end_line,
            );
        }
    }

    // Apply all edits in a single batch and report to action_log in the same
    // effect cycle. This prevents the buffer subscription from treating these
    // as user edits.
    if !edits_sorted.is_empty() {
        cx.update(|cx| {
            buffer.update(cx, |buffer, cx| {
                buffer.edit(
                    edits_sorted
                        .iter()
                        .map(|(range, new_text)| (range.clone(), new_text.as_str())),
                    None,
                    cx,
                );
            });
            action_log.update(cx, |log, cx| {
                log.buffer_edited(buffer.clone(), cx);
            });
        });
    }

    Ok(())
}

/// Resolves an edit operation by finding the matching text in the buffer.
/// Returns Ok(Some((range, new_text))) if a unique match is found,
/// Ok(None) if no match is found, or Err(ranges) if multiple matches are found.
fn resolve_edit(
    snapshot: &BufferSnapshot,
    edit: &EditOperation,
) -> std::result::Result<Option<(Range<usize>, String)>, Vec<Range<usize>>> {
    let mut matcher = StreamingFuzzyMatcher::new(snapshot.clone());
    matcher.push(&edit.old_text, None);
    let matches = matcher.finish();

    if matches.is_empty() {
        return Ok(None);
    }

    if matches.len() > 1 {
        return Err(matches);
    }

    let match_range = matches.into_iter().next().expect("checked len above");
    Ok(Some((match_range, edit.new_text.clone())))
}

fn resolve_path(
    input: &StreamingEditFileToolInput,
    project: Entity<Project>,
    cx: &mut App,
) -> Result<ProjectPath> {
    let project = project.read(cx);

    match input.mode {
        StreamingEditFileMode::Edit | StreamingEditFileMode::Overwrite => {
            let path = project
                .find_project_path(&input.path, cx)
                .context("Can't edit file: path not found")?;

            let entry = project
                .entry_for_path(&path, cx)
                .context("Can't edit file: path not found")?;

            anyhow::ensure!(entry.is_file(), "Can't edit file: path is a directory");
            Ok(path)
        }

        StreamingEditFileMode::Create => {
            if let Some(path) = project.find_project_path(&input.path, cx) {
                anyhow::ensure!(
                    project.entry_for_path(&path, cx).is_none(),
                    "Can't create file: file already exists"
                );
            }

            let parent_path = input
                .path
                .parent()
                .context("Can't create file: incorrect path")?;

            let parent_project_path = project.find_project_path(&parent_path, cx);

            let parent_entry = parent_project_path
                .as_ref()
                .and_then(|path| project.entry_for_path(path, cx))
                .context("Can't create file: parent directory doesn't exist")?;

            anyhow::ensure!(
                parent_entry.is_dir(),
                "Can't create file: parent is not a directory"
            );

            let file_name = input
                .path
                .file_name()
                .and_then(|file_name| file_name.to_str())
                .and_then(|file_name| RelPath::unix(file_name).ok())
                .context("Can't create file: invalid filename")?;

            let new_file_path = parent_project_path.map(|parent| ProjectPath {
                path: parent.path.join(file_name),
                ..parent
            });

            new_file_path.context("Can't create file")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ContextServerRegistry, Templates, ToolInputSender};
    use gpui::{TestAppContext, UpdateGlobal};
    use language_model::fake_provider::FakeLanguageModel;
    use prompt_store::ProjectContext;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    #[gpui::test]
    async fn test_streaming_edit_create_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"dir": {}})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let result = cx
            .update(|cx| {
                let input = StreamingEditFileToolInput {
                    display_description: "Create new file".into(),
                    path: "root/dir/new_file.txt".into(),
                    mode: StreamingEditFileMode::Create,
                    content: Some("Hello, World!".into()),
                    edits: None,
                };
                Arc::new(StreamingEditFileTool::new(
                    project.clone(),
                    thread.downgrade(),
                    language_registry,
                    Templates::new(),
                ))
                .run(
                    ToolInput::resolved(input),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let StreamingEditFileToolOutput::Success { new_text, diff, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "Hello, World!");
        assert!(!diff.is_empty());
    }

    #[gpui::test]
    async fn test_streaming_edit_overwrite_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"file.txt": "old content"}))
            .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let result = cx
            .update(|cx| {
                let input = StreamingEditFileToolInput {
                    display_description: "Overwrite file".into(),
                    path: "root/file.txt".into(),
                    mode: StreamingEditFileMode::Overwrite,
                    content: Some("new content".into()),
                    edits: None,
                };
                Arc::new(StreamingEditFileTool::new(
                    project.clone(),
                    thread.downgrade(),
                    language_registry,
                    Templates::new(),
                ))
                .run(
                    ToolInput::resolved(input),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let StreamingEditFileToolOutput::Success {
            new_text, old_text, ..
        } = result.unwrap()
        else {
            panic!("expected success");
        };
        assert_eq!(new_text, "new content");
        assert_eq!(*old_text, "old content");
    }

    #[gpui::test]
    async fn test_streaming_edit_granular_edits(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "line 1\nline 2\nline 3\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let result = cx
            .update(|cx| {
                let input = StreamingEditFileToolInput {
                    display_description: "Edit lines".into(),
                    path: "root/file.txt".into(),
                    mode: StreamingEditFileMode::Edit,
                    content: None,
                    edits: Some(vec![EditOperation {
                        old_text: "line 2".into(),
                        new_text: "modified line 2".into(),
                    }]),
                };
                Arc::new(StreamingEditFileTool::new(
                    project.clone(),
                    thread.downgrade(),
                    language_registry,
                    Templates::new(),
                ))
                .run(
                    ToolInput::resolved(input),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "line 1\nmodified line 2\nline 3\n");
    }

    #[gpui::test]
    async fn test_streaming_edit_multiple_nonoverlapping_edits(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let result = cx
            .update(|cx| {
                let input = StreamingEditFileToolInput {
                    display_description: "Edit multiple lines".into(),
                    path: "root/file.txt".into(),
                    mode: StreamingEditFileMode::Edit,
                    content: None,
                    edits: Some(vec![
                        EditOperation {
                            old_text: "line 5".into(),
                            new_text: "modified line 5".into(),
                        },
                        EditOperation {
                            old_text: "line 1".into(),
                            new_text: "modified line 1".into(),
                        },
                    ]),
                };
                Arc::new(StreamingEditFileTool::new(
                    project.clone(),
                    thread.downgrade(),
                    language_registry,
                    Templates::new(),
                ))
                .run(
                    ToolInput::resolved(input),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(
            new_text,
            "modified line 1\nline 2\nline 3\nline 4\nmodified line 5\n"
        );
    }

    #[gpui::test]
    async fn test_streaming_edit_adjacent_edits(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let result = cx
            .update(|cx| {
                let input = StreamingEditFileToolInput {
                    display_description: "Edit adjacent lines".into(),
                    path: "root/file.txt".into(),
                    mode: StreamingEditFileMode::Edit,
                    content: None,
                    edits: Some(vec![
                        EditOperation {
                            old_text: "line 2".into(),
                            new_text: "modified line 2".into(),
                        },
                        EditOperation {
                            old_text: "line 3".into(),
                            new_text: "modified line 3".into(),
                        },
                    ]),
                };
                Arc::new(StreamingEditFileTool::new(
                    project.clone(),
                    thread.downgrade(),
                    language_registry,
                    Templates::new(),
                ))
                .run(
                    ToolInput::resolved(input),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(
            new_text,
            "line 1\nmodified line 2\nmodified line 3\nline 4\nline 5\n"
        );
    }

    #[gpui::test]
    async fn test_streaming_edit_ascending_order_edits(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let result = cx
            .update(|cx| {
                let input = StreamingEditFileToolInput {
                    display_description: "Edit multiple lines in ascending order".into(),
                    path: "root/file.txt".into(),
                    mode: StreamingEditFileMode::Edit,
                    content: None,
                    edits: Some(vec![
                        EditOperation {
                            old_text: "line 1".into(),
                            new_text: "modified line 1".into(),
                        },
                        EditOperation {
                            old_text: "line 5".into(),
                            new_text: "modified line 5".into(),
                        },
                    ]),
                };
                Arc::new(StreamingEditFileTool::new(
                    project.clone(),
                    thread.downgrade(),
                    language_registry,
                    Templates::new(),
                ))
                .run(
                    ToolInput::resolved(input),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(
            new_text,
            "modified line 1\nline 2\nline 3\nline 4\nmodified line 5\n"
        );
    }

    #[gpui::test]
    async fn test_streaming_edit_nonexistent_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let result = cx
            .update(|cx| {
                let input = StreamingEditFileToolInput {
                    display_description: "Some edit".into(),
                    path: "root/nonexistent_file.txt".into(),
                    mode: StreamingEditFileMode::Edit,
                    content: None,
                    edits: Some(vec![EditOperation {
                        old_text: "foo".into(),
                        new_text: "bar".into(),
                    }]),
                };
                Arc::new(StreamingEditFileTool::new(
                    project,
                    thread.downgrade(),
                    language_registry,
                    Templates::new(),
                ))
                .run(
                    ToolInput::resolved(input),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let StreamingEditFileToolOutput::Error { error } = result.unwrap_err() else {
            panic!("expected error");
        };
        assert_eq!(error, "Can't edit file: path not found");
    }

    #[gpui::test]
    async fn test_streaming_edit_failed_match(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"file.txt": "hello world"}))
            .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let result = cx
            .update(|cx| {
                let input = StreamingEditFileToolInput {
                    display_description: "Edit file".into(),
                    path: "root/file.txt".into(),
                    mode: StreamingEditFileMode::Edit,
                    content: None,
                    edits: Some(vec![EditOperation {
                        old_text: "nonexistent text that is not in the file".into(),
                        new_text: "replacement".into(),
                    }]),
                };
                Arc::new(StreamingEditFileTool::new(
                    project,
                    thread.downgrade(),
                    language_registry,
                    Templates::new(),
                ))
                .run(
                    ToolInput::resolved(input),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let StreamingEditFileToolOutput::Error { error } = result.unwrap_err() else {
            panic!("expected error");
        };
        assert!(
            error.contains("Could not find matching text"),
            "Expected error containing 'Could not find matching text' but got: {error}"
        );
    }

    #[gpui::test]
    async fn test_streaming_edit_overlapping_edits_out_of_order(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        // Multi-line file so the line-based fuzzy matcher can resolve each edit.
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        // Edit A spans lines 3-4, edit B spans lines 2-3. They overlap on
        // "line 3" and are given in descending file order so the ascending
        // sort must reorder them before the pairwise overlap check can
        // detect them correctly.
        let result = cx
            .update(|cx| {
                let input = StreamingEditFileToolInput {
                    display_description: "Overlapping edits".into(),
                    path: "root/file.txt".into(),
                    mode: StreamingEditFileMode::Edit,
                    content: None,
                    edits: Some(vec![
                        EditOperation {
                            old_text: "line 3\nline 4".into(),
                            new_text: "SECOND".into(),
                        },
                        EditOperation {
                            old_text: "line 2\nline 3".into(),
                            new_text: "FIRST".into(),
                        },
                    ]),
                };
                Arc::new(StreamingEditFileTool::new(
                    project,
                    thread.downgrade(),
                    language_registry,
                    Templates::new(),
                ))
                .run(
                    ToolInput::resolved(input),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let StreamingEditFileToolOutput::Error { error } = result.unwrap_err() else {
            panic!("expected error");
        };
        assert!(
            error.contains("Conflicting edit ranges detected"),
            "Expected 'Conflicting edit ranges detected' but got: {error}"
        );
    }

    #[gpui::test]
    async fn test_streaming_early_buffer_open(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "line 1\nline 2\nline 3\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::channel_for_test();
        let (event_stream, _receiver) = ToolCallEventStream::test();

        let tool = Arc::new(StreamingEditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
            Templates::new(),
        ));

        let task = cx.update(|cx| tool.run(input, event_stream, cx));

        // Send partials simulating LLM streaming: description first, then path, then mode
        sender.send_partial(json!({"display_description": "Edit lines"}));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Edit lines",
            "path": "root/file.txt"
        }));
        cx.run_until_parked();

        // Path is NOT yet complete because mode hasn't appeared — no buffer open yet
        sender.send_partial(json!({
            "display_description": "Edit lines",
            "path": "root/file.txt",
            "mode": "edit"
        }));
        cx.run_until_parked();

        // Now send the final complete input
        sender.send_final(json!({
            "display_description": "Edit lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [{"old_text": "line 2", "new_text": "modified line 2"}]
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "line 1\nmodified line 2\nline 3\n");
    }

    #[gpui::test]
    async fn test_streaming_path_completeness_heuristic(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "hello world"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::channel_for_test();
        let (event_stream, _receiver) = ToolCallEventStream::test();

        let tool = Arc::new(StreamingEditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
            Templates::new(),
        ));

        let task = cx.update(|cx| tool.run(input, event_stream, cx));

        // Send partial with path but NO mode — path should NOT be treated as complete
        sender.send_partial(json!({
            "display_description": "Overwrite file",
            "path": "root/file"
        }));
        cx.run_until_parked();

        // Now the path grows and mode appears
        sender.send_partial(json!({
            "display_description": "Overwrite file",
            "path": "root/file.txt",
            "mode": "overwrite"
        }));
        cx.run_until_parked();

        // Send final
        sender.send_final(json!({
            "display_description": "Overwrite file",
            "path": "root/file.txt",
            "mode": "overwrite",
            "content": "new content"
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "new content");
    }

    #[gpui::test]
    async fn test_streaming_cancellation_during_partials(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "hello world"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::channel_for_test();
        let (event_stream, _receiver, mut cancellation_tx) =
            ToolCallEventStream::test_with_cancellation();

        let tool = Arc::new(StreamingEditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
            Templates::new(),
        ));

        let task = cx.update(|cx| tool.run(input, event_stream, cx));

        // Send a partial
        sender.send_partial(json!({"display_description": "Edit"}));
        cx.run_until_parked();

        // Cancel during streaming
        ToolCallEventStream::signal_cancellation_with_sender(&mut cancellation_tx);
        cx.run_until_parked();

        // The sender is still alive so the partial loop should detect cancellation
        // We need to drop the sender to also unblock recv() if the loop didn't catch it
        drop(sender);

        let result = task.await;
        let StreamingEditFileToolOutput::Error { error } = result.unwrap_err() else {
            panic!("expected error");
        };
        assert!(
            error.contains("cancelled"),
            "Expected cancellation error but got: {error}"
        );
    }

    #[gpui::test]
    async fn test_streaming_edit_with_multiple_partials(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::channel_for_test();
        let (event_stream, _receiver) = ToolCallEventStream::test();

        let tool = Arc::new(StreamingEditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
            Templates::new(),
        ));

        let task = cx.update(|cx| tool.run(input, event_stream, cx));

        // Simulate fine-grained streaming of the JSON
        sender.send_partial(json!({"display_description": "Edit multiple"}));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Edit multiple lines",
            "path": "root/file.txt"
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Edit multiple lines",
            "path": "root/file.txt",
            "mode": "edit"
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Edit multiple lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [{"old_text": "line 1"}]
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Edit multiple lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [
                {"old_text": "line 1", "new_text": "modified line 1"},
                {"old_text": "line 5"}
            ]
        }));
        cx.run_until_parked();

        // Send final complete input
        sender.send_final(json!({
            "display_description": "Edit multiple lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [
                {"old_text": "line 1", "new_text": "modified line 1"},
                {"old_text": "line 5", "new_text": "modified line 5"}
            ]
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(
            new_text,
            "modified line 1\nline 2\nline 3\nline 4\nmodified line 5\n"
        );
    }

    #[gpui::test]
    async fn test_streaming_create_file_with_partials(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"dir": {}})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::channel_for_test();
        let (event_stream, _receiver) = ToolCallEventStream::test();

        let tool = Arc::new(StreamingEditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
            Templates::new(),
        ));

        let task = cx.update(|cx| tool.run(input, event_stream, cx));

        // Stream partials for create mode
        sender.send_partial(json!({"display_description": "Create new file"}));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Create new file",
            "path": "root/dir/new_file.txt",
            "mode": "create"
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Create new file",
            "path": "root/dir/new_file.txt",
            "mode": "create",
            "content": "Hello, "
        }));
        cx.run_until_parked();

        // Final with full content
        sender.send_final(json!({
            "display_description": "Create new file",
            "path": "root/dir/new_file.txt",
            "mode": "create",
            "content": "Hello, World!"
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "Hello, World!");
    }

    #[gpui::test]
    async fn test_streaming_no_partials_direct_final(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "line 1\nline 2\nline 3\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::channel_for_test();
        let (event_stream, _receiver) = ToolCallEventStream::test();

        let tool = Arc::new(StreamingEditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
            Templates::new(),
        ));

        let task = cx.update(|cx| tool.run(input, event_stream, cx));

        // Send final immediately with no partials (simulates non-streaming path)
        sender.send_final(json!({
            "display_description": "Edit lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [{"old_text": "line 2", "new_text": "modified line 2"}]
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "line 1\nmodified line 2\nline 3\n");
    }

    #[gpui::test]
    async fn test_streaming_incremental_edit_application(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::channel_for_test();
        let (event_stream, _receiver) = ToolCallEventStream::test();

        let tool = Arc::new(StreamingEditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
            Templates::new(),
        ));

        let task = cx.update(|cx| tool.run(input, event_stream, cx));

        // Stream description, path, mode
        sender.send_partial(json!({"display_description": "Edit multiple lines"}));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Edit multiple lines",
            "path": "root/file.txt",
            "mode": "edit"
        }));
        cx.run_until_parked();

        // First edit starts streaming (old_text only, still in progress)
        sender.send_partial(json!({
            "display_description": "Edit multiple lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [{"old_text": "line 1"}]
        }));
        cx.run_until_parked();

        // Buffer should not have changed yet — the first edit is still in progress
        // (no second edit has appeared to prove the first is complete)
        let buffer_text = project.update(cx, |project, cx| {
            let project_path = project.find_project_path(&PathBuf::from("root/file.txt"), cx);
            project_path.and_then(|pp| {
                project
                    .get_open_buffer(&pp, cx)
                    .map(|buffer| buffer.read(cx).text())
            })
        });
        // Buffer is open (from streaming) but edit 1 is still in-progress
        assert_eq!(
            buffer_text.as_deref(),
            Some("line 1\nline 2\nline 3\nline 4\nline 5\n"),
            "Buffer should not be modified while first edit is still in progress"
        );

        // Second edit appears — this proves the first edit is complete, so it
        // should be applied immediately during streaming
        sender.send_partial(json!({
            "display_description": "Edit multiple lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [
                {"old_text": "line 1", "new_text": "MODIFIED 1"},
                {"old_text": "line 5"}
            ]
        }));
        cx.run_until_parked();

        // First edit should now be applied to the buffer
        let buffer_text = project.update(cx, |project, cx| {
            let project_path = project.find_project_path(&PathBuf::from("root/file.txt"), cx);
            project_path.and_then(|pp| {
                project
                    .get_open_buffer(&pp, cx)
                    .map(|buffer| buffer.read(cx).text())
            })
        });
        assert_eq!(
            buffer_text.as_deref(),
            Some("MODIFIED 1\nline 2\nline 3\nline 4\nline 5\n"),
            "First edit should be applied during streaming when second edit appears"
        );

        // Send final complete input
        sender.send_final(json!({
            "display_description": "Edit multiple lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [
                {"old_text": "line 1", "new_text": "MODIFIED 1"},
                {"old_text": "line 5", "new_text": "MODIFIED 5"}
            ]
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success {
            new_text, old_text, ..
        } = result.unwrap()
        else {
            panic!("expected success");
        };
        assert_eq!(new_text, "MODIFIED 1\nline 2\nline 3\nline 4\nMODIFIED 5\n");
        assert_eq!(
            *old_text, "line 1\nline 2\nline 3\nline 4\nline 5\n",
            "old_text should reflect the original file content before any edits"
        );
    }

    #[gpui::test]
    async fn test_streaming_incremental_three_edits(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "aaa\nbbb\nccc\nddd\neee\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::channel_for_test();
        let (event_stream, _receiver) = ToolCallEventStream::test();

        let tool = Arc::new(StreamingEditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
            Templates::new(),
        ));

        let task = cx.update(|cx| tool.run(input, event_stream, cx));

        // Setup: description + path + mode
        sender.send_partial(json!({
            "display_description": "Edit three lines",
            "path": "root/file.txt",
            "mode": "edit"
        }));
        cx.run_until_parked();

        // Edit 1 in progress
        sender.send_partial(json!({
            "display_description": "Edit three lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [{"old_text": "aaa", "new_text": "AAA"}]
        }));
        cx.run_until_parked();

        // Edit 2 appears — edit 1 is now complete and should be applied
        sender.send_partial(json!({
            "display_description": "Edit three lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [
                {"old_text": "aaa", "new_text": "AAA"},
                {"old_text": "ccc", "new_text": "CCC"}
            ]
        }));
        cx.run_until_parked();

        // Verify edit 1 applied
        let buffer_text = project.update(cx, |project, cx| {
            let pp = project
                .find_project_path(&PathBuf::from("root/file.txt"), cx)
                .unwrap();
            project.get_open_buffer(&pp, cx).map(|b| b.read(cx).text())
        });
        assert_eq!(buffer_text.as_deref(), Some("AAA\nbbb\nccc\nddd\neee\n"));

        // Edit 3 appears — edit 2 is now complete and should be applied
        sender.send_partial(json!({
            "display_description": "Edit three lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [
                {"old_text": "aaa", "new_text": "AAA"},
                {"old_text": "ccc", "new_text": "CCC"},
                {"old_text": "eee", "new_text": "EEE"}
            ]
        }));
        cx.run_until_parked();

        // Verify edits 1 and 2 both applied
        let buffer_text = project.update(cx, |project, cx| {
            let pp = project
                .find_project_path(&PathBuf::from("root/file.txt"), cx)
                .unwrap();
            project.get_open_buffer(&pp, cx).map(|b| b.read(cx).text())
        });
        assert_eq!(buffer_text.as_deref(), Some("AAA\nbbb\nCCC\nddd\neee\n"));

        // Send final
        sender.send_final(json!({
            "display_description": "Edit three lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [
                {"old_text": "aaa", "new_text": "AAA"},
                {"old_text": "ccc", "new_text": "CCC"},
                {"old_text": "eee", "new_text": "EEE"}
            ]
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "AAA\nbbb\nCCC\nddd\nEEE\n");
    }

    #[gpui::test]
    async fn test_streaming_edit_failure_mid_stream(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "line 1\nline 2\nline 3\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::channel_for_test();
        let (event_stream, _receiver) = ToolCallEventStream::test();

        let tool = Arc::new(StreamingEditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
            Templates::new(),
        ));

        let task = cx.update(|cx| tool.run(input, event_stream, cx));

        // Setup
        sender.send_partial(json!({
            "display_description": "Edit lines",
            "path": "root/file.txt",
            "mode": "edit"
        }));
        cx.run_until_parked();

        // Edit 1 (valid) in progress — not yet complete (no second edit)
        sender.send_partial(json!({
            "display_description": "Edit lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [
                {"old_text": "line 1", "new_text": "MODIFIED"}
            ]
        }));
        cx.run_until_parked();

        // Edit 2 appears (will fail to match) — this makes edit 1 complete.
        // Edit 1 should be applied. Edit 2 is still in-progress (last edit).
        sender.send_partial(json!({
            "display_description": "Edit lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [
                {"old_text": "line 1", "new_text": "MODIFIED"},
                {"old_text": "nonexistent text that does not appear anywhere in the file at all", "new_text": "whatever"}
            ]
        }));
        cx.run_until_parked();

        // Verify edit 1 was applied
        let buffer_text = project.update(cx, |project, cx| {
            let pp = project
                .find_project_path(&PathBuf::from("root/file.txt"), cx)
                .unwrap();
            project.get_open_buffer(&pp, cx).map(|b| b.read(cx).text())
        });
        assert_eq!(
            buffer_text.as_deref(),
            Some("MODIFIED\nline 2\nline 3\n"),
            "First edit should be applied even though second edit will fail"
        );

        // Edit 3 appears — this makes edit 2 "complete", triggering its
        // resolution which should fail (old_text doesn't exist in the file).
        sender.send_partial(json!({
            "display_description": "Edit lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [
                {"old_text": "line 1", "new_text": "MODIFIED"},
                {"old_text": "nonexistent text that does not appear anywhere in the file at all", "new_text": "whatever"},
                {"old_text": "line 3", "new_text": "MODIFIED 3"}
            ]
        }));
        cx.run_until_parked();

        // The error from edit 2 should have propagated out of the partial loop.
        // Drop sender to unblock recv() if the loop didn't catch it.
        drop(sender);

        let result = task.await;
        let StreamingEditFileToolOutput::Error { error } = result.unwrap_err() else {
            panic!("expected error");
        };
        assert!(
            error.contains("Could not find matching text for edit at index 1"),
            "Expected error about edit 1 failing, got: {error}"
        );
    }

    #[gpui::test]
    async fn test_streaming_overlapping_edits_detected_naturally(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "line 1\nline 2\nline 3\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::channel_for_test();
        let (event_stream, _receiver) = ToolCallEventStream::test();

        let tool = Arc::new(StreamingEditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
            Templates::new(),
        ));

        let task = cx.update(|cx| tool.run(input, event_stream, cx));

        // Setup
        sender.send_partial(json!({
            "display_description": "Overlapping edits",
            "path": "root/file.txt",
            "mode": "edit"
        }));
        cx.run_until_parked();

        // Edit 1 targets "line 1\nline 2" and replaces it.
        // Edit 2 targets "line 2\nline 3" — but after edit 1 is applied,
        // "line 2" has been removed so this should fail to match.
        // Edit 3 exists to make edit 2 "complete" during streaming.
        sender.send_partial(json!({
            "display_description": "Overlapping edits",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [
                {"old_text": "line 1\nline 2", "new_text": "REPLACED"},
                {"old_text": "line 2\nline 3", "new_text": "ALSO REPLACED"},
                {"old_text": "line 3", "new_text": "DUMMY"}
            ]
        }));
        cx.run_until_parked();

        // Edit 1 was applied, edit 2 should fail since "line 2" no longer exists
        drop(sender);

        let result = task.await;
        let StreamingEditFileToolOutput::Error { error } = result.unwrap_err() else {
            panic!("expected error");
        };
        assert!(
            error.contains("Could not find matching text for edit at index 1"),
            "Expected overlapping edit to fail naturally, got: {error}"
        );
    }

    #[gpui::test]
    async fn test_streaming_single_edit_no_incremental(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "hello world\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::channel_for_test();
        let (event_stream, _receiver) = ToolCallEventStream::test();

        let tool = Arc::new(StreamingEditFileTool::new(
            project.clone(),
            thread.downgrade(),
            language_registry,
            Templates::new(),
        ));

        let task = cx.update(|cx| tool.run(input, event_stream, cx));

        // Setup + single edit that stays in-progress (no second edit to prove completion)
        sender.send_partial(json!({
            "display_description": "Single edit",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [{"old_text": "hello world", "new_text": "goodbye world"}]
        }));
        cx.run_until_parked();

        // Buffer should NOT be modified — the single edit is still in-progress
        let buffer_text = project.update(cx, |project, cx| {
            let pp = project
                .find_project_path(&PathBuf::from("root/file.txt"), cx)
                .unwrap();
            project.get_open_buffer(&pp, cx).map(|b| b.read(cx).text())
        });
        assert_eq!(
            buffer_text.as_deref(),
            Some("hello world\n"),
            "Single in-progress edit should not be applied during streaming"
        );

        // Send final — the edit is applied during finalization
        sender.send_final(json!({
            "display_description": "Single edit",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [{"old_text": "hello world", "new_text": "goodbye world"}]
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "goodbye world\n");
    }

    #[gpui::test]
    async fn test_streaming_input_partials_then_final(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "line 1\nline 2\nline 3\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let (sender, input): (ToolInputSender, ToolInput<StreamingEditFileToolInput>) =
            ToolInput::channel_for_test();

        let (event_stream, _event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            Arc::new(StreamingEditFileTool::new(
                project.clone(),
                thread.downgrade(),
                language_registry,
                Templates::new(),
            ))
            .run(input, event_stream, cx)
        });

        // Send progressively more complete partial snapshots, as the LLM would
        sender.send_partial(json!({
            "display_description": "Edit lines"
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Edit lines",
            "path": "root/file.txt",
            "mode": "edit"
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Edit lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [{"old_text": "line 2", "new_text": "modified line 2"}]
        }));
        cx.run_until_parked();

        // Send the final complete input
        sender.send_final(json!({
            "display_description": "Edit lines",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [{"old_text": "line 2", "new_text": "modified line 2"}]
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "line 1\nmodified line 2\nline 3\n");
    }

    #[gpui::test]
    async fn test_streaming_input_sender_dropped_before_final(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "file.txt": "hello world\n"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let (sender, input): (ToolInputSender, ToolInput<StreamingEditFileToolInput>) =
            ToolInput::channel_for_test();

        let (event_stream, _event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            Arc::new(StreamingEditFileTool::new(
                project.clone(),
                thread.downgrade(),
                language_registry,
                Templates::new(),
            ))
            .run(input, event_stream, cx)
        });

        // Send a partial then drop the sender without sending final
        sender.send_partial(json!({
            "display_description": "Edit file"
        }));
        cx.run_until_parked();

        drop(sender);

        let result = task.await;
        assert!(
            result.is_err(),
            "Tool should error when sender is dropped without sending final input"
        );
    }

    #[gpui::test]
    async fn test_streaming_input_recv_drains_partials(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"dir": {}})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());
        let thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        // Create a channel and send multiple partials before a final, then use
        // ToolInput::resolved-style immediate delivery to confirm recv() works
        // when partials are already buffered.
        let (sender, input): (ToolInputSender, ToolInput<StreamingEditFileToolInput>) =
            ToolInput::channel_for_test();

        let (event_stream, _event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            Arc::new(StreamingEditFileTool::new(
                project.clone(),
                thread.downgrade(),
                language_registry,
                Templates::new(),
            ))
            .run(input, event_stream, cx)
        });

        // Buffer several partials before sending the final
        sender.send_partial(json!({"display_description": "Create"}));
        sender.send_partial(json!({"display_description": "Create", "path": "root/dir/new.txt"}));
        sender.send_partial(json!({
            "display_description": "Create",
            "path": "root/dir/new.txt",
            "mode": "create"
        }));
        sender.send_final(json!({
            "display_description": "Create",
            "path": "root/dir/new.txt",
            "mode": "create",
            "content": "streamed content"
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "streamed content");
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            SettingsStore::update_global(cx, |store: &mut SettingsStore, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .project
                        .all_languages
                        .defaults
                        .ensure_final_newline_on_save = Some(false);
                });
            });
        });
    }
}
