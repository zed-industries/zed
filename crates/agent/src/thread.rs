use crate::{
    AgentThread, AgentThreadUserMessageChunk, MessageId, ThreadId,
    agent_profile::AgentProfile,
    context::{AgentContextHandle, LoadedContext},
    thread_store::{SharedProjectContext, ThreadStore},
};
use agent_settings::{AgentProfileId, AgentSettings, CompletionMode};
use anyhow::Result;
use assistant_tool::{ActionLog, AnyToolCard, Tool};
use chrono::{DateTime, Utc};
use client::{ModelRequestUsage, RequestUsage};
use collections::{HashMap, HashSet};
use futures::{FutureExt, channel::oneshot, future::Shared};
use git::repository::DiffType;
use gpui::{
    App, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task, WeakEntity,
    Window,
};
use language_model::{
    ConfiguredModel, LanguageModelId, LanguageModelToolUseId, Role, StopReason, TokenUsage,
};
use markdown::Markdown;
use postage::stream::Stream as _;
use project::{
    Project,
    git_store::{GitStore, GitStoreCheckpoint, RepositoryState},
};
use proto::Plan;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{ops::Range, sync::Arc, time::Instant};
use thiserror::Error;
use util::ResultExt as _;
use zed_llm_client::UsageLimit;

/// Stored information that can be used to resurrect a context crease when creating an editor for a past message.
#[derive(Clone, Debug)]
pub struct MessageCrease {
    pub range: Range<usize>,
    pub icon_path: SharedString,
    pub label: SharedString,
    /// None for a deserialized message, Some otherwise.
    pub context: Option<AgentContextHandle>,
}

pub enum MessageToolCall {
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
    pub id: MessageId,
    pub role: Role,
    pub segments: Vec<MessageSegment>,
    pub loaded_context: LoadedContext,
    pub creases: Vec<MessageCrease>,
    pub is_hidden: bool, // todo!("do we need this?")
    pub ui_only: bool,   // todo!("do we need this?")
}

pub enum Message {
    User {
        text: String,
        creases: Vec<MessageCrease>,
    },
    Assistant {
        segments: Vec<MessageSegment>,
    },
}

pub enum MessageSegment {
    Text(Entity<Markdown>),
    Thinking(Entity<Markdown>),
    ToolCall(MessageToolCall),
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
    agent_thread: Arc<dyn AgentThread>,
    title: ThreadTitle,
    pending_send: Option<Task<Result<()>>>,
    pending_summary: Task<Option<()>>,
    detailed_summary_task: Task<Option<()>>,
    detailed_summary_tx: postage::watch::Sender<DetailedSummaryState>,
    detailed_summary_rx: postage::watch::Receiver<DetailedSummaryState>,
    completion_mode: agent_settings::CompletionMode,
    messages: Vec<Message>,
    checkpoints_by_message: HashMap<MessageId, ThreadCheckpoint>,
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
    message_feedback: HashMap<MessageId, ThreadFeedback>,
    last_auto_capture_at: Option<Instant>,
    last_received_chunk_at: Option<Instant>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ThreadTitle {
    Pending,
    Generating,
    Ready(SharedString),
    Error,
}

impl ThreadTitle {
    pub const DEFAULT: SharedString = SharedString::new_static("New Thread");

    pub fn or_default(&self) -> SharedString {
        self.unwrap_or(Self::DEFAULT)
    }

    pub fn unwrap_or(&self, message: impl Into<SharedString>) -> SharedString {
        self.ready().unwrap_or_else(|| message.into())
    }

    pub fn ready(&self) -> Option<SharedString> {
        match self {
            ThreadTitle::Ready(summary) => Some(summary.clone()),
            ThreadTitle::Pending | ThreadTitle::Generating | ThreadTitle::Error => None,
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
            title: ThreadTitle::Pending,
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

    pub fn id(&self) -> ThreadId {
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

    pub fn project_context(&self) -> SharedProjectContext {
        todo!()
        // self.project_context.clone()
    }

    pub fn title(&self) -> &ThreadTitle {
        &self.title
    }

    pub fn set_title(&mut self, new_title: impl Into<SharedString>, cx: &mut Context<Self>) {
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

    pub fn regenerate_summary(&self, cx: &mut Context<Self>) {
        todo!()
        // self.summarize(cx);
    }

    pub fn completion_mode(&self) -> CompletionMode {
        self.completion_mode
    }

    pub fn set_completion_mode(&mut self, mode: CompletionMode) {
        self.completion_mode = mode;
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
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

    pub fn set_model(&mut self, model: Option<ConfiguredModel>, cx: &mut Context<Self>) {
        todo!()
    }

    pub fn model(&self) -> Option<ConfiguredModel> {
        todo!()
    }

    pub fn send(
        &mut self,
        message: Vec<AgentThreadUserMessageChunk>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        todo!()
    }

    pub fn resume(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        todo!()
    }

    pub fn edit(
        &mut self,
        message_id: MessageId,
        message: Vec<AgentThreadUserMessageChunk>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        todo!()
    }

    pub fn cancel(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
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
    pub fn text(&self, cx: &App) -> String {
        let mut text = String::new();

        for message in &self.messages {
            text.push_str(match message.role {
                language_model::Role::User => "User:",
                language_model::Role::Assistant => "Agent:",
                language_model::Role::System => "System:",
            });
            text.push('\n');

            text.push_str("<think>");
            text.push_str(message.thinking.read(cx).source());
            text.push_str("</think>");
            text.push_str(message.text.read(cx).source());

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
                    return this.read_with(cx, |this, cx| this.text(cx).into()).ok();
                }
                DetailedSummaryState::Generated { text, .. } => return Some(text),
            }
        }
    }

    pub fn latest_detailed_summary_or_text(&self, cx: &App) -> SharedString {
        self.detailed_summary_rx
            .borrow()
            .text()
            .unwrap_or_else(|| self.text(cx).into())
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

    pub fn message_feedback(&self, message_id: MessageId) -> Option<ThreadFeedback> {
        self.message_feedback.get(&message_id).copied()
    }

    pub fn report_message_feedback(
        &mut self,
        message_id: MessageId,
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

    pub fn token_usage_up_to_message(&self, message_id: MessageId) -> TotalTokenUsage {
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
    NewRequest,
    Stopped(Result<StopReason, Arc<anyhow::Error>>),
    MessagesUpdated {
        old_range: Range<usize>,
        new_length: usize,
    },
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{
//         context::load_context, context_store::ContextStore, thread_store, thread_store::ThreadStore,
//     };

//     // Test-specific constants
//     const TEST_RATE_LIMIT_RETRY_SECS: u64 = 30;
//     use agent_settings::{AgentProfileId, AgentSettings, LanguageModelParameters};
//     use assistant_tool::ToolRegistry;
//     use futures::StreamExt;
//     use futures::future::BoxFuture;
//     use futures::stream::BoxStream;
//     use gpui::TestAppContext;
//     use icons::IconName;
//     use language_model::fake_provider::{FakeLanguageModel, FakeLanguageModelProvider};
//     use language_model::{
//         LanguageModelCompletionError, LanguageModelName, LanguageModelProviderId,
//         LanguageModelProviderName, LanguageModelToolChoice,
//     };
//     use parking_lot::Mutex;
//     use project::{FakeFs, Project};
//     use prompt_store::PromptBuilder;
//     use serde_json::json;
//     use settings::{Settings, SettingsStore};
//     use std::sync::Arc;
//     use std::time::Duration;
//     use theme::ThemeSettings;
//     use util::path;
//     use workspace::Workspace;

//     #[gpui::test]
//     async fn test_message_with_context(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(
//             cx,
//             json!({"code.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"}),
//         )
//         .await;

//         let (_workspace, _thread_store, thread, context_store, model) =
//             setup_test_environment(cx, project.clone()).await;

//         add_file_to_context(&project, &context_store, "test/code.rs", cx)
//             .await
//             .unwrap();

//         let context =
//             context_store.read_with(cx, |store, _| store.context().next().cloned().unwrap());
//         let loaded_context = cx
//             .update(|cx| load_context(vec![context], &project, &None, cx))
//             .await;

//         // Insert user message with context
//         let message_id = thread.update(cx, |thread, cx| {
//             thread.insert_user_message(
//                 "Please explain this code",
//                 loaded_context,
//                 None,
//                 Vec::new(),
//                 cx,
//             )
//         });

//         // Check content and context in message object
//         let message = thread.read_with(cx, |thread, _| thread.message(message_id).unwrap().clone());

//         // Use different path format strings based on platform for the test
//         #[cfg(windows)]
//         let path_part = r"test\code.rs";
//         #[cfg(not(windows))]
//         let path_part = "test/code.rs";

//         let expected_context = format!(
//             r#"
// <context>
// The following items were attached by the user. They are up-to-date and don't need to be re-read.

// <files>
// ```rs {path_part}
// fn main() {{
//     println!("Hello, world!");
// }}
// ```
// </files>
// </context>
// "#
//         );

//         assert_eq!(message.role, Role::User);
//         assert_eq!(message.segments.len(), 1);
//         assert_eq!(
//             message.segments[0],
//             MessageSegment::Text("Please explain this code".to_string())
//         );
//         assert_eq!(message.loaded_context.text, expected_context);

//         // Check message in request
//         let request = thread.update(cx, |thread, cx| {
//             thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
//         });

//         assert_eq!(request.messages.len(), 2);
//         let expected_full_message = format!("{}Please explain this code", expected_context);
//         assert_eq!(request.messages[1].string_contents(), expected_full_message);
//     }

//     #[gpui::test]
//     async fn test_only_include_new_contexts(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(
//             cx,
//             json!({
//                 "file1.rs": "fn function1() {}\n",
//                 "file2.rs": "fn function2() {}\n",
//                 "file3.rs": "fn function3() {}\n",
//                 "file4.rs": "fn function4() {}\n",
//             }),
//         )
//         .await;

//         let (_, _thread_store, thread, context_store, model) =
//             setup_test_environment(cx, project.clone()).await;

//         // First message with context 1
//         add_file_to_context(&project, &context_store, "test/file1.rs", cx)
//             .await
//             .unwrap();
//         let new_contexts = context_store.update(cx, |store, cx| {
//             store.new_context_for_thread(thread.read(cx), None)
//         });
//         assert_eq!(new_contexts.len(), 1);
//         let loaded_context = cx
//             .update(|cx| load_context(new_contexts, &project, &None, cx))
//             .await;
//         let message1_id = thread.update(cx, |thread, cx| {
//             thread.insert_user_message("Message 1", loaded_context, None, Vec::new(), cx)
//         });

//         // Second message with contexts 1 and 2 (context 1 should be skipped as it's already included)
//         add_file_to_context(&project, &context_store, "test/file2.rs", cx)
//             .await
//             .unwrap();
//         let new_contexts = context_store.update(cx, |store, cx| {
//             store.new_context_for_thread(thread.read(cx), None)
//         });
//         assert_eq!(new_contexts.len(), 1);
//         let loaded_context = cx
//             .update(|cx| load_context(new_contexts, &project, &None, cx))
//             .await;
//         let message2_id = thread.update(cx, |thread, cx| {
//             thread.insert_user_message("Message 2", loaded_context, None, Vec::new(), cx)
//         });

//         // Third message with all three contexts (contexts 1 and 2 should be skipped)
//         //
//         add_file_to_context(&project, &context_store, "test/file3.rs", cx)
//             .await
//             .unwrap();
//         let new_contexts = context_store.update(cx, |store, cx| {
//             store.new_context_for_thread(thread.read(cx), None)
//         });
//         assert_eq!(new_contexts.len(), 1);
//         let loaded_context = cx
//             .update(|cx| load_context(new_contexts, &project, &None, cx))
//             .await;
//         let message3_id = thread.update(cx, |thread, cx| {
//             thread.insert_user_message("Message 3", loaded_context, None, Vec::new(), cx)
//         });

//         // Check what contexts are included in each message
//         let (message1, message2, message3) = thread.read_with(cx, |thread, _| {
//             (
//                 thread.message(message1_id).unwrap().clone(),
//                 thread.message(message2_id).unwrap().clone(),
//                 thread.message(message3_id).unwrap().clone(),
//             )
//         });

//         // First message should include context 1
//         assert!(message1.loaded_context.text.contains("file1.rs"));

//         // Second message should include only context 2 (not 1)
//         assert!(!message2.loaded_context.text.contains("file1.rs"));
//         assert!(message2.loaded_context.text.contains("file2.rs"));

//         // Third message should include only context 3 (not 1 or 2)
//         assert!(!message3.loaded_context.text.contains("file1.rs"));
//         assert!(!message3.loaded_context.text.contains("file2.rs"));
//         assert!(message3.loaded_context.text.contains("file3.rs"));

//         // Check entire request to make sure all contexts are properly included
//         let request = thread.update(cx, |thread, cx| {
//             thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
//         });

//         // The request should contain all 3 messages
//         assert_eq!(request.messages.len(), 4);

//         // Check that the contexts are properly formatted in each message
//         assert!(request.messages[1].string_contents().contains("file1.rs"));
//         assert!(!request.messages[1].string_contents().contains("file2.rs"));
//         assert!(!request.messages[1].string_contents().contains("file3.rs"));

//         assert!(!request.messages[2].string_contents().contains("file1.rs"));
//         assert!(request.messages[2].string_contents().contains("file2.rs"));
//         assert!(!request.messages[2].string_contents().contains("file3.rs"));

//         assert!(!request.messages[3].string_contents().contains("file1.rs"));
//         assert!(!request.messages[3].string_contents().contains("file2.rs"));
//         assert!(request.messages[3].string_contents().contains("file3.rs"));

//         add_file_to_context(&project, &context_store, "test/file4.rs", cx)
//             .await
//             .unwrap();
//         let new_contexts = context_store.update(cx, |store, cx| {
//             store.new_context_for_thread(thread.read(cx), Some(message2_id))
//         });
//         assert_eq!(new_contexts.len(), 3);
//         let loaded_context = cx
//             .update(|cx| load_context(new_contexts, &project, &None, cx))
//             .await
//             .loaded_context;

//         assert!(!loaded_context.text.contains("file1.rs"));
//         assert!(loaded_context.text.contains("file2.rs"));
//         assert!(loaded_context.text.contains("file3.rs"));
//         assert!(loaded_context.text.contains("file4.rs"));

//         let new_contexts = context_store.update(cx, |store, cx| {
//             // Remove file4.rs
//             store.remove_context(&loaded_context.contexts[2].handle(), cx);
//             store.new_context_for_thread(thread.read(cx), Some(message2_id))
//         });
//         assert_eq!(new_contexts.len(), 2);
//         let loaded_context = cx
//             .update(|cx| load_context(new_contexts, &project, &None, cx))
//             .await
//             .loaded_context;

//         assert!(!loaded_context.text.contains("file1.rs"));
//         assert!(loaded_context.text.contains("file2.rs"));
//         assert!(loaded_context.text.contains("file3.rs"));
//         assert!(!loaded_context.text.contains("file4.rs"));

//         let new_contexts = context_store.update(cx, |store, cx| {
//             // Remove file3.rs
//             store.remove_context(&loaded_context.contexts[1].handle(), cx);
//             store.new_context_for_thread(thread.read(cx), Some(message2_id))
//         });
//         assert_eq!(new_contexts.len(), 1);
//         let loaded_context = cx
//             .update(|cx| load_context(new_contexts, &project, &None, cx))
//             .await
//             .loaded_context;

//         assert!(!loaded_context.text.contains("file1.rs"));
//         assert!(loaded_context.text.contains("file2.rs"));
//         assert!(!loaded_context.text.contains("file3.rs"));
//         assert!(!loaded_context.text.contains("file4.rs"));
//     }

//     #[gpui::test]
//     async fn test_message_without_files(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(
//             cx,
//             json!({"code.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"}),
//         )
//         .await;

//         let (_, _thread_store, thread, _context_store, model) =
//             setup_test_environment(cx, project.clone()).await;

//         // Insert user message without any context (empty context vector)
//         let message_id = thread.update(cx, |thread, cx| {
//             thread.insert_user_message(
//                 "What is the best way to learn Rust?",
//                 ContextLoadResult::default(),
//                 None,
//                 Vec::new(),
//                 cx,
//             )
//         });

//         // Check content and context in message object
//         let message = thread.read_with(cx, |thread, _| thread.message(message_id).unwrap().clone());

//         // Context should be empty when no files are included
//         assert_eq!(message.role, Role::User);
//         assert_eq!(message.segments.len(), 1);
//         assert_eq!(
//             message.segments[0],
//             MessageSegment::Text("What is the best way to learn Rust?".to_string())
//         );
//         assert_eq!(message.loaded_context.text, "");

//         // Check message in request
//         let request = thread.update(cx, |thread, cx| {
//             thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
//         });

//         assert_eq!(request.messages.len(), 2);
//         assert_eq!(
//             request.messages[1].string_contents(),
//             "What is the best way to learn Rust?"
//         );

//         // Add second message, also without context
//         let message2_id = thread.update(cx, |thread, cx| {
//             thread.insert_user_message(
//                 "Are there any good books?",
//                 ContextLoadResult::default(),
//                 None,
//                 Vec::new(),
//                 cx,
//             )
//         });

//         let message2 =
//             thread.read_with(cx, |thread, _| thread.message(message2_id).unwrap().clone());
//         assert_eq!(message2.loaded_context.text, "");

//         // Check that both messages appear in the request
//         let request = thread.update(cx, |thread, cx| {
//             thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
//         });

//         assert_eq!(request.messages.len(), 3);
//         assert_eq!(
//             request.messages[1].string_contents(),
//             "What is the best way to learn Rust?"
//         );
//         assert_eq!(
//             request.messages[2].string_contents(),
//             "Are there any good books?"
//         );
//     }

//     #[gpui::test]
//     async fn test_storing_profile_setting_per_thread(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(
//             cx,
//             json!({"code.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"}),
//         )
//         .await;

//         let (_workspace, thread_store, thread, _context_store, _model) =
//             setup_test_environment(cx, project.clone()).await;

//         // Check that we are starting with the default profile
//         let profile = cx.read(|cx| thread.read(cx).profile.clone());
//         let tool_set = cx.read(|cx| thread_store.read(cx).tools());
//         assert_eq!(
//             profile,
//             AgentProfile::new(AgentProfileId::default(), tool_set)
//         );
//     }

//     #[gpui::test]
//     async fn test_serializing_thread_profile(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(
//             cx,
//             json!({"code.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"}),
//         )
//         .await;

//         let (_workspace, thread_store, thread, _context_store, _model) =
//             setup_test_environment(cx, project.clone()).await;

//         // Profile gets serialized with default values
//         let serialized = thread
//             .update(cx, |thread, cx| thread.serialize(cx))
//             .await
//             .unwrap();

//         assert_eq!(serialized.profile, Some(AgentProfileId::default()));

//         let deserialized = cx.update(|cx| {
//             thread.update(cx, |thread, cx| {
//                 Thread::deserialize(
//                     thread.id.clone(),
//                     serialized,
//                     thread.project.clone(),
//                     thread.tools.clone(),
//                     thread.prompt_builder.clone(),
//                     thread.project_context.clone(),
//                     None,
//                     cx,
//                 )
//             })
//         });
//         let tool_set = cx.read(|cx| thread_store.read(cx).tools());

//         assert_eq!(
//             deserialized.profile,
//             AgentProfile::new(AgentProfileId::default(), tool_set)
//         );
//     }

//     #[gpui::test]
//     async fn test_temperature_setting(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(
//             cx,
//             json!({"code.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"}),
//         )
//         .await;

//         let (_workspace, _thread_store, thread, _context_store, model) =
//             setup_test_environment(cx, project.clone()).await;

//         // Both model and provider
//         cx.update(|cx| {
//             AgentSettings::override_global(
//                 AgentSettings {
//                     model_parameters: vec![LanguageModelParameters {
//                         provider: Some(model.provider_id().0.to_string().into()),
//                         model: Some(model.id().0.clone()),
//                         temperature: Some(0.66),
//                     }],
//                     ..AgentSettings::get_global(cx).clone()
//                 },
//                 cx,
//             );
//         });

//         let request = thread.update(cx, |thread, cx| {
//             thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
//         });
//         assert_eq!(request.temperature, Some(0.66));

//         // Only model
//         cx.update(|cx| {
//             AgentSettings::override_global(
//                 AgentSettings {
//                     model_parameters: vec![LanguageModelParameters {
//                         provider: None,
//                         model: Some(model.id().0.clone()),
//                         temperature: Some(0.66),
//                     }],
//                     ..AgentSettings::get_global(cx).clone()
//                 },
//                 cx,
//             );
//         });

//         let request = thread.update(cx, |thread, cx| {
//             thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
//         });
//         assert_eq!(request.temperature, Some(0.66));

//         // Only provider
//         cx.update(|cx| {
//             AgentSettings::override_global(
//                 AgentSettings {
//                     model_parameters: vec![LanguageModelParameters {
//                         provider: Some(model.provider_id().0.to_string().into()),
//                         model: None,
//                         temperature: Some(0.66),
//                     }],
//                     ..AgentSettings::get_global(cx).clone()
//                 },
//                 cx,
//             );
//         });

//         let request = thread.update(cx, |thread, cx| {
//             thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
//         });
//         assert_eq!(request.temperature, Some(0.66));

//         // Same model name, different provider
//         cx.update(|cx| {
//             AgentSettings::override_global(
//                 AgentSettings {
//                     model_parameters: vec![LanguageModelParameters {
//                         provider: Some("anthropic".into()),
//                         model: Some(model.id().0.clone()),
//                         temperature: Some(0.66),
//                     }],
//                     ..AgentSettings::get_global(cx).clone()
//                 },
//                 cx,
//             );
//         });

//         let request = thread.update(cx, |thread, cx| {
//             thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
//         });
//         assert_eq!(request.temperature, None);
//     }

//     #[gpui::test]
//     async fn test_thread_summary(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(cx, json!({})).await;

//         let (_, _thread_store, thread, _context_store, model) =
//             setup_test_environment(cx, project.clone()).await;

//         // Initial state should be pending
//         thread.read_with(cx, |thread, _| {
//             assert!(matches!(thread.summary(), ThreadSummary::Pending));
//             assert_eq!(thread.summary().or_default(), ThreadSummary::DEFAULT);
//         });

//         // Manually setting the summary should not be allowed in this state
//         thread.update(cx, |thread, cx| {
//             thread.set_summary("This should not work", cx);
//         });

//         thread.read_with(cx, |thread, _| {
//             assert!(matches!(thread.summary(), ThreadSummary::Pending));
//         });

//         // Send a message
//         thread.update(cx, |thread, cx| {
//             thread.insert_user_message("Hi!", ContextLoadResult::default(), None, vec![], cx);
//             thread.send_to_model(
//                 model.clone(),
//                 CompletionIntent::ThreadSummarization,
//                 None,
//                 cx,
//             );
//         });

//         let fake_model = model.as_fake();
//         simulate_successful_response(&fake_model, cx);

//         // Should start generating summary when there are >= 2 messages
//         thread.read_with(cx, |thread, _| {
//             assert_eq!(*thread.summary(), ThreadSummary::Generating);
//         });

//         // Should not be able to set the summary while generating
//         thread.update(cx, |thread, cx| {
//             thread.set_summary("This should not work either", cx);
//         });

//         thread.read_with(cx, |thread, _| {
//             assert!(matches!(thread.summary(), ThreadSummary::Generating));
//             assert_eq!(thread.summary().or_default(), ThreadSummary::DEFAULT);
//         });

//         cx.run_until_parked();
//         fake_model.stream_last_completion_response("Brief");
//         fake_model.stream_last_completion_response(" Introduction");
//         fake_model.end_last_completion_stream();
//         cx.run_until_parked();

//         // Summary should be set
//         thread.read_with(cx, |thread, _| {
//             assert!(matches!(thread.summary(), ThreadSummary::Ready(_)));
//             assert_eq!(thread.summary().or_default(), "Brief Introduction");
//         });

//         // Now we should be able to set a summary
//         thread.update(cx, |thread, cx| {
//             thread.set_summary("Brief Intro", cx);
//         });

//         thread.read_with(cx, |thread, _| {
//             assert_eq!(thread.summary().or_default(), "Brief Intro");
//         });

//         // Test setting an empty summary (should default to DEFAULT)
//         thread.update(cx, |thread, cx| {
//             thread.set_summary("", cx);
//         });

//         thread.read_with(cx, |thread, _| {
//             assert!(matches!(thread.summary(), ThreadSummary::Ready(_)));
//             assert_eq!(thread.summary().or_default(), ThreadSummary::DEFAULT);
//         });
//     }

//     #[gpui::test]
//     async fn test_thread_summary_error_set_manually(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(cx, json!({})).await;

//         let (_, _thread_store, thread, _context_store, model) =
//             setup_test_environment(cx, project.clone()).await;

//         test_summarize_error(&model, &thread, cx);

//         // Now we should be able to set a summary
//         thread.update(cx, |thread, cx| {
//             thread.set_summary("Brief Intro", cx);
//         });

//         thread.read_with(cx, |thread, _| {
//             assert!(matches!(thread.summary(), ThreadSummary::Ready(_)));
//             assert_eq!(thread.summary().or_default(), "Brief Intro");
//         });
//     }

//     #[gpui::test]
//     async fn test_thread_summary_error_retry(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(cx, json!({})).await;

//         let (_, _thread_store, thread, _context_store, model) =
//             setup_test_environment(cx, project.clone()).await;

//         test_summarize_error(&model, &thread, cx);

//         // Sending another message should not trigger another summarize request
//         thread.update(cx, |thread, cx| {
//             thread.insert_user_message(
//                 "How are you?",
//                 ContextLoadResult::default(),
//                 None,
//                 vec![],
//                 cx,
//             );
//             thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
//         });

//         let fake_model = model.as_fake();
//         simulate_successful_response(&fake_model, cx);

//         thread.read_with(cx, |thread, _| {
//             // State is still Error, not Generating
//             assert!(matches!(thread.summary(), ThreadSummary::Error));
//         });

//         // But the summarize request can be invoked manually
//         thread.update(cx, |thread, cx| {
//             thread.summarize(cx);
//         });

//         thread.read_with(cx, |thread, _| {
//             assert!(matches!(thread.summary(), ThreadSummary::Generating));
//         });

//         cx.run_until_parked();
//         fake_model.stream_last_completion_response("A successful summary");
//         fake_model.end_last_completion_stream();
//         cx.run_until_parked();

//         thread.read_with(cx, |thread, _| {
//             assert!(matches!(thread.summary(), ThreadSummary::Ready(_)));
//             assert_eq!(thread.summary().or_default(), "A successful summary");
//         });
//     }

//     #[gpui::test]
//     fn test_resolve_tool_name_conflicts() {
//         use assistant_tool::{Tool, ToolSource};

//         assert_resolve_tool_name_conflicts(
//             vec![
//                 TestTool::new("tool1", ToolSource::Native),
//                 TestTool::new("tool2", ToolSource::Native),
//                 TestTool::new("tool3", ToolSource::ContextServer { id: "mcp-1".into() }),
//             ],
//             vec!["tool1", "tool2", "tool3"],
//         );

//         assert_resolve_tool_name_conflicts(
//             vec![
//                 TestTool::new("tool1", ToolSource::Native),
//                 TestTool::new("tool2", ToolSource::Native),
//                 TestTool::new("tool3", ToolSource::ContextServer { id: "mcp-1".into() }),
//                 TestTool::new("tool3", ToolSource::ContextServer { id: "mcp-2".into() }),
//             ],
//             vec!["tool1", "tool2", "mcp-1_tool3", "mcp-2_tool3"],
//         );

//         assert_resolve_tool_name_conflicts(
//             vec![
//                 TestTool::new("tool1", ToolSource::Native),
//                 TestTool::new("tool2", ToolSource::Native),
//                 TestTool::new("tool3", ToolSource::Native),
//                 TestTool::new("tool3", ToolSource::ContextServer { id: "mcp-1".into() }),
//                 TestTool::new("tool3", ToolSource::ContextServer { id: "mcp-2".into() }),
//             ],
//             vec!["tool1", "tool2", "tool3", "mcp-1_tool3", "mcp-2_tool3"],
//         );

//         // Test that tool with very long name is always truncated
//         assert_resolve_tool_name_conflicts(
//             vec![TestTool::new(
//                 "tool-with-more-then-64-characters-blah-blah-blah-blah-blah-blah-blah-blah",
//                 ToolSource::Native,
//             )],
//             vec!["tool-with-more-then-64-characters-blah-blah-blah-blah-blah-blah-"],
//         );

//         // Test deduplication of tools with very long names, in this case the mcp server name should be truncated
//         assert_resolve_tool_name_conflicts(
//             vec![
//                 TestTool::new("tool-with-very-very-very-long-name", ToolSource::Native),
//                 TestTool::new(
//                     "tool-with-very-very-very-long-name",
//                     ToolSource::ContextServer {
//                         id: "mcp-with-very-very-very-long-name".into(),
//                     },
//                 ),
//             ],
//             vec![
//                 "tool-with-very-very-very-long-name",
//                 "mcp-with-very-very-very-long-_tool-with-very-very-very-long-name",
//             ],
//         );

//         fn assert_resolve_tool_name_conflicts(
//             tools: Vec<TestTool>,
//             expected: Vec<impl Into<String>>,
//         ) {
//             let tools: Vec<Arc<dyn Tool>> = tools
//                 .into_iter()
//                 .map(|t| Arc::new(t) as Arc<dyn Tool>)
//                 .collect();
//             let tools = resolve_tool_name_conflicts(&tools);
//             assert_eq!(tools.len(), expected.len());
//             for (i, expected_name) in expected.into_iter().enumerate() {
//                 let expected_name = expected_name.into();
//                 let actual_name = &tools[i].0;
//                 assert_eq!(
//                     actual_name, &expected_name,
//                     "Expected '{}' got '{}' at index {}",
//                     expected_name, actual_name, i
//                 );
//             }
//         }

//         struct TestTool {
//             name: String,
//             source: ToolSource,
//         }

//         impl TestTool {
//             fn new(name: impl Into<String>, source: ToolSource) -> Self {
//                 Self {
//                     name: name.into(),
//                     source,
//                 }
//             }
//         }

//         impl Tool for TestTool {
//             fn name(&self) -> String {
//                 self.name.clone()
//             }

//             fn icon(&self) -> IconName {
//                 IconName::Ai
//             }

//             fn may_perform_edits(&self) -> bool {
//                 false
//             }

//             fn needs_confirmation(&self, _input: &serde_json::Value, _cx: &App) -> bool {
//                 true
//             }

//             fn source(&self) -> ToolSource {
//                 self.source.clone()
//             }

//             fn description(&self) -> String {
//                 "Test tool".to_string()
//             }

//             fn ui_text(&self, _input: &serde_json::Value) -> String {
//                 "Test tool".to_string()
//             }

//             fn run(
//                 self: Arc<Self>,
//                 _input: serde_json::Value,
//                 _request: Arc<LanguageModelRequest>,
//                 _project: Entity<Project>,
//                 _action_log: Entity<ActionLog>,
//                 _model: Arc<dyn LanguageModel>,
//                 _window: Option<AnyWindowHandle>,
//                 _cx: &mut App,
//             ) -> assistant_tool::ToolResult {
//                 assistant_tool::ToolResult {
//                     output: Task::ready(Err(anyhow::anyhow!("No content"))),
//                     card: None,
//                 }
//             }
//         }
//     }

//     // Helper to create a model that returns errors
//     enum TestError {
//         Overloaded,
//         InternalServerError,
//     }

//     struct ErrorInjector {
//         inner: Arc<FakeLanguageModel>,
//         error_type: TestError,
//     }

//     impl ErrorInjector {
//         fn new(error_type: TestError) -> Self {
//             Self {
//                 inner: Arc::new(FakeLanguageModel::default()),
//                 error_type,
//             }
//         }
//     }

//     impl LanguageModel for ErrorInjector {
//         fn id(&self) -> LanguageModelId {
//             self.inner.id()
//         }

//         fn name(&self) -> LanguageModelName {
//             self.inner.name()
//         }

//         fn provider_id(&self) -> LanguageModelProviderId {
//             self.inner.provider_id()
//         }

//         fn provider_name(&self) -> LanguageModelProviderName {
//             self.inner.provider_name()
//         }

//         fn supports_tools(&self) -> bool {
//             self.inner.supports_tools()
//         }

//         fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
//             self.inner.supports_tool_choice(choice)
//         }

//         fn supports_images(&self) -> bool {
//             self.inner.supports_images()
//         }

//         fn telemetry_id(&self) -> String {
//             self.inner.telemetry_id()
//         }

//         fn max_token_count(&self) -> u64 {
//             self.inner.max_token_count()
//         }

//         fn count_tokens(
//             &self,
//             request: LanguageModelRequest,
//             cx: &App,
//         ) -> BoxFuture<'static, Result<u64>> {
//             self.inner.count_tokens(request, cx)
//         }

//         fn stream_completion(
//             &self,
//             _request: LanguageModelRequest,
//             _cx: &AsyncApp,
//         ) -> BoxFuture<
//             'static,
//             Result<
//                 BoxStream<
//                     'static,
//                     Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
//                 >,
//                 LanguageModelCompletionError,
//             >,
//         > {
//             let error = match self.error_type {
//                 TestError::Overloaded => LanguageModelCompletionError::Overloaded,
//                 TestError::InternalServerError => {
//                     LanguageModelCompletionError::ApiInternalServerError
//                 }
//             };
//             async move {
//                 let stream = futures::stream::once(async move { Err(error) });
//                 Ok(stream.boxed())
//             }
//             .boxed()
//         }

//         fn as_fake(&self) -> &FakeLanguageModel {
//             &self.inner
//         }
//     }

//     #[gpui::test]
//     async fn test_retry_on_overloaded_error(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(cx, json!({})).await;
//         let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

//         // Create model that returns overloaded error
//         let model = Arc::new(ErrorInjector::new(TestError::Overloaded));

//         // Insert a user message
//         thread.update(cx, |thread, cx| {
//             thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
//         });

//         // Start completion
//         thread.update(cx, |thread, cx| {
//             thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
//         });

//         cx.run_until_parked();

//         thread.read_with(cx, |thread, _| {
//             assert!(thread.retry_state.is_some(), "Should have retry state");
//             let retry_state = thread.retry_state.as_ref().unwrap();
//             assert_eq!(retry_state.attempt, 1, "Should be first retry attempt");
//             assert_eq!(
//                 retry_state.max_attempts, MAX_RETRY_ATTEMPTS,
//                 "Should have default max attempts"
//             );
//         });

//         // Check that a retry message was added
//         thread.read_with(cx, |thread, _| {
//             let mut messages = thread.messages();
//             assert!(
//                 messages.any(|msg| {
//                     msg.role == Role::System
//                         && msg.ui_only
//                         && msg.segments.iter().any(|seg| {
//                             if let MessageSegment::Text(text) = seg {
//                                 text.contains("overloaded")
//                                     && text
//                                         .contains(&format!("attempt 1 of {}", MAX_RETRY_ATTEMPTS))
//                             } else {
//                                 false
//                             }
//                         })
//                 }),
//                 "Should have added a system retry message"
//             );
//         });

//         let retry_count = thread.update(cx, |thread, _| {
//             thread
//                 .messages
//                 .iter()
//                 .filter(|m| {
//                     m.ui_only
//                         && m.segments.iter().any(|s| {
//                             if let MessageSegment::Text(text) = s {
//                                 text.contains("Retrying") && text.contains("seconds")
//                             } else {
//                                 false
//                             }
//                         })
//                 })
//                 .count()
//         });

//         assert_eq!(retry_count, 1, "Should have one retry message");
//     }

//     #[gpui::test]
//     async fn test_retry_on_internal_server_error(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(cx, json!({})).await;
//         let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

//         // Create model that returns internal server error
//         let model = Arc::new(ErrorInjector::new(TestError::InternalServerError));

//         // Insert a user message
//         thread.update(cx, |thread, cx| {
//             thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
//         });

//         // Start completion
//         thread.update(cx, |thread, cx| {
//             thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
//         });

//         cx.run_until_parked();

//         // Check retry state on thread
//         thread.read_with(cx, |thread, _| {
//             assert!(thread.retry_state.is_some(), "Should have retry state");
//             let retry_state = thread.retry_state.as_ref().unwrap();
//             assert_eq!(retry_state.attempt, 1, "Should be first retry attempt");
//             assert_eq!(
//                 retry_state.max_attempts, MAX_RETRY_ATTEMPTS,
//                 "Should have correct max attempts"
//             );
//         });

//         // Check that a retry message was added with provider name
//         thread.read_with(cx, |thread, _| {
//             let mut messages = thread.messages();
//             assert!(
//                 messages.any(|msg| {
//                     msg.role == Role::System
//                         && msg.ui_only
//                         && msg.segments.iter().any(|seg| {
//                             if let MessageSegment::Text(text) = seg {
//                                 text.contains("internal")
//                                     && text.contains("Fake")
//                                     && text
//                                         .contains(&format!("attempt 1 of {}", MAX_RETRY_ATTEMPTS))
//                             } else {
//                                 false
//                             }
//                         })
//                 }),
//                 "Should have added a system retry message with provider name"
//             );
//         });

//         // Count retry messages
//         let retry_count = thread.update(cx, |thread, _| {
//             thread
//                 .messages
//                 .iter()
//                 .filter(|m| {
//                     m.ui_only
//                         && m.segments.iter().any(|s| {
//                             if let MessageSegment::Text(text) = s {
//                                 text.contains("Retrying") && text.contains("seconds")
//                             } else {
//                                 false
//                             }
//                         })
//                 })
//                 .count()
//         });

//         assert_eq!(retry_count, 1, "Should have one retry message");
//     }

//     #[gpui::test]
//     async fn test_exponential_backoff_on_retries(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(cx, json!({})).await;
//         let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

//         // Create model that returns overloaded error
//         let model = Arc::new(ErrorInjector::new(TestError::Overloaded));

//         // Insert a user message
//         thread.update(cx, |thread, cx| {
//             thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
//         });

//         // Track retry events and completion count
//         // Track completion events
//         let completion_count = Arc::new(Mutex::new(0));
//         let completion_count_clone = completion_count.clone();

//         let _subscription = thread.update(cx, |_, cx| {
//             cx.subscribe(&thread, move |_, _, event: &ThreadEvent, _| {
//                 if let ThreadEvent::NewRequest = event {
//                     *completion_count_clone.lock() += 1;
//                 }
//             })
//         });

//         // First attempt
//         thread.update(cx, |thread, cx| {
//             thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
//         });
//         cx.run_until_parked();

//         // Should have scheduled first retry - count retry messages
//         let retry_count = thread.update(cx, |thread, _| {
//             thread
//                 .messages
//                 .iter()
//                 .filter(|m| {
//                     m.ui_only
//                         && m.segments.iter().any(|s| {
//                             if let MessageSegment::Text(text) = s {
//                                 text.contains("Retrying") && text.contains("seconds")
//                             } else {
//                                 false
//                             }
//                         })
//                 })
//                 .count()
//         });
//         assert_eq!(retry_count, 1, "Should have scheduled first retry");

//         // Check retry state
//         thread.read_with(cx, |thread, _| {
//             assert!(thread.retry_state.is_some(), "Should have retry state");
//             let retry_state = thread.retry_state.as_ref().unwrap();
//             assert_eq!(retry_state.attempt, 1, "Should be first retry attempt");
//         });

//         // Advance clock for first retry
//         cx.executor()
//             .advance_clock(Duration::from_secs(BASE_RETRY_DELAY_SECS));
//         cx.run_until_parked();

//         // Should have scheduled second retry - count retry messages
//         let retry_count = thread.update(cx, |thread, _| {
//             thread
//                 .messages
//                 .iter()
//                 .filter(|m| {
//                     m.ui_only
//                         && m.segments.iter().any(|s| {
//                             if let MessageSegment::Text(text) = s {
//                                 text.contains("Retrying") && text.contains("seconds")
//                             } else {
//                                 false
//                             }
//                         })
//                 })
//                 .count()
//         });
//         assert_eq!(retry_count, 2, "Should have scheduled second retry");

//         // Check retry state updated
//         thread.read_with(cx, |thread, _| {
//             assert!(thread.retry_state.is_some(), "Should have retry state");
//             let retry_state = thread.retry_state.as_ref().unwrap();
//             assert_eq!(retry_state.attempt, 2, "Should be second retry attempt");
//             assert_eq!(
//                 retry_state.max_attempts, MAX_RETRY_ATTEMPTS,
//                 "Should have correct max attempts"
//             );
//         });

//         // Advance clock for second retry (exponential backoff)
//         cx.executor()
//             .advance_clock(Duration::from_secs(BASE_RETRY_DELAY_SECS * 2));
//         cx.run_until_parked();

//         // Should have scheduled third retry
//         // Count all retry messages now
//         let retry_count = thread.update(cx, |thread, _| {
//             thread
//                 .messages
//                 .iter()
//                 .filter(|m| {
//                     m.ui_only
//                         && m.segments.iter().any(|s| {
//                             if let MessageSegment::Text(text) = s {
//                                 text.contains("Retrying") && text.contains("seconds")
//                             } else {
//                                 false
//                             }
//                         })
//                 })
//                 .count()
//         });
//         assert_eq!(
//             retry_count, MAX_RETRY_ATTEMPTS as usize,
//             "Should have scheduled third retry"
//         );

//         // Check retry state updated
//         thread.read_with(cx, |thread, _| {
//             assert!(thread.retry_state.is_some(), "Should have retry state");
//             let retry_state = thread.retry_state.as_ref().unwrap();
//             assert_eq!(
//                 retry_state.attempt, MAX_RETRY_ATTEMPTS,
//                 "Should be at max retry attempt"
//             );
//             assert_eq!(
//                 retry_state.max_attempts, MAX_RETRY_ATTEMPTS,
//                 "Should have correct max attempts"
//             );
//         });

//         // Advance clock for third retry (exponential backoff)
//         cx.executor()
//             .advance_clock(Duration::from_secs(BASE_RETRY_DELAY_SECS * 4));
//         cx.run_until_parked();

//         // No more retries should be scheduled after clock was advanced.
//         let retry_count = thread.update(cx, |thread, _| {
//             thread
//                 .messages
//                 .iter()
//                 .filter(|m| {
//                     m.ui_only
//                         && m.segments.iter().any(|s| {
//                             if let MessageSegment::Text(text) = s {
//                                 text.contains("Retrying") && text.contains("seconds")
//                             } else {
//                                 false
//                             }
//                         })
//                 })
//                 .count()
//         });
//         assert_eq!(
//             retry_count, MAX_RETRY_ATTEMPTS as usize,
//             "Should not exceed max retries"
//         );

//         // Final completion count should be initial + max retries
//         assert_eq!(
//             *completion_count.lock(),
//             (MAX_RETRY_ATTEMPTS + 1) as usize,
//             "Should have made initial + max retry attempts"
//         );
//     }

//     #[gpui::test]
//     async fn test_max_retries_exceeded(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(cx, json!({})).await;
//         let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

//         // Create model that returns overloaded error
//         let model = Arc::new(ErrorInjector::new(TestError::Overloaded));

//         // Insert a user message
//         thread.update(cx, |thread, cx| {
//             thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
//         });

//         // Track events
//         let retries_failed = Arc::new(Mutex::new(false));
//         let retries_failed_clone = retries_failed.clone();

//         let _subscription = thread.update(cx, |_, cx| {
//             cx.subscribe(&thread, move |_, _, event: &ThreadEvent, _| {
//                 if let ThreadEvent::RetriesFailed { .. } = event {
//                     *retries_failed_clone.lock() = true;
//                 }
//             })
//         });

//         // Start initial completion
//         thread.update(cx, |thread, cx| {
//             thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
//         });
//         cx.run_until_parked();

//         // Advance through all retries
//         for i in 0..MAX_RETRY_ATTEMPTS {
//             let delay = if i == 0 {
//                 BASE_RETRY_DELAY_SECS
//             } else {
//                 BASE_RETRY_DELAY_SECS * 2u64.pow(i as u32 - 1)
//             };
//             cx.executor().advance_clock(Duration::from_secs(delay));
//             cx.run_until_parked();
//         }

//         // After the 3rd retry is scheduled, we need to wait for it to execute and fail
//         // The 3rd retry has a delay of BASE_RETRY_DELAY_SECS * 4 (20 seconds)
//         let final_delay = BASE_RETRY_DELAY_SECS * 2u64.pow((MAX_RETRY_ATTEMPTS - 1) as u32);
//         cx.executor()
//             .advance_clock(Duration::from_secs(final_delay));
//         cx.run_until_parked();

//         let retry_count = thread.update(cx, |thread, _| {
//             thread
//                 .messages
//                 .iter()
//                 .filter(|m| {
//                     m.ui_only
//                         && m.segments.iter().any(|s| {
//                             if let MessageSegment::Text(text) = s {
//                                 text.contains("Retrying") && text.contains("seconds")
//                             } else {
//                                 false
//                             }
//                         })
//                 })
//                 .count()
//         });

//         // After max retries, should emit RetriesFailed event
//         assert_eq!(
//             retry_count, MAX_RETRY_ATTEMPTS as usize,
//             "Should have attempted max retries"
//         );
//         assert!(
//             *retries_failed.lock(),
//             "Should emit RetriesFailed event after max retries exceeded"
//         );

//         // Retry state should be cleared
//         thread.read_with(cx, |thread, _| {
//             assert!(
//                 thread.retry_state.is_none(),
//                 "Retry state should be cleared after max retries"
//             );

//             // Verify we have the expected number of retry messages
//             let retry_messages = thread
//                 .messages
//                 .iter()
//                 .filter(|msg| msg.ui_only && msg.role == Role::System)
//                 .count();
//             assert_eq!(
//                 retry_messages, MAX_RETRY_ATTEMPTS as usize,
//                 "Should have one retry message per attempt"
//             );
//         });
//     }

//     #[gpui::test]
//     async fn test_retry_message_removed_on_retry(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(cx, json!({})).await;
//         let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

//         // We'll use a wrapper to switch behavior after first failure
//         struct RetryTestModel {
//             inner: Arc<FakeLanguageModel>,
//             failed_once: Arc<Mutex<bool>>,
//         }

//         impl LanguageModel for RetryTestModel {
//             fn id(&self) -> LanguageModelId {
//                 self.inner.id()
//             }

//             fn name(&self) -> LanguageModelName {
//                 self.inner.name()
//             }

//             fn provider_id(&self) -> LanguageModelProviderId {
//                 self.inner.provider_id()
//             }

//             fn provider_name(&self) -> LanguageModelProviderName {
//                 self.inner.provider_name()
//             }

//             fn supports_tools(&self) -> bool {
//                 self.inner.supports_tools()
//             }

//             fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
//                 self.inner.supports_tool_choice(choice)
//             }

//             fn supports_images(&self) -> bool {
//                 self.inner.supports_images()
//             }

//             fn telemetry_id(&self) -> String {
//                 self.inner.telemetry_id()
//             }

//             fn max_token_count(&self) -> u64 {
//                 self.inner.max_token_count()
//             }

//             fn count_tokens(
//                 &self,
//                 request: LanguageModelRequest,
//                 cx: &App,
//             ) -> BoxFuture<'static, Result<u64>> {
//                 self.inner.count_tokens(request, cx)
//             }

//             fn stream_completion(
//                 &self,
//                 request: LanguageModelRequest,
//                 cx: &AsyncApp,
//             ) -> BoxFuture<
//                 'static,
//                 Result<
//                     BoxStream<
//                         'static,
//                         Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
//                     >,
//                     LanguageModelCompletionError,
//                 >,
//             > {
//                 if !*self.failed_once.lock() {
//                     *self.failed_once.lock() = true;
//                     // Return error on first attempt
//                     let stream = futures::stream::once(async move {
//                         Err(LanguageModelCompletionError::Overloaded)
//                     });
//                     async move { Ok(stream.boxed()) }.boxed()
//                 } else {
//                     // Succeed on retry
//                     self.inner.stream_completion(request, cx)
//                 }
//             }

//             fn as_fake(&self) -> &FakeLanguageModel {
//                 &self.inner
//             }
//         }

//         let model = Arc::new(RetryTestModel {
//             inner: Arc::new(FakeLanguageModel::default()),
//             failed_once: Arc::new(Mutex::new(false)),
//         });

//         // Insert a user message
//         thread.update(cx, |thread, cx| {
//             thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
//         });

//         // Track message deletions
//         // Track when retry completes successfully
//         let retry_completed = Arc::new(Mutex::new(false));
//         let retry_completed_clone = retry_completed.clone();

//         let _subscription = thread.update(cx, |_, cx| {
//             cx.subscribe(&thread, move |_, _, event: &ThreadEvent, _| {
//                 if let ThreadEvent::StreamedCompletion = event {
//                     *retry_completed_clone.lock() = true;
//                 }
//             })
//         });

//         // Start completion
//         thread.update(cx, |thread, cx| {
//             thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
//         });
//         cx.run_until_parked();

//         // Get the retry message ID
//         let retry_message_id = thread.read_with(cx, |thread, _| {
//             thread
//                 .messages()
//                 .find(|msg| msg.role == Role::System && msg.ui_only)
//                 .map(|msg| msg.id)
//                 .expect("Should have a retry message")
//         });

//         // Wait for retry
//         cx.executor()
//             .advance_clock(Duration::from_secs(BASE_RETRY_DELAY_SECS));
//         cx.run_until_parked();

//         // Stream some successful content
//         let fake_model = model.as_fake();
//         // After the retry, there should be a new pending completion
//         let pending = fake_model.pending_completions();
//         assert!(
//             !pending.is_empty(),
//             "Should have a pending completion after retry"
//         );
//         fake_model.stream_completion_response(&pending[0], "Success!");
//         fake_model.end_completion_stream(&pending[0]);
//         cx.run_until_parked();

//         // Check that the retry completed successfully
//         assert!(
//             *retry_completed.lock(),
//             "Retry should have completed successfully"
//         );

//         // Retry message should still exist but be marked as ui_only
//         thread.read_with(cx, |thread, _| {
//             let retry_msg = thread
//                 .message(retry_message_id)
//                 .expect("Retry message should still exist");
//             assert!(retry_msg.ui_only, "Retry message should be ui_only");
//             assert_eq!(
//                 retry_msg.role,
//                 Role::System,
//                 "Retry message should have System role"
//             );
//         });
//     }

//     #[gpui::test]
//     async fn test_successful_completion_clears_retry_state(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(cx, json!({})).await;
//         let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

//         // Create a model that fails once then succeeds
//         struct FailOnceModel {
//             inner: Arc<FakeLanguageModel>,
//             failed_once: Arc<Mutex<bool>>,
//         }

//         impl LanguageModel for FailOnceModel {
//             fn id(&self) -> LanguageModelId {
//                 self.inner.id()
//             }

//             fn name(&self) -> LanguageModelName {
//                 self.inner.name()
//             }

//             fn provider_id(&self) -> LanguageModelProviderId {
//                 self.inner.provider_id()
//             }

//             fn provider_name(&self) -> LanguageModelProviderName {
//                 self.inner.provider_name()
//             }

//             fn supports_tools(&self) -> bool {
//                 self.inner.supports_tools()
//             }

//             fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
//                 self.inner.supports_tool_choice(choice)
//             }

//             fn supports_images(&self) -> bool {
//                 self.inner.supports_images()
//             }

//             fn telemetry_id(&self) -> String {
//                 self.inner.telemetry_id()
//             }

//             fn max_token_count(&self) -> u64 {
//                 self.inner.max_token_count()
//             }

//             fn count_tokens(
//                 &self,
//                 request: LanguageModelRequest,
//                 cx: &App,
//             ) -> BoxFuture<'static, Result<u64>> {
//                 self.inner.count_tokens(request, cx)
//             }

//             fn stream_completion(
//                 &self,
//                 request: LanguageModelRequest,
//                 cx: &AsyncApp,
//             ) -> BoxFuture<
//                 'static,
//                 Result<
//                     BoxStream<
//                         'static,
//                         Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
//                     >,
//                     LanguageModelCompletionError,
//                 >,
//             > {
//                 if !*self.failed_once.lock() {
//                     *self.failed_once.lock() = true;
//                     // Return error on first attempt
//                     let stream = futures::stream::once(async move {
//                         Err(LanguageModelCompletionError::Overloaded)
//                     });
//                     async move { Ok(stream.boxed()) }.boxed()
//                 } else {
//                     // Succeed on retry
//                     self.inner.stream_completion(request, cx)
//                 }
//             }
//         }

//         let fail_once_model = Arc::new(FailOnceModel {
//             inner: Arc::new(FakeLanguageModel::default()),
//             failed_once: Arc::new(Mutex::new(false)),
//         });

//         // Insert a user message
//         thread.update(cx, |thread, cx| {
//             thread.insert_user_message(
//                 "Test message",
//                 ContextLoadResult::default(),
//                 None,
//                 vec![],
//                 cx,
//             );
//         });

//         // Start completion with fail-once model
//         thread.update(cx, |thread, cx| {
//             thread.send_to_model(
//                 fail_once_model.clone(),
//                 CompletionIntent::UserPrompt,
//                 None,
//                 cx,
//             );
//         });

//         cx.run_until_parked();

//         // Verify retry state exists after first failure
//         thread.read_with(cx, |thread, _| {
//             assert!(
//                 thread.retry_state.is_some(),
//                 "Should have retry state after failure"
//             );
//         });

//         // Wait for retry delay
//         cx.executor()
//             .advance_clock(Duration::from_secs(BASE_RETRY_DELAY_SECS));
//         cx.run_until_parked();

//         // The retry should now use our FailOnceModel which should succeed
//         // We need to help the FakeLanguageModel complete the stream
//         let inner_fake = fail_once_model.inner.clone();

//         // Wait a bit for the retry to start
//         cx.run_until_parked();

//         // Check for pending completions and complete them
//         if let Some(pending) = inner_fake.pending_completions().first() {
//             inner_fake.stream_completion_response(pending, "Success!");
//             inner_fake.end_completion_stream(pending);
//         }
//         cx.run_until_parked();

//         thread.read_with(cx, |thread, _| {
//             assert!(
//                 thread.retry_state.is_none(),
//                 "Retry state should be cleared after successful completion"
//             );

//             let has_assistant_message = thread
//                 .messages
//                 .iter()
//                 .any(|msg| msg.role == Role::Assistant && !msg.ui_only);
//             assert!(
//                 has_assistant_message,
//                 "Should have an assistant message after successful retry"
//             );
//         });
//     }

//     #[gpui::test]
//     async fn test_rate_limit_retry_single_attempt(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(cx, json!({})).await;
//         let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

//         // Create a model that returns rate limit error with retry_after
//         struct RateLimitModel {
//             inner: Arc<FakeLanguageModel>,
//         }

//         impl LanguageModel for RateLimitModel {
//             fn id(&self) -> LanguageModelId {
//                 self.inner.id()
//             }

//             fn name(&self) -> LanguageModelName {
//                 self.inner.name()
//             }

//             fn provider_id(&self) -> LanguageModelProviderId {
//                 self.inner.provider_id()
//             }

//             fn provider_name(&self) -> LanguageModelProviderName {
//                 self.inner.provider_name()
//             }

//             fn supports_tools(&self) -> bool {
//                 self.inner.supports_tools()
//             }

//             fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
//                 self.inner.supports_tool_choice(choice)
//             }

//             fn supports_images(&self) -> bool {
//                 self.inner.supports_images()
//             }

//             fn telemetry_id(&self) -> String {
//                 self.inner.telemetry_id()
//             }

//             fn max_token_count(&self) -> u64 {
//                 self.inner.max_token_count()
//             }

//             fn count_tokens(
//                 &self,
//                 request: LanguageModelRequest,
//                 cx: &App,
//             ) -> BoxFuture<'static, Result<u64>> {
//                 self.inner.count_tokens(request, cx)
//             }

//             fn stream_completion(
//                 &self,
//                 _request: LanguageModelRequest,
//                 _cx: &AsyncApp,
//             ) -> BoxFuture<
//                 'static,
//                 Result<
//                     BoxStream<
//                         'static,
//                         Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
//                     >,
//                     LanguageModelCompletionError,
//                 >,
//             > {
//                 async move {
//                     let stream = futures::stream::once(async move {
//                         Err(LanguageModelCompletionError::RateLimitExceeded {
//                             retry_after: Duration::from_secs(TEST_RATE_LIMIT_RETRY_SECS),
//                         })
//                     });
//                     Ok(stream.boxed())
//                 }
//                 .boxed()
//             }

//             fn as_fake(&self) -> &FakeLanguageModel {
//                 &self.inner
//             }
//         }

//         let model = Arc::new(RateLimitModel {
//             inner: Arc::new(FakeLanguageModel::default()),
//         });

//         // Insert a user message
//         thread.update(cx, |thread, cx| {
//             thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
//         });

//         // Start completion
//         thread.update(cx, |thread, cx| {
//             thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
//         });

//         cx.run_until_parked();

//         let retry_count = thread.update(cx, |thread, _| {
//             thread
//                 .messages
//                 .iter()
//                 .filter(|m| {
//                     m.ui_only
//                         && m.segments.iter().any(|s| {
//                             if let MessageSegment::Text(text) = s {
//                                 text.contains("rate limit exceeded")
//                             } else {
//                                 false
//                             }
//                         })
//                 })
//                 .count()
//         });
//         assert_eq!(retry_count, 1, "Should have scheduled one retry");

//         thread.read_with(cx, |thread, _| {
//             assert!(
//                 thread.retry_state.is_none(),
//                 "Rate limit errors should not set retry_state"
//             );
//         });

//         // Verify we have one retry message
//         thread.read_with(cx, |thread, _| {
//             let retry_messages = thread
//                 .messages
//                 .iter()
//                 .filter(|msg| {
//                     msg.ui_only
//                         && msg.segments.iter().any(|seg| {
//                             if let MessageSegment::Text(text) = seg {
//                                 text.contains("rate limit exceeded")
//                             } else {
//                                 false
//                             }
//                         })
//                 })
//                 .count();
//             assert_eq!(
//                 retry_messages, 1,
//                 "Should have one rate limit retry message"
//             );
//         });

//         // Check that retry message doesn't include attempt count
//         thread.read_with(cx, |thread, _| {
//             let retry_message = thread
//                 .messages
//                 .iter()
//                 .find(|msg| msg.role == Role::System && msg.ui_only)
//                 .expect("Should have a retry message");

//             // Check that the message doesn't contain attempt count
//             if let Some(MessageSegment::Text(text)) = retry_message.segments.first() {
//                 assert!(
//                     !text.contains("attempt"),
//                     "Rate limit retry message should not contain attempt count"
//                 );
//                 assert!(
//                     text.contains(&format!(
//                         "Retrying in {} seconds",
//                         TEST_RATE_LIMIT_RETRY_SECS
//                     )),
//                     "Rate limit retry message should contain retry delay"
//                 );
//             }
//         });
//     }

//     #[gpui::test]
//     async fn test_ui_only_messages_not_sent_to_model(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(cx, json!({})).await;
//         let (_, _, thread, _, model) = setup_test_environment(cx, project.clone()).await;

//         // Insert a regular user message
//         thread.update(cx, |thread, cx| {
//             thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
//         });

//         // Insert a UI-only message (like our retry notifications)
//         thread.update(cx, |thread, cx| {
//             let id = thread.next_message_id.post_inc();
//             thread.messages.push(Message {
//                 id,
//                 role: Role::System,
//                 segments: vec![MessageSegment::Text(
//                     "This is a UI-only message that should not be sent to the model".to_string(),
//                 )],
//                 loaded_context: LoadedContext::default(),
//                 creases: Vec::new(),
//                 is_hidden: true,
//                 ui_only: true,
//             });
//             cx.emit(ThreadEvent::MessageAdded(id));
//         });

//         // Insert another regular message
//         thread.update(cx, |thread, cx| {
//             thread.insert_user_message(
//                 "How are you?",
//                 ContextLoadResult::default(),
//                 None,
//                 vec![],
//                 cx,
//             );
//         });

//         // Generate the completion request
//         let request = thread.update(cx, |thread, cx| {
//             thread.to_completion_request(model.clone(), CompletionIntent::UserPrompt, cx)
//         });

//         // Verify that the request only contains non-UI-only messages
//         // Should have system prompt + 2 user messages, but not the UI-only message
//         let user_messages: Vec<_> = request
//             .messages
//             .iter()
//             .filter(|msg| msg.role == Role::User)
//             .collect();
//         assert_eq!(
//             user_messages.len(),
//             2,
//             "Should have exactly 2 user messages"
//         );

//         // Verify the UI-only content is not present anywhere in the request
//         let request_text = request
//             .messages
//             .iter()
//             .flat_map(|msg| &msg.content)
//             .filter_map(|content| match content {
//                 MessageContent::Text(text) => Some(text.as_str()),
//                 _ => None,
//             })
//             .collect::<String>();

//         assert!(
//             !request_text.contains("UI-only message"),
//             "UI-only message content should not be in the request"
//         );

//         // Verify the thread still has all 3 messages (including UI-only)
//         thread.read_with(cx, |thread, _| {
//             assert_eq!(
//                 thread.messages().count(),
//                 3,
//                 "Thread should have 3 messages"
//             );
//             assert_eq!(
//                 thread.messages().filter(|m| m.ui_only).count(),
//                 1,
//                 "Thread should have 1 UI-only message"
//             );
//         });

//         // Verify that UI-only messages are not serialized
//         let serialized = thread
//             .update(cx, |thread, cx| thread.serialize(cx))
//             .await
//             .unwrap();
//         assert_eq!(
//             serialized.messages.len(),
//             2,
//             "Serialized thread should only have 2 messages (no UI-only)"
//         );
//     }

//     #[gpui::test]
//     async fn test_retry_cancelled_on_stop(cx: &mut TestAppContext) {
//         init_test_settings(cx);

//         let project = create_test_project(cx, json!({})).await;
//         let (_, _, thread, _, _base_model) = setup_test_environment(cx, project.clone()).await;

//         // Create model that returns overloaded error
//         let model = Arc::new(ErrorInjector::new(TestError::Overloaded));

//         // Insert a user message
//         thread.update(cx, |thread, cx| {
//             thread.insert_user_message("Hello!", ContextLoadResult::default(), None, vec![], cx);
//         });

//         // Start completion
//         thread.update(cx, |thread, cx| {
//             thread.send_to_model(model.clone(), CompletionIntent::UserPrompt, None, cx);
//         });

//         cx.run_until_parked();

//         // Verify retry was scheduled by checking for retry message
//         let has_retry_message = thread.read_with(cx, |thread, _| {
//             thread.messages.iter().any(|m| {
//                 m.ui_only
//                     && m.segments.iter().any(|s| {
//                         if let MessageSegment::Text(text) = s {
//                             text.contains("Retrying") && text.contains("seconds")
//                         } else {
//                             false
//                         }
//                     })
//             })
//         });
//         assert!(has_retry_message, "Should have scheduled a retry");

//         // Cancel the completion before the retry happens
//         thread.update(cx, |thread, cx| {
//             thread.cancel_last_completion(None, cx);
//         });

//         cx.run_until_parked();

//         // The retry should not have happened - no pending completions
//         let fake_model = model.as_fake();
//         assert_eq!(
//             fake_model.pending_completions().len(),
//             0,
//             "Should have no pending completions after cancellation"
//         );

//         // Verify the retry was cancelled by checking retry state
//         thread.read_with(cx, |thread, _| {
//             if let Some(retry_state) = &thread.retry_state {
//                 panic!(
//                     "retry_state should be cleared after cancellation, but found: attempt={}, max_attempts={}, intent={:?}",
//                     retry_state.attempt, retry_state.max_attempts, retry_state.intent
//                 );
//             }
//         });
//     }

//     fn test_summarize_error(
//         model: &Arc<dyn LanguageModel>,
//         thread: &Entity<Thread>,
//         cx: &mut TestAppContext,
//     ) {
//         thread.update(cx, |thread, cx| {
//             thread.insert_user_message("Hi!", ContextLoadResult::default(), None, vec![], cx);
//             thread.send_to_model(
//                 model.clone(),
//                 CompletionIntent::ThreadSummarization,
//                 None,
//                 cx,
//             );
//         });

//         let fake_model = model.as_fake();
//         simulate_successful_response(&fake_model, cx);

//         thread.read_with(cx, |thread, _| {
//             assert!(matches!(thread.summary(), ThreadSummary::Generating));
//             assert_eq!(thread.summary().or_default(), ThreadSummary::DEFAULT);
//         });

//         // Simulate summary request ending
//         cx.run_until_parked();
//         fake_model.end_last_completion_stream();
//         cx.run_until_parked();

//         // State is set to Error and default message
//         thread.read_with(cx, |thread, _| {
//             assert!(matches!(thread.summary(), ThreadSummary::Error));
//             assert_eq!(thread.summary().or_default(), ThreadSummary::DEFAULT);
//         });
//     }

//     fn simulate_successful_response(fake_model: &FakeLanguageModel, cx: &mut TestAppContext) {
//         cx.run_until_parked();
//         fake_model.stream_last_completion_response("Assistant response");
//         fake_model.end_last_completion_stream();
//         cx.run_until_parked();
//     }

//     fn init_test_settings(cx: &mut TestAppContext) {
//         cx.update(|cx| {
//             let settings_store = SettingsStore::test(cx);
//             cx.set_global(settings_store);
//             language::init(cx);
//             Project::init_settings(cx);
//             AgentSettings::register(cx);
//             prompt_store::init(cx);
//             thread_store::init(cx);
//             workspace::init_settings(cx);
//             language_model::init_settings(cx);
//             ThemeSettings::register(cx);
//             ToolRegistry::default_global(cx);
//         });
//     }

//     // Helper to create a test project with test files
//     async fn create_test_project(
//         cx: &mut TestAppContext,
//         files: serde_json::Value,
//     ) -> Entity<Project> {
//         let fs = FakeFs::new(cx.executor());
//         fs.insert_tree(path!("/test"), files).await;
//         Project::test(fs, [path!("/test").as_ref()], cx).await
//     }

//     async fn setup_test_environment(
//         cx: &mut TestAppContext,
//         project: Entity<Project>,
//     ) -> (
//         Entity<Workspace>,
//         Entity<ThreadStore>,
//         Entity<Thread>,
//         Entity<ContextStore>,
//         Arc<dyn LanguageModel>,
//     ) {
//         let (workspace, cx) =
//             cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

//         let thread_store = cx
//             .update(|_, cx| {
//                 ThreadStore::load(
//                     project.clone(),
//                     cx.new(|_| ToolWorkingSet::default()),
//                     None,
//                     Arc::new(PromptBuilder::new(None).unwrap()),
//                     cx,
//                 )
//             })
//             .await
//             .unwrap();

//         let thread = thread_store.update(cx, |store, cx| store.create_thread(cx));
//         let context_store = cx.new(|_cx| ContextStore::new(project.downgrade(), None));

//         let provider = Arc::new(FakeLanguageModelProvider);
//         let model = provider.test_model();
//         let model: Arc<dyn LanguageModel> = Arc::new(model);

//         cx.update(|_, cx| {
//             LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
//                 registry.set_default_model(
//                     Some(ConfiguredModel {
//                         provider: provider.clone(),
//                         model: model.clone(),
//                     }),
//                     cx,
//                 );
//                 registry.set_thread_summary_model(
//                     Some(ConfiguredModel {
//                         provider,
//                         model: model.clone(),
//                     }),
//                     cx,
//                 );
//             })
//         });

//         (workspace, thread_store, thread, context_store, model)
//     }

//     async fn add_file_to_context(
//         project: &Entity<Project>,
//         context_store: &Entity<ContextStore>,
//         path: &str,
//         cx: &mut TestAppContext,
//     ) -> Result<Entity<language::Buffer>> {
//         let buffer_path = project
//             .read_with(cx, |project, cx| project.find_project_path(path, cx))
//             .unwrap();

//         let buffer = project
//             .update(cx, |project, cx| {
//                 project.open_buffer(buffer_path.clone(), cx)
//             })
//             .await
//             .unwrap();

//         context_store.update(cx, |context_store, cx| {
//             context_store.add_file_from_buffer(&buffer_path, buffer.clone(), false, cx);
//         });

//         Ok(buffer)
//     }
// }
