use std::fmt::Write as _;
use std::io::Write;
use std::ops::Range;
use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow};
use assistant_settings::AssistantSettings;
use assistant_tool::{ActionLog, Tool, ToolWorkingSet};
use chrono::{DateTime, Utc};
use collections::{BTreeMap, HashMap, HashSet};
use fs::Fs;
use futures::future::Shared;
use futures::{FutureExt, StreamExt as _};
use git::repository::DiffType;
use gpui::{App, AppContext, Context, Entity, EventEmitter, SharedString, Task, WeakEntity};
use language_model::{
    LanguageModel, LanguageModelCompletionEvent, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelRequestTool, LanguageModelToolResult,
    LanguageModelToolUseId, MaxMonthlySpendReachedError, MessageContent, PaymentRequiredError,
    Role, StopReason, TokenUsage,
};
use project::git_store::{GitStore, GitStoreCheckpoint, RepositoryState};
use project::{Project, Worktree};
use prompt_store::{
    AssistantSystemPromptContext, PromptBuilder, RulesFile, WorktreeInfoForSystemPrompt,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use util::{ResultExt as _, TryFutureExt as _, maybe, post_inc};
use uuid::Uuid;

use crate::context::{AssistantContext, ContextId, attach_context_to_message};
use crate::thread_store::{
    SerializedMessage, SerializedMessageSegment, SerializedThread, SerializedToolResult,
    SerializedToolUse,
};
use crate::tool_use::{PendingToolUse, ToolUse, ToolUseState};

#[derive(Debug, Clone, Copy)]
pub enum RequestKind {
    Chat,
    /// Used when summarizing a thread.
    Summarize,
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize, JsonSchema,
)]
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

impl From<&str> for ThreadId {
    fn from(value: &str) -> Self {
        Self(value.into())
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
    pub segments: Vec<MessageSegment>,
}

impl Message {
    pub fn push_thinking(&mut self, text: &str) {
        if let Some(MessageSegment::Thinking(segment)) = self.segments.last_mut() {
            segment.push_str(text);
        } else {
            self.segments
                .push(MessageSegment::Thinking(text.to_string()));
        }
    }

    pub fn push_text(&mut self, text: &str) {
        if let Some(MessageSegment::Text(segment)) = self.segments.last_mut() {
            segment.push_str(text);
        } else {
            self.segments.push(MessageSegment::Text(text.to_string()));
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for segment in &self.segments {
            match segment {
                MessageSegment::Text(text) => result.push_str(text),
                MessageSegment::Thinking(text) => {
                    result.push_str("<think>");
                    result.push_str(text);
                    result.push_str("</think>");
                }
            }
        }
        result
    }
}

#[derive(Debug, Clone)]
pub enum MessageSegment {
    Text(String),
    Thinking(String),
}

impl MessageSegment {
    pub fn text_mut(&mut self) -> &mut String {
        match self {
            Self::Text(text) => text,
            Self::Thinking(text) => text,
        }
    }
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

#[derive(Clone)]
pub struct ThreadCheckpoint {
    message_id: MessageId,
    git_checkpoint: GitStoreCheckpoint,
}

#[derive(Copy, Clone, Debug)]
pub enum ThreadFeedback {
    Positive,
    Negative,
}

pub enum LastRestoreCheckpoint {
    Pending {
        message_id: MessageId,
    },
    Error {
        message_id: MessageId,
        error: String,
    },
}

impl LastRestoreCheckpoint {
    pub fn message_id(&self) -> MessageId {
        match self {
            LastRestoreCheckpoint::Pending { message_id } => *message_id,
            LastRestoreCheckpoint::Error { message_id, .. } => *message_id,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum DetailedSummaryState {
    #[default]
    NotGenerated,
    Generating {
        message_id: MessageId,
    },
    Generated {
        text: SharedString,
        message_id: MessageId,
    },
}

/// A thread of conversation with the LLM.
pub struct Thread {
    id: ThreadId,
    updated_at: DateTime<Utc>,
    summary: Option<SharedString>,
    pending_summary: Task<Option<()>>,
    detailed_summary_state: DetailedSummaryState,
    messages: Vec<Message>,
    next_message_id: MessageId,
    context: BTreeMap<ContextId, AssistantContext>,
    context_by_message: HashMap<MessageId, Vec<ContextId>>,
    system_prompt_context: Option<AssistantSystemPromptContext>,
    checkpoints_by_message: HashMap<MessageId, ThreadCheckpoint>,
    completion_count: usize,
    pending_completions: Vec<PendingCompletion>,
    project: Entity<Project>,
    prompt_builder: Arc<PromptBuilder>,
    tools: Arc<ToolWorkingSet>,
    tool_use: ToolUseState,
    action_log: Entity<ActionLog>,
    last_restore_checkpoint: Option<LastRestoreCheckpoint>,
    pending_checkpoint: Option<ThreadCheckpoint>,
    initial_project_snapshot: Shared<Task<Option<Arc<ProjectSnapshot>>>>,
    cumulative_token_usage: TokenUsage,
    feedback: Option<ThreadFeedback>,
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
            detailed_summary_state: DetailedSummaryState::NotGenerated,
            messages: Vec::new(),
            next_message_id: MessageId(0),
            context: BTreeMap::default(),
            context_by_message: HashMap::default(),
            system_prompt_context: None,
            checkpoints_by_message: HashMap::default(),
            completion_count: 0,
            pending_completions: Vec::new(),
            project: project.clone(),
            prompt_builder,
            tools: tools.clone(),
            last_restore_checkpoint: None,
            pending_checkpoint: None,
            tool_use: ToolUseState::new(tools.clone()),
            action_log: cx.new(|_| ActionLog::new()),
            initial_project_snapshot: {
                let project_snapshot = Self::project_snapshot(project, cx);
                cx.foreground_executor()
                    .spawn(async move { Some(project_snapshot.await) })
                    .shared()
            },
            cumulative_token_usage: TokenUsage::default(),
            feedback: None,
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
        let tool_use =
            ToolUseState::from_serialized_messages(tools.clone(), &serialized.messages, |_| true);

        Self {
            id,
            updated_at: serialized.updated_at,
            summary: Some(serialized.summary),
            pending_summary: Task::ready(None),
            detailed_summary_state: serialized.detailed_summary_state,
            messages: serialized
                .messages
                .into_iter()
                .map(|message| Message {
                    id: message.id,
                    role: message.role,
                    segments: message
                        .segments
                        .into_iter()
                        .map(|segment| match segment {
                            SerializedMessageSegment::Text { text } => MessageSegment::Text(text),
                            SerializedMessageSegment::Thinking { text } => {
                                MessageSegment::Thinking(text)
                            }
                        })
                        .collect(),
                })
                .collect(),
            next_message_id,
            context: BTreeMap::default(),
            context_by_message: HashMap::default(),
            system_prompt_context: None,
            checkpoints_by_message: HashMap::default(),
            completion_count: 0,
            pending_completions: Vec::new(),
            last_restore_checkpoint: None,
            pending_checkpoint: None,
            project,
            prompt_builder,
            tools,
            tool_use,
            action_log: cx.new(|_| ActionLog::new()),
            initial_project_snapshot: Task::ready(serialized.initial_project_snapshot).shared(),
            cumulative_token_usage: serialized.cumulative_token_usage,
            feedback: None,
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

    pub fn latest_detailed_summary_or_text(&self) -> SharedString {
        self.latest_detailed_summary()
            .unwrap_or_else(|| self.text().into())
    }

    fn latest_detailed_summary(&self) -> Option<SharedString> {
        if let DetailedSummaryState::Generated { text, .. } = &self.detailed_summary_state {
            Some(text.clone())
        } else {
            None
        }
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

    pub fn pending_tool(&self, id: &LanguageModelToolUseId) -> Option<&PendingToolUse> {
        self.tool_use
            .pending_tool_uses()
            .into_iter()
            .find(|tool_use| &tool_use.id == id)
    }

    pub fn tools_needing_confirmation(&self) -> impl Iterator<Item = &PendingToolUse> {
        self.tool_use
            .pending_tool_uses()
            .into_iter()
            .filter(|tool_use| tool_use.status.needs_confirmation())
    }

    pub fn has_pending_tool_uses(&self) -> bool {
        !self.tool_use.pending_tool_uses().is_empty()
    }

    pub fn checkpoint_for_message(&self, id: MessageId) -> Option<ThreadCheckpoint> {
        self.checkpoints_by_message.get(&id).cloned()
    }

    pub fn restore_checkpoint(
        &mut self,
        checkpoint: ThreadCheckpoint,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.last_restore_checkpoint = Some(LastRestoreCheckpoint::Pending {
            message_id: checkpoint.message_id,
        });
        cx.emit(ThreadEvent::CheckpointChanged);
        cx.notify();

        let project = self.project.read(cx);
        let restore = project
            .git_store()
            .read(cx)
            .restore_checkpoint(checkpoint.git_checkpoint.clone(), cx);
        cx.spawn(async move |this, cx| {
            let result = restore.await;
            this.update(cx, |this, cx| {
                if let Err(err) = result.as_ref() {
                    this.last_restore_checkpoint = Some(LastRestoreCheckpoint::Error {
                        message_id: checkpoint.message_id,
                        error: err.to_string(),
                    });
                } else {
                    this.truncate(checkpoint.message_id, cx);
                    this.last_restore_checkpoint = None;
                }
                this.pending_checkpoint = None;
                cx.emit(ThreadEvent::CheckpointChanged);
                cx.notify();
            })?;
            result
        })
    }

    fn finalize_pending_checkpoint(&mut self, cx: &mut Context<Self>) {
        let pending_checkpoint = if self.is_generating() {
            return;
        } else if let Some(checkpoint) = self.pending_checkpoint.take() {
            checkpoint
        } else {
            return;
        };

        let git_store = self.project.read(cx).git_store().clone();
        let final_checkpoint = git_store.read(cx).checkpoint(cx);
        cx.spawn(async move |this, cx| match final_checkpoint.await {
            Ok(final_checkpoint) => {
                let equal = git_store
                    .read_with(cx, |store, cx| {
                        store.compare_checkpoints(
                            pending_checkpoint.git_checkpoint.clone(),
                            final_checkpoint.clone(),
                            cx,
                        )
                    })?
                    .await
                    .unwrap_or(false);

                if equal {
                    git_store
                        .read_with(cx, |store, cx| {
                            store.delete_checkpoint(pending_checkpoint.git_checkpoint, cx)
                        })?
                        .detach();
                } else {
                    this.update(cx, |this, cx| {
                        this.insert_checkpoint(pending_checkpoint, cx)
                    })?;
                }

                git_store
                    .read_with(cx, |store, cx| {
                        store.delete_checkpoint(final_checkpoint, cx)
                    })?
                    .detach();

                Ok(())
            }
            Err(_) => this.update(cx, |this, cx| {
                this.insert_checkpoint(pending_checkpoint, cx)
            }),
        })
        .detach();
    }

    fn insert_checkpoint(&mut self, checkpoint: ThreadCheckpoint, cx: &mut Context<Self>) {
        self.checkpoints_by_message
            .insert(checkpoint.message_id, checkpoint);
        cx.emit(ThreadEvent::CheckpointChanged);
        cx.notify();
    }

    pub fn last_restore_checkpoint(&self) -> Option<&LastRestoreCheckpoint> {
        self.last_restore_checkpoint.as_ref()
    }

    pub fn truncate(&mut self, message_id: MessageId, cx: &mut Context<Self>) {
        let Some(message_ix) = self
            .messages
            .iter()
            .rposition(|message| message.id == message_id)
        else {
            return;
        };
        for deleted_message in self.messages.drain(message_ix..) {
            self.context_by_message.remove(&deleted_message.id);
            self.checkpoints_by_message.remove(&deleted_message.id);
        }
        cx.notify();
    }

    pub fn context_for_message(&self, id: MessageId) -> impl Iterator<Item = &AssistantContext> {
        self.context_by_message
            .get(&id)
            .into_iter()
            .flat_map(|context| {
                context
                    .iter()
                    .filter_map(|context_id| self.context.get(&context_id))
            })
    }

    /// Returns whether all of the tool uses have finished running.
    pub fn all_tools_finished(&self) -> bool {
        // If the only pending tool uses left are the ones with errors, then
        // that means that we've finished running all of the pending tools.
        self.tool_use
            .pending_tool_uses()
            .iter()
            .all(|tool_use| tool_use.status.is_error())
    }

    pub fn tool_uses_for_message(&self, id: MessageId, cx: &App) -> Vec<ToolUse> {
        self.tool_use.tool_uses_for_message(id, cx)
    }

    pub fn tool_results_for_message(&self, id: MessageId) -> Vec<&LanguageModelToolResult> {
        self.tool_use.tool_results_for_message(id)
    }

    pub fn tool_result(&self, id: &LanguageModelToolUseId) -> Option<&LanguageModelToolResult> {
        self.tool_use.tool_result(id)
    }

    pub fn message_has_tool_results(&self, message_id: MessageId) -> bool {
        self.tool_use.message_has_tool_results(message_id)
    }

    pub fn insert_user_message(
        &mut self,
        text: impl Into<String>,
        context: Vec<AssistantContext>,
        git_checkpoint: Option<GitStoreCheckpoint>,
        cx: &mut Context<Self>,
    ) -> MessageId {
        let message_id =
            self.insert_message(Role::User, vec![MessageSegment::Text(text.into())], cx);
        let context_ids = context
            .iter()
            .map(|context| context.id())
            .collect::<Vec<_>>();
        self.context
            .extend(context.into_iter().map(|context| (context.id(), context)));
        self.context_by_message.insert(message_id, context_ids);
        if let Some(git_checkpoint) = git_checkpoint {
            self.pending_checkpoint = Some(ThreadCheckpoint {
                message_id,
                git_checkpoint,
            });
        }
        message_id
    }

    pub fn insert_message(
        &mut self,
        role: Role,
        segments: Vec<MessageSegment>,
        cx: &mut Context<Self>,
    ) -> MessageId {
        let id = self.next_message_id.post_inc();
        self.messages.push(Message { id, role, segments });
        self.touch_updated_at();
        cx.emit(ThreadEvent::MessageAdded(id));
        id
    }

    pub fn edit_message(
        &mut self,
        id: MessageId,
        new_role: Role,
        new_segments: Vec<MessageSegment>,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(message) = self.messages.iter_mut().find(|message| message.id == id) else {
            return false;
        };
        message.role = new_role;
        message.segments = new_segments;
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

            for segment in &message.segments {
                match segment {
                    MessageSegment::Text(content) => text.push_str(content),
                    MessageSegment::Thinking(content) => {
                        text.push_str(&format!("<think>{}</think>", content))
                    }
                }
            }
            text.push('\n');
        }

        text
    }

    /// Serializes this thread into a format for storage or telemetry.
    pub fn serialize(&self, cx: &mut Context<Self>) -> Task<Result<SerializedThread>> {
        let initial_project_snapshot = self.initial_project_snapshot.clone();
        cx.spawn(async move |this, cx| {
            let initial_project_snapshot = initial_project_snapshot.await;
            this.read_with(cx, |this, cx| SerializedThread {
                version: SerializedThread::VERSION.to_string(),
                summary: this.summary_or_default(),
                updated_at: this.updated_at(),
                messages: this
                    .messages()
                    .map(|message| SerializedMessage {
                        id: message.id,
                        role: message.role,
                        segments: message
                            .segments
                            .iter()
                            .map(|segment| match segment {
                                MessageSegment::Text(text) => {
                                    SerializedMessageSegment::Text { text: text.clone() }
                                }
                                MessageSegment::Thinking(text) => {
                                    SerializedMessageSegment::Thinking { text: text.clone() }
                                }
                            })
                            .collect(),
                        tool_uses: this
                            .tool_uses_for_message(message.id, cx)
                            .into_iter()
                            .map(|tool_use| SerializedToolUse {
                                id: tool_use.id,
                                name: tool_use.name,
                                input: tool_use.input,
                            })
                            .collect(),
                        tool_results: this
                            .tool_results_for_message(message.id)
                            .into_iter()
                            .map(|tool_result| SerializedToolResult {
                                tool_use_id: tool_result.tool_use_id.clone(),
                                is_error: tool_result.is_error,
                                content: tool_result.content.clone(),
                            })
                            .collect(),
                    })
                    .collect(),
                initial_project_snapshot,
                cumulative_token_usage: this.cumulative_token_usage.clone(),
                detailed_summary_state: this.detailed_summary_state.clone(),
            })
        })
    }

    pub fn set_system_prompt_context(&mut self, context: AssistantSystemPromptContext) {
        self.system_prompt_context = Some(context);
    }

    pub fn system_prompt_context(&self) -> &Option<AssistantSystemPromptContext> {
        &self.system_prompt_context
    }

    pub fn load_system_prompt_context(
        &self,
        cx: &App,
    ) -> Task<(AssistantSystemPromptContext, Option<ThreadError>)> {
        let project = self.project.read(cx);
        let tasks = project
            .visible_worktrees(cx)
            .map(|worktree| {
                Self::load_worktree_info_for_system_prompt(
                    project.fs().clone(),
                    worktree.read(cx),
                    cx,
                )
            })
            .collect::<Vec<_>>();

        cx.spawn(async |_cx| {
            let results = futures::future::join_all(tasks).await;
            let mut first_err = None;
            let worktrees = results
                .into_iter()
                .map(|(worktree, err)| {
                    if first_err.is_none() && err.is_some() {
                        first_err = err;
                    }
                    worktree
                })
                .collect::<Vec<_>>();
            (AssistantSystemPromptContext::new(worktrees), first_err)
        })
    }

    fn load_worktree_info_for_system_prompt(
        fs: Arc<dyn Fs>,
        worktree: &Worktree,
        cx: &App,
    ) -> Task<(WorktreeInfoForSystemPrompt, Option<ThreadError>)> {
        let root_name = worktree.root_name().into();
        let abs_path = worktree.abs_path();

        // Note that Cline supports `.clinerules` being a directory, but that is not currently
        // supported. This doesn't seem to occur often in GitHub repositories.
        const RULES_FILE_NAMES: [&'static str; 6] = [
            ".rules",
            ".cursorrules",
            ".windsurfrules",
            ".clinerules",
            ".github/copilot-instructions.md",
            "CLAUDE.md",
        ];
        let selected_rules_file = RULES_FILE_NAMES
            .into_iter()
            .filter_map(|name| {
                worktree
                    .entry_for_path(name)
                    .filter(|entry| entry.is_file())
                    .map(|entry| (entry.path.clone(), worktree.absolutize(&entry.path)))
            })
            .next();

        if let Some((rel_rules_path, abs_rules_path)) = selected_rules_file {
            cx.spawn(async move |_| {
                let rules_file_result = maybe!(async move {
                    let abs_rules_path = abs_rules_path?;
                    let text = fs.load(&abs_rules_path).await.with_context(|| {
                        format!("Failed to load assistant rules file {:?}", abs_rules_path)
                    })?;
                    anyhow::Ok(RulesFile {
                        rel_path: rel_rules_path,
                        abs_path: abs_rules_path.into(),
                        text: text.trim().to_string(),
                    })
                })
                .await;
                let (rules_file, rules_file_error) = match rules_file_result {
                    Ok(rules_file) => (Some(rules_file), None),
                    Err(err) => (
                        None,
                        Some(ThreadError::Message {
                            header: "Error loading rules file".into(),
                            message: format!("{err}").into(),
                        }),
                    ),
                };
                let worktree_info = WorktreeInfoForSystemPrompt {
                    root_name,
                    abs_path,
                    rules_file,
                };
                (worktree_info, rules_file_error)
            })
        } else {
            Task::ready((
                WorktreeInfoForSystemPrompt {
                    root_name,
                    abs_path,
                    rules_file: None,
                },
                None,
            ))
        }
    }

    pub fn send_to_model(
        &mut self,
        model: Arc<dyn LanguageModel>,
        request_kind: RequestKind,
        cx: &mut Context<Self>,
    ) {
        let mut request = self.to_completion_request(request_kind, cx);
        if model.supports_tools() {
            request.tools = {
                let mut tools = Vec::new();
                tools.extend(self.tools().enabled_tools(cx).into_iter().map(|tool| {
                    LanguageModelRequestTool {
                        name: tool.name(),
                        description: tool.description(),
                        input_schema: tool.input_schema(model.tool_input_format()),
                    }
                }));

                tools
            };
        }

        self.stream_completion(request, model, cx);
    }

    pub fn used_tools_since_last_user_message(&self) -> bool {
        for message in self.messages.iter().rev() {
            if self.tool_use.message_has_tool_results(message.id) {
                return true;
            } else if message.role == Role::User {
                return false;
            }
        }

        false
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

        if let Some(system_prompt_context) = self.system_prompt_context.as_ref() {
            if let Some(system_prompt) = self
                .prompt_builder
                .generate_assistant_system_prompt(system_prompt_context)
                .context("failed to generate assistant system prompt")
                .log_err()
            {
                request.messages.push(LanguageModelRequestMessage {
                    role: Role::System,
                    content: vec![MessageContent::Text(system_prompt)],
                    cache: true,
                });
            }
        } else {
            log::error!("system_prompt_context not set.")
        }

        let mut added_context_ids = HashSet::<ContextId>::default();

        for message in &self.messages {
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
                    if self.tool_use.message_has_tool_results(message.id) {
                        continue;
                    }
                }
            }

            // Attach context to this message if it's the first to reference it
            if let Some(context_ids) = self.context_by_message.get(&message.id) {
                let new_context_ids: Vec<_> = context_ids
                    .iter()
                    .filter(|id| !added_context_ids.contains(id))
                    .collect();

                if !new_context_ids.is_empty() {
                    let referenced_context = new_context_ids
                        .iter()
                        .filter_map(|context_id| self.context.get(*context_id));

                    attach_context_to_message(&mut request_message, referenced_context, cx);
                    added_context_ids.extend(context_ids.iter());
                }
            }

            if !message.segments.is_empty() {
                request_message
                    .content
                    .push(MessageContent::Text(message.to_string()));
            }

            match request_kind {
                RequestKind::Chat => {
                    self.tool_use
                        .attach_tool_uses(message.id, &mut request_message);
                }
                RequestKind::Summarize => {
                    // We don't care about tool use during summarization.
                }
            };

            request.messages.push(request_message);
        }

        // Set a cache breakpoint at the second-to-last message.
        // https://docs.anthropic.com/en/docs/build-with-claude/prompt-caching
        let breakpoint_index = request.messages.len() - 2;
        for (index, message) in request.messages.iter_mut().enumerate() {
            message.cache = index == breakpoint_index;
        }

        self.attached_tracked_files_state(&mut request.messages, cx);

        request
    }

    fn attached_tracked_files_state(
        &self,
        messages: &mut Vec<LanguageModelRequestMessage>,
        cx: &App,
    ) {
        const STALE_FILES_HEADER: &str = "These files changed since last read:";

        let mut stale_message = String::new();

        let action_log = self.action_log.read(cx);

        for stale_file in action_log.stale_buffers(cx) {
            let Some(file) = stale_file.read(cx).file() else {
                continue;
            };

            if stale_message.is_empty() {
                write!(&mut stale_message, "{}", STALE_FILES_HEADER).ok();
            }

            writeln!(&mut stale_message, "- {}", file.path().display()).ok();
        }

        let mut content = Vec::with_capacity(2);

        if !stale_message.is_empty() {
            content.push(stale_message.into());
        }

        if action_log.has_edited_files_since_project_diagnostics_check() {
            content.push(
                "\n\nWhen you're done making changes, make sure to check project diagnostics \
                and fix all errors AND warnings you introduced! \
                DO NOT mention you're going to do this until you're done."
                    .into(),
            );
        }

        if !content.is_empty() {
            let context_message = LanguageModelRequestMessage {
                role: Role::User,
                content,
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

        let task = cx.spawn(async move |thread, cx| {
            let stream = model.stream_completion(request, &cx);
            let initial_token_usage =
                thread.read_with(cx, |thread, _cx| thread.cumulative_token_usage.clone());
            let stream_completion = async {
                let mut events = stream.await?;
                let mut stop_reason = StopReason::EndTurn;
                let mut current_token_usage = TokenUsage::default();

                while let Some(event) = events.next().await {
                    let event = event?;

                    thread.update(cx, |thread, cx| {
                        match event {
                            LanguageModelCompletionEvent::StartMessage { .. } => {
                                thread.insert_message(
                                    Role::Assistant,
                                    vec![MessageSegment::Text(String::new())],
                                    cx,
                                );
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
                                        last_message.push_text(&chunk);
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
                                        thread.insert_message(
                                            Role::Assistant,
                                            vec![MessageSegment::Text(chunk.to_string())],
                                            cx,
                                        );
                                    };
                                }
                            }
                            LanguageModelCompletionEvent::Thinking(chunk) => {
                                if let Some(last_message) = thread.messages.last_mut() {
                                    if last_message.role == Role::Assistant {
                                        last_message.push_thinking(&chunk);
                                        cx.emit(ThreadEvent::StreamedAssistantThinking(
                                            last_message.id,
                                            chunk,
                                        ));
                                    } else {
                                        // If we won't have an Assistant message yet, assume this chunk marks the beginning
                                        // of a new Assistant response.
                                        //
                                        // Importantly: We do *not* want to emit a `StreamedAssistantText` event here, as it
                                        // will result in duplicating the text of the chunk in the rendered Markdown.
                                        thread.insert_message(
                                            Role::Assistant,
                                            vec![MessageSegment::Thinking(chunk.to_string())],
                                            cx,
                                        );
                                    };
                                }
                            }
                            LanguageModelCompletionEvent::ToolUse(tool_use) => {
                                let last_assistant_message = thread
                                    .messages
                                    .iter_mut()
                                    .rfind(|message| message.role == Role::Assistant);

                                let last_assistant_message_id =
                                    if let Some(message) = last_assistant_message {
                                        if let Some(segment) = message.segments.first_mut() {
                                            let text = segment.text_mut();
                                            if text.is_empty() {
                                                text.push_str("Using tool...");
                                            }
                                        } else {
                                            message.segments.push(MessageSegment::Text(
                                                "Using tool...".to_string(),
                                            ));
                                        }

                                        message.id
                                    } else {
                                        thread.insert_message(
                                            Role::Assistant,
                                            vec![MessageSegment::Text("Using tool...".to_string())],
                                            cx,
                                        )
                                    };
                                thread.tool_use.request_tool_use(
                                    last_assistant_message_id,
                                    tool_use,
                                    cx,
                                );
                            }
                        }

                        thread.touch_updated_at();
                        cx.emit(ThreadEvent::StreamedCompletion);
                        cx.notify();
                    })?;

                    smol::future::yield_now().await;
                }

                thread.update(cx, |thread, cx| {
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
                .update(cx, |thread, cx| {
                    thread.finalize_pending_checkpoint(cx);
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
                                cx.emit(ThreadEvent::ShowError(ThreadError::Message {
                                    header: "Error interacting with language model".into(),
                                    message: SharedString::from(error_message.clone()),
                                }));
                            }

                            thread.cancel_last_completion(cx);
                        }
                    }
                    cx.emit(ThreadEvent::DoneStreaming);

                    if let Ok(initial_usage) = initial_token_usage {
                        let usage = thread.cumulative_token_usage.clone() - initial_usage;

                        telemetry::event!(
                            "Assistant Thread Completion",
                            thread_id = thread.id().to_string(),
                            model = model.telemetry_id(),
                            model_provider = model.provider_id().to_string(),
                            input_tokens = usage.input_tokens,
                            output_tokens = usage.output_tokens,
                            cache_creation_input_tokens = usage.cache_creation_input_tokens,
                            cache_read_input_tokens = usage.cache_read_input_tokens,
                        );
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
                "Generate a concise 3-7 word title for this conversation, omitting punctuation. \
                 Go straight to the title, without any preamble and prefix like `Here's a concise suggestion:...` or `Title:`. \
                 If the conversation is about a specific subject, include it in the title. \
                 Be descriptive. DO NOT speak in the first person."
                    .into(),
            ],
            cache: false,
        });

        self.pending_summary = cx.spawn(async move |this, cx| {
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

                this.update(cx, |this, cx| {
                    if !new_summary.is_empty() {
                        this.summary = Some(new_summary.into());
                    }

                    cx.emit(ThreadEvent::SummaryChanged);
                })?;

                anyhow::Ok(())
            }
            .log_err()
            .await
        });
    }

    pub fn generate_detailed_summary(&mut self, cx: &mut Context<Self>) -> Option<Task<()>> {
        let last_message_id = self.messages.last().map(|message| message.id)?;

        match &self.detailed_summary_state {
            DetailedSummaryState::Generating { message_id, .. }
            | DetailedSummaryState::Generated { message_id, .. }
                if *message_id == last_message_id =>
            {
                // Already up-to-date
                return None;
            }
            _ => {}
        }

        let provider = LanguageModelRegistry::read_global(cx).active_provider()?;
        let model = LanguageModelRegistry::read_global(cx).active_model()?;

        if !provider.is_authenticated(cx) {
            return None;
        }

        let mut request = self.to_completion_request(RequestKind::Summarize, cx);

        request.messages.push(LanguageModelRequestMessage {
            role: Role::User,
            content: vec![
                "Generate a detailed summary of this conversation. Include:\n\
                1. A brief overview of what was discussed\n\
                2. Key facts or information discovered\n\
                3. Outcomes or conclusions reached\n\
                4. Any action items or next steps if any\n\
                Format it in Markdown with headings and bullet points."
                    .into(),
            ],
            cache: false,
        });

        let task = cx.spawn(async move |thread, cx| {
            let stream = model.stream_completion_text(request, &cx);
            let Some(mut messages) = stream.await.log_err() else {
                thread
                    .update(cx, |this, _cx| {
                        this.detailed_summary_state = DetailedSummaryState::NotGenerated;
                    })
                    .log_err();

                return;
            };

            let mut new_detailed_summary = String::new();

            while let Some(chunk) = messages.stream.next().await {
                if let Some(chunk) = chunk.log_err() {
                    new_detailed_summary.push_str(&chunk);
                }
            }

            thread
                .update(cx, |this, _cx| {
                    this.detailed_summary_state = DetailedSummaryState::Generated {
                        text: new_detailed_summary.into(),
                        message_id: last_message_id,
                    };
                })
                .log_err();
        });

        self.detailed_summary_state = DetailedSummaryState::Generating {
            message_id: last_message_id,
        };

        Some(task)
    }

    pub fn is_generating_detailed_summary(&self) -> bool {
        matches!(
            self.detailed_summary_state,
            DetailedSummaryState::Generating { .. }
        )
    }

    pub fn use_pending_tools(
        &mut self,
        cx: &mut Context<Self>,
    ) -> impl IntoIterator<Item = PendingToolUse> + use<> {
        let request = self.to_completion_request(RequestKind::Chat, cx);
        let messages = Arc::new(request.messages);
        let pending_tool_uses = self
            .tool_use
            .pending_tool_uses()
            .into_iter()
            .filter(|tool_use| tool_use.status.is_idle())
            .cloned()
            .collect::<Vec<_>>();

        for tool_use in pending_tool_uses.iter() {
            if let Some(tool) = self.tools.tool(&tool_use.name, cx) {
                if tool.needs_confirmation()
                    && !AssistantSettings::get_global(cx).always_allow_tool_actions
                {
                    self.tool_use.confirm_tool_use(
                        tool_use.id.clone(),
                        tool_use.ui_text.clone(),
                        tool_use.input.clone(),
                        messages.clone(),
                        tool,
                    );
                    cx.emit(ThreadEvent::ToolConfirmationNeeded);
                } else {
                    self.run_tool(
                        tool_use.id.clone(),
                        tool_use.ui_text.clone(),
                        tool_use.input.clone(),
                        &messages,
                        tool,
                        cx,
                    );
                }
            }
        }

        pending_tool_uses
    }

    pub fn run_tool(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        ui_text: impl Into<SharedString>,
        input: serde_json::Value,
        messages: &[LanguageModelRequestMessage],
        tool: Arc<dyn Tool>,
        cx: &mut Context<Thread>,
    ) {
        let task = self.spawn_tool_use(tool_use_id.clone(), messages, input, tool, cx);
        self.tool_use
            .run_pending_tool(tool_use_id, ui_text.into(), task);
    }

    fn spawn_tool_use(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        messages: &[LanguageModelRequestMessage],
        input: serde_json::Value,
        tool: Arc<dyn Tool>,
        cx: &mut Context<Thread>,
    ) -> Task<()> {
        let tool_name: Arc<str> = tool.name().into();

        let run_tool = if self.tools.is_disabled(&tool.source(), &tool_name) {
            Task::ready(Err(anyhow!("tool is disabled: {tool_name}")))
        } else {
            tool.run(
                input,
                messages,
                self.project.clone(),
                self.action_log.clone(),
                cx,
            )
        };

        cx.spawn({
            async move |thread: WeakEntity<Thread>, cx| {
                let output = run_tool.await;

                thread
                    .update(cx, |thread, cx| {
                        let pending_tool_use = thread.tool_use.insert_tool_output(
                            tool_use_id.clone(),
                            tool_name,
                            output,
                        );

                        cx.emit(ThreadEvent::ToolFinished {
                            tool_use_id,
                            pending_tool_use,
                            canceled: false,
                        });
                    })
                    .ok();
            }
        })
    }

    pub fn attach_tool_results(
        &mut self,
        updated_context: Vec<AssistantContext>,
        cx: &mut Context<Self>,
    ) {
        self.context.extend(
            updated_context
                .into_iter()
                .map(|context| (context.id(), context)),
        );

        // Insert a user message to contain the tool results.
        self.insert_user_message(
            // TODO: Sending up a user message without any content results in the model sending back
            // responses that also don't have any content. We currently don't handle this case well,
            // so for now we provide some text to keep the model on track.
            "Here are the tool results.",
            Vec::new(),
            None,
            cx,
        );
    }

    /// Cancels the last pending completion, if there are any pending.
    ///
    /// Returns whether a completion was canceled.
    pub fn cancel_last_completion(&mut self, cx: &mut Context<Self>) -> bool {
        let canceled = if self.pending_completions.pop().is_some() {
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
        };
        self.finalize_pending_checkpoint(cx);
        canceled
    }

    /// Returns the feedback given to the thread, if any.
    pub fn feedback(&self) -> Option<ThreadFeedback> {
        self.feedback
    }

    /// Reports feedback about the thread and stores it in our telemetry backend.
    pub fn report_feedback(
        &mut self,
        feedback: ThreadFeedback,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let final_project_snapshot = Self::project_snapshot(self.project.clone(), cx);
        let serialized_thread = self.serialize(cx);
        let thread_id = self.id().clone();
        let client = self.project.read(cx).client();
        self.feedback = Some(feedback);
        cx.notify();

        cx.background_spawn(async move {
            let final_project_snapshot = final_project_snapshot.await;
            let serialized_thread = serialized_thread.await?;
            let thread_data =
                serde_json::to_value(serialized_thread).unwrap_or_else(|_| serde_json::Value::Null);

            let rating = match feedback {
                ThreadFeedback::Positive => "positive",
                ThreadFeedback::Negative => "negative",
            };
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
        let git_store = project.read(cx).git_store().clone();
        let worktree_snapshots: Vec<_> = project
            .read(cx)
            .visible_worktrees(cx)
            .map(|worktree| Self::worktree_snapshot(worktree, git_store.clone(), cx))
            .collect();

        cx.spawn(async move |_, cx| {
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

    fn worktree_snapshot(
        worktree: Entity<project::Worktree>,
        git_store: Entity<GitStore>,
        cx: &App,
    ) -> Task<WorktreeSnapshot> {
        cx.spawn(async move |cx| {
            // Get worktree path and snapshot
            let worktree_info = cx.update(|app_cx| {
                let worktree = worktree.read(app_cx);
                let path = worktree.abs_path().to_string_lossy().to_string();
                let snapshot = worktree.snapshot();
                (path, snapshot)
            });

            let Ok((worktree_path, _snapshot)) = worktree_info else {
                return WorktreeSnapshot {
                    worktree_path: String::new(),
                    git_state: None,
                };
            };

            let git_state = git_store
                .update(cx, |git_store, cx| {
                    git_store
                        .repositories()
                        .values()
                        .find(|repo| {
                            repo.read(cx)
                                .abs_path_to_repo_path(&worktree.read(cx).abs_path())
                                .is_some()
                        })
                        .cloned()
                })
                .ok()
                .flatten()
                .map(|repo| {
                    repo.read_with(cx, |repo, _| {
                        let current_branch =
                            repo.branch.as_ref().map(|branch| branch.name.to_string());
                        repo.send_job(|state, _| async move {
                            let RepositoryState::Local { backend, .. } = state else {
                                return GitState {
                                    remote_url: None,
                                    head_sha: None,
                                    current_branch,
                                    diff: None,
                                };
                            };

                            let remote_url = backend.remote_url("origin");
                            let head_sha = backend.head_sha();
                            let diff = backend.diff(DiffType::HeadToWorktree).await.ok();

                            GitState {
                                remote_url,
                                head_sha,
                                current_branch,
                                diff,
                            }
                        })
                    })
                });

            let git_state = match git_state {
                Some(git_state) => match git_state.ok() {
                    Some(git_state) => git_state.await.ok(),
                    None => None,
                },
                None => None,
            };

            WorktreeSnapshot {
                worktree_path,
                git_state,
            }
        })
    }

    pub fn to_markdown(&self, cx: &App) -> Result<String> {
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
            for segment in &message.segments {
                match segment {
                    MessageSegment::Text(text) => writeln!(markdown, "{}\n", text)?,
                    MessageSegment::Thinking(text) => {
                        writeln!(markdown, "<think>{}</think>\n", text)?
                    }
                }
            }

            for tool_use in self.tool_uses_for_message(message.id, cx) {
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

    pub fn keep_edits_in_range(
        &mut self,
        buffer: Entity<language::Buffer>,
        buffer_range: Range<language::Anchor>,
        cx: &mut Context<Self>,
    ) {
        self.action_log.update(cx, |action_log, cx| {
            action_log.keep_edits_in_range(buffer, buffer_range, cx)
        });
    }

    pub fn keep_all_edits(&mut self, cx: &mut Context<Self>) {
        self.action_log
            .update(cx, |action_log, cx| action_log.keep_all_edits(cx));
    }

    pub fn action_log(&self) -> &Entity<ActionLog> {
        &self.action_log
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    pub fn cumulative_token_usage(&self) -> TokenUsage {
        self.cumulative_token_usage.clone()
    }

    pub fn is_getting_too_long(&self, cx: &App) -> bool {
        let model_registry = LanguageModelRegistry::read_global(cx);
        let Some(model) = model_registry.active_model() else {
            return false;
        };

        let max_tokens = model.max_token_count();

        let current_usage =
            self.cumulative_token_usage.input_tokens + self.cumulative_token_usage.output_tokens;

        #[cfg(debug_assertions)]
        let warning_threshold: f32 = std::env::var("ZED_THREAD_WARNING_THRESHOLD")
            .unwrap_or("0.9".to_string())
            .parse()
            .unwrap();
        #[cfg(not(debug_assertions))]
        let warning_threshold: f32 = 0.9;

        current_usage as f32 >= (max_tokens as f32 * warning_threshold)
    }

    pub fn deny_tool_use(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        cx: &mut Context<Self>,
    ) {
        let err = Err(anyhow::anyhow!(
            "Permission to run tool action denied by user"
        ));

        self.tool_use
            .insert_tool_output(tool_use_id.clone(), tool_name, err);

        cx.emit(ThreadEvent::ToolFinished {
            tool_use_id,
            pending_tool_use: None,
            canceled: true,
        });
    }
}

#[derive(Debug, Clone)]
pub enum ThreadError {
    PaymentRequired,
    MaxMonthlySpendReached,
    Message {
        header: SharedString,
        message: SharedString,
    },
}

#[derive(Debug, Clone)]
pub enum ThreadEvent {
    ShowError(ThreadError),
    StreamedCompletion,
    StreamedAssistantText(MessageId, String),
    StreamedAssistantThinking(MessageId, String),
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
    CheckpointChanged,
    ToolConfirmationNeeded,
}

impl EventEmitter<ThreadEvent> for Thread {}

struct PendingCompletion {
    id: usize,
    _task: Task<()>,
}
