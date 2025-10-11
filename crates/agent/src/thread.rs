use crate::{
    agent_profile::AgentProfile,
    context::{AgentContext, AgentContextHandle, ContextLoadResult, LoadedContext},
    thread_store::{
        SerializedCrease, SerializedLanguageModel, SerializedMessage, SerializedMessageSegment,
        SerializedThread, SerializedToolResult, SerializedToolUse, SharedProjectContext,
        ThreadStore,
    },
    tool_use::{PendingToolUse, ToolUse, ToolUseMetadata, ToolUseState},
};
use action_log::ActionLog;
use agent_settings::{
    AgentProfileId, AgentSettings, CompletionMode, SUMMARIZE_THREAD_DETAILED_PROMPT,
    SUMMARIZE_THREAD_PROMPT,
};
use anyhow::{Result, anyhow};
use assistant_tool::{AnyToolCard, Tool, ToolWorkingSet};
use chrono::{DateTime, Utc};
use client::{ModelRequestUsage, RequestUsage};
use cloud_llm_client::{CompletionIntent, CompletionRequestStatus, Plan, UsageLimit};
use collections::HashMap;
use futures::{FutureExt, StreamExt as _, future::Shared};
use git::repository::DiffType;
use gpui::{
    AnyWindowHandle, App, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task,
    WeakEntity, Window,
};
use http_client::StatusCode;
use language_model::{
    ConfiguredModel, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelExt as _, LanguageModelId, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelRequestTool, LanguageModelToolResult,
    LanguageModelToolResultContent, LanguageModelToolUse, LanguageModelToolUseId, MessageContent,
    ModelRequestLimitReachedError, PaymentRequiredError, Role, SelectedModel, StopReason,
    TokenUsage,
};
use postage::stream::Stream as _;
use project::{
    Project,
    git_store::{GitStore, GitStoreCheckpoint, RepositoryState},
};
use prompt_store::{ModelContext, PromptBuilder};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{
    io::Write,
    ops::Range,
    sync::Arc,
    time::{Duration, Instant},
};
use thiserror::Error;
use util::{ResultExt as _, post_inc};
use uuid::Uuid;

const MAX_RETRY_ATTEMPTS: u8 = 4;
const BASE_RETRY_DELAY: Duration = Duration::from_secs(5);

#[derive(Debug, Clone)]
enum RetryStrategy {
    ExponentialBackoff {
        initial_delay: Duration,
        max_attempts: u8,
    },
    Fixed {
        delay: Duration,
        max_attempts: u8,
    },
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

/// The ID of the user prompt that initiated a request.
///
/// This equates to the user physically submitting a message to the model (e.g., by pressing the Enter key).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct PromptId(Arc<str>);

impl PromptId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string().into())
    }
}

impl std::fmt::Display for PromptId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct MessageId(pub usize);

impl MessageId {
    fn post_inc(&mut self) -> Self {
        Self(post_inc(&mut self.0))
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

/// Stored information that can be used to resurrect a context crease when creating an editor for a past message.
#[derive(Clone, Debug)]
pub struct MessageCrease {
    pub range: Range<usize>,
    pub icon_path: SharedString,
    pub label: SharedString,
    /// None for a deserialized message, Some otherwise.
    pub context: Option<AgentContextHandle>,
}

/// A message in a [`Thread`].
#[derive(Debug, Clone)]
pub struct Message {
    pub id: MessageId,
    pub role: Role,
    pub segments: Vec<MessageSegment>,
    pub loaded_context: LoadedContext,
    pub creases: Vec<MessageCrease>,
    pub is_hidden: bool,
    pub ui_only: bool,
}

impl Message {
    /// Returns whether the message contains any meaningful text that should be displayed
    /// The model sometimes runs tool without producing any text or just a marker ([`USING_TOOL_MARKER`])
    pub fn should_display_content(&self) -> bool {
        self.segments.iter().all(|segment| segment.should_display())
    }

    pub fn push_thinking(&mut self, text: &str, signature: Option<String>) {
        if let Some(MessageSegment::Thinking {
            text: segment,
            signature: current_signature,
        }) = self.segments.last_mut()
        {
            if let Some(signature) = signature {
                *current_signature = Some(signature);
            }
            segment.push_str(text);
        } else {
            self.segments.push(MessageSegment::Thinking {
                text: text.to_string(),
                signature,
            });
        }
    }

    pub fn push_redacted_thinking(&mut self, data: String) {
        self.segments.push(MessageSegment::RedactedThinking(data));
    }

    pub fn push_text(&mut self, text: &str) {
        if let Some(MessageSegment::Text(segment)) = self.segments.last_mut() {
            segment.push_str(text);
        } else {
            self.segments.push(MessageSegment::Text(text.to_string()));
        }
    }

    pub fn to_message_content(&self) -> String {
        let mut result = String::new();

        if !self.loaded_context.text.is_empty() {
            result.push_str(&self.loaded_context.text);
        }

        for segment in &self.segments {
            match segment {
                MessageSegment::Text(text) => result.push_str(text),
                MessageSegment::Thinking { text, .. } => {
                    result.push_str("<think>\n");
                    result.push_str(text);
                    result.push_str("\n</think>");
                }
                MessageSegment::RedactedThinking(_) => {}
            }
        }

        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageSegment {
    Text(String),
    Thinking {
        text: String,
        signature: Option<String>,
    },
    RedactedThinking(String),
}

impl MessageSegment {
    pub fn should_display(&self) -> bool {
        match self {
            Self::Text(text) => text.is_empty(),
            Self::Thinking { text, .. } => text.is_empty(),
            Self::RedactedThinking(_) => false,
        }
    }

    pub fn text(&self) -> Option<&str> {
        match self {
            MessageSegment::Text(text) => Some(text),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectSnapshot {
    pub worktree_snapshots: Vec<WorktreeSnapshot>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorktreeSnapshot {
    pub worktree_path: String,
    pub git_state: Option<GitState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitState {
    pub remote_url: Option<String>,
    pub head_sha: Option<String>,
    pub current_branch: Option<String>,
    pub diff: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ThreadCheckpoint {
    message_id: MessageId,
    git_checkpoint: GitStoreCheckpoint,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
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

impl DetailedSummaryState {
    fn text(&self) -> Option<SharedString> {
        if let Self::Generated { text, .. } = self {
            Some(text.clone())
        } else {
            None
        }
    }
}

#[derive(Default, Debug)]
pub struct TotalTokenUsage {
    pub total: u64,
    pub max: u64,
}

impl TotalTokenUsage {
    pub fn ratio(&self) -> TokenUsageRatio {
        #[cfg(debug_assertions)]
        let warning_threshold: f32 = std::env::var("ZED_THREAD_WARNING_THRESHOLD")
            .unwrap_or("0.8".to_string())
            .parse()
            .unwrap();
        #[cfg(not(debug_assertions))]
        let warning_threshold: f32 = 0.8;

        // When the maximum is unknown because there is no selected model,
        // avoid showing the token limit warning.
        if self.max == 0 {
            TokenUsageRatio::Normal
        } else if self.total >= self.max {
            TokenUsageRatio::Exceeded
        } else if self.total as f32 / self.max as f32 >= warning_threshold {
            TokenUsageRatio::Warning
        } else {
            TokenUsageRatio::Normal
        }
    }

    pub fn add(&self, tokens: u64) -> TotalTokenUsage {
        TotalTokenUsage {
            total: self.total + tokens,
            max: self.max,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum TokenUsageRatio {
    #[default]
    Normal,
    Warning,
    Exceeded,
}

#[derive(Debug, Clone, Copy)]
pub enum QueueState {
    Sending,
    Queued { position: usize },
    Started,
}

/// A thread of conversation with the LLM.
pub struct Thread {
    id: ThreadId,
    updated_at: DateTime<Utc>,
    summary: ThreadSummary,
    pending_summary: Task<Option<()>>,
    detailed_summary_task: Task<Option<()>>,
    detailed_summary_tx: postage::watch::Sender<DetailedSummaryState>,
    detailed_summary_rx: postage::watch::Receiver<DetailedSummaryState>,
    completion_mode: agent_settings::CompletionMode,
    messages: Vec<Message>,
    next_message_id: MessageId,
    last_prompt_id: PromptId,
    project_context: SharedProjectContext,
    checkpoints_by_message: HashMap<MessageId, ThreadCheckpoint>,
    completion_count: usize,
    pending_completions: Vec<PendingCompletion>,
    project: Entity<Project>,
    prompt_builder: Arc<PromptBuilder>,
    tools: Entity<ToolWorkingSet>,
    tool_use: ToolUseState,
    action_log: Entity<ActionLog>,
    last_restore_checkpoint: Option<LastRestoreCheckpoint>,
    pending_checkpoint: Option<ThreadCheckpoint>,
    initial_project_snapshot: Shared<Task<Option<Arc<ProjectSnapshot>>>>,
    request_token_usage: Vec<TokenUsage>,
    cumulative_token_usage: TokenUsage,
    exceeded_window_error: Option<ExceededWindowError>,
    tool_use_limit_reached: bool,
    retry_state: Option<RetryState>,
    message_feedback: HashMap<MessageId, ThreadFeedback>,
    last_received_chunk_at: Option<Instant>,
    request_callback: Option<
        Box<dyn FnMut(&LanguageModelRequest, &[Result<LanguageModelCompletionEvent, String>])>,
    >,
    remaining_turns: u32,
    configured_model: Option<ConfiguredModel>,
    profile: AgentProfile,
    last_error_context: Option<(Arc<dyn LanguageModel>, CompletionIntent)>,
}

#[derive(Clone, Debug)]
struct RetryState {
    attempt: u8,
    max_attempts: u8,
    intent: CompletionIntent,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ThreadSummary {
    Pending,
    Generating,
    Ready(SharedString),
    Error,
}

impl ThreadSummary {
    pub const DEFAULT: SharedString = SharedString::new_static("New Thread");

    pub fn or_default(&self) -> SharedString {
        self.unwrap_or(Self::DEFAULT)
    }

    pub fn unwrap_or(&self, message: impl Into<SharedString>) -> SharedString {
        self.ready().unwrap_or_else(|| message.into())
    }

    pub fn ready(&self) -> Option<SharedString> {
        match self {
            ThreadSummary::Ready(summary) => Some(summary.clone()),
            ThreadSummary::Pending | ThreadSummary::Generating | ThreadSummary::Error => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExceededWindowError {
    /// Model used when last message exceeded context window
    model_id: LanguageModelId,
    /// Token count including last message
    token_count: u64,
}

impl Thread {
    pub fn new(
        project: Entity<Project>,
        tools: Entity<ToolWorkingSet>,
        prompt_builder: Arc<PromptBuilder>,
        system_prompt: SharedProjectContext,
        cx: &mut Context<Self>,
    ) -> Self {
        let (detailed_summary_tx, detailed_summary_rx) = postage::watch::channel();
        let configured_model = LanguageModelRegistry::read_global(cx).default_model();
        let profile_id = AgentSettings::get_global(cx).default_profile.clone();

        Self {
            id: ThreadId::new(),
            updated_at: Utc::now(),
            summary: ThreadSummary::Pending,
            pending_summary: Task::ready(None),
            detailed_summary_task: Task::ready(None),
            detailed_summary_tx,
            detailed_summary_rx,
            completion_mode: AgentSettings::get_global(cx).preferred_completion_mode,
            messages: Vec::new(),
            next_message_id: MessageId(0),
            last_prompt_id: PromptId::new(),
            project_context: system_prompt,
            checkpoints_by_message: HashMap::default(),
            completion_count: 0,
            pending_completions: Vec::new(),
            project: project.clone(),
            prompt_builder,
            tools: tools.clone(),
            last_restore_checkpoint: None,
            pending_checkpoint: None,
            tool_use: ToolUseState::new(tools.clone()),
            action_log: cx.new(|_| ActionLog::new(project.clone())),
            initial_project_snapshot: {
                let project_snapshot = Self::project_snapshot(project, cx);
                cx.foreground_executor()
                    .spawn(async move { Some(project_snapshot.await) })
                    .shared()
            },
            request_token_usage: Vec::new(),
            cumulative_token_usage: TokenUsage::default(),
            exceeded_window_error: None,
            tool_use_limit_reached: false,
            retry_state: None,
            message_feedback: HashMap::default(),
            last_error_context: None,
            last_received_chunk_at: None,
            request_callback: None,
            remaining_turns: u32::MAX,
            configured_model,
            profile: AgentProfile::new(profile_id, tools),
        }
    }

    pub fn deserialize(
        id: ThreadId,
        serialized: SerializedThread,
        project: Entity<Project>,
        tools: Entity<ToolWorkingSet>,
        prompt_builder: Arc<PromptBuilder>,
        project_context: SharedProjectContext,
        window: Option<&mut Window>, // None in headless mode
        cx: &mut Context<Self>,
    ) -> Self {
        let next_message_id = MessageId(
            serialized
                .messages
                .last()
                .map(|message| message.id.0 + 1)
                .unwrap_or(0),
        );
        let tool_use = ToolUseState::from_serialized_messages(
            tools.clone(),
            &serialized.messages,
            project.clone(),
            window,
            cx,
        );
        let (detailed_summary_tx, detailed_summary_rx) =
            postage::watch::channel_with(serialized.detailed_summary_state);

        let configured_model = LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            serialized
                .model
                .and_then(|model| {
                    let model = SelectedModel {
                        provider: model.provider.clone().into(),
                        model: model.model.into(),
                    };
                    registry.select_model(&model, cx)
                })
                .or_else(|| registry.default_model())
        });

        let completion_mode = serialized
            .completion_mode
            .unwrap_or_else(|| AgentSettings::get_global(cx).preferred_completion_mode);
        let profile_id = serialized
            .profile
            .unwrap_or_else(|| AgentSettings::get_global(cx).default_profile.clone());

        Self {
            id,
            updated_at: serialized.updated_at,
            summary: ThreadSummary::Ready(serialized.summary),
            pending_summary: Task::ready(None),
            detailed_summary_task: Task::ready(None),
            detailed_summary_tx,
            detailed_summary_rx,
            completion_mode,
            retry_state: None,
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
                            SerializedMessageSegment::Thinking { text, signature } => {
                                MessageSegment::Thinking { text, signature }
                            }
                            SerializedMessageSegment::RedactedThinking { data } => {
                                MessageSegment::RedactedThinking(data)
                            }
                        })
                        .collect(),
                    loaded_context: LoadedContext {
                        contexts: Vec::new(),
                        text: message.context,
                        images: Vec::new(),
                    },
                    creases: message
                        .creases
                        .into_iter()
                        .map(|crease| MessageCrease {
                            range: crease.start..crease.end,
                            icon_path: crease.icon_path,
                            label: crease.label,
                            context: None,
                        })
                        .collect(),
                    is_hidden: message.is_hidden,
                    ui_only: false, // UI-only messages are not persisted
                })
                .collect(),
            next_message_id,
            last_prompt_id: PromptId::new(),
            project_context,
            checkpoints_by_message: HashMap::default(),
            completion_count: 0,
            pending_completions: Vec::new(),
            last_restore_checkpoint: None,
            pending_checkpoint: None,
            project: project.clone(),
            prompt_builder,
            tools: tools.clone(),
            tool_use,
            action_log: cx.new(|_| ActionLog::new(project)),
            initial_project_snapshot: Task::ready(serialized.initial_project_snapshot).shared(),
            request_token_usage: serialized.request_token_usage,
            cumulative_token_usage: serialized.cumulative_token_usage,
            exceeded_window_error: None,
            tool_use_limit_reached: serialized.tool_use_limit_reached,
            message_feedback: HashMap::default(),
            last_error_context: None,
            last_received_chunk_at: None,
            request_callback: None,
            remaining_turns: u32::MAX,
            configured_model,
            profile: AgentProfile::new(profile_id, tools),
        }
    }

    pub fn set_request_callback(
        &mut self,
        callback: impl 'static
        + FnMut(&LanguageModelRequest, &[Result<LanguageModelCompletionEvent, String>]),
    ) {
        self.request_callback = Some(Box::new(callback));
    }

    pub fn id(&self) -> &ThreadId {
        &self.id
    }

    pub fn profile(&self) -> &AgentProfile {
        &self.profile
    }

    pub fn set_profile(&mut self, id: AgentProfileId, cx: &mut Context<Self>) {
        if &id != self.profile.id() {
            self.profile = AgentProfile::new(id, self.tools.clone());
            cx.emit(ThreadEvent::ProfileChanged);
        }
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

    pub fn advance_prompt_id(&mut self) {
        self.last_prompt_id = PromptId::new();
    }

    pub fn project_context(&self) -> SharedProjectContext {
        self.project_context.clone()
    }

    pub fn get_or_init_configured_model(&mut self, cx: &App) -> Option<ConfiguredModel> {
        if self.configured_model.is_none() {
            self.configured_model = LanguageModelRegistry::read_global(cx).default_model();
        }
        self.configured_model.clone()
    }

    pub fn configured_model(&self) -> Option<ConfiguredModel> {
        self.configured_model.clone()
    }

    pub fn set_configured_model(&mut self, model: Option<ConfiguredModel>, cx: &mut Context<Self>) {
        self.configured_model = model;
        cx.notify();
    }

    pub fn summary(&self) -> &ThreadSummary {
        &self.summary
    }

    pub fn set_summary(&mut self, new_summary: impl Into<SharedString>, cx: &mut Context<Self>) {
        let current_summary = match &self.summary {
            ThreadSummary::Pending | ThreadSummary::Generating => return,
            ThreadSummary::Ready(summary) => summary,
            ThreadSummary::Error => &ThreadSummary::DEFAULT,
        };

        let mut new_summary = new_summary.into();

        if new_summary.is_empty() {
            new_summary = ThreadSummary::DEFAULT;
        }

        if current_summary != &new_summary {
            self.summary = ThreadSummary::Ready(new_summary);
            cx.emit(ThreadEvent::SummaryChanged);
        }
    }

    pub fn completion_mode(&self) -> CompletionMode {
        self.completion_mode
    }

    pub fn set_completion_mode(&mut self, mode: CompletionMode) {
        self.completion_mode = mode;
    }

    pub fn message(&self, id: MessageId) -> Option<&Message> {
        let index = self
            .messages
            .binary_search_by(|message| message.id.cmp(&id))
            .ok()?;

        self.messages.get(index)
    }

    pub fn messages(&self) -> impl ExactSizeIterator<Item = &Message> {
        self.messages.iter()
    }

    pub fn is_generating(&self) -> bool {
        !self.pending_completions.is_empty() || !self.all_tools_finished()
    }

    /// Indicates whether streaming of language model events is stale.
    /// When `is_generating()` is false, this method returns `None`.
    pub fn is_generation_stale(&self) -> Option<bool> {
        const STALE_THRESHOLD: u128 = 250;

        self.last_received_chunk_at
            .map(|instant| instant.elapsed().as_millis() > STALE_THRESHOLD)
    }

    fn received_chunk(&mut self) {
        self.last_received_chunk_at = Some(Instant::now());
    }

    pub fn queue_state(&self) -> Option<QueueState> {
        self.pending_completions
            .first()
            .map(|pending_completion| pending_completion.queue_state)
    }

    pub fn tools(&self) -> &Entity<ToolWorkingSet> {
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

        let git_store = self.project().read(cx).git_store().clone();
        let restore = git_store.update(cx, |git_store, cx| {
            git_store.restore_checkpoint(checkpoint.git_checkpoint.clone(), cx)
        });

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

        self.finalize_checkpoint(pending_checkpoint, cx);
    }

    fn finalize_checkpoint(
        &mut self,
        pending_checkpoint: ThreadCheckpoint,
        cx: &mut Context<Self>,
    ) {
        let git_store = self.project.read(cx).git_store().clone();
        let final_checkpoint = git_store.update(cx, |git_store, cx| git_store.checkpoint(cx));
        cx.spawn(async move |this, cx| match final_checkpoint.await {
            Ok(final_checkpoint) => {
                let equal = git_store
                    .update(cx, |store, cx| {
                        store.compare_checkpoints(
                            pending_checkpoint.git_checkpoint.clone(),
                            final_checkpoint.clone(),
                            cx,
                        )
                    })?
                    .await
                    .unwrap_or(false);

                this.update(cx, |this, cx| {
                    this.pending_checkpoint = if equal {
                        Some(pending_checkpoint)
                    } else {
                        this.insert_checkpoint(pending_checkpoint, cx);
                        Some(ThreadCheckpoint {
                            message_id: this.next_message_id,
                            git_checkpoint: final_checkpoint,
                        })
                    }
                })?;

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
            self.checkpoints_by_message.remove(&deleted_message.id);
        }
        cx.notify();
    }

    pub fn context_for_message(&self, id: MessageId) -> impl Iterator<Item = &AgentContext> {
        self.messages
            .iter()
            .find(|message| message.id == id)
            .into_iter()
            .flat_map(|message| message.loaded_context.contexts.iter())
    }

    pub fn is_turn_end(&self, ix: usize) -> bool {
        if self.messages.is_empty() {
            return false;
        }

        if !self.is_generating() && ix == self.messages.len() - 1 {
            return true;
        }

        let Some(message) = self.messages.get(ix) else {
            return false;
        };

        if message.role != Role::Assistant {
            return false;
        }

        self.messages
            .get(ix + 1)
            .and_then(|message| {
                self.message(message.id)
                    .map(|next_message| next_message.role == Role::User && !next_message.is_hidden)
            })
            .unwrap_or(false)
    }

    pub fn tool_use_limit_reached(&self) -> bool {
        self.tool_use_limit_reached
    }

    /// Returns whether all of the tool uses have finished running.
    pub fn all_tools_finished(&self) -> bool {
        // If the only pending tool uses left are the ones with errors, then
        // that means that we've finished running all of the pending tools.
        self.tool_use
            .pending_tool_uses()
            .iter()
            .all(|pending_tool_use| pending_tool_use.status.is_error())
    }

    /// Returns whether any pending tool uses may perform edits
    pub fn has_pending_edit_tool_uses(&self) -> bool {
        self.tool_use
            .pending_tool_uses()
            .iter()
            .filter(|pending_tool_use| !pending_tool_use.status.is_error())
            .any(|pending_tool_use| pending_tool_use.may_perform_edits)
    }

    pub fn tool_uses_for_message(&self, id: MessageId, cx: &App) -> Vec<ToolUse> {
        self.tool_use.tool_uses_for_message(id, &self.project, cx)
    }

    pub fn tool_results_for_message(
        &self,
        assistant_message_id: MessageId,
    ) -> Vec<&LanguageModelToolResult> {
        self.tool_use.tool_results_for_message(assistant_message_id)
    }

    pub fn tool_result(&self, id: &LanguageModelToolUseId) -> Option<&LanguageModelToolResult> {
        self.tool_use.tool_result(id)
    }

    pub fn output_for_tool(&self, id: &LanguageModelToolUseId) -> Option<&Arc<str>> {
        match &self.tool_use.tool_result(id)?.content {
            LanguageModelToolResultContent::Text(text) => Some(text),
            LanguageModelToolResultContent::Image(_) => {
                // TODO: We should display image
                None
            }
        }
    }

    pub fn card_for_tool(&self, id: &LanguageModelToolUseId) -> Option<AnyToolCard> {
        self.tool_use.tool_result_card(id).cloned()
    }

    /// Return tools that are both enabled and supported by the model
    pub fn available_tools(
        &self,
        cx: &App,
        model: Arc<dyn LanguageModel>,
    ) -> Vec<LanguageModelRequestTool> {
        if model.supports_tools() {
            self.profile
                .enabled_tools(cx)
                .into_iter()
                .filter_map(|(name, tool)| {
                    // Skip tools that cannot be supported
                    let input_schema = tool.input_schema(model.tool_input_format()).ok()?;
                    Some(LanguageModelRequestTool {
                        name: name.into(),
                        description: tool.description(),
                        input_schema,
                    })
                })
                .collect()
        } else {
            Vec::default()
        }
    }

    pub fn insert_user_message(
        &mut self,
        text: impl Into<String>,
        loaded_context: ContextLoadResult,
        git_checkpoint: Option<GitStoreCheckpoint>,
        creases: Vec<MessageCrease>,
        cx: &mut Context<Self>,
    ) -> MessageId {
        if !loaded_context.referenced_buffers.is_empty() {
            self.action_log.update(cx, |log, cx| {
                for buffer in loaded_context.referenced_buffers {
                    log.buffer_read(buffer, cx);
                }
            });
        }

        let message_id = self.insert_message(
            Role::User,
            vec![MessageSegment::Text(text.into())],
            loaded_context.loaded_context,
            creases,
            false,
            cx,
        );

        if let Some(git_checkpoint) = git_checkpoint {
            self.pending_checkpoint = Some(ThreadCheckpoint {
                message_id,
                git_checkpoint,
            });
        }

        message_id
    }

    pub fn insert_invisible_continue_message(&mut self, cx: &mut Context<Self>) -> MessageId {
        let id = self.insert_message(
            Role::User,
            vec![MessageSegment::Text("Continue where you left off".into())],
            LoadedContext::default(),
            vec![],
            true,
            cx,
        );
        self.pending_checkpoint = None;

        id
    }

    pub fn insert_assistant_message(
        &mut self,
        segments: Vec<MessageSegment>,
        cx: &mut Context<Self>,
    ) -> MessageId {
        self.insert_message(
            Role::Assistant,
            segments,
            LoadedContext::default(),
            Vec::new(),
            false,
            cx,
        )
    }

    pub fn insert_message(
        &mut self,
        role: Role,
        segments: Vec<MessageSegment>,
        loaded_context: LoadedContext,
        creases: Vec<MessageCrease>,
        is_hidden: bool,
        cx: &mut Context<Self>,
    ) -> MessageId {
        let id = self.next_message_id.post_inc();
        self.messages.push(Message {
            id,
            role,
            segments,
            loaded_context,
            creases,
            is_hidden,
            ui_only: false,
        });
        self.touch_updated_at();
        cx.emit(ThreadEvent::MessageAdded(id));
        id
    }

    pub fn edit_message(
        &mut self,
        id: MessageId,
        new_role: Role,
        new_segments: Vec<MessageSegment>,
        creases: Vec<MessageCrease>,
        loaded_context: Option<LoadedContext>,
        checkpoint: Option<GitStoreCheckpoint>,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(message) = self.messages.iter_mut().find(|message| message.id == id) else {
            return false;
        };
        message.role = new_role;
        message.segments = new_segments;
        message.creases = creases;
        if let Some(context) = loaded_context {
            message.loaded_context = context;
        }
        if let Some(git_checkpoint) = checkpoint {
            self.checkpoints_by_message.insert(
                id,
                ThreadCheckpoint {
                    message_id: id,
                    git_checkpoint,
                },
            );
        }
        self.touch_updated_at();
        cx.emit(ThreadEvent::MessageEdited(id));
        true
    }

    pub fn delete_message(&mut self, id: MessageId, cx: &mut Context<Self>) -> bool {
        let Some(index) = self.messages.iter().position(|message| message.id == id) else {
            return false;
        };
        self.messages.remove(index);
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
                language_model::Role::Assistant => "Agent:",
                language_model::Role::System => "System:",
            });
            text.push('\n');

            for segment in &message.segments {
                match segment {
                    MessageSegment::Text(content) => text.push_str(content),
                    MessageSegment::Thinking { text: content, .. } => {
                        text.push_str(&format!("<think>{}</think>", content))
                    }
                    MessageSegment::RedactedThinking(_) => {}
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
                summary: this.summary().or_default(),
                updated_at: this.updated_at(),
                messages: this
                    .messages()
                    .filter(|message| !message.ui_only)
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
                                MessageSegment::Thinking { text, signature } => {
                                    SerializedMessageSegment::Thinking {
                                        text: text.clone(),
                                        signature: signature.clone(),
                                    }
                                }
                                MessageSegment::RedactedThinking(data) => {
                                    SerializedMessageSegment::RedactedThinking {
                                        data: data.clone(),
                                    }
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
                                output: tool_result.output.clone(),
                            })
                            .collect(),
                        context: message.loaded_context.text.clone(),
                        creases: message
                            .creases
                            .iter()
                            .map(|crease| SerializedCrease {
                                start: crease.range.start,
                                end: crease.range.end,
                                icon_path: crease.icon_path.clone(),
                                label: crease.label.clone(),
                            })
                            .collect(),
                        is_hidden: message.is_hidden,
                    })
                    .collect(),
                initial_project_snapshot,
                cumulative_token_usage: this.cumulative_token_usage,
                request_token_usage: this.request_token_usage.clone(),
                detailed_summary_state: this.detailed_summary_rx.borrow().clone(),
                exceeded_window_error: this.exceeded_window_error.clone(),
                model: this
                    .configured_model
                    .as_ref()
                    .map(|model| SerializedLanguageModel {
                        provider: model.provider.id().0.to_string(),
                        model: model.model.id().0.to_string(),
                    }),
                completion_mode: Some(this.completion_mode),
                tool_use_limit_reached: this.tool_use_limit_reached,
                profile: Some(this.profile.id().clone()),
            })
        })
    }

    pub fn remaining_turns(&self) -> u32 {
        self.remaining_turns
    }

    pub fn set_remaining_turns(&mut self, remaining_turns: u32) {
        self.remaining_turns = remaining_turns;
    }

    pub fn send_to_model(
        &mut self,
        model: Arc<dyn LanguageModel>,
        intent: CompletionIntent,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Self>,
    ) {
        if self.remaining_turns == 0 {
            return;
        }

        self.remaining_turns -= 1;

        self.flush_notifications(model.clone(), intent, cx);

        let _checkpoint = self.finalize_pending_checkpoint(cx);
        self.stream_completion(
            self.to_completion_request(model.clone(), intent, cx),
            model,
            intent,
            window,
            cx,
        );
    }

    pub fn to_completion_request(
        &self,
        model: Arc<dyn LanguageModel>,
        intent: CompletionIntent,
        cx: &mut Context<Self>,
    ) -> LanguageModelRequest {
        let mut request = LanguageModelRequest {
            thread_id: Some(self.id.to_string()),
            prompt_id: Some(self.last_prompt_id.to_string()),
            intent: Some(intent),
            mode: None,
            messages: vec![],
            tools: Vec::new(),
            tool_choice: None,
            stop: Vec::new(),
            temperature: AgentSettings::temperature_for_model(&model, cx),
            thinking_allowed: true,
        };

        let available_tools = self.available_tools(cx, model.clone());
        let available_tool_names = available_tools
            .iter()
            .map(|tool| tool.name.clone())
            .collect();

        let model_context = &ModelContext {
            available_tools: available_tool_names,
        };

        if let Some(project_context) = self.project_context.borrow().as_ref() {
            match self
                .prompt_builder
                .generate_assistant_system_prompt(project_context, model_context)
            {
                Err(err) => {
                    let message = format!("{err:?}").into();
                    log::error!("{message}");
                    cx.emit(ThreadEvent::ShowError(ThreadError::Message {
                        header: "Error generating system prompt".into(),
                        message,
                    }));
                }
                Ok(system_prompt) => {
                    request.messages.push(LanguageModelRequestMessage {
                        role: Role::System,
                        content: vec![MessageContent::Text(system_prompt)],
                        cache: true,
                    });
                }
            }
        } else {
            let message = "Context for system prompt unexpectedly not ready.".into();
            log::error!("{message}");
            cx.emit(ThreadEvent::ShowError(ThreadError::Message {
                header: "Error generating system prompt".into(),
                message,
            }));
        }

        let mut message_ix_to_cache = None;
        for message in &self.messages {
            // ui_only messages are for the UI only, not for the model
            if message.ui_only {
                continue;
            }

            let mut request_message = LanguageModelRequestMessage {
                role: message.role,
                content: Vec::new(),
                cache: false,
            };

            message
                .loaded_context
                .add_to_request_message(&mut request_message);

            for segment in &message.segments {
                match segment {
                    MessageSegment::Text(text) => {
                        let text = text.trim_end();
                        if !text.is_empty() {
                            request_message
                                .content
                                .push(MessageContent::Text(text.into()));
                        }
                    }
                    MessageSegment::Thinking { text, signature } => {
                        if !text.is_empty() {
                            request_message.content.push(MessageContent::Thinking {
                                text: text.into(),
                                signature: signature.clone(),
                            });
                        }
                    }
                    MessageSegment::RedactedThinking(data) => {
                        request_message
                            .content
                            .push(MessageContent::RedactedThinking(data.clone()));
                    }
                };
            }

            let mut cache_message = true;
            let mut tool_results_message = LanguageModelRequestMessage {
                role: Role::User,
                content: Vec::new(),
                cache: false,
            };
            for (tool_use, tool_result) in self.tool_use.tool_results(message.id) {
                if let Some(tool_result) = tool_result {
                    request_message
                        .content
                        .push(MessageContent::ToolUse(tool_use.clone()));
                    tool_results_message
                        .content
                        .push(MessageContent::ToolResult(LanguageModelToolResult {
                            tool_use_id: tool_use.id.clone(),
                            tool_name: tool_result.tool_name.clone(),
                            is_error: tool_result.is_error,
                            content: if tool_result.content.is_empty() {
                                // Surprisingly, the API fails if we return an empty string here.
                                // It thinks we are sending a tool use without a tool result.
                                "<Tool returned an empty string>".into()
                            } else {
                                tool_result.content.clone()
                            },
                            output: None,
                        }));
                } else {
                    cache_message = false;
                    log::debug!(
                        "skipped tool use {:?} because it is still pending",
                        tool_use
                    );
                }
            }

            if cache_message {
                message_ix_to_cache = Some(request.messages.len());
            }
            request.messages.push(request_message);

            if !tool_results_message.content.is_empty() {
                if cache_message {
                    message_ix_to_cache = Some(request.messages.len());
                }
                request.messages.push(tool_results_message);
            }
        }

        // https://docs.anthropic.com/en/docs/build-with-claude/prompt-caching
        if let Some(message_ix_to_cache) = message_ix_to_cache {
            request.messages[message_ix_to_cache].cache = true;
        }

        request.tools = available_tools;
        request.mode = if model.supports_burn_mode() {
            Some(self.completion_mode.into())
        } else {
            Some(CompletionMode::Normal.into())
        };

        request
    }

    fn to_summarize_request(
        &self,
        model: &Arc<dyn LanguageModel>,
        intent: CompletionIntent,
        added_user_message: String,
        cx: &App,
    ) -> LanguageModelRequest {
        let mut request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            intent: Some(intent),
            mode: None,
            messages: vec![],
            tools: Vec::new(),
            tool_choice: None,
            stop: Vec::new(),
            temperature: AgentSettings::temperature_for_model(model, cx),
            thinking_allowed: false,
        };

        for message in &self.messages {
            let mut request_message = LanguageModelRequestMessage {
                role: message.role,
                content: Vec::new(),
                cache: false,
            };

            for segment in &message.segments {
                match segment {
                    MessageSegment::Text(text) => request_message
                        .content
                        .push(MessageContent::Text(text.clone())),
                    MessageSegment::Thinking { .. } => {}
                    MessageSegment::RedactedThinking(_) => {}
                }
            }

            if request_message.content.is_empty() {
                continue;
            }

            request.messages.push(request_message);
        }

        request.messages.push(LanguageModelRequestMessage {
            role: Role::User,
            content: vec![MessageContent::Text(added_user_message)],
            cache: false,
        });

        request
    }

    /// Insert auto-generated notifications (if any) to the thread
    fn flush_notifications(
        &mut self,
        model: Arc<dyn LanguageModel>,
        intent: CompletionIntent,
        cx: &mut Context<Self>,
    ) {
        match intent {
            CompletionIntent::UserPrompt | CompletionIntent::ToolResults => {
                if let Some(pending_tool_use) = self.attach_tracked_files_state(model, cx) {
                    cx.emit(ThreadEvent::ToolFinished {
                        tool_use_id: pending_tool_use.id.clone(),
                        pending_tool_use: Some(pending_tool_use),
                    });
                }
            }
            CompletionIntent::ThreadSummarization
            | CompletionIntent::ThreadContextSummarization
            | CompletionIntent::CreateFile
            | CompletionIntent::EditFile
            | CompletionIntent::InlineAssist
            | CompletionIntent::TerminalInlineAssist
            | CompletionIntent::GenerateGitCommitMessage => {}
        };
    }

    fn attach_tracked_files_state(
        &mut self,
        model: Arc<dyn LanguageModel>,
        cx: &mut App,
    ) -> Option<PendingToolUse> {
        // Represent notification as a simulated `project_notifications` tool call
        let tool_name = Arc::from("project_notifications");
        let tool = self.tools.read(cx).tool(&tool_name, cx)?;

        if !self.profile.is_tool_enabled(tool.source(), tool.name(), cx) {
            return None;
        }

        if self
            .action_log
            .update(cx, |log, cx| log.unnotified_user_edits(cx).is_none())
        {
            return None;
        }

        let input = serde_json::json!({});
        let request = Arc::new(LanguageModelRequest::default()); // unused
        let window = None;
        let tool_result = tool.run(
            input,
            request,
            self.project.clone(),
            self.action_log.clone(),
            model.clone(),
            window,
            cx,
        );

        let tool_use_id =
            LanguageModelToolUseId::from(format!("project_notifications_{}", self.messages.len()));

        let tool_use = LanguageModelToolUse {
            id: tool_use_id.clone(),
            name: tool_name.clone(),
            raw_input: "{}".to_string(),
            input: serde_json::json!({}),
            is_input_complete: true,
        };

        let tool_output = cx.background_executor().block(tool_result.output);

        // Attach a project_notification tool call to the latest existing
        // Assistant message. We cannot create a new Assistant message
        // because thinking models require a `thinking` block that we
        // cannot mock. We cannot send a notification as a normal
        // (non-tool-use) User message because this distracts Agent
        // too much.
        let tool_message_id = self
            .messages
            .iter()
            .enumerate()
            .rfind(|(_, message)| message.role == Role::Assistant)
            .map(|(_, message)| message.id)?;

        let tool_use_metadata = ToolUseMetadata {
            model: model.clone(),
            thread_id: self.id.clone(),
            prompt_id: self.last_prompt_id.clone(),
        };

        self.tool_use
            .request_tool_use(tool_message_id, tool_use, tool_use_metadata, cx);

        self.tool_use.insert_tool_output(
            tool_use_id,
            tool_name,
            tool_output,
            self.configured_model.as_ref(),
            self.completion_mode,
        )
    }

    pub fn stream_completion(
        &mut self,
        request: LanguageModelRequest,
        model: Arc<dyn LanguageModel>,
        intent: CompletionIntent,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Self>,
    ) {
        self.tool_use_limit_reached = false;

        let pending_completion_id = post_inc(&mut self.completion_count);
        let mut request_callback_parameters = if self.request_callback.is_some() {
            Some((request.clone(), Vec::new()))
        } else {
            None
        };
        let prompt_id = self.last_prompt_id.clone();
        let tool_use_metadata = ToolUseMetadata {
            model: model.clone(),
            thread_id: self.id.clone(),
            prompt_id: prompt_id.clone(),
        };

        let completion_mode = request
            .mode
            .unwrap_or(cloud_llm_client::CompletionMode::Normal);

        self.last_received_chunk_at = Some(Instant::now());

        let task = cx.spawn(async move |thread, cx| {
            let stream_completion_future = model.stream_completion(request, cx);
            let initial_token_usage =
                thread.read_with(cx, |thread, _cx| thread.cumulative_token_usage);
            let stream_completion = async {
                let mut events = stream_completion_future.await?;

                let mut stop_reason = StopReason::EndTurn;
                let mut current_token_usage = TokenUsage::default();

                thread
                    .update(cx, |_thread, cx| {
                        cx.emit(ThreadEvent::NewRequest);
                    })
                    .ok();

                let mut request_assistant_message_id = None;

                while let Some(event) = events.next().await {
                    if let Some((_, response_events)) = request_callback_parameters.as_mut() {
                        response_events
                            .push(event.as_ref().map_err(|error| error.to_string()).cloned());
                    }

                    thread.update(cx, |thread, cx| {
                        match event? {
                            LanguageModelCompletionEvent::StartMessage { .. } => {
                                request_assistant_message_id =
                                    Some(thread.insert_assistant_message(
                                        vec![MessageSegment::Text(String::new())],
                                        cx,
                                    ));
                            }
                            LanguageModelCompletionEvent::Stop(reason) => {
                                stop_reason = reason;
                            }
                            LanguageModelCompletionEvent::UsageUpdate(token_usage) => {
                                thread.update_token_usage_at_last_message(token_usage);
                                thread.cumulative_token_usage = thread.cumulative_token_usage
                                    + token_usage
                                    - current_token_usage;
                                current_token_usage = token_usage;
                            }
                            LanguageModelCompletionEvent::Text(chunk) => {
                                thread.received_chunk();

                                cx.emit(ThreadEvent::ReceivedTextChunk);
                                if let Some(last_message) = thread.messages.last_mut() {
                                    if last_message.role == Role::Assistant
                                        && !thread.tool_use.has_tool_results(last_message.id)
                                    {
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
                                        request_assistant_message_id =
                                            Some(thread.insert_assistant_message(
                                                vec![MessageSegment::Text(chunk.to_string())],
                                                cx,
                                            ));
                                    };
                                }
                            }
                            LanguageModelCompletionEvent::Thinking {
                                text: chunk,
                                signature,
                            } => {
                                thread.received_chunk();

                                if let Some(last_message) = thread.messages.last_mut() {
                                    if last_message.role == Role::Assistant
                                        && !thread.tool_use.has_tool_results(last_message.id)
                                    {
                                        last_message.push_thinking(&chunk, signature);
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
                                        request_assistant_message_id =
                                            Some(thread.insert_assistant_message(
                                                vec![MessageSegment::Thinking {
                                                    text: chunk.to_string(),
                                                    signature,
                                                }],
                                                cx,
                                            ));
                                    };
                                }
                            }
                            LanguageModelCompletionEvent::RedactedThinking { data } => {
                                thread.received_chunk();

                                if let Some(last_message) = thread.messages.last_mut() {
                                    if last_message.role == Role::Assistant
                                        && !thread.tool_use.has_tool_results(last_message.id)
                                    {
                                        last_message.push_redacted_thinking(data);
                                    } else {
                                        request_assistant_message_id =
                                            Some(thread.insert_assistant_message(
                                                vec![MessageSegment::RedactedThinking(data)],
                                                cx,
                                            ));
                                    };
                                }
                            }
                            LanguageModelCompletionEvent::ToolUse(tool_use) => {
                                let last_assistant_message_id = request_assistant_message_id
                                    .unwrap_or_else(|| {
                                        let new_assistant_message_id =
                                            thread.insert_assistant_message(vec![], cx);
                                        request_assistant_message_id =
                                            Some(new_assistant_message_id);
                                        new_assistant_message_id
                                    });

                                let tool_use_id = tool_use.id.clone();
                                let streamed_input = if tool_use.is_input_complete {
                                    None
                                } else {
                                    Some(tool_use.input.clone())
                                };

                                let ui_text = thread.tool_use.request_tool_use(
                                    last_assistant_message_id,
                                    tool_use,
                                    tool_use_metadata.clone(),
                                    cx,
                                );

                                if let Some(input) = streamed_input {
                                    cx.emit(ThreadEvent::StreamedToolUse {
                                        tool_use_id,
                                        ui_text,
                                        input,
                                    });
                                }
                            }
                            LanguageModelCompletionEvent::ToolUseJsonParseError {
                                id,
                                tool_name,
                                raw_input: invalid_input_json,
                                json_parse_error,
                            } => {
                                thread.receive_invalid_tool_json(
                                    id,
                                    tool_name,
                                    invalid_input_json,
                                    json_parse_error,
                                    window,
                                    cx,
                                );
                            }
                            LanguageModelCompletionEvent::StatusUpdate(status_update) => {
                                if let Some(completion) = thread
                                    .pending_completions
                                    .iter_mut()
                                    .find(|completion| completion.id == pending_completion_id)
                                {
                                    match status_update {
                                        CompletionRequestStatus::Queued { position } => {
                                            completion.queue_state =
                                                QueueState::Queued { position };
                                        }
                                        CompletionRequestStatus::Started => {
                                            completion.queue_state = QueueState::Started;
                                        }
                                        CompletionRequestStatus::Failed {
                                            code,
                                            message,
                                            request_id: _,
                                            retry_after,
                                        } => {
                                            return Err(
                                                LanguageModelCompletionError::from_cloud_failure(
                                                    model.upstream_provider_name(),
                                                    code,
                                                    message,
                                                    retry_after.map(Duration::from_secs_f64),
                                                ),
                                            );
                                        }
                                        CompletionRequestStatus::UsageUpdated { amount, limit } => {
                                            thread.update_model_request_usage(
                                                amount as u32,
                                                limit,
                                                cx,
                                            );
                                        }
                                        CompletionRequestStatus::ToolUseLimitReached => {
                                            thread.tool_use_limit_reached = true;
                                            cx.emit(ThreadEvent::ToolUseLimitReached);
                                        }
                                    }
                                }
                            }
                        }

                        thread.touch_updated_at();
                        cx.emit(ThreadEvent::StreamedCompletion);
                        cx.notify();

                        Ok(())
                    })??;

                    smol::future::yield_now().await;
                }

                thread.update(cx, |thread, cx| {
                    thread.last_received_chunk_at = None;
                    thread
                        .pending_completions
                        .retain(|completion| completion.id != pending_completion_id);

                    // If there is a response without tool use, summarize the message. Otherwise,
                    // allow two tool uses before summarizing.
                    if matches!(thread.summary, ThreadSummary::Pending)
                        && thread.messages.len() >= 2
                        && (!thread.has_pending_tool_uses() || thread.messages.len() >= 6)
                    {
                        thread.summarize(cx);
                    }
                })?;

                anyhow::Ok(stop_reason)
            };

            let result = stream_completion.await;
            let mut retry_scheduled = false;

            thread
                .update(cx, |thread, cx| {
                    thread.finalize_pending_checkpoint(cx);
                    match result.as_ref() {
                        Ok(stop_reason) => {
                            match stop_reason {
                                StopReason::ToolUse => {
                                    let tool_uses =
                                        thread.use_pending_tools(window, model.clone(), cx);
                                    cx.emit(ThreadEvent::UsePendingTools { tool_uses });
                                }
                                StopReason::EndTurn | StopReason::MaxTokens => {
                                    thread.project.update(cx, |project, cx| {
                                        project.set_agent_location(None, cx);
                                    });
                                }
                                StopReason::Refusal => {
                                    thread.project.update(cx, |project, cx| {
                                        project.set_agent_location(None, cx);
                                    });

                                    // Remove the turn that was refused.
                                    //
                                    // https://docs.anthropic.com/en/docs/test-and-evaluate/strengthen-guardrails/handle-streaming-refusals#reset-context-after-refusal
                                    {
                                        let mut messages_to_remove = Vec::new();

                                        for (ix, message) in
                                            thread.messages.iter().enumerate().rev()
                                        {
                                            messages_to_remove.push(message.id);

                                            if message.role == Role::User {
                                                if ix == 0 {
                                                    break;
                                                }

                                                if let Some(prev_message) =
                                                    thread.messages.get(ix - 1)
                                                    && prev_message.role == Role::Assistant {
                                                        break;
                                                    }
                                            }
                                        }

                                        for message_id in messages_to_remove {
                                            thread.delete_message(message_id, cx);
                                        }
                                    }

                                    cx.emit(ThreadEvent::ShowError(ThreadError::Message {
                                        header: "Language model refusal".into(),
                                        message:
                                            "Model refused to generate content for safety reasons."
                                                .into(),
                                    }));
                                }
                            }

                            // We successfully completed, so cancel any remaining retries.
                            thread.retry_state = None;
                        }
                        Err(error) => {
                            thread.project.update(cx, |project, cx| {
                                project.set_agent_location(None, cx);
                            });

                            if error.is::<PaymentRequiredError>() {
                                cx.emit(ThreadEvent::ShowError(ThreadError::PaymentRequired));
                            } else if let Some(error) =
                                error.downcast_ref::<ModelRequestLimitReachedError>()
                            {
                                cx.emit(ThreadEvent::ShowError(
                                    ThreadError::ModelRequestLimitReached { plan: error.plan },
                                ));
                            } else if let Some(completion_error) =
                                error.downcast_ref::<LanguageModelCompletionError>()
                            {
                                match &completion_error {
                                    LanguageModelCompletionError::PromptTooLarge {
                                        tokens, ..
                                    } => {
                                        let tokens = tokens.unwrap_or_else(|| {
                                            // We didn't get an exact token count from the API, so fall back on our estimate.
                                            thread
                                                .total_token_usage()
                                                .map(|usage| usage.total)
                                                .unwrap_or(0)
                                                // We know the context window was exceeded in practice, so if our estimate was
                                                // lower than max tokens, the estimate was wrong; return that we exceeded by 1.
                                                .max(
                                                    model
                                                        .max_token_count_for_mode(completion_mode)
                                                        .saturating_add(1),
                                                )
                                        });
                                        thread.exceeded_window_error = Some(ExceededWindowError {
                                            model_id: model.id(),
                                            token_count: tokens,
                                        });
                                        cx.notify();
                                    }
                                    _ => {
                                        if let Some(retry_strategy) =
                                            Thread::get_retry_strategy(completion_error)
                                        {
                                            log::info!(
                                                "Retrying with {:?} for language model completion error {:?}",
                                                retry_strategy,
                                                completion_error
                                            );

                                            retry_scheduled = thread
                                                .handle_retryable_error_with_delay(
                                                    completion_error,
                                                    Some(retry_strategy),
                                                    model.clone(),
                                                    intent,
                                                    window,
                                                    cx,
                                                );
                                        }
                                    }
                                }
                            }

                            if !retry_scheduled {
                                thread.cancel_last_completion(window, cx);
                            }
                        }
                    }

                    if !retry_scheduled {
                        cx.emit(ThreadEvent::Stopped(result.map_err(Arc::new)));
                    }

                    if let Some((request_callback, (request, response_events))) = thread
                        .request_callback
                        .as_mut()
                        .zip(request_callback_parameters.as_ref())
                    {
                        request_callback(request, response_events);
                    }

                    if let Ok(initial_usage) = initial_token_usage {
                        let usage = thread.cumulative_token_usage - initial_usage;

                        telemetry::event!(
                            "Assistant Thread Completion",
                            thread_id = thread.id().to_string(),
                            prompt_id = prompt_id,
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
            queue_state: QueueState::Sending,
            _task: task,
        });
    }

    pub fn summarize(&mut self, cx: &mut Context<Self>) {
        let Some(model) = LanguageModelRegistry::read_global(cx).thread_summary_model() else {
            println!("No thread summary model");
            return;
        };

        if !model.provider.is_authenticated(cx) {
            return;
        }

        let request = self.to_summarize_request(
            &model.model,
            CompletionIntent::ThreadSummarization,
            SUMMARIZE_THREAD_PROMPT.into(),
            cx,
        );

        self.summary = ThreadSummary::Generating;

        self.pending_summary = cx.spawn(async move |this, cx| {
            let result = async {
                let mut messages = model.model.stream_completion(request, cx).await?;

                let mut new_summary = String::new();
                while let Some(event) = messages.next().await {
                    let Ok(event) = event else {
                        continue;
                    };
                    let text = match event {
                        LanguageModelCompletionEvent::Text(text) => text,
                        LanguageModelCompletionEvent::StatusUpdate(
                            CompletionRequestStatus::UsageUpdated { amount, limit },
                        ) => {
                            this.update(cx, |thread, cx| {
                                thread.update_model_request_usage(amount as u32, limit, cx);
                            })?;
                            continue;
                        }
                        _ => continue,
                    };

                    let mut lines = text.lines();
                    new_summary.extend(lines.next());

                    // Stop if the LLM generated multiple lines.
                    if lines.next().is_some() {
                        break;
                    }
                }

                anyhow::Ok(new_summary)
            }
            .await;

            this.update(cx, |this, cx| {
                match result {
                    Ok(new_summary) => {
                        if new_summary.is_empty() {
                            this.summary = ThreadSummary::Error;
                        } else {
                            this.summary = ThreadSummary::Ready(new_summary.into());
                        }
                    }
                    Err(err) => {
                        this.summary = ThreadSummary::Error;
                        log::error!("Failed to generate thread summary: {}", err);
                    }
                }
                cx.emit(ThreadEvent::SummaryGenerated);
            })
            .log_err()?;

            Some(())
        });
    }

    fn get_retry_strategy(error: &LanguageModelCompletionError) -> Option<RetryStrategy> {
        use LanguageModelCompletionError::*;

        // General strategy here:
        // - If retrying won't help (e.g. invalid API key or payload too large), return None so we don't retry at all.
        // - If it's a time-based issue (e.g. server overloaded, rate limit exceeded), retry up to 4 times with exponential backoff.
        // - If it's an issue that *might* be fixed by retrying (e.g. internal server error), retry up to 3 times.
        match error {
            HttpResponseError {
                status_code: StatusCode::TOO_MANY_REQUESTS,
                ..
            } => Some(RetryStrategy::ExponentialBackoff {
                initial_delay: BASE_RETRY_DELAY,
                max_attempts: MAX_RETRY_ATTEMPTS,
            }),
            ServerOverloaded { retry_after, .. } | RateLimitExceeded { retry_after, .. } => {
                Some(RetryStrategy::Fixed {
                    delay: retry_after.unwrap_or(BASE_RETRY_DELAY),
                    max_attempts: MAX_RETRY_ATTEMPTS,
                })
            }
            UpstreamProviderError {
                status,
                retry_after,
                ..
            } => match *status {
                StatusCode::TOO_MANY_REQUESTS | StatusCode::SERVICE_UNAVAILABLE => {
                    Some(RetryStrategy::Fixed {
                        delay: retry_after.unwrap_or(BASE_RETRY_DELAY),
                        max_attempts: MAX_RETRY_ATTEMPTS,
                    })
                }
                StatusCode::INTERNAL_SERVER_ERROR => Some(RetryStrategy::Fixed {
                    delay: retry_after.unwrap_or(BASE_RETRY_DELAY),
                    // Internal Server Error could be anything, retry up to 3 times.
                    max_attempts: 3,
                }),
                status => {
                    // There is no StatusCode variant for the unofficial HTTP 529 ("The service is overloaded"),
                    // but we frequently get them in practice. See https://http.dev/529
                    if status.as_u16() == 529 {
                        Some(RetryStrategy::Fixed {
                            delay: retry_after.unwrap_or(BASE_RETRY_DELAY),
                            max_attempts: MAX_RETRY_ATTEMPTS,
                        })
                    } else {
                        Some(RetryStrategy::Fixed {
                            delay: retry_after.unwrap_or(BASE_RETRY_DELAY),
                            max_attempts: 2,
                        })
                    }
                }
            },
            ApiInternalServerError { .. } => Some(RetryStrategy::Fixed {
                delay: BASE_RETRY_DELAY,
                max_attempts: 3,
            }),
            ApiReadResponseError { .. }
            | HttpSend { .. }
            | DeserializeResponse { .. }
            | BadRequestFormat { .. } => Some(RetryStrategy::Fixed {
                delay: BASE_RETRY_DELAY,
                max_attempts: 3,
            }),
            // Retrying these errors definitely shouldn't help.
            HttpResponseError {
                status_code:
                    StatusCode::PAYLOAD_TOO_LARGE | StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED,
                ..
            }
            | AuthenticationError { .. }
            | PermissionError { .. }
            | NoApiKey { .. }
            | ApiEndpointNotFound { .. }
            | PromptTooLarge { .. } => None,
            // These errors might be transient, so retry them
            SerializeRequest { .. } | BuildRequestBody { .. } => Some(RetryStrategy::Fixed {
                delay: BASE_RETRY_DELAY,
                max_attempts: 1,
            }),
            // Retry all other 4xx and 5xx errors once.
            HttpResponseError { status_code, .. }
                if status_code.is_client_error() || status_code.is_server_error() =>
            {
                Some(RetryStrategy::Fixed {
                    delay: BASE_RETRY_DELAY,
                    max_attempts: 3,
                })
            }
            Other(err)
                if err.is::<PaymentRequiredError>()
                    || err.is::<ModelRequestLimitReachedError>() =>
            {
                // Retrying won't help for Payment Required or Model Request Limit errors (where
                // the user must upgrade to usage-based billing to get more requests, or else wait
                // for a significant amount of time for the request limit to reset).
                None
            }
            // Conservatively assume that any other errors are non-retryable
            HttpResponseError { .. } | Other(..) => Some(RetryStrategy::Fixed {
                delay: BASE_RETRY_DELAY,
                max_attempts: 2,
            }),
        }
    }

    fn handle_retryable_error_with_delay(
        &mut self,
        error: &LanguageModelCompletionError,
        strategy: Option<RetryStrategy>,
        model: Arc<dyn LanguageModel>,
        intent: CompletionIntent,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Self>,
    ) -> bool {
        // Store context for the Retry button
        self.last_error_context = Some((model.clone(), intent));

        // Only auto-retry if Burn Mode is enabled
        if self.completion_mode != CompletionMode::Burn {
            // Show error with retry options
            cx.emit(ThreadEvent::ShowError(ThreadError::RetryableError {
                message: format!(
                    "{}\n\nTo automatically retry when similar errors happen, enable Burn Mode.",
                    error
                )
                .into(),
                can_enable_burn_mode: true,
            }));
            return false;
        }

        let Some(strategy) = strategy.or_else(|| Self::get_retry_strategy(error)) else {
            return false;
        };

        let max_attempts = match &strategy {
            RetryStrategy::ExponentialBackoff { max_attempts, .. } => *max_attempts,
            RetryStrategy::Fixed { max_attempts, .. } => *max_attempts,
        };

        let retry_state = self.retry_state.get_or_insert(RetryState {
            attempt: 0,
            max_attempts,
            intent,
        });

        retry_state.attempt += 1;
        let attempt = retry_state.attempt;
        let max_attempts = retry_state.max_attempts;
        let intent = retry_state.intent;

        if attempt <= max_attempts {
            let delay = match &strategy {
                RetryStrategy::ExponentialBackoff { initial_delay, .. } => {
                    let delay_secs = initial_delay.as_secs() * 2u64.pow((attempt - 1) as u32);
                    Duration::from_secs(delay_secs)
                }
                RetryStrategy::Fixed { delay, .. } => *delay,
            };

            // Add a transient message to inform the user
            let delay_secs = delay.as_secs();
            let retry_message = if max_attempts == 1 {
                format!("{error}. Retrying in {delay_secs} seconds...")
            } else {
                format!(
                    "{error}. Retrying (attempt {attempt} of {max_attempts}) \
                    in {delay_secs} seconds..."
                )
            };
            log::warn!(
                "Retrying completion request (attempt {attempt} of {max_attempts}) \
                in {delay_secs} seconds: {error:?}",
            );

            // Add a UI-only message instead of a regular message
            let id = self.next_message_id.post_inc();
            self.messages.push(Message {
                id,
                role: Role::System,
                segments: vec![MessageSegment::Text(retry_message)],
                loaded_context: LoadedContext::default(),
                creases: Vec::new(),
                is_hidden: false,
                ui_only: true,
            });
            cx.emit(ThreadEvent::MessageAdded(id));

            // Schedule the retry
            let thread_handle = cx.entity().downgrade();

            cx.spawn(async move |_thread, cx| {
                cx.background_executor().timer(delay).await;

                thread_handle
                    .update(cx, |thread, cx| {
                        // Retry the completion
                        thread.send_to_model(model, intent, window, cx);
                    })
                    .log_err();
            })
            .detach();

            true
        } else {
            // Max retries exceeded
            self.retry_state = None;

            // Stop generating since we're giving up on retrying.
            self.pending_completions.clear();

            // Show error alongside a Retry button, but no
            // Enable Burn Mode button (since it's already enabled)
            cx.emit(ThreadEvent::ShowError(ThreadError::RetryableError {
                message: format!("Failed after retrying: {}", error).into(),
                can_enable_burn_mode: false,
            }));

            false
        }
    }

    pub fn start_generating_detailed_summary_if_needed(
        &mut self,
        thread_store: WeakEntity<ThreadStore>,
        cx: &mut Context<Self>,
    ) {
        let Some(last_message_id) = self.messages.last().map(|message| message.id) else {
            return;
        };

        match &*self.detailed_summary_rx.borrow() {
            DetailedSummaryState::Generating { message_id, .. }
            | DetailedSummaryState::Generated { message_id, .. }
                if *message_id == last_message_id =>
            {
                // Already up-to-date
                return;
            }
            _ => {}
        }

        let Some(ConfiguredModel { model, provider }) =
            LanguageModelRegistry::read_global(cx).thread_summary_model()
        else {
            return;
        };

        if !provider.is_authenticated(cx) {
            return;
        }

        let request = self.to_summarize_request(
            &model,
            CompletionIntent::ThreadContextSummarization,
            SUMMARIZE_THREAD_DETAILED_PROMPT.into(),
            cx,
        );

        *self.detailed_summary_tx.borrow_mut() = DetailedSummaryState::Generating {
            message_id: last_message_id,
        };

        // Replace the detailed summarization task if there is one, cancelling it. It would probably
        // be better to allow the old task to complete, but this would require logic for choosing
        // which result to prefer (the old task could complete after the new one, resulting in a
        // stale summary).
        self.detailed_summary_task = cx.spawn(async move |thread, cx| {
            let stream = model.stream_completion_text(request, cx);
            let Some(mut messages) = stream.await.log_err() else {
                thread
                    .update(cx, |thread, _cx| {
                        *thread.detailed_summary_tx.borrow_mut() =
                            DetailedSummaryState::NotGenerated;
                    })
                    .ok()?;
                return None;
            };

            let mut new_detailed_summary = String::new();

            while let Some(chunk) = messages.stream.next().await {
                if let Some(chunk) = chunk.log_err() {
                    new_detailed_summary.push_str(&chunk);
                }
            }

            thread
                .update(cx, |thread, _cx| {
                    *thread.detailed_summary_tx.borrow_mut() = DetailedSummaryState::Generated {
                        text: new_detailed_summary.into(),
                        message_id: last_message_id,
                    };
                })
                .ok()?;

            // Save thread so its summary can be reused later
            if let Some(thread) = thread.upgrade()
                && let Ok(Ok(save_task)) = cx.update(|cx| {
                    thread_store
                        .update(cx, |thread_store, cx| thread_store.save_thread(&thread, cx))
                })
            {
                save_task.await.log_err();
            }

            Some(())
        });
    }

    pub async fn wait_for_detailed_summary_or_text(
        this: &Entity<Self>,
        cx: &mut AsyncApp,
    ) -> Option<SharedString> {
        let mut detailed_summary_rx = this
            .read_with(cx, |this, _cx| this.detailed_summary_rx.clone())
            .ok()?;
        loop {
            match detailed_summary_rx.recv().await? {
                DetailedSummaryState::Generating { .. } => {}
                DetailedSummaryState::NotGenerated => {
                    return this.read_with(cx, |this, _cx| this.text().into()).ok();
                }
                DetailedSummaryState::Generated { text, .. } => return Some(text),
            }
        }
    }

    pub fn latest_detailed_summary_or_text(&self) -> SharedString {
        self.detailed_summary_rx
            .borrow()
            .text()
            .unwrap_or_else(|| self.text().into())
    }

    pub fn is_generating_detailed_summary(&self) -> bool {
        matches!(
            &*self.detailed_summary_rx.borrow(),
            DetailedSummaryState::Generating { .. }
        )
    }

    pub fn use_pending_tools(
        &mut self,
        window: Option<AnyWindowHandle>,
        model: Arc<dyn LanguageModel>,
        cx: &mut Context<Self>,
    ) -> Vec<PendingToolUse> {
        let request =
            Arc::new(self.to_completion_request(model.clone(), CompletionIntent::ToolResults, cx));
        let pending_tool_uses = self
            .tool_use
            .pending_tool_uses()
            .into_iter()
            .filter(|tool_use| tool_use.status.is_idle())
            .cloned()
            .collect::<Vec<_>>();

        for tool_use in pending_tool_uses.iter() {
            self.use_pending_tool(tool_use.clone(), request.clone(), model.clone(), window, cx);
        }

        pending_tool_uses
    }

    fn use_pending_tool(
        &mut self,
        tool_use: PendingToolUse,
        request: Arc<LanguageModelRequest>,
        model: Arc<dyn LanguageModel>,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Self>,
    ) {
        let Some(tool) = self.tools.read(cx).tool(&tool_use.name, cx) else {
            return self.handle_hallucinated_tool_use(tool_use.id, tool_use.name, window, cx);
        };

        if !self.profile.is_tool_enabled(tool.source(), tool.name(), cx) {
            return self.handle_hallucinated_tool_use(tool_use.id, tool_use.name, window, cx);
        }

        if tool.needs_confirmation(&tool_use.input, &self.project, cx)
            && !AgentSettings::get_global(cx).always_allow_tool_actions
        {
            self.tool_use.confirm_tool_use(
                tool_use.id,
                tool_use.ui_text,
                tool_use.input,
                request,
                tool,
            );
            cx.emit(ThreadEvent::ToolConfirmationNeeded);
        } else {
            self.run_tool(
                tool_use.id,
                tool_use.ui_text,
                tool_use.input,
                request,
                tool,
                model,
                window,
                cx,
            );
        }
    }

    pub fn handle_hallucinated_tool_use(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        hallucinated_tool_name: Arc<str>,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Thread>,
    ) {
        let available_tools = self.profile.enabled_tools(cx);

        let tool_list = available_tools
            .iter()
            .map(|(name, tool)| format!("- {}: {}", name, tool.description()))
            .collect::<Vec<_>>()
            .join("\n");

        let error_message = format!(
            "The tool '{}' doesn't exist or is not enabled. Available tools:\n{}",
            hallucinated_tool_name, tool_list
        );

        let pending_tool_use = self.tool_use.insert_tool_output(
            tool_use_id.clone(),
            hallucinated_tool_name,
            Err(anyhow!("Missing tool call: {error_message}")),
            self.configured_model.as_ref(),
            self.completion_mode,
        );

        cx.emit(ThreadEvent::MissingToolUse {
            tool_use_id: tool_use_id.clone(),
            ui_text: error_message.into(),
        });

        self.tool_finished(tool_use_id, pending_tool_use, false, window, cx);
    }

    pub fn receive_invalid_tool_json(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        invalid_json: Arc<str>,
        error: String,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Thread>,
    ) {
        log::error!("The model returned invalid input JSON: {invalid_json}");

        let pending_tool_use = self.tool_use.insert_tool_output(
            tool_use_id.clone(),
            tool_name,
            Err(anyhow!("Error parsing input JSON: {error}")),
            self.configured_model.as_ref(),
            self.completion_mode,
        );
        let ui_text = if let Some(pending_tool_use) = &pending_tool_use {
            pending_tool_use.ui_text.clone()
        } else {
            log::error!(
                "There was no pending tool use for tool use {tool_use_id}, even though it finished (with invalid input JSON)."
            );
            format!("Unknown tool {}", tool_use_id).into()
        };

        cx.emit(ThreadEvent::InvalidToolInput {
            tool_use_id: tool_use_id.clone(),
            ui_text,
            invalid_input_json: invalid_json,
        });

        self.tool_finished(tool_use_id, pending_tool_use, false, window, cx);
    }

    pub fn run_tool(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        ui_text: impl Into<SharedString>,
        input: serde_json::Value,
        request: Arc<LanguageModelRequest>,
        tool: Arc<dyn Tool>,
        model: Arc<dyn LanguageModel>,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Thread>,
    ) {
        let task =
            self.spawn_tool_use(tool_use_id.clone(), request, input, tool, model, window, cx);
        self.tool_use
            .run_pending_tool(tool_use_id, ui_text.into(), task);
    }

    fn spawn_tool_use(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        request: Arc<LanguageModelRequest>,
        input: serde_json::Value,
        tool: Arc<dyn Tool>,
        model: Arc<dyn LanguageModel>,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Thread>,
    ) -> Task<()> {
        let tool_name: Arc<str> = tool.name().into();

        let tool_result = tool.run(
            input,
            request,
            self.project.clone(),
            self.action_log.clone(),
            model,
            window,
            cx,
        );

        // Store the card separately if it exists
        if let Some(card) = tool_result.card.clone() {
            self.tool_use
                .insert_tool_result_card(tool_use_id.clone(), card);
        }

        cx.spawn({
            async move |thread: WeakEntity<Thread>, cx| {
                let output = tool_result.output.await;

                thread
                    .update(cx, |thread, cx| {
                        let pending_tool_use = thread.tool_use.insert_tool_output(
                            tool_use_id.clone(),
                            tool_name,
                            output,
                            thread.configured_model.as_ref(),
                            thread.completion_mode,
                        );
                        thread.tool_finished(tool_use_id, pending_tool_use, false, window, cx);
                    })
                    .ok();
            }
        })
    }

    fn tool_finished(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        pending_tool_use: Option<PendingToolUse>,
        canceled: bool,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Self>,
    ) {
        if self.all_tools_finished()
            && let Some(ConfiguredModel { model, .. }) = self.configured_model.as_ref()
            && !canceled
        {
            self.send_to_model(model.clone(), CompletionIntent::ToolResults, window, cx);
        }

        cx.emit(ThreadEvent::ToolFinished {
            tool_use_id,
            pending_tool_use,
        });
    }

    /// Cancels the last pending completion, if there are any pending.
    ///
    /// Returns whether a completion was canceled.
    pub fn cancel_last_completion(
        &mut self,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Self>,
    ) -> bool {
        let mut canceled = self.pending_completions.pop().is_some() || self.retry_state.is_some();

        self.retry_state = None;

        for pending_tool_use in self.tool_use.cancel_pending() {
            canceled = true;
            self.tool_finished(
                pending_tool_use.id.clone(),
                Some(pending_tool_use),
                true,
                window,
                cx,
            );
        }

        if canceled {
            cx.emit(ThreadEvent::CompletionCanceled);

            // When canceled, we always want to insert the checkpoint.
            // (We skip over finalize_pending_checkpoint, because it
            // would conclude we didn't have anything to insert here.)
            if let Some(checkpoint) = self.pending_checkpoint.take() {
                self.insert_checkpoint(checkpoint, cx);
            }
        } else {
            self.finalize_pending_checkpoint(cx);
        }

        canceled
    }

    /// Signals that any in-progress editing should be canceled.
    ///
    /// This method is used to notify listeners (like ActiveThread) that
    /// they should cancel any editing operations.
    pub fn cancel_editing(&mut self, cx: &mut Context<Self>) {
        cx.emit(ThreadEvent::CancelEditing);
    }

    pub fn message_feedback(&self, message_id: MessageId) -> Option<ThreadFeedback> {
        self.message_feedback.get(&message_id).copied()
    }

    pub fn report_message_feedback(
        &mut self,
        message_id: MessageId,
        feedback: ThreadFeedback,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if self.message_feedback.get(&message_id) == Some(&feedback) {
            return Task::ready(Ok(()));
        }

        let final_project_snapshot = Self::project_snapshot(self.project.clone(), cx);
        let serialized_thread = self.serialize(cx);
        let thread_id = self.id().clone();
        let client = self.project.read(cx).client();

        let enabled_tool_names: Vec<String> = self
            .profile
            .enabled_tools(cx)
            .iter()
            .map(|(name, _)| name.clone().into())
            .collect();

        self.message_feedback.insert(message_id, feedback);

        cx.notify();

        let message_content = self
            .message(message_id)
            .map(|msg| msg.to_message_content())
            .unwrap_or_default();

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
                enabled_tool_names,
                message_id = message_id.0,
                message_content,
                thread_data,
                final_project_snapshot
            );
            client.telemetry().flush_events().await;

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

        cx.spawn(async move |_, _| {
            let worktree_snapshots = futures::future::join_all(worktree_snapshots).await;

            Arc::new(ProjectSnapshot {
                worktree_snapshots,
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
                let path = worktree.abs_path().to_string_lossy().into_owned();
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
                    repo.update(cx, |repo, _| {
                        let current_branch =
                            repo.branch.as_ref().map(|branch| branch.name().to_owned());
                        repo.send_job(None, |state, _| async move {
                            let RepositoryState::Local { backend, .. } = state else {
                                return GitState {
                                    remote_url: None,
                                    head_sha: None,
                                    current_branch,
                                    diff: None,
                                };
                            };

                            let remote_url = backend.remote_url("origin");
                            let head_sha = backend.head_sha().await;
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

        let summary = self.summary().or_default();
        writeln!(markdown, "# {summary}\n")?;

        for message in self.messages() {
            writeln!(
                markdown,
                "## {role}\n",
                role = match message.role {
                    Role::User => "User",
                    Role::Assistant => "Agent",
                    Role::System => "System",
                }
            )?;

            if !message.loaded_context.text.is_empty() {
                writeln!(markdown, "{}", message.loaded_context.text)?;
            }

            if !message.loaded_context.images.is_empty() {
                writeln!(
                    markdown,
                    "\n{} images attached as context.\n",
                    message.loaded_context.images.len()
                )?;
            }

            for segment in &message.segments {
                match segment {
                    MessageSegment::Text(text) => writeln!(markdown, "{}\n", text)?,
                    MessageSegment::Thinking { text, .. } => {
                        writeln!(markdown, "<think>\n{}\n</think>\n", text)?
                    }
                    MessageSegment::RedactedThinking(_) => {}
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
                write!(markdown, "\n**Tool Results: {}", tool_result.tool_use_id)?;
                if tool_result.is_error {
                    write!(markdown, " (Error)")?;
                }

                writeln!(markdown, "**\n")?;
                match &tool_result.content {
                    LanguageModelToolResultContent::Text(text) => {
                        writeln!(markdown, "{text}")?;
                    }
                    LanguageModelToolResultContent::Image(image) => {
                        writeln!(markdown, "![Image](data:base64,{})", image.source)?;
                    }
                }

                if let Some(output) = tool_result.output.as_ref() {
                    writeln!(
                        markdown,
                        "\n\nDebug Output:\n\n```json\n{}\n```\n",
                        serde_json::to_string_pretty(output)?
                    )?;
                }
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

    pub fn reject_edits_in_ranges(
        &mut self,
        buffer: Entity<language::Buffer>,
        buffer_ranges: Vec<Range<language::Anchor>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.action_log.update(cx, |action_log, cx| {
            action_log.reject_edits_in_ranges(buffer, buffer_ranges, cx)
        })
    }

    pub fn action_log(&self) -> &Entity<ActionLog> {
        &self.action_log
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    pub fn cumulative_token_usage(&self) -> TokenUsage {
        self.cumulative_token_usage
    }

    pub fn token_usage_up_to_message(&self, message_id: MessageId) -> TotalTokenUsage {
        let Some(model) = self.configured_model.as_ref() else {
            return TotalTokenUsage::default();
        };

        let max = model
            .model
            .max_token_count_for_mode(self.completion_mode().into());

        let index = self
            .messages
            .iter()
            .position(|msg| msg.id == message_id)
            .unwrap_or(0);

        if index == 0 {
            return TotalTokenUsage { total: 0, max };
        }

        let token_usage = &self
            .request_token_usage
            .get(index - 1)
            .cloned()
            .unwrap_or_default();

        TotalTokenUsage {
            total: token_usage.total_tokens(),
            max,
        }
    }

    pub fn total_token_usage(&self) -> Option<TotalTokenUsage> {
        let model = self.configured_model.as_ref()?;

        let max = model
            .model
            .max_token_count_for_mode(self.completion_mode().into());

        if let Some(exceeded_error) = &self.exceeded_window_error
            && model.model.id() == exceeded_error.model_id
        {
            return Some(TotalTokenUsage {
                total: exceeded_error.token_count,
                max,
            });
        }

        let total = self
            .token_usage_at_last_message()
            .unwrap_or_default()
            .total_tokens();

        Some(TotalTokenUsage { total, max })
    }

    fn token_usage_at_last_message(&self) -> Option<TokenUsage> {
        self.request_token_usage
            .get(self.messages.len().saturating_sub(1))
            .or_else(|| self.request_token_usage.last())
            .cloned()
    }

    fn update_token_usage_at_last_message(&mut self, token_usage: TokenUsage) {
        let placeholder = self.token_usage_at_last_message().unwrap_or_default();
        self.request_token_usage
            .resize(self.messages.len(), placeholder);

        if let Some(last) = self.request_token_usage.last_mut() {
            *last = token_usage;
        }
    }

    fn update_model_request_usage(&self, amount: u32, limit: UsageLimit, cx: &mut Context<Self>) {
        self.project
            .read(cx)
            .user_store()
            .update(cx, |user_store, cx| {
                user_store.update_model_request_usage(
                    ModelRequestUsage(RequestUsage {
                        amount: amount as i32,
                        limit,
                    }),
                    cx,
                )
            });
    }

    pub fn deny_tool_use(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Self>,
    ) {
        let err = Err(anyhow::anyhow!(
            "Permission to run tool action denied by user"
        ));

        self.tool_use.insert_tool_output(
            tool_use_id.clone(),
            tool_name,
            err,
            self.configured_model.as_ref(),
            self.completion_mode,
        );
        self.tool_finished(tool_use_id, None, true, window, cx);
    }
}

#[derive(Debug, Clone, Error)]
pub enum ThreadError {
    #[error("Payment required")]
    PaymentRequired,
    #[error("Model request limit reached")]
    ModelRequestLimitReached { plan: Plan },
    #[error("Message {header}: {message}")]
    Message {
        header: SharedString,
        message: SharedString,
    },
    #[error("Retryable error: {message}")]
    RetryableError {
        message: SharedString,
        can_enable_burn_mode: bool,
    },
}

#[derive(Debug, Clone)]
pub enum ThreadEvent {
    ShowError(ThreadError),
    StreamedCompletion,
    ReceivedTextChunk,
    NewRequest,
    StreamedAssistantText(MessageId, String),
    StreamedAssistantThinking(MessageId, String),
    StreamedToolUse {
        tool_use_id: LanguageModelToolUseId,
        ui_text: Arc<str>,
        input: serde_json::Value,
    },
    MissingToolUse {
        tool_use_id: LanguageModelToolUseId,
        ui_text: Arc<str>,
    },
    InvalidToolInput {
        tool_use_id: LanguageModelToolUseId,
        ui_text: Arc<str>,
        invalid_input_json: Arc<str>,
    },
    Stopped(Result<StopReason, Arc<anyhow::Error>>),
    MessageAdded(MessageId),
    MessageEdited(MessageId),
    MessageDeleted(MessageId),
    SummaryGenerated,
    SummaryChanged,
    UsePendingTools {
        tool_uses: Vec<PendingToolUse>,
    },
    ToolFinished {
        #[allow(unused)]
        tool_use_id: LanguageModelToolUseId,
        /// The pending tool use that corresponds to this tool.
        pending_tool_use: Option<PendingToolUse>,
    },
    CheckpointChanged,
    ToolConfirmationNeeded,
    ToolUseLimitReached,
    CancelEditing,
    CompletionCanceled,
    ProfileChanged,
}

impl EventEmitter<ThreadEvent> for Thread {}

struct PendingCompletion {
    id: usize,
    queue_state: QueueState,
    _task: Task<()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        context::load_context, context_store::ContextStore, thread_store, thread_store::ThreadStore,
    };

    // Test-specific constants
    const TEST_RATE_LIMIT_RETRY_SECS: u64 = 30;
    use agent_settings::{AgentProfileId, AgentSettings};
    use assistant_tool::ToolRegistry;
    use assistant_tools;
    use fs::Fs;
    use futures::StreamExt;
    use futures::future::BoxFuture;
    use futures::stream::BoxStream;
    use gpui::TestAppContext;
    use http_client;
    use language_model::fake_provider::{FakeLanguageModel, FakeLanguageModelProvider};
    use language_model::{
        LanguageModelCompletionError, LanguageModelName, LanguageModelProviderId,
        LanguageModelProviderName, LanguageModelToolChoice,
    };
    use parking_lot::Mutex;
    use project::{FakeFs, Project};
    use prompt_store::PromptBuilder;
    use serde_json::json;
    use settings::{LanguageModelParameters, Settings, SettingsStore};
    use std::sync::Arc;
    use std::time::Duration;
    use util::path;
    use workspace::Workspace;

    #[gpui::test]
    async fn test_message_with_context(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(
            &fs,
            cx,
            json!({"code.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"}),
        )
        .await;

        let (_workspace, _thread_store, thread, context_store, model) =
            setup_test_environment(cx, project.clone()).await;

        add_file_to_context(&project, &context_store, "test/code.rs", cx)
            .await
            .unwrap();

        let context =
            context_store.read_with(cx, |store, _| store.context().next().cloned().unwrap());
        let loaded_context = cx
            .update(|cx| load_context(vec![context], &project, &None, cx))
            .await;

        // Insert user message with context
        let message_id = thread.update(cx, |thread, cx| {
            thread.insert_user_message(
                "Please explain this code",
                loaded_context,
                None,
                Vec::new(),
                cx,
            )
        });

        // Check content and context in message object
        let message = thread.read_with(cx, |thread, _| thread.message(message_id).unwrap().clone());

        // Use different path format strings based on platform for the test
        #[cfg(windows)]
        let path_part = r"test\code.rs";
        #[cfg(not(windows))]
        let path_part = "test/code.rs";

        let expected_context = format!(
            r#"
<context>
The following items were attached by the user. They are up-to-date and don't need to be re-read.

<files>
```rs {path_part}
fn main() {{
    println!("Hello, world!");
}}
```
</files>
</context>
"#
        );

        assert_eq!(message.role, Role::User);
        assert_eq!(message.segments.len(), 1);
        assert_eq!(
            message.segments[0],
            MessageSegment::Text("Please explain this code".to_string())
        );
        assert_eq!(message.loaded_context.text, expected_context);

        // Check message in request
        let request = thread.update(cx, |thread, cx| {
            thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
        });

        assert_eq!(request.messages.len(), 2);
        let expected_full_message = format!("{}Please explain this code", expected_context);
        assert_eq!(request.messages[1].string_contents(), expected_full_message);
    }

    #[gpui::test]
    async fn test_only_include_new_contexts(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(
            &fs,
            cx,
            json!({
                "file1.rs": "fn function1() {}\n",
                "file2.rs": "fn function2() {}\n",
                "file3.rs": "fn function3() {}\n",
                "file4.rs": "fn function4() {}\n",
            }),
        )
        .await;

        let (_, _thread_store, thread, context_store, model) =
            setup_test_environment(cx, project.clone()).await;

        // First message with context 1
        add_file_to_context(&project, &context_store, "test/file1.rs", cx)
            .await
            .unwrap();
        let new_contexts = context_store.update(cx, |store, cx| {
            store.new_context_for_thread(thread.read(cx), None)
        });
        assert_eq!(new_contexts.len(), 1);
        let loaded_context = cx
            .update(|cx| load_context(new_contexts, &project, &None, cx))
            .await;
        let message1_id = thread.update(cx, |thread, cx| {
            thread.insert_user_message("Message 1", loaded_context, None, Vec::new(), cx)
        });

        // Second message with contexts 1 and 2 (context 1 should be skipped as it's already included)
        add_file_to_context(&project, &context_store, "test/file2.rs", cx)
            .await
            .unwrap();
        let new_contexts = context_store.update(cx, |store, cx| {
            store.new_context_for_thread(thread.read(cx), None)
        });
        assert_eq!(new_contexts.len(), 1);
        let loaded_context = cx
            .update(|cx| load_context(new_contexts, &project, &None, cx))
            .await;
        let message2_id = thread.update(cx, |thread, cx| {
            thread.insert_user_message("Message 2", loaded_context, None, Vec::new(), cx)
        });

        // Third message with all three contexts (contexts 1 and 2 should be skipped)
        //
        add_file_to_context(&project, &context_store, "test/file3.rs", cx)
            .await
            .unwrap();
        let new_contexts = context_store.update(cx, |store, cx| {
            store.new_context_for_thread(thread.read(cx), None)
        });
        assert_eq!(new_contexts.len(), 1);
        let loaded_context = cx
            .update(|cx| load_context(new_contexts, &project, &None, cx))
            .await;
        let message3_id = thread.update(cx, |thread, cx| {
            thread.insert_user_message("Message 3", loaded_context, None, Vec::new(), cx)
        });

        // Check what contexts are included in each message
        let (message1, message2, message3) = thread.read_with(cx, |thread, _| {
            (
                thread.message(message1_id).unwrap().clone(),
                thread.message(message2_id).unwrap().clone(),
                thread.message(message3_id).unwrap().clone(),
            )
        });

        // First message should include context 1
        assert!(message1.loaded_context.text.contains("file1.rs"));

        // Second message should include only context 2 (not 1)
        assert!(!message2.loaded_context.text.contains("file1.rs"));
        assert!(message2.loaded_context.text.contains("file2.rs"));

        // Third message should include only context 3 (not 1 or 2)
        assert!(!message3.loaded_context.text.contains("file1.rs"));
        assert!(!message3.loaded_context.text.contains("file2.rs"));
        assert!(message3.loaded_context.text.contains("file3.rs"));

        // Check entire request to make sure all contexts are properly included
        let request = thread.update(cx, |thread, cx| {
            thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
        });

        // The request should contain all 3 messages
        assert_eq!(request.messages.len(), 4);

        // Check that the contexts are properly formatted in each message
        assert!(request.messages[1].string_contents().contains("file1.rs"));
        assert!(!request.messages[1].string_contents().contains("file2.rs"));
        assert!(!request.messages[1].string_contents().contains("file3.rs"));

        assert!(!request.messages[2].string_contents().contains("file1.rs"));
        assert!(request.messages[2].string_contents().contains("file2.rs"));
        assert!(!request.messages[2].string_contents().contains("file3.rs"));

        assert!(!request.messages[3].string_contents().contains("file1.rs"));
        assert!(!request.messages[3].string_contents().contains("file2.rs"));
        assert!(request.messages[3].string_contents().contains("file3.rs"));

        add_file_to_context(&project, &context_store, "test/file4.rs", cx)
            .await
            .unwrap();
        let new_contexts = context_store.update(cx, |store, cx| {
            store.new_context_for_thread(thread.read(cx), Some(message2_id))
        });
        assert_eq!(new_contexts.len(), 3);
        let loaded_context = cx
            .update(|cx| load_context(new_contexts, &project, &None, cx))
            .await
            .loaded_context;

        assert!(!loaded_context.text.contains("file1.rs"));
        assert!(loaded_context.text.contains("file2.rs"));
        assert!(loaded_context.text.contains("file3.rs"));
        assert!(loaded_context.text.contains("file4.rs"));

        let new_contexts = context_store.update(cx, |store, cx| {
            // Remove file4.rs
            store.remove_context(&loaded_context.contexts[2].handle(), cx);
            store.new_context_for_thread(thread.read(cx), Some(message2_id))
        });
        assert_eq!(new_contexts.len(), 2);
        let loaded_context = cx
            .update(|cx| load_context(new_contexts, &project, &None, cx))
            .await
            .loaded_context;

        assert!(!loaded_context.text.contains("file1.rs"));
        assert!(loaded_context.text.contains("file2.rs"));
        assert!(loaded_context.text.contains("file3.rs"));
        assert!(!loaded_context.text.contains("file4.rs"));

        let new_contexts = context_store.update(cx, |store, cx| {
            // Remove file3.rs
            store.remove_context(&loaded_context.contexts[1].handle(), cx);
            store.new_context_for_thread(thread.read(cx), Some(message2_id))
        });
        assert_eq!(new_contexts.len(), 1);
        let loaded_context = cx
            .update(|cx| load_context(new_contexts, &project, &None, cx))
            .await
            .loaded_context;

        assert!(!loaded_context.text.contains("file1.rs"));
        assert!(loaded_context.text.contains("file2.rs"));
        assert!(!loaded_context.text.contains("file3.rs"));
        assert!(!loaded_context.text.contains("file4.rs"));
    }

    #[gpui::test]
    async fn test_message_without_files(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(
            &fs,
            cx,
            json!({"code.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"}),
        )
        .await;

        let (_, _thread_store, thread, _context_store, model) =
            setup_test_environment(cx, project.clone()).await;

        // Insert user message without any context (empty context vector)
        let message_id = thread.update(cx, |thread, cx| {
            thread.insert_user_message(
                "What is the best way to learn Rust?",
                ContextLoadResult::default(),
                None,
                Vec::new(),
                cx,
            )
        });

        // Check content and context in message object
        let message = thread.read_with(cx, |thread, _| thread.message(message_id).unwrap().clone());

        // Context should be empty when no files are included
        assert_eq!(message.role, Role::User);
        assert_eq!(message.segments.len(), 1);
        assert_eq!(
            message.segments[0],
            MessageSegment::Text("What is the best way to learn Rust?".to_string())
        );
        assert_eq!(message.loaded_context.text, "");

        // Check message in request
        let request = thread.update(cx, |thread, cx| {
            thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
        });

        assert_eq!(request.messages.len(), 2);
        assert_eq!(
            request.messages[1].string_contents(),
            "What is the best way to learn Rust?"
        );

        // Add second message, also without context
        let message2_id = thread.update(cx, |thread, cx| {
            thread.insert_user_message(
                "Are there any good books?",
                ContextLoadResult::default(),
                None,
                Vec::new(),
                cx,
            )
        });

        let message2 =
            thread.read_with(cx, |thread, _| thread.message(message2_id).unwrap().clone());
        assert_eq!(message2.loaded_context.text, "");

        // Check that both messages appear in the request
        let request = thread.update(cx, |thread, cx| {
            thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
        });

        assert_eq!(request.messages.len(), 3);
        assert_eq!(
            request.messages[1].string_contents(),
            "What is the best way to learn Rust?"
        );
        assert_eq!(
            request.messages[2].string_contents(),
            "Are there any good books?"
        );
    }

    #[gpui::test]
    #[ignore] // turn this test on when project_notifications tool is re-enabled
    async fn test_stale_buffer_notification(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(
            &fs,
            cx,
            json!({"code.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"}),
        )
        .await;

        let (_workspace, _thread_store, thread, context_store, model) =
            setup_test_environment(cx, project.clone()).await;

        // Add a buffer to the context. This will be a tracked buffer
        let buffer = add_file_to_context(&project, &context_store, "test/code.rs", cx)
            .await
            .unwrap();

        let context = context_store
            .read_with(cx, |store, _| store.context().next().cloned())
            .unwrap();
        let loaded_context = cx
            .update(|cx| load_context(vec![context], &project, &None, cx))
            .await;

        // Insert user message and assistant response
        thread.update(cx, |thread, cx| {
            thread.insert_user_message("Explain this code", loaded_context, None, Vec::new(), cx);
            thread.insert_assistant_message(
                vec![MessageSegment::Text("This code prints 42.".into())],
                cx,
            );
        });
        cx.run_until_parked();

        // We shouldn't have a stale buffer notification yet
        let notifications = thread.read_with(cx, |thread, _| {
            find_tool_uses(thread, "project_notifications")
        });
        assert!(
            notifications.is_empty(),
            "Should not have stale buffer notification before buffer is modified"
        );

        // Modify the buffer
        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(1..1, "\n    println!(\"Added a new line\");\n")],
                None,
                cx,
            );
        });

        // Insert another user message
        thread.update(cx, |thread, cx| {
            thread.insert_user_message(
                "What does the code do now?",
                ContextLoadResult::default(),
                None,
                Vec::new(),
                cx,
            )
        });
        cx.run_until_parked();

        // Check for the stale buffer warning
        thread.update(cx, |thread, cx| {
            thread.flush_notifications(model.clone(), CompletionIntent::UserPrompt, cx)
        });
        cx.run_until_parked();

        let notifications = thread.read_with(cx, |thread, _cx| {
            find_tool_uses(thread, "project_notifications")
        });

        let [notification] = notifications.as_slice() else {
            panic!("Should have a `project_notifications` tool use");
        };

        let Some(notification_content) = notification.content.to_str() else {
            panic!("`project_notifications` should return text");
        };

        assert!(notification_content.contains("These files have changed since the last read:"));
        assert!(notification_content.contains("code.rs"));

        // Insert another user message and flush notifications again
        thread.update(cx, |thread, cx| {
            thread.insert_user_message(
                "Can you tell me more?",
                ContextLoadResult::default(),
                None,
                Vec::new(),
                cx,
            )
        });

        thread.update(cx, |thread, cx| {
            thread.flush_notifications(model.clone(), CompletionIntent::UserPrompt, cx)
        });
        cx.run_until_parked();

        // There should be no new notifications (we already flushed one)
        let notifications = thread.read_with(cx, |thread, _cx| {
            find_tool_uses(thread, "project_notifications")
        });

        assert_eq!(
            notifications.len(),
            1,
            "Should still have only one notification after second flush - no duplicates"
        );
    }

    fn find_tool_uses(thread: &Thread, tool_name: &str) -> Vec<LanguageModelToolResult> {
        thread
            .messages()
            .flat_map(|message| {
                thread
                    .tool_results_for_message(message.id)
                    .into_iter()
                    .filter(|result| result.tool_name == tool_name.into())
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    #[gpui::test]
    async fn test_storing_profile_setting_per_thread(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(
            &fs,
            cx,
            json!({"code.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"}),
        )
        .await;

        let (_workspace, thread_store, thread, _context_store, _model) =
            setup_test_environment(cx, project.clone()).await;

        // Check that we are starting with the default profile
        let profile = cx.read(|cx| thread.read(cx).profile.clone());
        let tool_set = cx.read(|cx| thread_store.read(cx).tools());
        assert_eq!(
            profile,
            AgentProfile::new(AgentProfileId::default(), tool_set)
        );
    }

    #[gpui::test]
    async fn test_serializing_thread_profile(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(
            &fs,
            cx,
            json!({"code.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"}),
        )
        .await;

        let (_workspace, thread_store, thread, _context_store, _model) =
            setup_test_environment(cx, project.clone()).await;

        // Profile gets serialized with default values
        let serialized = thread
            .update(cx, |thread, cx| thread.serialize(cx))
            .await
            .unwrap();

        assert_eq!(serialized.profile, Some(AgentProfileId::default()));

        let deserialized = cx.update(|cx| {
            thread.update(cx, |thread, cx| {
                Thread::deserialize(
                    thread.id.clone(),
                    serialized,
                    thread.project.clone(),
                    thread.tools.clone(),
                    thread.prompt_builder.clone(),
                    thread.project_context.clone(),
                    None,
                    cx,
                )
            })
        });
        let tool_set = cx.read(|cx| thread_store.read(cx).tools());

        assert_eq!(
            deserialized.profile,
            AgentProfile::new(AgentProfileId::default(), tool_set)
        );
    }

    #[gpui::test]
    async fn test_temperature_setting(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(
            &fs,
            cx,
            json!({"code.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"}),
        )
        .await;

        let (_workspace, _thread_store, thread, _context_store, model) =
            setup_test_environment(cx, project.clone()).await;

        // Both model and provider
        cx.update(|cx| {
            AgentSettings::override_global(
                AgentSettings {
                    model_parameters: vec![LanguageModelParameters {
                        provider: Some(model.provider_id().0.to_string().into()),
                        model: Some(model.id().0),
                        temperature: Some(0.66),
                    }],
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let request = thread.update(cx, |thread, cx| {
            thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
        });
        assert_eq!(request.temperature, Some(0.66));

        // Only model
        cx.update(|cx| {
            AgentSettings::override_global(
                AgentSettings {
                    model_parameters: vec![LanguageModelParameters {
                        provider: None,
                        model: Some(model.id().0),
                        temperature: Some(0.66),
                    }],
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let request = thread.update(cx, |thread, cx| {
            thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
        });
        assert_eq!(request.temperature, Some(0.66));

        // Only provider
        cx.update(|cx| {
            AgentSettings::override_global(
                AgentSettings {
                    model_parameters: vec![LanguageModelParameters {
                        provider: Some(model.provider_id().0.to_string().into()),
                        model: None,
                        temperature: Some(0.66),
                    }],
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let request = thread.update(cx, |thread, cx| {
            thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
        });
        assert_eq!(request.temperature, Some(0.66));

        // Same model name, different provider
        cx.update(|cx| {
            AgentSettings::override_global(
                AgentSettings {
                    model_parameters: vec![LanguageModelParameters {
                        provider: Some("anthropic".into()),
                        model: Some(model.id().0),
                        temperature: Some(0.66),
                    }],
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let request = thread.update(cx, |thread, cx| {
            thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
        });
        assert_eq!(request.temperature, None);
    }

    #[gpui::test]
    async fn test_thread_summary(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(&fs, cx, json!({})).await;

        let (_, _thread_store, thread, _context_store, model) =
            setup_test_environment(cx, project.clone()).await;

        // Initial state should be pending
        thread.read_with(cx, |thread, _| {
            assert!(matches!(thread.summary(), ThreadSummary::Pending));
            assert_eq!(thread.summary().or_default(), ThreadSummary::DEFAULT);
        });

        // Manually setting the summary should not be allowed in this state
        thread.update(cx, |thread, cx| {
            thread.set_summary("This should not work", cx);
        });

        thread.read_with(cx, |thread, _| {
            assert!(matches!(thread.summary(), ThreadSummary::Pending));
        });

        // Send a message
        thread.update(cx, |thread, cx| {
            thread.insert_user_message("Hi!", ContextLoadResult::default(), None, vec![], cx);
            thread.send_to_model(
                model.clone(),
                CompletionIntent::ThreadSummarization,
                None,
                cx,
            );
        });

        let fake_model = model.as_fake();
        simulate_successful_response(fake_model, cx);

        // Should start generating summary when there are >= 2 messages
        thread.read_with(cx, |thread, _| {
            assert_eq!(*thread.summary(), ThreadSummary::Generating);
        });

        // Should not be able to set the summary while generating
        thread.update(cx, |thread, cx| {
            thread.set_summary("This should not work either", cx);
        });

        thread.read_with(cx, |thread, _| {
            assert!(matches!(thread.summary(), ThreadSummary::Generating));
            assert_eq!(thread.summary().or_default(), ThreadSummary::DEFAULT);
        });

        cx.run_until_parked();
        fake_model.send_last_completion_stream_text_chunk("Brief");
        fake_model.send_last_completion_stream_text_chunk(" Introduction");
        fake_model.end_last_completion_stream();
        cx.run_until_parked();

        // Summary should be set
        thread.read_with(cx, |thread, _| {
            assert!(matches!(thread.summary(), ThreadSummary::Ready(_)));
            assert_eq!(thread.summary().or_default(), "Brief Introduction");
        });

        // Now we should be able to set a summary
        thread.update(cx, |thread, cx| {
            thread.set_summary("Brief Intro", cx);
        });

        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.summary().or_default(), "Brief Intro");
        });

        // Test setting an empty summary (should default to DEFAULT)
        thread.update(cx, |thread, cx| {
            thread.set_summary("", cx);
        });

        thread.read_with(cx, |thread, _| {
            assert!(matches!(thread.summary(), ThreadSummary::Ready(_)));
            assert_eq!(thread.summary().or_default(), ThreadSummary::DEFAULT);
        });
    }

    #[gpui::test]
    async fn test_thread_summary_error_set_manually(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(&fs, cx, json!({})).await;

        let (_, _thread_store, thread, _context_store, model) =
            setup_test_environment(cx, project.clone()).await;

        test_summarize_error(&model, &thread, cx);

        // Now we should be able to set a summary
        thread.update(cx, |thread, cx| {
            thread.set_summary("Brief Intro", cx);
        });

        thread.read_with(cx, |thread, _| {
            assert!(matches!(thread.summary(), ThreadSummary::Ready(_)));
            assert_eq!(thread.summary().or_default(), "Brief Intro");
        });
    }

    #[gpui::test]
    async fn test_thread_summary_error_retry(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(&fs, cx, json!({})).await;

        let (_, _thread_store, thread, _context_store, model) =
            setup_test_environment(cx, project.clone()).await;

        test_summarize_error(&model, &thread, cx);

        // Sending another message should not trigger another summarize request
        thread.update(cx, |thread, cx| {
            thread.insert_user_message(
                "How are you?",
                ContextLoadResult::default(),
                None,
                vec![],
                cx,
            );
            thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
        });

        let fake_model = model.as_fake();
        simulate_successful_response(fake_model, cx);

        thread.read_with(cx, |thread, _| {
            // State is still Error, not Generating
            assert!(matches!(thread.summary(), ThreadSummary::Error));
        });

        // But the summarize request can be invoked manually
        thread.update(cx, |thread, cx| {
            thread.summarize(cx);
        });

        thread.read_with(cx, |thread, _| {
            assert!(matches!(thread.summary(), ThreadSummary::Generating));
        });

        cx.run_until_parked();
        fake_model.send_last_completion_stream_text_chunk("A successful summary");
        fake_model.end_last_completion_stream();
        cx.run_until_parked();

        thread.read_with(cx, |thread, _| {
            assert!(matches!(thread.summary(), ThreadSummary::Ready(_)));
            assert_eq!(thread.summary().or_default(), "A successful summary");
        });
    }

    // Helper to create a model that returns errors
    enum TestError {
        Overloaded,
        InternalServerError,
    }

    struct ErrorInjector {
        inner: Arc<FakeLanguageModel>,
        error_type: TestError,
    }

    impl ErrorInjector {
        fn new(error_type: TestError) -> Self {
            Self {
                inner: Arc::new(FakeLanguageModel::default()),
                error_type,
            }
        }
    }

    impl LanguageModel for ErrorInjector {
        fn id(&self) -> LanguageModelId {
            self.inner.id()
        }

        fn name(&self) -> LanguageModelName {
            self.inner.name()
        }

        fn provider_id(&self) -> LanguageModelProviderId {
            self.inner.provider_id()
        }

        fn provider_name(&self) -> LanguageModelProviderName {
            self.inner.provider_name()
        }

        fn supports_tools(&self) -> bool {
            self.inner.supports_tools()
        }

        fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
            self.inner.supports_tool_choice(choice)
        }

        fn supports_images(&self) -> bool {
            self.inner.supports_images()
        }

        fn telemetry_id(&self) -> String {
            self.inner.telemetry_id()
        }

        fn max_token_count(&self) -> u64 {
            self.inner.max_token_count()
        }

        fn count_tokens(
            &self,
            request: LanguageModelRequest,
            cx: &App,
        ) -> BoxFuture<'static, Result<u64>> {
            self.inner.count_tokens(request, cx)
        }

        fn stream_completion(
            &self,
            _request: LanguageModelRequest,
            _cx: &AsyncApp,
        ) -> BoxFuture<
            'static,
            Result<
                BoxStream<
                    'static,
                    Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
                >,
                LanguageModelCompletionError,
            >,
        > {
            let error = match self.error_type {
                TestError::Overloaded => LanguageModelCompletionError::ServerOverloaded {
                    provider: self.provider_name(),
                    retry_after: None,
                },
                TestError::InternalServerError => {
                    LanguageModelCompletionError::ApiInternalServerError {
                        provider: self.provider_name(),
                        message: "I'm a teapot orbiting the sun".to_string(),
                    }
                }
            };
            async move {
                let stream = futures::stream::once(async move { Err(error) });
                Ok(stream.boxed())
            }
            .boxed()
        }

        fn as_fake(&self) -> &FakeLanguageModel {
            &self.inner
        }
    }

    #[gpui::test]
    async fn test_retry_on_overloaded_error(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(&fs, cx, json!({})).await;
        let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

        // Enable Burn Mode to allow retries
        thread.update(cx, |thread, _| {
            thread.set_completion_mode(CompletionMode::Burn);
        });

        // Create model that returns overloaded error
        let model = Arc::new(ErrorInjector::new(TestError::Overloaded));

        // Insert a user message
        thread.update(cx, |thread, cx| {
            thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
        });

        // Start completion
        thread.update(cx, |thread, cx| {
            thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
        });

        cx.run_until_parked();

        thread.read_with(cx, |thread, _| {
            assert!(thread.retry_state.is_some(), "Should have retry state");
            let retry_state = thread.retry_state.as_ref().unwrap();
            assert_eq!(retry_state.attempt, 1, "Should be first retry attempt");
            assert_eq!(
                retry_state.max_attempts, MAX_RETRY_ATTEMPTS,
                "Should retry MAX_RETRY_ATTEMPTS times for overloaded errors"
            );
        });

        // Check that a retry message was added
        thread.read_with(cx, |thread, _| {
            let mut messages = thread.messages();
            assert!(
                messages.any(|msg| {
                    msg.role == Role::System
                        && msg.ui_only
                        && msg.segments.iter().any(|seg| {
                            if let MessageSegment::Text(text) = seg {
                                text.contains("overloaded")
                                    && text
                                        .contains(&format!("attempt 1 of {}", MAX_RETRY_ATTEMPTS))
                            } else {
                                false
                            }
                        })
                }),
                "Should have added a system retry message"
            );
        });

        let retry_count = thread.update(cx, |thread, _| {
            thread
                .messages
                .iter()
                .filter(|m| {
                    m.ui_only
                        && m.segments.iter().any(|s| {
                            if let MessageSegment::Text(text) = s {
                                text.contains("Retrying") && text.contains("seconds")
                            } else {
                                false
                            }
                        })
                })
                .count()
        });

        assert_eq!(retry_count, 1, "Should have one retry message");
    }

    #[gpui::test]
    async fn test_retry_on_internal_server_error(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(&fs, cx, json!({})).await;
        let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

        // Enable Burn Mode to allow retries
        thread.update(cx, |thread, _| {
            thread.set_completion_mode(CompletionMode::Burn);
        });

        // Create model that returns internal server error
        let model = Arc::new(ErrorInjector::new(TestError::InternalServerError));

        // Insert a user message
        thread.update(cx, |thread, cx| {
            thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
        });

        // Start completion
        thread.update(cx, |thread, cx| {
            thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
        });

        cx.run_until_parked();

        // Check retry state on thread
        thread.read_with(cx, |thread, _| {
            assert!(thread.retry_state.is_some(), "Should have retry state");
            let retry_state = thread.retry_state.as_ref().unwrap();
            assert_eq!(retry_state.attempt, 1, "Should be first retry attempt");
            assert_eq!(
                retry_state.max_attempts, 3,
                "Should have correct max attempts"
            );
        });

        // Check that a retry message was added with provider name
        thread.read_with(cx, |thread, _| {
            let mut messages = thread.messages();
            assert!(
                messages.any(|msg| {
                    msg.role == Role::System
                        && msg.ui_only
                        && msg.segments.iter().any(|seg| {
                            if let MessageSegment::Text(text) = seg {
                                text.contains("internal")
                                    && text.contains("Fake")
                                    && text.contains("Retrying")
                                    && text.contains("attempt 1 of 3")
                                    && text.contains("seconds")
                            } else {
                                false
                            }
                        })
                }),
                "Should have added a system retry message with provider name"
            );
        });

        // Count retry messages
        let retry_count = thread.update(cx, |thread, _| {
            thread
                .messages
                .iter()
                .filter(|m| {
                    m.ui_only
                        && m.segments.iter().any(|s| {
                            if let MessageSegment::Text(text) = s {
                                text.contains("Retrying") && text.contains("seconds")
                            } else {
                                false
                            }
                        })
                })
                .count()
        });

        assert_eq!(retry_count, 1, "Should have one retry message");
    }

    #[gpui::test]
    async fn test_exponential_backoff_on_retries(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(&fs, cx, json!({})).await;
        let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

        // Enable Burn Mode to allow retries
        thread.update(cx, |thread, _| {
            thread.set_completion_mode(CompletionMode::Burn);
        });

        // Create model that returns internal server error
        let model = Arc::new(ErrorInjector::new(TestError::InternalServerError));

        // Insert a user message
        thread.update(cx, |thread, cx| {
            thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
        });

        // Track retry events and completion count
        // Track completion events
        let completion_count = Arc::new(Mutex::new(0));
        let completion_count_clone = completion_count.clone();

        let _subscription = thread.update(cx, |_, cx| {
            cx.subscribe(&thread, move |_, _, event: &ThreadEvent, _| {
                if let ThreadEvent::NewRequest = event {
                    *completion_count_clone.lock() += 1;
                }
            })
        });

        // First attempt
        thread.update(cx, |thread, cx| {
            thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
        });
        cx.run_until_parked();

        // Should have scheduled first retry - count retry messages
        let retry_count = thread.update(cx, |thread, _| {
            thread
                .messages
                .iter()
                .filter(|m| {
                    m.ui_only
                        && m.segments.iter().any(|s| {
                            if let MessageSegment::Text(text) = s {
                                text.contains("Retrying") && text.contains("seconds")
                            } else {
                                false
                            }
                        })
                })
                .count()
        });
        assert_eq!(retry_count, 1, "Should have scheduled first retry");

        // Check retry state
        thread.read_with(cx, |thread, _| {
            assert!(thread.retry_state.is_some(), "Should have retry state");
            let retry_state = thread.retry_state.as_ref().unwrap();
            assert_eq!(retry_state.attempt, 1, "Should be first retry attempt");
            assert_eq!(
                retry_state.max_attempts, 3,
                "Internal server errors should retry up to 3 times"
            );
        });

        // Advance clock for first retry
        cx.executor().advance_clock(BASE_RETRY_DELAY);
        cx.run_until_parked();

        // Advance clock for second retry
        cx.executor().advance_clock(BASE_RETRY_DELAY);
        cx.run_until_parked();

        // Advance clock for third retry
        cx.executor().advance_clock(BASE_RETRY_DELAY);
        cx.run_until_parked();

        // Should have completed all retries - count retry messages
        let retry_count = thread.update(cx, |thread, _| {
            thread
                .messages
                .iter()
                .filter(|m| {
                    m.ui_only
                        && m.segments.iter().any(|s| {
                            if let MessageSegment::Text(text) = s {
                                text.contains("Retrying") && text.contains("seconds")
                            } else {
                                false
                            }
                        })
                })
                .count()
        });
        assert_eq!(
            retry_count, 3,
            "Should have 3 retries for internal server errors"
        );

        // For internal server errors, we retry 3 times and then give up
        // Check that retry_state is cleared after all retries
        thread.read_with(cx, |thread, _| {
            assert!(
                thread.retry_state.is_none(),
                "Retry state should be cleared after all retries"
            );
        });

        // Verify total attempts (1 initial + 3 retries)
        assert_eq!(
            *completion_count.lock(),
            4,
            "Should have attempted once plus 3 retries"
        );
    }

    #[gpui::test]
    async fn test_max_retries_exceeded(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(&fs, cx, json!({})).await;
        let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

        // Enable Burn Mode to allow retries
        thread.update(cx, |thread, _| {
            thread.set_completion_mode(CompletionMode::Burn);
        });

        // Create model that returns overloaded error
        let model = Arc::new(ErrorInjector::new(TestError::Overloaded));

        // Insert a user message
        thread.update(cx, |thread, cx| {
            thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
        });

        // Track events
        let stopped_with_error = Arc::new(Mutex::new(false));
        let stopped_with_error_clone = stopped_with_error.clone();

        let _subscription = thread.update(cx, |_, cx| {
            cx.subscribe(&thread, move |_, _, event: &ThreadEvent, _| {
                if let ThreadEvent::Stopped(Err(_)) = event {
                    *stopped_with_error_clone.lock() = true;
                }
            })
        });

        // Start initial completion
        thread.update(cx, |thread, cx| {
            thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
        });
        cx.run_until_parked();

        // Advance through all retries
        for _ in 0..MAX_RETRY_ATTEMPTS {
            cx.executor().advance_clock(BASE_RETRY_DELAY);
            cx.run_until_parked();
        }

        let retry_count = thread.update(cx, |thread, _| {
            thread
                .messages
                .iter()
                .filter(|m| {
                    m.ui_only
                        && m.segments.iter().any(|s| {
                            if let MessageSegment::Text(text) = s {
                                text.contains("Retrying") && text.contains("seconds")
                            } else {
                                false
                            }
                        })
                })
                .count()
        });

        // After max retries, should emit Stopped(Err(...)) event
        assert_eq!(
            retry_count, MAX_RETRY_ATTEMPTS as usize,
            "Should have attempted MAX_RETRY_ATTEMPTS retries for overloaded errors"
        );
        assert!(
            *stopped_with_error.lock(),
            "Should emit Stopped(Err(...)) event after max retries exceeded"
        );

        // Retry state should be cleared
        thread.read_with(cx, |thread, _| {
            assert!(
                thread.retry_state.is_none(),
                "Retry state should be cleared after max retries"
            );

            // Verify we have the expected number of retry messages
            let retry_messages = thread
                .messages
                .iter()
                .filter(|msg| msg.ui_only && msg.role == Role::System)
                .count();
            assert_eq!(
                retry_messages, MAX_RETRY_ATTEMPTS as usize,
                "Should have MAX_RETRY_ATTEMPTS retry messages for overloaded errors"
            );
        });
    }

    #[gpui::test]
    async fn test_retry_message_removed_on_retry(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(&fs, cx, json!({})).await;
        let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

        // Enable Burn Mode to allow retries
        thread.update(cx, |thread, _| {
            thread.set_completion_mode(CompletionMode::Burn);
        });

        // We'll use a wrapper to switch behavior after first failure
        struct RetryTestModel {
            inner: Arc<FakeLanguageModel>,
            failed_once: Arc<Mutex<bool>>,
        }

        impl LanguageModel for RetryTestModel {
            fn id(&self) -> LanguageModelId {
                self.inner.id()
            }

            fn name(&self) -> LanguageModelName {
                self.inner.name()
            }

            fn provider_id(&self) -> LanguageModelProviderId {
                self.inner.provider_id()
            }

            fn provider_name(&self) -> LanguageModelProviderName {
                self.inner.provider_name()
            }

            fn supports_tools(&self) -> bool {
                self.inner.supports_tools()
            }

            fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
                self.inner.supports_tool_choice(choice)
            }

            fn supports_images(&self) -> bool {
                self.inner.supports_images()
            }

            fn telemetry_id(&self) -> String {
                self.inner.telemetry_id()
            }

            fn max_token_count(&self) -> u64 {
                self.inner.max_token_count()
            }

            fn count_tokens(
                &self,
                request: LanguageModelRequest,
                cx: &App,
            ) -> BoxFuture<'static, Result<u64>> {
                self.inner.count_tokens(request, cx)
            }

            fn stream_completion(
                &self,
                request: LanguageModelRequest,
                cx: &AsyncApp,
            ) -> BoxFuture<
                'static,
                Result<
                    BoxStream<
                        'static,
                        Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
                    >,
                    LanguageModelCompletionError,
                >,
            > {
                if !*self.failed_once.lock() {
                    *self.failed_once.lock() = true;
                    let provider = self.provider_name();
                    // Return error on first attempt
                    let stream = futures::stream::once(async move {
                        Err(LanguageModelCompletionError::ServerOverloaded {
                            provider,
                            retry_after: None,
                        })
                    });
                    async move { Ok(stream.boxed()) }.boxed()
                } else {
                    // Succeed on retry
                    self.inner.stream_completion(request, cx)
                }
            }

            fn as_fake(&self) -> &FakeLanguageModel {
                &self.inner
            }
        }

        let model = Arc::new(RetryTestModel {
            inner: Arc::new(FakeLanguageModel::default()),
            failed_once: Arc::new(Mutex::new(false)),
        });

        // Insert a user message
        thread.update(cx, |thread, cx| {
            thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
        });

        // Track message deletions
        // Track when retry completes successfully
        let retry_completed = Arc::new(Mutex::new(false));
        let retry_completed_clone = retry_completed.clone();

        let _subscription = thread.update(cx, |_, cx| {
            cx.subscribe(&thread, move |_, _, event: &ThreadEvent, _| {
                if let ThreadEvent::StreamedCompletion = event {
                    *retry_completed_clone.lock() = true;
                }
            })
        });

        // Start completion
        thread.update(cx, |thread, cx| {
            thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
        });
        cx.run_until_parked();

        // Get the retry message ID
        let retry_message_id = thread.read_with(cx, |thread, _| {
            thread
                .messages()
                .find(|msg| msg.role == Role::System && msg.ui_only)
                .map(|msg| msg.id)
                .expect("Should have a retry message")
        });

        // Wait for retry
        cx.executor().advance_clock(BASE_RETRY_DELAY);
        cx.run_until_parked();

        // Stream some successful content
        let fake_model = model.as_fake();
        // After the retry, there should be a new pending completion
        let pending = fake_model.pending_completions();
        assert!(
            !pending.is_empty(),
            "Should have a pending completion after retry"
        );
        fake_model.send_completion_stream_text_chunk(&pending[0], "Success!");
        fake_model.end_completion_stream(&pending[0]);
        cx.run_until_parked();

        // Check that the retry completed successfully
        assert!(
            *retry_completed.lock(),
            "Retry should have completed successfully"
        );

        // Retry message should still exist but be marked as ui_only
        thread.read_with(cx, |thread, _| {
            let retry_msg = thread
                .message(retry_message_id)
                .expect("Retry message should still exist");
            assert!(retry_msg.ui_only, "Retry message should be ui_only");
            assert_eq!(
                retry_msg.role,
                Role::System,
                "Retry message should have System role"
            );
        });
    }

    #[gpui::test]
    async fn test_successful_completion_clears_retry_state(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(&fs, cx, json!({})).await;
        let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

        // Enable Burn Mode to allow retries
        thread.update(cx, |thread, _| {
            thread.set_completion_mode(CompletionMode::Burn);
        });

        // Create a model that fails once then succeeds
        struct FailOnceModel {
            inner: Arc<FakeLanguageModel>,
            failed_once: Arc<Mutex<bool>>,
        }

        impl LanguageModel for FailOnceModel {
            fn id(&self) -> LanguageModelId {
                self.inner.id()
            }

            fn name(&self) -> LanguageModelName {
                self.inner.name()
            }

            fn provider_id(&self) -> LanguageModelProviderId {
                self.inner.provider_id()
            }

            fn provider_name(&self) -> LanguageModelProviderName {
                self.inner.provider_name()
            }

            fn supports_tools(&self) -> bool {
                self.inner.supports_tools()
            }

            fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
                self.inner.supports_tool_choice(choice)
            }

            fn supports_images(&self) -> bool {
                self.inner.supports_images()
            }

            fn telemetry_id(&self) -> String {
                self.inner.telemetry_id()
            }

            fn max_token_count(&self) -> u64 {
                self.inner.max_token_count()
            }

            fn count_tokens(
                &self,
                request: LanguageModelRequest,
                cx: &App,
            ) -> BoxFuture<'static, Result<u64>> {
                self.inner.count_tokens(request, cx)
            }

            fn stream_completion(
                &self,
                request: LanguageModelRequest,
                cx: &AsyncApp,
            ) -> BoxFuture<
                'static,
                Result<
                    BoxStream<
                        'static,
                        Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
                    >,
                    LanguageModelCompletionError,
                >,
            > {
                if !*self.failed_once.lock() {
                    *self.failed_once.lock() = true;
                    let provider = self.provider_name();
                    // Return error on first attempt
                    let stream = futures::stream::once(async move {
                        Err(LanguageModelCompletionError::ServerOverloaded {
                            provider,
                            retry_after: None,
                        })
                    });
                    async move { Ok(stream.boxed()) }.boxed()
                } else {
                    // Succeed on retry
                    self.inner.stream_completion(request, cx)
                }
            }
        }

        let fail_once_model = Arc::new(FailOnceModel {
            inner: Arc::new(FakeLanguageModel::default()),
            failed_once: Arc::new(Mutex::new(false)),
        });

        // Insert a user message
        thread.update(cx, |thread, cx| {
            thread.insert_user_message(
                "Test message",
                ContextLoadResult::default(),
                None,
                vec![],
                cx,
            );
        });

        // Start completion with fail-once model
        thread.update(cx, |thread, cx| {
            thread.send_to_model(
                fail_once_model.clone(),
                CompletionIntent::UserPrompt,
                None,
                cx,
            );
        });

        cx.run_until_parked();

        // Verify retry state exists after first failure
        thread.read_with(cx, |thread, _| {
            assert!(
                thread.retry_state.is_some(),
                "Should have retry state after failure"
            );
        });

        // Wait for retry delay
        cx.executor().advance_clock(BASE_RETRY_DELAY);
        cx.run_until_parked();

        // The retry should now use our FailOnceModel which should succeed
        // We need to help the FakeLanguageModel complete the stream
        let inner_fake = fail_once_model.inner.clone();

        // Wait a bit for the retry to start
        cx.run_until_parked();

        // Check for pending completions and complete them
        if let Some(pending) = inner_fake.pending_completions().first() {
            inner_fake.send_completion_stream_text_chunk(pending, "Success!");
            inner_fake.end_completion_stream(pending);
        }
        cx.run_until_parked();

        thread.read_with(cx, |thread, _| {
            assert!(
                thread.retry_state.is_none(),
                "Retry state should be cleared after successful completion"
            );

            let has_assistant_message = thread
                .messages
                .iter()
                .any(|msg| msg.role == Role::Assistant && !msg.ui_only);
            assert!(
                has_assistant_message,
                "Should have an assistant message after successful retry"
            );
        });
    }

    #[gpui::test]
    async fn test_rate_limit_retry_single_attempt(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(&fs, cx, json!({})).await;
        let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

        // Enable Burn Mode to allow retries
        thread.update(cx, |thread, _| {
            thread.set_completion_mode(CompletionMode::Burn);
        });

        // Create a model that returns rate limit error with retry_after
        struct RateLimitModel {
            inner: Arc<FakeLanguageModel>,
        }

        impl LanguageModel for RateLimitModel {
            fn id(&self) -> LanguageModelId {
                self.inner.id()
            }

            fn name(&self) -> LanguageModelName {
                self.inner.name()
            }

            fn provider_id(&self) -> LanguageModelProviderId {
                self.inner.provider_id()
            }

            fn provider_name(&self) -> LanguageModelProviderName {
                self.inner.provider_name()
            }

            fn supports_tools(&self) -> bool {
                self.inner.supports_tools()
            }

            fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
                self.inner.supports_tool_choice(choice)
            }

            fn supports_images(&self) -> bool {
                self.inner.supports_images()
            }

            fn telemetry_id(&self) -> String {
                self.inner.telemetry_id()
            }

            fn max_token_count(&self) -> u64 {
                self.inner.max_token_count()
            }

            fn count_tokens(
                &self,
                request: LanguageModelRequest,
                cx: &App,
            ) -> BoxFuture<'static, Result<u64>> {
                self.inner.count_tokens(request, cx)
            }

            fn stream_completion(
                &self,
                _request: LanguageModelRequest,
                _cx: &AsyncApp,
            ) -> BoxFuture<
                'static,
                Result<
                    BoxStream<
                        'static,
                        Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
                    >,
                    LanguageModelCompletionError,
                >,
            > {
                let provider = self.provider_name();
                async move {
                    let stream = futures::stream::once(async move {
                        Err(LanguageModelCompletionError::RateLimitExceeded {
                            provider,
                            retry_after: Some(Duration::from_secs(TEST_RATE_LIMIT_RETRY_SECS)),
                        })
                    });
                    Ok(stream.boxed())
                }
                .boxed()
            }

            fn as_fake(&self) -> &FakeLanguageModel {
                &self.inner
            }
        }

        let model = Arc::new(RateLimitModel {
            inner: Arc::new(FakeLanguageModel::default()),
        });

        // Insert a user message
        thread.update(cx, |thread, cx| {
            thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
        });

        // Start completion
        thread.update(cx, |thread, cx| {
            thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
        });

        cx.run_until_parked();

        let retry_count = thread.update(cx, |thread, _| {
            thread
                .messages
                .iter()
                .filter(|m| {
                    m.ui_only
                        && m.segments.iter().any(|s| {
                            if let MessageSegment::Text(text) = s {
                                text.contains("rate limit exceeded")
                            } else {
                                false
                            }
                        })
                })
                .count()
        });
        assert_eq!(retry_count, 1, "Should have scheduled one retry");

        thread.read_with(cx, |thread, _| {
            assert!(
                thread.retry_state.is_some(),
                "Rate limit errors should set retry_state"
            );
            if let Some(retry_state) = &thread.retry_state {
                assert_eq!(
                    retry_state.max_attempts, MAX_RETRY_ATTEMPTS,
                    "Rate limit errors should use MAX_RETRY_ATTEMPTS"
                );
            }
        });

        // Verify we have one retry message
        thread.read_with(cx, |thread, _| {
            let retry_messages = thread
                .messages
                .iter()
                .filter(|msg| {
                    msg.ui_only
                        && msg.segments.iter().any(|seg| {
                            if let MessageSegment::Text(text) = seg {
                                text.contains("rate limit exceeded")
                            } else {
                                false
                            }
                        })
                })
                .count();
            assert_eq!(
                retry_messages, 1,
                "Should have one rate limit retry message"
            );
        });

        // Check that retry message doesn't include attempt count
        thread.read_with(cx, |thread, _| {
            let retry_message = thread
                .messages
                .iter()
                .find(|msg| msg.role == Role::System && msg.ui_only)
                .expect("Should have a retry message");

            // Check that the message contains attempt count since we use retry_state
            if let Some(MessageSegment::Text(text)) = retry_message.segments.first() {
                assert!(
                    text.contains(&format!("attempt 1 of {}", MAX_RETRY_ATTEMPTS)),
                    "Rate limit retry message should contain attempt count with MAX_RETRY_ATTEMPTS"
                );
                assert!(
                    text.contains("Retrying"),
                    "Rate limit retry message should contain retry text"
                );
            }
        });
    }

    #[gpui::test]
    async fn test_ui_only_messages_not_sent_to_model(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(&fs, cx, json!({})).await;
        let (_, _, thread, _, model) = setup_test_environment(cx, project.clone()).await;

        // Insert a regular user message
        thread.update(cx, |thread, cx| {
            thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
        });

        // Insert a UI-only message (like our retry notifications)
        thread.update(cx, |thread, cx| {
            let id = thread.next_message_id.post_inc();
            thread.messages.push(Message {
                id,
                role: Role::System,
                segments: vec![MessageSegment::Text(
                    "This is a UI-only message that should not be sent to the model".to_string(),
                )],
                loaded_context: LoadedContext::default(),
                creases: Vec::new(),
                is_hidden: true,
                ui_only: true,
            });
            cx.emit(ThreadEvent::MessageAdded(id));
        });

        // Insert another regular message
        thread.update(cx, |thread, cx| {
            thread.insert_user_message(
                "How are you?",
                ContextLoadResult::default(),
                None,
                vec![],
                cx,
            );
        });

        // Generate the completion request
        let request = thread.update(cx, |thread, cx| {
            thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
        });

        // Verify that the request only contains non-UI-only messages
        // Should have system prompt + 2 user messages, but not the UI-only message
        let user_messages: Vec<_> = request
            .messages
            .iter()
            .filter(|msg| msg.role == Role::User)
            .collect();
        assert_eq!(
            user_messages.len(),
            2,
            "Should have exactly 2 user messages"
        );

        // Verify the UI-only content is not present anywhere in the request
        let request_text = request
            .messages
            .iter()
            .flat_map(|msg| &msg.content)
            .filter_map(|content| match content {
                MessageContent::Text(text) => Some(text.as_str()),
                _ => None,
            })
            .collect::<String>();

        assert!(
            !request_text.contains("UI-only message"),
            "UI-only message content should not be in the request"
        );

        // Verify the thread still has all 3 messages (including UI-only)
        thread.read_with(cx, |thread, _| {
            assert_eq!(
                thread.messages().count(),
                3,
                "Thread should have 3 messages"
            );
            assert_eq!(
                thread.messages().filter(|m| m.ui_only).count(),
                1,
                "Thread should have 1 UI-only message"
            );
        });

        // Verify that UI-only messages are not serialized
        let serialized = thread
            .update(cx, |thread, cx| thread.serialize(cx))
            .await
            .unwrap();
        assert_eq!(
            serialized.messages.len(),
            2,
            "Serialized thread should only have 2 messages (no UI-only)"
        );
    }

    #[gpui::test]
    async fn test_no_retry_without_burn_mode(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(&fs, cx, json!({})).await;
        let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

        // Ensure we're in Normal mode (not Burn mode)
        thread.update(cx, |thread, _| {
            thread.set_completion_mode(CompletionMode::Normal);
        });

        // Track error events
        let error_events = Arc::new(Mutex::new(Vec::new()));
        let error_events_clone = error_events.clone();

        let _subscription = thread.update(cx, |_, cx| {
            cx.subscribe(&thread, move |_, _, event: &ThreadEvent, _| {
                if let ThreadEvent::ShowError(error) = event {
                    error_events_clone.lock().push(error.clone());
                }
            })
        });

        // Create model that returns overloaded error
        let model = Arc::new(ErrorInjector::new(TestError::Overloaded));

        // Insert a user message
        thread.update(cx, |thread, cx| {
            thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
        });

        // Start completion
        thread.update(cx, |thread, cx| {
            thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
        });

        cx.run_until_parked();

        // Verify no retry state was created
        thread.read_with(cx, |thread, _| {
            assert!(
                thread.retry_state.is_none(),
                "Should not have retry state in Normal mode"
            );
        });

        // Check that a retryable error was reported
        let errors = error_events.lock();
        assert!(!errors.is_empty(), "Should have received an error event");

        if let ThreadError::RetryableError {
            message: _,
            can_enable_burn_mode,
        } = &errors[0]
        {
            assert!(
                *can_enable_burn_mode,
                "Error should indicate burn mode can be enabled"
            );
        } else {
            panic!("Expected RetryableError, got {:?}", errors[0]);
        }

        // Verify the thread is no longer generating
        thread.read_with(cx, |thread, _| {
            assert!(
                !thread.is_generating(),
                "Should not be generating after error without retry"
            );
        });
    }

    #[gpui::test]
    async fn test_retry_canceled_on_stop(cx: &mut TestAppContext) {
        let fs = init_test_settings(cx);

        let project = create_test_project(&fs, cx, json!({})).await;
        let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

        // Enable Burn Mode to allow retries
        thread.update(cx, |thread, _| {
            thread.set_completion_mode(CompletionMode::Burn);
        });

        // Create model that returns overloaded error
        let model = Arc::new(ErrorInjector::new(TestError::Overloaded));

        // Insert a user message
        thread.update(cx, |thread, cx| {
            thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
        });

        // Start completion
        thread.update(cx, |thread, cx| {
            thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
        });

        cx.run_until_parked();

        // Verify retry was scheduled by checking for retry message
        let has_retry_message = thread.read_with(cx, |thread, _| {
            thread.messages.iter().any(|m| {
                m.ui_only
                    && m.segments.iter().any(|s| {
                        if let MessageSegment::Text(text) = s {
                            text.contains("Retrying") && text.contains("seconds")
                        } else {
                            false
                        }
                    })
            })
        });
        assert!(has_retry_message, "Should have scheduled a retry");

        // Cancel the completion before the retry happens
        thread.update(cx, |thread, cx| {
            thread.cancel_last_completion(None, cx);
        });

        cx.run_until_parked();

        // The retry should not have happened - no pending completions
        let fake_model = model.as_fake();
        assert_eq!(
            fake_model.pending_completions().len(),
            0,
            "Should have no pending completions after cancellation"
        );

        // Verify the retry was canceled by checking retry state
        thread.read_with(cx, |thread, _| {
            if let Some(retry_state) = &thread.retry_state {
                panic!(
                    "retry_state should be cleared after cancellation, but found: attempt={}, max_attempts={}, intent={:?}",
                    retry_state.attempt, retry_state.max_attempts, retry_state.intent
                );
            }
        });
    }

    fn test_summarize_error(
        model: &Arc<dyn LanguageModel>,
        thread: &Entity<Thread>,
        cx: &mut TestAppContext,
    ) {
        thread.update(cx, |thread, cx| {
            thread.insert_user_message("Hi!", ContextLoadResult::default(), None, vec![], cx);
            thread.send_to_model(
                model.clone(),
                CompletionIntent::ThreadSummarization,
                None,
                cx,
            );
        });

        let fake_model = model.as_fake();
        simulate_successful_response(fake_model, cx);

        thread.read_with(cx, |thread, _| {
            assert!(matches!(thread.summary(), ThreadSummary::Generating));
            assert_eq!(thread.summary().or_default(), ThreadSummary::DEFAULT);
        });

        // Simulate summary request ending
        cx.run_until_parked();
        fake_model.end_last_completion_stream();
        cx.run_until_parked();

        // State is set to Error and default message
        thread.read_with(cx, |thread, _| {
            assert!(matches!(thread.summary(), ThreadSummary::Error));
            assert_eq!(thread.summary().or_default(), ThreadSummary::DEFAULT);
        });
    }

    fn simulate_successful_response(fake_model: &FakeLanguageModel, cx: &mut TestAppContext) {
        cx.run_until_parked();
        fake_model.send_last_completion_stream_text_chunk("Assistant response");
        fake_model.end_last_completion_stream();
        cx.run_until_parked();
    }

    fn init_test_settings(cx: &mut TestAppContext) -> Arc<dyn Fs> {
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
            AgentSettings::register(cx);
            prompt_store::init(cx);
            thread_store::init(fs.clone(), cx);
            workspace::init_settings(cx);
            language_model::init_settings(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            ToolRegistry::default_global(cx);
            assistant_tool::init(cx);

            let http_client = Arc::new(http_client::HttpClientWithUrl::new(
                http_client::FakeHttpClient::with_200_response(),
                "http://localhost".to_string(),
                None,
            ));
            assistant_tools::init(http_client, cx);
        });
        fs
    }

    // Helper to create a test project with test files
    async fn create_test_project(
        fs: &Arc<dyn Fs>,
        cx: &mut TestAppContext,
        files: serde_json::Value,
    ) -> Entity<Project> {
        fs.as_fake().insert_tree(path!("/test"), files).await;
        Project::test(fs.clone(), [path!("/test").as_ref()], cx).await
    }

    async fn setup_test_environment(
        cx: &mut TestAppContext,
        project: Entity<Project>,
    ) -> (
        Entity<Workspace>,
        Entity<ThreadStore>,
        Entity<Thread>,
        Entity<ContextStore>,
        Arc<dyn LanguageModel>,
    ) {
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let thread_store = cx
            .update(|_, cx| {
                ThreadStore::load(
                    project.clone(),
                    cx.new(|_| ToolWorkingSet::default()),
                    None,
                    Arc::new(PromptBuilder::new(None).unwrap()),
                    cx,
                )
            })
            .await
            .unwrap();

        let thread = thread_store.update(cx, |store, cx| store.create_thread(cx));
        let context_store = cx.new(|_cx| ContextStore::new(project.downgrade(), None));

        let provider = Arc::new(FakeLanguageModelProvider::default());
        let model = provider.test_model();
        let model: Arc<dyn LanguageModel> = Arc::new(model);

        cx.update(|_, cx| {
            LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
                registry.set_default_model(
                    Some(ConfiguredModel {
                        provider: provider.clone(),
                        model: model.clone(),
                    }),
                    cx,
                );
                registry.set_thread_summary_model(
                    Some(ConfiguredModel {
                        provider,
                        model: model.clone(),
                    }),
                    cx,
                );
            })
        });

        (workspace, thread_store, thread, context_store, model)
    }

    async fn add_file_to_context(
        project: &Entity<Project>,
        context_store: &Entity<ContextStore>,
        path: &str,
        cx: &mut TestAppContext,
    ) -> Result<Entity<language::Buffer>> {
        let buffer_path = project
            .read_with(cx, |project, cx| project.find_project_path(path, cx))
            .unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_buffer(buffer_path.clone(), cx)
            })
            .await
            .unwrap();

        context_store.update(cx, |context_store, cx| {
            context_store.add_file_from_buffer(&buffer_path, buffer.clone(), false, cx);
        });

        Ok(buffer)
    }
}
