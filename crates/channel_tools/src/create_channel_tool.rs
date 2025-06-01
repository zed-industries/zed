use super::{ChannelVisibility, find_channel_by_name};
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

pub struct CreateChannelTool {
    channel_store: Entity<ChannelStore>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct CreateChannelInput {
    /// The name of the channel to create
    name: String,
    /// The name of the parent channel (optional, if not provided creates a root channel)
    #[serde(default)]
    parent: Option<String>,
    /// The visibility of the channel: "members" (default) or "public"
    #[serde(default = "default_visibility")]
    visibility: String,
}

fn default_visibility() -> String {
    "members".to_string()
}

impl CreateChannelTool {
    pub fn new(channel_store: Entity<ChannelStore>) -> Self {
        Self { channel_store }
    }
}

impl Tool for CreateChannelTool {
    fn name(&self) -> String {
        "create_channel".to_string()
    }

    fn description(&self) -> String {
        "Create a new channel in the workspace".to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Hash
    }

    fn needs_confirmation(&self, _input: &serde_json::Value, _cx: &App) -> bool {
        false
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        let schema = schemars::schema_for!(CreateChannelInput);
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
        let Ok(input) = serde_json::from_value::<CreateChannelInput>(input.clone()) else {
            return "Create channel (invalid input)".to_string();
        };

        if let Some(parent) = &input.parent {
            format!(
                "Create channel '{}' under '{}' (visibility: {})",
                input.name, parent, input.visibility
            )
        } else {
            format!(
                "Create channel '{}' (visibility: {})",
                input.name, input.visibility
            )
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
        let input: CreateChannelInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => {
                return ToolResult::from(Task::ready(Err(anyhow!("Invalid input: {}", err))));
            }
        };

        let visibility = match ChannelVisibility::from_str(&input.visibility) {
            Some(v) => v,
            None => {
                return ToolResult::from(Task::ready(Err(anyhow!(
                    "Invalid visibility '{}'. Use 'members' or 'public'",
                    input.visibility
                ))));
            }
        };

        let channel_store = self.channel_store.clone();
        let name = input.name.clone();
        let parent_name = input.parent.clone();

        // Find parent channel if specified
        let parent_id = if let Some(parent_name) = &parent_name {
            match find_channel_by_name(&channel_store, parent_name, cx) {
                Some((id, _)) => Some(id),
                None => {
                    return ToolResult::from(Task::ready(Err(anyhow!(
                        "Parent channel '{}' not found",
                        parent_name
                    ))));
                }
            }
        } else {
            None
        };

        let task = cx.spawn(async move |cx| {
            let create_task = cx.update(|cx| {
                channel_store.update(cx, |store, cx| store.create_channel(&name, parent_id, cx))
            })?;

            let channel_id = create_task.await?;

            // Set visibility if not default
            if visibility == ChannelVisibility::Public {
                let visibility_task = cx.update(|cx| {
                    channel_store.update(cx, |store, cx| {
                        store.set_channel_visibility(
                            channel_id,
                            rpc::proto::ChannelVisibility::Public,
                            cx,
                        )
                    })
                })?;
                visibility_task.await?;
            }

            let message = if let Some(parent_name) = parent_name {
                format!("Created channel '{}' under '{}'", name, parent_name)
            } else {
                format!("Created channel '{}'", name)
            };

            Ok(ToolResultOutput::from(message))
        });

        ToolResult::from(task)
    }
}
