use crate::{AgentToolOutput, AnyAgentTool, ToolCallEventStream};
use agent_client_protocol::ToolKind;
use anyhow::{Result, anyhow};
use collections::{BTreeMap, HashMap};
use context_server::{ContextServerId, client::NotificationSubscription};
use futures::FutureExt as _;
use gpui::{App, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task};
use project::context_server_store::{ContextServerStatus, ContextServerStore};
use std::sync::Arc;
use util::ResultExt;

pub struct ContextServerPrompt {
    pub server_id: ContextServerId,
    pub prompt: context_server::types::Prompt,
}

pub enum ContextServerRegistryEvent {
    ToolsChanged,
    PromptsChanged,
}

impl EventEmitter<ContextServerRegistryEvent> for ContextServerRegistry {}

pub struct ContextServerRegistry {
    server_store: Entity<ContextServerStore>,
    registered_servers: HashMap<ContextServerId, RegisteredContextServer>,
    _subscription: gpui::Subscription,
}

struct RegisteredContextServer {
    tools: BTreeMap<SharedString, Arc<dyn AnyAgentTool>>,
    prompts: BTreeMap<SharedString, ContextServerPrompt>,
    load_tools: Task<Result<()>>,
    load_prompts: Task<Result<()>>,
    _tools_updated_subscription: Option<NotificationSubscription>,
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
            this.reload_prompts_for_server(server.id(), cx);
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

    pub fn prompts(&self) -> impl Iterator<Item = &ContextServerPrompt> {
        self.registered_servers
            .values()
            .flat_map(|server| server.prompts.values())
    }

    pub fn find_prompt(
        &self,
        server_id: Option<&ContextServerId>,
        name: &str,
    ) -> Option<&ContextServerPrompt> {
        if let Some(server_id) = server_id {
            self.registered_servers
                .get(server_id)
                .and_then(|server| server.prompts.get(name))
        } else {
            self.registered_servers
                .values()
                .find_map(|server| server.prompts.get(name))
        }
    }

    pub fn server_store(&self) -> &Entity<ContextServerStore> {
        &self.server_store
    }

    fn get_or_register_server(
        &mut self,
        server_id: &ContextServerId,
        cx: &mut Context<Self>,
    ) -> &mut RegisteredContextServer {
        self.registered_servers
            .entry(server_id.clone())
            .or_insert_with(|| Self::init_registered_server(server_id, &self.server_store, cx))
    }

    fn init_registered_server(
        server_id: &ContextServerId,
        server_store: &Entity<ContextServerStore>,
        cx: &mut Context<Self>,
    ) -> RegisteredContextServer {
        let tools_updated_subscription = server_store
            .read(cx)
            .get_running_server(server_id)
            .and_then(|server| {
                let client = server.client()?;

                if !client.capable(context_server::protocol::ServerCapability::Tools) {
                    return None;
                }

                let server_id = server.id();
                let this = cx.entity().downgrade();

                Some(client.on_notification(
                    "notifications/tools/list_changed",
                    Box::new(move |_params, cx: AsyncApp| {
                        let server_id = server_id.clone();
                        let this = this.clone();
                        cx.spawn(async move |cx| {
                            this.update(cx, |this, cx| {
                                log::info!(
                                    "Received tools/list_changed notification for server {}",
                                    server_id
                                );
                                this.reload_tools_for_server(server_id, cx);
                            })
                        })
                        .detach();
                    }),
                ))
            });

        RegisteredContextServer {
            tools: BTreeMap::default(),
            prompts: BTreeMap::default(),
            load_tools: Task::ready(Ok(())),
            load_prompts: Task::ready(Ok(())),
            _tools_updated_subscription: tools_updated_subscription,
        }
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

        let registered_server = self.get_or_register_server(&server_id, cx);
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
                    cx.emit(ContextServerRegistryEvent::ToolsChanged);
                    cx.notify();
                }
            })
        });
    }

    fn reload_prompts_for_server(&mut self, server_id: ContextServerId, cx: &mut Context<Self>) {
        let Some(server) = self.server_store.read(cx).get_running_server(&server_id) else {
            return;
        };
        let Some(client) = server.client() else {
            return;
        };
        if !client.capable(context_server::protocol::ServerCapability::Prompts) {
            return;
        }

        let registered_server = self.get_or_register_server(&server_id, cx);

        registered_server.load_prompts = cx.spawn(async move |this, cx| {
            let response = client
                .request::<context_server::types::requests::PromptsList>(())
                .await;

            this.update(cx, |this, cx| {
                let Some(registered_server) = this.registered_servers.get_mut(&server_id) else {
                    return;
                };

                registered_server.prompts.clear();
                if let Some(response) = response.log_err() {
                    for prompt in response.prompts {
                        let name: SharedString = prompt.name.clone().into();
                        registered_server.prompts.insert(
                            name,
                            ContextServerPrompt {
                                server_id: server_id.clone(),
                                prompt,
                            },
                        );
                    }
                    cx.emit(ContextServerRegistryEvent::PromptsChanged);
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
                        self.reload_prompts_for_server(server_id.clone(), cx);
                    }
                    ContextServerStatus::Stopped | ContextServerStatus::Error(_) => {
                        if let Some(registered_server) = self.registered_servers.remove(server_id) {
                            if !registered_server.tools.is_empty() {
                                cx.emit(ContextServerRegistryEvent::ToolsChanged);
                            }
                            if !registered_server.prompts.is_empty() {
                                cx.emit(ContextServerRegistryEvent::PromptsChanged);
                            }
                        }
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
        language_model::tool_schema::adapt_schema_to_format(&mut schema, format)?;
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
                anyhow::bail!("Context server not initialized");
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

            let request = protocol.request::<context_server::types::requests::CallTool>(
                context_server::types::CallToolParams {
                    name: tool_name,
                    arguments,
                    meta: None,
                },
            );

            let response = futures::select! {
                response = request.fuse() => response?,
                _ = event_stream.cancelled_by_user().fuse() => {
                    anyhow::bail!("MCP tool cancelled by user");
                }
            };

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

pub fn get_prompt(
    server_store: &Entity<ContextServerStore>,
    server_id: &ContextServerId,
    prompt_name: &str,
    arguments: HashMap<String, String>,
    cx: &mut AsyncApp,
) -> Task<Result<context_server::types::PromptsGetResponse>> {
    let server = cx.update(|cx| server_store.read(cx).get_running_server(server_id));
    let Some(server) = server else {
        return Task::ready(Err(anyhow::anyhow!("Context server not found")));
    };

    let Some(protocol) = server.client() else {
        return Task::ready(Err(anyhow::anyhow!("Context server not initialized")));
    };

    let prompt_name = prompt_name.to_string();

    cx.background_spawn(async move {
        let response = protocol
            .request::<context_server::types::requests::PromptsGet>(
                context_server::types::PromptsGetParams {
                    name: prompt_name,
                    arguments: (!arguments.is_empty()).then(|| arguments),
                    meta: None,
                },
            )
            .await?;

        Ok(response)
    })
}
