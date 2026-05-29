use crate::{AgentToolOutput, AnyAgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema as acp;
use anyhow::Result;
use collections::{BTreeMap, HashMap};
use context_server::transport::TransportError;
use context_server::{ContextServerId, client::NotificationSubscription};
use futures::FutureExt as _;
use gpui::{App, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task};
use language_model::{LanguageModelImage, LanguageModelImageExt, LanguageModelToolResultContent};
use project::context_server_store::{ContextServerStatus, ContextServerStore};
use std::sync::Arc;
use util::ResultExt;

/// Generates a tool ID for an MCP tool that can be used in settings.
///
/// The format is `mcp:<server_id>:<tool_name>` to avoid collisions with built-in tools.
pub fn mcp_tool_id(server_id: &str, tool_name: &str) -> String {
    format!("mcp:{}:{}", server_id, tool_name)
}

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
        event: &project::context_server_store::ServerStatusChangedEvent,
        cx: &mut Context<Self>,
    ) {
        let project::context_server_store::ServerStatusChangedEvent { server_id, status } = event;

        match status {
            ContextServerStatus::Starting | ContextServerStatus::Authenticating => {}
            ContextServerStatus::Running => {
                self.reload_tools_for_server(server_id.clone(), cx);
                self.reload_prompts_for_server(server_id.clone(), cx);
            }
            ContextServerStatus::Stopped
            | ContextServerStatus::Error(_)
            | ContextServerStatus::AuthRequired
            | ContextServerStatus::ClientSecretRequired { .. }
            | ContextServerStatus::InsufficientScope {
                existing_scopes: _,
                required_scopes: _,
            } => {
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
        };
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

    fn kind(&self) -> acp::ToolKind {
        acp::ToolKind::Other
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
        input: ToolInput<serde_json::Value>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<AgentToolOutput, AgentToolOutput>> {
        let Some(server) = self.store.read(cx).get_running_server(&self.server_id) else {
            return Task::ready(Err(anyhow::anyhow!("Context server not found").into()));
        };
        let tool_name = self.tool.name.clone();
        let tool_id = mcp_tool_id(&self.server_id.0, &self.tool.name);
        let display_name = self.tool.name.clone();
        let initial_title = self.initial_title(serde_json::Value::Null, cx);
        let authorize =
            event_stream.authorize_third_party_tool(initial_title, tool_id, display_name, cx);

        cx.spawn(async move |mut async_cx| {
            let input = input
                .recv()
                .await
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;

            authorize
                .await
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;

            let mut protocol = server.client().ok_or_else(|| {
                anyhow::anyhow!("Context server not initialized")
            })?;

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

            let mut retry = true;

            let response = loop {
                let request = protocol.request::<context_server::types::requests::CallTool>(
                    context_server::types::CallToolParams {
                        name: tool_name.clone(),
                        arguments: arguments.clone(),
                        meta: None,
                    },
                );

                let result = futures::select! {
                    res = request.fuse() => res,
                    _ = event_stream.cancelled_by_user().fuse() => {
                        return Err(anyhow::anyhow!("MCP tool cancelled by user").into());
                    }
                };

                match result {
                    Ok(res) => break res,
                    Err(err) => {
                        if retry {
                            if let Some(TransportError::InsufficientScope { www_authenticate }) = err.downcast_ref::<TransportError>() {
                                log::info!("Tool execution blocked by 403 Insufficient Scope.");
                                retry = false;

                                let server_id = self.server_id.clone();
                                let configuration = async_cx.update(|cx| {
                                    self.store.read(cx).configuration_for_server(&server_id)
                                }).ok_or_else(|| anyhow::anyhow!("Failed to read context server config"))?;

                                let http_client = async_cx.update(|cx| cx.http_client());

                                let required_scopes = www_authenticate.scope.clone().unwrap_or_default();

                                let server_url = match configuration.as_ref() {
                                    project::context_server_store::ContextServerConfiguration::Http { url, .. } => Some(url.clone()),
                                    _ => None,
                                };

                                if let Some(server_url) = server_url {
                                    if let Ok(discovery) = context_server::oauth::discover(&http_client, &server_url, &www_authenticate).await {
                                        let existing_scopes = ContextServerStore::get_active_scopes(&server_url, &mut async_cx).await;

                                        let state_updated = async_cx.update(|cx| {
                                            self.store.update(cx, |store, cx| {
                                                    store.set_server_insufficient_scope(
                                                    &server_id,
                                                    discovery.clone(),
                                                    existing_scopes.clone(),
                                                    required_scopes.clone(),
                                                    cx,
                                                ).is_ok()
                                            })
                                        });

                                        if !state_updated {
                                            log::warn!("Could not transition server state to InsufficientScope");
                                        }

                                        let prompt_task = async_cx.update(|cx| {
                                            event_stream.prompt_oauth_upgrade(
                                                server_id.clone(),
                                                existing_scopes.clone(),
                                                required_scopes.clone(),
                                                cx
                                            )
                                        }).fuse();

                                        let cancel_signal = event_stream.cancelled_by_user().fuse();

                                        futures::pin_mut!(prompt_task, cancel_signal);

                                        let prompt_result = futures::select! {
                                            res = prompt_task => res,
                                            _ = cancel_signal => {
                                                log::info!("User cancelled tool execution during OAuth prompt.");
                                                let _ = async_cx.update(|cx| {
                                                    let _ = self.store.update(cx, |store, cx| {
                                                        store.set_server_running(&server_id, cx)
                                                    });
                                                });
                                                return Err(anyhow::anyhow!("MCP tool cancelled by user").into());
                                            }
                                        };

                                        if prompt_result.is_err() {
                                            log::info!("OAuth upgrade cancelled by user.");
                                            let _ = async_cx.update(|cx| {
                                                let _ = self.store.update(cx, |store, cx| {
                                                    store.set_server_running(&server_id, cx)
                                                });
                                            });
                                            return Err(anyhow::anyhow!("Tool execution blocked: User cancelled OAuth upgrade.").into());
                                        }

                                        log::info!("OAuth upgrade accepted. Waiting for server to restart...");

                                        let mut new_protocol = None;
                                        for _ in 0..50 {
                                            new_protocol = async_cx.update(|cx| {
                                                self.store.read(cx)
                                                    .get_running_server(&server_id)
                                                    .and_then(|s| s.client())
                                            });

                                            if new_protocol.is_some() {
                                                break;
                                            }
                                            let timer = async_cx.background_executor().timer(std::time::Duration::from_millis(100)).fuse();
                                            let cancel_signal = event_stream.cancelled_by_user().fuse();

                                            futures::pin_mut!(timer, cancel_signal);

                                            futures::select! {
                                                _ = timer => {},
                                                _ = cancel_signal => {
                                                    log::info!("User cancelled tool execution while waiting for server restart.");
                                                    return Err(anyhow::anyhow!("MCP tool cancelled by user").into());
                                                }
                                            }
                                        }

                                        if let Some(new_p) = new_protocol {
                                            protocol = new_p;
                                            continue;
                                        } else {
                                            return Err(anyhow::anyhow!("Server failed to restart in time after OAuth upgrade").into());
                                        }
                                    }
                                }
                            }
                        }
                        return Err(anyhow::anyhow!(err.to_string()).into());
                    }
                }
            };

            if response.is_error == Some(true) {
                let error_message: String =
                    response.content.iter().filter_map(|c| c.text()).collect();
                return Err(anyhow::anyhow!(error_message).into());
            }

            let mut llm_output = Vec::new();
            let mut tool_call_content = Vec::new();
            let mut concatenated_text = String::new();
            for content in response.content {
                match content {
                    context_server::types::ToolResponseContent::Text { text } => {
                        concatenated_text.push_str(&text);
                        tool_call_content.push(acp::ToolCallContent::Content(acp::Content::new(
                            acp::ContentBlock::Text(acp::TextContent::new(text.clone())),
                        )));
                        llm_output.push(LanguageModelToolResultContent::Text(text.into()));
                    }
                    context_server::types::ToolResponseContent::Image { data, mime_type } => {
                        tool_call_content.push(acp::ToolCallContent::Content(acp::Content::new(
                            acp::ContentBlock::Image(acp::ImageContent::new(
                                data.clone(),
                                mime_type.clone(),
                            )),
                        )));
                        let language_model_image = async_cx
                        .background_executor()
                        .spawn({
                            let mime_type = mime_type.clone();
                            async move {
                                LanguageModelImage::from_base64_image(&data, &mime_type)
                            }
                        })
                        .await;
                        match language_model_image {
                            Ok(Some(image)) => {
                                llm_output.push(LanguageModelToolResultContent::Image(image));
                            }
                            Ok(None) => {
                                log::warn!(
                                    "Skipping MCP tool response image with MIME type `{}` because it cannot be converted for language model input",
                                    mime_type
                                );
                            }
                            Err(error) => {
                                log::warn!(
                                    "Failed to convert MCP tool response image with MIME type `{}` for language model input: {:#}",
                                    mime_type,
                                    error
                                );
                            }
                        }
                    }
                    context_server::types::ToolResponseContent::Audio { .. } => {
                        log::warn!("Ignoring audio content from tool response");
                    }
                    context_server::types::ToolResponseContent::Resource { .. } => {
                        log::warn!("Ignoring resource content from tool response");
                    }
                    context_server::types::ToolResponseContent::ResourceLink { .. } => {
                        log::warn!("Ignoring resource link content from tool response");
                    }
                }
            }
            if !tool_call_content.is_empty() {
                event_stream
                    .update_fields(acp::ToolCallUpdateFields::new().content(tool_call_content));
            }
            let raw_output = serde_json::Value::String(concatenated_text);
            Ok(AgentToolOutput {
                raw_output,
                llm_output,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_tool_id_format() {
        assert_eq!(
            mcp_tool_id("filesystem", "read_file"),
            "mcp:filesystem:read_file"
        );
        assert_eq!(
            mcp_tool_id("github", "create_issue"),
            "mcp:github:create_issue"
        );
        assert_eq!(
            mcp_tool_id("my-custom-server", "do_something"),
            "mcp:my-custom-server:do_something"
        );
        // Underscores in names
        assert_eq!(mcp_tool_id("my_server", "my_tool"), "mcp:my_server:my_tool");
    }

    // Note: Tests for MCP tool ID collision with built-in tools and permission
    // decisions are in crates/agent/src/tool_permissions.rs to avoid duplication.
}
