use std::sync::Arc;

use anyhow::Result;
use assistant_tool::ToolWorkingSet;
use collections::HashMap;
use futures::future::Shared;
use futures::{FutureExt as _, StreamExt as _};
use gpui::{AppContext, EventEmitter, ModelContext, Task};
use language_model::{
    LanguageModel, LanguageModelCompletionEvent, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelToolResult, LanguageModelToolUse, LanguageModelToolUseId, MessageContent, Role,
    StopReason,
};
use serde::{Deserialize, Serialize};
use util::post_inc;

#[derive(Debug, Clone, Copy)]
pub enum RequestKind {
    Chat,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct MessageId(usize);

impl MessageId {
    fn post_inc(&mut self) -> Self {
        Self(post_inc(&mut self.0))
    }
}

/// A message in a [`Thread`].
#[derive(Debug, Clone)]
pub struct Message {
    pub id: MessageId,
    pub role: Role,
    pub text: String,
}

/// A thread of conversation with the LLM.
pub struct Thread {
    messages: Vec<Message>,
    next_message_id: MessageId,
    completion_count: usize,
    pending_completions: Vec<PendingCompletion>,
    tools: Arc<ToolWorkingSet>,
    tool_uses_by_message: HashMap<MessageId, Vec<LanguageModelToolUse>>,
    tool_results_by_message: HashMap<MessageId, Vec<LanguageModelToolResult>>,
    pending_tool_uses_by_id: HashMap<LanguageModelToolUseId, PendingToolUse>,
}

impl Thread {
    pub fn new(tools: Arc<ToolWorkingSet>, _cx: &mut ModelContext<Self>) -> Self {
        Self {
            messages: Vec::new(),
            next_message_id: MessageId(0),
            completion_count: 0,
            pending_completions: Vec::new(),
            tools,
            tool_uses_by_message: HashMap::default(),
            tool_results_by_message: HashMap::default(),
            pending_tool_uses_by_id: HashMap::default(),
        }
    }

    pub fn messages(&self) -> impl Iterator<Item = &Message> {
        self.messages.iter()
    }

    pub fn tools(&self) -> &Arc<ToolWorkingSet> {
        &self.tools
    }

    pub fn pending_tool_uses(&self) -> Vec<&PendingToolUse> {
        self.pending_tool_uses_by_id.values().collect()
    }

    pub fn insert_user_message(&mut self, text: impl Into<String>) {
        self.messages.push(Message {
            id: self.next_message_id.post_inc(),
            role: Role::User,
            text: text.into(),
        });
    }

    pub fn to_completion_request(
        &self,
        _request_kind: RequestKind,
        _cx: &AppContext,
    ) -> LanguageModelRequest {
        let mut request = LanguageModelRequest {
            messages: vec![],
            tools: Vec::new(),
            stop: Vec::new(),
            temperature: None,
        };

        for message in &self.messages {
            let mut request_message = LanguageModelRequestMessage {
                role: message.role,
                content: Vec::new(),
                cache: false,
            };

            if let Some(tool_results) = self.tool_results_by_message.get(&message.id) {
                for tool_result in tool_results {
                    request_message
                        .content
                        .push(MessageContent::ToolResult(tool_result.clone()));
                }
            }

            if !message.text.is_empty() {
                request_message
                    .content
                    .push(MessageContent::Text(message.text.clone()));
            }

            if let Some(tool_uses) = self.tool_uses_by_message.get(&message.id) {
                for tool_use in tool_uses {
                    request_message
                        .content
                        .push(MessageContent::ToolUse(tool_use.clone()));
                }
            }

            request.messages.push(request_message);
        }

        request
    }

    pub fn stream_completion(
        &mut self,
        request: LanguageModelRequest,
        model: Arc<dyn LanguageModel>,
        cx: &mut ModelContext<Self>,
    ) {
        let pending_completion_id = post_inc(&mut self.completion_count);

        let task = cx.spawn(|thread, mut cx| async move {
            let stream = model.stream_completion(request, &cx);
            let stream_completion = async {
                let mut events = stream.await?;
                let mut stop_reason = StopReason::EndTurn;

                while let Some(event) = events.next().await {
                    let event = event?;

                    thread.update(&mut cx, |thread, cx| {
                        match event {
                            LanguageModelCompletionEvent::StartMessage { .. } => {
                                thread.messages.push(Message {
                                    id: thread.next_message_id.post_inc(),
                                    role: Role::Assistant,
                                    text: String::new(),
                                });
                            }
                            LanguageModelCompletionEvent::Stop(reason) => {
                                stop_reason = reason;
                            }
                            LanguageModelCompletionEvent::Text(chunk) => {
                                if let Some(last_message) = thread.messages.last_mut() {
                                    if last_message.role == Role::Assistant {
                                        last_message.text.push_str(&chunk);
                                    }
                                }
                            }
                            LanguageModelCompletionEvent::ToolUse(tool_use) => {
                                if let Some(last_assistant_message) = thread
                                    .messages
                                    .iter()
                                    .rfind(|message| message.role == Role::Assistant)
                                {
                                    thread
                                        .tool_uses_by_message
                                        .entry(last_assistant_message.id)
                                        .or_default()
                                        .push(tool_use.clone());

                                    thread.pending_tool_uses_by_id.insert(
                                        tool_use.id.clone(),
                                        PendingToolUse {
                                            assistant_message_id: last_assistant_message.id,
                                            id: tool_use.id,
                                            name: tool_use.name,
                                            input: tool_use.input,
                                            status: PendingToolUseStatus::Idle,
                                        },
                                    );
                                }
                            }
                        }

                        cx.emit(ThreadEvent::StreamedCompletion);
                        cx.notify();
                    })?;

                    smol::future::yield_now().await;
                }

                thread.update(&mut cx, |thread, _cx| {
                    thread
                        .pending_completions
                        .retain(|completion| completion.id != pending_completion_id);
                })?;

                anyhow::Ok(stop_reason)
            };

            let result = stream_completion.await;

            thread
                .update(&mut cx, |_thread, cx| {
                    let error_message = if let Some(error) = result.as_ref().err() {
                        let error_message = error
                            .chain()
                            .map(|err| err.to_string())
                            .collect::<Vec<_>>()
                            .join("\n");
                        Some(error_message)
                    } else {
                        None
                    };

                    if let Some(error_message) = error_message {
                        eprintln!("Completion failed: {error_message:?}");
                    }

                    if let Ok(stop_reason) = result {
                        match stop_reason {
                            StopReason::ToolUse => {
                                cx.emit(ThreadEvent::UsePendingTools);
                            }
                            StopReason::EndTurn => {}
                            StopReason::MaxTokens => {}
                        }
                    }
                })
                .ok();
        });

        self.pending_completions.push(PendingCompletion {
            id: pending_completion_id,
            _task: task,
        });
    }

    pub fn insert_tool_output(
        &mut self,
        assistant_message_id: MessageId,
        tool_use_id: LanguageModelToolUseId,
        output: Task<Result<String>>,
        cx: &mut ModelContext<Self>,
    ) {
        let insert_output_task = cx.spawn(|thread, mut cx| {
            let tool_use_id = tool_use_id.clone();
            async move {
                let output = output.await;
                thread
                    .update(&mut cx, |thread, cx| {
                        // The tool use was requested by an Assistant message,
                        // so we want to attach the tool results to the next
                        // user message.
                        let next_user_message = MessageId(assistant_message_id.0 + 1);

                        let tool_results = thread
                            .tool_results_by_message
                            .entry(next_user_message)
                            .or_default();

                        match output {
                            Ok(output) => {
                                tool_results.push(LanguageModelToolResult {
                                    tool_use_id: tool_use_id.to_string(),
                                    content: output,
                                    is_error: false,
                                });

                                cx.emit(ThreadEvent::ToolFinished { tool_use_id });
                            }
                            Err(err) => {
                                tool_results.push(LanguageModelToolResult {
                                    tool_use_id: tool_use_id.to_string(),
                                    content: err.to_string(),
                                    is_error: true,
                                });

                                if let Some(tool_use) =
                                    thread.pending_tool_uses_by_id.get_mut(&tool_use_id)
                                {
                                    tool_use.status = PendingToolUseStatus::Error(err.to_string());
                                }
                            }
                        }
                    })
                    .ok();
            }
        });

        if let Some(tool_use) = self.pending_tool_uses_by_id.get_mut(&tool_use_id) {
            tool_use.status = PendingToolUseStatus::Running {
                _task: insert_output_task.shared(),
            };
        }
    }
}

#[derive(Debug, Clone)]
pub enum ThreadEvent {
    StreamedCompletion,
    UsePendingTools,
    ToolFinished {
        #[allow(unused)]
        tool_use_id: LanguageModelToolUseId,
    },
}

impl EventEmitter<ThreadEvent> for Thread {}

struct PendingCompletion {
    id: usize,
    _task: Task<()>,
}

#[derive(Debug, Clone)]
pub struct PendingToolUse {
    pub id: LanguageModelToolUseId,
    /// The ID of the Assistant message in which the tool use was requested.
    pub assistant_message_id: MessageId,
    pub name: String,
    pub input: serde_json::Value,
    pub status: PendingToolUseStatus,
}

#[derive(Debug, Clone)]
pub enum PendingToolUseStatus {
    Idle,
    Running { _task: Shared<Task<()>> },
    Error(#[allow(unused)] String),
}

impl PendingToolUseStatus {
    pub fn is_idle(&self) -> bool {
        matches!(self, PendingToolUseStatus::Idle)
    }
}
