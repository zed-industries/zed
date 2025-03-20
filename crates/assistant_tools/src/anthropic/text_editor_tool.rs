use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool, ToolSource};
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

        // Handle each command type
        match input {
            TextEditorToolInput::View { path, view_range } => {
                // TODO: Implement view functionality
                Task::ready(Ok(format!(
                    "View command not yet implemented for path: {}",
                    path.display()
                )))
            }
            TextEditorToolInput::StrReplace {
                path,
                old_str,
                new_str,
            } => {
                // TODO: Implement replace functionality
                Task::ready(Ok(format!(
                    "Replace command not yet implemented for path: {}",
                    path.display()
                )))
            }
            TextEditorToolInput::Create { path, file_text } => {
                // TODO: Implement create functionality
                Task::ready(Ok(format!(
                    "Create command not yet implemented for path: {}",
                    path.display()
                )))
            }
            TextEditorToolInput::Insert {
                path,
                insert_line,
                new_str,
            } => {
                // TODO: Implement insert functionality
                Task::ready(Ok(format!(
                    "Insert command not yet implemented for path: {}",
                    path.display()
                )))
            }
            TextEditorToolInput::UndoEdit { path } => {
                // TODO: Implement undo functionality
                Task::ready(Ok(format!(
                    "Undo command not yet implemented for path: {}",
                    path.display()
                )))
            }
        }
    }
}
