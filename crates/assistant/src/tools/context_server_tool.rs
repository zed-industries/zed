use std::sync::Arc;

use anyhow::{anyhow, bail};
use assistant_tool::Tool;
use context_servers::manager::ContextServerManager;
use context_servers::types;
use gpui::{Model, Task};

pub struct ContextServerTool {
    server_manager: Model<ContextServerManager>,
    server_id: Arc<str>,
    tool: types::Tool,
}

impl ContextServerTool {
    pub fn new(
        server_manager: Model<ContextServerManager>,
        server_id: impl Into<Arc<str>>,
        tool: types::Tool,
    ) -> Self {
        Self {
            server_manager,
            server_id: server_id.into(),
            tool,
        }
    }
}

impl Tool for ContextServerTool {
    fn name(&self) -> String {
        self.tool.name.clone()
    }

    fn description(&self) -> String {
        self.tool.description.clone().unwrap_or_default()
    }

    fn input_schema(&self) -> serde_json::Value {
        match &self.tool.input_schema {
            serde_json::Value::Null => {
                serde_json::json!({ "type": "object", "properties": [] })
            }
            serde_json::Value::Object(map) if map.is_empty() => {
                serde_json::json!({ "type": "object", "properties": [] })
            }
            _ => self.tool.input_schema.clone(),
        }
    }

    fn run(
        self: std::sync::Arc<Self>,
        input: serde_json::Value,
        _workspace: gpui::WeakView<workspace::Workspace>,
        cx: &mut ui::WindowContext,
    ) -> gpui::Task<gpui::Result<String>> {
        if let Some(server) = self.server_manager.read(cx).get_server(&self.server_id) {
            cx.foreground_executor().spawn({
                let tool_name = self.tool.name.clone();
                async move {
                    let Some(protocol) = server.client() else {
                        bail!("Context server not initialized");
                    };

                    let arguments = if let serde_json::Value::Object(map) = input {
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

                    let tool_result = match response.tool_result {
                        serde_json::Value::String(s) => s,
                        _ => serde_json::to_string(&response.tool_result)?,
                    };
                    Ok(tool_result)
                }
            })
        } else {
            Task::ready(Err(anyhow!("Context server not found")))
        }
    }
}
