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
use project::{Project, ProjectPath};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::sync::Arc;
use util::ResultExt;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditFilesToolInput {
    /// High-level edit instructions. These will be interpreted by a smaller model,
    /// so explain the edits you want that model to make and to which files need changing.
    /// The description should be concise and clear. We will show this description to the user
    /// as well.
    ///
    /// <example>
    /// If you want to rename a function you can say "Rename the function 'foo' to 'bar'".
    /// </example>
    ///
    /// <example>
    /// If you want to add a new function you can say "Add a new method to the `User` struct that prints the age".
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
                temperature: None,
            };

            let mut parser = EditActionParser::new();

            let stream = model.stream_completion_text(request, &cx);
            let mut chunks = stream.await?;

            let mut changed_buffers = HashSet::default();
            let mut applied_edits = 0;

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

                    let diff = buffer
                        .read_with(&cx, |buffer, cx| {
                            let new_text = match action {
                                EditAction::Replace { old, new, .. } => {
                                    // TODO: Replace in background?
                                    buffer.text().replace(&old, &new)
                                }
                                EditAction::Write { content, .. } => content,
                            };

                            buffer.diff(new_text, cx)
                        })?
                        .await;

                    let _clock =
                        buffer.update(&mut cx, |buffer, cx| buffer.apply_diff(diff, cx))?;

                    changed_buffers.insert(buffer);

                    applied_edits += 1;
                }
            }

            let mut answer = match changed_buffers.len() {
                0 => "No files were edited.".to_string(),
                1 => "Successfully edited ".to_string(),
                _ => "Successfully edited these files:\n\n".to_string(),
            };

            // Save each buffer once at the end
            for buffer in changed_buffers {
                project
                    .update(&mut cx, |project, cx| {
                        if let Some(file) = buffer.read(&cx).file() {
                            let _ = writeln!(&mut answer, "{}", &file.path().display());
                        }

                        project.save_buffer(buffer, cx)
                    })?
                    .await?;
            }

            let errors = parser.errors();

            if errors.is_empty() {
                Ok(answer.trim_end().to_string())
            } else {
                let error_message = errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");

                if applied_edits > 0 {
                    Err(anyhow!(
                        "Applied {} edit(s), but some blocks failed to parse:\n{}",
                        applied_edits,
                        error_message
                    ))
                } else {
                    Err(anyhow!(error_message))
                }
            }
        })
    }
}
