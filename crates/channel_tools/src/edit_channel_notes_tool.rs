use super::find_channel_by_name;
use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{Tool, ToolResult, ToolResultContent, ToolResultOutput};
use channel::{ChannelBuffer, ChannelStore};
use gpui::{App, Entity, Task};
use icons::IconName;
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use text::Point;

pub struct EditChannelNotesTool {
    channel_store: Entity<ChannelStore>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct EditChannelNotesInput {
    /// The name of the channel whose notes to edit
    channel: String,
    /// The edits to apply to the channel notes
    edits: Vec<ChannelNotesEdit>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct ChannelNotesEdit {
    /// The kind of edit: "create", "edit", or "append"
    kind: EditKind,
    /// The content to insert, replace, or append
    content: String,
    /// For "edit" kind: the range to replace (optional, defaults to entire buffer)
    #[serde(default)]
    range: Option<EditRange>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
enum EditKind {
    Create,
    Edit,
    Append,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct EditRange {
    /// Starting line (0-based)
    start_line: u32,
    /// Starting column (0-based)
    start_column: u32,
    /// Ending line (0-based)
    end_line: u32,
    /// Ending column (0-based)
    end_column: u32,
}

impl EditChannelNotesTool {
    pub fn new(channel_store: Entity<ChannelStore>) -> Self {
        Self { channel_store }
    }

    fn apply_edits(
        channel_buffer: &Entity<ChannelBuffer>,
        edits: Vec<ChannelNotesEdit>,
        cx: &mut App,
    ) -> Result<()> {
        channel_buffer.update(cx, |channel_buffer, cx| {
            let buffer = channel_buffer.buffer();
            buffer.update(cx, |buffer, cx| {
                for edit in edits {
                    match edit.kind {
                        EditKind::Create => {
                            // Replace entire content
                            let len = buffer.len();
                            buffer.edit([(0..len, edit.content)], None, cx);
                        }
                        EditKind::Edit => {
                            if let Some(range) = edit.range {
                                let start = Point::new(range.start_line, range.start_column);
                                let end = Point::new(range.end_line, range.end_column);
                                let start_offset = buffer.point_to_offset(start);
                                let end_offset = buffer.point_to_offset(end);
                                buffer.edit([(start_offset..end_offset, edit.content)], None, cx);
                            } else {
                                // Replace entire content if no range specified
                                let len = buffer.len();
                                buffer.edit([(0..len, edit.content)], None, cx);
                            }
                        }
                        EditKind::Append => {
                            let len = buffer.len();
                            buffer.edit([(len..len, edit.content)], None, cx);
                        }
                    }
                }
            });

            // Acknowledge the buffer version to sync changes
            channel_buffer.acknowledge_buffer_version(cx);
        });

        Ok(())
    }
}

impl Tool for EditChannelNotesTool {
    fn name(&self) -> String {
        "edit_channel_notes".to_string()
    }

    fn description(&self) -> String {
        "Edit the notes of a channel collaboratively".to_string()
    }

    fn icon(&self) -> IconName {
        IconName::FileText
    }

    fn needs_confirmation(&self, _input: &serde_json::Value, _cx: &App) -> bool {
        false
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<EditChannelNotesInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        let Ok(input) = serde_json::from_value::<EditChannelNotesInput>(input.clone()) else {
            return "Edit channel notes (invalid input)".to_string();
        };

        let action = if input.edits.len() == 1 {
            match &input.edits[0].kind {
                EditKind::Create => "Create notes for",
                EditKind::Edit => "Edit notes for",
                EditKind::Append => "Append to notes for",
            }
        } else {
            "Apply multiple edits to notes for"
        };

        format!("{} channel '{}'", action, input.channel)
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _request: Arc<LanguageModelRequest>,
        _project: Entity<Project>,
        _action_log: Entity<assistant_tool::ActionLog>,
        _model: Arc<dyn LanguageModel>,
        _window: Option<gpui::AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input: EditChannelNotesInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => {
                return ToolResult::from(Task::ready(Err(anyhow!("Invalid input: {}", err))));
            }
        };

        if input.edits.is_empty() {
            return ToolResult::from(Task::ready(Err(anyhow!("No edits provided"))));
        }

        let channel_store = self.channel_store.clone();
        let channel_name = input.channel.clone();
        let edits = input.edits;

        // Find the channel
        let (channel_id, _) = match find_channel_by_name(&channel_store, &channel_name, cx) {
            Some(channel) => channel,
            None => {
                return ToolResult::from(Task::ready(Err(anyhow!(
                    "Channel '{}' not found",
                    channel_name
                ))));
            }
        };

        let task = cx.spawn(async move |cx| {
            // Open the channel buffer
            let channel_buffer = cx
                .update(|cx| {
                    channel_store.update(cx, |store, cx| store.open_channel_buffer(channel_id, cx))
                })?
                .await
                .map_err(|e| anyhow!("Failed to open channel buffer: {}", e))?;

            // Check if the buffer is connected
            cx.update(|cx| {
                if !channel_buffer.read(cx).is_connected() {
                    return Err(anyhow!("Channel buffer is not connected"));
                }
                Ok(())
            })??;

            // Apply the edits
            cx.update(|cx| Self::apply_edits(&channel_buffer, edits, cx))??;

            let message = format!("Edited notes for channel '{}'", channel_name);
            Ok(ToolResultOutput {
                content: ToolResultContent::Text(message),
                output: None,
            })
        });

        ToolResult::from(task)
    }
}
