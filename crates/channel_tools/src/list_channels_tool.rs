use crate::schema::json_schema_for;
use anyhow::Result;
use assistant_tool::{Tool, ToolResult, ToolResultContent, ToolResultOutput};
use channel::ChannelStore;
use gpui::{App, Entity};
use icons::IconName;
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct ListChannelsTool {
    channel_store: Entity<ChannelStore>,
}

impl ListChannelsTool {
    pub fn new(channel_store: Entity<ChannelStore>) -> Self {
        Self { channel_store }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListChannelsInput {
    /// Optional filter to show only channels with names containing this string (case-insensitive)
    #[serde(default)]
    filter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelInfo {
    id: u64,
    name: String,
    parent_path: Vec<String>,
    visibility: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    participant_count: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListChannelsOutput {
    channels: Vec<ChannelInfo>,
    total_count: usize,
}

impl Tool for ListChannelsTool {
    fn name(&self) -> String {
        "list_channels".to_string()
    }

    fn needs_confirmation(&self, _input: &serde_json::Value, _cx: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        "List available channels in the workspace".to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Hash
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<ListChannelsInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        let input = serde_json::from_value::<ListChannelsInput>(input.clone())
            .unwrap_or(ListChannelsInput { filter: None });

        let mut text = "Listing channels".to_string();
        if let Some(ref filter) = input.filter {
            text.push_str(&format!(" containing '{}'", filter));
        }
        text
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
        let channel_store = self.channel_store.clone();

        let Ok(input) = serde_json::from_value::<ListChannelsInput>(input) else {
            return ToolResult::from(gpui::Task::ready(Err(anyhow::anyhow!(
                "Invalid input parameters"
            ))));
        };

        let filter = input.filter.clone();

        cx.spawn(async move |cx| {
            let channels = cx
                .update(|cx| {
                    let store = channel_store.read(cx);
                    let mut channel_infos = Vec::new();

                    for channel in store.channels() {
                        // Apply filter if provided
                        if let Some(ref filter_text) = filter {
                            if !channel
                                .name
                                .to_lowercase()
                                .contains(&filter_text.to_lowercase())
                            {
                                continue;
                            }
                        }

                        // Build parent path names
                        let mut parent_path_names = Vec::new();
                        for parent_id in &channel.parent_path {
                            if let Some(parent) = store.channel_for_id(*parent_id) {
                                parent_path_names.push(parent.name.to_string());
                            }
                        }

                        let visibility = match channel.visibility {
                            rpc::proto::ChannelVisibility::Public => "public",
                            rpc::proto::ChannelVisibility::Members => "members",
                        };

                        let participant_count = Some(store.channel_participants(channel.id).len());

                        channel_infos.push(ChannelInfo {
                            id: channel.id.0,
                            name: channel.name.to_string(),
                            parent_path: parent_path_names,
                            visibility: visibility.to_string(),
                            participant_count,
                        });
                    }

                    channel_infos
                })
                .ok()
                .unwrap_or_default();

            let total_count = channels.len();
            let output = ListChannelsOutput {
                channels,
                total_count,
            };

            let mut text = String::new();
            if output.channels.is_empty() {
                text.push_str("No channels found");
            } else {
                text.push_str(&format!("Found {} channels:\n\n", output.total_count));
                for channel in &output.channels {
                    if !channel.parent_path.is_empty() {
                        text.push_str(&channel.parent_path.join(" > "));
                        text.push_str(" > ");
                    }
                    text.push_str(&channel.name);
                    text.push_str(&format!(" ({})", channel.visibility));
                    if let Some(count) = channel.participant_count {
                        text.push_str(&format!(" - {} participants", count));
                    }
                    text.push('\n');
                }
            }

            Ok(ToolResultOutput {
                content: ToolResultContent::Text(text),
                output: Some(serde_json::to_value(output).unwrap_or(serde_json::Value::Null)),
            })
        })
        .into()
    }
}
