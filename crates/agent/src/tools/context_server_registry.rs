use crate::{AgentToolOutput, AnyAgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol as acp;
use anyhow::Result;
use collections::{BTreeMap, HashMap};
use context_server::{ContextServerId, client::NotificationSubscription, types::Notification as _};
use futures::FutureExt as _;
use gpui::{App, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task};
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
            | ContextServerStatus::AuthRequired => {
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
            return Task::ready(Err(AgentToolOutput::from_error("Context server not found")));
        };
        let tool_name = self.tool.name.clone();
        let tool_id = mcp_tool_id(&self.server_id.0, &self.tool.name);
        let display_name = self.tool.name.clone();
        let initial_title = self.initial_title(serde_json::Value::Null, cx);
        let authorize =
            event_stream.authorize_third_party_tool(initial_title, tool_id, display_name, cx);

        cx.spawn(async move |cx| {
            let input = input.recv().await.map_err(|e| {
                AgentToolOutput::from_error(format!("Failed to receive tool input: {e}"))
            })?;

            authorize.await.map_err(|e| AgentToolOutput::from_error(e.to_string()))?;

            let Some(protocol) = server.client() else {
                return Err(AgentToolOutput::from_error("Context server not initialized"));
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

            let progress_token = context_server::types::ProgressToken::String(
                uuid::Uuid::new_v4().to_string(),
            );
            let meta = {
                let mut map = collections::HashMap::default();
                map.insert(
                    "progressToken".to_string(),
                    serde_json::to_value(&progress_token)
                        .unwrap_or(serde_json::Value::Null),
                );
                Some(map)
            };

            let mut params = context_server::types::CallToolParams {
                name: tool_name.clone(),
                arguments,
                meta,
                task: None,
            };

            // Subscribe to notifications/progress so the server can push live
            // status updates for this tool call. We use an mpsc channel so
            // that messages arriving between poll iterations are queued
            // instead of overwriting each other.
            let (progress_tx, mut progress_rx) =
                futures::channel::mpsc::unbounded::<String>();
            let _progress_subscription = {
                let progress_tx = progress_tx.clone();
                let expected_token =
                    serde_json::to_value(&progress_token).ok();
                protocol.on_notification(
                    context_server::types::notifications::Progress::METHOD,
                    Box::new(move |notification_value, _cx| {
                        if let Ok(progress) =
                            serde_json::from_value::<
                                context_server::types::ProgressParams,
                            >(notification_value)
                        {
                            let token_json =
                                serde_json::to_value(&progress.progress_token)
                                    .ok();
                            if token_json == expected_token {
                                if let Some(message) = progress.message {
                                    let _ = progress_tx.unbounded_send(message);
                                }
                            }
                        }
                    }),
                )
            };

            // Determine whether to use the task-augmented protocol.
            let server_supports_tasks = protocol
                .initialize
                .capabilities
                .tasks
                .as_ref()
                .and_then(|t| t.requests.as_ref())
                .and_then(|r| r.tools.as_ref())
                .and_then(|t| t.call.as_ref())
                .is_some();

            let tool_task_support = self
                .tool
                .execution
                .as_ref()
                .and_then(|e| e.task_support.as_ref());

            let use_task = server_supports_tasks
                && matches!(
                    tool_task_support,
                    Some(context_server::types::TaskSupport::Required)
                        | Some(context_server::types::TaskSupport::Optional)
                );

            let response = if use_task {
                params.task =
                    Some(context_server::types::TaskParams { ttl: Some(300_000) });

                let create_result = protocol
                    .request::<context_server::types::requests::CallToolAsTask>(params)
                    .await
                    .map_err(|e| AgentToolOutput::from_error(e.to_string()))?;

                if let Some(msg) = &create_result.task.status_message {
                    event_stream
                        .update_fields(acp::ToolCallUpdateFields::new().title(msg.clone()));
                }

                let task_id = create_result.task.task_id.clone();
                let poll_interval_ms =
                    create_result.task.poll_interval.unwrap_or(2000);

                // Model continuation: if the server provided a provisional
                // result via model-immediate-response, return it to the model
                // immediately and continue polling in the background.
                //
                // Per the spec (2025-11-25), the value "should be a string
                // intended to be passed as an immediate tool result to the
                // model." In practice servers may also send a structured
                // CallToolResponse object (with audience annotations, etc.).
                // We handle both forms.
                if let Some(provisional_response) = create_result
                    .meta
                    .as_ref()
                    .and_then(|m| {
                        m.get(context_server::types::MODEL_IMMEDIATE_RESPONSE_KEY)
                    })
                    .and_then(|v| {
                        // Try structured CallToolResponse first, then fall
                        // back to a plain string wrapped in a text content
                        // block.
                        if let Ok(response) = serde_json::from_value::<
                            context_server::types::CallToolResponse,
                        >(v.clone()) {
                            Some(response)
                        } else if let Some(text) = v.as_str() {
                            Some(context_server::types::CallToolResponse {
                                content: vec![
                                    context_server::types::ToolResponseContent::Text {
                                        text: text.to_string(),
                                        annotations: None,
                                    },
                                ],
                                is_error: None,
                                meta: None,
                                structured_content: None,
                            })
                        } else {
                            log::warn!(
                                "MCP tool {}: model-immediate-response is neither \
                                 a string nor a CallToolResponse: {:?}",
                                tool_name,
                                v,
                            );
                            None
                        }
                    })
                {
                    log::debug!(
                        "MCP tool {}: using model-immediate-response for task {}",
                        tool_name,
                        task_id,
                    );

                    let provisional_output = process_tool_response(
                        provisional_response,
                        &tool_name,
                        &event_stream,
                    );

                    match provisional_output {
                        Ok(mut output) => {
                            output.is_provisional = true;

                            // Subscribe to task status notifications for
                            // the background poller.
                            let status_subscription = {
                                let task_id_for_status = task_id.clone();
                                let status_tx = progress_tx.clone();
                                protocol.on_notification(
                                    context_server::types::notifications::TaskStatus::METHOD,
                                    Box::new(move |params, _cx| {
                                        if let Ok(task) =
                                            serde_json::from_value::<
                                                context_server::types::Task,
                                            >(params)
                                        {
                                            if task.task_id == task_id_for_status {
                                                if let Some(msg) =
                                                    task.status_message
                                                {
                                                    let _ = status_tx
                                                        .unbounded_send(msg);
                                                }
                                            }
                                        }
                                    }),
                                )
                            };

                            // Spawn a background task that continues polling
                            // the MCP task and forwards progress/status
                            // updates to the tool card.
                            let background_event_stream = event_stream.clone();
                            let background_tool_name = tool_name.clone();
                            cx.foreground_executor()
                                .spawn(mcp_task_background_poller(
                                    protocol,
                                    task_id,
                                    poll_interval_ms,
                                    background_event_stream,
                                    progress_rx,
                                    background_tool_name,
                                    _progress_subscription,
                                    status_subscription,
                                ))
                                .detach();

                            return Ok(output);
                        }
                        Err(error_output) => {
                            // Cancel the orphaned MCP task so it doesn't
                            // run until TTL expiry with nobody polling it.
                            let _ = protocol
                                .request::<context_server::types::requests::TasksCancel>(
                                    context_server::types::TasksCancelParams {
                                        task_id: task_id.clone(),
                                    },
                                )
                                .await;
                            return Err(error_output);
                        }
                    }
                }

                // No model-immediate-response: blocking poll loop.
                let _status_subscription = {
                    let task_id_for_status = task_id.clone();
                    let status_tx = progress_tx.clone();
                    protocol.on_notification(
                        context_server::types::notifications::TaskStatus::METHOD,
                        Box::new(move |params, _cx| {
                            if let Ok(task) =
                                serde_json::from_value::<context_server::types::Task>(params)
                            {
                                if task.task_id == task_id_for_status {
                                    if let Some(msg) = task.status_message {
                                        let _ = status_tx.unbounded_send(msg);
                                    }
                                }
                            }
                        }),
                    )
                };

                let mut blocking_poll_interval_ms = poll_interval_ms;
                loop {
                    let sleep_duration =
                        std::time::Duration::from_millis(blocking_poll_interval_ms);
                    futures::select! {
                        _ = smol::Timer::after(sleep_duration).fuse() => {},
                        _ = event_stream.cancelled_by_user().fuse() => {
                            let _ = protocol
                                .request::<context_server::types::requests::TasksCancel>(
                                    context_server::types::TasksCancelParams {
                                        task_id: task_id.clone(),
                                    },
                                )
                                .await;
                            return Err(AgentToolOutput::from_error(
                                "MCP task cancelled by user",
                            ));
                        }
                    }

                    while let Ok(msg) = progress_rx.try_recv() {
                        event_stream.update_fields(
                            acp::ToolCallUpdateFields::new().title(msg),
                        );
                    }

                    let task_state = protocol
                        .request::<context_server::types::requests::TasksGet>(
                            context_server::types::TasksGetParams {
                                task_id: task_id.clone(),
                            },
                        )
                        .await
                        .map_err(|e| {
                            AgentToolOutput::from_error(format!(
                                "Failed to poll task: {e}"
                            ))
                        })?;

                    if let Some(msg) = &task_state.status_message {
                        event_stream.update_fields(
                            acp::ToolCallUpdateFields::new().title(msg.clone()),
                        );
                    }

                    if let Some(interval) = task_state.poll_interval {
                        blocking_poll_interval_ms = interval;
                    }

                    match task_state.status {
                        context_server::types::TaskStatus::Completed
                        | context_server::types::TaskStatus::Failed
                        | context_server::types::TaskStatus::Cancelled => break,
                        context_server::types::TaskStatus::Working
                        | context_server::types::TaskStatus::InputRequired => continue,
                    }
                }

                let result_value = protocol
                    .request::<context_server::types::requests::TasksResult>(
                        context_server::types::TasksResultParams {
                            task_id: task_id.clone(),
                        },
                    )
                    .await
                    .map_err(|e| {
                        AgentToolOutput::from_error(format!(
                            "Failed to get task result: {e}"
                        ))
                    })?;

                serde_json::from_value::<context_server::types::CallToolResponse>(
                    result_value,
                )
                .map_err(|e| {
                    AgentToolOutput::from_error(format!(
                        "Failed to parse task result: {e}"
                    ))
                })?
            } else {
                let request =
                    protocol.request::<context_server::types::requests::CallTool>(params);

                futures::select! {
                    response = request.fuse() => {
                        response.map_err(|e| AgentToolOutput::from_error(e.to_string()))?
                    }
                    _ = event_stream.cancelled_by_user().fuse() => {
                        return Err(AgentToolOutput::from_error("MCP tool cancelled by user"));
                    }
                }
            };

            while let Ok(msg) = progress_rx.try_recv() {
                log::debug!(
                    "MCP tool {}: received progress message: {}",
                    tool_name,
                    msg
                );
            }

            process_tool_response(response, &tool_name, &event_stream)
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

/// Background poller for MCP tasks when model-immediate-response is used.
///
/// Continues polling the task for status and forwards progress/status
/// updates to the tool card. When the task reaches a terminal state,
/// fetches the final result and updates the tool card status.
///
/// This poller intentionally does NOT use `cancelled_by_user()` because it
/// outlives the turn that spawned it. The turn's cancellation sender is
/// dropped when the turn ends normally, and `cancelled_by_user()` would
/// spin in an infinite loop polling the dropped sender. Instead, the poller
/// simply runs until the MCP task reaches a terminal state or the protocol
/// connection fails.
///
/// The notification subscriptions are kept alive by moving them into this
/// function — they are dropped when the poller exits.
async fn mcp_task_background_poller(
    protocol: Arc<context_server::protocol::InitializedContextServerProtocol>,
    task_id: String,
    initial_poll_interval_ms: u64,
    event_stream: ToolCallEventStream,
    mut progress_rx: futures::channel::mpsc::UnboundedReceiver<String>,
    tool_name: String,
    _progress_subscription: NotificationSubscription,
    _status_subscription: NotificationSubscription,
) {
    let mut poll_interval_ms = initial_poll_interval_ms;

    loop {
        let sleep_duration = std::time::Duration::from_millis(poll_interval_ms);
        smol::Timer::after(sleep_duration).await;

        // Drain queued progress/status messages.
        while let Ok(msg) = progress_rx.try_recv() {
            event_stream
                .update_fields(acp::ToolCallUpdateFields::new().title(msg));
        }

        let task_state = match protocol
            .request::<context_server::types::requests::TasksGet>(
                context_server::types::TasksGetParams {
                    task_id: task_id.clone(),
                },
            )
            .await
        {
            Ok(state) => state,
            Err(e) => {
                log::error!(
                    "MCP tool {}: background poller failed to poll task: {e}",
                    tool_name,
                );
                event_stream.update_fields(
                    acp::ToolCallUpdateFields::new()
                        .status(acp::ToolCallStatus::Failed),
                );
                return;
            }
        };

        if let Some(msg) = &task_state.status_message {
            event_stream.update_fields(
                acp::ToolCallUpdateFields::new().title(msg.clone()),
            );
        }

        if let Some(interval) = task_state.poll_interval {
            poll_interval_ms = interval;
        }

        match task_state.status {
            context_server::types::TaskStatus::Completed => {
                // Fetch the final result to display in the tool card.
                match protocol
                    .request::<context_server::types::requests::TasksResult>(
                        context_server::types::TasksResultParams {
                            task_id: task_id.clone(),
                        },
                    )
                    .await
                {
                    Ok(result_value) => {
                        event_stream.update_fields(
                            acp::ToolCallUpdateFields::new()
                                .status(acp::ToolCallStatus::Completed)
                                .raw_output(Some(result_value)),
                        );
                    }
                    Err(e) => {
                        log::warn!(
                            "MCP tool {}: background poller failed to fetch final result: {e}",
                            tool_name,
                        );
                        event_stream.update_fields(
                            acp::ToolCallUpdateFields::new()
                                .status(acp::ToolCallStatus::Completed),
                        );
                    }
                }
                return;
            }
            context_server::types::TaskStatus::Failed
            | context_server::types::TaskStatus::Cancelled => {
                event_stream.update_fields(
                    acp::ToolCallUpdateFields::new()
                        .status(acp::ToolCallStatus::Failed),
                );
                return;
            }
            context_server::types::TaskStatus::Working
            | context_server::types::TaskStatus::InputRequired => continue,
        }
    }
}

fn process_tool_response(
    response: context_server::types::CallToolResponse,
    tool_name: &str,
    event_stream: &ToolCallEventStream,
) -> Result<AgentToolOutput, AgentToolOutput> {
    if response.is_error == Some(true) {
        let error_message: String = response.content.iter().filter_map(|c| c.text()).collect();
        return Err(AgentToolOutput::from_error(error_message));
    }

    // Partition content blocks by MCP audience annotation.
    //
    // MCP spec (2025-03-26) defines `annotations.audience` on tool
    // response content blocks as an array of Role ("user" / "assistant").
    // Blocks with audience: ["user"] are displayed to the human but
    // excluded from model context.  When all blocks are user-only the
    // model receives "[output displayed to user]" as a placeholder.
    //
    // Spec: https://modelcontextprotocol.io/specification/2025-03-26/server/tools
    // Tests: crates/agent/src/tests/test_mcp_audience.rs
    let (user_only, model_facing): (Vec<_>, Vec<_>) =
        response.content.into_iter().partition(|c| c.is_user_only());

    let user_only_count = user_only.len();

    let mut result = String::new();
    for content in model_facing {
        match content {
            context_server::types::ToolResponseContent::Text { text, .. } => {
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

    if !user_only.is_empty() {
        let user_content: Vec<acp::ToolCallContent> = user_only
            .into_iter()
            .filter_map(|c| {
                if let context_server::types::ToolResponseContent::Text { text, .. } = c {
                    Some(acp::ToolCallContent::Content(acp::Content::new(text)))
                } else {
                    None
                }
            })
            .collect();

        if !user_content.is_empty() {
            event_stream
                .update_fields(acp::ToolCallUpdateFields::new().content(user_content));
        }

        log::debug!(
            "MCP tool {}: audience filtering excluded {} user-only content block(s) from model context",
            tool_name,
            user_only_count
        );

        if result.is_empty() {
            result = "[output displayed to user]".to_string();
        }
    }

    Ok(AgentToolOutput {
        raw_output: result.clone().into(),
        llm_output: result.into(),
        is_provisional: false,
    })
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
