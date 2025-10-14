use crate::{
    agent_profile::AgentProfile,
    context::{AgentContextHandle, LoadedContext},
    thread_store::{
        SerializedCrease, SerializedLanguageModel, SerializedMessage, SerializedMessageSegment,
        SerializedThread, SerializedToolResult, SerializedToolUse, SharedProjectContext,
        ThreadStore,
    },
    tool_use::{PendingToolUse, ToolUse, ToolUseState},
};
use action_log::ActionLog;
use agent_settings::{
    AgentProfileId, AgentSettings, SUMMARIZE_THREAD_DETAILED_PROMPT, SUMMARIZE_THREAD_PROMPT,
};
use anyhow::Result;
use assistant_tool::ToolWorkingSet;
use chrono::{DateTime, Utc};
use client::{ModelRequestUsage, RequestUsage};
use cloud_llm_client::{CompletionIntent, CompletionRequestStatus, Plan, UsageLimit};
use futures::{FutureExt, StreamExt as _, future::Shared};
use git::repository::DiffType;
use gpui::{
    App, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task, WeakEntity,
    Window,
};
use language_model::{
    ConfiguredModel, LanguageModel, LanguageModelCompletionEvent, LanguageModelId,
    LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelToolResult, LanguageModelToolUseId, MessageContent, Role, SelectedModel,
    StopReason, TokenUsage,
};
use postage::stream::Stream as _;
use project::{
    Project,
    git_store::{GitStore, RepositoryState},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{ops::Range, sync::Arc};
use thiserror::Error;
use util::ResultExt as _;
use uuid::Uuid;

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
    last_prompt_id: PromptId,
    project_context: SharedProjectContext,
    project: Entity<Project>,
    tools: Entity<ToolWorkingSet>,
    tool_use: ToolUseState,
    action_log: Entity<ActionLog>,
    initial_project_snapshot: Shared<Task<Option<Arc<ProjectSnapshot>>>>,
    request_token_usage: Vec<TokenUsage>,
    cumulative_token_usage: TokenUsage,
    exceeded_window_error: Option<ExceededWindowError>,
    tool_use_limit_reached: bool,
    request_callback: Option<
        Box<dyn FnMut(&LanguageModelRequest, &[Result<LanguageModelCompletionEvent, String>])>,
    >,
    remaining_turns: u32,
    configured_model: Option<ConfiguredModel>,
    profile: AgentProfile,
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
            last_prompt_id: PromptId::new(),
            project_context: system_prompt,
            project: project.clone(),
            tools: tools.clone(),
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
        project_context: SharedProjectContext,
        window: Option<&mut Window>, // None in headless mode
        cx: &mut Context<Self>,
    ) -> Self {
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
            last_prompt_id: PromptId::new(),
            project_context,
            project: project.clone(),
            tools: tools.clone(),
            tool_use,
            action_log: cx.new(|_| ActionLog::new(project)),
            initial_project_snapshot: Task::ready(serialized.initial_project_snapshot).shared(),
            request_token_usage: serialized.request_token_usage,
            cumulative_token_usage: serialized.cumulative_token_usage,
            exceeded_window_error: None,
            tool_use_limit_reached: serialized.tool_use_limit_reached,
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

    pub fn summary(&self) -> &ThreadSummary {
        &self.summary
    }

    pub fn messages(&self) -> impl ExactSizeIterator<Item = &Message> {
        self.messages.iter()
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

    pub fn action_log(&self) -> &Entity<ActionLog> {
        &self.action_log
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
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
