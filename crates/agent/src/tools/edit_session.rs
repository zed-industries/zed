mod reindent;
mod streaming_fuzzy_matcher;
mod streaming_parser;

use super::restore_file_from_disk_tool::RestoreFileFromDiskTool;
use super::save_file_tool::SaveFileTool;
use crate::{AgentTool, Thread, ToolCallEventStream};
use acp_thread::Diff;
use action_log::ActionLog;
use agent_client_protocol::schema::{ToolCallLocation, ToolCallUpdateFields};
use anyhow::Result;
use collections::HashSet;
use gpui::{App, AppContext, AsyncApp, Entity, Task, WeakEntity};
use language::language_settings::{self, FormatOnSave};
use language::{Buffer, LanguageRegistry};
use language_model::LanguageModelToolResultContent;
use project::lsp_store::{FormatTrigger, LspFormatTarget};
use project::{AgentLocation, Project, ProjectPath};
use reindent::{Reindenter, compute_indent_delta};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;
use streaming_diff::{CharOperation, StreamingDiff};
use streaming_fuzzy_matcher::StreamingFuzzyMatcher;
use streaming_parser::{EditEvent, StreamingParser, WriteEvent};
use text::ToOffset;
use ui::SharedString;
use util::rel_path::RelPath;
use util::{Deferred, ResultExt};

/// Operating mode used internally by `EditSession`/`Pipeline` to choose between
/// applying granular edits (the `edit_file` tool) or replacing/creating the
/// entire file content (the `write_file` tool).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EditSessionMode {
    Write,
    Edit,
}

/// A single edit operation that replaces old text with new text
/// Properly escape all text fields as valid JSON strings.
/// Remember to escape special characters like newlines (`\n`) and quotes (`"`) in JSON strings.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Edit {
    /// The exact text to find in the file. This will be matched using fuzzy matching
    /// to handle minor differences in whitespace or formatting.
    ///
    /// Be minimal with replacements:
    /// - For unique lines, include only those lines
    /// - For non-unique lines, include enough context to identify them
    pub old_text: String,
    /// The text to replace it with
    pub new_text: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct PartialEdit {
    #[serde(default)]
    pub old_text: Option<String>,
    #[serde(default)]
    pub new_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditSessionOutput {
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        input_path: Option<PathBuf>,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        diff: String,
    },
}

impl EditSessionOutput {
    pub fn error(error: impl Into<String>) -> Self {
        Self::Error {
            error: error.into(),
            input_path: None,
            diff: String::new(),
        }
    }
}

impl std::fmt::Display for EditSessionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditSessionOutput::Success {
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
            EditSessionOutput::Error {
                error,
                diff,
                input_path,
            } => {
                write!(f, "{error}\n")?;
                if let Some(input_path) = input_path
                    && !diff.is_empty()
                {
                    write!(
                        f,
                        "Edited {}:\n\n```diff\n{diff}\n```",
                        input_path.display()
                    )
                } else {
                    write!(f, "No edits were made.")
                }
            }
        }
    }
}

impl From<EditSessionOutput> for LanguageModelToolResultContent {
    fn from(output: EditSessionOutput) -> Self {
        output.to_string().into()
    }
}

pub(crate) struct EditSessionContext {
    project: Entity<Project>,
    thread: WeakEntity<Thread>,
    action_log: Entity<ActionLog>,
    language_registry: Arc<LanguageRegistry>,
}

impl EditSessionContext {
    pub(crate) fn new(
        project: Entity<Project>,
        thread: WeakEntity<Thread>,
        action_log: Entity<ActionLog>,
        language_registry: Arc<LanguageRegistry>,
    ) -> Self {
        Self {
            project,
            thread,
            action_log,
            language_registry,
        }
    }

    pub(crate) fn authorize(
        &self,
        tool_name: &str,
        path: &PathBuf,
        event_stream: &ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<()>> {
        super::tool_permissions::authorize_file_edit(
            tool_name,
            path,
            &self.thread,
            event_stream,
            cx,
        )
    }

    fn set_agent_location(&self, buffer: WeakEntity<Buffer>, position: text::Anchor, cx: &mut App) {
        let should_update_agent_location = self
            .thread
            .read_with(cx, |thread, _cx| !thread.is_subagent())
            .unwrap_or_default();
        if should_update_agent_location {
            self.project.update(cx, |project, cx| {
                project.set_agent_location(Some(AgentLocation { buffer, position }), cx);
            });
        }
    }

    async fn ensure_buffer_saved(&self, buffer: &Entity<Buffer>, cx: &mut AsyncApp) {
        let format_on_save_enabled = buffer.read_with(cx, |buffer, cx| {
            let settings = language_settings::LanguageSettings::for_buffer(buffer, cx);
            settings.format_on_save != FormatOnSave::Off
        });

        if format_on_save_enabled {
            self.project
                .update(cx, |project, cx| {
                    project.format(
                        HashSet::from_iter([buffer.clone()]),
                        LspFormatTarget::Buffers,
                        false,
                        FormatTrigger::Save,
                        cx,
                    )
                })
                .await
                .log_err();
        }

        self.project
            .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
            .await
            .log_err();

        self.action_log.update(cx, |log, cx| {
            log.buffer_edited(buffer.clone(), cx);
        });
    }

    pub(crate) fn initial_title_from_path(
        &self,
        path: &std::path::Path,
        default: &str,
        cx: &App,
    ) -> SharedString {
        let project = self.project.read(cx);
        if let Some(project_path) = project.find_project_path(path, cx)
            && let Some(short) = project.short_full_path_for_project_path(&project_path, cx)
        {
            return short.into();
        }

        let display = path.to_string_lossy();
        if display.is_empty() {
            default.into()
        } else {
            display.into_owned().into()
        }
    }

    pub(crate) fn replay_output(
        &self,
        output: EditSessionOutput,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Result<()> {
        match output {
            EditSessionOutput::Success {
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
            EditSessionOutput::Error { .. } => Ok(()),
        }
    }
}

pub(crate) enum EditSessionResult {
    Completed(EditSession),
    Failed {
        error: String,
        session: Option<EditSession>,
    },
}

pub(crate) async fn run_session(
    result: EditSessionResult,
    cx: &mut AsyncApp,
) -> Result<EditSessionOutput, EditSessionOutput> {
    match result {
        EditSessionResult::Completed(session) => {
            session
                .context
                .ensure_buffer_saved(&session.buffer, cx)
                .await;
            let (new_text, diff) = session.compute_new_text_and_diff(cx).await;
            Ok(EditSessionOutput::Success {
                old_text: session.old_text.clone(),
                new_text,
                input_path: session.input_path,
                diff,
            })
        }
        EditSessionResult::Failed {
            error,
            session: Some(session),
        } => {
            session
                .context
                .ensure_buffer_saved(&session.buffer, cx)
                .await;
            let (_new_text, diff) = session.compute_new_text_and_diff(cx).await;
            Err(EditSessionOutput::Error {
                error,
                input_path: Some(session.input_path),
                diff,
            })
        }
        EditSessionResult::Failed {
            error,
            session: None,
        } => Err(EditSessionOutput::Error {
            error,
            input_path: None,
            diff: String::new(),
        }),
    }
}

pub(crate) fn initial_title_from_partial_path<P>(
    context: &EditSessionContext,
    raw_input: serde_json::Value,
    extract_path: impl FnOnce(&P) -> Option<String>,
    default: &str,
    cx: &App,
) -> SharedString
where
    P: DeserializeOwned,
{
    if let Ok(partial) = serde_json::from_value::<P>(raw_input)
        && let Some(raw_path) = extract_path(&partial)
    {
        let trimmed = raw_path.trim();
        if !trimmed.is_empty() {
            return context.initial_title_from_path(std::path::Path::new(trimmed), default, cx);
        }
    }
    default.into()
}

pub(crate) struct EditSession {
    abs_path: PathBuf,
    pub(crate) input_path: PathBuf,
    pub(crate) buffer: Entity<Buffer>,
    pub(crate) old_text: Arc<String>,
    diff: Entity<Diff>,
    parser: StreamingParser,
    pipeline: Pipeline,
    context: Arc<EditSessionContext>,
    _finalize_diff_guard: Deferred<Box<dyn FnOnce()>>,
}

enum Pipeline {
    Write(WritePipeline),
    Edit(EditPipeline),
}

struct WritePipeline {
    content_written: bool,
}

struct EditPipeline {
    current_edit: Option<EditPipelineEntry>,
    file_changed_since_last_read: bool,
}

enum EditPipelineEntry {
    ResolvingOldText {
        matcher: StreamingFuzzyMatcher,
    },
    StreamingNewText {
        streaming_diff: StreamingDiff,
        edit_cursor: usize,
        reindenter: Reindenter,
        original_snapshot: text::BufferSnapshot,
    },
}

impl Pipeline {
    fn new(mode: EditSessionMode, file_changed_since_last_read: bool) -> Self {
        match mode {
            EditSessionMode::Write => Self::Write(WritePipeline {
                content_written: false,
            }),
            EditSessionMode::Edit => Self::Edit(EditPipeline {
                current_edit: None,
                file_changed_since_last_read,
            }),
        }
    }
}

impl WritePipeline {
    fn process_event(
        &mut self,
        event: &WriteEvent,
        buffer: &Entity<Buffer>,
        context: &EditSessionContext,
        cx: &mut AsyncApp,
    ) {
        let WriteEvent::ContentChunk { chunk } = event;

        let (buffer_id, buffer_len) =
            buffer.read_with(cx, |buffer, _cx| (buffer.remote_id(), buffer.len()));
        let edit_range = if self.content_written {
            buffer_len..buffer_len
        } else {
            0..buffer_len
        };

        agent_edit_buffer(
            buffer,
            [(edit_range, chunk.as_str())],
            &context.action_log,
            cx,
        );
        cx.update(|cx| {
            context.set_agent_location(
                buffer.downgrade(),
                text::Anchor::max_for_buffer(buffer_id),
                cx,
            );
        });
        self.content_written = true;
    }
}

impl EditPipeline {
    fn ensure_resolving_old_text(&mut self, buffer: &Entity<Buffer>, cx: &mut AsyncApp) {
        if self.current_edit.is_none() {
            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.text_snapshot());
            self.current_edit = Some(EditPipelineEntry::ResolvingOldText {
                matcher: StreamingFuzzyMatcher::new(snapshot),
            });
        }
    }

    fn process_event(
        &mut self,
        event: &EditEvent,
        buffer: &Entity<Buffer>,
        diff: &Entity<Diff>,
        abs_path: &PathBuf,
        context: &EditSessionContext,
        event_stream: &ToolCallEventStream,
        cx: &mut AsyncApp,
    ) -> Result<(), String> {
        match event {
            EditEvent::OldTextChunk {
                chunk, done: false, ..
            } => {
                log::debug!("old_text_chunk: done=false, chunk='{}'", chunk);
                self.ensure_resolving_old_text(buffer, cx);

                if let Some(EditPipelineEntry::ResolvingOldText { matcher }) =
                    &mut self.current_edit
                    && !chunk.is_empty()
                {
                    if let Some(match_range) = matcher.push(chunk, None) {
                        let anchor_range = buffer.read_with(cx, |buffer, _cx| {
                            buffer.anchor_range_outside(match_range.clone())
                        });
                        diff.update(cx, |diff, cx| diff.reveal_range(anchor_range, cx));

                        cx.update(|cx| {
                            let position = buffer.read(cx).anchor_before(match_range.end);
                            context.set_agent_location(buffer.downgrade(), position, cx);
                        });
                    }
                }
            }
            EditEvent::OldTextChunk {
                edit_index,
                chunk,
                done: true,
            } => {
                log::debug!("old_text_chunk: done=true, chunk='{}'", chunk);

                self.ensure_resolving_old_text(buffer, cx);

                let Some(EditPipelineEntry::ResolvingOldText { matcher }) = &mut self.current_edit
                else {
                    return Ok(());
                };

                if !chunk.is_empty() {
                    matcher.push(chunk, None);
                }
                let range = extract_match(
                    matcher.finish(),
                    buffer,
                    edit_index,
                    self.file_changed_since_last_read,
                    cx,
                )?;

                let anchor_range =
                    buffer.read_with(cx, |buffer, _cx| buffer.anchor_range_outside(range.clone()));
                diff.update(cx, |diff, cx| diff.reveal_range(anchor_range, cx));

                let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

                let line = snapshot.offset_to_point(range.start).row;
                event_stream.update_fields(
                    ToolCallUpdateFields::new()
                        .locations(vec![ToolCallLocation::new(abs_path).line(Some(line))]),
                );

                let buffer_indent = snapshot.line_indent_for_row(line);
                let query_indent = text::LineIndent::from_iter(
                    matcher
                        .query_lines()
                        .first()
                        .map(|s| s.as_str())
                        .unwrap_or("")
                        .chars(),
                );
                let indent_delta = compute_indent_delta(buffer_indent, query_indent);

                let old_text_in_buffer = snapshot.text_for_range(range.clone()).collect::<String>();

                log::debug!(
                    "edit[{}] old_text matched at {}..{}: {:?}",
                    edit_index,
                    range.start,
                    range.end,
                    old_text_in_buffer,
                );

                let text_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.text_snapshot());
                self.current_edit = Some(EditPipelineEntry::StreamingNewText {
                    streaming_diff: StreamingDiff::new(old_text_in_buffer),
                    edit_cursor: range.start,
                    reindenter: Reindenter::new(indent_delta),
                    original_snapshot: text_snapshot,
                });

                cx.update(|cx| {
                    let position = buffer.read(cx).anchor_before(range.end);
                    context.set_agent_location(buffer.downgrade(), position, cx);
                });
            }
            EditEvent::NewTextChunk {
                chunk, done: false, ..
            } => {
                log::debug!("new_text_chunk: done=false, chunk='{}'", chunk);

                let Some(EditPipelineEntry::StreamingNewText {
                    streaming_diff,
                    edit_cursor,
                    reindenter,
                    original_snapshot,
                    ..
                }) = &mut self.current_edit
                else {
                    return Ok(());
                };

                let reindented = reindenter.push(chunk);
                if reindented.is_empty() {
                    return Ok(());
                }

                let char_ops = streaming_diff.push_new(&reindented);
                apply_char_operations(
                    &char_ops,
                    buffer,
                    original_snapshot,
                    edit_cursor,
                    &context.action_log,
                    cx,
                );

                let position = original_snapshot.anchor_before(*edit_cursor);
                cx.update(|cx| {
                    context.set_agent_location(buffer.downgrade(), position, cx);
                });
            }
            EditEvent::NewTextChunk {
                chunk, done: true, ..
            } => {
                log::debug!("new_text_chunk: done=true, chunk='{}'", chunk);

                let Some(EditPipelineEntry::StreamingNewText {
                    mut streaming_diff,
                    mut edit_cursor,
                    mut reindenter,
                    original_snapshot,
                }) = self.current_edit.take()
                else {
                    return Ok(());
                };

                let mut final_text = reindenter.push(chunk);
                final_text.push_str(&reindenter.finish());

                log::debug!("new_text_chunk: done=true, final_text='{}'", final_text);

                if !final_text.is_empty() {
                    let char_ops = streaming_diff.push_new(&final_text);
                    apply_char_operations(
                        &char_ops,
                        buffer,
                        &original_snapshot,
                        &mut edit_cursor,
                        &context.action_log,
                        cx,
                    );
                }

                let remaining_ops = streaming_diff.finish();
                apply_char_operations(
                    &remaining_ops,
                    buffer,
                    &original_snapshot,
                    &mut edit_cursor,
                    &context.action_log,
                    cx,
                );

                let position = original_snapshot.anchor_before(edit_cursor);
                cx.update(|cx| {
                    context.set_agent_location(buffer.downgrade(), position, cx);
                });
            }
        }
        Ok(())
    }
}

impl EditSession {
    pub(crate) async fn new(
        path: PathBuf,
        mode: EditSessionMode,
        tool_name: &str,
        context: Arc<EditSessionContext>,
        event_stream: &ToolCallEventStream,
        cx: &mut AsyncApp,
    ) -> Result<Self, String> {
        let project_path = cx.update(|cx| resolve_path(mode, &path, &context.project, cx))?;

        let Some(abs_path) =
            cx.update(|cx| context.project.read(cx).absolute_path(&project_path, cx))
        else {
            return Err(format!(
                "Worktree at '{}' does not exist",
                path.to_string_lossy()
            ));
        };

        event_stream.update_fields(
            ToolCallUpdateFields::new().locations(vec![ToolCallLocation::new(abs_path.clone())]),
        );

        cx.update(|cx| context.authorize(tool_name, &path, event_stream, cx))
            .await
            .map_err(|e| e.to_string())?;

        let buffer = context
            .project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))
            .await
            .map_err(|e| e.to_string())?;

        let file_changed_since_last_read = ensure_buffer_saved(&buffer, &abs_path, &context, cx)?;

        let diff = cx.new(|cx| Diff::new(buffer.clone(), cx));
        event_stream.update_diff(diff.clone());
        let finalize_diff_guard = util::defer(Box::new({
            let diff = diff.downgrade();
            let mut cx = cx.clone();
            move || {
                diff.update(&mut cx, |diff, cx| diff.finalize(cx)).ok();
            }
        }) as Box<dyn FnOnce()>);

        context.action_log.update(cx, |log, cx| match mode {
            EditSessionMode::Write => log.buffer_created(buffer.clone(), cx),
            EditSessionMode::Edit => log.buffer_read(buffer.clone(), cx),
        });

        let old_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
        let old_text = cx
            .background_spawn({
                let old_snapshot = old_snapshot.clone();
                async move { Arc::new(old_snapshot.text()) }
            })
            .await;

        Ok(Self {
            abs_path,
            input_path: path,
            buffer,
            old_text,
            diff,
            parser: StreamingParser::default(),
            pipeline: Pipeline::new(mode, file_changed_since_last_read),
            context,
            _finalize_diff_guard: finalize_diff_guard,
        })
    }

    pub(crate) async fn finalize_edit(
        &mut self,
        edits: Vec<Edit>,
        event_stream: &ToolCallEventStream,
        cx: &mut AsyncApp,
    ) -> Result<(), String> {
        let Self {
            abs_path,
            buffer,
            diff,
            parser,
            pipeline,
            context,
            ..
        } = self;
        let Pipeline::Edit(edit_pipeline) = pipeline else {
            return Err("Cannot finalize edits on a write session".to_string());
        };

        for event in &parser.finalize_edits(&edits) {
            edit_pipeline.process_event(
                event,
                buffer,
                diff,
                abs_path,
                context,
                event_stream,
                cx,
            )?;
        }

        if log::log_enabled!(log::Level::Debug) {
            log::debug!("Got edits:");
            for edit in &edits {
                log::debug!(
                    "  old_text: '{}', new_text: '{}'",
                    edit.old_text.replace('\n', "\\n"),
                    edit.new_text.replace('\n', "\\n")
                );
            }
        }
        Ok(())
    }

    pub(crate) async fn finalize_write(
        &mut self,
        content: &str,
        cx: &mut AsyncApp,
    ) -> Result<(), String> {
        let Self {
            buffer,
            parser,
            pipeline,
            context,
            ..
        } = self;
        let Pipeline::Write(write) = pipeline else {
            return Err("Cannot finalize a write on an edit session".to_string());
        };

        for event in &parser.finalize_content(content) {
            write.process_event(event, buffer, context, cx);
        }
        Ok(())
    }

    async fn compute_new_text_and_diff(&self, cx: &mut AsyncApp) -> (String, String) {
        let new_snapshot = self.buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
        let (new_text, unified_diff) = cx
            .background_spawn({
                let new_snapshot = new_snapshot.clone();
                let old_text = self.old_text.clone();
                async move {
                    let new_text = new_snapshot.text();
                    let diff = language::unified_diff(&old_text, &new_text);
                    (new_text, diff)
                }
            })
            .await;
        (new_text, unified_diff)
    }

    pub(crate) fn process_edit(
        &mut self,
        edits: Option<&[PartialEdit]>,
        event_stream: &ToolCallEventStream,
        cx: &mut AsyncApp,
    ) -> Result<(), String> {
        let Self {
            abs_path,
            buffer,
            diff,
            parser,
            pipeline,
            context,
            ..
        } = self;
        let Pipeline::Edit(edit_pipeline) = pipeline else {
            return Err("Cannot apply partial edits on a write session".to_string());
        };
        let Some(edits) = edits else {
            return Ok(());
        };
        for event in &parser.push_edits(edits) {
            edit_pipeline.process_event(
                event,
                buffer,
                diff,
                abs_path,
                context,
                event_stream,
                cx,
            )?;
        }
        Ok(())
    }

    pub(crate) fn process_write(
        &mut self,
        content: Option<&str>,
        cx: &mut AsyncApp,
    ) -> Result<(), String> {
        let Self {
            buffer,
            parser,
            pipeline,
            context,
            ..
        } = self;
        let Pipeline::Write(write) = pipeline else {
            return Err("Cannot apply partial content on an edit session".to_string());
        };
        let Some(content) = content else {
            return Ok(());
        };
        for event in &parser.push_content(content) {
            write.process_event(event, buffer, context, cx);
        }
        Ok(())
    }
}

fn apply_char_operations(
    ops: &[CharOperation],
    buffer: &Entity<Buffer>,
    snapshot: &text::BufferSnapshot,
    edit_cursor: &mut usize,
    action_log: &Entity<ActionLog>,
    cx: &mut AsyncApp,
) {
    for op in ops {
        match op {
            CharOperation::Insert { text } => {
                let anchor = snapshot.anchor_after(*edit_cursor);
                agent_edit_buffer(&buffer, [(anchor..anchor, text.as_str())], action_log, cx);
            }
            CharOperation::Delete { bytes } => {
                let delete_end = *edit_cursor + bytes;
                let anchor_range = snapshot.anchor_range_inside(*edit_cursor..delete_end);
                agent_edit_buffer(&buffer, [(anchor_range, "")], action_log, cx);
                *edit_cursor = delete_end;
            }
            CharOperation::Keep { bytes } => {
                *edit_cursor += bytes;
            }
        }
    }
}

fn extract_match(
    matches: Vec<Range<usize>>,
    buffer: &Entity<Buffer>,
    edit_index: &usize,
    file_changed_since_last_read: bool,
    cx: &mut AsyncApp,
) -> Result<Range<usize>, String> {
    let file_changed_since_last_read_message = if file_changed_since_last_read {
        " The file has changed on disk since you last read it."
    } else {
        ""
    };

    match matches.len() {
        0 => Err(format!(
            "Could not find matching text for edit at index {}. \
                The old_text did not match any content in the file.{} \
                Please read the file again to get the current content.",
            edit_index, file_changed_since_last_read_message,
        )),
        1 => Ok(matches.into_iter().next().unwrap()),
        _ => {
            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
            let lines = matches
                .iter()
                .map(|range| (snapshot.offset_to_point(range.start).row + 1).to_string())
                .collect::<Vec<_>>()
                .join(", ");
            Err(format!(
                "Edit {} matched multiple locations in the file at lines: {}. \
                    Please provide more context in old_text to uniquely \
                    identify the location.",
                edit_index, lines
            ))
        }
    }
}

/// Edits a buffer and reports the edit to the action log in the same effect
/// cycle. This ensures the action log's subscription handler sees the version
/// already updated by `buffer_edited`, so it does not misattribute the agent's
/// edit as a user edit.
fn agent_edit_buffer<I, S, T>(
    buffer: &Entity<Buffer>,
    edits: I,
    action_log: &Entity<ActionLog>,
    cx: &mut AsyncApp,
) where
    I: IntoIterator<Item = (Range<S>, T)>,
    S: ToOffset,
    T: Into<Arc<str>>,
{
    cx.update(|cx| {
        buffer.update(cx, |buffer, cx| {
            buffer.edit(edits, None, cx);
        });
        action_log.update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx));
    });
}

fn ensure_buffer_saved(
    buffer: &Entity<Buffer>,
    abs_path: &PathBuf,
    context: &EditSessionContext,
    cx: &mut AsyncApp,
) -> Result<bool, String> {
    let last_read_mtime = context
        .action_log
        .read_with(cx, |log, _| log.file_read_time(abs_path));
    let check_result = context.thread.read_with(cx, |thread, cx| {
        let current = buffer
            .read(cx)
            .file()
            .and_then(|file| file.disk_state().mtime());
        let dirty = buffer.read(cx).is_dirty();
        let has_save = thread.has_tool(SaveFileTool::NAME);
        let has_restore = thread.has_tool(RestoreFileFromDiskTool::NAME);
        (current, dirty, has_save, has_restore)
    });

    let Ok((current_mtime, is_dirty, has_save_tool, has_restore_tool)) = check_result else {
        return Ok(false);
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
        return Err(message.to_string());
    }

    if let (Some(last_read), Some(current)) = (last_read_mtime, current_mtime)
        && current != last_read
    {
        return Ok(true);
    }

    Ok(false)
}

fn resolve_path(
    mode: EditSessionMode,
    path: &PathBuf,
    project: &Entity<Project>,
    cx: &mut App,
) -> Result<ProjectPath, String> {
    let project = project.read(cx);

    match mode {
        EditSessionMode::Edit => {
            let path = project
                .find_project_path(&path, cx)
                .ok_or_else(|| "Can't edit file: path not found".to_string())?;

            let entry = project
                .entry_for_path(&path, cx)
                .ok_or_else(|| "Can't edit file: path not found".to_string())?;

            if entry.is_file() {
                Ok(path)
            } else {
                Err("Can't edit file: path is a directory".to_string())
            }
        }
        EditSessionMode::Write => {
            if let Some(path) = project.find_project_path(&path, cx)
                && let Some(entry) = project.entry_for_path(&path, cx)
            {
                if entry.is_file() {
                    return Ok(path);
                } else {
                    return Err("Can't write to file: path is a directory".to_string());
                }
            }

            let parent_path = path
                .parent()
                .ok_or_else(|| "Can't create file: incorrect path".to_string())?;

            let parent_project_path = project.find_project_path(&parent_path, cx);

            let parent_entry = parent_project_path
                .as_ref()
                .and_then(|path| project.entry_for_path(path, cx))
                .ok_or_else(|| "Can't create file: parent directory doesn't exist")?;

            if !parent_entry.is_dir() {
                return Err("Can't create file: parent is not a directory".to_string());
            }

            let file_name = path
                .file_name()
                .and_then(|file_name| file_name.to_str())
                .and_then(|file_name| RelPath::unix(file_name).ok())
                .ok_or_else(|| "Can't create file: invalid filename".to_string())?;

            let new_file_path = parent_project_path.map(|parent| ProjectPath {
                path: parent.path.join(file_name),
                ..parent
            });

            new_file_path.ok_or_else(|| "Can't create file".to_string())
        }
    }
}

#[cfg(test)]
pub(crate) async fn test_resolve_path(
    mode: &EditSessionMode,
    path: &str,
    project: &Entity<Project>,
    cx: &mut gpui::TestAppContext,
) -> Result<ProjectPath, String> {
    cx.update(|cx| resolve_path(*mode, &PathBuf::from(path), project, cx))
}
