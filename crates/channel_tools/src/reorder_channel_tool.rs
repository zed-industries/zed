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

pub struct ReorderChannelTool {
    channel_store: Entity<ChannelStore>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct ReorderChannelInput {
    /// The name of the channel to reorder
    channel: String,
    /// The direction to move the channel: "up" or "down"
    direction: String,
}

impl ReorderChannelTool {
    pub fn new(channel_store: Entity<ChannelStore>) -> Self {
        Self { channel_store }
    }
}

impl Tool for ReorderChannelTool {
    fn name(&self) -> String {
        "reorder_channel".to_string()
    }

    fn description(&self) -> String {
        "Move a channel up or down in the list among its siblings".to_string()
    }

    fn icon(&self) -> IconName {
        IconName::ListTree
    }

    fn needs_confirmation(&self, _input: &serde_json::Value, _cx: &App) -> bool {
        false
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        let schema = schemars::schema_for!(ReorderChannelInput);
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
        let Ok(input) = serde_json::from_value::<ReorderChannelInput>(input.clone()) else {
            return "Reorder channel (invalid input)".to_string();
        };

        format!("Move channel '{}' {}", input.channel, input.direction)
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
        let input: ReorderChannelInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => {
                return ToolResult::from(Task::ready(Err(anyhow!("Invalid input: {}", err))));
            }
        };

        let channel_store = self.channel_store.clone();
        let channel_name = input.channel.clone();
        let direction_str = input.direction.to_lowercase();

        // Parse direction
        let direction = match direction_str.as_str() {
            "up" => rpc::proto::reorder_channel::Direction::Up,
            "down" => rpc::proto::reorder_channel::Direction::Down,
            _ => {
                return ToolResult::from(Task::ready(Err(anyhow!(
                    "Invalid direction '{}'. Use 'up' or 'down'",
                    input.direction
                ))));
            }
        };

        // Find the channel to reorder
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
            let reorder_task = cx.update(|cx| {
                channel_store.update(cx, |store, cx| {
                    store.reorder_channel(channel_id, direction, cx)
                })
            })?;

            reorder_task.await?;

            let message = format!("Moved channel '{}' {}", channel_name, direction_str);

            Ok(ToolResultOutput::from(message))
        });

        ToolResult::from(task)
    }
}
