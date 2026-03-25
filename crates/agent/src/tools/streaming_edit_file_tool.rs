use super::edit_file_tool::EditFileTool;
use super::restore_file_from_disk_tool::RestoreFileFromDiskTool;
use super::save_file_tool::SaveFileTool;
use super::tool_edit_parser::{ToolEditEvent, ToolEditParser};
use crate::{
    AgentTool, Thread, ToolCallEventStream, ToolInput,
    edit_agent::{
        reindent::{Reindenter, compute_indent_delta},
        streaming_fuzzy_matcher::StreamingFuzzyMatcher,
    },
};
use acp_thread::Diff;
use action_log::ActionLog;
use agent_client_protocol::{self as acp, ToolCallLocation, ToolCallUpdateFields};
use anyhow::{Context as _, Result};
use collections::HashSet;
use futures::FutureExt as _;
use gpui::{App, AppContext, AsyncApp, Entity, Task, WeakEntity};
use language::language_settings::{self, FormatOnSave};
use language::{Buffer, LanguageRegistry};
use language_model::LanguageModelToolResultContent;
use project::lsp_store::{FormatTrigger, LspFormatTarget};
use project::{AgentLocation, Project, ProjectPath};
use schemars::JsonSchema;
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{DeserializeOwned, Error as _},
};
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;
use streaming_diff::{CharOperation, StreamingDiff};
use text::ToOffset;
use ui::SharedString;
use util::rel_path::RelPath;
use util::{Deferred, ResultExt};

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
    /// - 'write': Replace the entire contents of the file. If the file doesn't exist, it will be created. Requires 'content' field.
    /// - 'edit': Make granular edits to an existing file. Requires 'edits' field.
    ///
    /// When a file already exists or you just created it, prefer editing it as opposed to recreating it from scratch.
    pub mode: StreamingEditFileMode,

    /// The complete content for the new file (required for 'write' mode).
    /// This field should contain the entire file content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// List of edit operations to apply sequentially (required for 'edit' mode).
    /// Each edit finds `old_text` in the file and replaces it with `new_text`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_vec_or_json_string"
    )]
    pub edits: Option<Vec<Edit>>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StreamingEditFileMode {
    /// Overwrite the file with new content (replacing any existing content).
    /// If the file does not exist, it will be created.
    Write,
    /// Make granular edits to an existing file
    Edit,
}

/// A single edit operation that replaces old text with new text
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Edit {
    /// The exact text to find in the file. This will be matched using fuzzy matching
    /// to handle minor differences in whitespace or formatting.
    ///
    /// Always include complete lines. Do not start or end mid-line.
    /// Be minimal with replacements:
    /// - For unique lines, include only those lines
    /// - For non-unique lines, include enough context to identify them
    pub old_text: String,
    /// The text to replace it with
    pub new_text: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
struct StreamingEditFileToolPartialInput {
    #[serde(default)]
    display_description: Option<String>,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    mode: Option<StreamingEditFileMode>,
    #[serde(default)]
    content: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_vec_or_json_string")]
    edits: Option<Vec<PartialEdit>>,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct PartialEdit {
    #[serde(default)]
    pub old_text: Option<String>,
    #[serde(default)]
    pub new_text: Option<String>,
}

/// Sometimes the model responds with a stringified JSON array of edits (`"[...]"`) instead of a regular array (`[...]`)
fn deserialize_optional_vec_or_json_string<'de, T, D>(
    deserializer: D,
) -> Result<Option<Vec<T>>, D::Error>
where
    T: DeserializeOwned,
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum VecOrJsonString<T> {
        Vec(Vec<T>),
        String(String),
    }

    let value = Option::<VecOrJsonString<T>>::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(VecOrJsonString::Vec(items)) => Ok(Some(items)),
        Some(VecOrJsonString::String(string)) => serde_json::from_str::<Vec<T>>(&string)
            .map(Some)
            .map_err(|error| {
                D::Error::custom(format!("failed to parse stringified edits array: {error}"))
            }),
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

impl StreamingEditFileToolOutput {
    pub fn error(error: impl Into<String>) -> Self {
        Self::Error {
            error: error.into(),
        }
    }
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
    project: Entity<Project>,
    thread: WeakEntity<Thread>,
    action_log: Entity<ActionLog>,
    language_registry: Arc<LanguageRegistry>,
}

impl StreamingEditFileTool {
    pub fn new(
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

    fn authorize(
        &self,
        path: &PathBuf,
        description: &str,
        event_stream: &ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<()>> {
        super::tool_permissions::authorize_file_edit(
            EditFileTool::NAME,
            path,
            description,
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
                if let Ok(input) =
                    serde_json::from_value::<StreamingEditFileToolPartialInput>(raw_input)
                {
                    let path = input.path.unwrap_or_default();
                    let path = path.trim();
                    if !path.is_empty() {
                        return self
                            .project
                            .read(cx)
                            .find_project_path(&path, cx)
                            .and_then(|project_path| {
                                self.project
                                    .read(cx)
                                    .short_full_path_for_project_path(&project_path, cx)
                            })
                            .unwrap_or_else(|| path.to_string())
                            .into();
                    }

                    let description = input.display_description.unwrap_or_default();
                    let description = description.trim();
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
            let mut state: Option<EditSession> = None;
            let mut last_partial: Option<StreamingEditFileToolPartialInput> = None;
            loop {
                futures::select! {
                    partial = input.recv_partial().fuse() => {
                        let Some(partial_value) = partial else { break };
                        if let Ok(parsed) = serde_json::from_value::<StreamingEditFileToolPartialInput>(partial_value) {
                            let path_complete = parsed.path.is_some()
                                && parsed.path.as_ref() == last_partial.as_ref().and_then(|p| p.path.as_ref());

                            last_partial = Some(parsed.clone());

                            if state.is_none()
                                && path_complete
                                && let StreamingEditFileToolPartialInput {
                                    path: Some(path),
                                    display_description: Some(display_description),
                                    mode: Some(mode),
                                    ..
                                } = &parsed
                            {
                                match EditSession::new(
                                    &PathBuf::from(path),
                                    display_description,
                                    *mode,
                                    &self,
                                    &event_stream,
                                    cx,
                                )
                                .await
                                {
                                    Ok(session) => state = Some(session),
                                    Err(e) => {
                                        log::error!("Failed to create edit session: {}", e);
                                        return Err(e);
                                    }
                                }
                            }

                            if let Some(state) = &mut state {
                                if let Err(e) = state.process(parsed, &self, &event_stream, cx) {
                                    log::error!("Failed to process edit: {}", e);
                                    return Err(e);
                                }
                            }
                        }
                    }
                    _ = event_stream.cancelled_by_user().fuse() => {
                        return Err(StreamingEditFileToolOutput::error("Edit cancelled by user"));
                    }
                }
            }
            let full_input =
                input
                    .recv()
                    .await
                    .map_err(|e| {
                        let err = StreamingEditFileToolOutput::error(format!("Failed to receive tool input: {e}"));
                        log::error!("Failed to receive tool input: {e}");
                        err
                    })?;

            let mut state = if let Some(state) = state {
                state
            } else {
                match EditSession::new(
                    &full_input.path,
                    &full_input.display_description,
                    full_input.mode,
                    &self,
                    &event_stream,
                    cx,
                )
                .await
                {
                    Ok(session) => session,
                    Err(e) => {
                        log::error!("Failed to create edit session: {}", e);
                        return Err(e);
                    }
                }
            };
            match state.finalize(full_input, &self, &event_stream, cx).await {
                Ok(output) => Ok(output),
                Err(e) => {
                    log::error!("Failed to finalize edit: {}", e);
                    Err(e)
                }
            }
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

pub struct EditSession {
    abs_path: PathBuf,
    buffer: Entity<Buffer>,
    old_text: Arc<String>,
    diff: Entity<Diff>,
    mode: StreamingEditFileMode,
    parser: ToolEditParser,
    pipeline: EditPipeline,
    _finalize_diff_guard: Deferred<Box<dyn FnOnce()>>,
}

struct EditPipeline {
    current_edit: Option<EditPipelineEntry>,
    content_written: bool,
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

impl EditPipeline {
    fn new() -> Self {
        Self {
            current_edit: None,
            content_written: false,
        }
    }

    fn ensure_resolving_old_text(&mut self, buffer: &Entity<Buffer>, cx: &mut AsyncApp) {
        if self.current_edit.is_none() {
            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.text_snapshot());
            self.current_edit = Some(EditPipelineEntry::ResolvingOldText {
                matcher: StreamingFuzzyMatcher::new(snapshot),
            });
        }
    }
}

impl EditSession {
    async fn new(
        path: &PathBuf,
        display_description: &str,
        mode: StreamingEditFileMode,
        tool: &StreamingEditFileTool,
        event_stream: &ToolCallEventStream,
        cx: &mut AsyncApp,
    ) -> Result<Self, StreamingEditFileToolOutput> {
        let project_path = cx
            .update(|cx| resolve_path(mode, &path, &tool.project, cx))
            .map_err(|e| StreamingEditFileToolOutput::error(e.to_string()))?;

        let Some(abs_path) = cx.update(|cx| tool.project.read(cx).absolute_path(&project_path, cx))
        else {
            return Err(StreamingEditFileToolOutput::error(format!(
                "Worktree at '{}' does not exist",
                path.to_string_lossy()
            )));
        };

        event_stream.update_fields(
            ToolCallUpdateFields::new().locations(vec![ToolCallLocation::new(abs_path.clone())]),
        );

        cx.update(|cx| tool.authorize(&path, &display_description, event_stream, cx))
            .await
            .map_err(|e| StreamingEditFileToolOutput::error(e.to_string()))?;

        let buffer = tool
            .project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))
            .await
            .map_err(|e| StreamingEditFileToolOutput::error(e.to_string()))?;

        ensure_buffer_saved(&buffer, &abs_path, tool, cx)?;

        let diff = cx.new(|cx| Diff::new(buffer.clone(), cx));
        event_stream.update_diff(diff.clone());
        let finalize_diff_guard = util::defer(Box::new({
            let diff = diff.downgrade();
            let mut cx = cx.clone();
            move || {
                diff.update(&mut cx, |diff, cx| diff.finalize(cx)).ok();
            }
        }) as Box<dyn FnOnce()>);

        tool.action_log.update(cx, |log, cx| match mode {
            StreamingEditFileMode::Write => log.buffer_created(buffer.clone(), cx),
            StreamingEditFileMode::Edit => log.buffer_read(buffer.clone(), cx),
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
            buffer,
            old_text,
            diff,
            mode,
            parser: ToolEditParser::default(),
            pipeline: EditPipeline::new(),
            _finalize_diff_guard: finalize_diff_guard,
        })
    }

    async fn finalize(
        &mut self,
        input: StreamingEditFileToolInput,
        tool: &StreamingEditFileTool,
        event_stream: &ToolCallEventStream,
        cx: &mut AsyncApp,
    ) -> Result<StreamingEditFileToolOutput, StreamingEditFileToolOutput> {
        let old_text = self.old_text.clone();

        match input.mode {
            StreamingEditFileMode::Write => {
                let content = input.content.ok_or_else(|| {
                    StreamingEditFileToolOutput::error("'content' field is required for write mode")
                })?;

                let events = self.parser.finalize_content(&content);
                self.process_events(&events, tool, event_stream, cx)?;
            }
            StreamingEditFileMode::Edit => {
                let edits = input.edits.ok_or_else(|| {
                    StreamingEditFileToolOutput::error("'edits' field is required for edit mode")
                })?;
                let events = self.parser.finalize_edits(&edits);
                self.process_events(&events, tool, event_stream, cx)?;

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
            }
        }

        let format_on_save_enabled = self.buffer.read_with(cx, |buffer, cx| {
            let settings = language_settings::LanguageSettings::for_buffer(buffer, cx);
            settings.format_on_save != FormatOnSave::Off
        });

        if format_on_save_enabled {
            tool.action_log.update(cx, |log, cx| {
                log.buffer_edited(self.buffer.clone(), cx);
            });

            let format_task = tool.project.update(cx, |project, cx| {
                project.format(
                    HashSet::from_iter([self.buffer.clone()]),
                    LspFormatTarget::Buffers,
                    false,
                    FormatTrigger::Save,
                    cx,
                )
            });
            futures::select! {
                result = format_task.fuse() => { result.log_err(); },
                _ = event_stream.cancelled_by_user().fuse() => {
                    return Err(StreamingEditFileToolOutput::error("Edit cancelled by user"));
                }
            };
        }

        let save_task = tool.project.update(cx, |project, cx| {
            project.save_buffer(self.buffer.clone(), cx)
        });
        futures::select! {
            result = save_task.fuse() => { result.map_err(|e| StreamingEditFileToolOutput::error(e.to_string()))?; },
            _ = event_stream.cancelled_by_user().fuse() => {
                return Err(StreamingEditFileToolOutput::error("Edit cancelled by user"));
            }
        };

        tool.action_log.update(cx, |log, cx| {
            log.buffer_edited(self.buffer.clone(), cx);
        });

        let new_snapshot = self.buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
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
            old_text: old_text.clone(),
            diff: unified_diff,
        };
        Ok(output)
    }

    fn process(
        &mut self,
        partial: StreamingEditFileToolPartialInput,
        tool: &StreamingEditFileTool,
        event_stream: &ToolCallEventStream,
        cx: &mut AsyncApp,
    ) -> Result<(), StreamingEditFileToolOutput> {
        match &self.mode {
            StreamingEditFileMode::Write => {
                if let Some(content) = &partial.content {
                    let events = self.parser.push_content(content);
                    self.process_events(&events, tool, event_stream, cx)?;
                }
            }
            StreamingEditFileMode::Edit => {
                if let Some(edits) = partial.edits {
                    let events = self.parser.push_edits(&edits);
                    self.process_events(&events, tool, event_stream, cx)?;
                }
            }
        }
        Ok(())
    }

    fn process_events(
        &mut self,
        events: &[ToolEditEvent],
        tool: &StreamingEditFileTool,
        event_stream: &ToolCallEventStream,
        cx: &mut AsyncApp,
    ) -> Result<(), StreamingEditFileToolOutput> {
        for event in events {
            match event {
                ToolEditEvent::ContentChunk { chunk } => {
                    let (buffer_id, buffer_len) = self
                        .buffer
                        .read_with(cx, |buffer, _cx| (buffer.remote_id(), buffer.len()));
                    let edit_range = if self.pipeline.content_written {
                        buffer_len..buffer_len
                    } else {
                        0..buffer_len
                    };

                    agent_edit_buffer(
                        &self.buffer,
                        [(edit_range, chunk.as_str())],
                        &tool.action_log,
                        cx,
                    );
                    cx.update(|cx| {
                        tool.set_agent_location(
                            self.buffer.downgrade(),
                            text::Anchor::max_for_buffer(buffer_id),
                            cx,
                        );
                    });
                    self.pipeline.content_written = true;
                }

                ToolEditEvent::OldTextChunk {
                    chunk, done: false, ..
                } => {
                    log::debug!("old_text_chunk: done=false, chunk='{}'", chunk);
                    self.pipeline.ensure_resolving_old_text(&self.buffer, cx);

                    if let Some(EditPipelineEntry::ResolvingOldText { matcher }) =
                        &mut self.pipeline.current_edit
                        && !chunk.is_empty()
                    {
                        if let Some(match_range) = matcher.push(chunk, None) {
                            let anchor_range = self.buffer.read_with(cx, |buffer, _cx| {
                                buffer.anchor_range_between(match_range.clone())
                            });
                            self.diff
                                .update(cx, |diff, cx| diff.reveal_range(anchor_range, cx));

                            cx.update(|cx| {
                                let position = self.buffer.read(cx).anchor_before(match_range.end);
                                tool.set_agent_location(self.buffer.downgrade(), position, cx);
                            });
                        }
                    }
                }

                ToolEditEvent::OldTextChunk {
                    edit_index,
                    chunk,
                    done: true,
                } => {
                    log::debug!("old_text_chunk: done=true, chunk='{}'", chunk);

                    self.pipeline.ensure_resolving_old_text(&self.buffer, cx);

                    let Some(EditPipelineEntry::ResolvingOldText { matcher }) =
                        &mut self.pipeline.current_edit
                    else {
                        continue;
                    };

                    if !chunk.is_empty() {
                        matcher.push(chunk, None);
                    }
                    let range = extract_match(matcher.finish(), &self.buffer, edit_index, cx)?;

                    let anchor_range = self
                        .buffer
                        .read_with(cx, |buffer, _cx| buffer.anchor_range_between(range.clone()));
                    self.diff
                        .update(cx, |diff, cx| diff.reveal_range(anchor_range, cx));

                    let snapshot = self.buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

                    let line = snapshot.offset_to_point(range.start).row;
                    event_stream.update_fields(
                        ToolCallUpdateFields::new().locations(vec![
                            ToolCallLocation::new(&self.abs_path).line(Some(line)),
                        ]),
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

                    let old_text_in_buffer =
                        snapshot.text_for_range(range.clone()).collect::<String>();

                    log::debug!(
                        "edit[{}] old_text matched at {}..{}: {:?}",
                        edit_index,
                        range.start,
                        range.end,
                        old_text_in_buffer,
                    );

                    let text_snapshot = self
                        .buffer
                        .read_with(cx, |buffer, _cx| buffer.text_snapshot());
                    self.pipeline.current_edit = Some(EditPipelineEntry::StreamingNewText {
                        streaming_diff: StreamingDiff::new(old_text_in_buffer),
                        edit_cursor: range.start,
                        reindenter: Reindenter::new(indent_delta),
                        original_snapshot: text_snapshot,
                    });

                    cx.update(|cx| {
                        let position = self.buffer.read(cx).anchor_before(range.end);
                        tool.set_agent_location(self.buffer.downgrade(), position, cx);
                    });
                }

                ToolEditEvent::NewTextChunk {
                    chunk, done: false, ..
                } => {
                    log::debug!("new_text_chunk: done=false, chunk='{}'", chunk);

                    let Some(EditPipelineEntry::StreamingNewText {
                        streaming_diff,
                        edit_cursor,
                        reindenter,
                        original_snapshot,
                        ..
                    }) = &mut self.pipeline.current_edit
                    else {
                        continue;
                    };

                    let reindented = reindenter.push(chunk);
                    if reindented.is_empty() {
                        continue;
                    }

                    let char_ops = streaming_diff.push_new(&reindented);
                    apply_char_operations(
                        &char_ops,
                        &self.buffer,
                        original_snapshot,
                        edit_cursor,
                        &tool.action_log,
                        cx,
                    );

                    let position = original_snapshot.anchor_before(*edit_cursor);
                    cx.update(|cx| {
                        tool.set_agent_location(self.buffer.downgrade(), position, cx);
                    });
                }

                ToolEditEvent::NewTextChunk {
                    chunk, done: true, ..
                } => {
                    log::debug!("new_text_chunk: done=true, chunk='{}'", chunk);

                    let Some(EditPipelineEntry::StreamingNewText {
                        mut streaming_diff,
                        mut edit_cursor,
                        mut reindenter,
                        original_snapshot,
                    }) = self.pipeline.current_edit.take()
                    else {
                        continue;
                    };

                    // Flush any remaining reindent buffer + final chunk.
                    let mut final_text = reindenter.push(chunk);
                    final_text.push_str(&reindenter.finish());

                    log::debug!("new_text_chunk: done=true, final_text='{}'", final_text);

                    if !final_text.is_empty() {
                        let char_ops = streaming_diff.push_new(&final_text);
                        apply_char_operations(
                            &char_ops,
                            &self.buffer,
                            &original_snapshot,
                            &mut edit_cursor,
                            &tool.action_log,
                            cx,
                        );
                    }

                    let remaining_ops = streaming_diff.finish();
                    apply_char_operations(
                        &remaining_ops,
                        &self.buffer,
                        &original_snapshot,
                        &mut edit_cursor,
                        &tool.action_log,
                        cx,
                    );

                    let position = original_snapshot.anchor_before(edit_cursor);
                    cx.update(|cx| {
                        tool.set_agent_location(self.buffer.downgrade(), position, cx);
                    });
                }
            }
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
                let anchor_range = snapshot.anchor_range_around(*edit_cursor..delete_end);
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
    cx: &mut AsyncApp,
) -> Result<Range<usize>, StreamingEditFileToolOutput> {
    match matches.len() {
        0 => Err(StreamingEditFileToolOutput::error(format!(
            "Could not find matching text for edit at index {}. \
                The old_text did not match any content in the file. \
                Please read the file again to get the current content.",
            edit_index,
        ))),
        1 => Ok(matches.into_iter().next().unwrap()),
        _ => {
            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
            let lines = matches
                .iter()
                .map(|r| (snapshot.offset_to_point(r.start).row + 1).to_string())
                .collect::<Vec<_>>()
                .join(", ");
            Err(StreamingEditFileToolOutput::error(format!(
                "Edit {} matched multiple locations in the file at lines: {}. \
                    Please provide more context in old_text to uniquely \
                    identify the location.",
                edit_index, lines
            )))
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
    tool: &StreamingEditFileTool,
    cx: &mut AsyncApp,
) -> Result<(), StreamingEditFileToolOutput> {
    let last_read_mtime = tool
        .action_log
        .read_with(cx, |log, _| log.file_read_time(abs_path));
    let check_result = tool.thread.read_with(cx, |thread, cx| {
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
        return Err(StreamingEditFileToolOutput::error(message));
    }

    if let (Some(last_read), Some(current)) = (last_read_mtime, current_mtime) {
        if current != last_read {
            return Err(StreamingEditFileToolOutput::error(
                "The file has been modified since you last read it. \
                             Please read the file again to get the current state before editing it.",
            ));
        }
    }

    Ok(())
}

fn resolve_path(
    mode: StreamingEditFileMode,
    path: &PathBuf,
    project: &Entity<Project>,
    cx: &mut App,
) -> Result<ProjectPath> {
    let project = project.read(cx);

    match mode {
        StreamingEditFileMode::Edit => {
            let path = project
                .find_project_path(&path, cx)
                .context("Can't edit file: path not found")?;

            let entry = project
                .entry_for_path(&path, cx)
                .context("Can't edit file: path not found")?;

            anyhow::ensure!(entry.is_file(), "Can't edit file: path is a directory");
            Ok(path)
        }
        StreamingEditFileMode::Write => {
            if let Some(path) = project.find_project_path(&path, cx)
                && let Some(entry) = project.entry_for_path(&path, cx)
            {
                anyhow::ensure!(entry.is_file(), "Can't write to file: path is a directory");
                return Ok(path);
            }

            let parent_path = path.parent().context("Can't create file: incorrect path")?;

            let parent_project_path = project.find_project_path(&parent_path, cx);

            let parent_entry = parent_project_path
                .as_ref()
                .and_then(|path| project.entry_for_path(path, cx))
                .context("Can't create file: parent directory doesn't exist")?;

            anyhow::ensure!(
                parent_entry.is_dir(),
                "Can't create file: parent is not a directory"
            );

            let file_name = path
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
    use fs::Fs as _;
    use futures::StreamExt as _;
    use gpui::{TestAppContext, UpdateGlobal};
    use language_model::fake_provider::FakeLanguageModel;
    use prompt_store::ProjectContext;
    use serde_json::json;
    use settings::Settings;
    use settings::SettingsStore;
    use util::path;
    use util::rel_path::rel_path;

    #[gpui::test]
    async fn test_streaming_edit_create_file(cx: &mut TestAppContext) {
        let (tool, _project, _action_log, _fs, _thread) = setup_test(cx, json!({"dir": {}})).await;
        let result = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "Create new file".into(),
                        path: "root/dir/new_file.txt".into(),
                        mode: StreamingEditFileMode::Write,
                        content: Some("Hello, World!".into()),
                        edits: None,
                    }),
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
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "old content"})).await;
        let result = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "Overwrite file".into(),
                        path: "root/file.txt".into(),
                        mode: StreamingEditFileMode::Write,
                        content: Some("new content".into()),
                        edits: None,
                    }),
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
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "line 1\nline 2\nline 3\n"})).await;
        let result = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "Edit lines".into(),
                        path: "root/file.txt".into(),
                        mode: StreamingEditFileMode::Edit,
                        content: None,
                        edits: Some(vec![Edit {
                            old_text: "line 2".into(),
                            new_text: "modified line 2".into(),
                        }]),
                    }),
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
    async fn test_streaming_edit_multiple_edits(cx: &mut TestAppContext) {
        let (tool, _project, _action_log, _fs, _thread) = setup_test(
            cx,
            json!({"file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"}),
        )
        .await;
        let result = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "Edit multiple lines".into(),
                        path: "root/file.txt".into(),
                        mode: StreamingEditFileMode::Edit,
                        content: None,
                        edits: Some(vec![
                            Edit {
                                old_text: "line 5".into(),
                                new_text: "modified line 5".into(),
                            },
                            Edit {
                                old_text: "line 1".into(),
                                new_text: "modified line 1".into(),
                            },
                        ]),
                    }),
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
        let (tool, _project, _action_log, _fs, _thread) = setup_test(
            cx,
            json!({"file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"}),
        )
        .await;
        let result = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "Edit adjacent lines".into(),
                        path: "root/file.txt".into(),
                        mode: StreamingEditFileMode::Edit,
                        content: None,
                        edits: Some(vec![
                            Edit {
                                old_text: "line 2".into(),
                                new_text: "modified line 2".into(),
                            },
                            Edit {
                                old_text: "line 3".into(),
                                new_text: "modified line 3".into(),
                            },
                        ]),
                    }),
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
        let (tool, _project, _action_log, _fs, _thread) = setup_test(
            cx,
            json!({"file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"}),
        )
        .await;
        let result = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "Edit multiple lines in ascending order".into(),
                        path: "root/file.txt".into(),
                        mode: StreamingEditFileMode::Edit,
                        content: None,
                        edits: Some(vec![
                            Edit {
                                old_text: "line 1".into(),
                                new_text: "modified line 1".into(),
                            },
                            Edit {
                                old_text: "line 5".into(),
                                new_text: "modified line 5".into(),
                            },
                        ]),
                    }),
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
        let (tool, _project, _action_log, _fs, _thread) = setup_test(cx, json!({})).await;
        let result = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "Some edit".into(),
                        path: "root/nonexistent_file.txt".into(),
                        mode: StreamingEditFileMode::Edit,
                        content: None,
                        edits: Some(vec![Edit {
                            old_text: "foo".into(),
                            new_text: "bar".into(),
                        }]),
                    }),
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
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "hello world"})).await;
        let result = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "Edit file".into(),
                        path: "root/file.txt".into(),
                        mode: StreamingEditFileMode::Edit,
                        content: None,
                        edits: Some(vec![Edit {
                            old_text: "nonexistent text that is not in the file".into(),
                            new_text: "replacement".into(),
                        }]),
                    }),
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
    async fn test_streaming_early_buffer_open(cx: &mut TestAppContext) {
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "line 1\nline 2\nline 3\n"})).await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

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
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "hello world"})).await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

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
            "mode": "write"
        }));
        cx.run_until_parked();

        // Send final
        sender.send_final(json!({
            "display_description": "Overwrite file",
            "path": "root/file.txt",
            "mode": "write",
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
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "hello world"})).await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver, mut cancellation_tx) =
            ToolCallEventStream::test_with_cancellation();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

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
        let (tool, _project, _action_log, _fs, _thread) = setup_test(
            cx,
            json!({"file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"}),
        )
        .await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

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
        let (tool, _project, _action_log, _fs, _thread) = setup_test(cx, json!({"dir": {}})).await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

        // Stream partials for create mode
        sender.send_partial(json!({"display_description": "Create new file"}));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Create new file",
            "path": "root/dir/new_file.txt",
            "mode": "write"
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Create new file",
            "path": "root/dir/new_file.txt",
            "mode": "write",
            "content": "Hello, "
        }));
        cx.run_until_parked();

        // Final with full content
        sender.send_final(json!({
            "display_description": "Create new file",
            "path": "root/dir/new_file.txt",
            "mode": "write",
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
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "line 1\nline 2\nline 3\n"})).await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

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
        let (tool, project, _action_log, _fs, _thread) = setup_test(
            cx,
            json!({"file.txt": "line 1\nline 2\nline 3\nline 4\nline 5\n"}),
        )
        .await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

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
        let (tool, project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "aaa\nbbb\nccc\nddd\neee\n"})).await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

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

        // Verify edit 1 fully applied. Edit 2's new_text is being
        // streamed: "CCC" is inserted but the old "ccc" isn't deleted
        // yet (StreamingDiff::finish runs when edit 3 marks edit 2 done).
        let buffer_text = project.update(cx, |project, cx| {
            let pp = project
                .find_project_path(&PathBuf::from("root/file.txt"), cx)
                .unwrap();
            project.get_open_buffer(&pp, cx).map(|b| b.read(cx).text())
        });
        assert_eq!(buffer_text.as_deref(), Some("AAA\nbbb\nCCCccc\nddd\neee\n"));

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

        // Verify edits 1 and 2 fully applied. Edit 3's new_text is being
        // streamed: "EEE" is inserted but old "eee" isn't deleted yet.
        let buffer_text = project.update(cx, |project, cx| {
            let pp = project
                .find_project_path(&PathBuf::from("root/file.txt"), cx)
                .unwrap();
            project.get_open_buffer(&pp, cx).map(|b| b.read(cx).text())
        });
        assert_eq!(buffer_text.as_deref(), Some("AAA\nbbb\nCCC\nddd\nEEEeee\n"));

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
        let (tool, project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "line 1\nline 2\nline 3\n"})).await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

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
    async fn test_streaming_single_edit_no_incremental(cx: &mut TestAppContext) {
        let (tool, project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "hello world\n"})).await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

        // Setup + single edit that stays in-progress (no second edit to prove completion)
        sender.send_partial(json!({
            "display_description": "Single edit",
            "path": "root/file.txt",
            "mode": "edit",
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Single edit",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [{"old_text": "hello world", "new_text": "goodbye world"}]
        }));
        cx.run_until_parked();

        // The edit's old_text and new_text both arrived in one partial, so
        // the old_text is resolved and new_text is being streamed via
        // StreamingDiff. The buffer reflects the in-progress diff (new text
        // inserted, old text not yet fully removed until finalization).
        let buffer_text = project.update(cx, |project, cx| {
            let pp = project
                .find_project_path(&PathBuf::from("root/file.txt"), cx)
                .unwrap();
            project.get_open_buffer(&pp, cx).map(|b| b.read(cx).text())
        });
        assert_eq!(
            buffer_text.as_deref(),
            Some("goodbye worldhello world\n"),
            "In-progress streaming diff: new text inserted, old text not yet removed"
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
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "line 1\nline 2\nline 3\n"})).await;
        let (sender, input): (ToolInputSender, ToolInput<StreamingEditFileToolInput>) =
            ToolInput::test();
        let (event_stream, _event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

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
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "hello world\n"})).await;
        let (sender, input): (ToolInputSender, ToolInput<StreamingEditFileToolInput>) =
            ToolInput::test();
        let (event_stream, _event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

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
        let (tool, _project, _action_log, _fs, _thread) = setup_test(cx, json!({"dir": {}})).await;
        // Create a channel and send multiple partials before a final, then use
        // ToolInput::resolved-style immediate delivery to confirm recv() works
        // when partials are already buffered.
        let (sender, input): (ToolInputSender, ToolInput<StreamingEditFileToolInput>) =
            ToolInput::test();
        let (event_stream, _event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

        // Buffer several partials before sending the final
        sender.send_partial(json!({"display_description": "Create"}));
        sender.send_partial(json!({"display_description": "Create", "path": "root/dir/new.txt"}));
        sender.send_partial(json!({
            "display_description": "Create",
            "path": "root/dir/new.txt",
            "mode": "write"
        }));
        sender.send_final(json!({
            "display_description": "Create",
            "path": "root/dir/new.txt",
            "mode": "write",
            "content": "streamed content"
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "streamed content");
    }

    #[gpui::test]
    async fn test_streaming_resolve_path_for_creating_file(cx: &mut TestAppContext) {
        let mode = StreamingEditFileMode::Write;

        let result = test_resolve_path(&mode, "root/new.txt", cx);
        assert_resolved_path_eq(result.await, rel_path("new.txt"));

        let result = test_resolve_path(&mode, "new.txt", cx);
        assert_resolved_path_eq(result.await, rel_path("new.txt"));

        let result = test_resolve_path(&mode, "dir/new.txt", cx);
        assert_resolved_path_eq(result.await, rel_path("dir/new.txt"));

        let result = test_resolve_path(&mode, "root/dir/subdir/existing.txt", cx);
        assert_resolved_path_eq(result.await, rel_path("dir/subdir/existing.txt"));

        let result = test_resolve_path(&mode, "root/dir/subdir", cx);
        assert_eq!(
            result.await.unwrap_err().to_string(),
            "Can't write to file: path is a directory"
        );

        let result = test_resolve_path(&mode, "root/dir/nonexistent_dir/new.txt", cx);
        assert_eq!(
            result.await.unwrap_err().to_string(),
            "Can't create file: parent directory doesn't exist"
        );
    }

    #[gpui::test]
    async fn test_streaming_resolve_path_for_editing_file(cx: &mut TestAppContext) {
        let mode = StreamingEditFileMode::Edit;

        let path_with_root = "root/dir/subdir/existing.txt";
        let path_without_root = "dir/subdir/existing.txt";
        let result = test_resolve_path(&mode, path_with_root, cx);
        assert_resolved_path_eq(result.await, rel_path(path_without_root));

        let result = test_resolve_path(&mode, path_without_root, cx);
        assert_resolved_path_eq(result.await, rel_path(path_without_root));

        let result = test_resolve_path(&mode, "root/nonexistent.txt", cx);
        assert_eq!(
            result.await.unwrap_err().to_string(),
            "Can't edit file: path not found"
        );

        let result = test_resolve_path(&mode, "root/dir", cx);
        assert_eq!(
            result.await.unwrap_err().to_string(),
            "Can't edit file: path is a directory"
        );
    }

    async fn test_resolve_path(
        mode: &StreamingEditFileMode,
        path: &str,
        cx: &mut TestAppContext,
    ) -> anyhow::Result<ProjectPath> {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "dir": {
                    "subdir": {
                        "existing.txt": "hello"
                    }
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

        cx.update(|cx| resolve_path(*mode, &PathBuf::from(path), &project, cx))
    }

    #[track_caller]
    fn assert_resolved_path_eq(path: anyhow::Result<ProjectPath>, expected: &RelPath) {
        let actual = path.expect("Should return valid path").path;
        assert_eq!(actual.as_ref(), expected);
    }

    #[gpui::test]
    async fn test_streaming_format_on_save(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"src": {}})).await;
        let (tool, project, action_log, fs, thread) =
            setup_test_with_fs(cx, fs, &[path!("/root").as_ref()]).await;

        let rust_language = Arc::new(language::Language::new(
            language::LanguageConfig {
                name: "Rust".into(),
                matcher: language::LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        ));

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_language);

        let mut fake_language_servers = language_registry.register_fake_lsp(
            "Rust",
            language::FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    document_formatting_provider: Some(lsp::OneOf::Left(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        fs.save(
            path!("/root/src/main.rs").as_ref(),
            &"initial content".into(),
            language::LineEnding::Unix,
        )
        .await
        .unwrap();

        // Open the buffer to trigger LSP initialization
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/src/main.rs"), cx)
            })
            .await
            .unwrap();

        // Register the buffer with language servers
        let _handle = project.update(cx, |project, cx| {
            project.register_buffer_with_language_servers(&buffer, cx)
        });

        const UNFORMATTED_CONTENT: &str = "fn main() {println!(\"Hello!\");}\
";
        const FORMATTED_CONTENT: &str = "This file was formatted by the fake formatter in the test.\
";

        // Get the fake language server and set up formatting handler
        let fake_language_server = fake_language_servers.next().await.unwrap();
        fake_language_server.set_request_handler::<lsp::request::Formatting, _, _>({
            |_, _| async move {
                Ok(Some(vec![lsp::TextEdit {
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(1, 0)),
                    new_text: FORMATTED_CONTENT.to_string(),
                }]))
            }
        });

        // Test with format_on_save enabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.all_languages.defaults.format_on_save = Some(FormatOnSave::On);
                    settings.project.all_languages.defaults.formatter =
                        Some(language::language_settings::FormatterList::default());
                });
            });
        });

        // Use streaming pattern so executor can pump the LSP request/response
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();

        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

        sender.send_partial(json!({
            "display_description": "Create main function",
            "path": "root/src/main.rs",
            "mode": "write"
        }));
        cx.run_until_parked();

        sender.send_final(json!({
            "display_description": "Create main function",
            "path": "root/src/main.rs",
            "mode": "write",
            "content": UNFORMATTED_CONTENT
        }));

        let result = task.await;
        assert!(result.is_ok());

        cx.executor().run_until_parked();

        let new_content = fs.load(path!("/root/src/main.rs").as_ref()).await.unwrap();
        assert_eq!(
            new_content.replace("\r\n", "\n"),
            FORMATTED_CONTENT,
            "Code should be formatted when format_on_save is enabled"
        );

        let stale_buffer_count = thread
            .read_with(cx, |thread, _cx| thread.action_log.clone())
            .read_with(cx, |log, cx| log.stale_buffers(cx).count());

        assert_eq!(
            stale_buffer_count, 0,
            "BUG: Buffer is incorrectly marked as stale after format-on-save. Found {} stale buffers.",
            stale_buffer_count
        );

        // Test with format_on_save disabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.all_languages.defaults.format_on_save =
                        Some(FormatOnSave::Off);
                });
            });
        });

        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();

        let tool2 = Arc::new(StreamingEditFileTool::new(
            project.clone(),
            thread.downgrade(),
            action_log.clone(),
            language_registry,
        ));

        let task = cx.update(|cx| tool2.run(input, event_stream, cx));

        sender.send_partial(json!({
            "display_description": "Update main function",
            "path": "root/src/main.rs",
            "mode": "write"
        }));
        cx.run_until_parked();

        sender.send_final(json!({
            "display_description": "Update main function",
            "path": "root/src/main.rs",
            "mode": "write",
            "content": UNFORMATTED_CONTENT
        }));

        let result = task.await;
        assert!(result.is_ok());

        cx.executor().run_until_parked();

        let new_content = fs.load(path!("/root/src/main.rs").as_ref()).await.unwrap();
        assert_eq!(
            new_content.replace("\r\n", "\n"),
            UNFORMATTED_CONTENT,
            "Code should not be formatted when format_on_save is disabled"
        );
    }

    #[gpui::test]
    async fn test_streaming_remove_trailing_whitespace(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"src": {}})).await;
        fs.save(
            path!("/root/src/main.rs").as_ref(),
            &"initial content".into(),
            language::LineEnding::Unix,
        )
        .await
        .unwrap();
        let (tool, project, action_log, fs, thread) =
            setup_test_with_fs(cx, fs, &[path!("/root").as_ref()]).await;
        let language_registry = project.read_with(cx, |p, _cx| p.languages().clone());

        // Test with remove_trailing_whitespace_on_save enabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .project
                        .all_languages
                        .defaults
                        .remove_trailing_whitespace_on_save = Some(true);
                });
            });
        });

        const CONTENT_WITH_TRAILING_WHITESPACE: &str =
            "fn main() {  \n    println!(\"Hello!\");  \n}\n";

        let result = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "Create main function".into(),
                        path: "root/src/main.rs".into(),
                        mode: StreamingEditFileMode::Write,
                        content: Some(CONTENT_WITH_TRAILING_WHITESPACE.into()),
                        edits: None,
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;
        assert!(result.is_ok());

        cx.executor().run_until_parked();

        assert_eq!(
            fs.load(path!("/root/src/main.rs").as_ref())
                .await
                .unwrap()
                .replace("\r\n", "\n"),
            "fn main() {\n    println!(\"Hello!\");\n}\n",
            "Trailing whitespace should be removed when remove_trailing_whitespace_on_save is enabled"
        );

        // Test with remove_trailing_whitespace_on_save disabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .project
                        .all_languages
                        .defaults
                        .remove_trailing_whitespace_on_save = Some(false);
                });
            });
        });

        let tool2 = Arc::new(StreamingEditFileTool::new(
            project.clone(),
            thread.downgrade(),
            action_log.clone(),
            language_registry,
        ));

        let result = cx
            .update(|cx| {
                tool2.run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "Update main function".into(),
                        path: "root/src/main.rs".into(),
                        mode: StreamingEditFileMode::Write,
                        content: Some(CONTENT_WITH_TRAILING_WHITESPACE.into()),
                        edits: None,
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;
        assert!(result.is_ok());

        cx.executor().run_until_parked();

        let final_content = fs.load(path!("/root/src/main.rs").as_ref()).await.unwrap();
        assert_eq!(
            final_content.replace("\r\n", "\n"),
            CONTENT_WITH_TRAILING_WHITESPACE,
            "Trailing whitespace should remain when remove_trailing_whitespace_on_save is disabled"
        );
    }

    #[gpui::test]
    async fn test_streaming_authorize(cx: &mut TestAppContext) {
        let (tool, _project, _action_log, _fs, _thread) = setup_test(cx, json!({})).await;

        // Test 1: Path with .zed component should require confirmation
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let _auth = cx.update(|cx| {
            tool.authorize(
                &PathBuf::from(".zed/settings.json"),
                "test 1",
                &stream_tx,
                cx,
            )
        });

        let event = stream_rx.expect_authorization().await;
        assert_eq!(
            event.tool_call.fields.title,
            Some("test 1 (local settings)".into())
        );

        // Test 2: Path outside project should require confirmation
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let _auth =
            cx.update(|cx| tool.authorize(&PathBuf::from("/etc/hosts"), "test 2", &stream_tx, cx));

        let event = stream_rx.expect_authorization().await;
        assert_eq!(event.tool_call.fields.title, Some("test 2".into()));

        // Test 3: Relative path without .zed should not require confirmation
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        cx.update(|cx| {
            tool.authorize(&PathBuf::from("root/src/main.rs"), "test 3", &stream_tx, cx)
        })
        .await
        .unwrap();
        assert!(stream_rx.try_next().is_err());

        // Test 4: Path with .zed in the middle should require confirmation
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let _auth = cx.update(|cx| {
            tool.authorize(
                &PathBuf::from("root/.zed/tasks.json"),
                "test 4",
                &stream_tx,
                cx,
            )
        });
        let event = stream_rx.expect_authorization().await;
        assert_eq!(
            event.tool_call.fields.title,
            Some("test 4 (local settings)".into())
        );

        // Test 5: When global default is allow, sensitive and outside-project
        // paths still require confirmation
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.tool_permissions.default = settings::ToolPermissionMode::Allow;
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        // 5.1: .zed/settings.json is a sensitive path — still prompts
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let _auth = cx.update(|cx| {
            tool.authorize(
                &PathBuf::from(".zed/settings.json"),
                "test 5.1",
                &stream_tx,
                cx,
            )
        });
        let event = stream_rx.expect_authorization().await;
        assert_eq!(
            event.tool_call.fields.title,
            Some("test 5.1 (local settings)".into())
        );

        // 5.2: /etc/hosts is outside the project, but Allow auto-approves
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        cx.update(|cx| tool.authorize(&PathBuf::from("/etc/hosts"), "test 5.2", &stream_tx, cx))
            .await
            .unwrap();
        assert!(stream_rx.try_next().is_err());

        // 5.3: Normal in-project path with allow — no confirmation needed
        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        cx.update(|cx| {
            tool.authorize(
                &PathBuf::from("root/src/main.rs"),
                "test 5.3",
                &stream_tx,
                cx,
            )
        })
        .await
        .unwrap();
        assert!(stream_rx.try_next().is_err());

        // 5.4: With Confirm default, non-project paths still prompt
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.tool_permissions.default = settings::ToolPermissionMode::Confirm;
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let _auth = cx
            .update(|cx| tool.authorize(&PathBuf::from("/etc/hosts"), "test 5.4", &stream_tx, cx));

        let event = stream_rx.expect_authorization().await;
        assert_eq!(event.tool_call.fields.title, Some("test 5.4".into()));
    }

    #[gpui::test]
    async fn test_streaming_authorize_create_under_symlink_with_allow(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({})).await;
        fs.insert_tree("/outside", json!({})).await;
        fs.insert_symlink("/root/link", PathBuf::from("/outside"))
            .await;
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/root").as_ref()]).await;

        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.tool_permissions.default = settings::ToolPermissionMode::Allow;
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let authorize_task = cx.update(|cx| {
            tool.authorize(
                &PathBuf::from("link/new.txt"),
                "create through symlink",
                &stream_tx,
                cx,
            )
        });

        let event = stream_rx.expect_authorization().await;
        assert!(
            event
                .tool_call
                .fields
                .title
                .as_deref()
                .is_some_and(|title| title.contains("points outside the project")),
            "Expected symlink escape authorization for create under external symlink"
        );

        event
            .response
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new("allow"),
                acp::PermissionOptionKind::AllowOnce,
            ))
            .unwrap();
        authorize_task.await.unwrap();
    }

    #[gpui::test]
    async fn test_streaming_edit_file_symlink_escape_requests_authorization(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;
        fs.insert_tree(
            path!("/outside"),
            json!({
                "config.txt": "old content"
            }),
        )
        .await;
        fs.create_symlink(
            path!("/root/link_to_external").as_ref(),
            PathBuf::from("/outside"),
        )
        .await
        .unwrap();
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/root").as_ref()]).await;

        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let _authorize_task = cx.update(|cx| {
            tool.authorize(
                &PathBuf::from("link_to_external/config.txt"),
                "edit through symlink",
                &stream_tx,
                cx,
            )
        });

        let auth = stream_rx.expect_authorization().await;
        let title = auth.tool_call.fields.title.as_deref().unwrap_or("");
        assert!(
            title.contains("points outside the project"),
            "title should mention symlink escape, got: {title}"
        );
    }

    #[gpui::test]
    async fn test_streaming_edit_file_symlink_escape_denied(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;
        fs.insert_tree(
            path!("/outside"),
            json!({
                "config.txt": "old content"
            }),
        )
        .await;
        fs.create_symlink(
            path!("/root/link_to_external").as_ref(),
            PathBuf::from("/outside"),
        )
        .await
        .unwrap();
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/root").as_ref()]).await;

        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let authorize_task = cx.update(|cx| {
            tool.authorize(
                &PathBuf::from("link_to_external/config.txt"),
                "edit through symlink",
                &stream_tx,
                cx,
            )
        });

        let auth = stream_rx.expect_authorization().await;
        drop(auth); // deny by dropping

        let result = authorize_task.await;
        assert!(result.is_err(), "should fail when denied");
    }

    #[gpui::test]
    async fn test_streaming_edit_file_symlink_escape_honors_deny_policy(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.tool_permissions.tools.insert(
                "edit_file".into(),
                agent_settings::ToolRules {
                    default: Some(settings::ToolPermissionMode::Deny),
                    ..Default::default()
                },
            );
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;
        fs.insert_tree(
            path!("/outside"),
            json!({
                "config.txt": "old content"
            }),
        )
        .await;
        fs.create_symlink(
            path!("/root/link_to_external").as_ref(),
            PathBuf::from("/outside"),
        )
        .await
        .unwrap();
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/root").as_ref()]).await;

        let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
        let result = cx
            .update(|cx| {
                tool.authorize(
                    &PathBuf::from("link_to_external/config.txt"),
                    "edit through symlink",
                    &stream_tx,
                    cx,
                )
            })
            .await;

        assert!(result.is_err(), "Tool should fail when policy denies");
        assert!(
            !matches!(
                stream_rx.try_next(),
                Ok(Some(Ok(crate::ThreadEvent::ToolCallAuthorization(_))))
            ),
            "Deny policy should not emit symlink authorization prompt",
        );
    }

    #[gpui::test]
    async fn test_streaming_authorize_global_config(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({})).await;
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/project").as_ref()]).await;

        let test_cases = vec![
            (
                "/etc/hosts",
                true,
                "System file should require confirmation",
            ),
            (
                "/usr/local/bin/script",
                true,
                "System bin file should require confirmation",
            ),
            (
                "project/normal_file.rs",
                false,
                "Normal project file should not require confirmation",
            ),
        ];

        for (path, should_confirm, description) in test_cases {
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let auth =
                cx.update(|cx| tool.authorize(&PathBuf::from(path), "Edit file", &stream_tx, cx));

            if should_confirm {
                stream_rx.expect_authorization().await;
            } else {
                auth.await.unwrap();
                assert!(
                    stream_rx.try_next().is_err(),
                    "Failed for case: {} - path: {} - expected no confirmation but got one",
                    description,
                    path
                );
            }
        }
    }

    #[gpui::test]
    async fn test_streaming_needs_confirmation_with_multiple_worktrees(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/workspace/frontend",
            json!({
                "src": {
                    "main.js": "console.log('frontend');"
                }
            }),
        )
        .await;
        fs.insert_tree(
            "/workspace/backend",
            json!({
                "src": {
                    "main.rs": "fn main() {}"
                }
            }),
        )
        .await;
        fs.insert_tree(
            "/workspace/shared",
            json!({
                ".zed": {
                    "settings.json": "{}"
                }
            }),
        )
        .await;
        let (tool, _project, _action_log, _fs, _thread) = setup_test_with_fs(
            cx,
            fs,
            &[
                path!("/workspace/frontend").as_ref(),
                path!("/workspace/backend").as_ref(),
                path!("/workspace/shared").as_ref(),
            ],
        )
        .await;

        let test_cases = vec![
            ("frontend/src/main.js", false, "File in first worktree"),
            ("backend/src/main.rs", false, "File in second worktree"),
            (
                "shared/.zed/settings.json",
                true,
                ".zed file in third worktree",
            ),
            ("/etc/hosts", true, "Absolute path outside all worktrees"),
            (
                "../outside/file.txt",
                true,
                "Relative path outside worktrees",
            ),
        ];

        for (path, should_confirm, description) in test_cases {
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let auth =
                cx.update(|cx| tool.authorize(&PathBuf::from(path), "Edit file", &stream_tx, cx));

            if should_confirm {
                stream_rx.expect_authorization().await;
            } else {
                auth.await.unwrap();
                assert!(
                    stream_rx.try_next().is_err(),
                    "Failed for case: {} - path: {} - expected no confirmation but got one",
                    description,
                    path
                );
            }
        }
    }

    #[gpui::test]
    async fn test_streaming_needs_confirmation_edge_cases(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".zed": {
                    "settings.json": "{}"
                },
                "src": {
                    ".zed": {
                        "local.json": "{}"
                    }
                }
            }),
        )
        .await;
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/project").as_ref()]).await;

        let test_cases = vec![
            ("", false, "Empty path is treated as project root"),
            ("/", true, "Root directory should be outside project"),
            (
                "project/../other",
                true,
                "Path with .. that goes outside of root directory",
            ),
            (
                "project/./src/file.rs",
                false,
                "Path with . should work normally",
            ),
            #[cfg(target_os = "windows")]
            ("C:\\Windows\\System32\\hosts", true, "Windows system path"),
            #[cfg(target_os = "windows")]
            ("project\\src\\main.rs", false, "Windows-style project path"),
        ];

        for (path, should_confirm, description) in test_cases {
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let auth =
                cx.update(|cx| tool.authorize(&PathBuf::from(path), "Edit file", &stream_tx, cx));

            cx.run_until_parked();

            if should_confirm {
                stream_rx.expect_authorization().await;
            } else {
                assert!(
                    stream_rx.try_next().is_err(),
                    "Failed for case: {} - path: {} - expected no confirmation but got one",
                    description,
                    path
                );
                auth.await.unwrap();
            }
        }
    }

    #[gpui::test]
    async fn test_streaming_needs_confirmation_with_different_modes(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                "existing.txt": "content",
                ".zed": {
                    "settings.json": "{}"
                }
            }),
        )
        .await;
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/project").as_ref()]).await;

        let modes = vec![StreamingEditFileMode::Edit, StreamingEditFileMode::Write];

        for _mode in modes {
            // Test .zed path with different modes
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let _auth = cx.update(|cx| {
                tool.authorize(
                    &PathBuf::from("project/.zed/settings.json"),
                    "Edit settings",
                    &stream_tx,
                    cx,
                )
            });

            stream_rx.expect_authorization().await;

            // Test outside path with different modes
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let _auth = cx.update(|cx| {
                tool.authorize(
                    &PathBuf::from("/outside/file.txt"),
                    "Edit file",
                    &stream_tx,
                    cx,
                )
            });

            stream_rx.expect_authorization().await;

            // Test normal path with different modes
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            cx.update(|cx| {
                tool.authorize(
                    &PathBuf::from("project/normal.txt"),
                    "Edit file",
                    &stream_tx,
                    cx,
                )
            })
            .await
            .unwrap();
            assert!(stream_rx.try_next().is_err());
        }
    }

    #[gpui::test]
    async fn test_streaming_initial_title_with_partial_input(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({})).await;
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test_with_fs(cx, fs, &[path!("/project").as_ref()]).await;

        cx.update(|cx| {
            assert_eq!(
                tool.initial_title(
                    Err(json!({
                        "path": "src/main.rs",
                        "display_description": "",
                    })),
                    cx
                ),
                "src/main.rs"
            );
            assert_eq!(
                tool.initial_title(
                    Err(json!({
                        "path": "",
                        "display_description": "Fix error handling",
                    })),
                    cx
                ),
                "Fix error handling"
            );
            assert_eq!(
                tool.initial_title(
                    Err(json!({
                        "path": "src/main.rs",
                        "display_description": "Fix error handling",
                    })),
                    cx
                ),
                "src/main.rs"
            );
            assert_eq!(
                tool.initial_title(
                    Err(json!({
                        "path": "",
                        "display_description": "",
                    })),
                    cx
                ),
                DEFAULT_UI_TEXT
            );
            assert_eq!(
                tool.initial_title(Err(serde_json::Value::Null), cx),
                DEFAULT_UI_TEXT
            );
        });
    }

    #[gpui::test]
    async fn test_streaming_diff_finalization(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/", json!({"main.rs": ""})).await;
        let (tool, project, action_log, _fs, thread) =
            setup_test_with_fs(cx, fs, &[path!("/").as_ref()]).await;
        let language_registry = project.read_with(cx, |p, _cx| p.languages().clone());

        // Ensure the diff is finalized after the edit completes.
        {
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let edit = cx.update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "Edit file".into(),
                        path: path!("/main.rs").into(),
                        mode: StreamingEditFileMode::Write,
                        content: Some("new content".into()),
                        edits: None,
                    }),
                    stream_tx,
                    cx,
                )
            });
            stream_rx.expect_update_fields().await;
            let diff = stream_rx.expect_diff().await;
            diff.read_with(cx, |diff, _| assert!(matches!(diff, Diff::Pending(_))));
            cx.run_until_parked();
            edit.await.unwrap();
            diff.read_with(cx, |diff, _| assert!(matches!(diff, Diff::Finalized(_))));
        }

        // Ensure the diff is finalized if the tool call gets dropped.
        {
            let tool = Arc::new(StreamingEditFileTool::new(
                project.clone(),
                thread.downgrade(),
                action_log,
                language_registry,
            ));
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let edit = cx.update(|cx| {
                tool.run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "Edit file".into(),
                        path: path!("/main.rs").into(),
                        mode: StreamingEditFileMode::Write,
                        content: Some("dropped content".into()),
                        edits: None,
                    }),
                    stream_tx,
                    cx,
                )
            });
            stream_rx.expect_update_fields().await;
            let diff = stream_rx.expect_diff().await;
            diff.read_with(cx, |diff, _| assert!(matches!(diff, Diff::Pending(_))));
            drop(edit);
            cx.run_until_parked();
            diff.read_with(cx, |diff, _| assert!(matches!(diff, Diff::Finalized(_))));
        }
    }

    #[gpui::test]
    async fn test_streaming_consecutive_edits_work(cx: &mut TestAppContext) {
        let (tool, project, action_log, _fs, _thread) =
            setup_test(cx, json!({"test.txt": "original content"})).await;
        let read_tool = Arc::new(crate::ReadFileTool::new(
            project.clone(),
            action_log.clone(),
            true,
        ));

        // Read the file first
        cx.update(|cx| {
            read_tool.clone().run(
                ToolInput::resolved(crate::ReadFileToolInput {
                    path: "root/test.txt".to_string(),
                    start_line: None,
                    end_line: None,
                }),
                ToolCallEventStream::test().0,
                cx,
            )
        })
        .await
        .unwrap();

        // First edit should work
        let edit_result = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "First edit".into(),
                        path: "root/test.txt".into(),
                        mode: StreamingEditFileMode::Edit,
                        content: None,
                        edits: Some(vec![Edit {
                            old_text: "original content".into(),
                            new_text: "modified content".into(),
                        }]),
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;
        assert!(
            edit_result.is_ok(),
            "First edit should succeed, got error: {:?}",
            edit_result.as_ref().err()
        );

        // Second edit should also work because the edit updated the recorded read time
        let edit_result = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "Second edit".into(),
                        path: "root/test.txt".into(),
                        mode: StreamingEditFileMode::Edit,
                        content: None,
                        edits: Some(vec![Edit {
                            old_text: "modified content".into(),
                            new_text: "further modified content".into(),
                        }]),
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;
        assert!(
            edit_result.is_ok(),
            "Second consecutive edit should succeed, got error: {:?}",
            edit_result.as_ref().err()
        );
    }

    #[gpui::test]
    async fn test_streaming_external_modification_detected(cx: &mut TestAppContext) {
        let (tool, project, action_log, fs, _thread) =
            setup_test(cx, json!({"test.txt": "original content"})).await;
        let read_tool = Arc::new(crate::ReadFileTool::new(
            project.clone(),
            action_log.clone(),
            true,
        ));

        // Read the file first
        cx.update(|cx| {
            read_tool.clone().run(
                ToolInput::resolved(crate::ReadFileToolInput {
                    path: "root/test.txt".to_string(),
                    start_line: None,
                    end_line: None,
                }),
                ToolCallEventStream::test().0,
                cx,
            )
        })
        .await
        .unwrap();

        // Simulate external modification
        cx.background_executor
            .advance_clock(std::time::Duration::from_secs(2));
        fs.save(
            path!("/root/test.txt").as_ref(),
            &"externally modified content".into(),
            language::LineEnding::Unix,
        )
        .await
        .unwrap();

        // Reload the buffer to pick up the new mtime
        let project_path = project
            .read_with(cx, |project, cx| {
                project.find_project_path("root/test.txt", cx)
            })
            .expect("Should find project path");
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))
            .await
            .unwrap();
        buffer
            .update(cx, |buffer, cx| buffer.reload(cx))
            .await
            .unwrap();

        cx.executor().run_until_parked();

        // Try to edit - should fail because file was modified externally
        let result = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "Edit after external change".into(),
                        path: "root/test.txt".into(),
                        mode: StreamingEditFileMode::Edit,
                        content: None,
                        edits: Some(vec![Edit {
                            old_text: "externally modified content".into(),
                            new_text: "new content".into(),
                        }]),
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let StreamingEditFileToolOutput::Error { error } = result.unwrap_err() else {
            panic!("expected error");
        };
        assert!(
            error.contains("has been modified since you last read it"),
            "Error should mention file modification, got: {}",
            error
        );
    }

    #[gpui::test]
    async fn test_streaming_dirty_buffer_detected(cx: &mut TestAppContext) {
        let (tool, project, action_log, _fs, _thread) =
            setup_test(cx, json!({"test.txt": "original content"})).await;
        let read_tool = Arc::new(crate::ReadFileTool::new(
            project.clone(),
            action_log.clone(),
            true,
        ));

        // Read the file first
        cx.update(|cx| {
            read_tool.clone().run(
                ToolInput::resolved(crate::ReadFileToolInput {
                    path: "root/test.txt".to_string(),
                    start_line: None,
                    end_line: None,
                }),
                ToolCallEventStream::test().0,
                cx,
            )
        })
        .await
        .unwrap();

        // Open the buffer and make it dirty
        let project_path = project
            .read_with(cx, |project, cx| {
                project.find_project_path("root/test.txt", cx)
            })
            .expect("Should find project path");
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))
            .await
            .unwrap();

        buffer.update(cx, |buffer, cx| {
            let end_point = buffer.max_point();
            buffer.edit([(end_point..end_point, " added text")], None, cx);
        });

        let is_dirty = buffer.read_with(cx, |buffer, _| buffer.is_dirty());
        assert!(is_dirty, "Buffer should be dirty after in-memory edit");

        // Try to edit - should fail because buffer has unsaved changes
        let result = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(StreamingEditFileToolInput {
                        display_description: "Edit with dirty buffer".into(),
                        path: "root/test.txt".into(),
                        mode: StreamingEditFileMode::Edit,
                        content: None,
                        edits: Some(vec![Edit {
                            old_text: "original content".into(),
                            new_text: "new content".into(),
                        }]),
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        let StreamingEditFileToolOutput::Error { error } = result.unwrap_err() else {
            panic!("expected error");
        };
        assert!(
            error.contains("This file has unsaved changes."),
            "Error should mention unsaved changes, got: {}",
            error
        );
        assert!(
            error.contains("keep or discard"),
            "Error should ask whether to keep or discard changes, got: {}",
            error
        );
        assert!(
            error.contains("save or revert the file manually"),
            "Error should ask user to manually save or revert when tools aren't available, got: {}",
            error
        );
    }

    #[gpui::test]
    async fn test_streaming_overlapping_edits_resolved_sequentially(cx: &mut TestAppContext) {
        // Edit 1's replacement introduces text that contains edit 2's
        // old_text as a substring. Because edits resolve sequentially
        // against the current buffer, edit 2 finds a unique match in
        // the modified buffer and succeeds.
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "aaa\nbbb\nccc\nddd\neee\n"})).await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

        // Setup: resolve the buffer
        sender.send_partial(json!({
            "display_description": "Overlapping edits",
            "path": "root/file.txt",
            "mode": "edit"
        }));
        cx.run_until_parked();

        // Edit 1 replaces "bbb\nccc" with "XXX\nccc\nddd", so the
        // buffer becomes "aaa\nXXX\nccc\nddd\nddd\neee\n".
        // Edit 2's old_text "ccc\nddd" matches the first occurrence
        // in the modified buffer and replaces it with "ZZZ".
        // Edit 3 exists only to mark edit 2 as "complete" during streaming.
        sender.send_partial(json!({
            "display_description": "Overlapping edits",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [
                {"old_text": "bbb\nccc", "new_text": "XXX\nccc\nddd"},
                {"old_text": "ccc\nddd", "new_text": "ZZZ"},
                {"old_text": "eee", "new_text": "DUMMY"}
            ]
        }));
        cx.run_until_parked();

        // Send the final input with all three edits.
        sender.send_final(json!({
            "display_description": "Overlapping edits",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [
                {"old_text": "bbb\nccc", "new_text": "XXX\nccc\nddd"},
                {"old_text": "ccc\nddd", "new_text": "ZZZ"},
                {"old_text": "eee", "new_text": "DUMMY"}
            ]
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "aaa\nXXX\nZZZ\nddd\nDUMMY\n");
    }

    #[gpui::test]
    async fn test_streaming_create_content_streamed(cx: &mut TestAppContext) {
        let (tool, project, _action_log, _fs, _thread) = setup_test(cx, json!({"dir": {}})).await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

        // Transition to BufferResolved
        sender.send_partial(json!({
            "display_description": "Create new file",
            "path": "root/dir/new_file.txt",
            "mode": "write"
        }));
        cx.run_until_parked();

        // Stream content incrementally
        sender.send_partial(json!({
            "display_description": "Create new file",
            "path": "root/dir/new_file.txt",
            "mode": "write",
            "content": "line 1\n"
        }));
        cx.run_until_parked();

        // Verify buffer has partial content
        let buffer = project.update(cx, |project, cx| {
            let path = project
                .find_project_path("root/dir/new_file.txt", cx)
                .unwrap();
            project.get_open_buffer(&path, cx).unwrap()
        });
        assert_eq!(buffer.read_with(cx, |b, _| b.text()), "line 1\n");

        // Stream more content
        sender.send_partial(json!({
            "display_description": "Create new file",
            "path": "root/dir/new_file.txt",
            "mode": "write",
            "content": "line 1\nline 2\n"
        }));
        cx.run_until_parked();
        assert_eq!(buffer.read_with(cx, |b, _| b.text()), "line 1\nline 2\n");

        // Stream final chunk
        sender.send_partial(json!({
            "display_description": "Create new file",
            "path": "root/dir/new_file.txt",
            "mode": "write",
            "content": "line 1\nline 2\nline 3\n"
        }));
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "line 1\nline 2\nline 3\n"
        );

        // Send final input
        sender.send_final(json!({
            "display_description": "Create new file",
            "path": "root/dir/new_file.txt",
            "mode": "write",
            "content": "line 1\nline 2\nline 3\n"
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "line 1\nline 2\nline 3\n");
    }

    #[gpui::test]
    async fn test_streaming_overwrite_diff_revealed_during_streaming(cx: &mut TestAppContext) {
        let (tool, _project, _action_log, _fs, _thread) = setup_test(
            cx,
            json!({"file.txt": "old line 1\nold line 2\nold line 3\n"}),
        )
        .await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, mut receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

        // Transition to BufferResolved
        sender.send_partial(json!({
            "display_description": "Overwrite file",
            "path": "root/file.txt",
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Overwrite file",
            "path": "root/file.txt",
            "mode": "write"
        }));
        cx.run_until_parked();

        // Get the diff entity from the event stream
        receiver.expect_update_fields().await;
        let diff = receiver.expect_diff().await;

        // Diff starts pending with no revealed ranges
        diff.read_with(cx, |diff, cx| {
            assert!(matches!(diff, Diff::Pending(_)));
            assert!(!diff.has_revealed_range(cx));
        });

        // Stream first content chunk
        sender.send_partial(json!({
            "display_description": "Overwrite file",
            "path": "root/file.txt",
            "mode": "write",
            "content": "new line 1\n"
        }));
        cx.run_until_parked();

        // Diff should now have revealed ranges showing the new content
        diff.read_with(cx, |diff, cx| {
            assert!(diff.has_revealed_range(cx));
        });

        // Send final input
        sender.send_final(json!({
            "display_description": "Overwrite file",
            "path": "root/file.txt",
            "mode": "write",
            "content": "new line 1\nnew line 2\n"
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success {
            new_text, old_text, ..
        } = result.unwrap()
        else {
            panic!("expected success");
        };
        assert_eq!(new_text, "new line 1\nnew line 2\n");
        assert_eq!(*old_text, "old line 1\nold line 2\nold line 3\n");

        // Diff is finalized after completion
        diff.read_with(cx, |diff, _| assert!(matches!(diff, Diff::Finalized(_))));
    }

    #[gpui::test]
    async fn test_streaming_overwrite_content_streamed(cx: &mut TestAppContext) {
        let (tool, project, _action_log, _fs, _thread) = setup_test(
            cx,
            json!({"file.txt": "old line 1\nold line 2\nold line 3\n"}),
        )
        .await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

        // Transition to BufferResolved
        sender.send_partial(json!({
            "display_description": "Overwrite file",
            "path": "root/file.txt",
            "mode": "write"
        }));
        cx.run_until_parked();

        // Verify buffer still has old content (no content partial yet)
        let buffer = project.update(cx, |project, cx| {
            let path = project.find_project_path("root/file.txt", cx).unwrap();
            project.open_buffer(path, cx)
        });
        let buffer = buffer.await.unwrap();
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "old line 1\nold line 2\nold line 3\n"
        );

        // First content partial replaces old content
        sender.send_partial(json!({
            "display_description": "Overwrite file",
            "path": "root/file.txt",
            "mode": "write",
            "content": "new line 1\n"
        }));
        cx.run_until_parked();
        assert_eq!(buffer.read_with(cx, |b, _| b.text()), "new line 1\n");

        // Subsequent content partials append
        sender.send_partial(json!({
            "display_description": "Overwrite file",
            "path": "root/file.txt",
            "mode": "write",
            "content": "new line 1\nnew line 2\n"
        }));
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |b, _| b.text()),
            "new line 1\nnew line 2\n"
        );

        // Send final input with complete content
        sender.send_final(json!({
            "display_description": "Overwrite file",
            "path": "root/file.txt",
            "mode": "write",
            "content": "new line 1\nnew line 2\nnew line 3\n"
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success {
            new_text, old_text, ..
        } = result.unwrap()
        else {
            panic!("expected success");
        };
        assert_eq!(new_text, "new line 1\nnew line 2\nnew line 3\n");
        assert_eq!(*old_text, "old line 1\nold line 2\nold line 3\n");
    }

    #[gpui::test]
    async fn test_streaming_edit_json_fixer_escape_corruption(cx: &mut TestAppContext) {
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "hello\nworld\nfoo\n"})).await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

        sender.send_partial(json!({
            "display_description": "Edit",
            "path": "root/file.txt",
            "mode": "edit"
        }));
        cx.run_until_parked();

        // Simulate JSON fixer producing a literal backslash when the LLM
        // stream cuts in the middle of a \n escape sequence.
        // The old_text "hello\nworld" would be streamed as:
        //   partial 1: old_text = "hello\\" (fixer closes incomplete \n as \\)
        //   partial 2: old_text = "hello\nworld" (fixer corrected the escape)
        sender.send_partial(json!({
            "display_description": "Edit",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [{"old_text": "hello\\"}]
        }));
        cx.run_until_parked();

        // Now the fixer corrects it to the real newline.
        sender.send_partial(json!({
            "display_description": "Edit",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [{"old_text": "hello\nworld"}]
        }));
        cx.run_until_parked();

        // Send final.
        sender.send_final(json!({
            "display_description": "Edit",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": [{"old_text": "hello\nworld", "new_text": "HELLO\nWORLD"}]
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "HELLO\nWORLD\nfoo\n");
    }

    #[gpui::test]
    async fn test_streaming_final_input_stringified_edits_succeeds(cx: &mut TestAppContext) {
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "hello\nworld\n"})).await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

        sender.send_partial(json!({
            "display_description": "Edit",
            "path": "root/file.txt",
            "mode": "edit"
        }));
        cx.run_until_parked();

        sender.send_final(json!({
            "display_description": "Edit",
            "path": "root/file.txt",
            "mode": "edit",
            "edits": "[{\"old_text\": \"hello\\nworld\", \"new_text\": \"HELLO\\nWORLD\"}]"
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "HELLO\nWORLD\n");
    }

    // Verifies that after streaming_edit_file_tool edits a file, the action log
    // reports changed buffers so that the Accept All / Reject All review UI appears.
    #[gpui::test]
    async fn test_streaming_edit_file_tool_registers_changed_buffers(cx: &mut TestAppContext) {
        let (tool, _project, action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "line 1\nline 2\nline 3\n"})).await;
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.tool_permissions.default = settings::ToolPermissionMode::Allow;
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        let (event_stream, _rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.clone().run(
                ToolInput::resolved(StreamingEditFileToolInput {
                    display_description: "Edit lines".to_string(),
                    path: "root/file.txt".into(),
                    mode: StreamingEditFileMode::Edit,
                    content: None,
                    edits: Some(vec![Edit {
                        old_text: "line 2".into(),
                        new_text: "modified line 2".into(),
                    }]),
                }),
                event_stream,
                cx,
            )
        });

        let result = task.await;
        assert!(result.is_ok(), "edit should succeed: {:?}", result.err());

        cx.run_until_parked();

        let changed = action_log.read_with(cx, |log, cx| log.changed_buffers(cx));
        assert!(
            !changed.is_empty(),
            "action_log.changed_buffers() should be non-empty after streaming edit,
             but no changed buffers were found - Accept All / Reject All will not appear"
        );
    }

    // Same test but for Write mode (overwrite entire file).
    #[gpui::test]
    async fn test_streaming_edit_file_tool_write_mode_registers_changed_buffers(
        cx: &mut TestAppContext,
    ) {
        let (tool, _project, action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "original content"})).await;
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.tool_permissions.default = settings::ToolPermissionMode::Allow;
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        let (event_stream, _rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.clone().run(
                ToolInput::resolved(StreamingEditFileToolInput {
                    display_description: "Overwrite file".to_string(),
                    path: "root/file.txt".into(),
                    mode: StreamingEditFileMode::Write,
                    content: Some("completely new content".into()),
                    edits: None,
                }),
                event_stream,
                cx,
            )
        });

        let result = task.await;
        assert!(result.is_ok(), "write should succeed: {:?}", result.err());

        cx.run_until_parked();

        let changed = action_log.read_with(cx, |log, cx| log.changed_buffers(cx));
        assert!(
            !changed.is_empty(),
            "action_log.changed_buffers() should be non-empty after streaming write, \
             but no changed buffers were found \u{2014} Accept All / Reject All will not appear"
        );
    }

    #[gpui::test]
    async fn test_streaming_edit_file_tool_fields_out_of_order_in_write_mode(
        cx: &mut TestAppContext,
    ) {
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "old_content"})).await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

        sender.send_partial(json!({
            "display_description": "Overwrite file",
            "mode": "write"
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Overwrite file",
            "mode": "write",
            "content": "new_content"
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Overwrite file",
            "mode": "write",
            "content": "new_content",
            "path": "root"
        }));
        cx.run_until_parked();

        // Send final.
        sender.send_final(json!({
            "display_description": "Overwrite file",
            "mode": "write",
            "content": "new_content",
            "path": "root/file.txt"
        }));

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "new_content");
    }

    #[gpui::test]
    async fn test_streaming_edit_file_tool_fields_out_of_order_in_edit_mode(
        cx: &mut TestAppContext,
    ) {
        let (tool, _project, _action_log, _fs, _thread) =
            setup_test(cx, json!({"file.txt": "old_content"})).await;
        let (sender, input) = ToolInput::<StreamingEditFileToolInput>::test();
        let (event_stream, _receiver) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.clone().run(input, event_stream, cx));

        sender.send_partial(json!({
            "display_description": "Overwrite file",
            "mode": "edit"
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Overwrite file",
            "mode": "edit",
            "edits": [{"old_text": "old_content"}]
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Overwrite file",
            "mode": "edit",
            "edits": [{"old_text": "old_content", "new_text": "new_content"}]
        }));
        cx.run_until_parked();

        sender.send_partial(json!({
            "display_description": "Overwrite file",
            "mode": "edit",
            "edits": [{"old_text": "old_content", "new_text": "new_content"}],
            "path": "root"
        }));
        cx.run_until_parked();

        // Send final.
        sender.send_final(json!({
            "display_description": "Overwrite file",
            "mode": "edit",
            "edits": [{"old_text": "old_content", "new_text": "new_content"}],
            "path": "root/file.txt"
        }));
        cx.run_until_parked();

        let result = task.await;
        let StreamingEditFileToolOutput::Success { new_text, .. } = result.unwrap() else {
            panic!("expected success");
        };
        assert_eq!(new_text, "new_content");
    }

    #[gpui::test]
    async fn test_streaming_reject_created_file_deletes_it(cx: &mut TestAppContext) {
        let (tool, _project, action_log, fs, _thread) = setup_test(cx, json!({"dir": {}})).await;
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.tool_permissions.default = settings::ToolPermissionMode::Allow;
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        // Create a new file via the streaming edit file tool
        let (event_stream, _rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.clone().run(
                ToolInput::resolved(StreamingEditFileToolInput {
                    display_description: "Create new file".into(),
                    path: "root/dir/new_file.txt".into(),
                    mode: StreamingEditFileMode::Write,
                    content: Some("Hello, World!".into()),
                    edits: None,
                }),
                event_stream,
                cx,
            )
        });
        let result = task.await;
        assert!(result.is_ok(), "create should succeed: {:?}", result.err());
        cx.run_until_parked();

        assert!(
            fs.is_file(path!("/root/dir/new_file.txt").as_ref()).await,
            "file should exist after creation"
        );

        // Reject all edits — this should delete the newly created file
        let changed = action_log.read_with(cx, |log, cx| log.changed_buffers(cx));
        assert!(
            !changed.is_empty(),
            "action_log should track the created file as changed"
        );

        action_log
            .update(cx, |log, cx| log.reject_all_edits(None, cx))
            .await;
        cx.run_until_parked();

        assert!(
            !fs.is_file(path!("/root/dir/new_file.txt").as_ref()).await,
            "file should be deleted after rejecting creation, but an empty file was left behind"
        );
    }

    async fn setup_test_with_fs(
        cx: &mut TestAppContext,
        fs: Arc<project::FakeFs>,
        worktree_paths: &[&std::path::Path],
    ) -> (
        Arc<StreamingEditFileTool>,
        Entity<Project>,
        Entity<ActionLog>,
        Arc<project::FakeFs>,
        Entity<Thread>,
    ) {
        let project = Project::test(fs.clone(), worktree_paths.iter().copied(), cx).await;
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
        let action_log = thread.read_with(cx, |thread, _| thread.action_log().clone());
        let tool = Arc::new(StreamingEditFileTool::new(
            project.clone(),
            thread.downgrade(),
            action_log.clone(),
            language_registry,
        ));
        (tool, project, action_log, fs, thread)
    }

    async fn setup_test(
        cx: &mut TestAppContext,
        initial_tree: serde_json::Value,
    ) -> (
        Arc<StreamingEditFileTool>,
        Entity<Project>,
        Entity<ActionLog>,
        Arc<project::FakeFs>,
        Entity<Thread>,
    ) {
        init_test(cx);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", initial_tree).await;
        setup_test_with_fs(cx, fs, &[path!("/root").as_ref()]).await
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
