use super::find_channel_by_name;
use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{Tool, ToolResult, ToolResultContent, ToolResultOutput};
use assistant_tools::{EditAgent, EditAgentOutputEvent, Templates};
use channel::ChannelStore;
use futures::StreamExt;
use gpui::{App, Entity, Task};
use icons::IconName;
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct StreamingEditChannelNotesTool {
    channel_store: Entity<ChannelStore>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct StreamingEditChannelNotesInput {
    /// The name of the channel whose notes to edit
    channel: String,
    /// A one-line, user-friendly markdown description of the edit.
    /// Be terse, but also descriptive in what you want to achieve with this edit.
    display_description: String,
    /// The edit mode: "edit" to modify existing content, "overwrite" to replace entire content
    #[serde(default)]
    mode: EditMode,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum EditMode {
    #[default]
    Edit,
    Overwrite,
}

impl StreamingEditChannelNotesTool {
    pub fn new(channel_store: Entity<ChannelStore>) -> Self {
        Self { channel_store }
    }
}

impl Tool for StreamingEditChannelNotesTool {
    fn name(&self) -> String {
        "streaming_edit_channel_notes".to_string()
    }

    fn description(&self) -> String {
        "Edit channel notes using AI-powered streaming edits for efficient collaborative editing"
            .to_string()
    }

    fn icon(&self) -> IconName {
        IconName::FileText
    }

    fn needs_confirmation(&self, _input: &serde_json::Value, _cx: &App) -> bool {
        false
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<StreamingEditChannelNotesInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        let Ok(input) = serde_json::from_value::<StreamingEditChannelNotesInput>(input.clone())
        else {
            return "Streaming edit channel notes (invalid input)".to_string();
        };

        if input.display_description.is_empty() {
            format!("Streaming edit notes for channel '{}'", input.channel)
        } else {
            format!(
                "{} in channel '{}'",
                input.display_description, input.channel
            )
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        request: Arc<LanguageModelRequest>,
        project: Entity<Project>,
        action_log: Entity<assistant_tool::ActionLog>,
        model: Arc<dyn LanguageModel>,
        _window: Option<gpui::AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input: StreamingEditChannelNotesInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => {
                return ToolResult::from(Task::ready(Err(anyhow!("Invalid input: {}", err))));
            }
        };

        let channel_store = self.channel_store.clone();
        let channel_name = input.channel.clone();

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

        let task = cx.spawn(async move |mut cx| {
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

            // Get the language buffer from the channel buffer
            let buffer = cx.update(|cx| {
                channel_buffer.read_with(cx, |channel_buffer, _| channel_buffer.buffer().clone())
            })?;

            // Create an EditAgent for streaming edits
            let edit_agent = EditAgent::new(model, project, action_log, Templates::new());

            // Perform the edit using EditAgent
            let (output, mut events) = match input.mode {
                EditMode::Edit => {
                    edit_agent.edit(buffer.clone(), input.display_description, &request, &mut cx)
                }
                EditMode::Overwrite => edit_agent.overwrite(
                    buffer.clone(),
                    input.display_description,
                    &request,
                    &mut cx,
                ),
            };

            // Process streaming events
            let mut edit_completed = false;
            let mut unresolved_range = false;
            while let Some(event) = events.next().await {
                match event {
                    EditAgentOutputEvent::Edited => {
                        edit_completed = true;
                        // Acknowledge the buffer version to sync changes
                        cx.update(|cx| {
                            channel_buffer.update(cx, |channel_buffer, cx| {
                                channel_buffer.acknowledge_buffer_version(cx);
                            })
                        })?;
                    }
                    EditAgentOutputEvent::UnresolvedEditRange => {
                        unresolved_range = true;
                        log::warn!("AI couldn't resolve edit range in channel notes");
                    }
                    EditAgentOutputEvent::ResolvingEditRange(_) => {
                        // This is primarily for UI updates, which we don't need here
                    }
                }
            }

            let _result = output.await?;

            if !edit_completed && !unresolved_range {
                return Err(anyhow!("Edit was not completed successfully"));
            }

            let message = if unresolved_range {
                format!(
                    "Applied {} to channel '{}' (some ranges could not be resolved)",
                    match input.mode {
                        EditMode::Edit => "streaming edits",
                        EditMode::Overwrite => "streaming overwrite",
                    },
                    channel_name
                )
            } else {
                format!(
                    "Applied {} to notes for channel '{}'",
                    match input.mode {
                        EditMode::Edit => "streaming edits",
                        EditMode::Overwrite => "streaming overwrite",
                    },
                    channel_name
                )
            };

            Ok(ToolResultOutput {
                content: ToolResultContent::Text(message),
                output: None,
            })
        });

        ToolResult::from(task)
    }
}
