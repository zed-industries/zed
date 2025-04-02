mod edit_action;
pub mod log;

use crate::replace::{replace_exact, replace_with_flexible_indent};
use crate::schema::json_schema_for;
use anyhow::{Context, Result, anyhow};
use assistant_tool::{ActionLog, Tool};
use collections::HashSet;
use edit_action::{EditAction, EditActionParser, edit_model_prompt};
use futures::{SinkExt, StreamExt, channel::mpsc};
use gpui::{App, AppContext, AsyncApp, Entity, Task};
use language_model::LanguageModelToolSchemaFormat;
use language_model::{
    LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage, MessageContent, Role,
};
use log::{EditToolLog, EditToolRequestId};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::sync::Arc;
use ui::IconName;
use util::ResultExt;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditFilesToolInput {
    /// High-level edit instructions. These will be interpreted by a smaller
    /// model, so explain the changes you want that model to make and which
    /// file paths need changing. The description should be concise and clear.
    ///
    /// WARNING: When specifying which file paths need changing, you MUST
    /// start each path with one of the project's root directories.
    ///
    /// WARNING: NEVER include code blocks or snippets in edit instructions.
    /// Only provide natural language descriptions of the changes needed! The tool will
    /// reject any instructions that contain code blocks or snippets.
    ///
    /// The following examples assume we have two root directories in the project:
    /// - root-1
    /// - root-2
    ///
    /// <example>
    /// If you want to introduce a new quit function to kill the process, your
    /// instructions should be: "Add a new `quit` function to
    /// `root-1/src/main.rs` to kill the process".
    ///
    /// Notice how the file path starts with root-1. Without that, the path
    /// would be ambiguous and the call would fail!
    /// </example>
    ///
    /// <example>
    /// If you want to change documentation to always start with a capital
    /// letter, your instructions should be: "In `root-2/db.js`,
    /// `root-2/inMemory.js` and `root-2/sql.js`, change all the documentation
    /// to start with a capital letter".
    ///
    /// Notice how we never specify code snippets in the instructions!
    /// </example>
    pub edit_instructions: String,

    /// A user-friendly description of what changes are being made.
    /// This will be shown to the user in the UI to describe the edit operation. The screen real estate for this UI will be extremely
    /// constrained, so make the description extremely terse.
    ///
    /// <example>
    /// For fixing a broken authentication system:
    /// "Fix auth bug in login flow"
    /// </example>
    ///
    /// <example>
    /// For adding unit tests to a module:
    /// "Add tests for user profile logic"
    /// </example>
    pub display_description: String,
}

pub struct EditFilesTool;

impl Tool for EditFilesTool {
    fn name(&self) -> String {
        "edit_files".into()
    }

    fn needs_confirmation(&self) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./edit_files_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Pencil
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> serde_json::Value {
        json_schema_for::<EditFilesToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<EditFilesToolInput>(input.clone()) {
            Ok(input) => input.display_description,
            Err(_) => "Edit files".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input = match serde_json::from_value::<EditFilesToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        match EditToolLog::try_global(cx) {
            Some(log) => {
                let req_id = log.update(cx, |log, cx| {
                    log.new_request(input.edit_instructions.clone(), cx)
                });

                let task = EditToolRequest::new(
                    input,
                    messages,
                    project,
                    action_log,
                    Some((log.clone(), req_id)),
                    cx,
                );

                cx.spawn(async move |cx| {
                    let result = task.await;

                    let str_result = match &result {
                        Ok(out) => Ok(out.clone()),
                        Err(err) => Err(err.to_string()),
                    };

                    log.update(cx, |log, cx| log.set_tool_output(req_id, str_result, cx))
                        .log_err();

                    result
                })
            }

            None => EditToolRequest::new(input, messages, project, action_log, None, cx),
        }
    }
}

struct EditToolRequest {
    parser: EditActionParser,
    editor_response: EditorResponse,
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
    tool_log: Option<(Entity<EditToolLog>, EditToolRequestId)>,
}

enum EditorResponse {
    /// The editor model hasn't produced any actions yet.
    /// If we don't have any by the end, we'll return its message to the architect model.
    Message(String),
    /// The editor model produced at least one action.
    Actions {
        applied: Vec<AppliedAction>,
        search_errors: Vec<SearchError>,
    },
}

struct AppliedAction {
    source: String,
    buffer: Entity<language::Buffer>,
}

#[derive(Debug)]
enum DiffResult {
    Diff(language::Diff),
    SearchError(SearchError),
}

#[derive(Debug)]
enum SearchError {
    NoMatch {
        file_path: String,
        search: String,
    },
    EmptyBuffer {
        file_path: String,
        search: String,
        exists: bool,
    },
}

impl EditToolRequest {
    fn new(
        input: EditFilesToolInput,
        messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        tool_log: Option<(Entity<EditToolLog>, EditToolRequestId)>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let model_registry = LanguageModelRegistry::read_global(cx);
        let Some(model) = model_registry.editor_model() else {
            return Task::ready(Err(anyhow!("No editor model configured")));
        };

        let mut messages = messages.to_vec();
        // Remove the last tool use (this run) to prevent an invalid request
        'outer: for message in messages.iter_mut().rev() {
            for (index, content) in message.content.iter().enumerate().rev() {
                match content {
                    MessageContent::ToolUse(_) => {
                        message.content.remove(index);
                        break 'outer;
                    }
                    MessageContent::ToolResult(_) => {
                        // If we find any tool results before a tool use, the request is already valid
                        break 'outer;
                    }
                    MessageContent::Text(_) | MessageContent::Image(_) => {}
                }
            }
        }

        messages.push(LanguageModelRequestMessage {
            role: Role::User,
            content: vec![edit_model_prompt().into(), input.edit_instructions.into()],
            cache: false,
        });

        cx.spawn(async move |cx| {
            let llm_request = LanguageModelRequest {
                messages,
                tools: vec![],
                stop: vec![],
                temperature: Some(0.0),
            };

            let (mut tx, mut rx) = mpsc::channel::<String>(32);
            let stream = model.stream_completion_text(llm_request, &cx);
            let reader_task = cx.background_spawn(async move {
                let mut chunks = stream.await?;

                while let Some(chunk) = chunks.stream.next().await {
                    if let Some(chunk) = chunk.log_err() {
                        // we don't process here because the API fails
                        // if we take too long between reads
                        tx.send(chunk).await?
                    }
                }
                tx.close().await?;
                anyhow::Ok(())
            });

            let mut request = Self {
                parser: EditActionParser::new(),
                editor_response: EditorResponse::Message(String::with_capacity(256)),
                action_log,
                project,
                tool_log,
            };

            while let Some(chunk) = rx.next().await {
                request.process_response_chunk(&chunk, cx).await?;
            }

            reader_task.await?;

            request.finalize(cx).await
        })
    }

    async fn process_response_chunk(&mut self, chunk: &str, cx: &mut AsyncApp) -> Result<()> {
        let new_actions = self.parser.parse_chunk(chunk);

        if let EditorResponse::Message(ref mut message) = self.editor_response {
            if new_actions.is_empty() {
                message.push_str(chunk);
            }
        }

        if let Some((ref log, req_id)) = self.tool_log {
            log.update(cx, |log, cx| {
                log.push_editor_response_chunk(req_id, chunk, &new_actions, cx)
            })
            .log_err();
        }

        for action in new_actions {
            self.apply_action(action, cx).await?;
        }

        Ok(())
    }

    async fn apply_action(
        &mut self,
        (action, source): (EditAction, String),
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let project_path = self.project.read_with(cx, |project, cx| {
            project
                .find_project_path(action.file_path(), cx)
                .context("Path not found in project")
        })??;

        let buffer = self
            .project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))?
            .await?;

        let result = match action {
            EditAction::Replace {
                old,
                new,
                file_path,
            } => {
                let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

                cx.background_executor()
                    .spawn(Self::replace_diff(old, new, file_path, snapshot))
                    .await
            }
            EditAction::Write { content, .. } => Ok(DiffResult::Diff(
                buffer
                    .read_with(cx, |buffer, cx| buffer.diff(content, cx))?
                    .await,
            )),
        }?;

        match result {
            DiffResult::SearchError(error) => {
                self.push_search_error(error);
            }
            DiffResult::Diff(diff) => {
                cx.update(|cx| {
                    self.action_log
                        .update(cx, |log, cx| log.buffer_read(buffer.clone(), cx));
                    buffer.update(cx, |buffer, cx| {
                        buffer.finalize_last_transaction();
                        buffer.apply_diff(diff, cx);
                        buffer.finalize_last_transaction();
                    });
                    self.action_log
                        .update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx));
                })?;

                self.push_applied_action(AppliedAction { source, buffer });
            }
        }

        anyhow::Ok(())
    }

    fn push_search_error(&mut self, error: SearchError) {
        match &mut self.editor_response {
            EditorResponse::Message(_) => {
                self.editor_response = EditorResponse::Actions {
                    applied: Vec::new(),
                    search_errors: vec![error],
                };
            }
            EditorResponse::Actions { search_errors, .. } => {
                search_errors.push(error);
            }
        }
    }

    fn push_applied_action(&mut self, action: AppliedAction) {
        match &mut self.editor_response {
            EditorResponse::Message(_) => {
                self.editor_response = EditorResponse::Actions {
                    applied: vec![action],
                    search_errors: Vec::new(),
                };
            }
            EditorResponse::Actions { applied, .. } => {
                applied.push(action);
            }
        }
    }

    async fn replace_diff(
        old: String,
        new: String,
        file_path: std::path::PathBuf,
        snapshot: language::BufferSnapshot,
    ) -> Result<DiffResult> {
        if snapshot.is_empty() {
            let exists = snapshot
                .file()
                .map_or(false, |file| file.disk_state().exists());

            let error = SearchError::EmptyBuffer {
                file_path: file_path.display().to_string(),
                exists,
                search: old,
            };

            return Ok(DiffResult::SearchError(error));
        }

        let replace_result =
            // Try to match exactly
            replace_exact(&old, &new, &snapshot)
            .await
            // If that fails, try being flexible about indentation
            .or_else(|| replace_with_flexible_indent(&old, &new, &snapshot));

        let Some(diff) = replace_result else {
            let error = SearchError::NoMatch {
                search: old,
                file_path: file_path.display().to_string(),
            };

            return Ok(DiffResult::SearchError(error));
        };

        Ok(DiffResult::Diff(diff))
    }

    async fn finalize(self, cx: &mut AsyncApp) -> Result<String> {
        match self.editor_response {
            EditorResponse::Message(message) => Err(anyhow!(
                "No edits were applied! You might need to provide more context.\n\n{}",
                message
            )),
            EditorResponse::Actions {
                applied,
                search_errors,
            } => {
                let mut output = String::with_capacity(1024);

                let parse_errors = self.parser.errors();
                let has_errors = !search_errors.is_empty() || !parse_errors.is_empty();

                if has_errors {
                    let error_count = search_errors.len() + parse_errors.len();

                    if applied.is_empty() {
                        writeln!(
                            &mut output,
                            "{} errors occurred! No edits were applied.",
                            error_count,
                        )?;
                    } else {
                        writeln!(
                            &mut output,
                            "{} errors occurred, but {} edits were correctly applied.",
                            error_count,
                            applied.len(),
                        )?;

                        writeln!(
                            &mut output,
                            "# {} SEARCH/REPLACE block(s) applied:\n\nDo not re-send these since they are already applied!\n",
                            applied.len()
                        )?;
                    }
                } else {
                    write!(
                        &mut output,
                        "Successfully applied! Here's a list of applied edits:"
                    )?;
                }

                let mut changed_buffers = HashSet::default();

                for action in applied {
                    changed_buffers.insert(action.buffer.clone());
                    write!(&mut output, "\n\n{}", action.source)?;
                }

                for buffer in &changed_buffers {
                    self.project
                        .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))?
                        .await?;
                }

                if !search_errors.is_empty() {
                    writeln!(
                        &mut output,
                        "\n\n## {} SEARCH/REPLACE block(s) failed to match:\n",
                        search_errors.len()
                    )?;

                    for error in search_errors {
                        match error {
                            SearchError::NoMatch { file_path, search } => {
                                writeln!(
                                    &mut output,
                                    "### No exact match in: `{}`\n```\n{}\n```\n",
                                    file_path, search,
                                )?;
                            }
                            SearchError::EmptyBuffer {
                                file_path,
                                exists: true,
                                search,
                            } => {
                                writeln!(
                                    &mut output,
                                    "### No match because `{}` is empty:\n```\n{}\n```\n",
                                    file_path, search,
                                )?;
                            }
                            SearchError::EmptyBuffer {
                                file_path,
                                exists: false,
                                search,
                            } => {
                                writeln!(
                                    &mut output,
                                    "### No match because `{}` does not exist:\n```\n{}\n```\n",
                                    file_path, search,
                                )?;
                            }
                        }
                    }

                    write!(
                        &mut output,
                        "The SEARCH section must exactly match an existing block of lines including all white \
                        space, comments, indentation, docstrings, etc."
                    )?;
                }

                if !parse_errors.is_empty() {
                    writeln!(
                        &mut output,
                        "\n\n## {} SEARCH/REPLACE blocks failed to parse:",
                        parse_errors.len()
                    )?;

                    for error in parse_errors {
                        writeln!(&mut output, "- {}", error)?;
                    }
                }

                if has_errors {
                    writeln!(
                        &mut output,
                        "\n\nYou can fix errors by running the tool again. You can include instructions, \
                        but errors are part of the conversation so you don't need to repeat them.",
                    )?;

                    Err(anyhow!(output))
                } else {
                    Ok(output)
                }
            }
        }
    }
}
