use crate::{prompts::BasePrompt, templates::Templates};
use agent_client_protocol as acp;
use anyhow::{anyhow, Result};
use cloud_llm_client::{CompletionIntent, CompletionMode};
use collections::HashMap;
use futures::{channel::mpsc, stream::FuturesUnordered};
use gpui::{App, Context, Entity, ImageFormat, SharedString, Task};
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelImage,
    LanguageModelRequest, LanguageModelRequestMessage, LanguageModelRequestTool,
    LanguageModelToolResult, LanguageModelToolResultContent, LanguageModelToolSchemaFormat,
    LanguageModelToolUse, LanguageModelToolUseId, MessageContent, Role, StopReason,
};
use log;
use project::Project;
use schemars::{JsonSchema, Schema};
use serde::Deserialize;
use smol::stream::StreamExt;
use std::{collections::BTreeMap, fmt::Write, sync::Arc};
use util::{markdown::MarkdownCodeBlock, ResultExt};

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
    ToolCallUpdate(acp::ToolCallUpdate),
    Stop(acp::StopReason),
}

pub trait Prompt {
    fn render(&self, prompts: &Templates, cx: &App) -> Result<String>;
}

pub struct Thread {
    messages: Vec<AgentMessage>,
    completion_mode: CompletionMode,
    /// Holds the task that handles agent interaction until the end of the turn.
    /// Survives across multiple requests as the model performs tool calls and
    /// we run tools, report their results.
    running_turn: Option<Task<()>>,
    pending_tool_uses: HashMap<LanguageModelToolUseId, LanguageModelToolUse>,
    system_prompts: Vec<Arc<dyn Prompt>>,
    tools: BTreeMap<SharedString, Arc<dyn AnyAgentTool>>,
    templates: Arc<Templates>,
    pub selected_model: Arc<dyn LanguageModel>,
    // action_log: Entity<ActionLog>,
}

impl Thread {
    pub fn new(
        project: Entity<Project>,
        templates: Arc<Templates>,
        default_model: Arc<dyn LanguageModel>,
    ) -> Self {
        Self {
            messages: Vec::new(),
            completion_mode: CompletionMode::Normal,
            system_prompts: vec![Arc::new(BasePrompt::new(project))],
            running_turn: None,
            pending_tool_uses: HashMap::default(),
            tools: BTreeMap::default(),
            templates,
            selected_model: default_model,
        }
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
        model: Arc<dyn LanguageModel>,
        content: impl Into<MessageContent>,
        cx: &mut Context<Self>,
    ) -> mpsc::UnboundedReceiver<Result<AgentResponseEvent, LanguageModelCompletionError>> {
        let content = content.into();
        log::info!("Thread::send called with model: {:?}", model.name());
        log::debug!("Thread::send content: {:?}", content);

        cx.notify();
        let (events_tx, events_rx) =
            mpsc::unbounded::<Result<AgentResponseEvent, LanguageModelCompletionError>>();

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
                                if let Some(reason) = to_acp_stop_reason(reason) {
                                    events_tx
                                        .unbounded_send(Ok(AgentResponseEvent::Stop(reason)))
                                        .ok();
                                }

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
                                            event, &events_tx, cx,
                                        ));
                                    })
                                    .ok();
                            }
                            Err(error) => {
                                log::error!("Error in completion stream: {:?}", error);
                                events_tx.unbounded_send(Err(error)).ok();
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

                        events_tx
                            .unbounded_send(Ok(AgentResponseEvent::ToolCallUpdate(
                                to_acp_tool_call_update(&tool_result),
                            )))
                            .ok();
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
                events_tx.unbounded_send(Err(error)).ok();
            } else {
                log::info!("Turn execution completed successfully");
            }
        }));
        events_rx
    }

    pub fn build_system_message(&self, cx: &App) -> Option<AgentMessage> {
        log::debug!("Building system message");
        let mut system_message = AgentMessage {
            role: Role::System,
            content: Vec::new(),
        };

        for prompt in &self.system_prompts {
            if let Some(rendered_prompt) = prompt.render(&self.templates, cx).log_err() {
                system_message
                    .content
                    .push(MessageContent::Text(rendered_prompt));
            }
        }

        let result = (!system_message.content.is_empty()).then_some(system_message);
        log::debug!("System message built: {}", result.is_some());
        result
    }

    /// A helper method that's called on every streamed completion event.
    /// Returns an optional tool result task, which the main agentic loop in
    /// send will send back to the model when it resolves.
    fn handle_streamed_completion_event(
        &mut self,
        event: LanguageModelCompletionEvent,
        events_tx: &mpsc::UnboundedSender<Result<AgentResponseEvent, LanguageModelCompletionError>>,
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
            Text(new_text) => self.handle_text_event(new_text, events_tx, cx),
            Thinking { text, signature } => {
                self.handle_thinking_event(text, signature, events_tx, cx)
            }
            RedactedThinking { data } => self.handle_redacted_thinking_event(data, cx),
            ToolUse(tool_use) => {
                return self.handle_tool_use_event(tool_use, events_tx, cx);
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
        events_tx: &mpsc::UnboundedSender<Result<AgentResponseEvent, LanguageModelCompletionError>>,
        cx: &mut Context<Self>,
    ) {
        events_tx
            .unbounded_send(Ok(AgentResponseEvent::Text(new_text.clone())))
            .ok();

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
        events_tx: &mpsc::UnboundedSender<Result<AgentResponseEvent, LanguageModelCompletionError>>,
        cx: &mut Context<Self>,
    ) {
        events_tx
            .unbounded_send(Ok(AgentResponseEvent::Thinking(new_text.clone())))
            .ok();

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
        events_tx: &mpsc::UnboundedSender<Result<AgentResponseEvent, LanguageModelCompletionError>>,
        cx: &mut Context<Self>,
    ) -> Option<Task<LanguageModelToolResult>> {
        cx.notify();

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
        if push_new_tool_use {
            events_tx
                .unbounded_send(Ok(AgentResponseEvent::ToolCall(acp::ToolCall {
                    id: acp::ToolCallId(tool_use.id.to_string().into()),
                    title: tool_use.name.to_string(),
                    kind: acp::ToolKind::Other,
                    status: acp::ToolCallStatus::Pending,
                    content: vec![],
                    locations: vec![],
                    raw_input: Some(tool_use.input.clone()),
                })))
                .ok();
            last_message
                .content
                .push(MessageContent::ToolUse(tool_use.clone()));
        } else {
            events_tx
                .unbounded_send(Ok(AgentResponseEvent::ToolCallUpdate(
                    acp::ToolCallUpdate {
                        id: acp::ToolCallId(tool_use.id.to_string().into()),
                        fields: acp::ToolCallUpdateFields {
                            raw_input: Some(tool_use.input.clone()),
                            ..Default::default()
                        },
                    },
                )))
                .ok();
        }

        if !tool_use.is_input_complete {
            return None;
        }

        if let Some(tool) = self.tools.get(tool_use.name.as_ref()) {
            events_tx
                .unbounded_send(Ok(AgentResponseEvent::ToolCallUpdate(
                    acp::ToolCallUpdate {
                        id: acp::ToolCallId(tool_use.id.to_string().into()),
                        fields: acp::ToolCallUpdateFields {
                            status: Some(acp::ToolCallStatus::InProgress),
                            ..Default::default()
                        },
                    },
                )))
                .ok();

            let pending_tool_result = tool.clone().run(tool_use.input, cx);

            Some(cx.foreground_executor().spawn(async move {
                match pending_tool_result.await {
                    Ok(tool_output) => LanguageModelToolResult {
                        tool_use_id: tool_use.id,
                        tool_name: tool_use.name,
                        is_error: false,
                        content: LanguageModelToolResultContent::Text(Arc::from(tool_output)),
                        output: None,
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
        } else {
            let content = format!("No tool named {} exists", tool_use.name);
            Some(Task::ready(LanguageModelToolResult {
                content: LanguageModelToolResultContent::Text(Arc::from(content)),
                tool_use_id: tool_use.id,
                tool_name: tool_use.name,
                is_error: true,
                output: None,
            }))
        }
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

    fn build_completion_request(
        &self,
        completion_intent: CompletionIntent,
        cx: &mut App,
    ) -> LanguageModelRequest {
        log::debug!("Building completion request");
        log::debug!("Completion intent: {:?}", completion_intent);
        log::debug!("Completion mode: {:?}", self.completion_mode);

        let messages = self.build_request_messages(cx);
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
                        .input_schema(LanguageModelToolSchemaFormat::JsonSchema)
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

    fn build_request_messages(&self, cx: &App) -> Vec<LanguageModelRequestMessage> {
        log::trace!(
            "Building request messages from {} thread messages",
            self.messages.len()
        );

        let messages = self
            .build_system_message(cx)
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
    type Input: for<'de> Deserialize<'de> + JsonSchema;

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

    /// Returns the JSON schema that describes the tool's input.
    fn input_schema(&self, _format: LanguageModelToolSchemaFormat) -> Schema {
        schemars::schema_for!(Self::Input)
    }

    /// Runs the tool with the provided input.
    fn run(self: Arc<Self>, input: Self::Input, cx: &mut App) -> Task<Result<String>>;

    fn erase(self) -> Arc<dyn AnyAgentTool> {
        Arc::new(Erased(Arc::new(self)))
    }
}

pub struct Erased<T>(T);

pub trait AnyAgentTool {
    fn name(&self) -> SharedString;
    fn description(&self, cx: &mut App) -> SharedString;
    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value>;
    fn run(self: Arc<Self>, input: serde_json::Value, cx: &mut App) -> Task<Result<String>>;
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

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(self.0.input_schema(format))?)
    }

    fn run(self: Arc<Self>, input: serde_json::Value, cx: &mut App) -> Task<Result<String>> {
        let parsed_input: Result<T::Input> = serde_json::from_value(input).map_err(Into::into);
        match parsed_input {
            Ok(input) => self.0.clone().run(input, cx),
            Err(error) => Task::ready(Err(anyhow!(error))),
        }
    }
}

fn to_acp_stop_reason(reason: StopReason) -> Option<acp::StopReason> {
    match reason {
        StopReason::EndTurn => Some(acp::StopReason::EndTurn),
        StopReason::MaxTokens => Some(acp::StopReason::MaxTokens),
        StopReason::Refusal => Some(acp::StopReason::Refusal),
        StopReason::ToolUse => None,
    }
}

fn to_acp_tool_call_update(tool_result: &LanguageModelToolResult) -> acp::ToolCallUpdate {
    let status = if tool_result.is_error {
        acp::ToolCallStatus::Failed
    } else {
        acp::ToolCallStatus::Completed
    };
    let content = match &tool_result.content {
        LanguageModelToolResultContent::Text(text) => text.to_string().into(),
        LanguageModelToolResultContent::Image(LanguageModelImage { source, .. }) => {
            acp::ToolCallContent::Content {
                content: acp::ContentBlock::Image(acp::ImageContent {
                    annotations: None,
                    data: source.to_string(),
                    mime_type: ImageFormat::Png.mime_type().to_string(),
                }),
            }
        }
    };
    acp::ToolCallUpdate {
        id: acp::ToolCallId(tool_result.tool_use_id.to_string().into()),
        fields: acp::ToolCallUpdateFields {
            status: Some(status),
            content: Some(vec![content]),
            ..Default::default()
        },
    }
}
