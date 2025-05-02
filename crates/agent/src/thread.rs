use std::fmt::Write as _;
use std::io::Write;
use std::ops::Range;
use std::sync::Arc;
use std::time::Instant;

use anyhow::{Result, anyhow};
use assistant_settings::AssistantSettings;
use assistant_tool::{ActionLog, AnyToolCard, Tool, ToolWorkingSet};
use chrono::{DateTime, Utc};
use collections::HashMap;
use editor::display_map::CreaseMetadata;
use feature_flags::{self, FeatureFlagAppExt};
use futures::future::Shared;
use futures::{FutureExt, StreamExt as _};
use git::repository::DiffType;
use gpui::{
    AnyWindowHandle, App, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task,
    WeakEntity,
};
use language_model::{
    ConfiguredModel, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelKnownError, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelRequestTool, LanguageModelToolResult,
    LanguageModelToolUseId, MaxMonthlySpendReachedError, MessageContent,
    ModelRequestLimitReachedError, PaymentRequiredError, RequestUsage, Role, SelectedModel,
    StopReason, TokenUsage,
};
use postage::stream::Stream as _;
use project::Project;
use project::git_store::{GitStore, GitStoreCheckpoint, RepositoryState};
use prompt_store::{ModelContext, PromptBuilder};
use proto::Plan;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use thiserror::Error;
use util::{ResultExt as _, TryFutureExt as _, post_inc};
use uuid::Uuid;
use zed_llm_client::CompletionMode;

use crate::ThreadStore;
use crate::context::{AgentContext, AgentContextHandle, ContextLoadResult, LoadedContext};
use crate::thread_store::{
    SerializedCrease, SerializedLanguageModel, SerializedMessage, SerializedMessageSegment,
    SerializedThread, SerializedToolResult, SerializedToolUse, SharedProjectContext,
};
use crate::tool_use::{PendingToolUse, ToolUse, ToolUseMetadata, ToolUseState};

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
pub struct MessageId(pub(crate) usize);

impl MessageId {
    fn post_inc(&mut self) -> Self {
        Self(post_inc(&mut self.0))
    }
}

/// Stored information that can be used to resurrect a context crease when creating an editor for a past message.
#[derive(Clone, Debug)]
pub struct MessageCrease {
    pub range: Range<usize>,
    pub metadata: CreaseMetadata,
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

    pub fn push_text(&mut self, text: &str) {
        if let Some(MessageSegment::Text(segment)) = self.segments.last_mut() {
            segment.push_str(text);
        } else {
            self.segments.push(MessageSegment::Text(text.to_string()));
        }
    }

    pub fn to_string(&self) -> String {
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
    RedactedThinking(Vec<u8>),
}

impl MessageSegment {
    pub fn should_display(&self) -> bool {
        match self {
            Self::Text(text) => text.is_empty(),
            Self::Thinking { text, .. } => text.is_empty(),
            Self::RedactedThinking(_) => false,
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

impl DetailedSummaryState {
    fn text(&self) -> Option<SharedString> {
        if let Self::Generated { text, .. } = self {
            Some(text.clone())
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct TotalTokenUsage {
    pub total: usize,
    pub max: usize,
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

    pub fn add(&self, tokens: usize) -> TotalTokenUsage {
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

fn default_completion_mode(cx: &App) -> CompletionMode {
    if cx.is_staff() {
        CompletionMode::Max
    } else {
        CompletionMode::Normal
    }
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
    summary: Option<SharedString>,
    pending_summary: Task<Option<()>>,
    detailed_summary_task: Task<Option<()>>,
    detailed_summary_tx: postage::watch::Sender<DetailedSummaryState>,
    detailed_summary_rx: postage::watch::Receiver<DetailedSummaryState>,
    completion_mode: CompletionMode,
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
    feedback: Option<ThreadFeedback>,
    message_feedback: HashMap<MessageId, ThreadFeedback>,
    last_auto_capture_at: Option<Instant>,
    last_received_chunk_at: Option<Instant>,
    request_callback: Option<
        Box<dyn FnMut(&LanguageModelRequest, &[Result<LanguageModelCompletionEvent, String>])>,
    >,
    remaining_turns: u32,
    configured_model: Option<ConfiguredModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceededWindowError {
    /// Model used when last message exceeded context window
    model_id: LanguageModelId,
    /// Token count including last message
    token_count: usize,
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

        Self {
            id: ThreadId::new(),
            updated_at: Utc::now(),
            summary: None,
            pending_summary: Task::ready(None),
            detailed_summary_task: Task::ready(None),
            detailed_summary_tx,
            detailed_summary_rx,
            completion_mode: default_completion_mode(cx),
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
            feedback: None,
            message_feedback: HashMap::default(),
            last_auto_capture_at: None,
            last_received_chunk_at: None,
            request_callback: None,
            remaining_turns: u32::MAX,
            configured_model,
        }
    }

    pub fn deserialize(
        id: ThreadId,
        serialized: SerializedThread,
        project: Entity<Project>,
        tools: Entity<ToolWorkingSet>,
        prompt_builder: Arc<PromptBuilder>,
        project_context: SharedProjectContext,
        cx: &mut Context<Self>,
    ) -> Self {
        let next_message_id = MessageId(
            serialized
                .messages
                .last()
                .map(|message| message.id.0 + 1)
                .unwrap_or(0),
        );
        let tool_use = ToolUseState::from_serialized_messages(tools.clone(), &serialized.messages);
        let (detailed_summary_tx, detailed_summary_rx) =
            postage::watch::channel_with(serialized.detailed_summary_state);

        let configured_model = LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            serialized
                .model
                .and_then(|model| {
                    let model = SelectedModel {
                        provider: model.provider.clone().into(),
                        model: model.model.clone().into(),
                    };
                    registry.select_model(&model, cx)
                })
                .or_else(|| registry.default_model())
        });

        Self {
            id,
            updated_at: serialized.updated_at,
            summary: Some(serialized.summary),
            pending_summary: Task::ready(None),
            detailed_summary_task: Task::ready(None),
            detailed_summary_tx,
            detailed_summary_rx,
            completion_mode: default_completion_mode(cx),
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
                            metadata: CreaseMetadata {
                                icon_path: crease.icon_path,
                                label: crease.label,
                            },
                            context: None,
                        })
                        .collect(),
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
            tools,
            tool_use,
            action_log: cx.new(|_| ActionLog::new(project)),
            initial_project_snapshot: Task::ready(serialized.initial_project_snapshot).shared(),
            request_token_usage: serialized.request_token_usage,
            cumulative_token_usage: serialized.cumulative_token_usage,
            exceeded_window_error: None,
            feedback: None,
            message_feedback: HashMap::default(),
            last_auto_capture_at: None,
            last_received_chunk_at: None,
            request_callback: None,
            remaining_turns: u32::MAX,
            configured_model,
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

    pub fn summary(&self) -> Option<SharedString> {
        self.summary.clone()
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

    pub const DEFAULT_SUMMARY: SharedString = SharedString::new_static("New Thread");

    pub fn summary_or_default(&self) -> SharedString {
        self.summary.clone().unwrap_or(Self::DEFAULT_SUMMARY)
    }

    pub fn set_summary(&mut self, new_summary: impl Into<SharedString>, cx: &mut Context<Self>) {
        let Some(current_summary) = &self.summary else {
            // Don't allow setting summary until generated
            return;
        };

        let mut new_summary = new_summary.into();

        if new_summary.is_empty() {
            new_summary = Self::DEFAULT_SUMMARY;
        }

        if current_summary != &new_summary {
            self.summary = Some(new_summary);
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

                if !equal {
                    this.update(cx, |this, cx| {
                        this.insert_checkpoint(pending_checkpoint, cx)
                    })?;
                }

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
                    .map(|next_message| next_message.role == Role::User)
            })
            .unwrap_or(false)
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
        Some(&self.tool_use.tool_result(id)?.content)
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
            self.tools()
                .read(cx)
                .enabled_tools(cx)
                .into_iter()
                .filter_map(|tool| {
                    // Skip tools that cannot be supported
                    let input_schema = tool.input_schema(model.tool_input_format()).ok()?;
                    Some(LanguageModelRequestTool {
                        name: tool.name(),
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
                    log.track_buffer(buffer, cx);
                }
            });
        }

        let message_id = self.insert_message(
            Role::User,
            vec![MessageSegment::Text(text.into())],
            loaded_context.loaded_context,
            creases,
            cx,
        );

        if let Some(git_checkpoint) = git_checkpoint {
            self.pending_checkpoint = Some(ThreadCheckpoint {
                message_id,
                git_checkpoint,
            });
        }

        self.auto_capture_telemetry(cx);

        message_id
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
            cx,
        )
    }

    pub fn insert_message(
        &mut self,
        role: Role,
        segments: Vec<MessageSegment>,
        loaded_context: LoadedContext,
        creases: Vec<MessageCrease>,
        cx: &mut Context<Self>,
    ) -> MessageId {
        let id = self.next_message_id.post_inc();
        self.messages.push(Message {
            id,
            role,
            segments,
            loaded_context,
            creases,
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
        loaded_context: Option<LoadedContext>,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(message) = self.messages.iter_mut().find(|message| message.id == id) else {
            return false;
        };
        message.role = new_role;
        message.segments = new_segments;
        if let Some(context) = loaded_context {
            message.loaded_context = context;
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
                language_model::Role::Assistant => "Assistant:",
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
                            })
                            .collect(),
                        context: message.loaded_context.text.clone(),
                        creases: message
                            .creases
                            .iter()
                            .map(|crease| SerializedCrease {
                                start: crease.range.start,
                                end: crease.range.end,
                                icon_path: crease.metadata.icon_path.clone(),
                                label: crease.metadata.label.clone(),
                            })
                            .collect(),
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
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Self>,
    ) {
        if self.remaining_turns == 0 {
            return;
        }

        self.remaining_turns -= 1;

        let request = self.to_completion_request(model.clone(), cx);

        self.stream_completion(request, model, window, cx);
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
        model: Arc<dyn LanguageModel>,
        cx: &mut Context<Self>,
    ) -> LanguageModelRequest {
        let mut request = LanguageModelRequest {
            thread_id: Some(self.id.to_string()),
            prompt_id: Some(self.last_prompt_id.to_string()),
            mode: None,
            messages: vec![],
            tools: Vec::new(),
            stop: Vec::new(),
            temperature: None,
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

        for message in &self.messages {
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

            self.tool_use
                .attach_tool_uses(message.id, &mut request_message);

            request.messages.push(request_message);

            if let Some(tool_results_message) = self.tool_use.tool_results_message(message.id) {
                request.messages.push(tool_results_message);
            }
        }

        // https://docs.anthropic.com/en/docs/build-with-claude/prompt-caching
        if let Some(last) = request.messages.last_mut() {
            last.cache = true;
        }

        self.attached_tracked_files_state(&mut request.messages, cx);

        request.tools = available_tools;
        request.mode = if model.supports_max_mode() {
            Some(self.completion_mode)
        } else {
            Some(CompletionMode::Normal)
        };

        request
    }

    fn to_summarize_request(&self, added_user_message: String) -> LanguageModelRequest {
        let mut request = LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            mode: None,
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
                write!(&mut stale_message, "{}\n", STALE_FILES_HEADER).ok();
            }

            writeln!(&mut stale_message, "- {}", file.path().display()).ok();
        }

        let mut content = Vec::with_capacity(2);

        if !stale_message.is_empty() {
            content.push(stale_message.into());
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
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Self>,
    ) {
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

        self.last_received_chunk_at = Some(Instant::now());

        let task = cx.spawn(async move |thread, cx| {
            let stream_completion_future = model.stream_completion_with_usage(request, &cx);
            let initial_token_usage =
                thread.read_with(cx, |thread, _cx| thread.cumulative_token_usage);
            let stream_completion = async {
                let (mut events, usage) = stream_completion_future.await?;

                let mut stop_reason = StopReason::EndTurn;
                let mut current_token_usage = TokenUsage::default();

                if let Some(usage) = usage {
                    thread
                        .update(cx, |_thread, cx| {
                            cx.emit(ThreadEvent::UsageUpdated(usage));
                        })
                        .ok();
                }

                let mut request_assistant_message_id = None;

                while let Some(event) = events.next().await {
                    if let Some((_, response_events)) = request_callback_parameters.as_mut() {
                        response_events
                            .push(event.as_ref().map_err(|error| error.to_string()).cloned());
                    }

                    thread.update(cx, |thread, cx| {
                        let event = match event {
                            Ok(event) => event,
                            Err(LanguageModelCompletionError::BadInputJson {
                                id,
                                tool_name,
                                raw_input: invalid_input_json,
                                json_parse_error,
                            }) => {
                                thread.receive_invalid_tool_json(
                                    id,
                                    tool_name,
                                    invalid_input_json,
                                    json_parse_error,
                                    window,
                                    cx,
                                );
                                return Ok(());
                            }
                            Err(LanguageModelCompletionError::Other(error)) => {
                                return Err(error);
                            }
                        };

                        match event {
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
                                    Some((&tool_use.input).clone())
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
                            LanguageModelCompletionEvent::QueueUpdate(queue_event) => {
                                if let Some(completion) = thread
                                    .pending_completions
                                    .iter_mut()
                                    .find(|completion| completion.id == pending_completion_id)
                                {
                                    completion.queue_state = match queue_event {
                                        language_model::QueueState::Queued { position } => {
                                            QueueState::Queued { position }
                                        }
                                        language_model::QueueState::Started => QueueState::Started,
                                    }
                                }
                            }
                        }

                        thread.touch_updated_at();
                        cx.emit(ThreadEvent::StreamedCompletion);
                        cx.notify();

                        thread.auto_capture_telemetry(cx);
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
                    if thread.summary.is_none()
                        && thread.messages.len() >= 2
                        && (!thread.has_pending_tool_uses() || thread.messages.len() >= 6)
                    {
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
                                let tool_uses = thread.use_pending_tools(window, cx, model.clone());
                                cx.emit(ThreadEvent::UsePendingTools { tool_uses });
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
                            } else if let Some(error) =
                                error.downcast_ref::<ModelRequestLimitReachedError>()
                            {
                                cx.emit(ThreadEvent::ShowError(
                                    ThreadError::ModelRequestLimitReached { plan: error.plan },
                                ));
                            } else if let Some(known_error) =
                                error.downcast_ref::<LanguageModelKnownError>()
                            {
                                match known_error {
                                    LanguageModelKnownError::ContextWindowLimitExceeded {
                                        tokens,
                                    } => {
                                        thread.exceeded_window_error = Some(ExceededWindowError {
                                            model_id: model.id(),
                                            token_count: *tokens,
                                        });
                                        cx.notify();
                                    }
                                }
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

                            thread.cancel_last_completion(window, cx);
                        }
                    }
                    cx.emit(ThreadEvent::Stopped(result.map_err(Arc::new)));

                    if let Some((request_callback, (request, response_events))) = thread
                        .request_callback
                        .as_mut()
                        .zip(request_callback_parameters.as_ref())
                    {
                        request_callback(request, response_events);
                    }

                    thread.auto_capture_telemetry(cx);

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
            return;
        };

        if !model.provider.is_authenticated(cx) {
            return;
        }

        let added_user_message = "Generate a concise 3-7 word title for this conversation, omitting punctuation. \
            Go straight to the title, without any preamble and prefix like `Here's a concise suggestion:...` or `Title:`. \
            If the conversation is about a specific subject, include it in the title. \
            Be descriptive. DO NOT speak in the first person.";

        let request = self.to_summarize_request(added_user_message.into());

        self.pending_summary = cx.spawn(async move |this, cx| {
            async move {
                let stream = model.model.stream_completion_text_with_usage(request, &cx);
                let (mut messages, usage) = stream.await?;

                if let Some(usage) = usage {
                    this.update(cx, |_thread, cx| {
                        cx.emit(ThreadEvent::UsageUpdated(usage));
                    })
                    .ok();
                }

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

                    cx.emit(ThreadEvent::SummaryGenerated);
                })?;

                anyhow::Ok(())
            }
            .log_err()
            .await
        });
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

        let added_user_message = "Generate a detailed summary of this conversation. Include:\n\
             1. A brief overview of what was discussed\n\
             2. Key facts or information discovered\n\
             3. Outcomes or conclusions reached\n\
             4. Any action items or next steps if any\n\
             Format it in Markdown with headings and bullet points.";

        let request = self.to_summarize_request(added_user_message.into());

        *self.detailed_summary_tx.borrow_mut() = DetailedSummaryState::Generating {
            message_id: last_message_id,
        };

        // Replace the detailed summarization task if there is one, cancelling it. It would probably
        // be better to allow the old task to complete, but this would require logic for choosing
        // which result to prefer (the old task could complete after the new one, resulting in a
        // stale summary).
        self.detailed_summary_task = cx.spawn(async move |thread, cx| {
            let stream = model.stream_completion_text(request, &cx);
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
            if let Some(thread) = thread.upgrade() {
                if let Ok(Ok(save_task)) = cx.update(|cx| {
                    thread_store
                        .update(cx, |thread_store, cx| thread_store.save_thread(&thread, cx))
                }) {
                    save_task.await.log_err();
                }
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
        cx: &mut Context<Self>,
        model: Arc<dyn LanguageModel>,
    ) -> Vec<PendingToolUse> {
        self.auto_capture_telemetry(cx);
        let request = self.to_completion_request(model, cx);
        let messages = Arc::new(request.messages);
        let pending_tool_uses = self
            .tool_use
            .pending_tool_uses()
            .into_iter()
            .filter(|tool_use| tool_use.status.is_idle())
            .cloned()
            .collect::<Vec<_>>();

        for tool_use in pending_tool_uses.iter() {
            if let Some(tool) = self.tools.read(cx).tool(&tool_use.name, cx) {
                if tool.needs_confirmation(&tool_use.input, cx)
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
                        window,
                        cx,
                    );
                }
            }
        }

        pending_tool_uses
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
        messages: &[LanguageModelRequestMessage],
        tool: Arc<dyn Tool>,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Thread>,
    ) {
        let task = self.spawn_tool_use(tool_use_id.clone(), messages, input, tool, window, cx);
        self.tool_use
            .run_pending_tool(tool_use_id, ui_text.into(), task);
    }

    fn spawn_tool_use(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        messages: &[LanguageModelRequestMessage],
        input: serde_json::Value,
        tool: Arc<dyn Tool>,
        window: Option<AnyWindowHandle>,
        cx: &mut Context<Thread>,
    ) -> Task<()> {
        let tool_name: Arc<str> = tool.name().into();

        let tool_result = if self.tools.read(cx).is_disabled(&tool.source(), &tool_name) {
            Task::ready(Err(anyhow!("tool is disabled: {tool_name}"))).into()
        } else {
            tool.run(
                input,
                messages,
                self.project.clone(),
                self.action_log.clone(),
                window,
                cx,
            )
        };

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
        if self.all_tools_finished() {
            if let Some(ConfiguredModel { model, .. }) = self.configured_model.as_ref() {
                if !canceled {
                    self.send_to_model(model.clone(), window, cx);
                }
                self.auto_capture_telemetry(cx);
            }
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
        let mut canceled = self.pending_completions.pop().is_some();

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

        self.finalize_pending_checkpoint(cx);
        canceled
    }

    /// Signals that any in-progress editing should be canceled.
    ///
    /// This method is used to notify listeners (like ActiveThread) that
    /// they should cancel any editing operations.
    pub fn cancel_editing(&mut self, cx: &mut Context<Self>) {
        cx.emit(ThreadEvent::CancelEditing);
    }

    pub fn feedback(&self) -> Option<ThreadFeedback> {
        self.feedback
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
            .tools()
            .read(cx)
            .enabled_tools(cx)
            .iter()
            .map(|tool| tool.name().to_string())
            .collect();

        self.message_feedback.insert(message_id, feedback);

        cx.notify();

        let message_content = self
            .message(message_id)
            .map(|msg| msg.to_string())
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

    pub fn report_feedback(
        &mut self,
        feedback: ThreadFeedback,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let last_assistant_message_id = self
            .messages
            .iter()
            .rev()
            .find(|msg| msg.role == Role::Assistant)
            .map(|msg| msg.id);

        if let Some(message_id) = last_assistant_message_id {
            self.report_message_feedback(message_id, feedback, cx)
        } else {
            let final_project_snapshot = Self::project_snapshot(self.project.clone(), cx);
            let serialized_thread = self.serialize(cx);
            let thread_id = self.id().clone();
            let client = self.project.read(cx).client();
            self.feedback = Some(feedback);
            cx.notify();

            cx.background_spawn(async move {
                let final_project_snapshot = final_project_snapshot.await;
                let serialized_thread = serialized_thread.await?;
                let thread_data = serde_json::to_value(serialized_thread)
                    .unwrap_or_else(|_| serde_json::Value::Null);

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
                client.telemetry().flush_events().await;

                Ok(())
            })
        }
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

    pub fn auto_capture_telemetry(&mut self, cx: &mut Context<Self>) {
        if !cx.has_flag::<feature_flags::ThreadAutoCaptureFeatureFlag>() {
            return;
        }

        let now = Instant::now();
        if let Some(last) = self.last_auto_capture_at {
            if now.duration_since(last).as_secs() < 10 {
                return;
            }
        }

        self.last_auto_capture_at = Some(now);

        let thread_id = self.id().clone();
        let github_login = self
            .project
            .read(cx)
            .user_store()
            .read(cx)
            .current_user()
            .map(|user| user.github_login.clone());
        let client = self.project.read(cx).client().clone();
        let serialize_task = self.serialize(cx);

        cx.background_executor()
            .spawn(async move {
                if let Ok(serialized_thread) = serialize_task.await {
                    if let Ok(thread_data) = serde_json::to_value(serialized_thread) {
                        telemetry::event!(
                            "Agent Thread Auto-Captured",
                            thread_id = thread_id.to_string(),
                            thread_data = thread_data,
                            auto_capture_reason = "tracked_user",
                            github_login = github_login
                        );

                        client.telemetry().flush_events().await;
                    }
                }
            })
            .detach();
    }

    pub fn cumulative_token_usage(&self) -> TokenUsage {
        self.cumulative_token_usage
    }

    pub fn token_usage_up_to_message(&self, message_id: MessageId) -> TotalTokenUsage {
        let Some(model) = self.configured_model.as_ref() else {
            return TotalTokenUsage::default();
        };

        let max = model.model.max_token_count();

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
            total: token_usage.total_tokens() as usize,
            max,
        }
    }

    pub fn total_token_usage(&self) -> Option<TotalTokenUsage> {
        let model = self.configured_model.as_ref()?;

        let max = model.model.max_token_count();

        if let Some(exceeded_error) = &self.exceeded_window_error {
            if model.model.id() == exceeded_error.model_id {
                return Some(TotalTokenUsage {
                    total: exceeded_error.token_count,
                    max,
                });
            }
        }

        let total = self
            .token_usage_at_last_message()
            .unwrap_or_default()
            .total_tokens() as usize;

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
        );
        self.tool_finished(tool_use_id.clone(), None, true, window, cx);
    }
}

#[derive(Debug, Clone, Error)]
pub enum ThreadError {
    #[error("Payment required")]
    PaymentRequired,
    #[error("Max monthly spend reached")]
    MaxMonthlySpendReached,
    #[error("Model request limit reached")]
    ModelRequestLimitReached { plan: Plan },
    #[error("Message {header}: {message}")]
    Message {
        header: SharedString,
        message: SharedString,
    },
}

#[derive(Debug, Clone)]
pub enum ThreadEvent {
    ShowError(ThreadError),
    UsageUpdated(RequestUsage),
    StreamedCompletion,
    ReceivedTextChunk,
    StreamedAssistantText(MessageId, String),
    StreamedAssistantThinking(MessageId, String),
    StreamedToolUse {
        tool_use_id: LanguageModelToolUseId,
        ui_text: Arc<str>,
        input: serde_json::Value,
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
    CancelEditing,
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
    use crate::{ThreadStore, context::load_context, context_store::ContextStore, thread_store};
    use assistant_settings::AssistantSettings;
    use assistant_tool::ToolRegistry;
    use context_server::ContextServerSettings;
    use editor::EditorSettings;
    use gpui::TestAppContext;
    use language_model::fake_provider::FakeLanguageModel;
    use project::{FakeFs, Project};
    use prompt_store::PromptBuilder;
    use serde_json::json;
    use settings::{Settings, SettingsStore};
    use std::sync::Arc;
    use theme::ThemeSettings;
    use util::path;
    use workspace::Workspace;

    #[gpui::test]
    async fn test_message_with_context(cx: &mut TestAppContext) {
        init_test_settings(cx);

        let project = create_test_project(
            cx,
            json!({"code.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"}),
        )
        .await;

        let (_workspace, _thread_store, thread, context_store, model) =
            setup_test_environment(cx, project.clone()).await;

        add_file_to_context(&project, &context_store, "test/code.rs", cx)
            .await
            .unwrap();

        let context = context_store.update(cx, |store, _| store.context().next().cloned().unwrap());
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
            thread.to_completion_request(model.clone(), cx)
        });

        assert_eq!(request.messages.len(), 2);
        let expected_full_message = format!("{}Please explain this code", expected_context);
        assert_eq!(request.messages[1].string_contents(), expected_full_message);
    }

    #[gpui::test]
    async fn test_only_include_new_contexts(cx: &mut TestAppContext) {
        init_test_settings(cx);

        let project = create_test_project(
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
            thread.to_completion_request(model.clone(), cx)
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
        init_test_settings(cx);

        let project = create_test_project(
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
            thread.to_completion_request(model.clone(), cx)
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
            thread.to_completion_request(model.clone(), cx)
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
    async fn test_stale_buffer_notification(cx: &mut TestAppContext) {
        init_test_settings(cx);

        let project = create_test_project(
            cx,
            json!({"code.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"}),
        )
        .await;

        let (_workspace, _thread_store, thread, context_store, model) =
            setup_test_environment(cx, project.clone()).await;

        // Open buffer and add it to context
        let buffer = add_file_to_context(&project, &context_store, "test/code.rs", cx)
            .await
            .unwrap();

        let context = context_store.update(cx, |store, _| store.context().next().cloned().unwrap());
        let loaded_context = cx
            .update(|cx| load_context(vec![context], &project, &None, cx))
            .await;

        // Insert user message with the buffer as context
        thread.update(cx, |thread, cx| {
            thread.insert_user_message("Explain this code", loaded_context, None, Vec::new(), cx)
        });

        // Create a request and check that it doesn't have a stale buffer warning yet
        let initial_request = thread.update(cx, |thread, cx| {
            thread.to_completion_request(model.clone(), cx)
        });

        // Make sure we don't have a stale file warning yet
        let has_stale_warning = initial_request.messages.iter().any(|msg| {
            msg.string_contents()
                .contains("These files changed since last read:")
        });
        assert!(
            !has_stale_warning,
            "Should not have stale buffer warning before buffer is modified"
        );

        // Modify the buffer
        buffer.update(cx, |buffer, cx| {
            // Find a position at the end of line 1
            buffer.edit(
                [(1..1, "\n    println!(\"Added a new line\");\n")],
                None,
                cx,
            );
        });

        // Insert another user message without context
        thread.update(cx, |thread, cx| {
            thread.insert_user_message(
                "What does the code do now?",
                ContextLoadResult::default(),
                None,
                Vec::new(),
                cx,
            )
        });

        // Create a new request and check for the stale buffer warning
        let new_request = thread.update(cx, |thread, cx| {
            thread.to_completion_request(model.clone(), cx)
        });

        // We should have a stale file warning as the last message
        let last_message = new_request
            .messages
            .last()
            .expect("Request should have messages");

        // The last message should be the stale buffer notification
        assert_eq!(last_message.role, Role::User);

        // Check the exact content of the message
        let expected_content = "These files changed since last read:\n- code.rs\n";
        assert_eq!(
            last_message.string_contents(),
            expected_content,
            "Last message should be exactly the stale buffer notification"
        );
    }

    fn init_test_settings(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
            AssistantSettings::register(cx);
            prompt_store::init(cx);
            thread_store::init(cx);
            workspace::init_settings(cx);
            language_model::init_settings(cx);
            ThemeSettings::register(cx);
            ContextServerSettings::register(cx);
            EditorSettings::register(cx);
            ToolRegistry::default_global(cx);
        });
    }

    // Helper to create a test project with test files
    async fn create_test_project(
        cx: &mut TestAppContext,
        files: serde_json::Value,
    ) -> Entity<Project> {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), files).await;
        Project::test(fs, [path!("/test").as_ref()], cx).await
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

        let model = FakeLanguageModel::default();
        let model: Arc<dyn LanguageModel> = Arc::new(model);

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
