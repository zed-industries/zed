use crate::{
    ContextServerRegistry, CopyPathTool, CreateDirectoryTool, DbLanguageModel, DbThread,
    DeletePathTool, DiagnosticsTool, EditFileTool, FetchTool, FindPathTool, GrepTool,
    ListDirectoryTool, MovePathTool, NowTool, OpenTool, ProjectSnapshot, ReadFileTool,
    RestoreFileFromDiskTool, SaveFileTool, SubagentTool, SystemPromptTemplate, Template, Templates,
    TerminalTool, ThinkingTool, ToolPermissionDecision, WebSearchTool,
    decide_permission_from_settings,
};
use acp_thread::{MentionUri, UserMessageId};
use action_log::ActionLog;
use feature_flags::{FeatureFlagAppExt as _, SubagentsFeatureFlag};

use agent_client_protocol as acp;
use agent_settings::{
    AgentProfileId, AgentProfileSettings, AgentSettings, SUMMARIZE_THREAD_DETAILED_PROMPT,
    SUMMARIZE_THREAD_PROMPT,
};
use anyhow::{Context as _, Result, anyhow};
use chrono::{DateTime, Utc};
use client::UserStore;
use cloud_llm_client::{CompletionIntent, Plan};
use collections::{HashMap, HashSet, IndexMap};
use fs::Fs;
use futures::stream;
use futures::{
    FutureExt,
    channel::{mpsc, oneshot},
    future::Shared,
    stream::FuturesUnordered,
};
use gpui::{
    App, AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task, WeakEntity,
};
use language::Buffer;
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelId,
    LanguageModelImage, LanguageModelProviderId, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelRequestTool, LanguageModelToolResult,
    LanguageModelToolResultContent, LanguageModelToolSchemaFormat, LanguageModelToolUse,
    LanguageModelToolUseId, Role, SelectedModel, StopReason, TokenUsage, ZED_CLOUD_PROVIDER_ID,
};
use project::Project;
use prompt_store::ProjectContext;
use schemars::{JsonSchema, Schema};
use serde::{Deserialize, Serialize};
use settings::{LanguageModelSelection, Settings, ToolPermissionMode, update_settings_file};
use smol::stream::StreamExt;
use std::{
    collections::BTreeMap,
    ops::RangeInclusive,
    path::Path,
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant},
};
use std::{fmt::Write, path::PathBuf};
use util::{ResultExt, debug_panic, markdown::MarkdownCodeBlock, paths::PathStyle};
use uuid::Uuid;

const TOOL_CANCELED_MESSAGE: &str = "Tool canceled by user";
pub const MAX_TOOL_NAME_LENGTH: usize = 64;
pub const MAX_SUBAGENT_DEPTH: u8 = 4;
pub const MAX_PARALLEL_SUBAGENTS: usize = 8;

/// Context passed to a subagent thread for lifecycle management
#[derive(Clone)]
pub struct SubagentContext {
    /// ID of the parent thread
    pub parent_thread_id: acp::SessionId,

    /// ID of the tool call that spawned this subagent
    pub tool_use_id: LanguageModelToolUseId,

    /// Current depth level (0 = root agent, 1 = first-level subagent, etc.)
    pub depth: u8,

    /// Prompt to send when subagent completes successfully
    pub summary_prompt: String,

    /// Prompt to send when context is running low (≤25% remaining)
    pub context_low_prompt: String,
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
            Message::User(message) => {
                if message.content.is_empty() {
                    vec![]
                } else {
                    vec![message.to_request()]
                }
            }
            Message::Agent(message) => message.to_request(),
            Message::Resume => vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["Continue where you left off".into()],
                cache: false,
                reasoning_details: None,
            }],
        }
    }

    pub fn to_markdown(&self) -> String {
        match self {
            Message::User(message) => message.to_markdown(),
            Message::Agent(message) => message.to_markdown(),
            Message::Resume => "[resume]\n".into(),
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
            reasoning_details: None,
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
        const OPEN_DIAGNOSTICS_TAG: &str = "<diagnostics>";

        let mut file_context = OPEN_FILES_TAG.to_string();
        let mut directory_context = OPEN_DIRECTORIES_TAG.to_string();
        let mut symbol_context = OPEN_SYMBOLS_TAG.to_string();
        let mut selection_context = OPEN_SELECTIONS_TAG.to_string();
        let mut thread_context = OPEN_THREADS_TAG.to_string();
        let mut fetch_context = OPEN_FETCH_TAG.to_string();
        let mut rules_context = OPEN_RULES_TAG.to_string();
        let mut diagnostics_context = OPEN_DIAGNOSTICS_TAG.to_string();

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
                        MentionUri::Diagnostics { .. } => {
                            write!(&mut diagnostics_context, "\n{}\n", content).ok();
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

        if diagnostics_context.len() > OPEN_DIAGNOSTICS_TAG.len() {
            diagnostics_context.push_str("</diagnostics>\n");
            message
                .content
                .push(language_model::MessageContent::Text(diagnostics_context));
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
            reasoning_details: self.reasoning_details.clone(),
        };
        for chunk in &self.content {
            match chunk {
                AgentMessageContent::Text(text) => {
                    assistant_message
                        .content
                        .push(language_model::MessageContent::Text(text.clone()));
                }
                AgentMessageContent::Thinking { text, signature } => {
                    assistant_message
                        .content
                        .push(language_model::MessageContent::Thinking {
                            text: text.clone(),
                            signature: signature.clone(),
                        });
                }
                AgentMessageContent::RedactedThinking(value) => {
                    assistant_message.content.push(
                        language_model::MessageContent::RedactedThinking(value.clone()),
                    );
                }
                AgentMessageContent::ToolUse(tool_use) => {
                    if self.tool_results.contains_key(&tool_use.id) {
                        assistant_message
                            .content
                            .push(language_model::MessageContent::ToolUse(tool_use.clone()));
                    }
                }
            };
        }

        let mut user_message = LanguageModelRequestMessage {
            role: Role::User,
            content: Vec::new(),
            cache: false,
            reasoning_details: None,
        };

        for tool_result in self.tool_results.values() {
            let mut tool_result = tool_result.clone();
            // Surprisingly, the API fails if we return an empty string here.
            // It thinks we are sending a tool use without a tool result.
            if tool_result.content.is_empty() {
                tool_result.content = "<Tool returned an empty string>".into();
            }
            user_message
                .content
                .push(language_model::MessageContent::ToolResult(tool_result));
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
    pub reasoning_details: Option<serde_json::Value>,
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

pub trait TerminalHandle {
    fn id(&self, cx: &AsyncApp) -> Result<acp::TerminalId>;
    fn current_output(&self, cx: &AsyncApp) -> Result<acp::TerminalOutputResponse>;
    fn wait_for_exit(&self, cx: &AsyncApp) -> Result<Shared<Task<acp::TerminalExitStatus>>>;
    fn kill(&self, cx: &AsyncApp) -> Result<()>;
    fn was_stopped_by_user(&self, cx: &AsyncApp) -> Result<bool>;
}

pub trait ThreadEnvironment {
    fn create_terminal(
        &self,
        command: String,
        cwd: Option<PathBuf>,
        output_byte_limit: Option<u64>,
        cx: &mut AsyncApp,
    ) -> Task<Result<Rc<dyn TerminalHandle>>>;
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
pub struct NewTerminal {
    pub command: String,
    pub output_byte_limit: Option<u64>,
    pub cwd: Option<PathBuf>,
    pub response: oneshot::Sender<Result<Entity<acp_thread::Terminal>>>,
}

#[derive(Debug, Clone)]
pub struct ToolPermissionContext {
    pub tool_name: String,
    pub input_value: String,
}

impl ToolPermissionContext {
    pub fn new(tool_name: impl Into<String>, input_value: impl Into<String>) -> Self {
        Self {
            tool_name: tool_name.into(),
            input_value: input_value.into(),
        }
    }

    /// Builds the permission options for this tool context.
    ///
    /// This is the canonical source for permission option generation.
    /// Tests should use this function rather than manually constructing options.
    pub fn build_permission_options(&self) -> acp_thread::PermissionOptions {
        use crate::pattern_extraction::*;

        let tool_name = &self.tool_name;
        let input_value = &self.input_value;

        let (pattern, pattern_display) = match tool_name.as_str() {
            "terminal" => (
                extract_terminal_pattern(input_value),
                extract_terminal_pattern_display(input_value),
            ),
            "edit_file" | "delete_path" | "move_path" | "create_directory" | "save_file" => (
                extract_path_pattern(input_value),
                extract_path_pattern_display(input_value),
            ),
            "fetch" => (
                extract_url_pattern(input_value),
                extract_url_pattern_display(input_value),
            ),
            _ => (None, None),
        };

        let mut choices = Vec::new();

        let mut push_choice = |label: String, allow_id, deny_id, allow_kind, deny_kind| {
            choices.push(acp_thread::PermissionOptionChoice {
                allow: acp::PermissionOption::new(
                    acp::PermissionOptionId::new(allow_id),
                    label.clone(),
                    allow_kind,
                ),
                deny: acp::PermissionOption::new(
                    acp::PermissionOptionId::new(deny_id),
                    label,
                    deny_kind,
                ),
            });
        };

        push_choice(
            format!("Always for {}", tool_name.replace('_', " ")),
            format!("always_allow:{}", tool_name),
            format!("always_deny:{}", tool_name),
            acp::PermissionOptionKind::AllowAlways,
            acp::PermissionOptionKind::RejectAlways,
        );

        if let (Some(pattern), Some(display)) = (pattern, pattern_display) {
            let button_text = match tool_name.as_str() {
                "terminal" => format!("Always for `{}` commands", display),
                "fetch" => format!("Always for `{}`", display),
                _ => format!("Always for `{}`", display),
            };
            push_choice(
                button_text,
                format!("always_allow_pattern:{}:{}", tool_name, pattern),
                format!("always_deny_pattern:{}:{}", tool_name, pattern),
                acp::PermissionOptionKind::AllowAlways,
                acp::PermissionOptionKind::RejectAlways,
            );
        }

        push_choice(
            "Only this time".to_string(),
            "allow".to_string(),
            "deny".to_string(),
            acp::PermissionOptionKind::AllowOnce,
            acp::PermissionOptionKind::RejectOnce,
        );

        acp_thread::PermissionOptions::Dropdown(choices)
    }
}

#[derive(Debug)]
pub struct ToolCallAuthorization {
    pub tool_call: acp::ToolCallUpdate,
    pub options: acp_thread::PermissionOptions,
    pub response: oneshot::Sender<acp::PermissionOptionId>,
    pub context: Option<ToolPermissionContext>,
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

pub struct QueuedMessage {
    pub content: Vec<acp::ContentBlock>,
    pub tracked_buffers: Vec<Entity<Buffer>>,
}

pub struct Thread {
    id: acp::SessionId,
    prompt_id: PromptId,
    updated_at: DateTime<Utc>,
    title: Option<SharedString>,
    pending_title_generation: Option<Task<()>>,
    pending_summary_generation: Option<Shared<Task<Option<SharedString>>>>,
    summary: Option<SharedString>,
    messages: Vec<Message>,
    user_store: Entity<UserStore>,
    /// Holds the task that handles agent interaction until the end of the turn.
    /// Survives across multiple requests as the model performs tool calls and
    /// we run tools, report their results.
    running_turn: Option<RunningTurn>,
    queued_messages: Vec<QueuedMessage>,
    pending_message: Option<AgentMessage>,
    tools: BTreeMap<SharedString, Arc<dyn AnyAgentTool>>,
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
    prompt_capabilities_tx: watch::Sender<acp::PromptCapabilities>,
    pub(crate) prompt_capabilities_rx: watch::Receiver<acp::PromptCapabilities>,
    pub(crate) project: Entity<Project>,
    pub(crate) action_log: Entity<ActionLog>,
    /// Tracks the last time files were read by the agent, to detect external modifications
    pub(crate) file_read_times: HashMap<PathBuf, fs::MTime>,
    /// True if this thread was imported from a shared thread and can be synced.
    imported: bool,
    /// If this is a subagent thread, contains context about the parent
    subagent_context: Option<SubagentContext>,
    /// Weak references to running subagent threads for cancellation propagation
    running_subagents: Vec<WeakEntity<Thread>>,
}

impl Thread {
    fn prompt_capabilities(model: Option<&dyn LanguageModel>) -> acp::PromptCapabilities {
        let image = model.map_or(true, |model| model.supports_images());
        acp::PromptCapabilities::new()
            .image(image)
            .embedded_context(true)
    }

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
        let (prompt_capabilities_tx, prompt_capabilities_rx) =
            watch::channel(Self::prompt_capabilities(model.as_deref()));
        Self {
            id: acp::SessionId::new(uuid::Uuid::new_v4().to_string()),
            prompt_id: PromptId::new(),
            updated_at: Utc::now(),
            title: None,
            pending_title_generation: None,
            pending_summary_generation: None,
            summary: None,
            messages: Vec::new(),
            user_store: project.read(cx).user_store(),
            running_turn: None,
            queued_messages: Vec::new(),
            pending_message: None,
            tools: BTreeMap::default(),
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
            prompt_capabilities_tx,
            prompt_capabilities_rx,
            project,
            action_log,
            file_read_times: HashMap::default(),
            imported: false,
            subagent_context: None,
            running_subagents: Vec::new(),
        }
    }

    pub fn new_subagent(
        project: Entity<Project>,
        project_context: Entity<ProjectContext>,
        context_server_registry: Entity<ContextServerRegistry>,
        templates: Arc<Templates>,
        model: Arc<dyn LanguageModel>,
        subagent_context: SubagentContext,
        parent_tools: BTreeMap<SharedString, Arc<dyn AnyAgentTool>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let profile_id = AgentSettings::get_global(cx).default_profile.clone();
        let action_log = cx.new(|_cx| ActionLog::new(project.clone()));
        let (prompt_capabilities_tx, prompt_capabilities_rx) =
            watch::channel(Self::prompt_capabilities(Some(model.as_ref())));
        Self {
            id: acp::SessionId::new(uuid::Uuid::new_v4().to_string()),
            prompt_id: PromptId::new(),
            updated_at: Utc::now(),
            title: None,
            pending_title_generation: None,
            pending_summary_generation: None,
            summary: None,
            messages: Vec::new(),
            user_store: project.read(cx).user_store(),
            running_turn: None,
            queued_messages: Vec::new(),
            pending_message: None,
            tools: parent_tools,
            request_token_usage: HashMap::default(),
            cumulative_token_usage: TokenUsage::default(),
            initial_project_snapshot: Task::ready(None).shared(),
            context_server_registry,
            profile_id,
            project_context,
            templates,
            model: Some(model),
            summarization_model: None,
            prompt_capabilities_tx,
            prompt_capabilities_rx,
            project,
            action_log,
            file_read_times: HashMap::default(),
            imported: false,
            subagent_context: Some(subagent_context),
            running_subagents: Vec::new(),
        }
    }

    pub fn id(&self) -> &acp::SessionId {
        &self.id
    }

    /// Returns true if this thread was imported from a shared thread.
    pub fn is_imported(&self) -> bool {
        self.imported
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
                .unbounded_send(Ok(ThreadEvent::ToolCall(
                    acp::ToolCall::new(tool_use.id.to_string(), tool_use.name.to_string())
                        .status(acp::ToolCallStatus::Failed)
                        .raw_input(tool_use.input.clone()),
                )))
                .ok();
            return;
        };

        let title = tool.initial_title(tool_use.input.clone(), cx);
        let kind = tool.kind();
        stream.send_tool_call(
            &tool_use.id,
            &tool_use.name,
            title,
            kind,
            tool_use.input.clone(),
        );

        let output = tool_result
            .as_ref()
            .and_then(|result| result.output.clone());
        if let Some(output) = output.clone() {
            // For replay, we use a dummy cancellation receiver since the tool already completed
            let (_cancellation_tx, cancellation_rx) = watch::channel(false);
            let tool_event_stream = ToolCallEventStream::new(
                tool_use.id.clone(),
                stream.clone(),
                Some(self.project.read(cx).fs().clone()),
                cancellation_rx,
            );
            tool.replay(tool_use.input.clone(), output, tool_event_stream, cx)
                .log_err();
        }

        stream.update_tool_call_fields(
            &tool_use.id,
            acp::ToolCallUpdateFields::new()
                .status(
                    tool_result
                        .as_ref()
                        .map_or(acp::ToolCallStatus::Failed, |result| {
                            if result.is_error {
                                acp::ToolCallStatus::Failed
                            } else {
                                acp::ToolCallStatus::Completed
                            }
                        }),
                )
                .raw_output(output),
        );
    }

    pub fn from_db(
        id: acp::SessionId,
        db_thread: DbThread,
        project: Entity<Project>,
        project_context: Entity<ProjectContext>,
        context_server_registry: Entity<ContextServerRegistry>,
        templates: Arc<Templates>,
        cx: &mut Context<Self>,
    ) -> Self {
        let profile_id = db_thread
            .profile
            .unwrap_or_else(|| AgentSettings::get_global(cx).default_profile.clone());

        let mut model = LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
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

        if model.is_none() {
            model = Self::resolve_profile_model(&profile_id, cx);
        }
        if model.is_none() {
            model = LanguageModelRegistry::global(cx).update(cx, |registry, _cx| {
                registry.default_model().map(|model| model.model)
            });
        }

        let (prompt_capabilities_tx, prompt_capabilities_rx) =
            watch::channel(Self::prompt_capabilities(model.as_deref()));

        let action_log = cx.new(|_| ActionLog::new(project.clone()));

        Self {
            id,
            prompt_id: PromptId::new(),
            title: if db_thread.title.is_empty() {
                None
            } else {
                Some(db_thread.title.clone())
            },
            pending_title_generation: None,
            pending_summary_generation: None,
            summary: db_thread.detailed_summary,
            messages: db_thread.messages,
            user_store: project.read(cx).user_store(),
            running_turn: None,
            queued_messages: Vec::new(),
            pending_message: None,
            tools: BTreeMap::default(),
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
            prompt_capabilities_tx,
            prompt_capabilities_rx,
            file_read_times: HashMap::default(),
            imported: db_thread.imported,
            subagent_context: None,
            running_subagents: Vec::new(),
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
            profile: Some(self.profile_id.clone()),
            imported: self.imported,
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
    ) -> Task<Arc<ProjectSnapshot>> {
        let task = project::telemetry_snapshot::TelemetrySnapshot::new(&project, cx);
        cx.spawn(async move |_, _| {
            let snapshot = task.await;

            Arc::new(ProjectSnapshot {
                worktree_snapshots: snapshot.worktree_snapshots,
                timestamp: Utc::now(),
            })
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
        let new_caps = Self::prompt_capabilities(self.model.as_deref());
        let new_usage = self.latest_token_usage();
        if old_usage != new_usage {
            cx.emit(TokenUsageUpdated(new_usage));
        }
        self.prompt_capabilities_tx.send(new_caps).log_err();
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

    pub fn last_message(&self) -> Option<Message> {
        if let Some(message) = self.pending_message.clone() {
            Some(Message::Agent(message))
        } else {
            self.messages.last().cloned()
        }
    }

    pub fn add_default_tools(
        &mut self,
        environment: Rc<dyn ThreadEnvironment>,
        cx: &mut Context<Self>,
    ) {
        let language_registry = self.project.read(cx).languages().clone();
        self.add_tool(CopyPathTool::new(self.project.clone()));
        self.add_tool(CreateDirectoryTool::new(self.project.clone()));
        self.add_tool(DeletePathTool::new(
            self.project.clone(),
            self.action_log.clone(),
        ));
        self.add_tool(DiagnosticsTool::new(self.project.clone()));
        self.add_tool(EditFileTool::new(
            self.project.clone(),
            cx.weak_entity(),
            language_registry,
            Templates::new(),
        ));
        self.add_tool(FetchTool::new(self.project.read(cx).client().http_client()));
        self.add_tool(FindPathTool::new(self.project.clone()));
        self.add_tool(GrepTool::new(self.project.clone()));
        self.add_tool(ListDirectoryTool::new(self.project.clone()));
        self.add_tool(MovePathTool::new(self.project.clone()));
        self.add_tool(NowTool);
        self.add_tool(OpenTool::new(self.project.clone()));
        self.add_tool(ReadFileTool::new(
            cx.weak_entity(),
            self.project.clone(),
            self.action_log.clone(),
        ));
        self.add_tool(SaveFileTool::new(self.project.clone()));
        self.add_tool(RestoreFileFromDiskTool::new(self.project.clone()));
        self.add_tool(TerminalTool::new(self.project.clone(), environment));
        self.add_tool(ThinkingTool);
        self.add_tool(WebSearchTool);

        if cx.has_flag::<SubagentsFeatureFlag>() && self.depth() < MAX_SUBAGENT_DEPTH {
            let parent_tools = self.tools.clone();
            self.add_tool(SubagentTool::new(
                cx.weak_entity(),
                self.project.clone(),
                self.project_context.clone(),
                self.context_server_registry.clone(),
                self.templates.clone(),
                self.depth(),
                parent_tools,
            ));
        }
    }

    pub fn add_tool<T: AgentTool>(&mut self, tool: T) {
        self.tools.insert(T::name().into(), tool.erase());
    }

    pub fn remove_tool(&mut self, name: &str) -> bool {
        self.tools.remove(name).is_some()
    }

    pub fn restrict_tools(&mut self, allowed: &collections::HashSet<SharedString>) {
        self.tools.retain(|name, _| allowed.contains(name));
    }

    pub fn profile(&self) -> &AgentProfileId {
        &self.profile_id
    }

    pub fn set_profile(&mut self, profile_id: AgentProfileId, cx: &mut Context<Self>) {
        if self.profile_id == profile_id {
            return;
        }

        self.profile_id = profile_id;

        // Swap to the profile's preferred model when available.
        if let Some(model) = Self::resolve_profile_model(&self.profile_id, cx) {
            self.set_model(model, cx);
        }
    }

    pub fn cancel(&mut self, cx: &mut Context<Self>) -> Task<()> {
        for subagent in self.running_subagents.drain(..) {
            if let Some(subagent) = subagent.upgrade() {
                subagent.update(cx, |thread, cx| thread.cancel(cx)).detach();
            }
        }

        let Some(running_turn) = self.running_turn.take() else {
            self.flush_pending_message(cx);
            return Task::ready(());
        };

        let turn_task = running_turn.cancel();

        cx.spawn(async move |this, cx| {
            turn_task.await;
            this.update(cx, |this, cx| {
                this.flush_pending_message(cx);
            })
            .ok();
        })
    }

    pub fn queue_message(
        &mut self,
        content: Vec<acp::ContentBlock>,
        tracked_buffers: Vec<Entity<Buffer>>,
    ) {
        self.queued_messages.push(QueuedMessage {
            content,
            tracked_buffers,
        });
    }

    pub fn queued_messages(&self) -> &[QueuedMessage] {
        &self.queued_messages
    }

    pub fn remove_queued_message(&mut self, index: usize) -> Option<QueuedMessage> {
        if index < self.queued_messages.len() {
            Some(self.queued_messages.remove(index))
        } else {
            None
        }
    }

    pub fn clear_queued_messages(&mut self) {
        self.queued_messages.clear();
    }

    fn has_queued_messages(&self) -> bool {
        !self.queued_messages.is_empty()
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
        self.cancel(cx).detach();
        // Clear pending message since cancel will try to flush it asynchronously,
        // and we don't want that content to be added after we truncate
        self.pending_message.take();
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
        self.clear_summary();
        cx.notify();
        Ok(())
    }

    pub fn latest_request_token_usage(&self) -> Option<language_model::TokenUsage> {
        let last_user_message = self.last_user_message()?;
        let tokens = self.request_token_usage.get(&last_user_message.id)?;
        Some(*tokens)
    }

    pub fn latest_token_usage(&self) -> Option<acp_thread::TokenUsage> {
        let usage = self.latest_request_token_usage()?;
        let model = self.model.clone()?;
        Some(acp_thread::TokenUsage {
            max_tokens: model.max_token_count(),
            used_tokens: usage.total_tokens(),
            input_tokens: usage.input_tokens,
            output_tokens: usage.output_tokens,
        })
    }

    /// Get the total input token count as of the message before the given message.
    ///
    /// Returns `None` if:
    /// - `target_id` is the first message (no previous message)
    /// - The previous message hasn't received a response yet (no usage data)
    /// - `target_id` is not found in the messages
    pub fn tokens_before_message(&self, target_id: &UserMessageId) -> Option<u64> {
        let mut previous_user_message_id: Option<&UserMessageId> = None;

        for message in &self.messages {
            if let Message::User(user_msg) = message {
                if &user_msg.id == target_id {
                    let prev_id = previous_user_message_id?;
                    let usage = self.request_token_usage.get(prev_id)?;
                    return Some(usage.input_tokens);
                }
                previous_user_message_id = Some(&user_msg.id);
            }
        }
        None
    }

    /// Look up the active profile and resolve its preferred model if one is configured.
    fn resolve_profile_model(
        profile_id: &AgentProfileId,
        cx: &mut Context<Self>,
    ) -> Option<Arc<dyn LanguageModel>> {
        let selection = AgentSettings::get_global(cx)
            .profiles
            .get(profile_id)?
            .default_model
            .clone()?;
        Self::resolve_model_from_selection(&selection, cx)
    }

    /// Translate a stored model selection into the configured model from the registry.
    fn resolve_model_from_selection(
        selection: &LanguageModelSelection,
        cx: &mut Context<Self>,
    ) -> Option<Arc<dyn LanguageModel>> {
        let selected = SelectedModel {
            provider: LanguageModelProviderId::from(selection.provider.0.clone()),
            model: LanguageModelId::from(selection.model.clone()),
        };
        LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            registry
                .select_model(&selected, cx)
                .map(|configured| configured.model)
        })
    }

    pub fn resume(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Result<mpsc::UnboundedReceiver<Result<ThreadEvent>>> {
        self.messages.push(Message::Resume);
        cx.notify();

        log::debug!("Total messages in thread: {}", self.messages.len());
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
        let content = content.into_iter().map(Into::into).collect::<Vec<_>>();
        log::debug!("Thread::send content: {:?}", content);

        self.messages
            .push(Message::User(UserMessage { id, content }));
        cx.notify();

        self.send_existing(cx)
    }

    pub fn send_existing(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Result<mpsc::UnboundedReceiver<Result<ThreadEvent>>> {
        let model = self.model().context("No language model configured")?;

        log::info!("Thread::send called with model: {}", model.name().0);
        self.advance_prompt_id();

        log::debug!("Total messages in thread: {}", self.messages.len());
        self.run_turn(cx)
    }

    pub fn push_acp_user_block(
        &mut self,
        id: UserMessageId,
        blocks: impl IntoIterator<Item = acp::ContentBlock>,
        path_style: PathStyle,
        cx: &mut Context<Self>,
    ) {
        let content = blocks
            .into_iter()
            .map(|block| UserMessageContent::from_content_block(block, path_style))
            .collect::<Vec<_>>();
        self.messages
            .push(Message::User(UserMessage { id, content }));
        cx.notify();
    }

    pub fn push_acp_agent_block(&mut self, block: acp::ContentBlock, cx: &mut Context<Self>) {
        let text = match block {
            acp::ContentBlock::Text(text_content) => text_content.text,
            acp::ContentBlock::Image(_) => "[image]".to_string(),
            acp::ContentBlock::Audio(_) => "[audio]".to_string(),
            acp::ContentBlock::ResourceLink(resource_link) => resource_link.uri,
            acp::ContentBlock::Resource(resource) => match resource.resource {
                acp::EmbeddedResourceResource::TextResourceContents(resource) => resource.uri,
                acp::EmbeddedResourceResource::BlobResourceContents(resource) => resource.uri,
                _ => "[resource]".to_string(),
            },
            _ => "[unknown]".to_string(),
        };

        self.messages.push(Message::Agent(AgentMessage {
            content: vec![AgentMessageContent::Text(text)],
            ..Default::default()
        }));
        cx.notify();
    }

    #[cfg(feature = "eval")]
    pub fn proceed(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Result<mpsc::UnboundedReceiver<Result<ThreadEvent>>> {
        self.run_turn(cx)
    }

    fn run_turn(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Result<mpsc::UnboundedReceiver<Result<ThreadEvent>>> {
        // Flush the old pending message synchronously before cancelling,
        // to avoid a race where the detached cancel task might flush the NEW
        // turn's pending message instead of the old one.
        self.flush_pending_message(cx);
        self.cancel(cx).detach();

        let model = self.model.clone().context("No language model configured")?;
        let profile = AgentSettings::get_global(cx)
            .profiles
            .get(&self.profile_id)
            .context("Profile not found")?;
        let (events_tx, events_rx) = mpsc::unbounded::<Result<ThreadEvent>>();
        let event_stream = ThreadEventStream(events_tx);
        let message_ix = self.messages.len().saturating_sub(1);
        self.clear_summary();
        let (cancellation_tx, mut cancellation_rx) = watch::channel(false);
        self.running_turn = Some(RunningTurn {
            event_stream: event_stream.clone(),
            tools: self.enabled_tools(profile, &model, cx),
            cancellation_tx,
            _task: cx.spawn(async move |this, cx| {
                log::debug!("Starting agent turn execution");

                let turn_result = Self::run_turn_internal(
                    &this,
                    model,
                    &event_stream,
                    cancellation_rx.clone(),
                    cx,
                )
                .await;

                // Check if we were cancelled - if so, cancel() already took running_turn
                // and we shouldn't touch it (it might be a NEW turn now)
                let was_cancelled = *cancellation_rx.borrow();
                if was_cancelled {
                    log::debug!("Turn was cancelled, skipping cleanup");
                    return;
                }

                _ = this.update(cx, |this, cx| this.flush_pending_message(cx));

                match turn_result {
                    Ok(()) => {
                        log::debug!("Turn execution completed");
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

    async fn run_turn_internal(
        this: &WeakEntity<Self>,
        model: Arc<dyn LanguageModel>,
        event_stream: &ThreadEventStream,
        mut cancellation_rx: watch::Receiver<bool>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let mut attempt = 0;
        let mut intent = CompletionIntent::UserPrompt;
        loop {
            let request =
                this.update(cx, |this, cx| this.build_completion_request(intent, cx))??;

            telemetry::event!(
                "Agent Thread Completion",
                thread_id = this.read_with(cx, |this, _| this.id.to_string())?,
                prompt_id = this.read_with(cx, |this, _| this.prompt_id.to_string())?,
                model = model.telemetry_id(),
                model_provider = model.provider_id().to_string(),
                attempt
            );

            log::debug!("Calling model.stream_completion, attempt {}", attempt);

            let (mut events, mut error) = match model.stream_completion(request, cx).await {
                Ok(events) => (events.fuse(), None),
                Err(err) => (stream::empty().boxed().fuse(), Some(err)),
            };
            let mut tool_results = FuturesUnordered::new();
            let mut cancelled = false;
            loop {
                // Race between getting the first event and cancellation
                let first_event = futures::select! {
                    event = events.next().fuse() => event,
                    _ = cancellation_rx.changed().fuse() => {
                        if *cancellation_rx.borrow() {
                            cancelled = true;
                            break;
                        }
                        continue;
                    }
                };
                let Some(first_event) = first_event else {
                    break;
                };

                // Collect all immediately available events to process as a batch
                let mut batch = vec![first_event];
                while let Some(event) = events.next().now_or_never().flatten() {
                    batch.push(event);
                }

                // Process the batch in a single update
                let batch_result = this.update(cx, |this, cx| {
                    let mut batch_tool_results = Vec::new();
                    let mut batch_error = None;

                    for event in batch {
                        log::trace!("Received completion event: {:?}", event);
                        match event {
                            Ok(event) => {
                                match this.handle_completion_event(
                                    event,
                                    event_stream,
                                    cancellation_rx.clone(),
                                    cx,
                                ) {
                                    Ok(Some(task)) => batch_tool_results.push(task),
                                    Ok(None) => {}
                                    Err(err) => {
                                        batch_error = Some(err);
                                        break;
                                    }
                                }
                            }
                            Err(err) => {
                                batch_error = Some(err.into());
                                break;
                            }
                        }
                    }

                    cx.notify();
                    (batch_tool_results, batch_error)
                })?;

                tool_results.extend(batch_result.0);
                if let Some(err) = batch_result.1 {
                    error = Some(err.downcast()?);
                    break;
                }
            }

            let end_turn = tool_results.is_empty();
            while let Some(tool_result) = tool_results.next().await {
                log::debug!("Tool finished {:?}", tool_result);

                event_stream.update_tool_call_fields(
                    &tool_result.tool_use_id,
                    acp::ToolCallUpdateFields::new()
                        .status(if tool_result.is_error {
                            acp::ToolCallStatus::Failed
                        } else {
                            acp::ToolCallStatus::Completed
                        })
                        .raw_output(tool_result.output.clone()),
                );
                this.update(cx, |this, _cx| {
                    this.pending_message()
                        .tool_results
                        .insert(tool_result.tool_use_id.clone(), tool_result);
                })?;
            }

            this.update(cx, |this, cx| {
                this.flush_pending_message(cx);
                if this.title.is_none() && this.pending_title_generation.is_none() {
                    this.generate_title(cx);
                }
            })?;

            if cancelled {
                log::debug!("Turn cancelled by user, exiting");
                return Ok(());
            }

            if let Some(error) = error {
                attempt += 1;
                let retry = this.update(cx, |this, cx| {
                    let user_store = this.user_store.read(cx);
                    this.handle_completion_error(error, attempt, user_store.plan())
                })??;
                let timer = cx.background_executor().timer(retry.duration);
                event_stream.send_retry(retry);
                timer.await;
                this.update(cx, |this, _cx| {
                    if let Some(Message::Agent(message)) = this.messages.last() {
                        if message.tool_results.is_empty() {
                            intent = CompletionIntent::UserPrompt;
                            this.messages.push(Message::Resume);
                        }
                    }
                })?;
            } else if end_turn {
                return Ok(());
            } else {
                let has_queued = this.update(cx, |this, _| this.has_queued_messages())?;
                if has_queued {
                    log::debug!("Queued message found, ending turn at message boundary");
                    return Ok(());
                }
                intent = CompletionIntent::ToolResults;
                attempt = 0;
            }
        }
    }

    fn handle_completion_error(
        &mut self,
        error: LanguageModelCompletionError,
        attempt: u8,
        plan: Option<Plan>,
    ) -> Result<acp_thread::RetryStatus> {
        let Some(model) = self.model.as_ref() else {
            return Err(anyhow!(error));
        };

        let auto_retry = if model.provider_id() == ZED_CLOUD_PROVIDER_ID {
            match plan {
                Some(Plan::V2(_)) => true,
                None => false,
            }
        } else {
            true
        };

        if !auto_retry {
            return Err(anyhow!(error));
        }

        let Some(strategy) = Self::retry_strategy_for(&error) else {
            return Err(anyhow!(error));
        };

        let max_attempts = match &strategy {
            RetryStrategy::ExponentialBackoff { max_attempts, .. } => *max_attempts,
            RetryStrategy::Fixed { max_attempts, .. } => *max_attempts,
        };

        if attempt > max_attempts {
            return Err(anyhow!(error));
        }

        let delay = match &strategy {
            RetryStrategy::ExponentialBackoff { initial_delay, .. } => {
                let delay_secs = initial_delay.as_secs() * 2u64.pow((attempt - 1) as u32);
                Duration::from_secs(delay_secs)
            }
            RetryStrategy::Fixed { delay, .. } => *delay,
        };
        log::debug!("Retry attempt {attempt} with delay {delay:?}");

        Ok(acp_thread::RetryStatus {
            last_error: error.to_string().into(),
            attempt: attempt as usize,
            max_attempts: max_attempts as usize,
            started_at: Instant::now(),
            duration: delay,
        })
    }

    /// A helper method that's called on every streamed completion event.
    /// Returns an optional tool result task, which the main agentic loop will
    /// send back to the model when it resolves.
    fn handle_completion_event(
        &mut self,
        event: LanguageModelCompletionEvent,
        event_stream: &ThreadEventStream,
        cancellation_rx: watch::Receiver<bool>,
        cx: &mut Context<Self>,
    ) -> Result<Option<Task<LanguageModelToolResult>>> {
        log::trace!("Handling streamed completion event: {:?}", event);
        use LanguageModelCompletionEvent::*;

        match event {
            StartMessage { .. } => {
                self.flush_pending_message(cx);
                self.pending_message = Some(AgentMessage::default());
            }
            Text(new_text) => self.handle_text_event(new_text, event_stream),
            Thinking { text, signature } => {
                self.handle_thinking_event(text, signature, event_stream)
            }
            RedactedThinking { data } => self.handle_redacted_thinking_event(data),
            ReasoningDetails(details) => {
                let last_message = self.pending_message();
                // Store the last non-empty reasoning_details (overwrites earlier ones)
                // This ensures we keep the encrypted reasoning with signatures, not the early text reasoning
                if let serde_json::Value::Array(ref arr) = details {
                    if !arr.is_empty() {
                        last_message.reasoning_details = Some(details);
                    }
                } else {
                    last_message.reasoning_details = Some(details);
                }
            }
            ToolUse(tool_use) => {
                return Ok(self.handle_tool_use_event(tool_use, event_stream, cancellation_rx, cx));
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
            Stop(StopReason::Refusal) => return Err(CompletionError::Refusal.into()),
            Stop(StopReason::MaxTokens) => return Err(CompletionError::MaxTokens.into()),
            Stop(StopReason::ToolUse | StopReason::EndTurn) => {}
            Started | Queued { .. } => {}
        }

        Ok(None)
    }

    fn handle_text_event(&mut self, new_text: String, event_stream: &ThreadEventStream) {
        event_stream.send_text(&new_text);

        let last_message = self.pending_message();
        if let Some(AgentMessageContent::Text(text)) = last_message.content.last_mut() {
            text.push_str(&new_text);
        } else {
            last_message
                .content
                .push(AgentMessageContent::Text(new_text));
        }
    }

    fn handle_thinking_event(
        &mut self,
        new_text: String,
        new_signature: Option<String>,
        event_stream: &ThreadEventStream,
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
    }

    fn handle_redacted_thinking_event(&mut self, data: String) {
        let last_message = self.pending_message();
        last_message
            .content
            .push(AgentMessageContent::RedactedThinking(data));
    }

    fn handle_tool_use_event(
        &mut self,
        tool_use: LanguageModelToolUse,
        event_stream: &ThreadEventStream,
        cancellation_rx: watch::Receiver<bool>,
        cx: &mut Context<Self>,
    ) -> Option<Task<LanguageModelToolResult>> {
        cx.notify();

        let tool = self.tool(tool_use.name.as_ref());
        let mut title = SharedString::from(&tool_use.name);
        let mut kind = acp::ToolKind::Other;
        if let Some(tool) = tool.as_ref() {
            title = tool.initial_title(tool_use.input.clone(), cx);
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
            event_stream.send_tool_call(
                &tool_use.id,
                &tool_use.name,
                title,
                kind,
                tool_use.input.clone(),
            );
            last_message
                .content
                .push(AgentMessageContent::ToolUse(tool_use.clone()));
        } else {
            event_stream.update_tool_call_fields(
                &tool_use.id,
                acp::ToolCallUpdateFields::new()
                    .title(title.as_str())
                    .kind(kind)
                    .raw_input(tool_use.input.clone()),
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
        let tool_event_stream = ToolCallEventStream::new(
            tool_use.id.clone(),
            event_stream.clone(),
            Some(fs),
            cancellation_rx,
        );
        tool_event_stream.update_fields(
            acp::ToolCallUpdateFields::new().status(acp::ToolCallStatus::InProgress),
        );
        let supports_images = self.model().is_some_and(|model| model.supports_images());
        let tool_result = tool.run(tool_use.input, tool_event_stream, cx);
        log::debug!("Running tool {}", tool_use.name);
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
                    output: Some(error.to_string().into()),
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

    pub fn title(&self) -> SharedString {
        self.title.clone().unwrap_or("New Thread".into())
    }

    pub fn is_generating_summary(&self) -> bool {
        self.pending_summary_generation.is_some()
    }

    pub fn is_generating_title(&self) -> bool {
        self.pending_title_generation.is_some()
    }

    pub fn summary(&mut self, cx: &mut Context<Self>) -> Shared<Task<Option<SharedString>>> {
        if let Some(summary) = self.summary.as_ref() {
            return Task::ready(Some(summary.clone())).shared();
        }
        if let Some(task) = self.pending_summary_generation.clone() {
            return task;
        }
        let Some(model) = self.summarization_model.clone() else {
            log::error!("No summarization model available");
            return Task::ready(None).shared();
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
            reasoning_details: None,
        });

        let task = cx
            .spawn(async move |this, cx| {
                let mut summary = String::new();
                let mut messages = model.stream_completion(request, cx).await.log_err()?;
                while let Some(event) = messages.next().await {
                    let event = event.log_err()?;
                    let text = match event {
                        LanguageModelCompletionEvent::Text(text) => text,
                        _ => continue,
                    };

                    let mut lines = text.lines();
                    summary.extend(lines.next());
                }

                log::debug!("Setting summary: {}", summary);
                let summary = SharedString::from(summary);

                this.update(cx, |this, cx| {
                    this.summary = Some(summary.clone());
                    this.pending_summary_generation = None;
                    cx.notify()
                })
                .ok()?;

                Some(summary)
            })
            .shared();
        self.pending_summary_generation = Some(task.clone());
        task
    }

    pub fn generate_title(&mut self, cx: &mut Context<Self>) {
        let Some(model) = self.summarization_model.clone() else {
            return;
        };

        log::debug!(
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
            reasoning_details: None,
        });
        self.pending_title_generation = Some(cx.spawn(async move |this, cx| {
            let mut title = String::new();

            let generate = async {
                let mut messages = model.stream_completion(request, cx).await?;
                while let Some(event) = messages.next().await {
                    let event = event?;
                    let text = match event {
                        LanguageModelCompletionEvent::Text(text) => text,
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

    fn clear_summary(&mut self) {
        self.summary = None;
        self.pending_summary_generation = None;
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

        if message.content.is_empty() {
            return;
        }

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
        self.clear_summary();
        cx.notify()
    }

    pub(crate) fn build_completion_request(
        &self,
        completion_intent: CompletionIntent,
        cx: &App,
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

        let available_tools: Vec<_> = self
            .running_turn
            .as_ref()
            .map(|turn| turn.tools.keys().cloned().collect())
            .unwrap_or_default();

        log::debug!("Request includes {} tools", available_tools.len());
        let messages = self.build_request_messages(available_tools, cx);
        log::debug!("Request will include {} messages", messages.len());

        let request = LanguageModelRequest {
            thread_id: Some(self.id.to_string()),
            prompt_id: Some(self.prompt_id.to_string()),
            intent: Some(completion_intent),
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
                if tool.supports_provider(&model.provider_id())
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

    pub fn has_tool(&self, name: &str) -> bool {
        self.running_turn
            .as_ref()
            .is_some_and(|turn| turn.tools.contains_key(name))
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn has_registered_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }

    pub fn registered_tool_names(&self) -> Vec<SharedString> {
        self.tools.keys().cloned().collect()
    }

    pub fn register_running_subagent(&mut self, subagent: WeakEntity<Thread>) {
        self.running_subagents.push(subagent);
    }

    pub fn unregister_running_subagent(&mut self, subagent: &WeakEntity<Thread>) {
        self.running_subagents
            .retain(|s| s.entity_id() != subagent.entity_id());
    }

    pub fn running_subagent_count(&self) -> usize {
        self.running_subagents
            .iter()
            .filter(|s| s.upgrade().is_some())
            .count()
    }

    pub fn is_subagent(&self) -> bool {
        self.subagent_context.is_some()
    }

    pub fn depth(&self) -> u8 {
        self.subagent_context.as_ref().map(|c| c.depth).unwrap_or(0)
    }

    pub fn is_turn_complete(&self) -> bool {
        self.running_turn.is_none()
    }

    pub fn submit_user_message(
        &mut self,
        content: impl Into<String>,
        cx: &mut Context<Self>,
    ) -> Result<mpsc::UnboundedReceiver<Result<ThreadEvent>>> {
        let content = content.into();
        self.messages.push(Message::User(UserMessage {
            id: UserMessageId::new(),
            content: vec![UserMessageContent::Text(content)],
        }));
        cx.notify();
        self.send_existing(cx)
    }

    pub fn interrupt_for_summary(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Result<mpsc::UnboundedReceiver<Result<ThreadEvent>>> {
        let context = self
            .subagent_context
            .as_ref()
            .context("Not a subagent thread")?;
        let prompt = context.context_low_prompt.clone();
        self.cancel(cx).detach();
        self.submit_user_message(prompt, cx)
    }

    pub fn request_final_summary(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Result<mpsc::UnboundedReceiver<Result<ThreadEvent>>> {
        let context = self
            .subagent_context
            .as_ref()
            .context("Not a subagent thread")?;
        let prompt = context.summary_prompt.clone();
        self.submit_user_message(prompt, cx)
    }

    fn build_request_messages(
        &self,
        available_tools: Vec<SharedString>,
        cx: &App,
    ) -> Vec<LanguageModelRequestMessage> {
        log::trace!(
            "Building request messages from {} thread messages",
            self.messages.len()
        );

        let system_prompt = SystemPromptTemplate {
            project: self.project_context.read(cx),
            available_tools,
            model_name: self.model.as_ref().map(|m| m.name().0.to_string()),
        }
        .render(&self.templates)
        .context("failed to build system prompt")
        .expect("Invalid template");
        let mut messages = vec![LanguageModelRequestMessage {
            role: Role::System,
            content: vec![system_prompt.into()],
            cache: false,
            reasoning_details: None,
        }];
        for message in &self.messages {
            messages.extend(message.to_request());
        }

        if let Some(last_message) = messages.last_mut() {
            last_message.cache = true;
        }

        if let Some(message) = self.pending_message.as_ref() {
            messages.extend(message.to_request());
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
            Other(err) if err.is::<language_model::PaymentRequiredError>() => {
                // Retrying won't help for Payment Required errors.
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
    /// Sender to signal tool cancellation. When cancel is called, this is
    /// set to true so all tools can detect user-initiated cancellation.
    cancellation_tx: watch::Sender<bool>,
}

impl RunningTurn {
    fn cancel(mut self) -> Task<()> {
        log::debug!("Cancelling in progress turn");
        self.cancellation_tx.send(true).ok();
        self.event_stream.send_canceled();
        self._task
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

    fn description() -> SharedString {
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
    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        cx: &mut App,
    ) -> SharedString;

    /// Returns the JSON schema that describes the tool's input.
    fn input_schema(format: LanguageModelToolSchemaFormat) -> Schema {
        language_model::tool_schema::root_schema_for::<Self::Input>(format)
    }

    /// Some tools rely on a provider for the underlying billing or other reasons.
    /// Allow the tool to check if they are compatible, or should be filtered out.
    fn supports_provider(_provider: &LanguageModelProviderId) -> bool {
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
    fn initial_title(&self, input: serde_json::Value, _cx: &mut App) -> SharedString;
    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value>;
    fn supports_provider(&self, _provider: &LanguageModelProviderId) -> bool {
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
        T::description()
    }

    fn kind(&self) -> agent_client_protocol::ToolKind {
        T::kind()
    }

    fn initial_title(&self, input: serde_json::Value, _cx: &mut App) -> SharedString {
        let parsed_input = serde_json::from_value(input.clone()).map_err(|_| input);
        self.0.initial_title(parsed_input, _cx)
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        let mut json = serde_json::to_value(T::input_schema(format))?;
        language_model::tool_schema::adapt_schema_to_format(&mut json, format)?;
        Ok(json)
    }

    fn supports_provider(&self, provider: &LanguageModelProviderId) -> bool {
        T::supports_provider(provider)
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
                .update(|cx| self.0.clone().run(input, event_stream, cx))
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
        tool_name: &str,
        title: SharedString,
        kind: acp::ToolKind,
        input: serde_json::Value,
    ) {
        self.0
            .unbounded_send(Ok(ThreadEvent::ToolCall(Self::initial_tool_call(
                id,
                tool_name,
                title.to_string(),
                kind,
                input,
            ))))
            .ok();
    }

    fn initial_tool_call(
        id: &LanguageModelToolUseId,
        tool_name: &str,
        title: String,
        kind: acp::ToolKind,
        input: serde_json::Value,
    ) -> acp::ToolCall {
        acp::ToolCall::new(id.to_string(), title)
            .kind(kind)
            .raw_input(input)
            .meta(acp_thread::meta_with_tool_name(tool_name))
    }

    fn update_tool_call_fields(
        &self,
        tool_use_id: &LanguageModelToolUseId,
        fields: acp::ToolCallUpdateFields,
    ) {
        self.0
            .unbounded_send(Ok(ThreadEvent::ToolCallUpdate(
                acp::ToolCallUpdate::new(tool_use_id.to_string(), fields).into(),
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
    cancellation_rx: watch::Receiver<bool>,
}

impl ToolCallEventStream {
    #[cfg(any(test, feature = "test-support"))]
    pub fn test() -> (Self, ToolCallEventStreamReceiver) {
        let (stream, receiver, _cancellation_tx) = Self::test_with_cancellation();
        (stream, receiver)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_with_cancellation() -> (Self, ToolCallEventStreamReceiver, watch::Sender<bool>) {
        let (events_tx, events_rx) = mpsc::unbounded::<Result<ThreadEvent>>();
        let (cancellation_tx, cancellation_rx) = watch::channel(false);

        let stream = ToolCallEventStream::new(
            "test_id".into(),
            ThreadEventStream(events_tx),
            None,
            cancellation_rx,
        );

        (
            stream,
            ToolCallEventStreamReceiver(events_rx),
            cancellation_tx,
        )
    }

    /// Signal cancellation for this event stream. Only available in tests.
    #[cfg(any(test, feature = "test-support"))]
    pub fn signal_cancellation_with_sender(cancellation_tx: &mut watch::Sender<bool>) {
        cancellation_tx.send(true).ok();
    }

    fn new(
        tool_use_id: LanguageModelToolUseId,
        stream: ThreadEventStream,
        fs: Option<Arc<dyn Fs>>,
        cancellation_rx: watch::Receiver<bool>,
    ) -> Self {
        Self {
            tool_use_id,
            stream,
            fs,
            cancellation_rx,
        }
    }

    /// Returns a future that resolves when the user cancels the tool call.
    /// Tools should select on this alongside their main work to detect user cancellation.
    pub fn cancelled_by_user(&self) -> impl std::future::Future<Output = ()> + '_ {
        let mut rx = self.cancellation_rx.clone();
        async move {
            loop {
                if *rx.borrow() {
                    return;
                }
                if rx.changed().await.is_err() {
                    // Sender dropped, will never be cancelled
                    std::future::pending::<()>().await;
                }
            }
        }
    }

    /// Returns true if the user has cancelled this tool call.
    /// This is useful for checking cancellation state after an operation completes,
    /// to determine if the completion was due to user cancellation.
    pub fn was_cancelled_by_user(&self) -> bool {
        *self.cancellation_rx.clone().borrow()
    }

    pub fn tool_use_id(&self) -> &LanguageModelToolUseId {
        &self.tool_use_id
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
                    id: acp::ToolCallId::new(self.tool_use_id.to_string()),
                    diff,
                }
                .into(),
            )))
            .ok();
    }

    pub fn update_subagent_thread(&self, thread: Entity<acp_thread::AcpThread>) {
        self.stream
            .0
            .unbounded_send(Ok(ThreadEvent::ToolCallUpdate(
                acp_thread::ToolCallUpdateSubagentThread {
                    id: acp::ToolCallId::new(self.tool_use_id.to_string()),
                    thread,
                }
                .into(),
            )))
            .ok();
    }

    /// Authorize a third-party tool (e.g., MCP tool from a context server).
    ///
    /// Unlike built-in tools, third-party tools don't support pattern-based permissions.
    /// They only support `default_mode` (allow/deny/confirm) per tool.
    ///
    /// Uses the dropdown authorization flow with two granularities:
    /// - "Always for <display_name> MCP tool" → sets `tools.<tool_id>.default_mode = "allow"` or "deny"
    /// - "Only this time" → allow/deny once
    pub fn authorize_third_party_tool(
        &self,
        title: impl Into<String>,
        tool_id: String,
        display_name: String,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let settings = agent_settings::AgentSettings::get_global(cx);

        let decision = decide_permission_from_settings(&tool_id, "", &settings);

        match decision {
            ToolPermissionDecision::Allow => return Task::ready(Ok(())),
            ToolPermissionDecision::Deny(reason) => return Task::ready(Err(anyhow!(reason))),
            ToolPermissionDecision::Confirm => {}
        }

        let (response_tx, response_rx) = oneshot::channel();
        self.stream
            .0
            .unbounded_send(Ok(ThreadEvent::ToolCallAuthorization(
                ToolCallAuthorization {
                    tool_call: acp::ToolCallUpdate::new(
                        self.tool_use_id.to_string(),
                        acp::ToolCallUpdateFields::new().title(title.into()),
                    ),
                    options: acp_thread::PermissionOptions::Dropdown(vec![
                        acp_thread::PermissionOptionChoice {
                            allow: acp::PermissionOption::new(
                                acp::PermissionOptionId::new(format!(
                                    "always_allow_mcp:{}",
                                    tool_id
                                )),
                                format!("Always for {} MCP tool", display_name),
                                acp::PermissionOptionKind::AllowAlways,
                            ),
                            deny: acp::PermissionOption::new(
                                acp::PermissionOptionId::new(format!(
                                    "always_deny_mcp:{}",
                                    tool_id
                                )),
                                format!("Always for {} MCP tool", display_name),
                                acp::PermissionOptionKind::RejectAlways,
                            ),
                        },
                        acp_thread::PermissionOptionChoice {
                            allow: acp::PermissionOption::new(
                                acp::PermissionOptionId::new("allow"),
                                "Only this time",
                                acp::PermissionOptionKind::AllowOnce,
                            ),
                            deny: acp::PermissionOption::new(
                                acp::PermissionOptionId::new("deny"),
                                "Only this time",
                                acp::PermissionOptionKind::RejectOnce,
                            ),
                        },
                    ]),
                    response: response_tx,
                    context: None,
                },
            )))
            .ok();

        let fs = self.fs.clone();
        cx.spawn(async move |cx| {
            let response_str = response_rx.await?.0.to_string();

            if response_str == format!("always_allow_mcp:{}", tool_id) {
                if let Some(fs) = fs.clone() {
                    cx.update(|cx| {
                        update_settings_file(fs, cx, move |settings, _| {
                            settings
                                .agent
                                .get_or_insert_default()
                                .set_tool_default_mode(&tool_id, ToolPermissionMode::Allow);
                        });
                    });
                }
                return Ok(());
            }
            if response_str == format!("always_deny_mcp:{}", tool_id) {
                if let Some(fs) = fs.clone() {
                    cx.update(|cx| {
                        update_settings_file(fs, cx, move |settings, _| {
                            settings
                                .agent
                                .get_or_insert_default()
                                .set_tool_default_mode(&tool_id, ToolPermissionMode::Deny);
                        });
                    });
                }
                return Err(anyhow!("Permission to run tool denied by user"));
            }

            if response_str == "allow" {
                return Ok(());
            }

            Err(anyhow!("Permission to run tool denied by user"))
        })
    }

    pub fn authorize(
        &self,
        title: impl Into<String>,
        context: ToolPermissionContext,
        cx: &mut App,
    ) -> Task<Result<()>> {
        use settings::ToolPermissionMode;

        let options = context.build_permission_options();

        let (response_tx, response_rx) = oneshot::channel();
        self.stream
            .0
            .unbounded_send(Ok(ThreadEvent::ToolCallAuthorization(
                ToolCallAuthorization {
                    tool_call: acp::ToolCallUpdate::new(
                        self.tool_use_id.to_string(),
                        acp::ToolCallUpdateFields::new().title(title.into()),
                    ),
                    options,
                    response: response_tx,
                    context: Some(context),
                },
            )))
            .ok();

        let fs = self.fs.clone();
        cx.spawn(async move |cx| {
            let response_str = response_rx.await?.0.to_string();

            // Handle "always allow tool" - e.g., "always_allow:terminal"
            if let Some(tool) = response_str.strip_prefix("always_allow:") {
                if let Some(fs) = fs.clone() {
                    let tool = tool.to_string();
                    cx.update(|cx| {
                        update_settings_file(fs, cx, move |settings, _| {
                            settings
                                .agent
                                .get_or_insert_default()
                                .set_tool_default_mode(&tool, ToolPermissionMode::Allow);
                        });
                    });
                }
                return Ok(());
            }

            // Handle "always deny tool" - e.g., "always_deny:terminal"
            if let Some(tool) = response_str.strip_prefix("always_deny:") {
                if let Some(fs) = fs.clone() {
                    let tool = tool.to_string();
                    cx.update(|cx| {
                        update_settings_file(fs, cx, move |settings, _| {
                            settings
                                .agent
                                .get_or_insert_default()
                                .set_tool_default_mode(&tool, ToolPermissionMode::Deny);
                        });
                    });
                }
                return Err(anyhow!("Permission to run tool denied by user"));
            }

            // Handle "always allow pattern" - e.g., "always_allow_pattern:terminal:^cargo\s"
            if response_str.starts_with("always_allow_pattern:") {
                let parts: Vec<&str> = response_str.splitn(3, ':').collect();
                if parts.len() == 3 {
                    let pattern_tool_name = parts[1].to_string();
                    let pattern = parts[2].to_string();
                    if let Some(fs) = fs.clone() {
                        cx.update(|cx| {
                            update_settings_file(fs, cx, move |settings, _| {
                                settings
                                    .agent
                                    .get_or_insert_default()
                                    .add_tool_allow_pattern(&pattern_tool_name, pattern);
                            });
                        });
                    }
                }
                return Ok(());
            }

            // Handle "always deny pattern" - e.g., "always_deny_pattern:terminal:^cargo\s"
            if response_str.starts_with("always_deny_pattern:") {
                let parts: Vec<&str> = response_str.splitn(3, ':').collect();
                if parts.len() == 3 {
                    let pattern_tool_name = parts[1].to_string();
                    let pattern = parts[2].to_string();
                    if let Some(fs) = fs.clone() {
                        cx.update(|cx| {
                            update_settings_file(fs, cx, move |settings, _| {
                                settings
                                    .agent
                                    .get_or_insert_default()
                                    .add_tool_deny_pattern(&pattern_tool_name, pattern);
                            });
                        });
                    }
                }
                return Err(anyhow!("Permission to run tool denied by user"));
            }

            // Handle simple "allow" (allow once)
            if response_str == "allow" {
                return Ok(());
            }

            // Handle simple "deny" (deny once)
            Err(anyhow!("Permission to run tool denied by user"))
        })
    }
}

#[cfg(any(test, feature = "test-support"))]
pub struct ToolCallEventStreamReceiver(mpsc::UnboundedReceiver<Result<ThreadEvent>>);

#[cfg(any(test, feature = "test-support"))]
impl ToolCallEventStreamReceiver {
    pub async fn expect_authorization(&mut self) -> ToolCallAuthorization {
        let event = self.0.next().await;
        if let Some(Ok(ThreadEvent::ToolCallAuthorization(auth))) = event {
            auth
        } else {
            panic!("Expected ToolCallAuthorization but got: {:?}", event);
        }
    }

    pub async fn expect_update_fields(&mut self) -> acp::ToolCallUpdateFields {
        let event = self.0.next().await;
        if let Some(Ok(ThreadEvent::ToolCallUpdate(acp_thread::ToolCallUpdate::UpdateFields(
            update,
        )))) = event
        {
            update.fields
        } else {
            panic!("Expected update fields but got: {:?}", event);
        }
    }

    pub async fn expect_diff(&mut self) -> Entity<acp_thread::Diff> {
        let event = self.0.next().await;
        if let Some(Ok(ThreadEvent::ToolCallUpdate(acp_thread::ToolCallUpdate::UpdateDiff(
            update,
        )))) = event
        {
            update.diff
        } else {
            panic!("Expected diff but got: {:?}", event);
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

#[cfg(any(test, feature = "test-support"))]
impl std::ops::Deref for ToolCallEventStreamReceiver {
    type Target = mpsc::UnboundedReceiver<Result<ThreadEvent>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(any(test, feature = "test-support"))]
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

impl UserMessageContent {
    pub fn from_content_block(value: acp::ContentBlock, path_style: PathStyle) -> Self {
        match value {
            acp::ContentBlock::Text(text_content) => Self::Text(text_content.text),
            acp::ContentBlock::Image(image_content) => Self::Image(convert_image(image_content)),
            acp::ContentBlock::Audio(_) => {
                // TODO
                Self::Text("[audio]".to_string())
            }
            acp::ContentBlock::ResourceLink(resource_link) => {
                match MentionUri::parse(&resource_link.uri, path_style) {
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
                    match MentionUri::parse(&resource.uri, path_style) {
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
                other => {
                    log::warn!("Unexpected content type: {:?}", other);
                    Self::Text("[unknown]".to_string())
                }
            },
            other => {
                log::warn!("Unexpected content type: {:?}", other);
                Self::Text("[unknown]".to_string())
            }
        }
    }
}

impl From<UserMessageContent> for acp::ContentBlock {
    fn from(content: UserMessageContent) -> Self {
        match content {
            UserMessageContent::Text(text) => text.into(),
            UserMessageContent::Image(image) => {
                acp::ContentBlock::Image(acp::ImageContent::new(image.source, "image/png"))
            }
            UserMessageContent::Mention { uri, content } => acp::ContentBlock::Resource(
                acp::EmbeddedResource::new(acp::EmbeddedResourceResource::TextResourceContents(
                    acp::TextResourceContents::new(content, uri.to_uri().to_string()),
                )),
            ),
        }
    }
}

fn convert_image(image_content: acp::ImageContent) -> LanguageModelImage {
    LanguageModelImage {
        source: image_content.data.into(),
        size: None,
    }
}
