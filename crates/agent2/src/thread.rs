use crate::templates::Templates;
use anyhow::{anyhow, Result};
use cloud_llm_client::{CompletionIntent, CompletionMode};
use futures::{channel::mpsc, future};
use gpui::{App, Context, SharedString, Task};
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelRequest, LanguageModelRequestMessage, LanguageModelRequestTool,
    LanguageModelToolResult, LanguageModelToolResultContent, LanguageModelToolSchemaFormat,
    LanguageModelToolUse, MessageContent, Role, StopReason,
};
use schemars::{JsonSchema, Schema};
use serde::Deserialize;
use smol::stream::StreamExt;
use std::{collections::BTreeMap, sync::Arc};
use util::ResultExt;

#[derive(Debug)]
pub struct AgentMessage {
    pub role: Role,
    pub content: Vec<MessageContent>,
}

pub type AgentResponseEvent = LanguageModelCompletionEvent;

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
    system_prompts: Vec<Arc<dyn Prompt>>,
    tools: BTreeMap<SharedString, Arc<dyn AgentToolErased>>,
    templates: Arc<Templates>,
    // project: Entity<Project>,
    // action_log: Entity<ActionLog>,
}

impl Thread {
    pub fn new(templates: Arc<Templates>) -> Self {
        Self {
            messages: Vec::new(),
            completion_mode: CompletionMode::Normal,
            system_prompts: Vec::new(),
            running_turn: None,
            tools: BTreeMap::default(),
            templates,
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

    /// Sending a message results in the model streaming a response, which could include tool calls.
    /// After calling tools, the model will stops and waits for any outstanding tool calls to be completed and their results sent.
    /// The returned channel will report all the occurrences in which the model stops before erroring or ending its turn.
    pub fn send(
        &mut self,
        model: Arc<dyn LanguageModel>,
        content: impl Into<MessageContent>,
        cx: &mut Context<Self>,
    ) -> mpsc::UnboundedReceiver<Result<AgentResponseEvent, LanguageModelCompletionError>> {
        cx.notify();
        let (events_tx, events_rx) =
            mpsc::unbounded::<Result<AgentResponseEvent, LanguageModelCompletionError>>();

        let system_message = self.build_system_message(cx);
        self.messages.extend(system_message);

        self.messages.push(AgentMessage {
            role: Role::User,
            content: vec![content.into()],
        });
        self.running_turn = Some(cx.spawn(async move |thread, cx| {
            let turn_result = async {
                // Perform one request, then keep looping if the model makes tool calls.
                let mut completion_intent = CompletionIntent::UserPrompt;
                loop {
                    let request = thread.update(cx, |thread, cx| {
                        thread.build_completion_request(completion_intent, cx)
                    })?;

                    // println!(
                    //     "request: {}",
                    //     serde_json::to_string_pretty(&request).unwrap()
                    // );

                    // Stream events, appending to messages and collecting up tool uses.
                    let mut events = model.stream_completion(request, cx).await?;
                    let mut tool_uses = Vec::new();
                    while let Some(event) = events.next().await {
                        match event {
                            Ok(event) => {
                                thread
                                    .update(cx, |thread, cx| {
                                        tool_uses.extend(thread.handle_streamed_completion_event(
                                            event,
                                            events_tx.clone(),
                                            cx,
                                        ));
                                    })
                                    .ok();
                            }
                            Err(error) => {
                                events_tx.unbounded_send(Err(error)).ok();
                                break;
                            }
                        }
                    }

                    // If there are no tool uses, the turn is done.
                    if tool_uses.is_empty() {
                        break;
                    }

                    // If there are tool uses, wait for their results to be
                    // computed, then send them together in a single message on
                    // the next loop iteration.
                    let tool_results = future::join_all(tool_uses).await;
                    thread
                        .update(cx, |thread, _cx| {
                            thread.messages.push(AgentMessage {
                                role: Role::User,
                                content: tool_results
                                    .into_iter()
                                    .map(MessageContent::ToolResult)
                                    .collect(),
                            });
                        })
                        .ok();
                    completion_intent = CompletionIntent::ToolResults;
                }

                Ok(())
            }
            .await;

            if let Err(error) = turn_result {
                events_tx.unbounded_send(Err(error)).ok();
            }
        }));
        events_rx
    }

    pub fn build_system_message(&mut self, cx: &App) -> Option<AgentMessage> {
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

        (!system_message.content.is_empty()).then_some(system_message)
    }

    /// A helper method that's called on every streamed completion event.
    /// Returns an optional tool result task, which the main agentic loop in
    /// send will send back to the model when it resolves.
    fn handle_streamed_completion_event(
        &mut self,
        event: LanguageModelCompletionEvent,
        events_tx: mpsc::UnboundedSender<Result<AgentResponseEvent, LanguageModelCompletionError>>,
        cx: &mut Context<Self>,
    ) -> Option<Task<LanguageModelToolResult>> {
        use LanguageModelCompletionEvent::*;
        events_tx.unbounded_send(Ok(event.clone())).ok();

        match event {
            Text(new_text) => self.handle_text_event(new_text, cx),
            Thinking {
                text: _text,
                signature: _signature,
            } => {
                todo!()
            }
            ToolUse(tool_use) => {
                return self.handle_tool_use_event(tool_use, cx);
            }
            StartMessage { .. } => {
                self.messages.push(AgentMessage {
                    role: Role::Assistant,
                    content: Vec::new(),
                });
            }
            UsageUpdate(_) => {}
            Stop(stop_reason) => self.handle_stop_event(stop_reason),
            StatusUpdate(_completion_request_status) => {}
            RedactedThinking { data: _data } => todo!(),
            ToolUseJsonParseError {
                id: _id,
                tool_name: _tool_name,
                raw_input: _raw_input,
                json_parse_error: _json_parse_error,
            } => todo!(),
        }

        None
    }

    fn handle_stop_event(&mut self, stop_reason: StopReason) {
        match stop_reason {
            StopReason::EndTurn | StopReason::ToolUse => {}
            StopReason::MaxTokens => todo!(),
            StopReason::Refusal => todo!(),
        }
    }

    fn handle_text_event(&mut self, new_text: String, cx: &mut Context<Self>) {
        let last_message = self.last_assistant_message();
        if let Some(MessageContent::Text(text)) = last_message.content.last_mut() {
            text.push_str(&new_text);
        } else {
            last_message.content.push(MessageContent::Text(new_text));
        }

        cx.notify();
    }

    fn handle_tool_use_event(
        &mut self,
        tool_use: LanguageModelToolUse,
        cx: &mut Context<Self>,
    ) -> Option<Task<LanguageModelToolResult>> {
        cx.notify();

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
            last_message
                .content
                .push(MessageContent::ToolUse(tool_use.clone()));
        }

        if !tool_use.is_input_complete {
            return None;
        }

        if let Some(tool) = self.tools.get(tool_use.name.as_ref()) {
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
            Some(Task::ready(LanguageModelToolResult {
                content: LanguageModelToolResultContent::Text(Arc::from(format!(
                    "No tool named {} exists",
                    tool_use.name
                ))),
                tool_use_id: tool_use.id,
                tool_name: tool_use.name,
                is_error: true,
                output: None,
            }))
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

    fn build_completion_request(
        &self,
        completion_intent: CompletionIntent,
        cx: &mut App,
    ) -> LanguageModelRequest {
        LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: Some(completion_intent),
            mode: Some(self.completion_mode),
            messages: self.build_request_messages(),
            tools: self
                .tools
                .values()
                .filter_map(|tool| {
                    Some(LanguageModelRequestTool {
                        name: tool.name().to_string(),
                        description: tool.description(cx).to_string(),
                        input_schema: tool
                            .input_schema(LanguageModelToolSchemaFormat::JsonSchema)
                            .log_err()?,
                    })
                })
                .collect(),
            tool_choice: None,
            stop: Vec::new(),
            temperature: None,
            thinking_allowed: false,
        }
    }

    fn build_request_messages(&self) -> Vec<LanguageModelRequestMessage> {
        self.messages
            .iter()
            .map(|message| LanguageModelRequestMessage {
                role: message.role,
                content: message.content.clone(),
                cache: false,
            })
            .collect()
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

    fn erase(self) -> Arc<dyn AgentToolErased> {
        Arc::new(Erased(Arc::new(self)))
    }
}

pub struct Erased<T>(T);

pub trait AgentToolErased {
    fn name(&self) -> SharedString;
    fn description(&self, cx: &mut App) -> SharedString;
    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value>;
    fn run(self: Arc<Self>, input: serde_json::Value, cx: &mut App) -> Task<Result<String>>;
}

impl<T> AgentToolErased for Erased<Arc<T>>
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
