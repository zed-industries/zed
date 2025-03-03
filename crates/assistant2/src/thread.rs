use std::sync::Arc;

use anyhow::Result;
use assistant_tool::ToolWorkingSet;
use chrono::{DateTime, Utc};
use collections::{BTreeMap, HashMap, HashSet};
use futures::StreamExt as _;
use gpui::{App, Context, EventEmitter, SharedString, Task};
use language_model::{
    LanguageModel, LanguageModelCompletionEvent, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelRequestTool, LanguageModelToolResult,
    LanguageModelToolUseId, MaxMonthlySpendReachedError, MessageContent, PaymentRequiredError,
    Role, StopReason,
};
use serde::{Deserialize, Serialize};
use util::{post_inc, TryFutureExt as _};
use uuid::Uuid;

use crate::context::{attach_context_to_message, ContextId, ContextSnapshot};
use crate::thread_store::SavedThread;
use crate::tool_use::{PendingToolUse, ToolUse, ToolUseState};

#[derive(Debug, Clone, Copy)]
pub enum RequestKind {
    Chat,
    /// Used when summarizing a thread.
    Summarize,
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
pub struct MessageId(pub(crate) usize);

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
    tool_use: ToolUseState,
}

impl Thread {
    pub fn new(tools: Arc<ToolWorkingSet>, _cx: &mut Context<Self>) -> Self {
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
            tool_use: ToolUseState::new(),
        }
    }

    pub fn from_saved(
        id: ThreadId,
        saved: SavedThread,
        tools: Arc<ToolWorkingSet>,
        _cx: &mut Context<Self>,
    ) -> Self {
        let next_message_id = MessageId(saved.messages.len());
        let tool_use = ToolUseState::from_saved_messages(&saved.messages);

        Self {
            id,
            updated_at: saved.updated_at,
            summary: Some(saved.summary),
            pending_summary: Task::ready(None),
            messages: saved
                .messages
                .into_iter()
                .map(|message| Message {
                    id: message.id,
                    role: message.role,
                    text: message.text,
                })
                .collect(),
            next_message_id,
            context: BTreeMap::default(),
            context_by_message: HashMap::default(),
            completion_count: 0,
            pending_completions: Vec::new(),
            tools,
            tool_use,
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

    pub fn set_summary(&mut self, summary: impl Into<SharedString>, cx: &mut Context<Self>) {
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
        self.tool_use.pending_tool_uses()
    }

    pub fn tool_uses_for_message(&self, id: MessageId) -> Vec<ToolUse> {
        self.tool_use.tool_uses_for_message(id)
    }

    pub fn tool_results_for_message(&self, id: MessageId) -> Vec<&LanguageModelToolResult> {
        self.tool_use.tool_results_for_message(id)
    }

    pub fn message_has_tool_results(&self, message_id: MessageId) -> bool {
        self.tool_use.message_has_tool_results(message_id)
    }

    pub fn insert_user_message(
        &mut self,
        text: impl Into<String>,
        context: Vec<ContextSnapshot>,
        cx: &mut Context<Self>,
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
        cx: &mut Context<Self>,
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

    pub fn send_to_model(
        &mut self,
        model: Arc<dyn LanguageModel>,
        request_kind: RequestKind,
        use_tools: bool,
        cx: &mut Context<Self>,
    ) {
        let mut request = self.to_completion_request(request_kind, cx);

        if use_tools {
            request.tools = self
                .tools()
                .tools(cx)
                .into_iter()
                .map(|tool| LanguageModelRequestTool {
                    name: tool.name(),
                    description: tool.description(),
                    input_schema: tool.input_schema(),
                })
                .collect();
        }

        self.stream_completion(request, model, cx);
    }

    pub fn to_completion_request(
        &self,
        request_kind: RequestKind,
        _cx: &App,
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
            match request_kind {
                RequestKind::Chat => {
                    self.tool_use
                        .attach_tool_results(message.id, &mut request_message);
                }
                RequestKind::Summarize => {
                    // We don't care about tool use during summarization.
                }
            }

            if !message.text.is_empty() {
                request_message
                    .content
                    .push(MessageContent::Text(message.text.clone()));
            }

            match request_kind {
                RequestKind::Chat => {
                    self.tool_use
                        .attach_tool_uses(message.id, &mut request_message);
                }
                RequestKind::Summarize => {
                    // We don't care about tool use during summarization.
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
        cx: &mut Context<Self>,
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
                                    } else {
                                        // If we won't have an Assistant message yet, assume this chunk marks the beginning
                                        // of a new Assistant response.
                                        //
                                        // Importantly: We do *not* want to emit a `StreamedAssistantText` event here, as it
                                        // will result in duplicating the text of the chunk in the rendered Markdown.
                                        thread.insert_message(Role::Assistant, chunk, cx);
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
                                        .tool_use
                                        .request_tool_use(last_assistant_message.id, tool_use);
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

    pub fn summarize(&mut self, cx: &mut Context<Self>) {
        let Some(provider) = LanguageModelRegistry::read_global(cx).active_provider() else {
            return;
        };
        let Some(model) = LanguageModelRegistry::read_global(cx).active_model() else {
            return;
        };

        if !provider.is_authenticated(cx) {
            return;
        }

        let mut request = self.to_completion_request(RequestKind::Summarize, cx);
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
        tool_use_id: LanguageModelToolUseId,
        output: Task<Result<String>>,
        cx: &mut Context<Self>,
    ) {
        let insert_output_task = cx.spawn(|thread, mut cx| {
            let tool_use_id = tool_use_id.clone();
            async move {
                let output = output.await;
                thread
                    .update(&mut cx, |thread, cx| {
                        thread
                            .tool_use
                            .insert_tool_output(tool_use_id.clone(), output);

                        cx.emit(ThreadEvent::ToolFinished { tool_use_id });
                    })
                    .ok();
            }
        });

        self.tool_use
            .run_pending_tool(tool_use_id, insert_output_task);
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
