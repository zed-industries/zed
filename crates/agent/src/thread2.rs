use crate::{
    AgentThread, AgentThreadId, AgentThreadMessageId, AgentThreadUserMessageChunk,
    agent_profile::AgentProfile,
    context::{AgentContext, AgentContextHandle, ContextLoadResult, LoadedContext},
    thread_store::{SharedProjectContext, ThreadStore},
};
use agent_settings::{AgentProfileId, AgentSettings, CompletionMode};
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, AnyToolCard, Tool, ToolWorkingSet};
use chrono::{DateTime, Utc};
use client::{ModelRequestUsage, RequestUsage};
use collections::{HashMap, HashSet};
use feature_flags::{self, FeatureFlagAppExt};
use futures::{FutureExt, StreamExt as _, channel::oneshot, future::Shared};
use git::repository::DiffType;
use gpui::{
    AnyWindowHandle, App, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task,
    WeakEntity,
};
use language_model::{
    ConfiguredModel, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelKnownError, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelRequestTool, LanguageModelToolResult,
    LanguageModelToolResultContent, LanguageModelToolUseId, MessageContent,
    ModelRequestLimitReachedError, PaymentRequiredError, Role, StopReason, TokenUsage,
};
use postage::stream::Stream as _;
use project::{
    Project,
    git_store::{GitStore, GitStoreCheckpoint, RepositoryState},
};
use prompt_store::{ModelContext, PromptBuilder};
use proto::Plan;
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
use zed_llm_client::{CompletionIntent, CompletionRequestStatus, UsageLimit};

/// Stored information that can be used to resurrect a context crease when creating an editor for a past message.
#[derive(Clone, Debug)]
pub struct MessageCrease {
    pub range: Range<usize>,
    pub icon_path: SharedString,
    pub label: SharedString,
    /// None for a deserialized message, Some otherwise.
    pub context: Option<AgentContextHandle>,
}

pub enum MessageTool {
    Pending {
        tool: Arc<dyn Tool>,
        input: serde_json::Value,
    },
    NeedsConfirmation {
        tool: Arc<dyn Tool>,
        input_json: serde_json::Value,
        confirm_tx: oneshot::Sender<bool>,
    },
    Confirmed {
        card: AnyToolCard,
    },
    Declined {
        tool: Arc<dyn Tool>,
        input_json: serde_json::Value,
    },
}

/// A message in a [`Thread`].
pub struct Message {
    pub id: AgentThreadMessageId,
    pub role: Role,
    pub thinking: String,
    pub text: String,
    pub tools: Vec<MessageTool>,
    pub loaded_context: LoadedContext,
    pub creases: Vec<MessageCrease>,
    pub is_hidden: bool,
    pub ui_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectSnapshot {
    pub worktree_snapshots: Vec<WorktreeSnapshot>,
    pub unsaved_buffer_paths: Vec<String>,
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
    message_id: AgentThreadMessageId,
    git_checkpoint: GitStoreCheckpoint,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ThreadFeedback {
    Positive,
    Negative,
}

pub enum LastRestoreCheckpoint {
    Pending {
        message_id: AgentThreadMessageId,
    },
    Error {
        message_id: AgentThreadMessageId,
        error: String,
    },
}

impl LastRestoreCheckpoint {
    pub fn message_id(&self) -> AgentThreadMessageId {
        match self {
            LastRestoreCheckpoint::Pending { message_id } => *message_id,
            LastRestoreCheckpoint::Error { message_id, .. } => *message_id,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum DetailedSummaryState {
    #[default]
    NotGenerated,
    Generating {
        message_id: AgentThreadMessageId,
    },
    Generated {
        text: SharedString,
        message_id: AgentThreadMessageId,
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
    agent_thread: Arc<dyn AgentThread>,
    summary: ThreadSummary,
    pending_send: Option<Task<Result<()>>>,
    pending_summary: Task<Option<()>>,
    detailed_summary_task: Task<Option<()>>,
    detailed_summary_tx: postage::watch::Sender<DetailedSummaryState>,
    detailed_summary_rx: postage::watch::Receiver<DetailedSummaryState>,
    completion_mode: agent_settings::CompletionMode,
    messages: Vec<Message>,
    checkpoints_by_message: HashMap<AgentThreadMessageId, ThreadCheckpoint>,
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
    last_restore_checkpoint: Option<LastRestoreCheckpoint>,
    pending_checkpoint: Option<ThreadCheckpoint>,
    initial_project_snapshot: Shared<Task<Option<Arc<ProjectSnapshot>>>>,
    request_token_usage: Vec<TokenUsage>,
    cumulative_token_usage: TokenUsage,
    exceeded_window_error: Option<ExceededWindowError>,
    tool_use_limit_reached: bool,
    // todo!(keep track of retries from the underlying agent)
    feedback: Option<ThreadFeedback>,
    message_feedback: HashMap<AgentThreadMessageId, ThreadFeedback>,
    last_auto_capture_at: Option<Instant>,
    last_received_chunk_at: Option<Instant>,
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
    pub fn load(
        agent_thread: Arc<dyn AgentThread>,
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Self {
        let (detailed_summary_tx, detailed_summary_rx) = postage::watch::channel();
        Self {
            agent_thread,
            summary: ThreadSummary::Pending,
            pending_send: None,
            pending_summary: Task::ready(None),
            detailed_summary_task: Task::ready(None),
            detailed_summary_tx,
            detailed_summary_rx,
            completion_mode: AgentSettings::get_global(cx).preferred_completion_mode,
            messages: todo!("read from agent"),
            checkpoints_by_message: HashMap::default(),
            project: project.clone(),
            last_restore_checkpoint: None,
            pending_checkpoint: None,
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
            feedback: None,
            message_feedback: HashMap::default(),
            last_auto_capture_at: None,
            last_received_chunk_at: None,
        }
    }

    pub fn id(&self) -> AgentThreadId {
        self.agent_thread.id()
    }

    pub fn profile(&self) -> &AgentProfile {
        todo!()
    }

    pub fn set_profile(&mut self, id: AgentProfileId, cx: &mut Context<Self>) {
        todo!()
        // if &id != self.profile.id() {
        //     self.profile = AgentProfile::new(id, self.tools.clone());
        //     cx.emit(ThreadEvent::ProfileChanged);
        // }
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn advance_prompt_id(&mut self) {
        todo!()
        // self.last_prompt_id = PromptId::new();
    }

    pub fn project_context(&self) -> SharedProjectContext {
        todo!()
        // self.project_context.clone()
    }

    pub fn summary(&self) -> &ThreadSummary {
        &self.summary
    }

    pub fn set_summary(&mut self, new_summary: impl Into<SharedString>, cx: &mut Context<Self>) {
        todo!()
        // let current_summary = match &self.summary {
        //     ThreadSummary::Pending | ThreadSummary::Generating => return,
        //     ThreadSummary::Ready(summary) => summary,
        //     ThreadSummary::Error => &ThreadSummary::DEFAULT,
        // };

        // let mut new_summary = new_summary.into();

        // if new_summary.is_empty() {
        //     new_summary = ThreadSummary::DEFAULT;
        // }

        // if current_summary != &new_summary {
        //     self.summary = ThreadSummary::Ready(new_summary);
        //     cx.emit(ThreadEvent::SummaryChanged);
        // }
    }

    pub fn completion_mode(&self) -> CompletionMode {
        self.completion_mode
    }

    pub fn set_completion_mode(&mut self, mode: CompletionMode) {
        self.completion_mode = mode;
    }

    pub fn message(&self, id: AgentThreadMessageId) -> Option<&Message> {
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
        self.pending_send.is_some()
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

    pub fn checkpoint_for_message(&self, id: AgentThreadMessageId) -> Option<ThreadCheckpoint> {
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

    pub fn truncate(&mut self, message_id: AgentThreadMessageId, cx: &mut Context<Self>) {
        todo!("call truncate on the agent");
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

    pub fn is_turn_end(&self, ix: usize) -> bool {
        todo!()
        // if self.messages.is_empty() {
        //     return false;
        // }

        // if !self.is_generating() && ix == self.messages.len() - 1 {
        //     return true;
        // }

        // let Some(message) = self.messages.get(ix) else {
        //     return false;
        // };

        // if message.role != Role::Assistant {
        //     return false;
        // }

        // self.messages
        //     .get(ix + 1)
        //     .and_then(|message| {
        //         self.message(message.id)
        //             .map(|next_message| next_message.role == Role::User && !next_message.is_hidden)
        //     })
        //     .unwrap_or(false)
    }

    pub fn tool_use_limit_reached(&self) -> bool {
        self.tool_use_limit_reached
    }

    /// Returns whether any pending tool uses may perform edits
    pub fn has_pending_edit_tool_uses(&self) -> bool {
        todo!()
    }

    // pub fn insert_user_message(
    //     &mut self,
    //     text: impl Into<String>,
    //     loaded_context: ContextLoadResult,
    //     git_checkpoint: Option<GitStoreCheckpoint>,
    //     creases: Vec<MessageCrease>,
    //     cx: &mut Context<Self>,
    // ) -> AgentThreadMessageId {
    //     todo!("move this logic into send")
    //     if !loaded_context.referenced_buffers.is_empty() {
    //         self.action_log.update(cx, |log, cx| {
    //             for buffer in loaded_context.referenced_buffers {
    //                 log.buffer_read(buffer, cx);
    //             }
    //         });
    //     }

    //     let message_id = self.insert_message(
    //         Role::User,
    //         vec![MessageSegment::Text(text.into())],
    //         loaded_context.loaded_context,
    //         creases,
    //         false,
    //         cx,
    //     );

    //     if let Some(git_checkpoint) = git_checkpoint {
    //         self.pending_checkpoint = Some(ThreadCheckpoint {
    //             message_id,
    //             git_checkpoint,
    //         });
    // }

    // self.auto_capture_telemetry(cx);

    // message_id
    // }

    pub fn send(&mut self, message: Vec<AgentThreadUserMessageChunk>, cx: &mut Context<Self>) {}

    pub fn resume(&mut self, cx: &mut Context<Self>) {
        todo!()
    }

    pub fn edit(
        &mut self,
        message_id: AgentThreadMessageId,
        message: Vec<AgentThreadUserMessageChunk>,
        cx: &mut Context<Self>,
    ) {
        todo!()
    }

    pub fn cancel(&mut self, cx: &mut Context<Self>) {
        todo!()
    }

    // pub fn insert_invisible_continue_message(
    //     &mut self,
    //     cx: &mut Context<Self>,
    // ) -> AgentThreadMessageId {
    //     let id = self.insert_message(
    //         Role::User,
    //         vec![MessageSegment::Text("Continue where you left off".into())],
    //         LoadedContext::default(),
    //         vec![],
    //         true,
    //         cx,
    //     );
    //     self.pending_checkpoint = None;

    //     id
    // }

    // pub fn insert_assistant_message(
    //     &mut self,
    //     segments: Vec<MessageSegment>,
    //     cx: &mut Context<Self>,
    // ) -> AgentThreadMessageId {
    //     self.insert_message(
    //         Role::Assistant,
    //         segments,
    //         LoadedContext::default(),
    //         Vec::new(),
    //         false,
    //         cx,
    //     )
    // }

    // pub fn insert_message(
    //     &mut self,
    //     role: Role,
    //     segments: Vec<MessageSegment>,
    //     loaded_context: LoadedContext,
    //     creases: Vec<MessageCrease>,
    //     is_hidden: bool,
    //     cx: &mut Context<Self>,
    // ) -> AgentThreadMessageId {
    //     let id = self.next_message_id.post_inc();
    //     self.messages.push(Message {
    //         id,
    //         role,
    //         segments,
    //         loaded_context,
    //         creases,
    //         is_hidden,
    //         ui_only: false,
    //     });
    //     self.touch_updated_at();
    //     cx.emit(ThreadEvent::MessageAdded(id));
    //     id
    // }

    // pub fn edit_message(
    //     &mut self,
    //     id: AgentThreadMessageId,
    //     new_role: Role,
    //     new_segments: Vec<MessageSegment>,
    //     creases: Vec<MessageCrease>,
    //     loaded_context: Option<LoadedContext>,
    //     checkpoint: Option<GitStoreCheckpoint>,
    //     cx: &mut Context<Self>,
    // ) -> bool {
    //     let Some(message) = self.messages.iter_mut().find(|message| message.id == id) else {
    //         return false;
    //     };
    //     message.role = new_role;
    //     message.segments = new_segments;
    //     message.creases = creases;
    //     if let Some(context) = loaded_context {
    //         message.loaded_context = context;
    //     }
    //     if let Some(git_checkpoint) = checkpoint {
    //         self.checkpoints_by_message.insert(
    //             id,
    //             ThreadCheckpoint {
    //                 message_id: id,
    //                 git_checkpoint,
    //             },
    //         );
    //     }
    //     self.touch_updated_at();
    //     cx.emit(ThreadEvent::MessageEdited(id));
    //     true
    // }

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

            text.push_str("<think>");
            text.push_str(&message.thinking);
            text.push_str("</think>");
            text.push_str(&message.text);

            // todo!('what about tools?');

            text.push('\n');
        }

        text
    }

    pub fn used_tools_since_last_user_message(&self) -> bool {
        todo!()
        // for message in self.messages.iter().rev() {
        //     if self.tool_use.message_has_tool_results(message.id) {
        //         return true;
        //     } else if message.role == Role::User {
        //         return false;
        //     }
        // }

        // false
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

        let summary = self.agent_thread.summary();

        *self.detailed_summary_tx.borrow_mut() = DetailedSummaryState::Generating {
            message_id: last_message_id,
        };

        // Replace the detailed summarization task if there is one, cancelling it. It would probably
        // be better to allow the old task to complete, but this would require logic for choosing
        // which result to prefer (the old task could complete after the new one, resulting in a
        // stale summary).
        self.detailed_summary_task = cx.spawn(async move |thread, cx| {
            let Some(summary) = summary.await.log_err() else {
                thread
                    .update(cx, |thread, _cx| {
                        *thread.detailed_summary_tx.borrow_mut() =
                            DetailedSummaryState::NotGenerated;
                    })
                    .ok()?;
                return None;
            };

            thread
                .update(cx, |thread, _cx| {
                    *thread.detailed_summary_tx.borrow_mut() = DetailedSummaryState::Generated {
                        text: summary.into(),
                        message_id: last_message_id,
                    };
                })
                .ok()?;

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

    pub fn feedback(&self) -> Option<ThreadFeedback> {
        self.feedback
    }

    pub fn message_feedback(&self, message_id: AgentThreadMessageId) -> Option<ThreadFeedback> {
        self.message_feedback.get(&message_id).copied()
    }

    pub fn report_message_feedback(
        &mut self,
        message_id: AgentThreadMessageId,
        feedback: ThreadFeedback,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        todo!()
        // if self.message_feedback.get(&message_id) == Some(&feedback) {
        //     return Task::ready(Ok(()));
        // }

        // let final_project_snapshot = Self::project_snapshot(self.project.clone(), cx);
        // let serialized_thread = self.serialize(cx);
        // let thread_id = self.id().clone();
        // let client = self.project.read(cx).client();

        // let enabled_tool_names: Vec<String> = self
        //     .profile
        //     .enabled_tools(cx)
        //     .iter()
        //     .map(|tool| tool.name())
        //     .collect();

        // self.message_feedback.insert(message_id, feedback);

        // cx.notify();

        // let message_content = self
        //     .message(message_id)
        //     .map(|msg| msg.to_string())
        //     .unwrap_or_default();

        // cx.background_spawn(async move {
        //     let final_project_snapshot = final_project_snapshot.await;
        //     let serialized_thread = serialized_thread.await?;
        //     let thread_data =
        //         serde_json::to_value(serialized_thread).unwrap_or_else(|_| serde_json::Value::Null);

        //     let rating = match feedback {
        //         ThreadFeedback::Positive => "positive",
        //         ThreadFeedback::Negative => "negative",
        //     };
        //     telemetry::event!(
        //         "Assistant Thread Rated",
        //         rating,
        //         thread_id,
        //         enabled_tool_names,
        //         message_id = message_id,
        //         message_content,
        //         thread_data,
        //         final_project_snapshot
        //     );
        //     client.telemetry().flush_events().await;

        //     Ok(())
        // })
    }

    pub fn report_feedback(
        &mut self,
        feedback: ThreadFeedback,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        todo!()
        // let last_assistant_message_id = self
        //     .messages
        //     .iter()
        //     .rev()
        //     .find(|msg| msg.role == Role::Assistant)
        //     .map(|msg| msg.id);

        // if let Some(message_id) = last_assistant_message_id {
        //     self.report_message_feedback(message_id, feedback, cx)
        // } else {
        //     let final_project_snapshot = Self::project_snapshot(self.project.clone(), cx);
        //     let serialized_thread = self.serialize(cx);
        //     let thread_id = self.id().clone();
        //     let client = self.project.read(cx).client();
        //     self.feedback = Some(feedback);
        //     cx.notify();

        //     cx.background_spawn(async move {
        //         let final_project_snapshot = final_project_snapshot.await;
        //         let serialized_thread = serialized_thread.await?;
        //         let thread_data = serde_json::to_value(serialized_thread)
        //             .unwrap_or_else(|_| serde_json::Value::Null);

        //         let rating = match feedback {
        //             ThreadFeedback::Positive => "positive",
        //             ThreadFeedback::Negative => "negative",
        //         };
        //         telemetry::event!(
        //             "Assistant Thread Rated",
        //             rating,
        //             thread_id,
        //             thread_data,
        //             final_project_snapshot
        //         );
        //         client.telemetry().flush_events().await;

        //         Ok(())
        //     })
        // }
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
        todo!()
        // let mut markdown = Vec::new();

        // let summary = self.summary().or_default();
        // writeln!(markdown, "# {summary}\n")?;

        // for message in self.messages() {
        //     writeln!(
        //         markdown,
        //         "## {role}\n",
        //         role = match message.role {
        //             Role::User => "User",
        //             Role::Assistant => "Agent",
        //             Role::System => "System",
        //         }
        //     )?;

        //     if !message.loaded_context.text.is_empty() {
        //         writeln!(markdown, "{}", message.loaded_context.text)?;
        //     }

        //     if !message.loaded_context.images.is_empty() {
        //         writeln!(
        //             markdown,
        //             "\n{} images attached as context.\n",
        //             message.loaded_context.images.len()
        //         )?;
        //     }

        //     for segment in &message.segments {
        //         match segment {
        //             MessageSegment::Text(text) => writeln!(markdown, "{}\n", text)?,
        //             MessageSegment::Thinking { text, .. } => {
        //                 writeln!(markdown, "<think>\n{}\n</think>\n", text)?
        //             }
        //             MessageSegment::RedactedThinking(_) => {}
        //         }
        //     }

        //     for tool_use in self.tool_uses_for_message(message.id, cx) {
        //         writeln!(
        //             markdown,
        //             "**Use Tool: {} ({})**",
        //             tool_use.name, tool_use.id
        //         )?;
        //         writeln!(markdown, "```json")?;
        //         writeln!(
        //             markdown,
        //             "{}",
        //             serde_json::to_string_pretty(&tool_use.input)?
        //         )?;
        //         writeln!(markdown, "```")?;
        //     }

        //     for tool_result in self.tool_results_for_message(message.id) {
        //         write!(markdown, "\n**Tool Results: {}", tool_result.tool_use_id)?;
        //         if tool_result.is_error {
        //             write!(markdown, " (Error)")?;
        //         }

        //         writeln!(markdown, "**\n")?;
        //         match &tool_result.content {
        //             LanguageModelToolResultContent::Text(text) => {
        //                 writeln!(markdown, "{text}")?;
        //             }
        //             LanguageModelToolResultContent::Image(image) => {
        //                 writeln!(markdown, "![Image](data:base64,{})", image.source)?;
        //             }
        //         }

        //         if let Some(output) = tool_result.output.as_ref() {
        //             writeln!(
        //                 markdown,
        //                 "\n\nDebug Output:\n\n```json\n{}\n```\n",
        //                 serde_json::to_string_pretty(output)?
        //             )?;
        //         }
        //     }
        // }

        // Ok(String::from_utf8_lossy(&markdown).to_string())
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
        todo!()
        // if !cx.has_flag::<feature_flags::ThreadAutoCaptureFeatureFlag>() {
        //     return;
        // }

        // let now = Instant::now();
        // if let Some(last) = self.last_auto_capture_at {
        //     if now.duration_since(last).as_secs() < 10 {
        //         return;
        //     }
        // }

        // self.last_auto_capture_at = Some(now);

        // let thread_id = self.id().clone();
        // let github_login = self
        //     .project
        //     .read(cx)
        //     .user_store()
        //     .read(cx)
        //     .current_user()
        //     .map(|user| user.github_login.clone());
        // let client = self.project.read(cx).client();
        // let serialize_task = self.serialize(cx);

        // cx.background_executor()
        //     .spawn(async move {
        //         if let Ok(serialized_thread) = serialize_task.await {
        //             if let Ok(thread_data) = serde_json::to_value(serialized_thread) {
        //                 telemetry::event!(
        //                     "Agent Thread Auto-Captured",
        //                     thread_id = thread_id.to_string(),
        //                     thread_data = thread_data,
        //                     auto_capture_reason = "tracked_user",
        //                     github_login = github_login
        //                 );

        //                 client.telemetry().flush_events().await;
        //             }
        //         }
        //     })
        //     .detach();
    }

    pub fn cumulative_token_usage(&self) -> TokenUsage {
        self.cumulative_token_usage
    }

    pub fn token_usage_up_to_message(&self, message_id: AgentThreadMessageId) -> TotalTokenUsage {
        todo!()
        // let Some(model) = self.configured_model.as_ref() else {
        //     return TotalTokenUsage::default();
        // };

        // let max = model.model.max_token_count();

        // let index = self
        //     .messages
        //     .iter()
        //     .position(|msg| msg.id == message_id)
        //     .unwrap_or(0);

        // if index == 0 {
        //     return TotalTokenUsage { total: 0, max };
        // }

        // let token_usage = &self
        //     .request_token_usage
        //     .get(index - 1)
        //     .cloned()
        //     .unwrap_or_default();

        // TotalTokenUsage {
        //     total: token_usage.total_tokens(),
        //     max,
        // }
    }

    pub fn total_token_usage(&self) -> Option<TotalTokenUsage> {
        todo!()
        // let model = self.configured_model.as_ref()?;

        // let max = model.model.max_token_count();

        // if let Some(exceeded_error) = &self.exceeded_window_error {
        //     if model.model.id() == exceeded_error.model_id {
        //         return Some(TotalTokenUsage {
        //             total: exceeded_error.token_count,
        //             max,
        //         });
        //     }
        // }

        // let total = self
        //     .token_usage_at_last_message()
        //     .unwrap_or_default()
        //     .total_tokens();

        // Some(TotalTokenUsage { total, max })
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
        self.project.update(cx, |project, cx| {
            project.user_store().update(cx, |user_store, cx| {
                user_store.update_model_request_usage(
                    ModelRequestUsage(RequestUsage {
                        amount: amount as i32,
                        limit,
                    }),
                    cx,
                )
            })
        });
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
}

#[derive(Debug, Clone)]
pub enum ThreadEvent {
    ShowError(ThreadError),
    StreamedCompletion,
    ReceivedTextChunk,
    NewRequest,
    StreamedAssistantText(AgentThreadMessageId, String),
    StreamedAssistantThinking(AgentThreadMessageId, String),
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
    MessageAdded(AgentThreadMessageId),
    MessageEdited(AgentThreadMessageId),
    MessageDeleted(AgentThreadMessageId),
    SummaryGenerated,
    SummaryChanged,
    CheckpointChanged,
    ToolConfirmationNeeded,
    ToolUseLimitReached,
    CancelEditing,
    CompletionCanceled,
    ProfileChanged,
    RetriesFailed {
        message: SharedString,
    },
}

impl EventEmitter<ThreadEvent> for Thread {}

struct PendingCompletion {
    id: usize,
    queue_state: QueueState,
    _task: Task<()>,
}

/// Resolves tool name conflicts by ensuring all tool names are unique.
///
/// When multiple tools have the same name, this function applies the following rules:
/// 1. Native tools always keep their original name
/// 2. Context server tools get prefixed with their server ID and an underscore
/// 3. All tool names are truncated to MAX_TOOL_NAME_LENGTH (64 characters)
/// 4. If conflicts still exist after prefixing, the conflicting tools are filtered out
///
/// Note: This function assumes that built-in tools occur before MCP tools in the tools list.
fn resolve_tool_name_conflicts(tools: &[Arc<dyn Tool>]) -> Vec<(String, Arc<dyn Tool>)> {
    fn resolve_tool_name(tool: &Arc<dyn Tool>) -> String {
        let mut tool_name = tool.name();
        tool_name.truncate(MAX_TOOL_NAME_LENGTH);
        tool_name
    }

    const MAX_TOOL_NAME_LENGTH: usize = 64;

    let mut duplicated_tool_names = HashSet::default();
    let mut seen_tool_names = HashSet::default();
    for tool in tools {
        let tool_name = resolve_tool_name(tool);
        if seen_tool_names.contains(&tool_name) {
            debug_assert!(
                tool.source() != assistant_tool::ToolSource::Native,
                "There are two built-in tools with the same name: {}",
                tool_name
            );
            duplicated_tool_names.insert(tool_name);
        } else {
            seen_tool_names.insert(tool_name);
        }
    }

    if duplicated_tool_names.is_empty() {
        return tools
            .into_iter()
            .map(|tool| (resolve_tool_name(tool), tool.clone()))
            .collect();
    }

    tools
        .into_iter()
        .filter_map(|tool| {
            let mut tool_name = resolve_tool_name(tool);
            if !duplicated_tool_names.contains(&tool_name) {
                return Some((tool_name, tool.clone()));
            }
            match tool.source() {
                assistant_tool::ToolSource::Native => {
                    // Built-in tools always keep their original name
                    Some((tool_name, tool.clone()))
                }
                assistant_tool::ToolSource::ContextServer { id } => {
                    // Context server tools are prefixed with the context server ID, and truncated if necessary
                    tool_name.insert(0, '_');
                    if tool_name.len() + id.len() > MAX_TOOL_NAME_LENGTH {
                        let len = MAX_TOOL_NAME_LENGTH - tool_name.len();
                        let mut id = id.to_string();
                        id.truncate(len);
                        tool_name.insert_str(0, &id);
                    } else {
                        tool_name.insert_str(0, &id);
                    }

                    tool_name.truncate(MAX_TOOL_NAME_LENGTH);

                    if seen_tool_names.contains(&tool_name) {
                        log::error!("Cannot resolve tool name conflict for tool {}", tool.name());
                        None
                    } else {
                        Some((tool_name, tool.clone()))
                    }
                }
            }
        })
        .collect()
}
