use std::sync::Arc;

use anyhow::Result;
use assistant_scripting::{
    Script, ScriptEvent, ScriptId, ScriptSession, ScriptTagParser, SCRIPTING_PROMPT,
};
use assistant_tool::ToolWorkingSet;
use chrono::{DateTime, Utc};
use collections::{BTreeMap, HashMap, HashSet};
use futures::StreamExt as _;
use gpui::{App, AppContext, Context, Entity, EventEmitter, SharedString, Subscription, Task};
use language_model::{
    LanguageModel, LanguageModelCompletionEvent, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelRequestTool, LanguageModelToolResult,
    LanguageModelToolUseId, MaxMonthlySpendReachedError, MessageContent, PaymentRequiredError,
    Role, StopReason,
};
use project::Project;
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
    project: Entity<Project>,
    tools: Arc<ToolWorkingSet>,
    tool_use: ToolUseState,
    scripts_by_assistant_message: HashMap<MessageId, ScriptId>,
    script_output_messages: HashSet<MessageId>,
    script_session: Entity<ScriptSession>,
    _script_session_subscription: Subscription,
}

impl Thread {
    pub fn new(
        project: Entity<Project>,
        tools: Arc<ToolWorkingSet>,
        cx: &mut Context<Self>,
    ) -> Self {
        let script_session = cx.new(|cx| ScriptSession::new(project.clone(), cx));
        let script_session_subscription = cx.subscribe(&script_session, Self::handle_script_event);

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
            project,
            tools,
            tool_use: ToolUseState::new(),
            scripts_by_assistant_message: HashMap::default(),
            script_output_messages: HashSet::default(),
            script_session,
            _script_session_subscription: script_session_subscription,
        }
    }

    pub fn from_saved(
        id: ThreadId,
        saved: SavedThread,
        project: Entity<Project>,
        tools: Arc<ToolWorkingSet>,
        cx: &mut Context<Self>,
    ) -> Self {
        let next_message_id = MessageId(
            saved
                .messages
                .last()
                .map(|message| message.id.0 + 1)
                .unwrap_or(0),
        );
        let tool_use = ToolUseState::from_saved_messages(&saved.messages);
        let script_session = cx.new(|cx| ScriptSession::new(project.clone(), cx));
        let script_session_subscription = cx.subscribe(&script_session, Self::handle_script_event);

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
            project,
            tools,
            tool_use,
            scripts_by_assistant_message: HashMap::default(),
            script_output_messages: HashSet::default(),
            script_session,
            _script_session_subscription: script_session_subscription,
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

    /// Returns whether all of the tool uses have finished running.
    pub fn all_tools_finished(&self) -> bool {
        // If the only pending tool uses left are the ones with errors, then that means that we've finished running all
        // of the pending tools.
        self.pending_tool_uses()
            .into_iter()
            .all(|tool_use| tool_use.status.is_error())
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

    pub fn message_has_script_output(&self, message_id: MessageId) -> bool {
        self.script_output_messages.contains(&message_id)
    }

    pub fn insert_user_message(
        &mut self,
        text: impl Into<String>,
        context: Vec<ContextSnapshot>,
        cx: &mut Context<Self>,
    ) -> MessageId {
        let message_id = self.insert_message(Role::User, text, cx);
        let context_ids = context.iter().map(|context| context.id).collect::<Vec<_>>();
        self.context
            .extend(context.into_iter().map(|context| (context.id, context)));
        self.context_by_message.insert(message_id, context_ids);
        message_id
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

    pub fn edit_message(
        &mut self,
        id: MessageId,
        new_role: Role,
        new_text: String,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(message) = self.messages.iter_mut().find(|message| message.id == id) else {
            return false;
        };
        message.role = new_role;
        message.text = new_text;
        self.touch_updated_at();
        cx.emit(ThreadEvent::MessageEdited(id));
        true
    }

    pub fn delete_message(&mut self, id: MessageId, cx: &mut Context<Self>) -> bool {
        let Some(index) = self.messages.iter().position(|message| message.id == id) else {
            return false;
        };
        self.messages.remove(index);
        self.context_by_message.remove(&id);
        self.touch_updated_at();
        cx.emit(ThreadEvent::MessageDeleted(id));
        true
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

    pub fn script_for_message<'a>(
        &'a self,
        message_id: MessageId,
        cx: &'a App,
    ) -> Option<&'a Script> {
        self.scripts_by_assistant_message
            .get(&message_id)
            .map(|script_id| self.script_session.read(cx).get(*script_id))
    }

    fn handle_script_event(
        &mut self,
        _script_session: Entity<ScriptSession>,
        event: &ScriptEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            ScriptEvent::Spawned(_) => {}
            ScriptEvent::Exited(script_id) => {
                if let Some(output_message) = self
                    .script_session
                    .read(cx)
                    .get(*script_id)
                    .output_message_for_llm()
                {
                    let message_id = self.insert_user_message(output_message, vec![], cx);
                    self.script_output_messages.insert(message_id);
                    cx.emit(ThreadEvent::ScriptFinished)
                }
            }
        }
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
        cx: &App,
    ) -> LanguageModelRequest {
        let mut request = LanguageModelRequest {
            messages: vec![],
            tools: Vec::new(),
            stop: Vec::new(),
            temperature: None,
        };

        request.messages.push(LanguageModelRequestMessage {
            role: Role::System,
            content: vec![SCRIPTING_PROMPT.to_string().into()],
            cache: true,
        });

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

                    if matches!(message.role, Role::Assistant) {
                        if let Some(script_id) = self.scripts_by_assistant_message.get(&message.id)
                        {
                            let script = self.script_session.read(cx).get(*script_id);

                            request_message.content.push(script.source_tag().into());
                        }
                    }
                }
                RequestKind::Summarize => {
                    // We don't care about tool use during summarization.
                }
            };

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
                let mut script_tag_parser = ScriptTagParser::new();
                let mut script_id = None;

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
                                    let chunk = script_tag_parser.parse_chunk(&chunk);

                                    let message_id = if last_message.role == Role::Assistant {
                                        last_message.text.push_str(&chunk.content);
                                        cx.emit(ThreadEvent::StreamedAssistantText(
                                            last_message.id,
                                            chunk.content,
                                        ));
                                        last_message.id
                                    } else {
                                        // If we won't have an Assistant message yet, assume this chunk marks the beginning
                                        // of a new Assistant response.
                                        //
                                        // Importantly: We do *not* want to emit a `StreamedAssistantText` event here, as it
                                        // will result in duplicating the text of the chunk in the rendered Markdown.
                                        thread.insert_message(Role::Assistant, chunk.content, cx)
                                    };

                                    if script_id.is_none() && script_tag_parser.found_script() {
                                        let id = thread
                                            .script_session
                                            .update(cx, |session, _cx| session.new_script());
                                        thread.scripts_by_assistant_message.insert(message_id, id);

                                        script_id = Some(id);
                                    }

                                    if let (Some(script_source), Some(script_id)) =
                                        (chunk.script_source, script_id)
                                    {
                                        // TODO: move buffer to script and run as it streams
                                        thread
                                            .script_session
                                            .update(cx, |this, cx| {
                                                this.run_script(script_id, script_source, cx)
                                            })
                                            .detach_and_log_err(cx);
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

    pub fn use_pending_tools(&mut self, cx: &mut Context<Self>) {
        let pending_tool_uses = self
            .pending_tool_uses()
            .into_iter()
            .filter(|tool_use| tool_use.status.is_idle())
            .cloned()
            .collect::<Vec<_>>();

        for tool_use in pending_tool_uses {
            if let Some(tool) = self.tools.tool(&tool_use.name, cx) {
                let task = tool.run(tool_use.input, self.project.clone(), cx);

                self.insert_tool_output(tool_use.id.clone(), task, cx);
            }
        }
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

    pub fn send_tool_results_to_model(
        &mut self,
        model: Arc<dyn LanguageModel>,
        cx: &mut Context<Self>,
    ) {
        // Insert a user message to contain the tool results.
        self.insert_user_message(
            // TODO: Sending up a user message without any content results in the model sending back
            // responses that also don't have any content. We currently don't handle this case well,
            // so for now we provide some text to keep the model on track.
            "Here are the tool results.",
            Vec::new(),
            cx,
        );
        self.send_to_model(model, RequestKind::Chat, true, cx);
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
    MessageEdited(MessageId),
    MessageDeleted(MessageId),
    SummaryChanged,
    UsePendingTools,
    ToolFinished {
        #[allow(unused)]
        tool_use_id: LanguageModelToolUseId,
    },
    ScriptFinished,
}

impl EventEmitter<ThreadEvent> for Thread {}

struct PendingCompletion {
    id: usize,
    _task: Task<()>,
}
