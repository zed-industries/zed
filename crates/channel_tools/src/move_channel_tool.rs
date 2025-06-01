use super::find_channel_by_name;
use anyhow::{Result, anyhow};
use assistant_tool::{Tool, ToolResult, ToolResultOutput};
use channel::ChannelStore;
use gpui::{App, Entity, Task};
use icons::IconName;
use language_model::{LanguageModelRequest, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct MoveChannelTool {
    channel_store: Entity<ChannelStore>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct MoveChannelInput {
    /// The name of the channel to move
    channel: String,
    /// The name of the new parent channel (null to move to root)
    to: Option<String>,
}

impl MoveChannelTool {
    pub fn new(channel_store: Entity<ChannelStore>) -> Self {
        Self { channel_store }
    }
}

impl Tool for MoveChannelTool {
    fn name(&self) -> String {
        "move_channel".to_string()
    }

    fn description(&self) -> String {
        "Move a channel to a different parent or to the root level".to_string()
    }

    fn icon(&self) -> IconName {
        IconName::FolderOpen
    }

    fn needs_confirmation(&self, _input: &serde_json::Value, _cx: &App) -> bool {
        // Moving channels is a significant operation that should be confirmed
        true
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        let schema = schemars::schema_for!(MoveChannelInput);
        let mut json = serde_json::to_value(schema)?;

        match format {
            LanguageModelToolSchemaFormat::JsonSchema => Ok(json),
            LanguageModelToolSchemaFormat::JsonSchemaSubset => {
                assistant_tool::adapt_schema_to_format(&mut json, format)?;
                Ok(json)
            }
        }
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        let Ok(input) = serde_json::from_value::<MoveChannelInput>(input.clone()) else {
            return "Move channel (invalid input)".to_string();
        };

        if let Some(to) = &input.to {
            format!("Move channel '{}' to '{}'", input.channel, to)
        } else {
            format!("Move channel '{}' to root", input.channel)
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _request: Arc<LanguageModelRequest>,
        _project: Entity<Project>,
        _action_log: Entity<assistant_tool::ActionLog>,
        _model: Arc<dyn language_model::LanguageModel>,
        _window: Option<gpui::AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input: MoveChannelInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => {
                return ToolResult::from(Task::ready(Err(anyhow!("Invalid input: {}", err))));
            }
        };

        let channel_store = self.channel_store.clone();
        let channel_name = input.channel.clone();
        let to_name = input.to.clone();

        // Find the channel to move
        let (channel_id, _) = match find_channel_by_name(&channel_store, &channel_name, cx) {
            Some(channel) => channel,
            None => {
                return ToolResult::from(Task::ready(Err(anyhow!(
                    "Channel '{}' not found",
                    channel_name
                ))));
            }
        };

        // Find the target parent channel if specified
        let new_parent_id = if let Some(to_name) = &to_name {
            match find_channel_by_name(&channel_store, to_name, cx) {
                Some((id, _)) => Some(id),
                None => {
                    return ToolResult::from(Task::ready(Err(anyhow!(
                        "Target channel '{}' not found",
                        to_name
                    ))));
                }
            }
        } else {
            None
        };

        // Check if we're trying to move a channel to itself or its descendant
        if let Some(new_parent_id) = new_parent_id {
            if channel_id == new_parent_id {
                return ToolResult::from(Task::ready(Err(anyhow!(
                    "Cannot move channel to itself"
                ))));
            }

            // Check if new parent is a descendant of the channel being moved
            let store = channel_store.read(cx);
            if let Some(new_parent) = store.channel_for_id(new_parent_id) {
                if new_parent.parent_path.contains(&channel_id) {
                    return ToolResult::from(Task::ready(Err(anyhow!(
                        "Cannot move channel to one of its descendants"
                    ))));
                }
            }
        }

        let task = cx.spawn(async move |cx| {
            // Check if we're trying to move to root
            let new_parent_id = new_parent_id
                .ok_or_else(|| anyhow!("Moving channels to root is not currently supported"))?;

            let move_task = cx.update(|cx| {
                channel_store.update(cx, |store, cx| {
                    store.move_channel(channel_id, new_parent_id, cx)
                })
            })?;

            move_task.await?;

            let message = format!("Moved channel '{}' to '{}'", channel_name, to_name.unwrap());

            Ok(ToolResultOutput::from(message))
        });

        ToolResult::from(task)
    }
}
