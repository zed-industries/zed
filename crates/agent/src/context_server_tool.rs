use std::sync::Arc;

use anyhow::{Result, anyhow, bail};
use assistant_tool::{ActionLog, Tool, ToolResult, ToolSource};
use context_server::{ContextServerId, types};
use gpui::{AnyWindowHandle, App, Entity, Task};
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelToolSchemaFormat};
use project::{Project, context_server_store::ContextServerStore};
use ui::IconName;

pub struct ContextServerTool {
    store: Entity<ContextServerStore>,
    server_id: ContextServerId,
    tool: types::Tool,
}

impl ContextServerTool {
    pub fn new(
        store: Entity<ContextServerStore>,
        server_id: ContextServerId,
        tool: types::Tool,
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
        format!("{}-{}", self.server_id, self.tool.name)
    }

    fn ui_name(&self) -> String {
        self.tool.name.clone()
    }

    fn description(&self) -> String {
        self.tool.description.clone().unwrap_or_default()
    }

    fn icon(&self) -> IconName {
        IconName::Cog
    }

    fn source(&self) -> ToolSource {
        ToolSource::ContextServer {
            id: self.server_id.clone().0.into(),
        }
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        true
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        let mut schema = self.tool.input_schema.clone();
        assistant_tool::adapt_schema_to_format(&mut schema, format)?;
        Ok(match schema {
            serde_json::Value::Null => {
                serde_json::json!({ "type": "object", "properties": [] })
            }
            serde_json::Value::Object(map) if map.is_empty() => {
                serde_json::json!({ "type": "object", "properties": [] })
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
            let server_clone = server.clone();
            let input_clone = input.clone();

            cx.spawn(async move |_cx| {
                let Some(protocol) = server_clone.client() else {
                    bail!("Context server not initialized");
                };

                let arguments = if let serde_json::Value::Object(map) = input_clone {
                    Some(map.into_iter().collect())
                } else {
                    None
                };

                log::trace!(
                    "Running tool: {} with arguments: {:?}",
                    tool_name,
                    arguments
                );
                let response = protocol.run_tool(tool_name, arguments).await?;

                let mut result = String::new();
                for content in response.content {
                    match content {
                        types::ToolResponseContent::Text { text } => {
                            result.push_str(&text);
                        }
                        types::ToolResponseContent::Image { .. } => {
                            log::warn!("Ignoring image content from tool response");
                        }
                        types::ToolResponseContent::Resource { .. } => {
                            log::warn!("Ignoring resource content from tool response");
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
