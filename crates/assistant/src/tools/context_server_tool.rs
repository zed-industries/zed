use anyhow::anyhow;
use assistant_tool::Tool;
use context_servers::manager::ContextServerManager;
use gpui::Task;

pub struct SlashCommandTool {
    server_id: String,
    name: String,
    description: String,
    input_schema: serde_json::Value,
}

impl SlashCommandTool {
    pub fn new<S: Into<String>>(
        server_id: S,
        name: S,
        description: S,
        input_schema: serde_json::Value,
    ) -> Self {
        Self {
            server_id: server_id.into(),
            name: name.into(),
            description: description.into(),
            input_schema,
        }
    }
}

impl Tool for SlashCommandTool {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn run(
        self: std::sync::Arc<Self>,
        input: serde_json::Value,
        _workspace: gpui::WeakView<workspace::Workspace>,
        cx: &mut ui::WindowContext,
    ) -> gpui::Task<gpui::Result<String>> {
        let tool_name = self.name.clone();

        let manager = ContextServerManager::global(cx);
        let manager = manager.read(cx);
        if let Some(server) = manager.get_server(&self.server_id) {
            cx.foreground_executor().spawn(async move {
                let Some(protocol) = server.client.read().clone() else {
                    return Err(anyhow!("Context server not initialized"));
                };

                // Convert input to our expect map from parameters to serde_json::Value
                let arguments = if let serde_json::Value::Object(map) = input {
                    Some(map.into_iter().collect())
                } else {
                    None
                };

                let response = protocol.run_tool(tool_name, arguments).await?;

                let tool_result = response.tool_result;
                if let serde_json::Value::String(result_string) = tool_result {
                    Ok(result_string)
                } else {
                    Err(anyhow!("Tool response did not contain a string value"))
                }
            })
        } else {
            Task::ready(Err(anyhow!("Context server not found")))
        }
    }
}
