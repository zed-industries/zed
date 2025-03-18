use std::fmt::Write as _;
use std::io::Write;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use assistant_tool::{ActionLog, ToolWorkingSet};
use chrono::{DateTime, Utc};
use collections::{BTreeMap, HashMap, HashSet};
use futures::future::Shared;
use futures::{FutureExt, StreamExt as _};
use git;
use gpui::{App, AppContext, Context, Entity, EventEmitter, SharedString, Task};
use language_model::{
    LanguageModel, LanguageModelCompletionEvent, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelRequestTool, LanguageModelToolResult,
    LanguageModelToolUseId, MaxMonthlySpendReachedError, MessageContent, PaymentRequiredError,
    Role, StopReason, TokenUsage,
};
use project::Project;
use prompt_store::{AssistantSystemPromptWorktree, PromptBuilder};
use scripting_tool::{ScriptingSession, ScriptingTool};
use serde::{Deserialize, Serialize};
use util::{post_inc, ResultExt, TryFutureExt as _};
use uuid::Uuid;

use crate::context::{attach_context_to_message, ContextId, ContextSnapshot};
use crate::thread_store::{
    SerializedMessage, SerializedThread, SerializedToolResult, SerializedToolUse,
};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSnapshot {
    pub worktree_snapshots: Vec<WorktreeSnapshot>,
    pub unsaved_buffer_paths: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorktreeSnapshot {
    pub worktree_path: String,
    pub git_state: Option<GitState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitState {
    pub remote_url: Option<String>,
    pub head_sha: Option<String>,
    pub current_branch: Option<String>,
    pub diff: Option<String>,
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
    prompt_builder: Arc<PromptBuilder>,
    tools: Arc<ToolWorkingSet>,
    tool_use: ToolUseState,
    action_log: Entity<ActionLog>,
    scripting_session: Entity<ScriptingSession>,
    scripting_tool_use: ToolUseState,
    initial_project_snapshot: Shared<Task<Option<Arc<ProjectSnapshot>>>>,
    cumulative_token_usage: TokenUsage,
}

impl Thread {
    pub fn new(
        project: Entity<Project>,
        tools: Arc<ToolWorkingSet>,
        prompt_builder: Arc<PromptBuilder>,
        cx: &mut Context<Self>,
    ) -> Self {
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
            project: project.clone(),
            prompt_builder,
            tools,
            tool_use: ToolUseState::new(),
            scripting_session: cx.new(|cx| ScriptingSession::new(project.clone(), cx)),
            scripting_tool_use: ToolUseState::new(),
            action_log: cx.new(|_| ActionLog::new()),
            initial_project_snapshot: {
                let project_snapshot = Self::project_snapshot(project, cx);
                cx.foreground_executor()
                    .spawn(async move { Some(project_snapshot.await) })
                    .shared()
            },
            cumulative_token_usage: TokenUsage::default(),
        }
    }

    pub fn deserialize(
        id: ThreadId,
        serialized: SerializedThread,
        project: Entity<Project>,
        tools: Arc<ToolWorkingSet>,
        prompt_builder: Arc<PromptBuilder>,
        cx: &mut Context<Self>,
    ) -> Self {
        let next_message_id = MessageId(
            serialized
                .messages
                .last()
                .map(|message| message.id.0 + 1)
                .unwrap_or(0),
        );
        let tool_use = ToolUseState::from_serialized_messages(&serialized.messages, |name| {
            name != ScriptingTool::NAME
        });
        let scripting_tool_use =
            ToolUseState::from_serialized_messages(&serialized.messages, |name| {
                name == ScriptingTool::NAME
            });
        let scripting_session = cx.new(|cx| ScriptingSession::new(project.clone(), cx));

        Self {
            id,
            updated_at: serialized.updated_at,
            summary: Some(serialized.summary),
            pending_summary: Task::ready(None),
            messages: serialized
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
            prompt_builder,
            tools,
            tool_use,
            action_log: cx.new(|_| ActionLog::new()),
            scripting_session,
            scripting_tool_use,
            initial_project_snapshot: Task::ready(serialized.initial_project_snapshot).shared(),
            // TODO: persist token usage?
            cumulative_token_usage: TokenUsage::default(),
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

    pub fn is_generating(&self) -> bool {
        !self.pending_completions.is_empty() || !self.all_tools_finished()
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

    /// Returns whether all of the tool uses have finished running.
    pub fn all_tools_finished(&self) -> bool {
        let mut all_pending_tool_uses = self
            .tool_use
            .pending_tool_uses()
            .into_iter()
            .chain(self.scripting_tool_use.pending_tool_uses());

        // If the only pending tool uses left are the ones with errors, then
        // that means that we've finished running all of the pending tools.
        all_pending_tool_uses.all(|tool_use| tool_use.status.is_error())
    }

    pub fn tool_uses_for_message(&self, id: MessageId) -> Vec<ToolUse> {
        self.tool_use.tool_uses_for_message(id)
    }

    pub fn scripting_tool_uses_for_message(&self, id: MessageId) -> Vec<ToolUse> {
        self.scripting_tool_use.tool_uses_for_message(id)
    }

    pub fn tool_results_for_message(&self, id: MessageId) -> Vec<&LanguageModelToolResult> {
        self.tool_use.tool_results_for_message(id)
    }

    pub fn tool_result(&self, id: &LanguageModelToolUseId) -> Option<&LanguageModelToolResult> {
        self.tool_use.tool_result(id)
    }

    pub fn scripting_tool_results_for_message(
        &self,
        id: MessageId,
    ) -> Vec<&LanguageModelToolResult> {
        self.scripting_tool_use.tool_results_for_message(id)
    }

    pub fn scripting_changed_buffers<'a>(
        &self,
        cx: &'a App,
    ) -> impl ExactSizeIterator<Item = &'a Entity<language::Buffer>> {
        self.scripting_session.read(cx).changed_buffers()
    }

    pub fn message_has_tool_results(&self, message_id: MessageId) -> bool {
        self.tool_use.message_has_tool_results(message_id)
    }

    pub fn message_has_scripting_tool_results(&self, message_id: MessageId) -> bool {
        self.scripting_tool_use.message_has_tool_results(message_id)
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

    /// Serializes this thread into a format for storage or telemetry.
    pub fn serialize(&self, cx: &mut Context<Self>) -> Task<Result<SerializedThread>> {
        let initial_project_snapshot = self.initial_project_snapshot.clone();
        cx.spawn(|this, cx| async move {
            let initial_project_snapshot = initial_project_snapshot.await;
            this.read_with(&cx, |this, _| SerializedThread {
                summary: this.summary_or_default(),
                updated_at: this.updated_at(),
                messages: this
                    .messages()
                    .map(|message| SerializedMessage {
                        id: message.id,
                        role: message.role,
                        text: message.text.clone(),
                        tool_uses: this
                            .tool_uses_for_message(message.id)
                            .into_iter()
                            .chain(this.scripting_tool_uses_for_message(message.id))
                            .map(|tool_use| SerializedToolUse {
                                id: tool_use.id,
                                name: tool_use.name,
                                input: tool_use.input,
                            })
                            .collect(),
                        tool_results: this
                            .tool_results_for_message(message.id)
                            .into_iter()
                            .chain(this.scripting_tool_results_for_message(message.id))
                            .map(|tool_result| SerializedToolResult {
                                tool_use_id: tool_result.tool_use_id.clone(),
                                is_error: tool_result.is_error,
                                content: tool_result.content.clone(),
                            })
                            .collect(),
                    })
                    .collect(),
                initial_project_snapshot,
            })
        })
    }

    pub fn send_to_model(
        &mut self,
        model: Arc<dyn LanguageModel>,
        request_kind: RequestKind,
        cx: &mut Context<Self>,
    ) {
        let mut request = self.to_completion_request(request_kind, cx);
        request.tools = {
            let mut tools = Vec::new();

            if self.tools.is_scripting_tool_enabled() {
                tools.push(LanguageModelRequestTool {
                    name: ScriptingTool::NAME.into(),
                    description: ScriptingTool::DESCRIPTION.into(),
                    input_schema: ScriptingTool::input_schema(),
                });
            }

            tools.extend(self.tools().enabled_tools(cx).into_iter().map(|tool| {
                LanguageModelRequestTool {
                    name: tool.name(),
                    description: tool.description(),
                    input_schema: tool.input_schema(),
                }
            }));

            tools
        };

        self.stream_completion(request, model, cx);
    }

    pub fn to_completion_request(
        &self,
        request_kind: RequestKind,
        cx: &App,
    ) -> LanguageModelRequest {
        let worktree_root_names = self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .map(|worktree| {
                let worktree = worktree.read(cx);
                AssistantSystemPromptWorktree {
                    root_name: worktree.root_name().into(),
                    abs_path: worktree.abs_path(),
                }
            })
            .collect::<Vec<_>>();
        let system_prompt = self
            .prompt_builder
            .generate_assistant_system_prompt(worktree_root_names)
            .context("failed to generate assistant system prompt")
            .log_err()
            .unwrap_or_default();

        let mut request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::System,
                content: vec![MessageContent::Text(system_prompt)],
                cache: true,
            }],
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
                    self.scripting_tool_use
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
                    self.scripting_tool_use
                        .attach_tool_uses(message.id, &mut request_message);
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

        self.attach_stale_files(&mut request.messages, cx);

        request
    }

    fn attach_stale_files(&self, messages: &mut Vec<LanguageModelRequestMessage>, cx: &App) {
        const STALE_FILES_HEADER: &str = "These files changed since last read:";

        let mut stale_message = String::new();

        for stale_file in self.action_log.read(cx).stale_buffers(cx) {
            let Some(file) = stale_file.read(cx).file() else {
                continue;
            };

            if stale_message.is_empty() {
                write!(&mut stale_message, "{}", STALE_FILES_HEADER).ok();
            }

            writeln!(&mut stale_message, "- {}", file.path().display()).ok();
        }

        if !stale_message.is_empty() {
            let context_message = LanguageModelRequestMessage {
                role: Role::User,
                content: vec![stale_message.into()],
                cache: false,
            };

            messages.push(context_message);
        }
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
                let mut current_token_usage = TokenUsage::default();

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
                            LanguageModelCompletionEvent::UsageUpdate(token_usage) => {
                                thread.cumulative_token_usage =
                                    thread.cumulative_token_usage.clone() + token_usage.clone()
                                        - current_token_usage.clone();
                                current_token_usage = token_usage;
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
                                    };
                                }
                            }
                            LanguageModelCompletionEvent::ToolUse(tool_use) => {
                                if let Some(last_assistant_message) = thread
                                    .messages
                                    .iter()
                                    .rfind(|message| message.role == Role::Assistant)
                                {
                                    if tool_use.name.as_ref() == ScriptingTool::NAME {
                                        thread
                                            .scripting_tool_use
                                            .request_tool_use(last_assistant_message.id, tool_use);
                                    } else {
                                        thread
                                            .tool_use
                                            .request_tool_use(last_assistant_message.id, tool_use);
                                    }
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
                .update(&mut cx, |thread, cx| {
                    match result.as_ref() {
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
                                cx.emit(ThreadEvent::ShowError(
                                    ThreadError::MaxMonthlySpendReached,
                                ));
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

                            thread.cancel_last_completion(cx);
                        }
                    }
                    cx.emit(ThreadEvent::DoneStreaming);
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
        let request = self.to_completion_request(RequestKind::Chat, cx);
        let pending_tool_uses = self
            .tool_use
            .pending_tool_uses()
            .into_iter()
            .filter(|tool_use| tool_use.status.is_idle())
            .cloned()
            .collect::<Vec<_>>();

        for tool_use in pending_tool_uses {
            if let Some(tool) = self.tools.tool(&tool_use.name, cx) {
                let task = tool.run(
                    tool_use.input,
                    &request.messages,
                    self.project.clone(),
                    self.action_log.clone(),
                    cx,
                );

                self.insert_tool_output(tool_use.id.clone(), task, cx);
            }
        }

        let pending_scripting_tool_uses = self
            .scripting_tool_use
            .pending_tool_uses()
            .into_iter()
            .filter(|tool_use| tool_use.status.is_idle())
            .cloned()
            .collect::<Vec<_>>();

        for scripting_tool_use in pending_scripting_tool_uses {
            let task = match ScriptingTool::deserialize_input(scripting_tool_use.input) {
                Err(err) => Task::ready(Err(err.into())),
                Ok(input) => {
                    let (script_id, script_task) =
                        self.scripting_session.update(cx, move |session, cx| {
                            session.run_script(input.lua_script, cx)
                        });

                    let session = self.scripting_session.clone();
                    cx.spawn(|_, cx| async move {
                        script_task.await;

                        let message = session.read_with(&cx, |session, _cx| {
                            // Using a id to get the script output seems impractical.
                            // Why not just include it in the Task result?
                            // This is because we'll later report the script state as it runs,
                            session
                                .get(script_id)
                                .output_message_for_llm()
                                .expect("Script shouldn't still be running")
                        })?;

                        Ok(message)
                    })
                }
            };

            self.insert_scripting_tool_output(scripting_tool_use.id.clone(), task, cx);
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
                        let pending_tool_use = thread
                            .tool_use
                            .insert_tool_output(tool_use_id.clone(), output);

                        cx.emit(ThreadEvent::ToolFinished {
                            tool_use_id,
                            pending_tool_use,
                            canceled: false,
                        });
                    })
                    .ok();
            }
        });

        self.tool_use
            .run_pending_tool(tool_use_id, insert_output_task);
    }

    pub fn insert_scripting_tool_output(
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
                        let pending_tool_use = thread
                            .scripting_tool_use
                            .insert_tool_output(tool_use_id.clone(), output);

                        cx.emit(ThreadEvent::ToolFinished {
                            tool_use_id,
                            pending_tool_use,
                            canceled: false,
                        });
                    })
                    .ok();
            }
        });

        self.scripting_tool_use
            .run_pending_tool(tool_use_id, insert_output_task);
    }

    pub fn attach_tool_results(
        &mut self,
        updated_context: Vec<ContextSnapshot>,
        cx: &mut Context<Self>,
    ) {
        self.context.extend(
            updated_context
                .into_iter()
                .map(|context| (context.id, context)),
        );

        // Insert a user message to contain the tool results.
        self.insert_user_message(
            // TODO: Sending up a user message without any content results in the model sending back
            // responses that also don't have any content. We currently don't handle this case well,
            // so for now we provide some text to keep the model on track.
            "Here are the tool results.",
            Vec::new(),
            cx,
        );
    }

    /// Cancels the last pending completion, if there are any pending.
    ///
    /// Returns whether a completion was canceled.
    pub fn cancel_last_completion(&mut self, cx: &mut Context<Self>) -> bool {
        if self.pending_completions.pop().is_some() {
            true
        } else {
            let mut canceled = false;
            for pending_tool_use in self.tool_use.cancel_pending() {
                canceled = true;
                cx.emit(ThreadEvent::ToolFinished {
                    tool_use_id: pending_tool_use.id.clone(),
                    pending_tool_use: Some(pending_tool_use),
                    canceled: true,
                });
            }
            canceled
        }
    }

    /// Reports feedback about the thread and stores it in our telemetry backend.
    pub fn report_feedback(&self, is_positive: bool, cx: &mut Context<Self>) -> Task<Result<()>> {
        let final_project_snapshot = Self::project_snapshot(self.project.clone(), cx);
        let serialized_thread = self.serialize(cx);
        let thread_id = self.id().clone();
        let client = self.project.read(cx).client();

        cx.background_spawn(async move {
            let final_project_snapshot = final_project_snapshot.await;
            let serialized_thread = serialized_thread.await?;
            let thread_data =
                serde_json::to_value(serialized_thread).unwrap_or_else(|_| serde_json::Value::Null);

            let rating = if is_positive { "positive" } else { "negative" };
            telemetry::event!(
                "Assistant Thread Rated",
                rating,
                thread_id,
                thread_data,
                final_project_snapshot
            );
            client.telemetry().flush_events();

            Ok(())
        })
    }

    /// Create a snapshot of the current project state including git information and unsaved buffers.
    fn project_snapshot(
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Task<Arc<ProjectSnapshot>> {
        let worktree_snapshots: Vec<_> = project
            .read(cx)
            .visible_worktrees(cx)
            .map(|worktree| Self::worktree_snapshot(worktree, cx))
            .collect();

        cx.spawn(move |_, cx| async move {
            let worktree_snapshots = futures::future::join_all(worktree_snapshots).await;

            let mut unsaved_buffers = Vec::new();
            cx.update(|app_cx| {
                let buffer_store = project.read(app_cx).buffer_store();
                for buffer_handle in buffer_store.read(app_cx).buffers() {
                    let buffer = buffer_handle.read(app_cx);
                    if buffer.is_dirty() {
                        if let Some(file) = buffer.file() {
                            let path = file.path().to_string_lossy().to_string();
                            unsaved_buffers.push(path);
                        }
                    }
                }
            })
            .ok();

            Arc::new(ProjectSnapshot {
                worktree_snapshots,
                unsaved_buffer_paths: unsaved_buffers,
                timestamp: Utc::now(),
            })
        })
    }

    fn worktree_snapshot(worktree: Entity<project::Worktree>, cx: &App) -> Task<WorktreeSnapshot> {
        cx.spawn(move |cx| async move {
            // Get worktree path and snapshot
            let worktree_info = cx.update(|app_cx| {
                let worktree = worktree.read(app_cx);
                let path = worktree.abs_path().to_string_lossy().to_string();
                let snapshot = worktree.snapshot();
                (path, snapshot)
            });

            let Ok((worktree_path, snapshot)) = worktree_info else {
                return WorktreeSnapshot {
                    worktree_path: String::new(),
                    git_state: None,
                };
            };

            // Extract git information
            let git_state = match snapshot.repositories().first() {
                None => None,
                Some(repo_entry) => {
                    // Get branch information
                    let current_branch = repo_entry.branch().map(|branch| branch.name.to_string());

                    // Get repository info
                    let repo_result = worktree.read_with(&cx, |worktree, _cx| {
                        if let project::Worktree::Local(local_worktree) = &worktree {
                            local_worktree.get_local_repo(repo_entry).map(|local_repo| {
                                let repo = local_repo.repo();
                                (repo.remote_url("origin"), repo.head_sha(), repo.clone())
                            })
                        } else {
                            None
                        }
                    });

                    match repo_result {
                        Ok(Some((remote_url, head_sha, repository))) => {
                            // Get diff asynchronously
                            let diff = repository
                                .diff(git::repository::DiffType::HeadToWorktree, cx)
                                .await
                                .ok();

                            Some(GitState {
                                remote_url,
                                head_sha,
                                current_branch,
                                diff,
                            })
                        }
                        Err(_) | Ok(None) => None,
                    }
                }
            };

            WorktreeSnapshot {
                worktree_path,
                git_state,
            }
        })
    }

    pub fn to_markdown(&self) -> Result<String> {
        let mut markdown = Vec::new();

        if let Some(summary) = self.summary() {
            writeln!(markdown, "# {summary}\n")?;
        };

        for message in self.messages() {
            writeln!(
                markdown,
                "## {role}\n",
                role = match message.role {
                    Role::User => "User",
                    Role::Assistant => "Assistant",
                    Role::System => "System",
                }
            )?;
            writeln!(markdown, "{}\n", message.text)?;

            for tool_use in self.tool_uses_for_message(message.id) {
                writeln!(
                    markdown,
                    "**Use Tool: {} ({})**",
                    tool_use.name, tool_use.id
                )?;
                writeln!(markdown, "```json")?;
                writeln!(
                    markdown,
                    "{}",
                    serde_json::to_string_pretty(&tool_use.input)?
                )?;
                writeln!(markdown, "```")?;
            }

            for tool_result in self.tool_results_for_message(message.id) {
                write!(markdown, "**Tool Results: {}", tool_result.tool_use_id)?;
                if tool_result.is_error {
                    write!(markdown, " (Error)")?;
                }

                writeln!(markdown, "**\n")?;
                writeln!(markdown, "{}", tool_result.content)?;
            }
        }

        Ok(String::from_utf8_lossy(&markdown).to_string())
    }

    pub fn action_log(&self) -> &Entity<ActionLog> {
        &self.action_log
    }

    pub fn cumulative_token_usage(&self) -> TokenUsage {
        self.cumulative_token_usage.clone()
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
    DoneStreaming,
    MessageAdded(MessageId),
    MessageEdited(MessageId),
    MessageDeleted(MessageId),
    SummaryChanged,
    UsePendingTools,
    ToolFinished {
        #[allow(unused)]
        tool_use_id: LanguageModelToolUseId,
        /// The pending tool use that corresponds to this tool.
        pending_tool_use: Option<PendingToolUse>,
        /// Whether the tool was canceled by the user.
        canceled: bool,
    },
}

impl EventEmitter<ThreadEvent> for Thread {}

struct PendingCompletion {
    id: usize,
    _task: Task<()>,
}
