mod edit_action;

use anyhow::{anyhow, Context, Result};
use assistant_tool::Tool;
use collections::HashSet;
use edit_action::{EditAction, EditActionParser};
use futures::StreamExt;
use gpui::{App, Entity, Task};
use language_model::{
    LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage, Role,
};
use project::{search::SearchQuery, Project, ProjectPath};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::sync::Arc;
use util::paths::PathMatcher;

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

            struct BadSearch {
                file_path: String,
                search: String,
                matches: usize,
            }

            let mut bad_searches = Vec::new();

            while let Some(chunk) = chunks.stream.next().await {
                for action in parser.parse_chunk(&chunk?) {
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

                                    if matches.len() != 1 {
                                        return Ok(DiffResult::InvalidReplace(BadSearch {
                                            search: new.clone(),
                                            file_path: file_path.display().to_string(),
                                            matches: matches.len(),
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

                            applied_edits += 1;
                        }
                    }
                }
            }

            // Save each buffer once at the end
            for buffer in changed_buffers {
                project
                    .update(&mut cx, |project, cx| project.save_buffer(buffer, cx))?
                    .await?;
            }

            let errors = parser.errors();

            if errors.is_empty() && bad_searches.is_empty() {
                Ok("Successfully applied all edits".into())
            } else {
                let mut error_message = String::new();

                if !bad_searches.is_empty() {
                    writeln!(
                        &mut error_message,
                        "These searches failed because they didn't match exactly one string:"
                    )?;

                    for replace in bad_searches {
                        writeln!(
                            &mut error_message,
                            "- '{}' appears {} times in {}",
                            replace.search.replace("\r", "\\r").replace("\n", "\\n"),
                            replace.matches,
                            replace.file_path
                        )?;
                    }

                    writeln!(&mut error_message, "Make sure to use exact queries.")?;
                }

                if !errors.is_empty() {
                    if !error_message.is_empty() {
                        writeln!(&mut error_message, "\n")?;
                    }

                    writeln!(&mut error_message, "Some blocks failed to parse:")?;

                    for error in errors {
                        writeln!(&mut error_message, "- {}", error)?;
                    }
                }

                if applied_edits > 0 {
                    Err(anyhow!(
                        "Applied {} edit(s), but there were errors.\n\n{}",
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
