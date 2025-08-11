use crate::{SystemPromptTemplate, Template, Templates};
use acp_thread::Diff;
use action_log::ActionLog;
use agent_client_protocol as acp;
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::adapt_schema_to_format;
use cloud_llm_client::{CompletionIntent, CompletionMode};
use collections::HashMap;
use futures::{
    channel::{mpsc, oneshot},
    stream::FuturesUnordered,
};
use gpui::{App, Context, Entity, SharedString, Task};
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelRequest, LanguageModelRequestMessage, LanguageModelRequestTool,
    LanguageModelToolResult, LanguageModelToolResultContent, LanguageModelToolSchemaFormat,
    LanguageModelToolUse, LanguageModelToolUseId, MessageContent, Role, StopReason,
};
use log;
use project::Project;
use prompt_store::ProjectContext;
use schemars::{JsonSchema, Schema};
use serde::{Deserialize, Serialize};
use smol::stream::StreamExt;
use std::{cell::RefCell, collections::BTreeMap, fmt::Write, future::Future, rc::Rc, sync::Arc};
use util::{ResultExt, markdown::MarkdownCodeBlock};

#[derive(Debug, Clone)]
pub struct AgentMessage {
    pub role: Role,
    pub content: Vec<MessageContent>,
}

impl AgentMessage {
    pub fn to_markdown(&self) -> String {
        let mut markdown = format!("## {}\n", self.role);

        for content in &self.content {
            match content {
                MessageContent::Text(text) => {
                    markdown.push_str(text);
                    markdown.push('\n');
                }
                MessageContent::Thinking { text, .. } => {
                    markdown.push_str("<think>");
                    markdown.push_str(text);
                    markdown.push_str("</think>\n");
                }
                MessageContent::RedactedThinking(_) => markdown.push_str("<redacted_thinking />\n"),
                MessageContent::Image(_) => {
                    markdown.push_str("<image />\n");
                }
                MessageContent::ToolUse(tool_use) => {
                    markdown.push_str(&format!(
                        "**Tool Use**: {} (ID: {})\n",
                        tool_use.name, tool_use.id
                    ));
                    markdown.push_str(&format!(
                        "{}\n",
                        MarkdownCodeBlock {
                            tag: "json",
                            text: &format!("{:#}", tool_use.input)
                        }
                    ));
                }
                MessageContent::ToolResult(tool_result) => {
                    markdown.push_str(&format!(
                        "**Tool Result**: {} (ID: {})\n\n",
                        tool_result.tool_name, tool_result.tool_use_id
                    ));
                    if tool_result.is_error {
                        markdown.push_str("**ERROR:**\n");
                    }

                    match &tool_result.content {
                        LanguageModelToolResultContent::Text(text) => {
                            writeln!(markdown, "{text}\n").ok();
                        }
                        LanguageModelToolResultContent::Image(_) => {
                            writeln!(markdown, "<image />\n").ok();
                        }
                    }

                    if let Some(output) = tool_result.output.as_ref() {
                        writeln!(
                            markdown,
                            "**Debug Output**:\n\n```json\n{}\n```\n",
                            serde_json::to_string_pretty(output).unwrap()
                        )
                        .unwrap();
                    }
                }
            }
        }

        markdown
    }
}

#[derive(Debug)]
pub enum AgentResponseEvent {
    Text(String),
    Thinking(String),
    ToolCall(acp::ToolCall),
    ToolCallUpdate(acp_thread::ToolCallUpdate),
    ToolCallAuthorization(ToolCallAuthorization),
    Stop(acp::StopReason),
}

#[derive(Debug)]
pub struct ToolCallAuthorization {
    pub tool_call: acp::ToolCall,
    pub options: Vec<acp::PermissionOption>,
    pub response: oneshot::Sender<acp::PermissionOptionId>,
}

pub struct Thread {
    messages: Vec<AgentMessage>,
    completion_mode: CompletionMode,
    /// Holds the task that handles agent interaction until the end of the turn.
    /// Survives across multiple requests as the model performs tool calls and
    /// we run tools, report their results.
    running_turn: Option<Task<()>>,
    pending_tool_uses: HashMap<LanguageModelToolUseId, LanguageModelToolUse>,
    tools: BTreeMap<SharedString, Arc<dyn AnyAgentTool>>,
    project_context: Rc<RefCell<ProjectContext>>,
    templates: Arc<Templates>,
    pub selected_model: Arc<dyn LanguageModel>,
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
}

impl Thread {
    pub fn new(
        project: Entity<Project>,
        project_context: Rc<RefCell<ProjectContext>>,
        action_log: Entity<ActionLog>,
        templates: Arc<Templates>,
        default_model: Arc<dyn LanguageModel>,
    ) -> Self {
        Self {
            messages: Vec::new(),
            completion_mode: CompletionMode::Normal,
            running_turn: None,
            pending_tool_uses: HashMap::default(),
            tools: BTreeMap::default(),
            project_context,
            templates,
            selected_model: default_model,
            project,
            action_log,
        }
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    pub fn action_log(&self) -> &Entity<ActionLog> {
        &self.action_log
    }

    pub fn set_mode(&mut self, mode: CompletionMode) {
        self.completion_mode = mode;
    }

    pub fn messages(&self) -> &[AgentMessage] {
        &self.messages
    }

    pub fn add_tool(&mut self, tool: impl AgentTool) {
        self.tools.insert(tool.name(), tool.erase());
    }

    pub fn remove_tool(&mut self, name: &str) -> bool {
        self.tools.remove(name).is_some()
    }

    pub fn cancel(&mut self) {
        self.running_turn.take();

        let tool_results = self
            .pending_tool_uses
            .drain()
            .map(|(tool_use_id, tool_use)| {
                MessageContent::ToolResult(LanguageModelToolResult {
                    tool_use_id,
                    tool_name: tool_use.name.clone(),
                    is_error: true,
                    content: LanguageModelToolResultContent::Text("Tool canceled by user".into()),
                    output: None,
                })
            })
            .collect::<Vec<_>>();
        self.last_user_message().content.extend(tool_results);
    }

    /// Sending a message results in the model streaming a response, which could include tool calls.
    /// After calling tools, the model will stops and waits for any outstanding tool calls to be completed and their results sent.
    /// The returned channel will report all the occurrences in which the model stops before erroring or ending its turn.
    pub fn send(
        &mut self,
        content: impl Into<MessageContent>,
        cx: &mut Context<Self>,
    ) -> mpsc::UnboundedReceiver<Result<AgentResponseEvent, LanguageModelCompletionError>> {
        let content = content.into();
        let model = self.selected_model.clone();
        log::info!("Thread::send called with model: {:?}", model.name());
        log::debug!("Thread::send content: {:?}", content);

        cx.notify();
        let (events_tx, events_rx) =
            mpsc::unbounded::<Result<AgentResponseEvent, LanguageModelCompletionError>>();
        let event_stream = AgentResponseEventStream(events_tx);

        let user_message_ix = self.messages.len();
        self.messages.push(AgentMessage {
            role: Role::User,
            content: vec![content],
        });
        log::info!("Total messages in thread: {}", self.messages.len());
        self.running_turn = Some(cx.spawn(async move |thread, cx| {
            log::info!("Starting agent turn execution");
            let turn_result = async {
                // Perform one request, then keep looping if the model makes tool calls.
                let mut completion_intent = CompletionIntent::UserPrompt;
                'outer: loop {
                    log::debug!(
                        "Building completion request with intent: {:?}",
                        completion_intent
                    );
                    let request = thread.update(cx, |thread, cx| {
                        thread.build_completion_request(completion_intent, cx)
                    })?;

                    // println!(
                    //     "request: {}",
                    //     serde_json::to_string_pretty(&request).unwrap()
                    // );

                    // Stream events, appending to messages and collecting up tool uses.
                    log::info!("Calling model.stream_completion");
                    let mut events = model.stream_completion(request, cx).await?;
                    log::debug!("Stream completion started successfully");
                    let mut tool_uses = FuturesUnordered::new();
                    while let Some(event) = events.next().await {
                        match event {
                            Ok(LanguageModelCompletionEvent::Stop(reason)) => {
                                event_stream.send_stop(reason);
                                if reason == StopReason::Refusal {
                                    thread.update(cx, |thread, _cx| {
                                        thread.messages.truncate(user_message_ix);
                                    })?;
                                    break 'outer;
                                }
                            }
                            Ok(event) => {
                                log::trace!("Received completion event: {:?}", event);
                                thread
                                    .update(cx, |thread, cx| {
                                        tool_uses.extend(thread.handle_streamed_completion_event(
                                            event,
                                            &event_stream,
                                            cx,
                                        ));
                                    })
                                    .ok();
                            }
                            Err(error) => {
                                log::error!("Error in completion stream: {:?}", error);
                                event_stream.send_error(error);
                                break;
                            }
                        }
                    }

                    // If there are no tool uses, the turn is done.
                    if tool_uses.is_empty() {
                        log::info!("No tool uses found, completing turn");
                        break;
                    }
                    log::info!("Found {} tool uses to execute", tool_uses.len());

                    // As tool results trickle in, insert them in the last user
                    // message so that they can be sent on the next tick of the
                    // agentic loop.
                    while let Some(tool_result) = tool_uses.next().await {
                        log::info!("Tool finished {:?}", tool_result);

                        event_stream.update_tool_call_fields(
                            &tool_result.tool_use_id,
                            acp::ToolCallUpdateFields {
                                status: Some(if tool_result.is_error {
                                    acp::ToolCallStatus::Failed
                                } else {
                                    acp::ToolCallStatus::Completed
                                }),
                                ..Default::default()
                            },
                        );
                        thread
                            .update(cx, |thread, _cx| {
                                thread.pending_tool_uses.remove(&tool_result.tool_use_id);
                                thread
                                    .last_user_message()
                                    .content
                                    .push(MessageContent::ToolResult(tool_result));
                            })
                            .ok();
                    }

                    completion_intent = CompletionIntent::ToolResults;
                }

                Ok(())
            }
            .await;

            if let Err(error) = turn_result {
                log::error!("Turn execution failed: {:?}", error);
                event_stream.send_error(error);
            } else {
                log::info!("Turn execution completed successfully");
            }
        }));
        events_rx
    }

    pub fn build_system_message(&self) -> AgentMessage {
        log::debug!("Building system message");
        let prompt = SystemPromptTemplate {
            project: &self.project_context.borrow(),
            available_tools: self.tools.keys().cloned().collect(),
        }
        .render(&self.templates)
        .context("failed to build system prompt")
        .expect("Invalid template");
        log::debug!("System message built");
        AgentMessage {
            role: Role::System,
            content: vec![prompt.into()],
        }
    }

    /// A helper method that's called on every streamed completion event.
    /// Returns an optional tool result task, which the main agentic loop in
    /// send will send back to the model when it resolves.
    fn handle_streamed_completion_event(
        &mut self,
        event: LanguageModelCompletionEvent,
        event_stream: &AgentResponseEventStream,
        cx: &mut Context<Self>,
    ) -> Option<Task<LanguageModelToolResult>> {
        log::trace!("Handling streamed completion event: {:?}", event);
        use LanguageModelCompletionEvent::*;

        match event {
            StartMessage { .. } => {
                self.messages.push(AgentMessage {
                    role: Role::Assistant,
                    content: Vec::new(),
                });
            }
            Text(new_text) => self.handle_text_event(new_text, event_stream, cx),
            Thinking { text, signature } => {
                self.handle_thinking_event(text, signature, event_stream, cx)
            }
            RedactedThinking { data } => self.handle_redacted_thinking_event(data, cx),
            ToolUse(tool_use) => {
                return self.handle_tool_use_event(tool_use, event_stream, cx);
            }
            ToolUseJsonParseError {
                id,
                tool_name,
                raw_input,
                json_parse_error,
            } => {
                return Some(Task::ready(self.handle_tool_use_json_parse_error_event(
                    id,
                    tool_name,
                    raw_input,
                    json_parse_error,
                )));
            }
            UsageUpdate(_) | StatusUpdate(_) => {}
            Stop(_) => unreachable!(),
        }

        None
    }

    fn handle_text_event(
        &mut self,
        new_text: String,
        events_stream: &AgentResponseEventStream,
        cx: &mut Context<Self>,
    ) {
        events_stream.send_text(&new_text);

        let last_message = self.last_assistant_message();
        if let Some(MessageContent::Text(text)) = last_message.content.last_mut() {
            text.push_str(&new_text);
        } else {
            last_message.content.push(MessageContent::Text(new_text));
        }

        cx.notify();
    }

    fn handle_thinking_event(
        &mut self,
        new_text: String,
        new_signature: Option<String>,
        event_stream: &AgentResponseEventStream,
        cx: &mut Context<Self>,
    ) {
        event_stream.send_thinking(&new_text);

        let last_message = self.last_assistant_message();
        if let Some(MessageContent::Thinking { text, signature }) = last_message.content.last_mut()
        {
            text.push_str(&new_text);
            *signature = new_signature.or(signature.take());
        } else {
            last_message.content.push(MessageContent::Thinking {
                text: new_text,
                signature: new_signature,
            });
        }

        cx.notify();
    }

    fn handle_redacted_thinking_event(&mut self, data: String, cx: &mut Context<Self>) {
        let last_message = self.last_assistant_message();
        last_message
            .content
            .push(MessageContent::RedactedThinking(data));
        cx.notify();
    }

    fn handle_tool_use_event(
        &mut self,
        tool_use: LanguageModelToolUse,
        event_stream: &AgentResponseEventStream,
        cx: &mut Context<Self>,
    ) -> Option<Task<LanguageModelToolResult>> {
        cx.notify();

        let tool = self.tools.get(tool_use.name.as_ref()).cloned();

        self.pending_tool_uses
            .insert(tool_use.id.clone(), tool_use.clone());
        let last_message = self.last_assistant_message();

        // Ensure the last message ends in the current tool use
        let push_new_tool_use = last_message.content.last_mut().map_or(true, |content| {
            if let MessageContent::ToolUse(last_tool_use) = content {
                if last_tool_use.id == tool_use.id {
                    *last_tool_use = tool_use.clone();
                    false
                } else {
                    true
                }
            } else {
                true
            }
        });

        let mut title = SharedString::from(&tool_use.name);
        let mut kind = acp::ToolKind::Other;
        if let Some(tool) = tool.as_ref() {
            title = tool.initial_title(tool_use.input.clone());
            kind = tool.kind();
        }

        if push_new_tool_use {
            event_stream.send_tool_call(&tool_use.id, title, kind, tool_use.input.clone());
            last_message
                .content
                .push(MessageContent::ToolUse(tool_use.clone()));
        } else {
            event_stream.update_tool_call_fields(
                &tool_use.id,
                acp::ToolCallUpdateFields {
                    title: Some(title.into()),
                    kind: Some(kind),
                    raw_input: Some(tool_use.input.clone()),
                    ..Default::default()
                },
            );
        }

        if !tool_use.is_input_complete {
            return None;
        }

        let Some(tool) = tool else {
            let content = format!("No tool named {} exists", tool_use.name);
            return Some(Task::ready(LanguageModelToolResult {
                content: LanguageModelToolResultContent::Text(Arc::from(content)),
                tool_use_id: tool_use.id,
                tool_name: tool_use.name,
                is_error: true,
                output: None,
            }));
        };

        let tool_event_stream =
            ToolCallEventStream::new(&tool_use, tool.kind(), event_stream.clone());
        tool_event_stream.update_fields(acp::ToolCallUpdateFields {
            status: Some(acp::ToolCallStatus::InProgress),
            ..Default::default()
        });
        let supports_images = self.selected_model.supports_images();
        let tool_result = tool.run(tool_use.input, tool_event_stream, cx);
        Some(cx.foreground_executor().spawn(async move {
            let tool_result = tool_result.await.and_then(|output| {
                if let LanguageModelToolResultContent::Image(_) = &output.llm_output {
                    if !supports_images {
                        return Err(anyhow!(
                            "Attempted to read an image, but this model doesn't support it.",
                        ));
                    }
                }
                Ok(output)
            });

            match tool_result {
                Ok(output) => LanguageModelToolResult {
                    tool_use_id: tool_use.id,
                    tool_name: tool_use.name,
                    is_error: false,
                    content: output.llm_output,
                    output: Some(output.raw_output),
                },
                Err(error) => LanguageModelToolResult {
                    tool_use_id: tool_use.id,
                    tool_name: tool_use.name,
                    is_error: true,
                    content: LanguageModelToolResultContent::Text(Arc::from(error.to_string())),
                    output: None,
                },
            }
        }))
    }

    fn handle_tool_use_json_parse_error_event(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        raw_input: Arc<str>,
        json_parse_error: String,
    ) -> LanguageModelToolResult {
        let tool_output = format!("Error parsing input JSON: {json_parse_error}");
        LanguageModelToolResult {
            tool_use_id,
            tool_name,
            is_error: true,
            content: LanguageModelToolResultContent::Text(tool_output.into()),
            output: Some(serde_json::Value::String(raw_input.to_string())),
        }
    }

    /// Guarantees the last message is from the assistant and returns a mutable reference.
    fn last_assistant_message(&mut self) -> &mut AgentMessage {
        if self
            .messages
            .last()
            .map_or(true, |m| m.role != Role::Assistant)
        {
            self.messages.push(AgentMessage {
                role: Role::Assistant,
                content: Vec::new(),
            });
        }
        self.messages.last_mut().unwrap()
    }

    /// Guarantees the last message is from the user and returns a mutable reference.
    fn last_user_message(&mut self) -> &mut AgentMessage {
        if self.messages.last().map_or(true, |m| m.role != Role::User) {
            self.messages.push(AgentMessage {
                role: Role::User,
                content: Vec::new(),
            });
        }
        self.messages.last_mut().unwrap()
    }

    pub(crate) fn build_completion_request(
        &self,
        completion_intent: CompletionIntent,
        cx: &mut App,
    ) -> LanguageModelRequest {
        log::debug!("Building completion request");
        log::debug!("Completion intent: {:?}", completion_intent);
        log::debug!("Completion mode: {:?}", self.completion_mode);

        let messages = self.build_request_messages();
        log::info!("Request will include {} messages", messages.len());

        let tools: Vec<LanguageModelRequestTool> = self
            .tools
            .values()
            .filter_map(|tool| {
                let tool_name = tool.name().to_string();
                log::trace!("Including tool: {}", tool_name);
                Some(LanguageModelRequestTool {
                    name: tool_name,
                    description: tool.description(cx).to_string(),
                    input_schema: tool
                        .input_schema(self.selected_model.tool_input_format())
                        .log_err()?,
                })
            })
            .collect();

        log::info!("Request includes {} tools", tools.len());

        let request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: Some(completion_intent),
            mode: Some(self.completion_mode),
            messages,
            tools,
            tool_choice: None,
            stop: Vec::new(),
            temperature: None,
            thinking_allowed: true,
        };

        log::debug!("Completion request built successfully");
        request
    }

    fn build_request_messages(&self) -> Vec<LanguageModelRequestMessage> {
        log::trace!(
            "Building request messages from {} thread messages",
            self.messages.len()
        );

        let messages = Some(self.build_system_message())
            .iter()
            .chain(self.messages.iter())
            .map(|message| {
                log::trace!(
                    "  - {} message with {} content items",
                    match message.role {
                        Role::System => "System",
                        Role::User => "User",
                        Role::Assistant => "Assistant",
                    },
                    message.content.len()
                );
                LanguageModelRequestMessage {
                    role: message.role,
                    content: message.content.clone(),
                    cache: false,
                }
            })
            .collect();
        messages
    }

    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        for message in &self.messages {
            markdown.push_str(&message.to_markdown());
        }
        markdown
    }
}

pub trait AgentTool
where
    Self: 'static + Sized,
{
    type Input: for<'de> Deserialize<'de> + Serialize + JsonSchema;
    type Output: for<'de> Deserialize<'de> + Serialize + Into<LanguageModelToolResultContent>;

    fn name(&self) -> SharedString;

    fn description(&self, _cx: &mut App) -> SharedString {
        let schema = schemars::schema_for!(Self::Input);
        SharedString::new(
            schema
                .get("description")
                .and_then(|description| description.as_str())
                .unwrap_or_default(),
        )
    }

    fn kind(&self) -> acp::ToolKind;

    /// The initial tool title to display. Can be updated during the tool run.
    fn initial_title(&self, input: Result<Self::Input, serde_json::Value>) -> SharedString;

    /// Returns the JSON schema that describes the tool's input.
    fn input_schema(&self) -> Schema {
        schemars::schema_for!(Self::Input)
    }

    /// Runs the tool with the provided input.
    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>>;

    fn erase(self) -> Arc<dyn AnyAgentTool> {
        Arc::new(Erased(Arc::new(self)))
    }
}

pub struct Erased<T>(T);

pub struct AgentToolOutput {
    llm_output: LanguageModelToolResultContent,
    raw_output: serde_json::Value,
}

pub trait AnyAgentTool {
    fn name(&self) -> SharedString;
    fn description(&self, cx: &mut App) -> SharedString;
    fn kind(&self) -> acp::ToolKind;
    fn initial_title(&self, input: serde_json::Value) -> SharedString;
    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value>;
    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<AgentToolOutput>>;
}

impl<T> AnyAgentTool for Erased<Arc<T>>
where
    T: AgentTool,
{
    fn name(&self) -> SharedString {
        self.0.name()
    }

    fn description(&self, cx: &mut App) -> SharedString {
        self.0.description(cx)
    }

    fn kind(&self) -> agent_client_protocol::ToolKind {
        self.0.kind()
    }

    fn initial_title(&self, input: serde_json::Value) -> SharedString {
        let parsed_input = serde_json::from_value(input.clone()).map_err(|_| input);
        self.0.initial_title(parsed_input)
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        let mut json = serde_json::to_value(self.0.input_schema())?;
        adapt_schema_to_format(&mut json, format)?;
        Ok(json)
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<AgentToolOutput>> {
        cx.spawn(async move |cx| {
            let input = serde_json::from_value(input)?;
            let output = cx
                .update(|cx| self.0.clone().run(input, event_stream, cx))?
                .await?;
            let raw_output = serde_json::to_value(&output)?;
            Ok(AgentToolOutput {
                llm_output: output.into(),
                raw_output,
            })
        })
    }
}

#[derive(Clone)]
struct AgentResponseEventStream(
    mpsc::UnboundedSender<Result<AgentResponseEvent, LanguageModelCompletionError>>,
);

impl AgentResponseEventStream {
    fn send_text(&self, text: &str) {
        self.0
            .unbounded_send(Ok(AgentResponseEvent::Text(text.to_string())))
            .ok();
    }

    fn send_thinking(&self, text: &str) {
        self.0
            .unbounded_send(Ok(AgentResponseEvent::Thinking(text.to_string())))
            .ok();
    }

    fn authorize_tool_call(
        &self,
        id: &LanguageModelToolUseId,
        title: String,
        kind: acp::ToolKind,
        input: serde_json::Value,
    ) -> impl use<> + Future<Output = Result<()>> {
        let (response_tx, response_rx) = oneshot::channel();
        self.0
            .unbounded_send(Ok(AgentResponseEvent::ToolCallAuthorization(
                ToolCallAuthorization {
                    tool_call: Self::initial_tool_call(id, title, kind, input),
                    options: vec![
                        acp::PermissionOption {
                            id: acp::PermissionOptionId("always_allow".into()),
                            name: "Always Allow".into(),
                            kind: acp::PermissionOptionKind::AllowAlways,
                        },
                        acp::PermissionOption {
                            id: acp::PermissionOptionId("allow".into()),
                            name: "Allow".into(),
                            kind: acp::PermissionOptionKind::AllowOnce,
                        },
                        acp::PermissionOption {
                            id: acp::PermissionOptionId("deny".into()),
                            name: "Deny".into(),
                            kind: acp::PermissionOptionKind::RejectOnce,
                        },
                    ],
                    response: response_tx,
                },
            )))
            .ok();
        async move {
            match response_rx.await?.0.as_ref() {
                "allow" | "always_allow" => Ok(()),
                _ => Err(anyhow!("Permission to run tool denied by user")),
            }
        }
    }

    fn send_tool_call(
        &self,
        id: &LanguageModelToolUseId,
        title: SharedString,
        kind: acp::ToolKind,
        input: serde_json::Value,
    ) {
        self.0
            .unbounded_send(Ok(AgentResponseEvent::ToolCall(Self::initial_tool_call(
                id,
                title.to_string(),
                kind,
                input,
            ))))
            .ok();
    }

    fn initial_tool_call(
        id: &LanguageModelToolUseId,
        title: String,
        kind: acp::ToolKind,
        input: serde_json::Value,
    ) -> acp::ToolCall {
        acp::ToolCall {
            id: acp::ToolCallId(id.to_string().into()),
            title,
            kind,
            status: acp::ToolCallStatus::Pending,
            content: vec![],
            locations: vec![],
            raw_input: Some(input),
            raw_output: None,
        }
    }

    fn update_tool_call_fields(
        &self,
        tool_use_id: &LanguageModelToolUseId,
        fields: acp::ToolCallUpdateFields,
    ) {
        self.0
            .unbounded_send(Ok(AgentResponseEvent::ToolCallUpdate(
                acp::ToolCallUpdate {
                    id: acp::ToolCallId(tool_use_id.to_string().into()),
                    fields,
                }
                .into(),
            )))
            .ok();
    }

    fn update_tool_call_diff(&self, tool_use_id: &LanguageModelToolUseId, diff: Entity<Diff>) {
        self.0
            .unbounded_send(Ok(AgentResponseEvent::ToolCallUpdate(
                acp_thread::ToolCallUpdateDiff {
                    id: acp::ToolCallId(tool_use_id.to_string().into()),
                    diff,
                }
                .into(),
            )))
            .ok();
    }

    fn send_stop(&self, reason: StopReason) {
        match reason {
            StopReason::EndTurn => {
                self.0
                    .unbounded_send(Ok(AgentResponseEvent::Stop(acp::StopReason::EndTurn)))
                    .ok();
            }
            StopReason::MaxTokens => {
                self.0
                    .unbounded_send(Ok(AgentResponseEvent::Stop(acp::StopReason::MaxTokens)))
                    .ok();
            }
            StopReason::Refusal => {
                self.0
                    .unbounded_send(Ok(AgentResponseEvent::Stop(acp::StopReason::Refusal)))
                    .ok();
            }
            StopReason::ToolUse => {}
        }
    }

    fn send_error(&self, error: LanguageModelCompletionError) {
        self.0.unbounded_send(Err(error)).ok();
    }
}

#[derive(Clone)]
pub struct ToolCallEventStream {
    tool_use_id: LanguageModelToolUseId,
    kind: acp::ToolKind,
    input: serde_json::Value,
    stream: AgentResponseEventStream,
}

impl ToolCallEventStream {
    #[cfg(test)]
    pub fn test() -> (Self, ToolCallEventStreamReceiver) {
        let (events_tx, events_rx) =
            mpsc::unbounded::<Result<AgentResponseEvent, LanguageModelCompletionError>>();

        let stream = ToolCallEventStream::new(
            &LanguageModelToolUse {
                id: "test_id".into(),
                name: "test_tool".into(),
                raw_input: String::new(),
                input: serde_json::Value::Null,
                is_input_complete: true,
            },
            acp::ToolKind::Other,
            AgentResponseEventStream(events_tx),
        );

        (stream, ToolCallEventStreamReceiver(events_rx))
    }

    fn new(
        tool_use: &LanguageModelToolUse,
        kind: acp::ToolKind,
        stream: AgentResponseEventStream,
    ) -> Self {
        Self {
            tool_use_id: tool_use.id.clone(),
            kind,
            input: tool_use.input.clone(),
            stream,
        }
    }

    pub fn update_fields(&self, fields: acp::ToolCallUpdateFields) {
        self.stream
            .update_tool_call_fields(&self.tool_use_id, fields);
    }

    pub fn update_diff(&self, diff: Entity<Diff>) {
        self.stream.update_tool_call_diff(&self.tool_use_id, diff);
    }

    pub fn authorize(&self, title: String) -> impl use<> + Future<Output = Result<()>> {
        self.stream.authorize_tool_call(
            &self.tool_use_id,
            title,
            self.kind.clone(),
            self.input.clone(),
        )
    }
}

#[cfg(test)]
pub struct ToolCallEventStreamReceiver(
    mpsc::UnboundedReceiver<Result<AgentResponseEvent, LanguageModelCompletionError>>,
);

#[cfg(test)]
impl ToolCallEventStreamReceiver {
    pub async fn expect_tool_authorization(&mut self) -> ToolCallAuthorization {
        let event = self.0.next().await;
        if let Some(Ok(AgentResponseEvent::ToolCallAuthorization(auth))) = event {
            auth
        } else {
            panic!("Expected ToolCallAuthorization but got: {:?}", event);
        }
    }
}

#[cfg(test)]
impl std::ops::Deref for ToolCallEventStreamReceiver {
    type Target = mpsc::UnboundedReceiver<Result<AgentResponseEvent, LanguageModelCompletionError>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
impl std::ops::DerefMut for ToolCallEventStreamReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
