use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool, ToolSource};
use collections::HashSet;
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum TextEditorToolInput {
    /// View file contents or directory listing
    View {
        /// The path to the file or directory to view
        path: Arc<Path>,
        /// Optional line range to view (1-based, inclusive)
        #[serde(default)]
        view_range: Option<(usize, usize)>,
    },

    /// Replace specific text in a file
    StrReplace {
        /// The path to the file to modify
        path: Arc<Path>,
        /// The text to replace (must match exactly)
        old_str: String,
        /// The new text to insert
        new_str: String,
    },

    /// Create a new file with content
    Create {
        /// The path where the new file should be created
        path: Arc<Path>,
        /// The content to write to the new file
        file_text: String,
    },

    /// Insert text at a specific line
    Insert {
        /// The path to the file to modify
        path: Arc<Path>,
        /// The line number after which to insert (0 for beginning)
        insert_line: usize,
        /// The text to insert
        new_str: String,
    },

    /// Undo the last edit made to a file
    UndoEdit {
        /// The path to the file whose last edit should be undone
        path: Arc<Path>,
    },
}

impl TextEditorToolInput {
    pub fn path(&self) -> &Arc<Path> {
        match self {
            TextEditorToolInput::View { path, .. } => path,
            TextEditorToolInput::StrReplace { path, .. } => path,
            TextEditorToolInput::Create { path, .. } => path,
            TextEditorToolInput::Insert { path, .. } => path,
            TextEditorToolInput::UndoEdit { path } => path,
        }
    }
}

pub struct TextEditorTool;

impl Tool for TextEditorTool {
    fn name(&self) -> String {
        "str_replace_editor".into()
    }

    fn description(&self) -> String {
        String::new()
    }

    fn source(&self) -> ToolSource {
        ToolSource::RefactorMeProviderDefined {
            provider_name: "Anthropic",
            // todo! Depends on model!
            tool_type: "text_editor_20250124",
        }
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(TextEditorToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<TextEditorToolInput>(input.clone()) {
            Ok(input) => match input {
                TextEditorToolInput::View { path, .. } => format!("View file `{}`", path.display()),
                TextEditorToolInput::StrReplace { path, .. } => {
                    format!("Edit file `{}`", path.display())
                }
                TextEditorToolInput::Create { path, .. } => {
                    format!("Create file `{}`", path.display())
                }
                TextEditorToolInput::Insert { path, .. } => {
                    format!("Edit file `{}`", path.display())
                }
                TextEditorToolInput::UndoEdit { path } => {
                    format!("Undo edit in `{}`", path.display())
                }
            },
            Err(_) => "Edit file".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        // Basic input validation
        let input = match serde_json::from_value::<TextEditorToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        let path = input.path().clone();

        let project_path = if path.is_absolute() {
            // todo! how can we tell the model to not use abs paths
            project.read(cx).project_path_for_absolute_path(&path, cx)
        } else {
            project.read(cx).find_project_path(&path, cx)
        };

        let Some(project_path) = project_path else {
            return Task::ready(Err(anyhow!("Path {} not found in project", path.display())));
        };

        let mut changed = true;

        cx.spawn(async move |cx| {
            let buffer = project
                .update(cx, |project, cx| project.open_buffer(project_path, cx))?
                .await?;

            // Handle each command type
            let result = match input {
                TextEditorToolInput::View { view_range, .. } => {
                    changed = false;

                    format!(
                        "View command not yet implemented for path: {}",
                        path.display()
                    )
                }
                TextEditorToolInput::StrReplace {
                    path,
                    old_str,
                    new_str,
                } => {
                    // TODO: Implement replace functionality
                    format!(
                        "Replace command not yet implemented for path: {}",
                        path.display()
                    )
                }
                TextEditorToolInput::Create { path, file_text } => {
                    buffer.update(cx, |buffer, cx| buffer.set_text(file_text, cx))?;
                    format!("Created `{}`", path.display())
                }
                TextEditorToolInput::Insert {
                    path,
                    insert_line,
                    new_str,
                } => {
                    // TODO: Implement insert functionality
                    format!(
                        "Insert command not yet implemented for path: {}",
                        path.display()
                    )
                }
                TextEditorToolInput::UndoEdit { path } => {
                    // TODO: Implement undo functionality
                    format!(
                        "Undo command not yet implemented for path: {}",
                        path.display()
                    )
                }
            };

            if changed {
                project
                    .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))?
                    .await?;

                action_log.update(cx, |log, cx| {
                    let mut changed_buffers = HashSet::default();
                    changed_buffers.insert(buffer);

                    log.buffer_edited(changed_buffers, cx);
                })?;
            }

            Ok(result)
        })
    }
}
