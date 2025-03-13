mod edit_action;
pub mod log;

use anyhow::{anyhow, Context, Result};
use assistant_tool::Tool;
use collections::HashSet;
use edit_action::{EditAction, EditActionParser};
use futures::StreamExt;
use gpui::{App, Entity, Task};
use language_model::{
    LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage, Role,
};
use log::{EditToolLog, EditToolRequestId};
use project::{search::SearchQuery, Project, ProjectPath};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::sync::Arc;
use util::paths::PathMatcher;
use util::ResultExt;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditFilesToolInput {
    /// High-level edit instructions. These will be interpreted by a smaller
    /// model, so explain the changes you want that model to make and which
    /// file paths need changing.
    ///
    /// The description should be concise and clear. We will show this
    /// description to the user as well.
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
}

pub struct EditFilesTool;

impl Tool for EditFilesTool {
    fn name(&self) -> String {
        "edit-files".into()
    }

    fn description(&self) -> String {
        include_str!("./edit_files_tool/description.md").into()
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(EditFilesToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
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

                let task =
                    EditFilesTool::run(input, messages, project, Some((log.clone(), req_id)), cx);

                cx.spawn(|mut cx| async move {
                    let result = task.await;

                    let str_result = match &result {
                        Ok(out) => Ok(out.clone()),
                        Err(err) => Err(err.to_string()),
                    };

                    log.update(&mut cx, |log, cx| {
                        log.set_tool_output(req_id, str_result, cx)
                    })
                    .log_err();

                    result
                })
            }

            None => EditFilesTool::run(input, messages, project, None, cx),
        }
    }
}

impl EditFilesTool {
    fn run(
        input: EditFilesToolInput,
        messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        log: Option<(Entity<EditToolLog>, EditToolRequestId)>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let model_registry = LanguageModelRegistry::read_global(cx);
        let Some(model) = model_registry.editor_model() else {
            return Task::ready(Err(anyhow!("No editor model configured")));
        };

        let mut messages = messages.to_vec();
        if let Some(last_message) = messages.last_mut() {
            // Strip out tool use from the last message because we're in the middle of executing a tool call.
            last_message
                .content
                .retain(|content| !matches!(content, language_model::MessageContent::ToolUse(_)))
        }
        messages.push(LanguageModelRequestMessage {
            role: Role::User,
            content: vec![
                include_str!("./edit_files_tool/edit_prompt.md").into(),
                input.edit_instructions.into(),
            ],
            cache: false,
        });

        cx.spawn(|mut cx| async move {
            let request = LanguageModelRequest {
                messages,
                tools: vec![],
                stop: vec![],
                temperature: Some(0.0),
            };

            let mut parser = EditActionParser::new();

            let stream = model.stream_completion_text(request, &cx);
            let mut chunks = stream.await?;

            let mut changed_buffers = HashSet::default();
            let mut bad_searches = Vec::new();

            #[derive(Debug)]
            struct BadSearch {
                file_path: String,
                search: String,
            }

            let log = log.clone();

            while let Some(chunk) = chunks.stream.next().await {
                let chunk = chunk?;

                let new_actions = parser.parse_chunk(&chunk);

                if let Some((ref log, req_id)) = log {
                    log.update(&mut cx, |log, cx| {
                        log.push_editor_response_chunk(req_id, &chunk, &new_actions, cx)
                    })
                    .log_err();
                }

                for action in new_actions {
                    let project_path = project.read_with(&cx, |project, cx| {
                        let worktree_root_name = action
                            .file_path()
                            .components()
                            .next()
                            .context("Invalid path")?;
                        let worktree = project
                            .worktree_for_root_name(
                                &worktree_root_name.as_os_str().to_string_lossy(),
                                cx,
                            )
                            .context("Directory not found in project")?;
                        anyhow::Ok(ProjectPath {
                            worktree_id: worktree.read(cx).id(),
                            path: Arc::from(
                                action.file_path().strip_prefix(worktree_root_name).unwrap(),
                            ),
                        })
                    })??;

                    let buffer = project
                        .update(&mut cx, |project, cx| project.open_buffer(project_path, cx))?
                        .await?;

                    #[derive(Debug)]
                    enum DiffResult {
                        InvalidReplace(BadSearch),
                        Diff(language::Diff),
                    }

                    let result = match action {
                        EditAction::Replace {
                            old,
                            new,
                            file_path,
                        } => {
                            let snapshot =
                                buffer.read_with(&cx, |buffer, _cx| buffer.snapshot())?;

                            cx.background_executor()
                                .spawn(async move {
                                    let query = SearchQuery::text(
                                        old.clone(),
                                        false,
                                        true,
                                        true,
                                        PathMatcher::new(&[])?,
                                        PathMatcher::new(&[])?,
                                        None,
                                    )?;

                                    let matches = query.search(&snapshot, None).await;

                                    if matches.is_empty() {
                                        return Ok(DiffResult::InvalidReplace(BadSearch {
                                            search: new.clone(),
                                            file_path: file_path.display().to_string(),
                                        }));
                                    }

                                    let edit_range = matches[0].clone();
                                    let diff = language::text_diff(&old, &new);

                                    let edits = diff
                                        .into_iter()
                                        .map(|(old_range, text)| {
                                            let start = edit_range.start + old_range.start;
                                            let end = edit_range.start + old_range.end;
                                            (start..end, text)
                                        })
                                        .collect::<Vec<_>>();

                                    let diff = language::Diff {
                                        base_version: snapshot.version().clone(),
                                        line_ending: snapshot.line_ending(),
                                        edits,
                                    };

                                    anyhow::Ok(DiffResult::Diff(diff))
                                })
                                .await
                        }
                        EditAction::Write { content, .. } => Ok(DiffResult::Diff(
                            buffer
                                .read_with(&cx, |buffer, cx| buffer.diff(content, cx))?
                                .await,
                        )),
                    }?;

                    match result {
                        DiffResult::InvalidReplace(invalid_replace) => {
                            bad_searches.push(invalid_replace);
                        }
                        DiffResult::Diff(diff) => {
                            let _clock =
                                buffer.update(&mut cx, |buffer, cx| buffer.apply_diff(diff, cx))?;

                            changed_buffers.insert(buffer);
                        }
                    }
                }
            }

            let mut answer = match changed_buffers.len() {
                0 => "No files were edited.".to_string(),
                1 => "Successfully edited ".to_string(),
                _ => "Successfully edited these files:\n\n".to_string(),
            };

            // Save each buffer once at the end
            for buffer in changed_buffers {
                let (path, save_task) = project.update(&mut cx, |project, cx| {
                    let path = buffer
                        .read(cx)
                        .file()
                        .map(|file| file.path().display().to_string());

                    let task = project.save_buffer(buffer.clone(), cx);

                    (path, task)
                })?;

                save_task.await?;

                if let Some(path) = path {
                    writeln!(&mut answer, "{}", path)?;
                }
            }

            let errors = parser.errors();

            if errors.is_empty() && bad_searches.is_empty() {
                Ok(answer.trim_end().to_string())
            } else {
                writeln!(&mut answer, "\nThe following errors occurred:")?;

                if !bad_searches.is_empty() {
                    writeln!(
                        &mut answer,
                        "These searches failed because they didn't match any strings:"
                    )?;

                    for replace in bad_searches {
                        writeln!(
                            &mut answer,
                            "- '{}' does not appear in {}",
                            replace.search.replace("\r", "\\r").replace("\n", "\\n"),
                            replace.file_path
                        )?;
                    }

                    writeln!(&mut answer, "Make sure to use exact queries.")?;
                }

                if !errors.is_empty() {
                    if !answer.is_empty() {
                        writeln!(&mut answer, "\n")?;
                    }

                    writeln!(&mut answer, "These SEARCH/REPLACE blocks failed to parse:")?;

                    for error in errors {
                        writeln!(&mut answer, "- {}", error)?;
                    }
                }

                Err(anyhow!(answer))
            }
        })
    }
}
