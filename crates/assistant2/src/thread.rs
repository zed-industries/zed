use std::sync::Arc;

use anyhow::Result;
use assistant_tool::ToolWorkingSet;
use chrono::{DateTime, Utc};
use collections::{BTreeMap, HashMap, HashSet};
use futures::future::Shared;
use futures::{FutureExt as _, StreamExt as _};
use gpui::{AppContext, EventEmitter, ModelContext, SharedString, Task};
use language_model::{
    LanguageModel, LanguageModelCompletionEvent, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelToolResult, LanguageModelToolUse,
    LanguageModelToolUseId, MessageContent, Role, StopReason,
};
use language_models::provider::cloud::{MaxMonthlySpendReachedError, PaymentRequiredError};
use serde::{Deserialize, Serialize};
use util::{post_inc, TryFutureExt as _};
use uuid::Uuid;

use crate::context::{attach_context_to_message, ContextId, ContextSnapshot};

#[derive(Debug, Clone, Copy)]
pub enum RequestKind {
    Chat,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct ThreadId(Arc<str>);

impl ThreadId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string().into())
    }
}

impl std::fmt::Display for ThreadId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
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
    id: ThreadId,
    updated_at: DateTime<Utc>,
    summary: Option<SharedString>,
    pending_summary: Task<Option<()>>,
    messages: Vec<Message>,
    next_message_id: MessageId,
    context: BTreeMap<ContextId, ContextSnapshot>,
    context_by_message: HashMap<MessageId, Vec<ContextId>>,
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
            id: ThreadId::new(),
            updated_at: Utc::now(),
            summary: None,
            pending_summary: Task::ready(None),
            messages: Vec::new(),
            next_message_id: MessageId(0),
            context: BTreeMap::default(),
            context_by_message: HashMap::default(),
            completion_count: 0,
            pending_completions: Vec::new(),
            tools,
            tool_uses_by_message: HashMap::default(),
            tool_results_by_message: HashMap::default(),
            pending_tool_uses_by_id: HashMap::default(),
        }
    }

    pub fn id(&self) -> &ThreadId {
        &self.id
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn touch_updated_at(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn summary(&self) -> Option<SharedString> {
        self.summary.clone()
    }

    pub fn summary_or_default(&self) -> SharedString {
        const DEFAULT: SharedString = SharedString::new_static("New Thread");
        self.summary.clone().unwrap_or(DEFAULT)
    }

    pub fn set_summary(&mut self, summary: impl Into<SharedString>, cx: &mut ModelContext<Self>) {
        self.summary = Some(summary.into());
        cx.emit(ThreadEvent::SummaryChanged);
    }

    pub fn message(&self, id: MessageId) -> Option<&Message> {
        self.messages.iter().find(|message| message.id == id)
    }

    pub fn messages(&self) -> impl Iterator<Item = &Message> {
        self.messages.iter()
    }

    pub fn is_streaming(&self) -> bool {
        !self.pending_completions.is_empty()
    }

    pub fn tools(&self) -> &Arc<ToolWorkingSet> {
        &self.tools
    }

    pub fn context_for_message(&self, id: MessageId) -> Option<Vec<ContextSnapshot>> {
        let context = self.context_by_message.get(&id)?;
        Some(
            context
                .into_iter()
                .filter_map(|context_id| self.context.get(&context_id))
                .cloned()
                .collect::<Vec<_>>(),
        )
    }

    pub fn pending_tool_uses(&self) -> Vec<&PendingToolUse> {
        self.pending_tool_uses_by_id.values().collect()
    }

    pub fn insert_user_message(
        &mut self,
        text: impl Into<String>,
        context: Vec<ContextSnapshot>,
        cx: &mut ModelContext<Self>,
    ) {
        let message_id = self.insert_message(Role::User, text, cx);
        let context_ids = context.iter().map(|context| context.id).collect::<Vec<_>>();
        self.context
            .extend(context.into_iter().map(|context| (context.id, context)));
        self.context_by_message.insert(message_id, context_ids);
    }

    pub fn insert_message(
        &mut self,
        role: Role,
        text: impl Into<String>,
        cx: &mut ModelContext<Self>,
    ) -> MessageId {
        let id = self.next_message_id.post_inc();
        self.messages.push(Message {
            id,
            role,
            text: text.into(),
        });
        self.touch_updated_at();
        cx.emit(ThreadEvent::MessageAdded(id));
        id
    }

    /// Returns the representation of this [`Thread`] in a textual form.
    ///
    /// This is the representation we use when attaching a thread as context to another thread.
    pub fn text(&self) -> String {
        let mut text = String::new();

        for message in &self.messages {
            text.push_str(match message.role {
                language_model::Role::User => "User:",
                language_model::Role::Assistant => "Assistant:",
                language_model::Role::System => "System:",
            });
            text.push('\n');

            text.push_str(&message.text);
            text.push('\n');
        }

        text
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

        let mut referenced_context_ids = HashSet::default();

        for message in &self.messages {
            if let Some(context_ids) = self.context_by_message.get(&message.id) {
                referenced_context_ids.extend(context_ids);
            }

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

        if !referenced_context_ids.is_empty() {
            let mut context_message = LanguageModelRequestMessage {
                role: Role::User,
                content: Vec::new(),
                cache: false,
            };

            let referenced_context = referenced_context_ids
                .into_iter()
                .filter_map(|context_id| self.context.get(context_id))
                .cloned();
            attach_context_to_message(&mut context_message, referenced_context);

            request.messages.push(context_message);
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
                                thread.insert_message(Role::Assistant, String::new(), cx);
                            }
                            LanguageModelCompletionEvent::Stop(reason) => {
                                stop_reason = reason;
                            }
                            LanguageModelCompletionEvent::Text(chunk) => {
                                if let Some(last_message) = thread.messages.last_mut() {
                                    if last_message.role == Role::Assistant {
                                        last_message.text.push_str(&chunk);
                                        cx.emit(ThreadEvent::StreamedAssistantText(
                                            last_message.id,
                                            chunk,
                                        ));
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

                        thread.touch_updated_at();
                        cx.emit(ThreadEvent::StreamedCompletion);
                        cx.notify();
                    })?;

                    smol::future::yield_now().await;
                }

                thread.update(&mut cx, |thread, cx| {
                    thread
                        .pending_completions
                        .retain(|completion| completion.id != pending_completion_id);

                    if thread.summary.is_none() && thread.messages.len() >= 2 {
                        thread.summarize(cx);
                    }
                })?;

                anyhow::Ok(stop_reason)
            };

            let result = stream_completion.await;

            thread
                .update(&mut cx, |thread, cx| match result.as_ref() {
                    Ok(stop_reason) => match stop_reason {
                        StopReason::ToolUse => {
                            cx.emit(ThreadEvent::UsePendingTools);
                        }
                        StopReason::EndTurn => {}
                        StopReason::MaxTokens => {}
                    },
                    Err(error) => {
                        if error.is::<PaymentRequiredError>() {
                            cx.emit(ThreadEvent::ShowError(ThreadError::PaymentRequired));
                        } else if error.is::<MaxMonthlySpendReachedError>() {
                            cx.emit(ThreadEvent::ShowError(ThreadError::MaxMonthlySpendReached));
                        } else {
                            let error_message = error
                                .chain()
                                .map(|err| err.to_string())
                                .collect::<Vec<_>>()
                                .join("\n");
                            cx.emit(ThreadEvent::ShowError(ThreadError::Message(
                                SharedString::from(error_message.clone()),
                            )));
                        }

                        thread.cancel_last_completion();
                    }
                })
                .ok();
        });

        self.pending_completions.push(PendingCompletion {
            id: pending_completion_id,
            _task: task,
        });
    }

    pub fn summarize(&mut self, cx: &mut ModelContext<Self>) {
        let Some(provider) = LanguageModelRegistry::read_global(cx).active_provider() else {
            return;
        };
        let Some(model) = LanguageModelRegistry::read_global(cx).active_model() else {
            return;
        };

        if !provider.is_authenticated(cx) {
            return;
        }

        let mut request = self.to_completion_request(RequestKind::Chat, cx);
        request.messages.push(LanguageModelRequestMessage {
            role: Role::User,
            content: vec![
                "Generate a concise 3-7 word title for this conversation, omitting punctuation. Go straight to the title, without any preamble and prefix like `Here's a concise suggestion:...` or `Title:`"
                    .into(),
            ],
            cache: false,
        });

        self.pending_summary = cx.spawn(|this, mut cx| {
            async move {
                let stream = model.stream_completion_text(request, &cx);
                let mut messages = stream.await?;

                let mut new_summary = String::new();
                while let Some(message) = messages.stream.next().await {
                    let text = message?;
                    let mut lines = text.lines();
                    new_summary.extend(lines.next());

                    // Stop if the LLM generated multiple lines.
                    if lines.next().is_some() {
                        break;
                    }
                }

                this.update(&mut cx, |this, cx| {
                    if !new_summary.is_empty() {
                        this.summary = Some(new_summary.into());
                    }

                    cx.emit(ThreadEvent::SummaryChanged);
                })?;

                anyhow::Ok(())
            }
            .log_err()
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

    /// Cancels the last pending completion, if there are any pending.
    ///
    /// Returns whether a completion was canceled.
    pub fn cancel_last_completion(&mut self) -> bool {
        if let Some(_last_completion) = self.pending_completions.pop() {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub enum ThreadError {
    PaymentRequired,
    MaxMonthlySpendReached,
    Message(SharedString),
}

#[derive(Debug, Clone)]
pub enum ThreadEvent {
    ShowError(ThreadError),
    StreamedCompletion,
    StreamedAssistantText(MessageId, String),
    MessageAdded(MessageId),
    SummaryChanged,
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
