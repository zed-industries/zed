use std::sync::Arc;

use action_log::ActionLog;
use anyhow::{Result, anyhow, bail};
use assistant_tool::{Tool, ToolResult, ToolSource};
use context_server::ContextServerId;
use gpui::{AnyWindowHandle, App, Entity, Task};
use icons::IconName;
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelToolSchemaFormat};
use project::{Project, context_server_store::ContextServerStore};

pub struct ContextServerTool {
    store: Entity<ContextServerStore>,
    server_id: ContextServerId,
    tool: rmcp::model::Tool,
}

impl ContextServerTool {
    pub fn new(
        store: Entity<ContextServerStore>,
        server_id: ContextServerId,
        tool: rmcp::model::Tool,
    ) -> Self {
        Self {
            store,
            server_id,
            tool,
        }
    }
}

impl Tool for ContextServerTool {
    fn name(&self) -> String {
        // Convert Cow<str> to String
        self.tool.name.to_string()
    }

    fn description(&self) -> String {
        // Convert Option<Cow<str>> to String
        self.tool
            .description
            .as_ref()
            .map(|d| d.to_string())
            .unwrap_or_default()
    }

    fn icon(&self) -> IconName {
        IconName::ToolHammer
    }

    fn source(&self) -> ToolSource {
        ToolSource::ContextServer {
            id: self.server_id.clone().0.into(),
        }
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &Entity<Project>, _: &App) -> bool {
        true
    }

    fn may_perform_edits(&self) -> bool {
        true
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        // Convert Arc<Map<String, Value>> to Value
        let schema_value = serde_json::Value::Object((*self.tool.input_schema).clone());
        let mut schema = schema_value.clone();

        assistant_tool::adapt_schema_to_format(&mut schema, format)?;

        Ok(match schema {
            serde_json::Value::Null => {
                serde_json::json!({ "type": "object", "properties": {} })
            }
            serde_json::Value::Object(ref map) if map.is_empty() => {
                serde_json::json!({ "type": "object", "properties": {} })
            }
            _ => schema,
        })
    }

    fn ui_text(&self, _input: &serde_json::Value) -> String {
        format!("Run MCP tool `{}`", self.tool.name)
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _request: Arc<LanguageModelRequest>,
        _project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        _model: Arc<dyn LanguageModel>,
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        if let Some(server) = self.store.read(cx).get_running_server(&self.server_id) {
            let tool_name = self.tool.name.clone();

            cx.spawn(async move |_cx| {
                let Some(service) = server.service() else {
                    bail!("Context server not initialized");
                };

                let arguments = if let serde_json::Value::Object(map) = input {
                    Some(map)
                } else {
                    None
                };

                log::trace!(
                    "Running tool: {} with arguments: {:?}",
                    tool_name,
                    arguments
                );
                let response = service
                    .call_tool(rmcp::model::CallToolRequestParam {
                        name: tool_name,
                        arguments,
                    })
                    .await?;

                let mut result = String::new();

                // Handle Annotated<RawContent> properly - access the 'raw' field
                for content_item in response.content {
                    // Access the raw field and match on tuple variants
                    match &content_item.raw {
                        rmcp::model::RawContent::Text(text_content) => {
                            // Text is a tuple variant containing a TextContent struct
                            result.push_str(&text_content.text);
                        }
                        rmcp::model::RawContent::Image(image_content) => {
                            // Image is a tuple variant containing an ImageContent struct
                            log::warn!(
                                "Ignoring image content from tool response (mime: {})",
                                image_content.mime_type
                            );
                        }
                        rmcp::model::RawContent::Resource(resource_content) => {
                            // EmbeddedResource is a tuple variant containing an EmbeddedResourceContent struct
                            log::warn!(
                                "Ignoring embedded resource content from tool response: {:?}",
                                resource_content.resource
                            );
                        }
                        rmcp::model::RawContent::ResourceLink(resource_link_content) => {
                            // EmbeddedResource is a tuple variant containing an EmbeddedResourceContent struct
                            log::warn!(
                                "Ignoring embedded resource content from tool response: {:?}",
                                resource_link_content.uri
                            );
                        }
                        rmcp::model::RawContent::Audio(audio_content) => {
                            // EmbeddedResource is a tuple variant containing an EmbeddedResourceContent struct
                            log::warn!(
                                "Ignoring audio content from tool response (mime: {})",
                                audio_content.mime_type
                            );
                        }
                    }
                }

                Ok(result.into())
            })
            .into()
        } else {
            Task::ready(Err(anyhow!("Context server not found"))).into()
        }
    }
}
