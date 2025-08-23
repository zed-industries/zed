use crate::{
    ContextServerRegistry, CopyPathTool, CreateDirectoryTool, DbLanguageModel, DbThread,
    DeletePathTool, DiagnosticsTool, EditFileTool, FetchTool, FindPathTool, GrepTool,
    ListDirectoryTool, MovePathTool, NowTool, OpenTool, ReadFileTool, SystemPromptTemplate,
    Template, Templates, TerminalTool, ThinkingTool, WebSearchTool,
};
use acp_thread::{MentionUri, UserMessageId};
use action_log::ActionLog;
use agent::thread::{GitState, ProjectSnapshot, WorktreeSnapshot};
use agent_client_protocol as acp;
use agent_settings::{
    AgentProfileId, AgentProfileSettings, AgentSettings, CompletionMode,
    SUMMARIZE_THREAD_DETAILED_PROMPT, SUMMARIZE_THREAD_PROMPT,
};
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::adapt_schema_to_format;
use chrono::{DateTime, Utc};
use client::{ModelRequestUsage, RequestUsage};
use cloud_llm_client::{CompletionIntent, CompletionRequestStatus, UsageLimit};
use collections::{HashMap, HashSet, IndexMap};
use fs::Fs;
use futures::{
    FutureExt,
    channel::{mpsc, oneshot},
    future::Shared,
    stream::FuturesUnordered,
};
use git::repository::DiffType;
use gpui::{
    App, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task, WeakEntity,
};
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelExt,
    LanguageModelImage, LanguageModelProviderId, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelRequestTool, LanguageModelToolResult,
    LanguageModelToolResultContent, LanguageModelToolSchemaFormat, LanguageModelToolUse,
    LanguageModelToolUseId, Role, SelectedModel, StopReason, TokenUsage,
};
use project::{
    Project,
    git_store::{GitStore, RepositoryState},
};
use prompt_store::ProjectContext;
use schemars::{JsonSchema, Schema};
use serde::{Deserialize, Serialize};
use settings::{Settings, update_settings_file};
use smol::stream::StreamExt;
use std::fmt::Write;
use std::{
    collections::BTreeMap,
    ops::RangeInclusive,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
use util::{ResultExt, debug_panic, markdown::MarkdownCodeBlock};
use uuid::Uuid;

const TOOL_CANCELED_MESSAGE: &str = "Tool canceled by user";
pub const MAX_TOOL_NAME_LENGTH: usize = 64;

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

pub(crate) const MAX_RETRY_ATTEMPTS: u8 = 4;
pub(crate) const BASE_RETRY_DELAY: Duration = Duration::from_secs(5);

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    User(UserMessage),
    Agent(AgentMessage),
    Resume,
}

impl Message {
    pub fn as_agent_message(&self) -> Option<&AgentMessage> {
        match self {
            Message::Agent(agent_message) => Some(agent_message),
            _ => None,
        }
    }

    pub fn to_request(&self) -> Vec<LanguageModelRequestMessage> {
        match self {
            Message::User(message) => vec![message.to_request()],
            Message::Agent(message) => message.to_request(),
            Message::Resume => vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["Continue where you left off".into()],
                cache: false,
            }],
        }
    }

    pub fn to_markdown(&self) -> String {
        match self {
            Message::User(message) => message.to_markdown(),
            Message::Agent(message) => message.to_markdown(),
            Message::Resume => "[resumed after tool use limit was reached]".into(),
        }
    }

    pub fn role(&self) -> Role {
        match self {
            Message::User(_) | Message::Resume => Role::User,
            Message::Agent(_) => Role::Assistant,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserMessage {
    pub id: UserMessageId,
    pub content: Vec<UserMessageContent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserMessageContent {
    Text(String),
    Mention { uri: MentionUri, content: String },
    Image(LanguageModelImage),
}

impl UserMessage {
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::from("## User\n\n");

        for content in &self.content {
            match content {
                UserMessageContent::Text(text) => {
                    markdown.push_str(text);
                    markdown.push('\n');
                }
                UserMessageContent::Image(_) => {
                    markdown.push_str("<image />\n");
                }
                UserMessageContent::Mention { uri, content } => {
                    if !content.is_empty() {
                        let _ = writeln!(&mut markdown, "{}\n\n{}", uri.as_link(), content);
                    } else {
                        let _ = writeln!(&mut markdown, "{}", uri.as_link());
                    }
                }
            }
        }

        markdown
    }

    fn to_request(&self) -> LanguageModelRequestMessage {
        let mut message = LanguageModelRequestMessage {
            role: Role::User,
            content: Vec::with_capacity(self.content.len()),
            cache: false,
        };

        const OPEN_CONTEXT: &str = "<context>\n\
            The following items were attached by the user. \
            They are up-to-date and don't need to be re-read.\n\n";

        const OPEN_FILES_TAG: &str = "<files>";
        const OPEN_DIRECTORIES_TAG: &str = "<directories>";
        const OPEN_SYMBOLS_TAG: &str = "<symbols>";
        const OPEN_SELECTIONS_TAG: &str = "<selections>";
        const OPEN_THREADS_TAG: &str = "<threads>";
        const OPEN_FETCH_TAG: &str = "<fetched_urls>";
        const OPEN_RULES_TAG: &str =
            "<rules>\nThe user has specified the following rules that should be applied:\n";

        let mut file_context = OPEN_FILES_TAG.to_string();
        let mut directory_context = OPEN_DIRECTORIES_TAG.to_string();
        let mut symbol_context = OPEN_SYMBOLS_TAG.to_string();
        let mut selection_context = OPEN_SELECTIONS_TAG.to_string();
        let mut thread_context = OPEN_THREADS_TAG.to_string();
        let mut fetch_context = OPEN_FETCH_TAG.to_string();
        let mut rules_context = OPEN_RULES_TAG.to_string();

        for chunk in &self.content {
            let chunk = match chunk {
                UserMessageContent::Text(text) => {
                    language_model::MessageContent::Text(text.clone())
                }
                UserMessageContent::Image(value) => {
                    language_model::MessageContent::Image(value.clone())
                }
                UserMessageContent::Mention { uri, content } => {
                    match uri {
                        MentionUri::File { abs_path } => {
                            write!(
                                &mut file_context,
                                "\n{}",
                                MarkdownCodeBlock {
                                    tag: &codeblock_tag(abs_path, None),
                                    text: &content.to_string(),
                                }
                            )
                            .ok();
                        }
                        MentionUri::PastedImage => {
                            debug_panic!("pasted image URI should not be used in mention content")
                        }
                        MentionUri::Directory { .. } => {
                            write!(&mut directory_context, "\n{}\n", content).ok();
                        }
                        MentionUri::Symbol {
                            abs_path: path,
                            line_range,
                            ..
                        } => {
                            write!(
                                &mut symbol_context,
                                "\n{}",
                                MarkdownCodeBlock {
                                    tag: &codeblock_tag(path, Some(line_range)),
                                    text: content
                                }
                            )
                            .ok();
                        }
                        MentionUri::Selection {
                            abs_path: path,
                            line_range,
                            ..
                        } => {
                            write!(
                                &mut selection_context,
                                "\n{}",
                                MarkdownCodeBlock {
                                    tag: &codeblock_tag(
                                        path.as_deref().unwrap_or("Untitled".as_ref()),
                                        Some(line_range)
                                    ),
                                    text: content
                                }
                            )
                            .ok();
                        }
                        MentionUri::Thread { .. } => {
                            write!(&mut thread_context, "\n{}\n", content).ok();
                        }
                        MentionUri::TextThread { .. } => {
                            write!(&mut thread_context, "\n{}\n", content).ok();
                        }
                        MentionUri::Rule { .. } => {
                            write!(
                                &mut rules_context,
                                "\n{}",
                                MarkdownCodeBlock {
                                    tag: "",
                                    text: content
                                }
                            )
                            .ok();
                        }
                        MentionUri::Fetch { url } => {
                            write!(&mut fetch_context, "\nFetch: {}\n\n{}", url, content).ok();
                        }
                    }

                    language_model::MessageContent::Text(uri.as_link().to_string())
                }
            };

            message.content.push(chunk);
        }

        let len_before_context = message.content.len();

        if file_context.len() > OPEN_FILES_TAG.len() {
            file_context.push_str("</files>\n");
            message
                .content
                .push(language_model::MessageContent::Text(file_context));
        }

        if directory_context.len() > OPEN_DIRECTORIES_TAG.len() {
            directory_context.push_str("</directories>\n");
            message
                .content
                .push(language_model::MessageContent::Text(directory_context));
        }

        if symbol_context.len() > OPEN_SYMBOLS_TAG.len() {
            symbol_context.push_str("</symbols>\n");
            message
                .content
                .push(language_model::MessageContent::Text(symbol_context));
        }

        if selection_context.len() > OPEN_SELECTIONS_TAG.len() {
            selection_context.push_str("</selections>\n");
            message
                .content
                .push(language_model::MessageContent::Text(selection_context));
        }

        if thread_context.len() > OPEN_THREADS_TAG.len() {
            thread_context.push_str("</threads>\n");
            message
                .content
                .push(language_model::MessageContent::Text(thread_context));
        }

        if fetch_context.len() > OPEN_FETCH_TAG.len() {
            fetch_context.push_str("</fetched_urls>\n");
            message
                .content
                .push(language_model::MessageContent::Text(fetch_context));
        }

        if rules_context.len() > OPEN_RULES_TAG.len() {
            rules_context.push_str("</user_rules>\n");
            message
                .content
                .push(language_model::MessageContent::Text(rules_context));
        }

        if message.content.len() > len_before_context {
            message.content.insert(
                len_before_context,
                language_model::MessageContent::Text(OPEN_CONTEXT.into()),
            );
            message
                .content
                .push(language_model::MessageContent::Text("</context>".into()));
        }

        message
    }
}

fn codeblock_tag(full_path: &Path, line_range: Option<&RangeInclusive<u32>>) -> String {
    let mut result = String::new();

    if let Some(extension) = full_path.extension().and_then(|ext| ext.to_str()) {
        let _ = write!(result, "{} ", extension);
    }

    let _ = write!(result, "{}", full_path.display());

    if let Some(range) = line_range {
        if range.start() == range.end() {
            let _ = write!(result, ":{}", range.start() + 1);
        } else {
            let _ = write!(result, ":{}-{}", range.start() + 1, range.end() + 1);
        }
    }

    result
}

impl AgentMessage {
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::from("## Assistant\n\n");

        for content in &self.content {
            match content {
                AgentMessageContent::Text(text) => {
                    markdown.push_str(text);
                    markdown.push('\n');
                }
                AgentMessageContent::Thinking { text, .. } => {
                    markdown.push_str("<think>");
                    markdown.push_str(text);
                    markdown.push_str("</think>\n");
                }
                AgentMessageContent::RedactedThinking(_) => {
                    markdown.push_str("<redacted_thinking />\n")
                }
                AgentMessageContent::ToolUse(tool_use) => {
                    markdown.push_str(&format!(
                        "**Tool Use**: {} (ID: {})\n",
                        tool_use.name, tool_use.id
                    ));
                    markdown.push_str(&format!(
                        "{}\n",
                        MarkdownCodeBlock {
                            tag: "json",
                            text: &format!("{:#}", tool_use.input)
                        }
                    ));
                }
            }
        }

        for tool_result in self.tool_results.values() {
            markdown.push_str(&format!(
                "**Tool Result**: {} (ID: {})\n\n",
                tool_result.tool_name, tool_result.tool_use_id
            ));
            if tool_result.is_error {
                markdown.push_str("**ERROR:**\n");
            }

            match &tool_result.content {
                LanguageModelToolResultContent::Text(text) => {
                    writeln!(markdown, "{text}\n").ok();
                }
                LanguageModelToolResultContent::Image(_) => {
                    writeln!(markdown, "<image />\n").ok();
                }
            }

            if let Some(output) = tool_result.output.as_ref() {
                writeln!(
                    markdown,
                    "**Debug Output**:\n\n```json\n{}\n```\n",
                    serde_json::to_string_pretty(output).unwrap()
                )
                .unwrap();
            }
        }

        markdown
    }

    pub fn to_request(&self) -> Vec<LanguageModelRequestMessage> {
        let mut assistant_message = LanguageModelRequestMessage {
            role: Role::Assistant,
            content: Vec::with_capacity(self.content.len()),
            cache: false,
        };
        for chunk in &self.content {
            let chunk = match chunk {
                AgentMessageContent::Text(text) => {
                    language_model::MessageContent::Text(text.clone())
                }
                AgentMessageContent::Thinking { text, signature } => {
                    language_model::MessageContent::Thinking {
                        text: text.clone(),
                        signature: signature.clone(),
                    }
                }
                AgentMessageContent::RedactedThinking(value) => {
                    language_model::MessageContent::RedactedThinking(value.clone())
                }
                AgentMessageContent::ToolUse(value) => {
                    language_model::MessageContent::ToolUse(value.clone())
                }
            };
            assistant_message.content.push(chunk);
        }

        let mut user_message = LanguageModelRequestMessage {
            role: Role::User,
            content: Vec::new(),
            cache: false,
        };

        for tool_result in self.tool_results.values() {
            user_message
                .content
                .push(language_model::MessageContent::ToolResult(
                    tool_result.clone(),
                ));
        }

        let mut messages = Vec::new();
        if !assistant_message.content.is_empty() {
            messages.push(assistant_message);
        }
        if !user_message.content.is_empty() {
            messages.push(user_message);
        }
        messages
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentMessage {
    pub content: Vec<AgentMessageContent>,
    pub tool_results: IndexMap<LanguageModelToolUseId, LanguageModelToolResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentMessageContent {
    Text(String),
    Thinking {
        text: String,
        signature: Option<String>,
    },
    RedactedThinking(String),
    ToolUse(LanguageModelToolUse),
}

#[derive(Debug)]
pub enum ThreadEvent {
    UserMessage(UserMessage),
    AgentText(String),
    AgentThinking(String),
    ToolCall(acp::ToolCall),
    ToolCallUpdate(acp_thread::ToolCallUpdate),
    ToolCallAuthorization(ToolCallAuthorization),
    Retry(acp_thread::RetryStatus),
    Stop(acp::StopReason),
}

#[derive(Debug)]
pub struct ToolCallAuthorization {
    pub tool_call: acp::ToolCallUpdate,
    pub options: Vec<acp::PermissionOption>,
    pub response: oneshot::Sender<acp::PermissionOptionId>,
}

#[derive(Debug, thiserror::Error)]
enum CompletionError {
    #[error("max tokens")]
    MaxTokens,
    #[error("refusal")]
    Refusal,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub struct Thread {
    id: acp::SessionId,
    prompt_id: PromptId,
    updated_at: DateTime<Utc>,
    title: Option<SharedString>,
    pending_title_generation: Option<Task<()>>,
    summary: Option<SharedString>,
    messages: Vec<Message>,
    completion_mode: CompletionMode,
    /// Holds the task that handles agent interaction until the end of the turn.
    /// Survives across multiple requests as the model performs tool calls and
    /// we run tools, report their results.
    running_turn: Option<RunningTurn>,
    pending_message: Option<AgentMessage>,
    tools: BTreeMap<SharedString, Arc<dyn AnyAgentTool>>,
    tool_use_limit_reached: bool,
    request_token_usage: HashMap<UserMessageId, language_model::TokenUsage>,
    #[allow(unused)]
    cumulative_token_usage: TokenUsage,
    #[allow(unused)]
    initial_project_snapshot: Shared<Task<Option<Arc<ProjectSnapshot>>>>,
    context_server_registry: Entity<ContextServerRegistry>,
    profile_id: AgentProfileId,
    project_context: Entity<ProjectContext>,
    templates: Arc<Templates>,
    model: Option<Arc<dyn LanguageModel>>,
    summarization_model: Option<Arc<dyn LanguageModel>>,
    pub(crate) project: Entity<Project>,
    pub(crate) action_log: Entity<ActionLog>,
}

impl Thread {
    pub fn new(
        project: Entity<Project>,
        project_context: Entity<ProjectContext>,
        context_server_registry: Entity<ContextServerRegistry>,
        templates: Arc<Templates>,
        model: Option<Arc<dyn LanguageModel>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let profile_id = AgentSettings::get_global(cx).default_profile.clone();
        let action_log = cx.new(|_cx| ActionLog::new(project.clone()));
        Self {
            id: acp::SessionId(uuid::Uuid::new_v4().to_string().into()),
            prompt_id: PromptId::new(),
            updated_at: Utc::now(),
            title: None,
            pending_title_generation: None,
            summary: None,
            messages: Vec::new(),
            completion_mode: AgentSettings::get_global(cx).preferred_completion_mode,
            running_turn: None,
            pending_message: None,
            tools: BTreeMap::default(),
            tool_use_limit_reached: false,
            request_token_usage: HashMap::default(),
            cumulative_token_usage: TokenUsage::default(),
            initial_project_snapshot: {
                let project_snapshot = Self::project_snapshot(project.clone(), cx);
                cx.foreground_executor()
                    .spawn(async move { Some(project_snapshot.await) })
                    .shared()
            },
            context_server_registry,
            profile_id,
            project_context,
            templates,
            model,
            summarization_model: None,
            project,
            action_log,
        }
    }

    pub fn id(&self) -> &acp::SessionId {
        &self.id
    }

    pub fn replay(
        &mut self,
        cx: &mut Context<Self>,
    ) -> mpsc::UnboundedReceiver<Result<ThreadEvent>> {
        let (tx, rx) = mpsc::unbounded();
        let stream = ThreadEventStream(tx);
        for message in &self.messages {
            match message {
                Message::User(user_message) => stream.send_user_message(user_message),
                Message::Agent(assistant_message) => {
                    for content in &assistant_message.content {
                        match content {
                            AgentMessageContent::Text(text) => stream.send_text(text),
                            AgentMessageContent::Thinking { text, .. } => {
                                stream.send_thinking(text)
                            }
                            AgentMessageContent::RedactedThinking(_) => {}
                            AgentMessageContent::ToolUse(tool_use) => {
                                self.replay_tool_call(
                                    tool_use,
                                    assistant_message.tool_results.get(&tool_use.id),
                                    &stream,
                                    cx,
                                );
                            }
                        }
                    }
                }
                Message::Resume => {}
            }
        }
        rx
    }

    fn replay_tool_call(
        &self,
        tool_use: &LanguageModelToolUse,
        tool_result: Option<&LanguageModelToolResult>,
        stream: &ThreadEventStream,
        cx: &mut Context<Self>,
    ) {
        let tool = self.tools.get(tool_use.name.as_ref()).cloned().or_else(|| {
            self.context_server_registry
                .read(cx)
                .servers()
                .find_map(|(_, tools)| {
                    if let Some(tool) = tools.get(tool_use.name.as_ref()) {
                        Some(tool.clone())
                    } else {
                        None
                    }
                })
        });

        let Some(tool) = tool else {
            stream
                .0
                .unbounded_send(Ok(ThreadEvent::ToolCall(acp::ToolCall {
                    id: acp::ToolCallId(tool_use.id.to_string().into()),
                    title: tool_use.name.to_string(),
                    kind: acp::ToolKind::Other,
                    status: acp::ToolCallStatus::Failed,
                    content: Vec::new(),
                    locations: Vec::new(),
                    raw_input: Some(tool_use.input.clone()),
                    raw_output: None,
                })))
                .ok();
            return;
        };

        let title = tool.initial_title(tool_use.input.clone());
        let kind = tool.kind();
        stream.send_tool_call(&tool_use.id, title, kind, tool_use.input.clone());

        let output = tool_result
            .as_ref()
            .and_then(|result| result.output.clone());
        if let Some(output) = output.clone() {
            let tool_event_stream = ToolCallEventStream::new(
                tool_use.id.clone(),
                stream.clone(),
                Some(self.project.read(cx).fs().clone()),
            );
            tool.replay(tool_use.input.clone(), output, tool_event_stream, cx)
                .log_err();
        }

        stream.update_tool_call_fields(
            &tool_use.id,
            acp::ToolCallUpdateFields {
                status: Some(acp::ToolCallStatus::Completed),
                raw_output: output,
                ..Default::default()
            },
        );
    }

    pub fn from_db(
        id: acp::SessionId,
        db_thread: DbThread,
        project: Entity<Project>,
        project_context: Entity<ProjectContext>,
        context_server_registry: Entity<ContextServerRegistry>,
        action_log: Entity<ActionLog>,
        templates: Arc<Templates>,
        cx: &mut Context<Self>,
    ) -> Self {
        let profile_id = db_thread
            .profile
            .unwrap_or_else(|| AgentSettings::get_global(cx).default_profile.clone());
        let model = LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            db_thread
                .model
                .and_then(|model| {
                    let model = SelectedModel {
                        provider: model.provider.clone().into(),
                        model: model.model.into(),
                    };
                    registry.select_model(&model, cx)
                })
                .or_else(|| registry.default_model())
                .map(|model| model.model)
        });

        Self {
            id,
            prompt_id: PromptId::new(),
            title: if db_thread.title.is_empty() {
                None
            } else {
                Some(db_thread.title.clone())
            },
            pending_title_generation: None,
            summary: db_thread.detailed_summary,
            messages: db_thread.messages,
            completion_mode: db_thread.completion_mode.unwrap_or_default(),
            running_turn: None,
            pending_message: None,
            tools: BTreeMap::default(),
            tool_use_limit_reached: false,
            request_token_usage: db_thread.request_token_usage.clone(),
            cumulative_token_usage: db_thread.cumulative_token_usage,
            initial_project_snapshot: Task::ready(db_thread.initial_project_snapshot).shared(),
            context_server_registry,
            profile_id,
            project_context,
            templates,
            model,
            summarization_model: None,
            project,
            action_log,
            updated_at: db_thread.updated_at,
        }
    }

    pub fn to_db(&self, cx: &App) -> Task<DbThread> {
        let initial_project_snapshot = self.initial_project_snapshot.clone();
        let mut thread = DbThread {
            title: self.title(),
            messages: self.messages.clone(),
            updated_at: self.updated_at,
            detailed_summary: self.summary.clone(),
            initial_project_snapshot: None,
            cumulative_token_usage: self.cumulative_token_usage,
            request_token_usage: self.request_token_usage.clone(),
            model: self.model.as_ref().map(|model| DbLanguageModel {
                provider: model.provider_id().to_string(),
                model: model.name().0.to_string(),
            }),
            completion_mode: Some(self.completion_mode),
            profile: Some(self.profile_id.clone()),
        };

        cx.background_spawn(async move {
            let initial_project_snapshot = initial_project_snapshot.await;
            thread.initial_project_snapshot = initial_project_snapshot;
            thread
        })
    }

    /// Create a snapshot of the current project state including git information and unsaved buffers.
    fn project_snapshot(
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Task<Arc<agent::thread::ProjectSnapshot>> {
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
                    if buffer.is_dirty()
                        && let Some(file) = buffer.file()
                    {
                        let path = file.path().to_string_lossy().to_string();
                        unsaved_buffers.push(path);
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
    ) -> Task<agent::thread::WorktreeSnapshot> {
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

    pub fn project_context(&self) -> &Entity<ProjectContext> {
        &self.project_context
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    pub fn action_log(&self) -> &Entity<ActionLog> {
        &self.action_log
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty() && self.title.is_none()
    }

    pub fn model(&self) -> Option<&Arc<dyn LanguageModel>> {
        self.model.as_ref()
    }

    pub fn set_model(&mut self, model: Arc<dyn LanguageModel>, cx: &mut Context<Self>) {
        let old_usage = self.latest_token_usage();
        self.model = Some(model);
        let new_usage = self.latest_token_usage();
        if old_usage != new_usage {
            cx.emit(TokenUsageUpdated(new_usage));
        }
        cx.notify()
    }

    pub fn summarization_model(&self) -> Option<&Arc<dyn LanguageModel>> {
        self.summarization_model.as_ref()
    }

    pub fn set_summarization_model(
        &mut self,
        model: Option<Arc<dyn LanguageModel>>,
        cx: &mut Context<Self>,
    ) {
        self.summarization_model = model;
        cx.notify()
    }

    pub fn completion_mode(&self) -> CompletionMode {
        self.completion_mode
    }

    pub fn set_completion_mode(&mut self, mode: CompletionMode, cx: &mut Context<Self>) {
        let old_usage = self.latest_token_usage();
        self.completion_mode = mode;
        let new_usage = self.latest_token_usage();
        if old_usage != new_usage {
            cx.emit(TokenUsageUpdated(new_usage));
        }
        cx.notify()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn last_message(&self) -> Option<Message> {
        if let Some(message) = self.pending_message.clone() {
            Some(Message::Agent(message))
        } else {
            self.messages.last().cloned()
        }
    }

    pub fn add_default_tools(&mut self, cx: &mut Context<Self>) {
        let language_registry = self.project.read(cx).languages().clone();
        self.add_tool(CopyPathTool::new(self.project.clone()));
        self.add_tool(CreateDirectoryTool::new(self.project.clone()));
        self.add_tool(DeletePathTool::new(
            self.project.clone(),
            self.action_log.clone(),
        ));
        self.add_tool(DiagnosticsTool::new(self.project.clone()));
        self.add_tool(EditFileTool::new(cx.weak_entity(), language_registry));
        self.add_tool(FetchTool::new(self.project.read(cx).client().http_client()));
        self.add_tool(FindPathTool::new(self.project.clone()));
        self.add_tool(GrepTool::new(self.project.clone()));
        self.add_tool(ListDirectoryTool::new(self.project.clone()));
        self.add_tool(MovePathTool::new(self.project.clone()));
        self.add_tool(NowTool);
        self.add_tool(OpenTool::new(self.project.clone()));
        self.add_tool(ReadFileTool::new(
            self.project.clone(),
            self.action_log.clone(),
        ));
        self.add_tool(TerminalTool::new(self.project.clone(), cx));
        self.add_tool(ThinkingTool);
        self.add_tool(WebSearchTool);
    }

    pub fn add_tool<T: AgentTool>(&mut self, tool: T) {
        self.tools.insert(T::name().into(), tool.erase());
    }

    pub fn remove_tool(&mut self, name: &str) -> bool {
        self.tools.remove(name).is_some()
    }

    pub fn profile(&self) -> &AgentProfileId {
        &self.profile_id
    }

    pub fn set_profile(&mut self, profile_id: AgentProfileId) {
        self.profile_id = profile_id;
    }

    pub fn cancel(&mut self, cx: &mut Context<Self>) {
        if let Some(running_turn) = self.running_turn.take() {
            running_turn.cancel();
        }
        self.flush_pending_message(cx);
    }

    fn update_token_usage(&mut self, update: language_model::TokenUsage, cx: &mut Context<Self>) {
        let Some(last_user_message) = self.last_user_message() else {
            return;
        };

        self.request_token_usage
            .insert(last_user_message.id.clone(), update);
        cx.emit(TokenUsageUpdated(self.latest_token_usage()));
        cx.notify();
    }

    pub fn truncate(&mut self, message_id: UserMessageId, cx: &mut Context<Self>) -> Result<()> {
        self.cancel(cx);
        let Some(position) = self.messages.iter().position(
            |msg| matches!(msg, Message::User(UserMessage { id, .. }) if id == &message_id),
        ) else {
            return Err(anyhow!("Message not found"));
        };

        for message in self.messages.drain(position..) {
            match message {
                Message::User(message) => {
                    self.request_token_usage.remove(&message.id);
                }
                Message::Agent(_) | Message::Resume => {}
            }
        }
        self.summary = None;
        cx.notify();
        Ok(())
    }

    pub fn latest_token_usage(&self) -> Option<acp_thread::TokenUsage> {
        let last_user_message = self.last_user_message()?;
        let tokens = self.request_token_usage.get(&last_user_message.id)?;
        let model = self.model.clone()?;

        Some(acp_thread::TokenUsage {
            max_tokens: model.max_token_count_for_mode(self.completion_mode.into()),
            used_tokens: tokens.total_tokens(),
        })
    }

    pub fn resume(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Result<mpsc::UnboundedReceiver<Result<ThreadEvent>>> {
        anyhow::ensure!(
            self.tool_use_limit_reached,
            "can only resume after tool use limit is reached"
        );

        self.messages.push(Message::Resume);
        cx.notify();

        log::info!("Total messages in thread: {}", self.messages.len());
        self.run_turn(cx)
    }

    /// Sending a message results in the model streaming a response, which could include tool calls.
    /// After calling tools, the model will stops and waits for any outstanding tool calls to be completed and their results sent.
    /// The returned channel will report all the occurrences in which the model stops before erroring or ending its turn.
    pub fn send<T>(
        &mut self,
        id: UserMessageId,
        content: impl IntoIterator<Item = T>,
        cx: &mut Context<Self>,
    ) -> Result<mpsc::UnboundedReceiver<Result<ThreadEvent>>>
    where
        T: Into<UserMessageContent>,
    {
        let model = self.model().context("No language model configured")?;

        log::info!("Thread::send called with model: {:?}", model.name());
        self.advance_prompt_id();

        let content = content.into_iter().map(Into::into).collect::<Vec<_>>();
        log::debug!("Thread::send content: {:?}", content);

        self.messages
            .push(Message::User(UserMessage { id, content }));
        cx.notify();

        log::info!("Total messages in thread: {}", self.messages.len());
        self.run_turn(cx)
    }

    fn run_turn(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Result<mpsc::UnboundedReceiver<Result<ThreadEvent>>> {
        self.cancel(cx);

        let model = self.model.clone().context("No language model configured")?;
        let profile = AgentSettings::get_global(cx)
            .profiles
            .get(&self.profile_id)
            .context("Profile not found")?;
        let (events_tx, events_rx) = mpsc::unbounded::<Result<ThreadEvent>>();
        let event_stream = ThreadEventStream(events_tx);
        let message_ix = self.messages.len().saturating_sub(1);
        self.tool_use_limit_reached = false;
        self.summary = None;
        self.running_turn = Some(RunningTurn {
            event_stream: event_stream.clone(),
            tools: self.enabled_tools(profile, &model, cx),
            _task: cx.spawn(async move |this, cx| {
                log::info!("Starting agent turn execution");

                let turn_result: Result<()> = async {
                    let mut intent = CompletionIntent::UserPrompt;
                    loop {
                        Self::stream_completion(&this, &model, intent, &event_stream, cx).await?;

                        let mut end_turn = true;
                        this.update(cx, |this, cx| {
                            // Generate title if needed.
                            if this.title.is_none() && this.pending_title_generation.is_none() {
                                this.generate_title(cx);
                            }

                            // End the turn if the model didn't use tools.
                            let message = this.pending_message.as_ref();
                            end_turn =
                                message.map_or(true, |message| message.tool_results.is_empty());
                            this.flush_pending_message(cx);
                        })?;

                        if this.read_with(cx, |this, _| this.tool_use_limit_reached)? {
                            log::info!("Tool use limit reached, completing turn");
                            return Err(language_model::ToolUseLimitReachedError.into());
                        } else if end_turn {
                            log::info!("No tool uses found, completing turn");
                            return Ok(());
                        } else {
                            intent = CompletionIntent::ToolResults;
                        }
                    }
                }
                .await;
                _ = this.update(cx, |this, cx| this.flush_pending_message(cx));

                match turn_result {
                    Ok(()) => {
                        log::info!("Turn execution completed");
                        event_stream.send_stop(acp::StopReason::EndTurn);
                    }
                    Err(error) => {
                        log::error!("Turn execution failed: {:?}", error);
                        match error.downcast::<CompletionError>() {
                            Ok(CompletionError::Refusal) => {
                                event_stream.send_stop(acp::StopReason::Refusal);
                                _ = this.update(cx, |this, _| this.messages.truncate(message_ix));
                            }
                            Ok(CompletionError::MaxTokens) => {
                                event_stream.send_stop(acp::StopReason::MaxTokens);
                            }
                            Ok(CompletionError::Other(error)) | Err(error) => {
                                event_stream.send_error(error);
                            }
                        }
                    }
                }

                _ = this.update(cx, |this, _| this.running_turn.take());
            }),
        });
        Ok(events_rx)
    }

    async fn stream_completion(
        this: &WeakEntity<Self>,
        model: &Arc<dyn LanguageModel>,
        completion_intent: CompletionIntent,
        event_stream: &ThreadEventStream,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        log::debug!("Stream completion started successfully");
        let request = this.update(cx, |this, cx| {
            this.build_completion_request(completion_intent, cx)
        })??;

        let mut attempt = None;
        'retry: loop {
            telemetry::event!(
                "Agent Thread Completion",
                thread_id = this.read_with(cx, |this, _| this.id.to_string())?,
                prompt_id = this.read_with(cx, |this, _| this.prompt_id.to_string())?,
                model = model.telemetry_id(),
                model_provider = model.provider_id().to_string(),
                attempt
            );

            log::info!(
                "Calling model.stream_completion, attempt {}",
                attempt.unwrap_or(0)
            );
            let mut events = model
                .stream_completion(request.clone(), cx)
                .await
                .map_err(|error| anyhow!(error))?;
            let mut tool_results = FuturesUnordered::new();

            while let Some(event) = events.next().await {
                match event {
                    Ok(event) => {
                        log::trace!("Received completion event: {:?}", event);
                        tool_results.extend(this.update(cx, |this, cx| {
                            this.handle_streamed_completion_event(event, event_stream, cx)
                        })??);
                    }
                    Err(error) => {
                        let completion_mode =
                            this.read_with(cx, |thread, _cx| thread.completion_mode())?;
                        if completion_mode == CompletionMode::Normal {
                            return Err(anyhow!(error))?;
                        }

                        let Some(strategy) = Self::retry_strategy_for(&error) else {
                            return Err(anyhow!(error))?;
                        };

                        let max_attempts = match &strategy {
                            RetryStrategy::ExponentialBackoff { max_attempts, .. } => *max_attempts,
                            RetryStrategy::Fixed { max_attempts, .. } => *max_attempts,
                        };

                        let attempt = attempt.get_or_insert(0u8);

                        *attempt += 1;

                        let attempt = *attempt;
                        if attempt > max_attempts {
                            return Err(anyhow!(error))?;
                        }

                        let delay = match &strategy {
                            RetryStrategy::ExponentialBackoff { initial_delay, .. } => {
                                let delay_secs =
                                    initial_delay.as_secs() * 2u64.pow((attempt - 1) as u32);
                                Duration::from_secs(delay_secs)
                            }
                            RetryStrategy::Fixed { delay, .. } => *delay,
                        };
                        log::debug!("Retry attempt {attempt} with delay {delay:?}");

                        event_stream.send_retry(acp_thread::RetryStatus {
                            last_error: error.to_string().into(),
                            attempt: attempt as usize,
                            max_attempts: max_attempts as usize,
                            started_at: Instant::now(),
                            duration: delay,
                        });

                        cx.background_executor().timer(delay).await;
                        continue 'retry;
                    }
                }
            }

            while let Some(tool_result) = tool_results.next().await {
                log::info!("Tool finished {:?}", tool_result);

                event_stream.update_tool_call_fields(
                    &tool_result.tool_use_id,
                    acp::ToolCallUpdateFields {
                        status: Some(if tool_result.is_error {
                            acp::ToolCallStatus::Failed
                        } else {
                            acp::ToolCallStatus::Completed
                        }),
                        raw_output: tool_result.output.clone(),
                        ..Default::default()
                    },
                );
                this.update(cx, |this, _cx| {
                    this.pending_message()
                        .tool_results
                        .insert(tool_result.tool_use_id.clone(), tool_result);
                })?;
            }

            return Ok(());
        }
    }

    pub fn build_system_message(&self, cx: &App) -> LanguageModelRequestMessage {
        log::debug!("Building system message");
        let prompt = SystemPromptTemplate {
            project: self.project_context.read(cx),
            available_tools: self.tools.keys().cloned().collect(),
        }
        .render(&self.templates)
        .context("failed to build system prompt")
        .expect("Invalid template");
        log::debug!("System message built");
        LanguageModelRequestMessage {
            role: Role::System,
            content: vec![prompt.into()],
            cache: true,
        }
    }

    /// A helper method that's called on every streamed completion event.
    /// Returns an optional tool result task, which the main agentic loop will
    /// send back to the model when it resolves.
    fn handle_streamed_completion_event(
        &mut self,
        event: LanguageModelCompletionEvent,
        event_stream: &ThreadEventStream,
        cx: &mut Context<Self>,
    ) -> Result<Option<Task<LanguageModelToolResult>>> {
        log::trace!("Handling streamed completion event: {:?}", event);
        use LanguageModelCompletionEvent::*;

        match event {
            StartMessage { .. } => {
                self.flush_pending_message(cx);
                self.pending_message = Some(AgentMessage::default());
            }
            Text(new_text) => self.handle_text_event(new_text, event_stream, cx),
            Thinking { text, signature } => {
                self.handle_thinking_event(text, signature, event_stream, cx)
            }
            RedactedThinking { data } => self.handle_redacted_thinking_event(data, cx),
            ToolUse(tool_use) => {
                return Ok(self.handle_tool_use_event(tool_use, event_stream, cx));
            }
            ToolUseJsonParseError {
                id,
                tool_name,
                raw_input,
                json_parse_error,
            } => {
                return Ok(Some(Task::ready(
                    self.handle_tool_use_json_parse_error_event(
                        id,
                        tool_name,
                        raw_input,
                        json_parse_error,
                    ),
                )));
            }
            UsageUpdate(usage) => {
                telemetry::event!(
                    "Agent Thread Completion Usage Updated",
                    thread_id = self.id.to_string(),
                    prompt_id = self.prompt_id.to_string(),
                    model = self.model.as_ref().map(|m| m.telemetry_id()),
                    model_provider = self.model.as_ref().map(|m| m.provider_id().to_string()),
                    input_tokens = usage.input_tokens,
                    output_tokens = usage.output_tokens,
                    cache_creation_input_tokens = usage.cache_creation_input_tokens,
                    cache_read_input_tokens = usage.cache_read_input_tokens,
                );
                self.update_token_usage(usage, cx);
            }
            StatusUpdate(CompletionRequestStatus::UsageUpdated { amount, limit }) => {
                self.update_model_request_usage(amount, limit, cx);
            }
            StatusUpdate(
                CompletionRequestStatus::Started
                | CompletionRequestStatus::Queued { .. }
                | CompletionRequestStatus::Failed { .. },
            ) => {}
            StatusUpdate(CompletionRequestStatus::ToolUseLimitReached) => {
                self.tool_use_limit_reached = true;
            }
            Stop(StopReason::Refusal) => return Err(CompletionError::Refusal.into()),
            Stop(StopReason::MaxTokens) => return Err(CompletionError::MaxTokens.into()),
            Stop(StopReason::ToolUse | StopReason::EndTurn) => {}
        }

        Ok(None)
    }

    fn handle_text_event(
        &mut self,
        new_text: String,
        event_stream: &ThreadEventStream,
        cx: &mut Context<Self>,
    ) {
        event_stream.send_text(&new_text);

        let last_message = self.pending_message();
        if let Some(AgentMessageContent::Text(text)) = last_message.content.last_mut() {
            text.push_str(&new_text);
        } else {
            last_message
                .content
                .push(AgentMessageContent::Text(new_text));
        }

        cx.notify();
    }

    fn handle_thinking_event(
        &mut self,
        new_text: String,
        new_signature: Option<String>,
        event_stream: &ThreadEventStream,
        cx: &mut Context<Self>,
    ) {
        event_stream.send_thinking(&new_text);

        let last_message = self.pending_message();
        if let Some(AgentMessageContent::Thinking { text, signature }) =
            last_message.content.last_mut()
        {
            text.push_str(&new_text);
            *signature = new_signature.or(signature.take());
        } else {
            last_message.content.push(AgentMessageContent::Thinking {
                text: new_text,
                signature: new_signature,
            });
        }

        cx.notify();
    }

    fn handle_redacted_thinking_event(&mut self, data: String, cx: &mut Context<Self>) {
        let last_message = self.pending_message();
        last_message
            .content
            .push(AgentMessageContent::RedactedThinking(data));
        cx.notify();
    }

    fn handle_tool_use_event(
        &mut self,
        tool_use: LanguageModelToolUse,
        event_stream: &ThreadEventStream,
        cx: &mut Context<Self>,
    ) -> Option<Task<LanguageModelToolResult>> {
        cx.notify();

        let tool = self.tool(tool_use.name.as_ref());
        let mut title = SharedString::from(&tool_use.name);
        let mut kind = acp::ToolKind::Other;
        if let Some(tool) = tool.as_ref() {
            title = tool.initial_title(tool_use.input.clone());
            kind = tool.kind();
        }

        // Ensure the last message ends in the current tool use
        let last_message = self.pending_message();
        let push_new_tool_use = last_message.content.last_mut().is_none_or(|content| {
            if let AgentMessageContent::ToolUse(last_tool_use) = content {
                if last_tool_use.id == tool_use.id {
                    *last_tool_use = tool_use.clone();
                    false
                } else {
                    true
                }
            } else {
                true
            }
        });

        if push_new_tool_use {
            event_stream.send_tool_call(&tool_use.id, title, kind, tool_use.input.clone());
            last_message
                .content
                .push(AgentMessageContent::ToolUse(tool_use.clone()));
        } else {
            event_stream.update_tool_call_fields(
                &tool_use.id,
                acp::ToolCallUpdateFields {
                    title: Some(title.into()),
                    kind: Some(kind),
                    raw_input: Some(tool_use.input.clone()),
                    ..Default::default()
                },
            );
        }

        if !tool_use.is_input_complete {
            return None;
        }

        let Some(tool) = tool else {
            let content = format!("No tool named {} exists", tool_use.name);
            return Some(Task::ready(LanguageModelToolResult {
                content: LanguageModelToolResultContent::Text(Arc::from(content)),
                tool_use_id: tool_use.id,
                tool_name: tool_use.name,
                is_error: true,
                output: None,
            }));
        };

        let fs = self.project.read(cx).fs().clone();
        let tool_event_stream =
            ToolCallEventStream::new(tool_use.id.clone(), event_stream.clone(), Some(fs));
        tool_event_stream.update_fields(acp::ToolCallUpdateFields {
            status: Some(acp::ToolCallStatus::InProgress),
            ..Default::default()
        });
        let supports_images = self.model().is_some_and(|model| model.supports_images());
        let tool_result = tool.run(tool_use.input, tool_event_stream, cx);
        log::info!("Running tool {}", tool_use.name);
        Some(cx.foreground_executor().spawn(async move {
            let tool_result = tool_result.await.and_then(|output| {
                if let LanguageModelToolResultContent::Image(_) = &output.llm_output
                    && !supports_images
                {
                    return Err(anyhow!(
                        "Attempted to read an image, but this model doesn't support it.",
                    ));
                }
                Ok(output)
            });

            match tool_result {
                Ok(output) => LanguageModelToolResult {
                    tool_use_id: tool_use.id,
                    tool_name: tool_use.name,
                    is_error: false,
                    content: output.llm_output,
                    output: Some(output.raw_output),
                },
                Err(error) => LanguageModelToolResult {
                    tool_use_id: tool_use.id,
                    tool_name: tool_use.name,
                    is_error: true,
                    content: LanguageModelToolResultContent::Text(Arc::from(error.to_string())),
                    output: None,
                },
            }
        }))
    }

    fn handle_tool_use_json_parse_error_event(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        raw_input: Arc<str>,
        json_parse_error: String,
    ) -> LanguageModelToolResult {
        let tool_output = format!("Error parsing input JSON: {json_parse_error}");
        LanguageModelToolResult {
            tool_use_id,
            tool_name,
            is_error: true,
            content: LanguageModelToolResultContent::Text(tool_output.into()),
            output: Some(serde_json::Value::String(raw_input.to_string())),
        }
    }

    fn update_model_request_usage(&self, amount: usize, limit: UsageLimit, cx: &mut Context<Self>) {
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

    pub fn title(&self) -> SharedString {
        self.title.clone().unwrap_or("New Thread".into())
    }

    pub fn summary(&mut self, cx: &mut Context<Self>) -> Task<Result<SharedString>> {
        if let Some(summary) = self.summary.as_ref() {
            return Task::ready(Ok(summary.clone()));
        }
        let Some(model) = self.summarization_model.clone() else {
            return Task::ready(Err(anyhow!("No summarization model available")));
        };
        let mut request = LanguageModelRequest {
            intent: Some(CompletionIntent::ThreadContextSummarization),
            temperature: AgentSettings::temperature_for_model(&model, cx),
            ..Default::default()
        };

        for message in &self.messages {
            request.messages.extend(message.to_request());
        }

        request.messages.push(LanguageModelRequestMessage {
            role: Role::User,
            content: vec![SUMMARIZE_THREAD_DETAILED_PROMPT.into()],
            cache: false,
        });
        cx.spawn(async move |this, cx| {
            let mut summary = String::new();
            let mut messages = model.stream_completion(request, cx).await?;
            while let Some(event) = messages.next().await {
                let event = event?;
                let text = match event {
                    LanguageModelCompletionEvent::Text(text) => text,
                    LanguageModelCompletionEvent::StatusUpdate(
                        CompletionRequestStatus::UsageUpdated { amount, limit },
                    ) => {
                        this.update(cx, |thread, cx| {
                            thread.update_model_request_usage(amount, limit, cx);
                        })?;
                        continue;
                    }
                    _ => continue,
                };

                let mut lines = text.lines();
                summary.extend(lines.next());
            }

            log::info!("Setting summary: {}", summary);
            let summary = SharedString::from(summary);

            this.update(cx, |this, cx| {
                this.summary = Some(summary.clone());
                cx.notify()
            })?;

            Ok(summary)
        })
    }

    fn generate_title(&mut self, cx: &mut Context<Self>) {
        let Some(model) = self.summarization_model.clone() else {
            return;
        };

        log::info!(
            "Generating title with model: {:?}",
            self.summarization_model.as_ref().map(|model| model.name())
        );
        let mut request = LanguageModelRequest {
            intent: Some(CompletionIntent::ThreadSummarization),
            temperature: AgentSettings::temperature_for_model(&model, cx),
            ..Default::default()
        };

        for message in &self.messages {
            request.messages.extend(message.to_request());
        }

        request.messages.push(LanguageModelRequestMessage {
            role: Role::User,
            content: vec![SUMMARIZE_THREAD_PROMPT.into()],
            cache: false,
        });
        self.pending_title_generation = Some(cx.spawn(async move |this, cx| {
            let mut title = String::new();

            let generate = async {
                let mut messages = model.stream_completion(request, cx).await?;
                while let Some(event) = messages.next().await {
                    let event = event?;
                    let text = match event {
                        LanguageModelCompletionEvent::Text(text) => text,
                        LanguageModelCompletionEvent::StatusUpdate(
                            CompletionRequestStatus::UsageUpdated { amount, limit },
                        ) => {
                            this.update(cx, |thread, cx| {
                                thread.update_model_request_usage(amount, limit, cx);
                            })?;
                            continue;
                        }
                        _ => continue,
                    };

                    let mut lines = text.lines();
                    title.extend(lines.next());

                    // Stop if the LLM generated multiple lines.
                    if lines.next().is_some() {
                        break;
                    }
                }
                anyhow::Ok(())
            };

            if generate.await.context("failed to generate title").is_ok() {
                _ = this.update(cx, |this, cx| this.set_title(title.into(), cx));
            }
            _ = this.update(cx, |this, _| this.pending_title_generation = None);
        }));
    }

    pub fn set_title(&mut self, title: SharedString, cx: &mut Context<Self>) {
        self.pending_title_generation = None;
        if Some(&title) != self.title.as_ref() {
            self.title = Some(title);
            cx.emit(TitleUpdated);
            cx.notify();
        }
    }

    fn last_user_message(&self) -> Option<&UserMessage> {
        self.messages
            .iter()
            .rev()
            .find_map(|message| match message {
                Message::User(user_message) => Some(user_message),
                Message::Agent(_) => None,
                Message::Resume => None,
            })
    }

    fn pending_message(&mut self) -> &mut AgentMessage {
        self.pending_message.get_or_insert_default()
    }

    fn flush_pending_message(&mut self, cx: &mut Context<Self>) {
        let Some(mut message) = self.pending_message.take() else {
            return;
        };

        for content in &message.content {
            let AgentMessageContent::ToolUse(tool_use) = content else {
                continue;
            };

            if !message.tool_results.contains_key(&tool_use.id) {
                message.tool_results.insert(
                    tool_use.id.clone(),
                    LanguageModelToolResult {
                        tool_use_id: tool_use.id.clone(),
                        tool_name: tool_use.name.clone(),
                        is_error: true,
                        content: LanguageModelToolResultContent::Text(TOOL_CANCELED_MESSAGE.into()),
                        output: None,
                    },
                );
            }
        }

        self.messages.push(Message::Agent(message));
        self.updated_at = Utc::now();
        self.summary = None;
        cx.notify()
    }

    pub(crate) fn build_completion_request(
        &self,
        completion_intent: CompletionIntent,
        cx: &mut App,
    ) -> Result<LanguageModelRequest> {
        let model = self.model().context("No language model configured")?;
        let tools = if let Some(turn) = self.running_turn.as_ref() {
            turn.tools
                .iter()
                .filter_map(|(tool_name, tool)| {
                    log::trace!("Including tool: {}", tool_name);
                    Some(LanguageModelRequestTool {
                        name: tool_name.to_string(),
                        description: tool.description().to_string(),
                        input_schema: tool.input_schema(model.tool_input_format()).log_err()?,
                    })
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        log::debug!("Building completion request");
        log::debug!("Completion intent: {:?}", completion_intent);
        log::debug!("Completion mode: {:?}", self.completion_mode);

        let messages = self.build_request_messages(cx);
        log::info!("Request will include {} messages", messages.len());
        log::info!("Request includes {} tools", tools.len());

        let request = LanguageModelRequest {
            thread_id: Some(self.id.to_string()),
            prompt_id: Some(self.prompt_id.to_string()),
            intent: Some(completion_intent),
            mode: Some(self.completion_mode.into()),
            messages,
            tools,
            tool_choice: None,
            stop: Vec::new(),
            temperature: AgentSettings::temperature_for_model(model, cx),
            thinking_allowed: true,
        };

        log::debug!("Completion request built successfully");
        Ok(request)
    }

    fn enabled_tools(
        &self,
        profile: &AgentProfileSettings,
        model: &Arc<dyn LanguageModel>,
        cx: &App,
    ) -> BTreeMap<SharedString, Arc<dyn AnyAgentTool>> {
        fn truncate(tool_name: &SharedString) -> SharedString {
            if tool_name.len() > MAX_TOOL_NAME_LENGTH {
                let mut truncated = tool_name.to_string();
                truncated.truncate(MAX_TOOL_NAME_LENGTH);
                truncated.into()
            } else {
                tool_name.clone()
            }
        }

        let mut tools = self
            .tools
            .iter()
            .filter_map(|(tool_name, tool)| {
                if tool.supported_provider(&model.provider_id())
                    && profile.is_tool_enabled(tool_name)
                {
                    Some((truncate(tool_name), tool.clone()))
                } else {
                    None
                }
            })
            .collect::<BTreeMap<_, _>>();

        let mut context_server_tools = Vec::new();
        let mut seen_tools = tools.keys().cloned().collect::<HashSet<_>>();
        let mut duplicate_tool_names = HashSet::default();
        for (server_id, server_tools) in self.context_server_registry.read(cx).servers() {
            for (tool_name, tool) in server_tools {
                if profile.is_context_server_tool_enabled(&server_id.0, &tool_name) {
                    let tool_name = truncate(tool_name);
                    if !seen_tools.insert(tool_name.clone()) {
                        duplicate_tool_names.insert(tool_name.clone());
                    }
                    context_server_tools.push((server_id.clone(), tool_name, tool.clone()));
                }
            }
        }

        // When there are duplicate tool names, disambiguate by prefixing them
        // with the server ID. In the rare case there isn't enough space for the
        // disambiguated tool name, keep only the last tool with this name.
        for (server_id, tool_name, tool) in context_server_tools {
            if duplicate_tool_names.contains(&tool_name) {
                let available = MAX_TOOL_NAME_LENGTH.saturating_sub(tool_name.len());
                if available >= 2 {
                    let mut disambiguated = server_id.0.to_string();
                    disambiguated.truncate(available - 1);
                    disambiguated.push('_');
                    disambiguated.push_str(&tool_name);
                    tools.insert(disambiguated.into(), tool.clone());
                } else {
                    tools.insert(tool_name, tool.clone());
                }
            } else {
                tools.insert(tool_name, tool.clone());
            }
        }

        tools
    }

    fn tool(&self, name: &str) -> Option<Arc<dyn AnyAgentTool>> {
        self.running_turn.as_ref()?.tools.get(name).cloned()
    }

    fn build_request_messages(&self, cx: &App) -> Vec<LanguageModelRequestMessage> {
        log::trace!(
            "Building request messages from {} thread messages",
            self.messages.len()
        );
        let mut messages = vec![self.build_system_message(cx)];
        for message in &self.messages {
            messages.extend(message.to_request());
        }

        if let Some(message) = self.pending_message.as_ref() {
            messages.extend(message.to_request());
        }

        if let Some(last_user_message) = messages
            .iter_mut()
            .rev()
            .find(|message| message.role == Role::User)
        {
            last_user_message.cache = true;
        }

        messages
    }

    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        for (ix, message) in self.messages.iter().enumerate() {
            if ix > 0 {
                markdown.push('\n');
            }
            markdown.push_str(&message.to_markdown());
        }

        if let Some(message) = self.pending_message.as_ref() {
            markdown.push('\n');
            markdown.push_str(&message.to_markdown());
        }

        markdown
    }

    fn advance_prompt_id(&mut self) {
        self.prompt_id = PromptId::new();
    }

    fn retry_strategy_for(error: &LanguageModelCompletionError) -> Option<RetryStrategy> {
        use LanguageModelCompletionError::*;
        use http_client::StatusCode;

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
                if err.is::<language_model::PaymentRequiredError>()
                    || err.is::<language_model::ModelRequestLimitReachedError>() =>
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
}

struct RunningTurn {
    /// Holds the task that handles agent interaction until the end of the turn.
    /// Survives across multiple requests as the model performs tool calls and
    /// we run tools, report their results.
    _task: Task<()>,
    /// The current event stream for the running turn. Used to report a final
    /// cancellation event if we cancel the turn.
    event_stream: ThreadEventStream,
    /// The tools that were enabled for this turn.
    tools: BTreeMap<SharedString, Arc<dyn AnyAgentTool>>,
}

impl RunningTurn {
    fn cancel(self) {
        log::debug!("Cancelling in progress turn");
        self.event_stream.send_canceled();
    }
}

pub struct TokenUsageUpdated(pub Option<acp_thread::TokenUsage>);

impl EventEmitter<TokenUsageUpdated> for Thread {}

pub struct TitleUpdated;

impl EventEmitter<TitleUpdated> for Thread {}

pub trait AgentTool
where
    Self: 'static + Sized,
{
    type Input: for<'de> Deserialize<'de> + Serialize + JsonSchema;
    type Output: for<'de> Deserialize<'de> + Serialize + Into<LanguageModelToolResultContent>;

    fn name() -> &'static str;

    fn description(&self) -> SharedString {
        let schema = schemars::schema_for!(Self::Input);
        SharedString::new(
            schema
                .get("description")
                .and_then(|description| description.as_str())
                .unwrap_or_default(),
        )
    }

    fn kind() -> acp::ToolKind;

    /// The initial tool title to display. Can be updated during the tool run.
    fn initial_title(&self, input: Result<Self::Input, serde_json::Value>) -> SharedString;

    /// Returns the JSON schema that describes the tool's input.
    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Schema {
        crate::tool_schema::root_schema_for::<Self::Input>(format)
    }

    /// Some tools rely on a provider for the underlying billing or other reasons.
    /// Allow the tool to check if they are compatible, or should be filtered out.
    fn supported_provider(&self, _provider: &LanguageModelProviderId) -> bool {
        true
    }

    /// Runs the tool with the provided input.
    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>>;

    /// Emits events for a previous execution of the tool.
    fn replay(
        &self,
        _input: Self::Input,
        _output: Self::Output,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Result<()> {
        Ok(())
    }

    fn erase(self) -> Arc<dyn AnyAgentTool> {
        Arc::new(Erased(Arc::new(self)))
    }
}

pub struct Erased<T>(T);

pub struct AgentToolOutput {
    pub llm_output: LanguageModelToolResultContent,
    pub raw_output: serde_json::Value,
}

pub trait AnyAgentTool {
    fn name(&self) -> SharedString;
    fn description(&self) -> SharedString;
    fn kind(&self) -> acp::ToolKind;
    fn initial_title(&self, input: serde_json::Value) -> SharedString;
    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value>;
    fn supported_provider(&self, _provider: &LanguageModelProviderId) -> bool {
        true
    }
    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<AgentToolOutput>>;
    fn replay(
        &self,
        input: serde_json::Value,
        output: serde_json::Value,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Result<()>;
}

impl<T> AnyAgentTool for Erased<Arc<T>>
where
    T: AgentTool,
{
    fn name(&self) -> SharedString {
        T::name().into()
    }

    fn description(&self) -> SharedString {
        self.0.description()
    }

    fn kind(&self) -> agent_client_protocol::ToolKind {
        T::kind()
    }

    fn initial_title(&self, input: serde_json::Value) -> SharedString {
        let parsed_input = serde_json::from_value(input.clone()).map_err(|_| input);
        self.0.initial_title(parsed_input)
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        let mut json = serde_json::to_value(self.0.input_schema(format))?;
        adapt_schema_to_format(&mut json, format)?;
        Ok(json)
    }

    fn supported_provider(&self, provider: &LanguageModelProviderId) -> bool {
        self.0.supported_provider(provider)
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<AgentToolOutput>> {
        cx.spawn(async move |cx| {
            let input = serde_json::from_value(input)?;
            let output = cx
                .update(|cx| self.0.clone().run(input, event_stream, cx))?
                .await?;
            let raw_output = serde_json::to_value(&output)?;
            Ok(AgentToolOutput {
                llm_output: output.into(),
                raw_output,
            })
        })
    }

    fn replay(
        &self,
        input: serde_json::Value,
        output: serde_json::Value,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Result<()> {
        let input = serde_json::from_value(input)?;
        let output = serde_json::from_value(output)?;
        self.0.replay(input, output, event_stream, cx)
    }
}

#[derive(Clone)]
struct ThreadEventStream(mpsc::UnboundedSender<Result<ThreadEvent>>);

impl ThreadEventStream {
    fn send_user_message(&self, message: &UserMessage) {
        self.0
            .unbounded_send(Ok(ThreadEvent::UserMessage(message.clone())))
            .ok();
    }

    fn send_text(&self, text: &str) {
        self.0
            .unbounded_send(Ok(ThreadEvent::AgentText(text.to_string())))
            .ok();
    }

    fn send_thinking(&self, text: &str) {
        self.0
            .unbounded_send(Ok(ThreadEvent::AgentThinking(text.to_string())))
            .ok();
    }

    fn send_tool_call(
        &self,
        id: &LanguageModelToolUseId,
        title: SharedString,
        kind: acp::ToolKind,
        input: serde_json::Value,
    ) {
        self.0
            .unbounded_send(Ok(ThreadEvent::ToolCall(Self::initial_tool_call(
                id,
                title.to_string(),
                kind,
                input,
            ))))
            .ok();
    }

    fn initial_tool_call(
        id: &LanguageModelToolUseId,
        title: String,
        kind: acp::ToolKind,
        input: serde_json::Value,
    ) -> acp::ToolCall {
        acp::ToolCall {
            id: acp::ToolCallId(id.to_string().into()),
            title,
            kind,
            status: acp::ToolCallStatus::Pending,
            content: vec![],
            locations: vec![],
            raw_input: Some(input),
            raw_output: None,
        }
    }

    fn update_tool_call_fields(
        &self,
        tool_use_id: &LanguageModelToolUseId,
        fields: acp::ToolCallUpdateFields,
    ) {
        self.0
            .unbounded_send(Ok(ThreadEvent::ToolCallUpdate(
                acp::ToolCallUpdate {
                    id: acp::ToolCallId(tool_use_id.to_string().into()),
                    fields,
                }
                .into(),
            )))
            .ok();
    }

    fn send_retry(&self, status: acp_thread::RetryStatus) {
        self.0.unbounded_send(Ok(ThreadEvent::Retry(status))).ok();
    }

    fn send_stop(&self, reason: acp::StopReason) {
        self.0.unbounded_send(Ok(ThreadEvent::Stop(reason))).ok();
    }

    fn send_canceled(&self) {
        self.0
            .unbounded_send(Ok(ThreadEvent::Stop(acp::StopReason::Cancelled)))
            .ok();
    }

    fn send_error(&self, error: impl Into<anyhow::Error>) {
        self.0.unbounded_send(Err(error.into())).ok();
    }
}

#[derive(Clone)]
pub struct ToolCallEventStream {
    tool_use_id: LanguageModelToolUseId,
    stream: ThreadEventStream,
    fs: Option<Arc<dyn Fs>>,
}

impl ToolCallEventStream {
    #[cfg(test)]
    pub fn test() -> (Self, ToolCallEventStreamReceiver) {
        let (events_tx, events_rx) = mpsc::unbounded::<Result<ThreadEvent>>();

        let stream = ToolCallEventStream::new("test_id".into(), ThreadEventStream(events_tx), None);

        (stream, ToolCallEventStreamReceiver(events_rx))
    }

    fn new(
        tool_use_id: LanguageModelToolUseId,
        stream: ThreadEventStream,
        fs: Option<Arc<dyn Fs>>,
    ) -> Self {
        Self {
            tool_use_id,
            stream,
            fs,
        }
    }

    pub fn update_fields(&self, fields: acp::ToolCallUpdateFields) {
        self.stream
            .update_tool_call_fields(&self.tool_use_id, fields);
    }

    pub fn update_diff(&self, diff: Entity<acp_thread::Diff>) {
        self.stream
            .0
            .unbounded_send(Ok(ThreadEvent::ToolCallUpdate(
                acp_thread::ToolCallUpdateDiff {
                    id: acp::ToolCallId(self.tool_use_id.to_string().into()),
                    diff,
                }
                .into(),
            )))
            .ok();
    }

    pub fn update_terminal(&self, terminal: Entity<acp_thread::Terminal>) {
        self.stream
            .0
            .unbounded_send(Ok(ThreadEvent::ToolCallUpdate(
                acp_thread::ToolCallUpdateTerminal {
                    id: acp::ToolCallId(self.tool_use_id.to_string().into()),
                    terminal,
                }
                .into(),
            )))
            .ok();
    }

    pub fn authorize(&self, title: impl Into<String>, cx: &mut App) -> Task<Result<()>> {
        if agent_settings::AgentSettings::get_global(cx).always_allow_tool_actions {
            return Task::ready(Ok(()));
        }

        let (response_tx, response_rx) = oneshot::channel();
        self.stream
            .0
            .unbounded_send(Ok(ThreadEvent::ToolCallAuthorization(
                ToolCallAuthorization {
                    tool_call: acp::ToolCallUpdate {
                        id: acp::ToolCallId(self.tool_use_id.to_string().into()),
                        fields: acp::ToolCallUpdateFields {
                            title: Some(title.into()),
                            ..Default::default()
                        },
                    },
                    options: vec![
                        acp::PermissionOption {
                            id: acp::PermissionOptionId("always_allow".into()),
                            name: "Always Allow".into(),
                            kind: acp::PermissionOptionKind::AllowAlways,
                        },
                        acp::PermissionOption {
                            id: acp::PermissionOptionId("allow".into()),
                            name: "Allow".into(),
                            kind: acp::PermissionOptionKind::AllowOnce,
                        },
                        acp::PermissionOption {
                            id: acp::PermissionOptionId("deny".into()),
                            name: "Deny".into(),
                            kind: acp::PermissionOptionKind::RejectOnce,
                        },
                    ],
                    response: response_tx,
                },
            )))
            .ok();
        let fs = self.fs.clone();
        cx.spawn(async move |cx| match response_rx.await?.0.as_ref() {
            "always_allow" => {
                if let Some(fs) = fs.clone() {
                    cx.update(|cx| {
                        update_settings_file::<AgentSettings>(fs, cx, |settings, _| {
                            settings.set_always_allow_tool_actions(true);
                        });
                    })?;
                }

                Ok(())
            }
            "allow" => Ok(()),
            _ => Err(anyhow!("Permission to run tool denied by user")),
        })
    }
}

#[cfg(test)]
pub struct ToolCallEventStreamReceiver(mpsc::UnboundedReceiver<Result<ThreadEvent>>);

#[cfg(test)]
impl ToolCallEventStreamReceiver {
    pub async fn expect_authorization(&mut self) -> ToolCallAuthorization {
        let event = self.0.next().await;
        if let Some(Ok(ThreadEvent::ToolCallAuthorization(auth))) = event {
            auth
        } else {
            panic!("Expected ToolCallAuthorization but got: {:?}", event);
        }
    }

    pub async fn expect_terminal(&mut self) -> Entity<acp_thread::Terminal> {
        let event = self.0.next().await;
        if let Some(Ok(ThreadEvent::ToolCallUpdate(acp_thread::ToolCallUpdate::UpdateTerminal(
            update,
        )))) = event
        {
            update.terminal
        } else {
            panic!("Expected terminal but got: {:?}", event);
        }
    }
}

#[cfg(test)]
impl std::ops::Deref for ToolCallEventStreamReceiver {
    type Target = mpsc::UnboundedReceiver<Result<ThreadEvent>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
impl std::ops::DerefMut for ToolCallEventStreamReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&str> for UserMessageContent {
    fn from(text: &str) -> Self {
        Self::Text(text.into())
    }
}

impl From<acp::ContentBlock> for UserMessageContent {
    fn from(value: acp::ContentBlock) -> Self {
        match value {
            acp::ContentBlock::Text(text_content) => Self::Text(text_content.text),
            acp::ContentBlock::Image(image_content) => Self::Image(convert_image(image_content)),
            acp::ContentBlock::Audio(_) => {
                // TODO
                Self::Text("[audio]".to_string())
            }
            acp::ContentBlock::ResourceLink(resource_link) => {
                match MentionUri::parse(&resource_link.uri) {
                    Ok(uri) => Self::Mention {
                        uri,
                        content: String::new(),
                    },
                    Err(err) => {
                        log::error!("Failed to parse mention link: {}", err);
                        Self::Text(format!("[{}]({})", resource_link.name, resource_link.uri))
                    }
                }
            }
            acp::ContentBlock::Resource(resource) => match resource.resource {
                acp::EmbeddedResourceResource::TextResourceContents(resource) => {
                    match MentionUri::parse(&resource.uri) {
                        Ok(uri) => Self::Mention {
                            uri,
                            content: resource.text,
                        },
                        Err(err) => {
                            log::error!("Failed to parse mention link: {}", err);
                            Self::Text(
                                MarkdownCodeBlock {
                                    tag: &resource.uri,
                                    text: &resource.text,
                                }
                                .to_string(),
                            )
                        }
                    }
                }
                acp::EmbeddedResourceResource::BlobResourceContents(_) => {
                    // TODO
                    Self::Text("[blob]".to_string())
                }
            },
        }
    }
}

impl From<UserMessageContent> for acp::ContentBlock {
    fn from(content: UserMessageContent) -> Self {
        match content {
            UserMessageContent::Text(text) => acp::ContentBlock::Text(acp::TextContent {
                text,
                annotations: None,
            }),
            UserMessageContent::Image(image) => acp::ContentBlock::Image(acp::ImageContent {
                data: image.source.to_string(),
                mime_type: "image/png".to_string(),
                annotations: None,
                uri: None,
            }),
            UserMessageContent::Mention { uri, content } => {
                acp::ContentBlock::Resource(acp::EmbeddedResource {
                    resource: acp::EmbeddedResourceResource::TextResourceContents(
                        acp::TextResourceContents {
                            mime_type: None,
                            text: content,
                            uri: uri.to_uri().to_string(),
                        },
                    ),
                    annotations: None,
                })
            }
        }
    }
}

fn convert_image(image_content: acp::ImageContent) -> LanguageModelImage {
    LanguageModelImage {
        source: image_content.data.into(),
        // TODO: make this optional?
        size: gpui::Size::new(0.into(), 0.into()),
    }
}
