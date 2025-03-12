mod edit_action;

use collections::HashSet;
use std::{path::Path, sync::Arc};

use anyhow::{anyhow, Result};
use assistant_tool::Tool;
use edit_action::{EditAction, EditActionParser};
use futures::StreamExt;
use gpui::{App, Entity, Task};
use language_model::{
    LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage, Role,
};
use project::{Project, ProjectPath, WorktreeId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditFilesToolInput {
    /// The ID of the worktree in which the files reside.
    pub worktree_id: usize,
    /// Instruct how to modify the files.
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

            while let Some(chunk) = chunks.stream.next().await {
                for action in parser.parse_chunk(&chunk?) {
                    let project_path = ProjectPath {
                        worktree_id: WorktreeId::from_usize(input.worktree_id),
                        path: Path::new(action.file_path()).into(),
                    };

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

            // Save each buffer once at the end
            for buffer in changed_buffers {
                project
                    .update(&mut cx, |project, cx| project.save_buffer(buffer, cx))?
                    .await?;
            }

            let errors = parser.errors();

            if errors.is_empty() {
                Ok("Successfully applied all edits".into())
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
