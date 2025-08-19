use crate::{ContextServerRegistry, SystemPromptTemplate, Template, Templates};
use acp_thread::{MentionUri, UserMessageId};
use action_log::ActionLog;
use agent_client_protocol as acp;
use agent_settings::{AgentProfileId, AgentSettings, CompletionMode};
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::adapt_schema_to_format;
use cloud_llm_client::{CompletionIntent, CompletionRequestStatus};
use collections::IndexMap;
use fs::Fs;
use futures::{
    channel::{mpsc, oneshot},
    stream::FuturesUnordered,
};
use gpui::{App, Context, Entity, SharedString, Task};
use language_model::{
    LanguageModel, LanguageModelCompletionEvent, LanguageModelImage, LanguageModelProviderId,
    LanguageModelRequest, LanguageModelRequestMessage, LanguageModelRequestTool,
    LanguageModelToolResult, LanguageModelToolResultContent, LanguageModelToolSchemaFormat,
    LanguageModelToolUse, LanguageModelToolUseId, Role, StopReason,
};
use project::Project;
use prompt_store::ProjectContext;
use schemars::{JsonSchema, Schema};
use serde::{Deserialize, Serialize};
use settings::{Settings, update_settings_file};
use smol::stream::StreamExt;
use std::{cell::RefCell, collections::BTreeMap, path::Path, rc::Rc, sync::Arc};
use std::{fmt::Write, ops::Range};
use util::{ResultExt, markdown::MarkdownCodeBlock};
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn to_markdown(&self) -> String {
        match self {
            Message::User(message) => message.to_markdown(),
            Message::Agent(message) => message.to_markdown(),
            Message::Resume => "[resumed after tool use limit was reached]".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserMessage {
    pub id: UserMessageId,
    pub content: Vec<UserMessageContent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
                        let _ = write!(&mut markdown, "{}\n\n{}\n", uri.as_link(), content);
                    } else {
                        let _ = write!(&mut markdown, "{}\n", uri.as_link());
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
        const OPEN_THREADS_TAG: &str = "<threads>";
        const OPEN_FETCH_TAG: &str = "<fetched_urls>";
        const OPEN_RULES_TAG: &str =
            "<rules>\nThe user has specified the following rules that should be applied:\n";

        let mut file_context = OPEN_FILES_TAG.to_string();
        let mut directory_context = OPEN_DIRECTORIES_TAG.to_string();
        let mut symbol_context = OPEN_SYMBOLS_TAG.to_string();
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
                                &mut symbol_context,
                                "\n{}",
                                MarkdownCodeBlock {
                                    tag: &codeblock_tag(abs_path, None),
                                    text: &content.to_string(),
                                }
                            )
                            .ok();
                        }
                        MentionUri::Directory { .. } => {
                            write!(&mut directory_context, "\n{}\n", content).ok();
                        }
                        MentionUri::Symbol {
                            path, line_range, ..
                        }
                        | MentionUri::Selection {
                            path, line_range, ..
                        } => {
                            write!(
                                &mut rules_context,
                                "\n{}",
                                MarkdownCodeBlock {
                                    tag: &codeblock_tag(path, Some(line_range)),
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

fn codeblock_tag(full_path: &Path, line_range: Option<&Range<u32>>) -> String {
    let mut result = String::new();

    if let Some(extension) = full_path.extension().and_then(|ext| ext.to_str()) {
        let _ = write!(result, "{} ", extension);
    }

    let _ = write!(result, "{}", full_path.display());

    if let Some(range) = line_range {
        if range.start == range.end {
            let _ = write!(result, ":{}", range.start + 1);
        } else {
            let _ = write!(result, ":{}-{}", range.start + 1, range.end + 1);
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
                AgentMessageContent::Image(_) => {
                    markdown.push_str("<image />\n");
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
                AgentMessageContent::Image(value) => {
                    language_model::MessageContent::Image(value.clone())
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

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AgentMessage {
    pub content: Vec<AgentMessageContent>,
    pub tool_results: IndexMap<LanguageModelToolUseId, LanguageModelToolResult>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentMessageContent {
    Text(String),
    Thinking {
        text: String,
        signature: Option<String>,
    },
    RedactedThinking(String),
    Image(LanguageModelImage),
    ToolUse(LanguageModelToolUse),
}

#[derive(Debug)]
pub enum AgentResponseEvent {
    Text(String),
    Thinking(String),
    ToolCall(acp::ToolCall),
    ToolCallUpdate(acp_thread::ToolCallUpdate),
    ToolCallAuthorization(ToolCallAuthorization),
    Stop(acp::StopReason),
}

#[derive(Debug)]
pub struct ToolCallAuthorization {
    pub tool_call: acp::ToolCallUpdate,
    pub options: Vec<acp::PermissionOption>,
    pub response: oneshot::Sender<acp::PermissionOptionId>,
}

pub struct Thread {
    id: ThreadId,
    prompt_id: PromptId,
    messages: Vec<Message>,
    completion_mode: CompletionMode,
    /// Holds the task that handles agent interaction until the end of the turn.
    /// Survives across multiple requests as the model performs tool calls and
    /// we run tools, report their results.
    running_turn: Option<RunningTurn>,
    pending_message: Option<AgentMessage>,
    tools: BTreeMap<SharedString, Arc<dyn AnyAgentTool>>,
    tool_use_limit_reached: bool,
    context_server_registry: Entity<ContextServerRegistry>,
    profile_id: AgentProfileId,
    project_context: Rc<RefCell<ProjectContext>>,
    templates: Arc<Templates>,
    model: Option<Arc<dyn LanguageModel>>,
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
}

impl Thread {
    pub fn new(
        project: Entity<Project>,
        project_context: Rc<RefCell<ProjectContext>>,
        context_server_registry: Entity<ContextServerRegistry>,
        action_log: Entity<ActionLog>,
        templates: Arc<Templates>,
        model: Option<Arc<dyn LanguageModel>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let profile_id = AgentSettings::get_global(cx).default_profile.clone();
        Self {
            id: ThreadId::new(),
            prompt_id: PromptId::new(),
            messages: Vec::new(),
            completion_mode: CompletionMode::Normal,
            running_turn: None,
            pending_message: None,
            tools: BTreeMap::default(),
            tool_use_limit_reached: false,
            context_server_registry,
            profile_id,
            project_context,
            templates,
            model,
            project,
            action_log,
        }
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    pub fn action_log(&self) -> &Entity<ActionLog> {
        &self.action_log
    }

    pub fn model(&self) -> Option<&Arc<dyn LanguageModel>> {
        self.model.as_ref()
    }

    pub fn set_model(&mut self, model: Arc<dyn LanguageModel>) {
        self.model = Some(model);
    }

    pub fn completion_mode(&self) -> CompletionMode {
        self.completion_mode
    }

    pub fn set_completion_mode(&mut self, mode: CompletionMode) {
        self.completion_mode = mode;
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn last_message(&self) -> Option<Message> {
        if let Some(message) = self.pending_message.clone() {
            Some(Message::Agent(message))
        } else {
            self.messages.last().cloned()
        }
    }

    pub fn add_tool(&mut self, tool: impl AgentTool) {
        self.tools.insert(tool.name(), tool.erase());
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

    pub fn cancel(&mut self) {
        if let Some(running_turn) = self.running_turn.take() {
            running_turn.cancel();
        }
        self.flush_pending_message();
    }

    pub fn truncate(&mut self, message_id: UserMessageId) -> Result<()> {
        self.cancel();
        let Some(position) = self.messages.iter().position(
            |msg| matches!(msg, Message::User(UserMessage { id, .. }) if id == &message_id),
        ) else {
            return Err(anyhow!("Message not found"));
        };
        self.messages.truncate(position);
        Ok(())
    }

    pub fn resume(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Result<mpsc::UnboundedReceiver<Result<AgentResponseEvent>>> {
        anyhow::ensure!(self.model.is_some(), "Model not set");
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
    ) -> Result<mpsc::UnboundedReceiver<Result<AgentResponseEvent>>>
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
    ) -> Result<mpsc::UnboundedReceiver<Result<AgentResponseEvent>>> {
        self.cancel();

        let model = self
            .model()
            .cloned()
            .context("No language model configured")?;
        let (events_tx, events_rx) = mpsc::unbounded::<Result<AgentResponseEvent>>();
        let event_stream = AgentResponseEventStream(events_tx);
        let message_ix = self.messages.len().saturating_sub(1);
        self.tool_use_limit_reached = false;
        self.running_turn = Some(RunningTurn {
            event_stream: event_stream.clone(),
            _task: cx.spawn(async move |this, cx| {
                log::info!("Starting agent turn execution");
                let turn_result: Result<()> = async {
                    let mut completion_intent = CompletionIntent::UserPrompt;
                    loop {
                        log::debug!(
                            "Building completion request with intent: {:?}",
                            completion_intent
                        );
                        let request = this.update(cx, |this, cx| {
                            this.build_completion_request(completion_intent, cx)
                        })??;

                        log::info!("Calling model.stream_completion");
                        let mut events = model.stream_completion(request, cx).await?;
                        log::debug!("Stream completion started successfully");

                        let mut tool_use_limit_reached = false;
                        let mut tool_uses = FuturesUnordered::new();
                        while let Some(event) = events.next().await {
                            match event? {
                                LanguageModelCompletionEvent::StatusUpdate(
                                    CompletionRequestStatus::ToolUseLimitReached,
                                ) => {
                                    tool_use_limit_reached = true;
                                }
                                LanguageModelCompletionEvent::Stop(reason) => {
                                    event_stream.send_stop(reason);
                                    if reason == StopReason::Refusal {
                                        this.update(cx, |this, _cx| {
                                            this.flush_pending_message();
                                            this.messages.truncate(message_ix);
                                        })?;
                                        return Ok(());
                                    }
                                }
                                event => {
                                    log::trace!("Received completion event: {:?}", event);
                                    this.update(cx, |this, cx| {
                                        tool_uses.extend(this.handle_streamed_completion_event(
                                            event,
                                            &event_stream,
                                            cx,
                                        ));
                                    })
                                    .ok();
                                }
                            }
                        }

                        let used_tools = tool_uses.is_empty();
                        while let Some(tool_result) = tool_uses.next().await {
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
                            })
                            .ok();
                        }

                        if tool_use_limit_reached {
                            log::info!("Tool use limit reached, completing turn");
                            this.update(cx, |this, _cx| this.tool_use_limit_reached = true)?;
                            return Err(language_model::ToolUseLimitReachedError.into());
                        } else if used_tools {
                            log::info!("No tool uses found, completing turn");
                            return Ok(());
                        } else {
                            this.update(cx, |this, _| this.flush_pending_message())?;
                            completion_intent = CompletionIntent::ToolResults;
                        }
                    }
                }
                .await;

                if let Err(error) = turn_result {
                    log::error!("Turn execution failed: {:?}", error);
                    event_stream.send_error(error);
                } else {
                    log::info!("Turn execution completed successfully");
                }

                this.update(cx, |this, _| {
                    this.flush_pending_message();
                    this.running_turn.take();
                })
                .ok();
            }),
        });
        Ok(events_rx)
    }

    pub fn build_system_message(&self) -> LanguageModelRequestMessage {
        log::debug!("Building system message");
        let prompt = SystemPromptTemplate {
            project: &self.project_context.borrow(),
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
    /// Returns an optional tool result task, which the main agentic loop in
    /// send will send back to the model when it resolves.
    fn handle_streamed_completion_event(
        &mut self,
        event: LanguageModelCompletionEvent,
        event_stream: &AgentResponseEventStream,
        cx: &mut Context<Self>,
    ) -> Option<Task<LanguageModelToolResult>> {
        log::trace!("Handling streamed completion event: {:?}", event);
        use LanguageModelCompletionEvent::*;

        match event {
            StartMessage { .. } => {
                self.flush_pending_message();
                self.pending_message = Some(AgentMessage::default());
            }
            Text(new_text) => self.handle_text_event(new_text, event_stream, cx),
            Thinking { text, signature } => {
                self.handle_thinking_event(text, signature, event_stream, cx)
            }
            RedactedThinking { data } => self.handle_redacted_thinking_event(data, cx),
            ToolUse(tool_use) => {
                return self.handle_tool_use_event(tool_use, event_stream, cx);
            }
            ToolUseJsonParseError {
                id,
                tool_name,
                raw_input,
                json_parse_error,
            } => {
                return Some(Task::ready(self.handle_tool_use_json_parse_error_event(
                    id,
                    tool_name,
                    raw_input,
                    json_parse_error,
                )));
            }
            UsageUpdate(_) | StatusUpdate(_) => {}
            Stop(_) => unreachable!(),
        }

        None
    }

    fn handle_text_event(
        &mut self,
        new_text: String,
        event_stream: &AgentResponseEventStream,
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
        event_stream: &AgentResponseEventStream,
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
        event_stream: &AgentResponseEventStream,
        cx: &mut Context<Self>,
    ) -> Option<Task<LanguageModelToolResult>> {
        cx.notify();

        let tool = self.tools.get(tool_use.name.as_ref()).cloned();
        let mut title = SharedString::from(&tool_use.name);
        let mut kind = acp::ToolKind::Other;
        if let Some(tool) = tool.as_ref() {
            title = tool.initial_title(tool_use.input.clone());
            kind = tool.kind();
        }

        // Ensure the last message ends in the current tool use
        let last_message = self.pending_message();
        let push_new_tool_use = last_message.content.last_mut().map_or(true, |content| {
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
        let supports_images = self.model().map_or(false, |model| model.supports_images());
        let tool_result = tool.run(tool_use.input, tool_event_stream, cx);
        log::info!("Running tool {}", tool_use.name);
        Some(cx.foreground_executor().spawn(async move {
            let tool_result = tool_result.await.and_then(|output| {
                if let LanguageModelToolResultContent::Image(_) = &output.llm_output {
                    if !supports_images {
                        return Err(anyhow!(
                            "Attempted to read an image, but this model doesn't support it.",
                        ));
                    }
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

    fn pending_message(&mut self) -> &mut AgentMessage {
        self.pending_message.get_or_insert_default()
    }

    fn flush_pending_message(&mut self) {
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
                        content: LanguageModelToolResultContent::Text(
                            "Tool canceled by user".into(),
                        ),
                        output: None,
                    },
                );
            }
        }

        self.messages.push(Message::Agent(message));
    }

    pub(crate) fn build_completion_request(
        &self,
        completion_intent: CompletionIntent,
        cx: &mut App,
    ) -> Result<LanguageModelRequest> {
        let model = self.model().context("No language model configured")?;

        log::debug!("Building completion request");
        log::debug!("Completion intent: {:?}", completion_intent);
        log::debug!("Completion mode: {:?}", self.completion_mode);

        let messages = self.build_request_messages();
        log::info!("Request will include {} messages", messages.len());

        let tools = if let Some(tools) = self.tools(cx).log_err() {
            tools
                .filter_map(|tool| {
                    let tool_name = tool.name().to_string();
                    log::trace!("Including tool: {}", tool_name);
                    Some(LanguageModelRequestTool {
                        name: tool_name,
                        description: tool.description().to_string(),
                        input_schema: tool.input_schema(model.tool_input_format()).log_err()?,
                    })
                })
                .collect()
        } else {
            Vec::new()
        };

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

    fn tools<'a>(&'a self, cx: &'a App) -> Result<impl Iterator<Item = &'a Arc<dyn AnyAgentTool>>> {
        let model = self.model().context("No language model configured")?;

        let profile = AgentSettings::get_global(cx)
            .profiles
            .get(&self.profile_id)
            .context("profile not found")?;
        let provider_id = model.provider_id();

        Ok(self
            .tools
            .iter()
            .filter(move |(_, tool)| tool.supported_provider(&provider_id))
            .filter_map(|(tool_name, tool)| {
                if profile.is_tool_enabled(tool_name) {
                    Some(tool)
                } else {
                    None
                }
            })
            .chain(self.context_server_registry.read(cx).servers().flat_map(
                |(server_id, tools)| {
                    tools.iter().filter_map(|(tool_name, tool)| {
                        if profile.is_context_server_tool_enabled(&server_id.0, tool_name) {
                            Some(tool)
                        } else {
                            None
                        }
                    })
                },
            )))
    }

    fn build_request_messages(&self) -> Vec<LanguageModelRequestMessage> {
        log::trace!(
            "Building request messages from {} thread messages",
            self.messages.len()
        );
        let mut messages = vec![self.build_system_message()];
        for message in &self.messages {
            match message {
                Message::User(message) => messages.push(message.to_request()),
                Message::Agent(message) => messages.extend(message.to_request()),
                Message::Resume => messages.push(LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec!["Continue where you left off".into()],
                    cache: false,
                }),
            }
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
}

struct RunningTurn {
    /// Holds the task that handles agent interaction until the end of the turn.
    /// Survives across multiple requests as the model performs tool calls and
    /// we run tools, report their results.
    _task: Task<()>,
    /// The current event stream for the running turn. Used to report a final
    /// cancellation event if we cancel the turn.
    event_stream: AgentResponseEventStream,
}

impl RunningTurn {
    fn cancel(self) {
        log::debug!("Cancelling in progress turn");
        self.event_stream.send_canceled();
    }
}

pub trait AgentTool
where
    Self: 'static + Sized,
{
    type Input: for<'de> Deserialize<'de> + Serialize + JsonSchema;
    type Output: for<'de> Deserialize<'de> + Serialize + Into<LanguageModelToolResultContent>;

    fn name(&self) -> SharedString;

    fn description(&self) -> SharedString {
        let schema = schemars::schema_for!(Self::Input);
        SharedString::new(
            schema
                .get("description")
                .and_then(|description| description.as_str())
                .unwrap_or_default(),
        )
    }

    fn kind(&self) -> acp::ToolKind;

    /// The initial tool title to display. Can be updated during the tool run.
    fn initial_title(&self, input: Result<Self::Input, serde_json::Value>) -> SharedString;

    /// Returns the JSON schema that describes the tool's input.
    fn input_schema(&self) -> Schema {
        schemars::schema_for!(Self::Input)
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
}

impl<T> AnyAgentTool for Erased<Arc<T>>
where
    T: AgentTool,
{
    fn name(&self) -> SharedString {
        self.0.name()
    }

    fn description(&self) -> SharedString {
        self.0.description()
    }

    fn kind(&self) -> agent_client_protocol::ToolKind {
        self.0.kind()
    }

    fn initial_title(&self, input: serde_json::Value) -> SharedString {
        let parsed_input = serde_json::from_value(input.clone()).map_err(|_| input);
        self.0.initial_title(parsed_input)
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        let mut json = serde_json::to_value(self.0.input_schema())?;
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
}

#[derive(Clone)]
struct AgentResponseEventStream(mpsc::UnboundedSender<Result<AgentResponseEvent>>);

impl AgentResponseEventStream {
    fn send_text(&self, text: &str) {
        self.0
            .unbounded_send(Ok(AgentResponseEvent::Text(text.to_string())))
            .ok();
    }

    fn send_thinking(&self, text: &str) {
        self.0
            .unbounded_send(Ok(AgentResponseEvent::Thinking(text.to_string())))
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
            .unbounded_send(Ok(AgentResponseEvent::ToolCall(Self::initial_tool_call(
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
            .unbounded_send(Ok(AgentResponseEvent::ToolCallUpdate(
                acp::ToolCallUpdate {
                    id: acp::ToolCallId(tool_use_id.to_string().into()),
                    fields,
                }
                .into(),
            )))
            .ok();
    }

    fn send_stop(&self, reason: StopReason) {
        match reason {
            StopReason::EndTurn => {
                self.0
                    .unbounded_send(Ok(AgentResponseEvent::Stop(acp::StopReason::EndTurn)))
                    .ok();
            }
            StopReason::MaxTokens => {
                self.0
                    .unbounded_send(Ok(AgentResponseEvent::Stop(acp::StopReason::MaxTokens)))
                    .ok();
            }
            StopReason::Refusal => {
                self.0
                    .unbounded_send(Ok(AgentResponseEvent::Stop(acp::StopReason::Refusal)))
                    .ok();
            }
            StopReason::ToolUse => {}
        }
    }

    fn send_canceled(&self) {
        self.0
            .unbounded_send(Ok(AgentResponseEvent::Stop(acp::StopReason::Canceled)))
            .ok();
    }

    fn send_error(&self, error: impl Into<anyhow::Error>) {
        self.0.unbounded_send(Err(error.into())).ok();
    }
}

#[derive(Clone)]
pub struct ToolCallEventStream {
    tool_use_id: LanguageModelToolUseId,
    stream: AgentResponseEventStream,
    fs: Option<Arc<dyn Fs>>,
}

impl ToolCallEventStream {
    #[cfg(test)]
    pub fn test() -> (Self, ToolCallEventStreamReceiver) {
        let (events_tx, events_rx) = mpsc::unbounded::<Result<AgentResponseEvent>>();

        let stream =
            ToolCallEventStream::new("test_id".into(), AgentResponseEventStream(events_tx), None);

        (stream, ToolCallEventStreamReceiver(events_rx))
    }

    fn new(
        tool_use_id: LanguageModelToolUseId,
        stream: AgentResponseEventStream,
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
            .unbounded_send(Ok(AgentResponseEvent::ToolCallUpdate(
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
            .unbounded_send(Ok(AgentResponseEvent::ToolCallUpdate(
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
            .unbounded_send(Ok(AgentResponseEvent::ToolCallAuthorization(
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
pub struct ToolCallEventStreamReceiver(mpsc::UnboundedReceiver<Result<AgentResponseEvent>>);

#[cfg(test)]
impl ToolCallEventStreamReceiver {
    pub async fn expect_authorization(&mut self) -> ToolCallAuthorization {
        let event = self.0.next().await;
        if let Some(Ok(AgentResponseEvent::ToolCallAuthorization(auth))) = event {
            auth
        } else {
            panic!("Expected ToolCallAuthorization but got: {:?}", event);
        }
    }

    pub async fn expect_terminal(&mut self) -> Entity<acp_thread::Terminal> {
        let event = self.0.next().await;
        if let Some(Ok(AgentResponseEvent::ToolCallUpdate(
            acp_thread::ToolCallUpdate::UpdateTerminal(update),
        ))) = event
        {
            update.terminal
        } else {
            panic!("Expected terminal but got: {:?}", event);
        }
    }
}

#[cfg(test)]
impl std::ops::Deref for ToolCallEventStreamReceiver {
    type Target = mpsc::UnboundedReceiver<Result<AgentResponseEvent>>;

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

fn convert_image(image_content: acp::ImageContent) -> LanguageModelImage {
    LanguageModelImage {
        source: image_content.data.into(),
        // TODO: make this optional?
        size: gpui::Size::new(0.into(), 0.into()),
    }
}
