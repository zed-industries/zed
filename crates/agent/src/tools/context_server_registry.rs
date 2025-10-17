use crate::{AgentToolOutput, AnyAgentTool, ToolCallEventStream};
use agent_client_protocol::ToolKind;
use anyhow::{Result, anyhow, bail};
use collections::{BTreeMap, HashMap};
use context_server::ContextServerId;
use gpui::{App, Context, Entity, SharedString, Task};
use project::context_server_store::{ContextServerStatus, ContextServerStore};
use std::sync::Arc;
use util::ResultExt;

pub struct ContextServerRegistry {
    server_store: Entity<ContextServerStore>,
    registered_servers: HashMap<ContextServerId, RegisteredContextServer>,
    _subscription: gpui::Subscription,
}

struct RegisteredContextServer {
    tools: BTreeMap<SharedString, Arc<dyn AnyAgentTool>>,
    load_tools: Task<Result<()>>,
}

impl ContextServerRegistry {
    pub fn new(server_store: Entity<ContextServerStore>, cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            server_store: server_store.clone(),
            registered_servers: HashMap::default(),
            _subscription: cx.subscribe(&server_store, Self::handle_context_server_store_event),
        };
        for server in server_store.read(cx).running_servers() {
            this.reload_tools_for_server(server.id(), cx);
        }
        this
    }

    pub fn tools_for_server(
        &self,
        server_id: &ContextServerId,
    ) -> impl Iterator<Item = &Arc<dyn AnyAgentTool>> {
        self.registered_servers
            .get(server_id)
            .map(|server| server.tools.values())
            .into_iter()
            .flatten()
    }

    pub fn servers(
        &self,
    ) -> impl Iterator<
        Item = (
            &ContextServerId,
            &BTreeMap<SharedString, Arc<dyn AnyAgentTool>>,
        ),
    > {
        self.registered_servers
            .iter()
            .map(|(id, server)| (id, &server.tools))
    }

    fn reload_tools_for_server(&mut self, server_id: ContextServerId, cx: &mut Context<Self>) {
        let Some(server) = self.server_store.read(cx).get_running_server(&server_id) else {
            return;
        };
        let Some(client) = server.client() else {
            return;
        };
        if !client.capable(context_server::protocol::ServerCapability::Tools) {
            return;
        }

        let registered_server =
            self.registered_servers
                .entry(server_id.clone())
                .or_insert(RegisteredContextServer {
                    tools: BTreeMap::default(),
                    load_tools: Task::ready(Ok(())),
                });
        registered_server.load_tools = cx.spawn(async move |this, cx| {
            let response = client
                .request::<context_server::types::requests::ListTools>(())
                .await;

            this.update(cx, |this, cx| {
                let Some(registered_server) = this.registered_servers.get_mut(&server_id) else {
                    return;
                };

                registered_server.tools.clear();
                if let Some(response) = response.log_err() {
                    for tool in response.tools {
                        let tool = Arc::new(ContextServerTool::new(
                            this.server_store.clone(),
                            server.id(),
                            tool,
                        ));
                        registered_server.tools.insert(tool.name(), tool);
                    }
                    cx.notify();
                }
            })
        });
    }

    fn handle_context_server_store_event(
        &mut self,
        _: Entity<ContextServerStore>,
        event: &project::context_server_store::Event,
        cx: &mut Context<Self>,
    ) {
        match event {
            project::context_server_store::Event::ServerStatusChanged { server_id, status } => {
                match status {
                    ContextServerStatus::Starting => {}
                    ContextServerStatus::Running => {
                        self.reload_tools_for_server(server_id.clone(), cx);
                    }
                    ContextServerStatus::Stopped | ContextServerStatus::Error(_) => {
                        self.registered_servers.remove(server_id);
                        cx.notify();
                    }
                }
            }
        }
    }
}

struct ContextServerTool {
    store: Entity<ContextServerStore>,
    server_id: ContextServerId,
    tool: context_server::types::Tool,
}

impl ContextServerTool {
    fn new(
        store: Entity<ContextServerStore>,
        server_id: ContextServerId,
        tool: context_server::types::Tool,
    ) -> Self {
        Self {
            store,
            server_id,
            tool,
        }
    }
}

impl AnyAgentTool for ContextServerTool {
    fn name(&self) -> SharedString {
        self.tool.name.clone().into()
    }

    fn description(&self) -> SharedString {
        self.tool.description.clone().unwrap_or_default().into()
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Other
    }

    fn initial_title(&self, _input: serde_json::Value, _cx: &mut App) -> SharedString {
        format!("Run MCP tool `{}`", self.tool.name).into()
    }

    fn input_schema(
        &self,
        format: language_model::LanguageModelToolSchemaFormat,
    ) -> Result<serde_json::Value> {
        let mut schema = self.tool.input_schema.clone();
        crate::tool_schema::adapt_schema_to_format(&mut schema, format)?;
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

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<AgentToolOutput>> {
        let Some(server) = self.store.read(cx).get_running_server(&self.server_id) else {
            return Task::ready(Err(anyhow!("Context server not found")));
        };
        let tool_name = self.tool.name.clone();
        let authorize = event_stream.authorize(self.initial_title(input.clone(), cx), cx);

        cx.spawn(async move |_cx| {
            authorize.await?;

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
            let response = protocol
                .request::<context_server::types::requests::CallTool>(
                    context_server::types::CallToolParams {
                        name: tool_name,
                        arguments,
                        meta: None,
                    },
                )
                .await?;

            let mut result = String::new();
            for content in response.content {
                match content {
                    context_server::types::ToolResponseContent::Text { text } => {
                        result.push_str(&text);
                    }
                    context_server::types::ToolResponseContent::Image { .. } => {
                        log::warn!("Ignoring image content from tool response");
                    }
                    context_server::types::ToolResponseContent::Audio { .. } => {
                        log::warn!("Ignoring audio content from tool response");
                    }
                    context_server::types::ToolResponseContent::Resource { .. } => {
                        log::warn!("Ignoring resource content from tool response");
                    }
                }
            }
            Ok(AgentToolOutput {
                raw_output: result.clone().into(),
                llm_output: result.into(),
            })
        })
    }

    fn replay(
        &self,
        _input: serde_json::Value,
        _output: serde_json::Value,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Result<()> {
        Ok(())
    }
}
