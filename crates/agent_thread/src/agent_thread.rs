#[cfg(test)]
mod tests;

use anyhow::Result;
use assistant_tool::{ActionLog, Tool};
use futures::{channel::mpsc, stream::FuturesUnordered};
use gpui::{Context, Entity, Task};
use language_model::{
    LanguageModel, LanguageModelCompletionEvent, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelRequestTool, LanguageModelToolResult, LanguageModelToolSchemaFormat,
    LanguageModelToolUse, LanguageModelToolUseId, MessageContent, Role, StopReason,
};
use project::Project;
use smol::stream::StreamExt;
use std::{collections::BTreeMap, sync::Arc};
use util::ResultExt;

#[derive(Debug)]
pub struct AgentMessage {
    pub role: Role,
    pub content: Vec<MessageContent>,
}

impl AgentMessage {
    fn to_request_message(&self) -> LanguageModelRequestMessage {
        LanguageModelRequestMessage {
            role: self.role,
            content: self.content.clone(),
            cache: false, // TODO: Figure out caching
        }
    }
}

pub type AgentResponseEvent = LanguageModelCompletionEvent;

pub struct AgentThread {
    sent: Vec<AgentMessage>,
    unsent: Vec<AgentMessage>,
    streaming: Option<Task<Option<()>>>,
    tools: BTreeMap<Arc<str>, Arc<dyn Tool>>,
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
}

impl AgentThread {
    pub fn new(project: Entity<Project>, action_log: Entity<ActionLog>) -> Self {
        Self {
            sent: Vec::new(),
            unsent: Vec::new(),
            streaming: None,
            tools: BTreeMap::default(),
            project,
            action_log,
        }
    }

    pub fn add_tool(&mut self, tool: Arc<dyn Tool>) {
        let name = Arc::from(tool.name());
        self.tools.insert(name, tool);
    }

    /// Cancels in-flight streaming, aborting any pending tool calls.
    pub fn cancel_streaming(&mut self, cx: &mut Context<Self>) -> bool {
        self.unsent.clear();
        self.streaming.take().is_some()
    }

    /// Sending a message results in the model streaming a response, which could include tool calls.
    /// After calling tools, the model will stops and waits for any outstanding tool calls to be completed and their results sent.
    /// The returned channel will report all the occurrences in which the model stops before erroring or ending its turn.
    pub fn send(
        &mut self,
        model: Arc<dyn LanguageModel>,
        content: impl Into<MessageContent>,
        cx: &mut Context<Self>,
    ) -> mpsc::UnboundedReceiver<Result<AgentResponseEvent>> {
        self.cancel_streaming(cx);
        let events = mpsc::unbounded();
        self.enqueue_unsent(model, content, events.0, cx);
        events.1
    }

    /// Internal method which is called by send and also any tool call results.
    /// If currently streaming a completion, these events will be sent when the streaming stops.
    fn enqueue_unsent(
        &mut self,
        model: Arc<dyn LanguageModel>,
        content: impl Into<MessageContent>,
        events_tx: mpsc::UnboundedSender<Result<AgentResponseEvent>>,
        cx: &mut Context<Self>,
    ) {
        cx.notify();

        self.unsent.push(AgentMessage {
            role: Role::User,
            content: vec![content.into()],
        });
        if !dbg!(self.streaming.is_some()) {
            self.flush_unsent_messages(model, events_tx, cx)
        }
    }

    fn flush_unsent_messages(
        &mut self,
        model: Arc<dyn LanguageModel>,
        events_tx: mpsc::UnboundedSender<Result<AgentResponseEvent>>,
        cx: &mut Context<Self>,
    ) {
        cx.notify();
        self.streaming = Some(
            cx.spawn(async move |thread, cx| {
                let mut subtasks = FuturesUnordered::new();

                // Perform completion requests until the unsent messages are empty.
                loop {
                    let unsent =
                        thread.update(cx, |thread, _cx| std::mem::take(&mut thread.unsent))?;

                    if unsent.is_empty() {
                        thread.update(cx, |thread, _cx| thread.streaming.take())?;
                        break;
                    }

                    let completion_request = thread.update(cx, |thread, _cx| {
                        thread.sent.extend(unsent);
                        thread.build_completion_request()
                    })?;

                    let mut events = model.stream_completion(completion_request, cx).await?;
                    while let Some(event) = events.next().await {
                        match event {
                            Ok(event) => {
                                thread
                                    .update(cx, |thread, cx| {
                                        let subtask = thread.handle_stream_event(
                                            &model,
                                            event,
                                            events_tx.clone(),
                                            cx,
                                        );
                                        subtasks.extend(subtask);
                                    })
                                    .ok();
                            }
                            Err(error) => {
                                events_tx.unbounded_send(Err(error)).ok();
                                break;
                            }
                        }
                    }

                    // Wait for any tasks we spawned to enqueue tool results before looping again.
                    subtasks.next().await;
                }

                anyhow::Ok(())
            })
            .log_err_in_task(cx),
        );
    }

    fn handle_stream_event(
        &mut self,
        model: &Arc<dyn LanguageModel>,
        event: LanguageModelCompletionEvent,
        events_tx: mpsc::UnboundedSender<Result<AgentResponseEvent>>,
        cx: &mut Context<Self>,
    ) -> Option<Task<()>> {
        use LanguageModelCompletionEvent::*;
        events_tx.unbounded_send(Ok(event.clone())).ok();

        match dbg!(event) {
            Text(new_text) => self.handle_text_event(new_text, cx),
            Thinking { text, signature } => {
                dbg!(text, signature);
            }
            ToolUse(tool_use) => {
                return self.handle_tool_use_event(model.clone(), tool_use, events_tx, cx);
            }
            StartMessage { message_id, role } => {
                self.sent.push(AgentMessage {
                    role,
                    content: Vec::new(),
                });
            }
            UsageUpdate(token_usage) => {}
            Stop(stop_reason) => {}
        }

        None
    }

    fn handle_text_event(&mut self, new_text: String, cx: &mut Context<Self>) {
        if let Some(last_message) = self.sent.last_mut() {
            debug_assert!(last_message.role == Role::Assistant);
            if let Some(MessageContent::Text(text)) = last_message.content.last_mut() {
                text.push_str(&new_text);
            } else {
                last_message.content.push(MessageContent::Text(new_text));
            }

            cx.notify();
        } else {
            todo!("does this happen in practice?");
        }
    }

    fn handle_tool_use_event(
        &mut self,
        model: Arc<dyn LanguageModel>,
        tool_use: LanguageModelToolUse,
        events_tx: mpsc::UnboundedSender<Result<AgentResponseEvent>>,
        cx: &mut Context<Self>,
    ) -> Option<Task<()>> {
        if let Some(last_message) = self.sent.last_mut() {
            debug_assert!(last_message.role == Role::Assistant);
            last_message.content.push(tool_use.clone().into());
            cx.notify();
        } else {
            todo!("does this happen in practice?");
        }

        if let Some(tool) = self.tools.get(&tool_use.name) {
            let pending_tool_result = tool.clone().run(
                tool_use.input,
                &self.build_request_messages(),
                self.project.clone(),
                self.action_log.clone(),
                cx,
            );

            Some(cx.spawn(async move |thread, cx| {
                let tool_result = match pending_tool_result.output.await {
                    Ok(tool_output) => LanguageModelToolResult {
                        tool_use_id: tool_use.id,
                        tool_name: tool_use.name,
                        is_error: false,
                        content: Arc::from(tool_output),
                    },
                    Err(error) => LanguageModelToolResult {
                        tool_use_id: tool_use.id,
                        tool_name: tool_use.name,
                        is_error: true,
                        content: Arc::from(error.to_string()),
                    },
                };

                thread
                    .update(cx, |thread, cx| {
                        thread.enqueue_unsent(model, tool_result, events_tx, cx)
                    })
                    .ok();
            }))
        } else {
            self.enqueue_unsent(
                model,
                LanguageModelToolResult {
                    tool_use_id: tool_use.id,
                    tool_name: tool_use.name,
                    is_error: true,
                    content: Arc::from("tool does not exist"),
                },
                events_tx,
                cx,
            );
            None
        }
    }

    fn build_completion_request(&self) -> LanguageModelRequest {
        LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            messages: self.build_request_messages(),
            tools: self
                .tools
                .values()
                .filter_map(|tool| {
                    Some(LanguageModelRequestTool {
                        name: tool.name(),
                        description: tool.description(),
                        input_schema: tool
                            .input_schema(LanguageModelToolSchemaFormat::JsonSchema)
                            .log_err()?,
                    })
                })
                .collect(),
            stop: Vec::new(),
            temperature: None,
        }
    }

    fn build_request_messages(&self) -> Vec<LanguageModelRequestMessage> {
        self.sent
            .iter()
            .map(|message| LanguageModelRequestMessage {
                role: message.role,
                content: message.content.clone(),
                cache: false,
            })
            .collect()
    }
}
