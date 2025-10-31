mod connection;
mod diff;
mod mention;
mod terminal;

use ::terminal::terminal_settings::TerminalSettings;
use agent_settings::AgentSettings;
use collections::HashSet;
pub use connection::*;
pub use diff::*;
use language::language_settings::FormatOnSave;
pub use mention::*;
use project::lsp_store::{FormatTrigger, LspFormatTarget};
use serde::{Deserialize, Serialize};
use settings::{Settings as _, SettingsLocation};
use task::{Shell, ShellBuilder};
pub use terminal::*;

use action_log::ActionLog;
use agent_client_protocol::{self as acp};
use anyhow::{Context as _, Result, anyhow};
use editor::Bias;
use futures::{FutureExt, channel::oneshot, future::BoxFuture};
use gpui::{AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task, WeakEntity};
use itertools::Itertools;
use language::{Anchor, Buffer, BufferSnapshot, LanguageRegistry, Point, ToPoint, text_diff};
use markdown::Markdown;
use project::{AgentLocation, Project, git_store::GitStoreCheckpoint};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Formatter, Write};
use std::ops::Range;
use std::process::ExitStatus;
use std::rc::Rc;
use std::time::{Duration, Instant};
use std::{fmt::Display, mem, path::PathBuf, sync::Arc};
use ui::App;
use util::{ResultExt, get_default_system_shell_preferring_bash, paths::PathStyle};
use uuid::Uuid;

#[derive(Debug)]
pub struct UserMessage {
    pub id: Option<UserMessageId>,
    pub content: ContentBlock,
    pub chunks: Vec<acp::ContentBlock>,
    pub checkpoint: Option<Checkpoint>,
}

#[derive(Debug)]
pub struct Checkpoint {
    git_checkpoint: GitStoreCheckpoint,
    pub show: bool,
}

impl UserMessage {
    fn to_markdown(&self, cx: &App) -> String {
        let mut markdown = String::new();
        if self
            .checkpoint
            .as_ref()
            .is_some_and(|checkpoint| checkpoint.show)
        {
            writeln!(markdown, "## User (checkpoint)").unwrap();
        } else {
            writeln!(markdown, "## User").unwrap();
        }
        writeln!(markdown).unwrap();
        writeln!(markdown, "{}", self.content.to_markdown(cx)).unwrap();
        writeln!(markdown).unwrap();
        markdown
    }
}

#[derive(Debug, PartialEq)]
pub struct AssistantMessage {
    pub chunks: Vec<AssistantMessageChunk>,
}

impl AssistantMessage {
    pub fn to_markdown(&self, cx: &App) -> String {
        format!(
            "## Assistant\n\n{}\n\n",
            self.chunks
                .iter()
                .map(|chunk| chunk.to_markdown(cx))
                .join("\n\n")
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum AssistantMessageChunk {
    Message { block: ContentBlock },
    Thought { block: ContentBlock },
}

impl AssistantMessageChunk {
    pub fn from_str(
        chunk: &str,
        language_registry: &Arc<LanguageRegistry>,
        path_style: PathStyle,
        cx: &mut App,
    ) -> Self {
        Self::Message {
            block: ContentBlock::new(chunk.into(), language_registry, path_style, cx),
        }
    }

    fn to_markdown(&self, cx: &App) -> String {
        match self {
            Self::Message { block } => block.to_markdown(cx).to_string(),
            Self::Thought { block } => {
                format!("<thinking>\n{}\n</thinking>", block.to_markdown(cx))
            }
        }
    }
}

#[derive(Debug)]
pub enum AgentThreadEntry {
    UserMessage(UserMessage),
    AssistantMessage(AssistantMessage),
    ToolCall(ToolCall),
}

impl AgentThreadEntry {
    pub fn to_markdown(&self, cx: &App) -> String {
        match self {
            Self::UserMessage(message) => message.to_markdown(cx),
            Self::AssistantMessage(message) => message.to_markdown(cx),
            Self::ToolCall(tool_call) => tool_call.to_markdown(cx),
        }
    }

    pub fn user_message(&self) -> Option<&UserMessage> {
        if let AgentThreadEntry::UserMessage(message) = self {
            Some(message)
        } else {
            None
        }
    }

    pub fn diffs(&self) -> impl Iterator<Item = &Entity<Diff>> {
        if let AgentThreadEntry::ToolCall(call) = self {
            itertools::Either::Left(call.diffs())
        } else {
            itertools::Either::Right(std::iter::empty())
        }
    }

    pub fn terminals(&self) -> impl Iterator<Item = &Entity<Terminal>> {
        if let AgentThreadEntry::ToolCall(call) = self {
            itertools::Either::Left(call.terminals())
        } else {
            itertools::Either::Right(std::iter::empty())
        }
    }

    pub fn location(&self, ix: usize) -> Option<(acp::ToolCallLocation, AgentLocation)> {
        if let AgentThreadEntry::ToolCall(ToolCall {
            locations,
            resolved_locations,
            ..
        }) = self
        {
            Some((
                locations.get(ix)?.clone(),
                resolved_locations.get(ix)?.clone()?,
            ))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ToolCall {
    pub id: acp::ToolCallId,
    pub label: Entity<Markdown>,
    pub kind: acp::ToolKind,
    pub content: Vec<ToolCallContent>,
    pub status: ToolCallStatus,
    pub locations: Vec<acp::ToolCallLocation>,
    pub resolved_locations: Vec<Option<AgentLocation>>,
    pub raw_input: Option<serde_json::Value>,
    pub raw_output: Option<serde_json::Value>,
}

impl ToolCall {
    fn from_acp(
        tool_call: acp::ToolCall,
        status: ToolCallStatus,
        language_registry: Arc<LanguageRegistry>,
        path_style: PathStyle,
        terminals: &HashMap<acp::TerminalId, Entity<Terminal>>,
        cx: &mut App,
    ) -> Result<Self> {
        let title = if let Some((first_line, _)) = tool_call.title.split_once("\n") {
            first_line.to_owned() + "…"
        } else {
            tool_call.title
        };
        let mut content = Vec::with_capacity(tool_call.content.len());
        for item in tool_call.content {
            content.push(ToolCallContent::from_acp(
                item,
                language_registry.clone(),
                path_style,
                terminals,
                cx,
            )?);
        }

        let result = Self {
            id: tool_call.id,
            label: cx
                .new(|cx| Markdown::new(title.into(), Some(language_registry.clone()), None, cx)),
            kind: tool_call.kind,
            content,
            locations: tool_call.locations,
            resolved_locations: Vec::default(),
            status,
            raw_input: tool_call.raw_input,
            raw_output: tool_call.raw_output,
        };
        Ok(result)
    }

    fn update_fields(
        &mut self,
        fields: acp::ToolCallUpdateFields,
        language_registry: Arc<LanguageRegistry>,
        path_style: PathStyle,
        terminals: &HashMap<acp::TerminalId, Entity<Terminal>>,
        cx: &mut App,
    ) -> Result<()> {
        let acp::ToolCallUpdateFields {
            kind,
            status,
            title,
            content,
            locations,
            raw_input,
            raw_output,
        } = fields;

        if let Some(kind) = kind {
            self.kind = kind;
        }

        if let Some(status) = status {
            self.status = status.into();
        }

        if let Some(title) = title {
            self.label.update(cx, |label, cx| {
                if let Some((first_line, _)) = title.split_once("\n") {
                    label.replace(first_line.to_owned() + "…", cx)
                } else {
                    label.replace(title, cx);
                }
            });
        }

        if let Some(content) = content {
            let new_content_len = content.len();
            let mut content = content.into_iter();

            // Reuse existing content if we can
            for (old, new) in self.content.iter_mut().zip(content.by_ref()) {
                old.update_from_acp(new, language_registry.clone(), path_style, terminals, cx)?;
            }
            for new in content {
                self.content.push(ToolCallContent::from_acp(
                    new,
                    language_registry.clone(),
                    path_style,
                    terminals,
                    cx,
                )?)
            }
            self.content.truncate(new_content_len);
        }

        if let Some(locations) = locations {
            self.locations = locations;
        }

        if let Some(raw_input) = raw_input {
            self.raw_input = Some(raw_input);
        }

        if let Some(raw_output) = raw_output {
            if self.content.is_empty()
                && let Some(markdown) = markdown_for_raw_output(&raw_output, &language_registry, cx)
            {
                self.content
                    .push(ToolCallContent::ContentBlock(ContentBlock::Markdown {
                        markdown,
                    }));
            }
            self.raw_output = Some(raw_output);
        }
        Ok(())
    }

    pub fn diffs(&self) -> impl Iterator<Item = &Entity<Diff>> {
        self.content.iter().filter_map(|content| match content {
            ToolCallContent::Diff(diff) => Some(diff),
            ToolCallContent::ContentBlock(_) => None,
            ToolCallContent::Terminal(_) => None,
        })
    }

    pub fn terminals(&self) -> impl Iterator<Item = &Entity<Terminal>> {
        self.content.iter().filter_map(|content| match content {
            ToolCallContent::Terminal(terminal) => Some(terminal),
            ToolCallContent::ContentBlock(_) => None,
            ToolCallContent::Diff(_) => None,
        })
    }

    fn to_markdown(&self, cx: &App) -> String {
        let mut markdown = format!(
            "**Tool Call: {}**\nStatus: {}\n\n",
            self.label.read(cx).source(),
            self.status
        );
        for content in &self.content {
            markdown.push_str(content.to_markdown(cx).as_str());
            markdown.push_str("\n\n");
        }
        markdown
    }

    async fn resolve_location(
        location: acp::ToolCallLocation,
        project: WeakEntity<Project>,
        cx: &mut AsyncApp,
    ) -> Option<ResolvedLocation> {
        let buffer = project
            .update(cx, |project, cx| {
                project
                    .project_path_for_absolute_path(&location.path, cx)
                    .map(|path| project.open_buffer(path, cx))
            })
            .ok()??;
        let buffer = buffer.await.log_err()?;
        let position = buffer
            .update(cx, |buffer, _| {
                if let Some(row) = location.line {
                    let snapshot = buffer.snapshot();
                    let column = snapshot.indent_size_for_line(row).len;
                    let point = snapshot.clip_point(Point::new(row, column), Bias::Left);
                    snapshot.anchor_before(point)
                } else {
                    Anchor::MIN
                }
            })
            .ok()?;

        Some(ResolvedLocation { buffer, position })
    }

    fn resolve_locations(
        &self,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Task<Vec<Option<ResolvedLocation>>> {
        let locations = self.locations.clone();
        project.update(cx, |_, cx| {
            cx.spawn(async move |project, cx| {
                let mut new_locations = Vec::new();
                for location in locations {
                    new_locations.push(Self::resolve_location(location, project.clone(), cx).await);
                }
                new_locations
            })
        })
    }
}

// Separate so we can hold a strong reference to the buffer
// for saving on the thread
#[derive(Clone, Debug, PartialEq, Eq)]
struct ResolvedLocation {
    buffer: Entity<Buffer>,
    position: Anchor,
}

impl From<&ResolvedLocation> for AgentLocation {
    fn from(value: &ResolvedLocation) -> Self {
        Self {
            buffer: value.buffer.downgrade(),
            position: value.position,
        }
    }
}

#[derive(Debug)]
pub enum ToolCallStatus {
    /// The tool call hasn't started running yet, but we start showing it to
    /// the user.
    Pending,
    /// The tool call is waiting for confirmation from the user.
    WaitingForConfirmation {
        options: Vec<acp::PermissionOption>,
        respond_tx: oneshot::Sender<acp::PermissionOptionId>,
    },
    /// The tool call is currently running.
    InProgress,
    /// The tool call completed successfully.
    Completed,
    /// The tool call failed.
    Failed,
    /// The user rejected the tool call.
    Rejected,
    /// The user canceled generation so the tool call was canceled.
    Canceled,
}

impl From<acp::ToolCallStatus> for ToolCallStatus {
    fn from(status: acp::ToolCallStatus) -> Self {
        match status {
            acp::ToolCallStatus::Pending => Self::Pending,
            acp::ToolCallStatus::InProgress => Self::InProgress,
            acp::ToolCallStatus::Completed => Self::Completed,
            acp::ToolCallStatus::Failed => Self::Failed,
        }
    }
}

impl Display for ToolCallStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ToolCallStatus::Pending => "Pending",
                ToolCallStatus::WaitingForConfirmation { .. } => "Waiting for confirmation",
                ToolCallStatus::InProgress => "In Progress",
                ToolCallStatus::Completed => "Completed",
                ToolCallStatus::Failed => "Failed",
                ToolCallStatus::Rejected => "Rejected",
                ToolCallStatus::Canceled => "Canceled",
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ContentBlock {
    Empty,
    Markdown { markdown: Entity<Markdown> },
    ResourceLink { resource_link: acp::ResourceLink },
}

impl ContentBlock {
    pub fn new(
        block: acp::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        path_style: PathStyle,
        cx: &mut App,
    ) -> Self {
        let mut this = Self::Empty;
        this.append(block, language_registry, path_style, cx);
        this
    }

    pub fn new_combined(
        blocks: impl IntoIterator<Item = acp::ContentBlock>,
        language_registry: Arc<LanguageRegistry>,
        path_style: PathStyle,
        cx: &mut App,
    ) -> Self {
        let mut this = Self::Empty;
        for block in blocks {
            this.append(block, &language_registry, path_style, cx);
        }
        this
    }

    pub fn append(
        &mut self,
        block: acp::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        path_style: PathStyle,
        cx: &mut App,
    ) {
        if matches!(self, ContentBlock::Empty)
            && let acp::ContentBlock::ResourceLink(resource_link) = block
        {
            *self = ContentBlock::ResourceLink { resource_link };
            return;
        }

        let new_content = self.block_string_contents(block, path_style);

        match self {
            ContentBlock::Empty => {
                *self = Self::create_markdown_block(new_content, language_registry, cx);
            }
            ContentBlock::Markdown { markdown } => {
                markdown.update(cx, |markdown, cx| markdown.append(&new_content, cx));
            }
            ContentBlock::ResourceLink { resource_link } => {
                let existing_content = Self::resource_link_md(&resource_link.uri, path_style);
                let combined = format!("{}\n{}", existing_content, new_content);

                *self = Self::create_markdown_block(combined, language_registry, cx);
            }
        }
    }

    fn create_markdown_block(
        content: String,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> ContentBlock {
        ContentBlock::Markdown {
            markdown: cx
                .new(|cx| Markdown::new(content.into(), Some(language_registry.clone()), None, cx)),
        }
    }

    fn block_string_contents(&self, block: acp::ContentBlock, path_style: PathStyle) -> String {
        match block {
            acp::ContentBlock::Text(text_content) => text_content.text,
            acp::ContentBlock::ResourceLink(resource_link) => {
                Self::resource_link_md(&resource_link.uri, path_style)
            }
            acp::ContentBlock::Resource(acp::EmbeddedResource {
                resource:
                    acp::EmbeddedResourceResource::TextResourceContents(acp::TextResourceContents {
                        uri,
                        ..
                    }),
                ..
            }) => Self::resource_link_md(&uri, path_style),
            acp::ContentBlock::Image(image) => Self::image_md(&image),
            acp::ContentBlock::Audio(_) | acp::ContentBlock::Resource(_) => String::new(),
        }
    }

    fn resource_link_md(uri: &str, path_style: PathStyle) -> String {
        if let Some(uri) = MentionUri::parse(uri, path_style).log_err() {
            uri.as_link().to_string()
        } else {
            uri.to_string()
        }
    }

    fn image_md(_image: &acp::ImageContent) -> String {
        "`Image`".into()
    }

    pub fn to_markdown<'a>(&'a self, cx: &'a App) -> &'a str {
        match self {
            ContentBlock::Empty => "",
            ContentBlock::Markdown { markdown } => markdown.read(cx).source(),
            ContentBlock::ResourceLink { resource_link } => &resource_link.uri,
        }
    }

    pub fn markdown(&self) -> Option<&Entity<Markdown>> {
        match self {
            ContentBlock::Empty => None,
            ContentBlock::Markdown { markdown } => Some(markdown),
            ContentBlock::ResourceLink { .. } => None,
        }
    }

    pub fn resource_link(&self) -> Option<&acp::ResourceLink> {
        match self {
            ContentBlock::ResourceLink { resource_link } => Some(resource_link),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ToolCallContent {
    ContentBlock(ContentBlock),
    Diff(Entity<Diff>),
    Terminal(Entity<Terminal>),
}

impl ToolCallContent {
    pub fn from_acp(
        content: acp::ToolCallContent,
        language_registry: Arc<LanguageRegistry>,
        path_style: PathStyle,
        terminals: &HashMap<acp::TerminalId, Entity<Terminal>>,
        cx: &mut App,
    ) -> Result<Self> {
        match content {
            acp::ToolCallContent::Content { content } => Ok(Self::ContentBlock(ContentBlock::new(
                content,
                &language_registry,
                path_style,
                cx,
            ))),
            acp::ToolCallContent::Diff { diff } => Ok(Self::Diff(cx.new(|cx| {
                Diff::finalized(
                    diff.path.to_string_lossy().into_owned(),
                    diff.old_text,
                    diff.new_text,
                    language_registry,
                    cx,
                )
            }))),
            acp::ToolCallContent::Terminal { terminal_id } => terminals
                .get(&terminal_id)
                .cloned()
                .map(Self::Terminal)
                .ok_or_else(|| anyhow::anyhow!("Terminal with id `{}` not found", terminal_id)),
        }
    }

    pub fn update_from_acp(
        &mut self,
        new: acp::ToolCallContent,
        language_registry: Arc<LanguageRegistry>,
        path_style: PathStyle,
        terminals: &HashMap<acp::TerminalId, Entity<Terminal>>,
        cx: &mut App,
    ) -> Result<()> {
        let needs_update = match (&self, &new) {
            (Self::Diff(old_diff), acp::ToolCallContent::Diff { diff: new_diff }) => {
                old_diff.read(cx).needs_update(
                    new_diff.old_text.as_deref().unwrap_or(""),
                    &new_diff.new_text,
                    cx,
                )
            }
            _ => true,
        };

        if needs_update {
            *self = Self::from_acp(new, language_registry, path_style, terminals, cx)?;
        }
        Ok(())
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        match self {
            Self::ContentBlock(content) => content.to_markdown(cx).to_string(),
            Self::Diff(diff) => diff.read(cx).to_markdown(cx),
            Self::Terminal(terminal) => terminal.read(cx).to_markdown(cx),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ToolCallUpdate {
    UpdateFields(acp::ToolCallUpdate),
    UpdateDiff(ToolCallUpdateDiff),
    UpdateTerminal(ToolCallUpdateTerminal),
}

impl ToolCallUpdate {
    fn id(&self) -> &acp::ToolCallId {
        match self {
            Self::UpdateFields(update) => &update.id,
            Self::UpdateDiff(diff) => &diff.id,
            Self::UpdateTerminal(terminal) => &terminal.id,
        }
    }
}

impl From<acp::ToolCallUpdate> for ToolCallUpdate {
    fn from(update: acp::ToolCallUpdate) -> Self {
        Self::UpdateFields(update)
    }
}

impl From<ToolCallUpdateDiff> for ToolCallUpdate {
    fn from(diff: ToolCallUpdateDiff) -> Self {
        Self::UpdateDiff(diff)
    }
}

#[derive(Debug, PartialEq)]
pub struct ToolCallUpdateDiff {
    pub id: acp::ToolCallId,
    pub diff: Entity<Diff>,
}

impl From<ToolCallUpdateTerminal> for ToolCallUpdate {
    fn from(terminal: ToolCallUpdateTerminal) -> Self {
        Self::UpdateTerminal(terminal)
    }
}

#[derive(Debug, PartialEq)]
pub struct ToolCallUpdateTerminal {
    pub id: acp::ToolCallId,
    pub terminal: Entity<Terminal>,
}

#[derive(Debug, Default)]
pub struct Plan {
    pub entries: Vec<PlanEntry>,
}

#[derive(Debug)]
pub struct PlanStats<'a> {
    pub in_progress_entry: Option<&'a PlanEntry>,
    pub pending: u32,
    pub completed: u32,
}

impl Plan {
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn stats(&self) -> PlanStats<'_> {
        let mut stats = PlanStats {
            in_progress_entry: None,
            pending: 0,
            completed: 0,
        };

        for entry in &self.entries {
            match &entry.status {
                acp::PlanEntryStatus::Pending => {
                    stats.pending += 1;
                }
                acp::PlanEntryStatus::InProgress => {
                    stats.in_progress_entry = stats.in_progress_entry.or(Some(entry));
                }
                acp::PlanEntryStatus::Completed => {
                    stats.completed += 1;
                }
            }
        }

        stats
    }
}

#[derive(Debug)]
pub struct PlanEntry {
    pub content: Entity<Markdown>,
    pub priority: acp::PlanEntryPriority,
    pub status: acp::PlanEntryStatus,
}

impl PlanEntry {
    pub fn from_acp(entry: acp::PlanEntry, cx: &mut App) -> Self {
        Self {
            content: cx.new(|cx| Markdown::new(entry.content.into(), None, None, cx)),
            priority: entry.priority,
            status: entry.status,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenUsage {
    pub max_tokens: u64,
    pub used_tokens: u64,
}

impl TokenUsage {
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
        if self.max_tokens == 0 {
            TokenUsageRatio::Normal
        } else if self.used_tokens >= self.max_tokens {
            TokenUsageRatio::Exceeded
        } else if self.used_tokens as f32 / self.max_tokens as f32 >= warning_threshold {
            TokenUsageRatio::Warning
        } else {
            TokenUsageRatio::Normal
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenUsageRatio {
    Normal,
    Warning,
    Exceeded,
}

#[derive(Debug, Clone)]
pub struct RetryStatus {
    pub last_error: SharedString,
    pub attempt: usize,
    pub max_attempts: usize,
    pub started_at: Instant,
    pub duration: Duration,
}

pub struct AcpThread {
    title: SharedString,
    entries: Vec<AgentThreadEntry>,
    plan: Plan,
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
    shared_buffers: HashMap<Entity<Buffer>, BufferSnapshot>,
    send_task: Option<Task<()>>,
    connection: Rc<dyn AgentConnection>,
    session_id: acp::SessionId,
    token_usage: Option<TokenUsage>,
    prompt_capabilities: acp::PromptCapabilities,
    _observe_prompt_capabilities: Task<anyhow::Result<()>>,
    terminals: HashMap<acp::TerminalId, Entity<Terminal>>,
    pending_terminal_output: HashMap<acp::TerminalId, Vec<Vec<u8>>>,
    pending_terminal_exit: HashMap<acp::TerminalId, acp::TerminalExitStatus>,
}

#[derive(Debug)]
pub enum AcpThreadEvent {
    NewEntry,
    TitleUpdated,
    TokenUsageUpdated,
    EntryUpdated(usize),
    EntriesRemoved(Range<usize>),
    ToolAuthorizationRequired,
    Retry(RetryStatus),
    Stopped,
    Error,
    LoadError(LoadError),
    PromptCapabilitiesUpdated,
    Refusal,
    AvailableCommandsUpdated(Vec<acp::AvailableCommand>),
    ModeUpdated(acp::SessionModeId),
}

impl EventEmitter<AcpThreadEvent> for AcpThread {}

#[derive(Debug, Clone)]
pub enum TerminalProviderEvent {
    Created {
        terminal_id: acp::TerminalId,
        label: String,
        cwd: Option<PathBuf>,
        output_byte_limit: Option<u64>,
        terminal: Entity<::terminal::Terminal>,
    },
    Output {
        terminal_id: acp::TerminalId,
        data: Vec<u8>,
    },
    TitleChanged {
        terminal_id: acp::TerminalId,
        title: String,
    },
    Exit {
        terminal_id: acp::TerminalId,
        status: acp::TerminalExitStatus,
    },
}

#[derive(Debug, Clone)]
pub enum TerminalProviderCommand {
    WriteInput {
        terminal_id: acp::TerminalId,
        bytes: Vec<u8>,
    },
    Resize {
        terminal_id: acp::TerminalId,
        cols: u16,
        rows: u16,
    },
    Close {
        terminal_id: acp::TerminalId,
    },
}

impl AcpThread {
    pub fn on_terminal_provider_event(
        &mut self,
        event: TerminalProviderEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            TerminalProviderEvent::Created {
                terminal_id,
                label,
                cwd,
                output_byte_limit,
                terminal,
            } => {
                let entity = self.register_terminal_created(
                    terminal_id.clone(),
                    label,
                    cwd,
                    output_byte_limit,
                    terminal,
                    cx,
                );

                if let Some(mut chunks) = self.pending_terminal_output.remove(&terminal_id) {
                    for data in chunks.drain(..) {
                        entity.update(cx, |term, cx| {
                            term.inner().update(cx, |inner, cx| {
                                inner.write_output(&data, cx);
                            })
                        });
                    }
                }

                if let Some(_status) = self.pending_terminal_exit.remove(&terminal_id) {
                    entity.update(cx, |_term, cx| {
                        cx.notify();
                    });
                }

                cx.notify();
            }
            TerminalProviderEvent::Output { terminal_id, data } => {
                if let Some(entity) = self.terminals.get(&terminal_id) {
                    entity.update(cx, |term, cx| {
                        term.inner().update(cx, |inner, cx| {
                            inner.write_output(&data, cx);
                        })
                    });
                } else {
                    self.pending_terminal_output
                        .entry(terminal_id)
                        .or_default()
                        .push(data);
                }
            }
            TerminalProviderEvent::TitleChanged { terminal_id, title } => {
                if let Some(entity) = self.terminals.get(&terminal_id) {
                    entity.update(cx, |term, cx| {
                        term.inner().update(cx, |inner, cx| {
                            inner.breadcrumb_text = title;
                            cx.emit(::terminal::Event::BreadcrumbsChanged);
                        })
                    });
                }
            }
            TerminalProviderEvent::Exit {
                terminal_id,
                status,
            } => {
                if let Some(entity) = self.terminals.get(&terminal_id) {
                    entity.update(cx, |_term, cx| {
                        cx.notify();
                    });
                } else {
                    self.pending_terminal_exit.insert(terminal_id, status);
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ThreadStatus {
    Idle,
    Generating,
}

#[derive(Debug, Clone)]
pub enum LoadError {
    Unsupported {
        command: SharedString,
        current_version: SharedString,
        minimum_version: SharedString,
    },
    FailedToInstall(SharedString),
    Exited {
        status: ExitStatus,
    },
    Other(SharedString),
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Unsupported {
                command: path,
                current_version,
                minimum_version,
            } => {
                write!(
                    f,
                    "version {current_version} from {path} is not supported (need at least {minimum_version})"
                )
            }
            LoadError::FailedToInstall(msg) => write!(f, "Failed to install: {msg}"),
            LoadError::Exited { status } => write!(f, "Server exited with status {status}"),
            LoadError::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl Error for LoadError {}

impl AcpThread {
    pub fn new(
        title: impl Into<SharedString>,
        connection: Rc<dyn AgentConnection>,
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        session_id: acp::SessionId,
        mut prompt_capabilities_rx: watch::Receiver<acp::PromptCapabilities>,
        cx: &mut Context<Self>,
    ) -> Self {
        let prompt_capabilities = prompt_capabilities_rx.borrow().clone();
        let task = cx.spawn::<_, anyhow::Result<()>>(async move |this, cx| {
            loop {
                let caps = prompt_capabilities_rx.recv().await?;
                this.update(cx, |this, cx| {
                    this.prompt_capabilities = caps;
                    cx.emit(AcpThreadEvent::PromptCapabilitiesUpdated);
                })?;
            }
        });

        Self {
            action_log,
            shared_buffers: Default::default(),
            entries: Default::default(),
            plan: Default::default(),
            title: title.into(),
            project,
            send_task: None,
            connection,
            session_id,
            token_usage: None,
            prompt_capabilities,
            _observe_prompt_capabilities: task,
            terminals: HashMap::default(),
            pending_terminal_output: HashMap::default(),
            pending_terminal_exit: HashMap::default(),
        }
    }

    pub fn prompt_capabilities(&self) -> acp::PromptCapabilities {
        self.prompt_capabilities.clone()
    }

    pub fn connection(&self) -> &Rc<dyn AgentConnection> {
        &self.connection
    }

    pub fn action_log(&self) -> &Entity<ActionLog> {
        &self.action_log
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    pub fn title(&self) -> SharedString {
        self.title.clone()
    }

    pub fn entries(&self) -> &[AgentThreadEntry] {
        &self.entries
    }

    pub fn session_id(&self) -> &acp::SessionId {
        &self.session_id
    }

    pub fn status(&self) -> ThreadStatus {
        if self.send_task.is_some() {
            ThreadStatus::Generating
        } else {
            ThreadStatus::Idle
        }
    }

    pub fn token_usage(&self) -> Option<&TokenUsage> {
        self.token_usage.as_ref()
    }

    pub fn has_pending_edit_tool_calls(&self) -> bool {
        for entry in self.entries.iter().rev() {
            match entry {
                AgentThreadEntry::UserMessage(_) => return false,
                AgentThreadEntry::ToolCall(
                    call @ ToolCall {
                        status: ToolCallStatus::InProgress | ToolCallStatus::Pending,
                        ..
                    },
                ) if call.diffs().next().is_some() => {
                    return true;
                }
                AgentThreadEntry::ToolCall(_) | AgentThreadEntry::AssistantMessage(_) => {}
            }
        }

        false
    }

    pub fn used_tools_since_last_user_message(&self) -> bool {
        for entry in self.entries.iter().rev() {
            match entry {
                AgentThreadEntry::UserMessage(..) => return false,
                AgentThreadEntry::AssistantMessage(..) => continue,
                AgentThreadEntry::ToolCall(..) => return true,
            }
        }

        false
    }

    pub fn handle_session_update(
        &mut self,
        update: acp::SessionUpdate,
        cx: &mut Context<Self>,
    ) -> Result<(), acp::Error> {
        match update {
            acp::SessionUpdate::UserMessageChunk(acp::ContentChunk { content, .. }) => {
                self.push_user_content_block(None, content, cx);
            }
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk { content, .. }) => {
                self.push_assistant_content_block(content, false, cx);
            }
            acp::SessionUpdate::AgentThoughtChunk(acp::ContentChunk { content, .. }) => {
                self.push_assistant_content_block(content, true, cx);
            }
            acp::SessionUpdate::ToolCall(tool_call) => {
                self.upsert_tool_call(tool_call, cx)?;
            }
            acp::SessionUpdate::ToolCallUpdate(tool_call_update) => {
                self.update_tool_call(tool_call_update, cx)?;
            }
            acp::SessionUpdate::Plan(plan) => {
                self.update_plan(plan, cx);
            }
            acp::SessionUpdate::AvailableCommandsUpdate(acp::AvailableCommandsUpdate {
                available_commands,
                ..
            }) => cx.emit(AcpThreadEvent::AvailableCommandsUpdated(available_commands)),
            acp::SessionUpdate::CurrentModeUpdate(acp::CurrentModeUpdate {
                current_mode_id,
                ..
            }) => cx.emit(AcpThreadEvent::ModeUpdated(current_mode_id)),
        }
        Ok(())
    }

    pub fn push_user_content_block(
        &mut self,
        message_id: Option<UserMessageId>,
        chunk: acp::ContentBlock,
        cx: &mut Context<Self>,
    ) {
        let language_registry = self.project.read(cx).languages().clone();
        let path_style = self.project.read(cx).path_style(cx);
        let entries_len = self.entries.len();

        if let Some(last_entry) = self.entries.last_mut()
            && let AgentThreadEntry::UserMessage(UserMessage {
                id,
                content,
                chunks,
                ..
            }) = last_entry
        {
            *id = message_id.or(id.take());
            content.append(chunk.clone(), &language_registry, path_style, cx);
            chunks.push(chunk);
            let idx = entries_len - 1;
            cx.emit(AcpThreadEvent::EntryUpdated(idx));
        } else {
            let content = ContentBlock::new(chunk.clone(), &language_registry, path_style, cx);
            self.push_entry(
                AgentThreadEntry::UserMessage(UserMessage {
                    id: message_id,
                    content,
                    chunks: vec![chunk],
                    checkpoint: None,
                }),
                cx,
            );
        }
    }

    pub fn push_assistant_content_block(
        &mut self,
        chunk: acp::ContentBlock,
        is_thought: bool,
        cx: &mut Context<Self>,
    ) {
        let language_registry = self.project.read(cx).languages().clone();
        let path_style = self.project.read(cx).path_style(cx);
        let entries_len = self.entries.len();
        if let Some(last_entry) = self.entries.last_mut()
            && let AgentThreadEntry::AssistantMessage(AssistantMessage { chunks }) = last_entry
        {
            let idx = entries_len - 1;
            cx.emit(AcpThreadEvent::EntryUpdated(idx));
            match (chunks.last_mut(), is_thought) {
                (Some(AssistantMessageChunk::Message { block }), false)
                | (Some(AssistantMessageChunk::Thought { block }), true) => {
                    block.append(chunk, &language_registry, path_style, cx)
                }
                _ => {
                    let block = ContentBlock::new(chunk, &language_registry, path_style, cx);
                    if is_thought {
                        chunks.push(AssistantMessageChunk::Thought { block })
                    } else {
                        chunks.push(AssistantMessageChunk::Message { block })
                    }
                }
            }
        } else {
            let block = ContentBlock::new(chunk, &language_registry, path_style, cx);
            let chunk = if is_thought {
                AssistantMessageChunk::Thought { block }
            } else {
                AssistantMessageChunk::Message { block }
            };

            self.push_entry(
                AgentThreadEntry::AssistantMessage(AssistantMessage {
                    chunks: vec![chunk],
                }),
                cx,
            );
        }
    }

    fn push_entry(&mut self, entry: AgentThreadEntry, cx: &mut Context<Self>) {
        self.entries.push(entry);
        cx.emit(AcpThreadEvent::NewEntry);
    }

    pub fn can_set_title(&mut self, cx: &mut Context<Self>) -> bool {
        self.connection.set_title(&self.session_id, cx).is_some()
    }

    pub fn set_title(&mut self, title: SharedString, cx: &mut Context<Self>) -> Task<Result<()>> {
        if title != self.title {
            self.title = title.clone();
            cx.emit(AcpThreadEvent::TitleUpdated);
            if let Some(set_title) = self.connection.set_title(&self.session_id, cx) {
                return set_title.run(title, cx);
            }
        }
        Task::ready(Ok(()))
    }

    pub fn update_token_usage(&mut self, usage: Option<TokenUsage>, cx: &mut Context<Self>) {
        self.token_usage = usage;
        cx.emit(AcpThreadEvent::TokenUsageUpdated);
    }

    pub fn update_retry_status(&mut self, status: RetryStatus, cx: &mut Context<Self>) {
        cx.emit(AcpThreadEvent::Retry(status));
    }

    pub fn update_tool_call(
        &mut self,
        update: impl Into<ToolCallUpdate>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let update = update.into();
        let languages = self.project.read(cx).languages().clone();
        let path_style = self.project.read(cx).path_style(cx);

        let ix = match self.index_for_tool_call(update.id()) {
            Some(ix) => ix,
            None => {
                // Tool call not found - create a failed tool call entry
                let failed_tool_call = ToolCall {
                    id: update.id().clone(),
                    label: cx.new(|cx| Markdown::new("Tool call not found".into(), None, None, cx)),
                    kind: acp::ToolKind::Fetch,
                    content: vec![ToolCallContent::ContentBlock(ContentBlock::new(
                        acp::ContentBlock::Text(acp::TextContent {
                            text: "Tool call not found".to_string(),
                            annotations: None,
                            meta: None,
                        }),
                        &languages,
                        path_style,
                        cx,
                    ))],
                    status: ToolCallStatus::Failed,
                    locations: Vec::new(),
                    resolved_locations: Vec::new(),
                    raw_input: None,
                    raw_output: None,
                };
                self.push_entry(AgentThreadEntry::ToolCall(failed_tool_call), cx);
                return Ok(());
            }
        };
        let AgentThreadEntry::ToolCall(call) = &mut self.entries[ix] else {
            unreachable!()
        };

        match update {
            ToolCallUpdate::UpdateFields(update) => {
                let location_updated = update.fields.locations.is_some();
                call.update_fields(update.fields, languages, path_style, &self.terminals, cx)?;
                if location_updated {
                    self.resolve_locations(update.id, cx);
                }
            }
            ToolCallUpdate::UpdateDiff(update) => {
                call.content.clear();
                call.content.push(ToolCallContent::Diff(update.diff));
            }
            ToolCallUpdate::UpdateTerminal(update) => {
                call.content.clear();
                call.content
                    .push(ToolCallContent::Terminal(update.terminal));
            }
        }

        cx.emit(AcpThreadEvent::EntryUpdated(ix));

        Ok(())
    }

    /// Updates a tool call if id matches an existing entry, otherwise inserts a new one.
    pub fn upsert_tool_call(
        &mut self,
        tool_call: acp::ToolCall,
        cx: &mut Context<Self>,
    ) -> Result<(), acp::Error> {
        let status = tool_call.status.into();
        self.upsert_tool_call_inner(tool_call.into(), status, cx)
    }

    /// Fails if id does not match an existing entry.
    pub fn upsert_tool_call_inner(
        &mut self,
        update: acp::ToolCallUpdate,
        status: ToolCallStatus,
        cx: &mut Context<Self>,
    ) -> Result<(), acp::Error> {
        let language_registry = self.project.read(cx).languages().clone();
        let path_style = self.project.read(cx).path_style(cx);
        let id = update.id.clone();

        if let Some(ix) = self.index_for_tool_call(&id) {
            let AgentThreadEntry::ToolCall(call) = &mut self.entries[ix] else {
                unreachable!()
            };

            call.update_fields(
                update.fields,
                language_registry,
                path_style,
                &self.terminals,
                cx,
            )?;
            call.status = status;

            cx.emit(AcpThreadEvent::EntryUpdated(ix));
        } else {
            let call = ToolCall::from_acp(
                update.try_into()?,
                status,
                language_registry,
                self.project.read(cx).path_style(cx),
                &self.terminals,
                cx,
            )?;
            self.push_entry(AgentThreadEntry::ToolCall(call), cx);
        };

        self.resolve_locations(id, cx);
        Ok(())
    }

    fn index_for_tool_call(&self, id: &acp::ToolCallId) -> Option<usize> {
        self.entries
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, entry)| {
                if let AgentThreadEntry::ToolCall(tool_call) = entry
                    && &tool_call.id == id
                {
                    Some(index)
                } else {
                    None
                }
            })
    }

    fn tool_call_mut(&mut self, id: &acp::ToolCallId) -> Option<(usize, &mut ToolCall)> {
        // The tool call we are looking for is typically the last one, or very close to the end.
        // At the moment, it doesn't seem like a hashmap would be a good fit for this use case.
        self.entries
            .iter_mut()
            .enumerate()
            .rev()
            .find_map(|(index, tool_call)| {
                if let AgentThreadEntry::ToolCall(tool_call) = tool_call
                    && &tool_call.id == id
                {
                    Some((index, tool_call))
                } else {
                    None
                }
            })
    }

    pub fn tool_call(&mut self, id: &acp::ToolCallId) -> Option<(usize, &ToolCall)> {
        self.entries
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, tool_call)| {
                if let AgentThreadEntry::ToolCall(tool_call) = tool_call
                    && &tool_call.id == id
                {
                    Some((index, tool_call))
                } else {
                    None
                }
            })
    }

    pub fn resolve_locations(&mut self, id: acp::ToolCallId, cx: &mut Context<Self>) {
        let project = self.project.clone();
        let Some((_, tool_call)) = self.tool_call_mut(&id) else {
            return;
        };
        let task = tool_call.resolve_locations(project, cx);
        cx.spawn(async move |this, cx| {
            let resolved_locations = task.await;

            this.update(cx, |this, cx| {
                let project = this.project.clone();

                for location in resolved_locations.iter().flatten() {
                    this.shared_buffers
                        .insert(location.buffer.clone(), location.buffer.read(cx).snapshot());
                }
                let Some((ix, tool_call)) = this.tool_call_mut(&id) else {
                    return;
                };

                if let Some(Some(location)) = resolved_locations.last() {
                    project.update(cx, |project, cx| {
                        let should_ignore = if let Some(agent_location) = project
                            .agent_location()
                            .filter(|agent_location| agent_location.buffer == location.buffer)
                        {
                            let snapshot = location.buffer.read(cx).snapshot();
                            let old_position = agent_location.position.to_point(&snapshot);
                            let new_position = location.position.to_point(&snapshot);

                            // ignore this so that when we get updates from the edit tool
                            // the position doesn't reset to the startof line
                            old_position.row == new_position.row
                                && old_position.column > new_position.column
                        } else {
                            false
                        };
                        if !should_ignore {
                            project.set_agent_location(Some(location.into()), cx);
                        }
                    });
                }

                let resolved_locations = resolved_locations
                    .iter()
                    .map(|l| l.as_ref().map(|l| AgentLocation::from(l)))
                    .collect::<Vec<_>>();

                if tool_call.resolved_locations != resolved_locations {
                    tool_call.resolved_locations = resolved_locations;
                    cx.emit(AcpThreadEvent::EntryUpdated(ix));
                }
            })
        })
        .detach();
    }

    pub fn request_tool_call_authorization(
        &mut self,
        tool_call: acp::ToolCallUpdate,
        options: Vec<acp::PermissionOption>,
        respect_always_allow_setting: bool,
        cx: &mut Context<Self>,
    ) -> Result<BoxFuture<'static, acp::RequestPermissionOutcome>> {
        let (tx, rx) = oneshot::channel();

        if respect_always_allow_setting && AgentSettings::get_global(cx).always_allow_tool_actions {
            // Don't use AllowAlways, because then if you were to turn off always_allow_tool_actions,
            // some tools would (incorrectly) continue to auto-accept.
            if let Some(allow_once_option) = options.iter().find_map(|option| {
                if matches!(option.kind, acp::PermissionOptionKind::AllowOnce) {
                    Some(option.id.clone())
                } else {
                    None
                }
            }) {
                self.upsert_tool_call_inner(tool_call, ToolCallStatus::Pending, cx)?;
                return Ok(async {
                    acp::RequestPermissionOutcome::Selected {
                        option_id: allow_once_option,
                    }
                }
                .boxed());
            }
        }

        let status = ToolCallStatus::WaitingForConfirmation {
            options,
            respond_tx: tx,
        };

        self.upsert_tool_call_inner(tool_call, status, cx)?;
        cx.emit(AcpThreadEvent::ToolAuthorizationRequired);

        let fut = async {
            match rx.await {
                Ok(option) => acp::RequestPermissionOutcome::Selected { option_id: option },
                Err(oneshot::Canceled) => acp::RequestPermissionOutcome::Cancelled,
            }
        }
        .boxed();

        Ok(fut)
    }

    pub fn authorize_tool_call(
        &mut self,
        id: acp::ToolCallId,
        option_id: acp::PermissionOptionId,
        option_kind: acp::PermissionOptionKind,
        cx: &mut Context<Self>,
    ) {
        let Some((ix, call)) = self.tool_call_mut(&id) else {
            return;
        };

        let new_status = match option_kind {
            acp::PermissionOptionKind::RejectOnce | acp::PermissionOptionKind::RejectAlways => {
                ToolCallStatus::Rejected
            }
            acp::PermissionOptionKind::AllowOnce | acp::PermissionOptionKind::AllowAlways => {
                ToolCallStatus::InProgress
            }
        };

        let curr_status = mem::replace(&mut call.status, new_status);

        if let ToolCallStatus::WaitingForConfirmation { respond_tx, .. } = curr_status {
            respond_tx.send(option_id).log_err();
        } else if cfg!(debug_assertions) {
            panic!("tried to authorize an already authorized tool call");
        }

        cx.emit(AcpThreadEvent::EntryUpdated(ix));
    }

    pub fn first_tool_awaiting_confirmation(&self) -> Option<&ToolCall> {
        let mut first_tool_call = None;

        for entry in self.entries.iter().rev() {
            match &entry {
                AgentThreadEntry::ToolCall(call) => {
                    if let ToolCallStatus::WaitingForConfirmation { .. } = call.status {
                        first_tool_call = Some(call);
                    } else {
                        continue;
                    }
                }
                AgentThreadEntry::UserMessage(_) | AgentThreadEntry::AssistantMessage(_) => {
                    // Reached the beginning of the turn.
                    // If we had pending permission requests in the previous turn, they have been cancelled.
                    break;
                }
            }
        }

        first_tool_call
    }

    pub fn plan(&self) -> &Plan {
        &self.plan
    }

    pub fn update_plan(&mut self, request: acp::Plan, cx: &mut Context<Self>) {
        let new_entries_len = request.entries.len();
        let mut new_entries = request.entries.into_iter();

        // Reuse existing markdown to prevent flickering
        for (old, new) in self.plan.entries.iter_mut().zip(new_entries.by_ref()) {
            let PlanEntry {
                content,
                priority,
                status,
            } = old;
            content.update(cx, |old, cx| {
                old.replace(new.content, cx);
            });
            *priority = new.priority;
            *status = new.status;
        }
        for new in new_entries {
            self.plan.entries.push(PlanEntry::from_acp(new, cx))
        }
        self.plan.entries.truncate(new_entries_len);

        cx.notify();
    }

    fn clear_completed_plan_entries(&mut self, cx: &mut Context<Self>) {
        self.plan
            .entries
            .retain(|entry| !matches!(entry.status, acp::PlanEntryStatus::Completed));
        cx.notify();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn send_raw(
        &mut self,
        message: &str,
        cx: &mut Context<Self>,
    ) -> BoxFuture<'static, Result<()>> {
        self.send(
            vec![acp::ContentBlock::Text(acp::TextContent {
                text: message.to_string(),
                annotations: None,
                meta: None,
            })],
            cx,
        )
    }

    pub fn send(
        &mut self,
        message: Vec<acp::ContentBlock>,
        cx: &mut Context<Self>,
    ) -> BoxFuture<'static, Result<()>> {
        let block = ContentBlock::new_combined(
            message.clone(),
            self.project.read(cx).languages().clone(),
            self.project.read(cx).path_style(cx),
            cx,
        );
        let request = acp::PromptRequest {
            prompt: message.clone(),
            session_id: self.session_id.clone(),
            meta: None,
        };
        let git_store = self.project.read(cx).git_store().clone();

        let message_id = if self.connection.truncate(&self.session_id, cx).is_some() {
            Some(UserMessageId::new())
        } else {
            None
        };

        self.run_turn(cx, async move |this, cx| {
            this.update(cx, |this, cx| {
                this.push_entry(
                    AgentThreadEntry::UserMessage(UserMessage {
                        id: message_id.clone(),
                        content: block,
                        chunks: message,
                        checkpoint: None,
                    }),
                    cx,
                );
            })
            .ok();

            let old_checkpoint = git_store
                .update(cx, |git, cx| git.checkpoint(cx))?
                .await
                .context("failed to get old checkpoint")
                .log_err();
            this.update(cx, |this, cx| {
                if let Some((_ix, message)) = this.last_user_message() {
                    message.checkpoint = old_checkpoint.map(|git_checkpoint| Checkpoint {
                        git_checkpoint,
                        show: false,
                    });
                }
                this.connection.prompt(message_id, request, cx)
            })?
            .await
        })
    }

    pub fn can_resume(&self, cx: &App) -> bool {
        self.connection.resume(&self.session_id, cx).is_some()
    }

    pub fn resume(&mut self, cx: &mut Context<Self>) -> BoxFuture<'static, Result<()>> {
        self.run_turn(cx, async move |this, cx| {
            this.update(cx, |this, cx| {
                this.connection
                    .resume(&this.session_id, cx)
                    .map(|resume| resume.run(cx))
            })?
            .context("resuming a session is not supported")?
            .await
        })
    }

    fn run_turn(
        &mut self,
        cx: &mut Context<Self>,
        f: impl 'static + AsyncFnOnce(WeakEntity<Self>, &mut AsyncApp) -> Result<acp::PromptResponse>,
    ) -> BoxFuture<'static, Result<()>> {
        self.clear_completed_plan_entries(cx);

        let (tx, rx) = oneshot::channel();
        let cancel_task = self.cancel(cx);

        self.send_task = Some(cx.spawn(async move |this, cx| {
            cancel_task.await;
            tx.send(f(this, cx).await).ok();
        }));

        cx.spawn(async move |this, cx| {
            let response = rx.await;

            this.update(cx, |this, cx| this.update_last_checkpoint(cx))?
                .await?;

            this.update(cx, |this, cx| {
                this.project
                    .update(cx, |project, cx| project.set_agent_location(None, cx));
                match response {
                    Ok(Err(e)) => {
                        this.send_task.take();
                        cx.emit(AcpThreadEvent::Error);
                        Err(e)
                    }
                    result => {
                        let canceled = matches!(
                            result,
                            Ok(Ok(acp::PromptResponse {
                                stop_reason: acp::StopReason::Cancelled,
                                meta: None,
                            }))
                        );

                        // We only take the task if the current prompt wasn't canceled.
                        //
                        // This prompt may have been canceled because another one was sent
                        // while it was still generating. In these cases, dropping `send_task`
                        // would cause the next generation to be canceled.
                        if !canceled {
                            this.send_task.take();
                        }

                        // Handle refusal - distinguish between user prompt and tool call refusals
                        if let Ok(Ok(acp::PromptResponse {
                            stop_reason: acp::StopReason::Refusal,
                            meta: _,
                        })) = result
                        {
                            if let Some((user_msg_ix, _)) = this.last_user_message() {
                                // Check if there's a completed tool call with results after the last user message
                                // This indicates the refusal is in response to tool output, not the user's prompt
                                let has_completed_tool_call_after_user_msg =
                                    this.entries.iter().skip(user_msg_ix + 1).any(|entry| {
                                        if let AgentThreadEntry::ToolCall(tool_call) = entry {
                                            // Check if the tool call has completed and has output
                                            matches!(tool_call.status, ToolCallStatus::Completed)
                                                && tool_call.raw_output.is_some()
                                        } else {
                                            false
                                        }
                                    });

                                if has_completed_tool_call_after_user_msg {
                                    // Refusal is due to tool output - don't truncate, just notify
                                    // The model refused based on what the tool returned
                                    cx.emit(AcpThreadEvent::Refusal);
                                } else {
                                    // User prompt was refused - truncate back to before the user message
                                    let range = user_msg_ix..this.entries.len();
                                    if range.start < range.end {
                                        this.entries.truncate(user_msg_ix);
                                        cx.emit(AcpThreadEvent::EntriesRemoved(range));
                                    }
                                    cx.emit(AcpThreadEvent::Refusal);
                                }
                            } else {
                                // No user message found, treat as general refusal
                                cx.emit(AcpThreadEvent::Refusal);
                            }
                        }

                        cx.emit(AcpThreadEvent::Stopped);
                        Ok(())
                    }
                }
            })?
        })
        .boxed()
    }

    pub fn cancel(&mut self, cx: &mut Context<Self>) -> Task<()> {
        let Some(send_task) = self.send_task.take() else {
            return Task::ready(());
        };

        for entry in self.entries.iter_mut() {
            if let AgentThreadEntry::ToolCall(call) = entry {
                let cancel = matches!(
                    call.status,
                    ToolCallStatus::Pending
                        | ToolCallStatus::WaitingForConfirmation { .. }
                        | ToolCallStatus::InProgress
                );

                if cancel {
                    call.status = ToolCallStatus::Canceled;
                }
            }
        }

        self.connection.cancel(&self.session_id, cx);

        // Wait for the send task to complete
        cx.foreground_executor().spawn(send_task)
    }

    /// Restores the git working tree to the state at the given checkpoint (if one exists)
    pub fn restore_checkpoint(
        &mut self,
        id: UserMessageId,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some((_, message)) = self.user_message_mut(&id) else {
            return Task::ready(Err(anyhow!("message not found")));
        };

        let checkpoint = message
            .checkpoint
            .as_ref()
            .map(|c| c.git_checkpoint.clone());
        let rewind = self.rewind(id.clone(), cx);
        let git_store = self.project.read(cx).git_store().clone();

        cx.spawn(async move |_, cx| {
            rewind.await?;
            if let Some(checkpoint) = checkpoint {
                git_store
                    .update(cx, |git, cx| git.restore_checkpoint(checkpoint, cx))?
                    .await?;
            }

            Ok(())
        })
    }

    /// Rewinds this thread to before the entry at `index`, removing it and all
    /// subsequent entries while rejecting any action_log changes made from that point.
    /// Unlike `restore_checkpoint`, this method does not restore from git.
    pub fn rewind(&mut self, id: UserMessageId, cx: &mut Context<Self>) -> Task<Result<()>> {
        let Some(truncate) = self.connection.truncate(&self.session_id, cx) else {
            return Task::ready(Err(anyhow!("not supported")));
        };

        cx.spawn(async move |this, cx| {
            cx.update(|cx| truncate.run(id.clone(), cx))?.await?;
            this.update(cx, |this, cx| {
                if let Some((ix, _)) = this.user_message_mut(&id) {
                    let range = ix..this.entries.len();
                    this.entries.truncate(ix);
                    cx.emit(AcpThreadEvent::EntriesRemoved(range));
                }
                this.action_log()
                    .update(cx, |action_log, cx| action_log.reject_all_edits(cx))
            })?
            .await;
            Ok(())
        })
    }

    fn update_last_checkpoint(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let git_store = self.project.read(cx).git_store().clone();

        let old_checkpoint = if let Some((_, message)) = self.last_user_message() {
            if let Some(checkpoint) = message.checkpoint.as_ref() {
                checkpoint.git_checkpoint.clone()
            } else {
                return Task::ready(Ok(()));
            }
        } else {
            return Task::ready(Ok(()));
        };

        let new_checkpoint = git_store.update(cx, |git, cx| git.checkpoint(cx));
        cx.spawn(async move |this, cx| {
            let new_checkpoint = new_checkpoint
                .await
                .context("failed to get new checkpoint")
                .log_err();
            if let Some(new_checkpoint) = new_checkpoint {
                let equal = git_store
                    .update(cx, |git, cx| {
                        git.compare_checkpoints(old_checkpoint.clone(), new_checkpoint, cx)
                    })?
                    .await
                    .unwrap_or(true);
                this.update(cx, |this, cx| {
                    let (ix, message) = this.last_user_message().context("no user message")?;
                    let checkpoint = message.checkpoint.as_mut().context("no checkpoint")?;
                    checkpoint.show = !equal;
                    cx.emit(AcpThreadEvent::EntryUpdated(ix));
                    anyhow::Ok(())
                })??;
            }

            Ok(())
        })
    }

    fn last_user_message(&mut self) -> Option<(usize, &mut UserMessage)> {
        self.entries
            .iter_mut()
            .enumerate()
            .rev()
            .find_map(|(ix, entry)| {
                if let AgentThreadEntry::UserMessage(message) = entry {
                    Some((ix, message))
                } else {
                    None
                }
            })
    }

    fn user_message_mut(&mut self, id: &UserMessageId) -> Option<(usize, &mut UserMessage)> {
        self.entries.iter_mut().enumerate().find_map(|(ix, entry)| {
            if let AgentThreadEntry::UserMessage(message) = entry {
                if message.id.as_ref() == Some(id) {
                    Some((ix, message))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn read_text_file(
        &self,
        path: PathBuf,
        line: Option<u32>,
        limit: Option<u32>,
        reuse_shared_snapshot: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<String, acp::Error>> {
        // Args are 1-based, move to 0-based
        let line = line.unwrap_or_default().saturating_sub(1);
        let limit = limit.unwrap_or(u32::MAX);
        let project = self.project.clone();
        let action_log = self.action_log.clone();
        cx.spawn(async move |this, cx| {
            let load = project
                .update(cx, |project, cx| {
                    let path = project
                        .project_path_for_absolute_path(&path, cx)
                        .ok_or_else(|| {
                            acp::Error::resource_not_found(Some(path.display().to_string()))
                        })?;
                    Ok(project.open_buffer(path, cx))
                })
                .map_err(|e| acp::Error::internal_error().with_data(e.to_string()))
                .flatten()?;

            let buffer = load.await?;

            let snapshot = if reuse_shared_snapshot {
                this.read_with(cx, |this, _| {
                    this.shared_buffers.get(&buffer.clone()).cloned()
                })
                .log_err()
                .flatten()
            } else {
                None
            };

            let snapshot = if let Some(snapshot) = snapshot {
                snapshot
            } else {
                action_log.update(cx, |action_log, cx| {
                    action_log.buffer_read(buffer.clone(), cx);
                })?;

                let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot())?;
                this.update(cx, |this, _| {
                    this.shared_buffers.insert(buffer.clone(), snapshot.clone());
                })?;
                snapshot
            };

            let max_point = snapshot.max_point();
            let start_position = Point::new(line, 0);

            if start_position > max_point {
                return Err(acp::Error::invalid_params().with_data(format!(
                    "Attempting to read beyond the end of the file, line {}:{}",
                    max_point.row + 1,
                    max_point.column
                )));
            }

            let start = snapshot.anchor_before(start_position);
            let end = snapshot.anchor_before(Point::new(line.saturating_add(limit), 0));

            project.update(cx, |project, cx| {
                project.set_agent_location(
                    Some(AgentLocation {
                        buffer: buffer.downgrade(),
                        position: start,
                    }),
                    cx,
                );
            })?;

            Ok(snapshot.text_for_range(start..end).collect::<String>())
        })
    }

    pub fn write_text_file(
        &self,
        path: PathBuf,
        content: String,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let project = self.project.clone();
        let action_log = self.action_log.clone();
        cx.spawn(async move |this, cx| {
            let load = project.update(cx, |project, cx| {
                let path = project
                    .project_path_for_absolute_path(&path, cx)
                    .context("invalid path")?;
                anyhow::Ok(project.open_buffer(path, cx))
            });
            let buffer = load??.await?;
            let snapshot = this.update(cx, |this, cx| {
                this.shared_buffers
                    .get(&buffer)
                    .cloned()
                    .unwrap_or_else(|| buffer.read(cx).snapshot())
            })?;
            let edits = cx
                .background_executor()
                .spawn(async move {
                    let old_text = snapshot.text();
                    text_diff(old_text.as_str(), &content)
                        .into_iter()
                        .map(|(range, replacement)| {
                            (
                                snapshot.anchor_after(range.start)
                                    ..snapshot.anchor_before(range.end),
                                replacement,
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .await;

            project.update(cx, |project, cx| {
                project.set_agent_location(
                    Some(AgentLocation {
                        buffer: buffer.downgrade(),
                        position: edits
                            .last()
                            .map(|(range, _)| range.end)
                            .unwrap_or(Anchor::MIN),
                    }),
                    cx,
                );
            })?;

            let format_on_save = cx.update(|cx| {
                action_log.update(cx, |action_log, cx| {
                    action_log.buffer_read(buffer.clone(), cx);
                });

                let format_on_save = buffer.update(cx, |buffer, cx| {
                    buffer.edit(edits, None, cx);

                    let settings = language::language_settings::language_settings(
                        buffer.language().map(|l| l.name()),
                        buffer.file(),
                        cx,
                    );

                    settings.format_on_save != FormatOnSave::Off
                });
                action_log.update(cx, |action_log, cx| {
                    action_log.buffer_edited(buffer.clone(), cx);
                });
                format_on_save
            })?;

            if format_on_save {
                let format_task = project.update(cx, |project, cx| {
                    project.format(
                        HashSet::from_iter([buffer.clone()]),
                        LspFormatTarget::Buffers,
                        false,
                        FormatTrigger::Save,
                        cx,
                    )
                })?;
                format_task.await.log_err();

                action_log.update(cx, |action_log, cx| {
                    action_log.buffer_edited(buffer.clone(), cx);
                })?;
            }

            project
                .update(cx, |project, cx| project.save_buffer(buffer, cx))?
                .await
        })
    }

    pub fn create_terminal(
        &self,
        command: String,
        args: Vec<String>,
        extra_env: Vec<acp::EnvVariable>,
        cwd: Option<PathBuf>,
        output_byte_limit: Option<u64>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Terminal>>> {
        let env = match &cwd {
            Some(dir) => self.project.update(cx, |project, cx| {
                let worktree = project.find_worktree(dir.as_path(), cx);
                let shell = TerminalSettings::get(
                    worktree.as_ref().map(|(worktree, path)| SettingsLocation {
                        worktree_id: worktree.read(cx).id(),
                        path: &path,
                    }),
                    cx,
                )
                .shell
                .clone();
                project.directory_environment(&shell, dir.as_path().into(), cx)
            }),
            None => Task::ready(None).shared(),
        };
        let env = cx.spawn(async move |_, _| {
            let mut env = env.await.unwrap_or_default();
            // Disables paging for `git` and hopefully other commands
            env.insert("PAGER".into(), "".into());
            for var in extra_env {
                env.insert(var.name, var.value);
            }
            env
        });

        let project = self.project.clone();
        let language_registry = project.read(cx).languages().clone();
        let is_windows = project.read(cx).path_style(cx).is_windows();

        let terminal_id = acp::TerminalId(Uuid::new_v4().to_string().into());
        let terminal_task = cx.spawn({
            let terminal_id = terminal_id.clone();
            async move |_this, cx| {
                let env = env.await;
                let shell = project
                    .update(cx, |project, cx| {
                        project
                            .remote_client()
                            .and_then(|r| r.read(cx).default_system_shell())
                    })?
                    .unwrap_or_else(|| get_default_system_shell_preferring_bash());
                let (task_command, task_args) =
                    ShellBuilder::new(&Shell::Program(shell), is_windows)
                        .redirect_stdin_to_dev_null()
                        .build(Some(command.clone()), &args);
                let terminal = project
                    .update(cx, |project, cx| {
                        project.create_terminal_task(
                            task::SpawnInTerminal {
                                command: Some(task_command),
                                args: task_args,
                                cwd: cwd.clone(),
                                env,
                                ..Default::default()
                            },
                            cx,
                        )
                    })?
                    .await?;

                cx.new(|cx| {
                    Terminal::new(
                        terminal_id,
                        &format!("{} {}", command, args.join(" ")),
                        cwd,
                        output_byte_limit.map(|l| l as usize),
                        terminal,
                        language_registry,
                        cx,
                    )
                })
            }
        });

        cx.spawn(async move |this, cx| {
            let terminal = terminal_task.await?;
            this.update(cx, |this, _cx| {
                this.terminals.insert(terminal_id, terminal.clone());
                terminal
            })
        })
    }

    pub fn kill_terminal(
        &mut self,
        terminal_id: acp::TerminalId,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        self.terminals
            .get(&terminal_id)
            .context("Terminal not found")?
            .update(cx, |terminal, cx| {
                terminal.kill(cx);
            });

        Ok(())
    }

    pub fn release_terminal(
        &mut self,
        terminal_id: acp::TerminalId,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        self.terminals
            .remove(&terminal_id)
            .context("Terminal not found")?
            .update(cx, |terminal, cx| {
                terminal.kill(cx);
            });

        Ok(())
    }

    pub fn terminal(&self, terminal_id: acp::TerminalId) -> Result<Entity<Terminal>> {
        self.terminals
            .get(&terminal_id)
            .context("Terminal not found")
            .cloned()
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        self.entries.iter().map(|e| e.to_markdown(cx)).collect()
    }

    pub fn emit_load_error(&mut self, error: LoadError, cx: &mut Context<Self>) {
        cx.emit(AcpThreadEvent::LoadError(error));
    }

    pub fn register_terminal_created(
        &mut self,
        terminal_id: acp::TerminalId,
        command_label: String,
        working_dir: Option<PathBuf>,
        output_byte_limit: Option<u64>,
        terminal: Entity<::terminal::Terminal>,
        cx: &mut Context<Self>,
    ) -> Entity<Terminal> {
        let language_registry = self.project.read(cx).languages().clone();

        let entity = cx.new(|cx| {
            Terminal::new(
                terminal_id.clone(),
                &command_label,
                working_dir.clone(),
                output_byte_limit.map(|l| l as usize),
                terminal,
                language_registry,
                cx,
            )
        });
        self.terminals.insert(terminal_id.clone(), entity.clone());
        entity
    }
}

fn markdown_for_raw_output(
    raw_output: &serde_json::Value,
    language_registry: &Arc<LanguageRegistry>,
    cx: &mut App,
) -> Option<Entity<Markdown>> {
    match raw_output {
        serde_json::Value::Null => None,
        serde_json::Value::Bool(value) => Some(cx.new(|cx| {
            Markdown::new(
                value.to_string().into(),
                Some(language_registry.clone()),
                None,
                cx,
            )
        })),
        serde_json::Value::Number(value) => Some(cx.new(|cx| {
            Markdown::new(
                value.to_string().into(),
                Some(language_registry.clone()),
                None,
                cx,
            )
        })),
        serde_json::Value::String(value) => Some(cx.new(|cx| {
            Markdown::new(
                value.clone().into(),
                Some(language_registry.clone()),
                None,
                cx,
            )
        })),
        value => Some(cx.new(|cx| {
            Markdown::new(
                format!("```json\n{}\n```", value).into(),
                Some(language_registry.clone()),
                None,
                cx,
            )
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use futures::{channel::mpsc, future::LocalBoxFuture, select};
    use gpui::{App, AsyncApp, TestAppContext, WeakEntity};
    use indoc::indoc;
    use project::{FakeFs, Fs};
    use rand::{distr, prelude::*};
    use serde_json::json;
    use settings::SettingsStore;
    use smol::stream::StreamExt as _;
    use std::{
        any::Any,
        cell::RefCell,
        path::Path,
        rc::Rc,
        sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst},
        time::Duration,
    };
    use util::path;

    fn init_test(cx: &mut TestAppContext) {
        env_logger::try_init().ok();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
            language::init(cx);
        });
    }

    #[gpui::test]
    async fn test_terminal_output_buffered_before_created_renders(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| connection.new_thread(project, std::path::Path::new(path!("/test")), cx))
            .await
            .unwrap();

        let terminal_id = acp::TerminalId(uuid::Uuid::new_v4().to_string().into());

        // Send Output BEFORE Created - should be buffered by acp_thread
        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Output {
                    terminal_id: terminal_id.clone(),
                    data: b"hello buffered".to_vec(),
                },
                cx,
            );
        });

        // Create a display-only terminal and then send Created
        let lower = cx.new(|cx| {
            let builder = ::terminal::TerminalBuilder::new_display_only(
                ::terminal::terminal_settings::CursorShape::default(),
                ::terminal::terminal_settings::AlternateScroll::On,
                None,
                0,
            )
            .unwrap();
            builder.subscribe(cx)
        });

        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Created {
                    terminal_id: terminal_id.clone(),
                    label: "Buffered Test".to_string(),
                    cwd: None,
                    output_byte_limit: None,
                    terminal: lower.clone(),
                },
                cx,
            );
        });

        // After Created, buffered Output should have been flushed into the renderer
        let content = thread.read_with(cx, |thread, cx| {
            let term = thread.terminal(terminal_id.clone()).unwrap();
            term.read_with(cx, |t, cx| t.inner().read(cx).get_content())
        });

        assert!(
            content.contains("hello buffered"),
            "expected buffered output to render, got: {content}"
        );
    }

    #[gpui::test]
    async fn test_terminal_output_and_exit_buffered_before_created(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| connection.new_thread(project, std::path::Path::new(path!("/test")), cx))
            .await
            .unwrap();

        let terminal_id = acp::TerminalId(uuid::Uuid::new_v4().to_string().into());

        // Send Output BEFORE Created
        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Output {
                    terminal_id: terminal_id.clone(),
                    data: b"pre-exit data".to_vec(),
                },
                cx,
            );
        });

        // Send Exit BEFORE Created
        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Exit {
                    terminal_id: terminal_id.clone(),
                    status: acp::TerminalExitStatus {
                        exit_code: Some(0),
                        signal: None,
                        meta: None,
                    },
                },
                cx,
            );
        });

        // Now create a display-only lower-level terminal and send Created
        let lower = cx.new(|cx| {
            let builder = ::terminal::TerminalBuilder::new_display_only(
                ::terminal::terminal_settings::CursorShape::default(),
                ::terminal::terminal_settings::AlternateScroll::On,
                None,
                0,
            )
            .unwrap();
            builder.subscribe(cx)
        });

        thread.update(cx, |thread, cx| {
            thread.on_terminal_provider_event(
                TerminalProviderEvent::Created {
                    terminal_id: terminal_id.clone(),
                    label: "Buffered Exit Test".to_string(),
                    cwd: None,
                    output_byte_limit: None,
                    terminal: lower.clone(),
                },
                cx,
            );
        });

        // Output should be present after Created (flushed from buffer)
        let content = thread.read_with(cx, |thread, cx| {
            let term = thread.terminal(terminal_id.clone()).unwrap();
            term.read_with(cx, |t, cx| t.inner().read(cx).get_content())
        });

        assert!(
            content.contains("pre-exit data"),
            "expected pre-exit data to render, got: {content}"
        );
    }

    #[gpui::test]
    async fn test_push_user_content_block(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| connection.new_thread(project, Path::new(path!("/test")), cx))
            .await
            .unwrap();

        // Test creating a new user message
        thread.update(cx, |thread, cx| {
            thread.push_user_content_block(
                None,
                acp::ContentBlock::Text(acp::TextContent {
                    annotations: None,
                    text: "Hello, ".to_string(),
                    meta: None,
                }),
                cx,
            );
        });

        thread.update(cx, |thread, cx| {
            assert_eq!(thread.entries.len(), 1);
            if let AgentThreadEntry::UserMessage(user_msg) = &thread.entries[0] {
                assert_eq!(user_msg.id, None);
                assert_eq!(user_msg.content.to_markdown(cx), "Hello, ");
            } else {
                panic!("Expected UserMessage");
            }
        });

        // Test appending to existing user message
        let message_1_id = UserMessageId::new();
        thread.update(cx, |thread, cx| {
            thread.push_user_content_block(
                Some(message_1_id.clone()),
                acp::ContentBlock::Text(acp::TextContent {
                    annotations: None,
                    text: "world!".to_string(),
                    meta: None,
                }),
                cx,
            );
        });

        thread.update(cx, |thread, cx| {
            assert_eq!(thread.entries.len(), 1);
            if let AgentThreadEntry::UserMessage(user_msg) = &thread.entries[0] {
                assert_eq!(user_msg.id, Some(message_1_id));
                assert_eq!(user_msg.content.to_markdown(cx), "Hello, world!");
            } else {
                panic!("Expected UserMessage");
            }
        });

        // Test creating new user message after assistant message
        thread.update(cx, |thread, cx| {
            thread.push_assistant_content_block(
                acp::ContentBlock::Text(acp::TextContent {
                    annotations: None,
                    text: "Assistant response".to_string(),
                    meta: None,
                }),
                false,
                cx,
            );
        });

        let message_2_id = UserMessageId::new();
        thread.update(cx, |thread, cx| {
            thread.push_user_content_block(
                Some(message_2_id.clone()),
                acp::ContentBlock::Text(acp::TextContent {
                    annotations: None,
                    text: "New user message".to_string(),
                    meta: None,
                }),
                cx,
            );
        });

        thread.update(cx, |thread, cx| {
            assert_eq!(thread.entries.len(), 3);
            if let AgentThreadEntry::UserMessage(user_msg) = &thread.entries[2] {
                assert_eq!(user_msg.id, Some(message_2_id));
                assert_eq!(user_msg.content.to_markdown(cx), "New user message");
            } else {
                panic!("Expected UserMessage at index 2");
            }
        });
    }

    #[gpui::test]
    async fn test_thinking_concatenation(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new().on_user_message(
            |_, thread, mut cx| {
                async move {
                    thread.update(&mut cx, |thread, cx| {
                        thread
                            .handle_session_update(
                                acp::SessionUpdate::AgentThoughtChunk(acp::ContentChunk {
                                    content: "Thinking ".into(),
                                    meta: None,
                                }),
                                cx,
                            )
                            .unwrap();
                        thread
                            .handle_session_update(
                                acp::SessionUpdate::AgentThoughtChunk(acp::ContentChunk {
                                    content: "hard!".into(),
                                    meta: None,
                                }),
                                cx,
                            )
                            .unwrap();
                    })?;
                    Ok(acp::PromptResponse {
                        stop_reason: acp::StopReason::EndTurn,
                        meta: None,
                    })
                }
                .boxed_local()
            },
        ));

        let thread = cx
            .update(|cx| connection.new_thread(project, Path::new(path!("/test")), cx))
            .await
            .unwrap();

        thread
            .update(cx, |thread, cx| thread.send_raw("Hello from Zed!", cx))
            .await
            .unwrap();

        let output = thread.read_with(cx, |thread, cx| thread.to_markdown(cx));
        assert_eq!(
            output,
            indoc! {r#"
            ## User

            Hello from Zed!

            ## Assistant

            <thinking>
            Thinking hard!
            </thinking>

            "#}
        );
    }

    #[gpui::test]
    async fn test_edits_concurrently_to_user(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/tmp"), json!({"foo": "one\ntwo\nthree\n"}))
            .await;
        let project = Project::test(fs.clone(), [], cx).await;
        let (read_file_tx, read_file_rx) = oneshot::channel::<()>();
        let read_file_tx = Rc::new(RefCell::new(Some(read_file_tx)));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message(
            move |_, thread, mut cx| {
                let read_file_tx = read_file_tx.clone();
                async move {
                    let content = thread
                        .update(&mut cx, |thread, cx| {
                            thread.read_text_file(path!("/tmp/foo").into(), None, None, false, cx)
                        })
                        .unwrap()
                        .await
                        .unwrap();
                    assert_eq!(content, "one\ntwo\nthree\n");
                    read_file_tx.take().unwrap().send(()).unwrap();
                    thread
                        .update(&mut cx, |thread, cx| {
                            thread.write_text_file(
                                path!("/tmp/foo").into(),
                                "one\ntwo\nthree\nfour\nfive\n".to_string(),
                                cx,
                            )
                        })
                        .unwrap()
                        .await
                        .unwrap();
                    Ok(acp::PromptResponse {
                        stop_reason: acp::StopReason::EndTurn,
                        meta: None,
                    })
                }
                .boxed_local()
            },
        ));

        let (worktree, pathbuf) = project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path!("/tmp/foo"), true, cx)
            })
            .await
            .unwrap();
        let buffer = project
            .update(cx, |project, cx| {
                project.open_buffer((worktree.read(cx).id(), pathbuf), cx)
            })
            .await
            .unwrap();

        let thread = cx
            .update(|cx| connection.new_thread(project, Path::new(path!("/tmp")), cx))
            .await
            .unwrap();

        let request = thread.update(cx, |thread, cx| {
            thread.send_raw("Extend the count in /tmp/foo", cx)
        });
        read_file_rx.await.ok();
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "zero\n".to_string())], None, cx);
        });
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "zero\none\ntwo\nthree\nfour\nfive\n"
        );
        assert_eq!(
            String::from_utf8(fs.read_file_sync(path!("/tmp/foo")).unwrap()).unwrap(),
            "zero\none\ntwo\nthree\nfour\nfive\n"
        );
        request.await.unwrap();
    }

    #[gpui::test]
    async fn test_reading_from_line(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/tmp"), json!({"foo": "one\ntwo\nthree\nfour\n"}))
            .await;
        let project = Project::test(fs.clone(), [], cx).await;
        project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path!("/tmp/foo"), true, cx)
            })
            .await
            .unwrap();

        let connection = Rc::new(FakeAgentConnection::new());

        let thread = cx
            .update(|cx| connection.new_thread(project, Path::new(path!("/tmp")), cx))
            .await
            .unwrap();

        // Whole file
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), None, None, false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "one\ntwo\nthree\nfour\n");

        // Only start line
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), Some(3), None, false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "three\nfour\n");

        // Only limit
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), None, Some(2), false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "one\ntwo\n");

        // Range
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), Some(2), Some(2), false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "two\nthree\n");

        // Invalid
        let err = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), Some(6), Some(2), false, cx)
            })
            .await
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            "Invalid params: \"Attempting to read beyond the end of the file, line 5:0\""
        );
    }

    #[gpui::test]
    async fn test_reading_empty_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/tmp"), json!({"foo": ""})).await;
        let project = Project::test(fs.clone(), [], cx).await;
        project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path!("/tmp/foo"), true, cx)
            })
            .await
            .unwrap();

        let connection = Rc::new(FakeAgentConnection::new());

        let thread = cx
            .update(|cx| connection.new_thread(project, Path::new(path!("/tmp")), cx))
            .await
            .unwrap();

        // Whole file
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), None, None, false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "");

        // Only start line
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), Some(1), None, false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "");

        // Only limit
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), None, Some(2), false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "");

        // Range
        let content = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), Some(1), Some(1), false, cx)
            })
            .await
            .unwrap();

        assert_eq!(content, "");

        // Invalid
        let err = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/tmp/foo").into(), Some(5), Some(2), false, cx)
            })
            .await
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            "Invalid params: \"Attempting to read beyond the end of the file, line 1:0\""
        );
    }
    #[gpui::test]
    async fn test_reading_non_existing_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/tmp"), json!({})).await;
        let project = Project::test(fs.clone(), [], cx).await;
        project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path!("/tmp"), true, cx)
            })
            .await
            .unwrap();

        let connection = Rc::new(FakeAgentConnection::new());

        let thread = cx
            .update(|cx| connection.new_thread(project, Path::new(path!("/tmp")), cx))
            .await
            .unwrap();

        // Out of project file
        let err = thread
            .update(cx, |thread, cx| {
                thread.read_text_file(path!("/foo").into(), None, None, false, cx)
            })
            .await
            .unwrap_err();

        assert_eq!(err.code, acp::ErrorCode::RESOURCE_NOT_FOUND.code);
    }

    #[gpui::test]
    async fn test_succeeding_canceled_toolcall(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let id = acp::ToolCallId("test".into());

        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let id = id.clone();
            move |_, thread, mut cx| {
                let id = id.clone();
                async move {
                    thread
                        .update(&mut cx, |thread, cx| {
                            thread.handle_session_update(
                                acp::SessionUpdate::ToolCall(acp::ToolCall {
                                    id: id.clone(),
                                    title: "Label".into(),
                                    kind: acp::ToolKind::Fetch,
                                    status: acp::ToolCallStatus::InProgress,
                                    content: vec![],
                                    locations: vec![],
                                    raw_input: None,
                                    raw_output: None,
                                    meta: None,
                                }),
                                cx,
                            )
                        })
                        .unwrap()
                        .unwrap();
                    Ok(acp::PromptResponse {
                        stop_reason: acp::StopReason::EndTurn,
                        meta: None,
                    })
                }
                .boxed_local()
            }
        }));

        let thread = cx
            .update(|cx| connection.new_thread(project, Path::new(path!("/test")), cx))
            .await
            .unwrap();

        let request = thread.update(cx, |thread, cx| {
            thread.send_raw("Fetch https://example.com", cx)
        });

        run_until_first_tool_call(&thread, cx).await;

        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                thread.entries[1],
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::InProgress,
                    ..
                })
            ));
        });

        thread.update(cx, |thread, cx| thread.cancel(cx)).await;

        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                &thread.entries[1],
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::Canceled,
                    ..
                })
            ));
        });

        thread
            .update(cx, |thread, cx| {
                thread.handle_session_update(
                    acp::SessionUpdate::ToolCallUpdate(acp::ToolCallUpdate {
                        id,
                        fields: acp::ToolCallUpdateFields {
                            status: Some(acp::ToolCallStatus::Completed),
                            ..Default::default()
                        },
                        meta: None,
                    }),
                    cx,
                )
            })
            .unwrap();

        request.await.unwrap();

        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                thread.entries[1],
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::Completed,
                    ..
                })
            ));
        });
    }

    #[gpui::test]
    async fn test_no_pending_edits_if_tool_calls_are_completed(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/test"), json!({})).await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;

        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            move |_, thread, mut cx| {
                async move {
                    thread
                        .update(&mut cx, |thread, cx| {
                            thread.handle_session_update(
                                acp::SessionUpdate::ToolCall(acp::ToolCall {
                                    id: acp::ToolCallId("test".into()),
                                    title: "Label".into(),
                                    kind: acp::ToolKind::Edit,
                                    status: acp::ToolCallStatus::Completed,
                                    content: vec![acp::ToolCallContent::Diff {
                                        diff: acp::Diff {
                                            path: "/test/test.txt".into(),
                                            old_text: None,
                                            new_text: "foo".into(),
                                            meta: None,
                                        },
                                    }],
                                    locations: vec![],
                                    raw_input: None,
                                    raw_output: None,
                                    meta: None,
                                }),
                                cx,
                            )
                        })
                        .unwrap()
                        .unwrap();
                    Ok(acp::PromptResponse {
                        stop_reason: acp::StopReason::EndTurn,
                        meta: None,
                    })
                }
                .boxed_local()
            }
        }));

        let thread = cx
            .update(|cx| connection.new_thread(project, Path::new(path!("/test")), cx))
            .await
            .unwrap();

        cx.update(|cx| thread.update(cx, |thread, cx| thread.send(vec!["Hi".into()], cx)))
            .await
            .unwrap();

        assert!(cx.read(|cx| !thread.read(cx).has_pending_edit_tool_calls()));
    }

    #[gpui::test(iterations = 10)]
    async fn test_checkpoints(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/test"),
            json!({
                ".git": {}
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

        let simulate_changes = Arc::new(AtomicBool::new(true));
        let next_filename = Arc::new(AtomicUsize::new(0));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let simulate_changes = simulate_changes.clone();
            let next_filename = next_filename.clone();
            let fs = fs.clone();
            move |request, thread, mut cx| {
                let fs = fs.clone();
                let simulate_changes = simulate_changes.clone();
                let next_filename = next_filename.clone();
                async move {
                    if simulate_changes.load(SeqCst) {
                        let filename = format!("/test/file-{}", next_filename.fetch_add(1, SeqCst));
                        fs.write(Path::new(&filename), b"").await?;
                    }

                    let acp::ContentBlock::Text(content) = &request.prompt[0] else {
                        panic!("expected text content block");
                    };
                    thread.update(&mut cx, |thread, cx| {
                        thread
                            .handle_session_update(
                                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk {
                                    content: content.text.to_uppercase().into(),
                                    meta: None,
                                }),
                                cx,
                            )
                            .unwrap();
                    })?;
                    Ok(acp::PromptResponse {
                        stop_reason: acp::StopReason::EndTurn,
                        meta: None,
                    })
                }
                .boxed_local()
            }
        }));
        let thread = cx
            .update(|cx| connection.new_thread(project, Path::new(path!("/test")), cx))
            .await
            .unwrap();

        cx.update(|cx| thread.update(cx, |thread, cx| thread.send(vec!["Lorem".into()], cx)))
            .await
            .unwrap();
        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User (checkpoint)

                    Lorem

                    ## Assistant

                    LOREM

                "}
            );
        });
        assert_eq!(fs.files(), vec![Path::new(path!("/test/file-0"))]);

        cx.update(|cx| thread.update(cx, |thread, cx| thread.send(vec!["ipsum".into()], cx)))
            .await
            .unwrap();
        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User (checkpoint)

                    Lorem

                    ## Assistant

                    LOREM

                    ## User (checkpoint)

                    ipsum

                    ## Assistant

                    IPSUM

                "}
            );
        });
        assert_eq!(
            fs.files(),
            vec![
                Path::new(path!("/test/file-0")),
                Path::new(path!("/test/file-1"))
            ]
        );

        // Checkpoint isn't stored when there are no changes.
        simulate_changes.store(false, SeqCst);
        cx.update(|cx| thread.update(cx, |thread, cx| thread.send(vec!["dolor".into()], cx)))
            .await
            .unwrap();
        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User (checkpoint)

                    Lorem

                    ## Assistant

                    LOREM

                    ## User (checkpoint)

                    ipsum

                    ## Assistant

                    IPSUM

                    ## User

                    dolor

                    ## Assistant

                    DOLOR

                "}
            );
        });
        assert_eq!(
            fs.files(),
            vec![
                Path::new(path!("/test/file-0")),
                Path::new(path!("/test/file-1"))
            ]
        );

        // Rewinding the conversation truncates the history and restores the checkpoint.
        thread
            .update(cx, |thread, cx| {
                let AgentThreadEntry::UserMessage(message) = &thread.entries[2] else {
                    panic!("unexpected entries {:?}", thread.entries)
                };
                thread.restore_checkpoint(message.id.clone().unwrap(), cx)
            })
            .await
            .unwrap();
        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User (checkpoint)

                    Lorem

                    ## Assistant

                    LOREM

                "}
            );
        });
        assert_eq!(fs.files(), vec![Path::new(path!("/test/file-0"))]);
    }

    #[gpui::test]
    async fn test_tool_result_refusal(cx: &mut TestAppContext) {
        use std::sync::atomic::AtomicUsize;
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, None, cx).await;

        // Create a connection that simulates refusal after tool result
        let prompt_count = Arc::new(AtomicUsize::new(0));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let prompt_count = prompt_count.clone();
            move |_request, thread, mut cx| {
                let count = prompt_count.fetch_add(1, SeqCst);
                async move {
                    if count == 0 {
                        // First prompt: Generate a tool call with result
                        thread.update(&mut cx, |thread, cx| {
                            thread
                                .handle_session_update(
                                    acp::SessionUpdate::ToolCall(acp::ToolCall {
                                        id: acp::ToolCallId("tool1".into()),
                                        title: "Test Tool".into(),
                                        kind: acp::ToolKind::Fetch,
                                        status: acp::ToolCallStatus::Completed,
                                        content: vec![],
                                        locations: vec![],
                                        raw_input: Some(serde_json::json!({"query": "test"})),
                                        raw_output: Some(
                                            serde_json::json!({"result": "inappropriate content"}),
                                        ),
                                        meta: None,
                                    }),
                                    cx,
                                )
                                .unwrap();
                        })?;

                        // Now return refusal because of the tool result
                        Ok(acp::PromptResponse {
                            stop_reason: acp::StopReason::Refusal,
                            meta: None,
                        })
                    } else {
                        Ok(acp::PromptResponse {
                            stop_reason: acp::StopReason::EndTurn,
                            meta: None,
                        })
                    }
                }
                .boxed_local()
            }
        }));

        let thread = cx
            .update(|cx| connection.new_thread(project, Path::new(path!("/test")), cx))
            .await
            .unwrap();

        // Track if we see a Refusal event
        let saw_refusal_event = Arc::new(std::sync::Mutex::new(false));
        let saw_refusal_event_captured = saw_refusal_event.clone();
        thread.update(cx, |_thread, cx| {
            cx.subscribe(
                &thread,
                move |_thread, _event_thread, event: &AcpThreadEvent, _cx| {
                    if matches!(event, AcpThreadEvent::Refusal) {
                        *saw_refusal_event_captured.lock().unwrap() = true;
                    }
                },
            )
            .detach();
        });

        // Send a user message - this will trigger tool call and then refusal
        let send_task = thread.update(cx, |thread, cx| {
            thread.send(
                vec![acp::ContentBlock::Text(acp::TextContent {
                    text: "Hello".into(),
                    annotations: None,
                    meta: None,
                })],
                cx,
            )
        });
        cx.background_executor.spawn(send_task).detach();
        cx.run_until_parked();

        // Verify that:
        // 1. A Refusal event WAS emitted (because it's a tool result refusal, not user prompt)
        // 2. The user message was NOT truncated
        assert!(
            *saw_refusal_event.lock().unwrap(),
            "Refusal event should be emitted for tool result refusals"
        );

        thread.read_with(cx, |thread, _| {
            let entries = thread.entries();
            assert!(entries.len() >= 2, "Should have user message and tool call");

            // Verify user message is still there
            assert!(
                matches!(entries[0], AgentThreadEntry::UserMessage(_)),
                "User message should not be truncated"
            );

            // Verify tool call is there with result
            if let AgentThreadEntry::ToolCall(tool_call) = &entries[1] {
                assert!(
                    tool_call.raw_output.is_some(),
                    "Tool call should have output"
                );
            } else {
                panic!("Expected tool call at index 1");
            }
        });
    }

    #[gpui::test]
    async fn test_user_prompt_refusal_emits_event(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, None, cx).await;

        let refuse_next = Arc::new(AtomicBool::new(false));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let refuse_next = refuse_next.clone();
            move |_request, _thread, _cx| {
                if refuse_next.load(SeqCst) {
                    async move {
                        Ok(acp::PromptResponse {
                            stop_reason: acp::StopReason::Refusal,
                            meta: None,
                        })
                    }
                    .boxed_local()
                } else {
                    async move {
                        Ok(acp::PromptResponse {
                            stop_reason: acp::StopReason::EndTurn,
                            meta: None,
                        })
                    }
                    .boxed_local()
                }
            }
        }));

        let thread = cx
            .update(|cx| connection.new_thread(project, Path::new(path!("/test")), cx))
            .await
            .unwrap();

        // Track if we see a Refusal event
        let saw_refusal_event = Arc::new(std::sync::Mutex::new(false));
        let saw_refusal_event_captured = saw_refusal_event.clone();
        thread.update(cx, |_thread, cx| {
            cx.subscribe(
                &thread,
                move |_thread, _event_thread, event: &AcpThreadEvent, _cx| {
                    if matches!(event, AcpThreadEvent::Refusal) {
                        *saw_refusal_event_captured.lock().unwrap() = true;
                    }
                },
            )
            .detach();
        });

        // Send a message that will be refused
        refuse_next.store(true, SeqCst);
        cx.update(|cx| thread.update(cx, |thread, cx| thread.send(vec!["hello".into()], cx)))
            .await
            .unwrap();

        // Verify that a Refusal event WAS emitted for user prompt refusal
        assert!(
            *saw_refusal_event.lock().unwrap(),
            "Refusal event should be emitted for user prompt refusals"
        );

        // Verify the message was truncated (user prompt refusal)
        thread.read_with(cx, |thread, cx| {
            assert_eq!(thread.to_markdown(cx), "");
        });
    }

    #[gpui::test]
    async fn test_refusal(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/"), json!({})).await;
        let project = Project::test(fs.clone(), [path!("/").as_ref()], cx).await;

        let refuse_next = Arc::new(AtomicBool::new(false));
        let connection = Rc::new(FakeAgentConnection::new().on_user_message({
            let refuse_next = refuse_next.clone();
            move |request, thread, mut cx| {
                let refuse_next = refuse_next.clone();
                async move {
                    if refuse_next.load(SeqCst) {
                        return Ok(acp::PromptResponse {
                            stop_reason: acp::StopReason::Refusal,
                            meta: None,
                        });
                    }

                    let acp::ContentBlock::Text(content) = &request.prompt[0] else {
                        panic!("expected text content block");
                    };
                    thread.update(&mut cx, |thread, cx| {
                        thread
                            .handle_session_update(
                                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk {
                                    content: content.text.to_uppercase().into(),
                                    meta: None,
                                }),
                                cx,
                            )
                            .unwrap();
                    })?;
                    Ok(acp::PromptResponse {
                        stop_reason: acp::StopReason::EndTurn,
                        meta: None,
                    })
                }
                .boxed_local()
            }
        }));
        let thread = cx
            .update(|cx| connection.new_thread(project, Path::new(path!("/test")), cx))
            .await
            .unwrap();

        cx.update(|cx| thread.update(cx, |thread, cx| thread.send(vec!["hello".into()], cx)))
            .await
            .unwrap();
        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User

                    hello

                    ## Assistant

                    HELLO

                "}
            );
        });

        // Simulate refusing the second message. The message should be truncated
        // when a user prompt is refused.
        refuse_next.store(true, SeqCst);
        cx.update(|cx| thread.update(cx, |thread, cx| thread.send(vec!["world".into()], cx)))
            .await
            .unwrap();
        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User

                    hello

                    ## Assistant

                    HELLO

                "}
            );
        });
    }

    async fn run_until_first_tool_call(
        thread: &Entity<AcpThread>,
        cx: &mut TestAppContext,
    ) -> usize {
        let (mut tx, mut rx) = mpsc::channel::<usize>(1);

        let subscription = cx.update(|cx| {
            cx.subscribe(thread, move |thread, _, cx| {
                for (ix, entry) in thread.read(cx).entries.iter().enumerate() {
                    if matches!(entry, AgentThreadEntry::ToolCall(_)) {
                        return tx.try_send(ix).unwrap();
                    }
                }
            })
        });

        select! {
            _ = futures::FutureExt::fuse(smol::Timer::after(Duration::from_secs(10))) => {
                panic!("Timeout waiting for tool call")
            }
            ix = rx.next().fuse() => {
                drop(subscription);
                ix.unwrap()
            }
        }
    }

    #[derive(Clone, Default)]
    struct FakeAgentConnection {
        auth_methods: Vec<acp::AuthMethod>,
        sessions: Arc<parking_lot::Mutex<HashMap<acp::SessionId, WeakEntity<AcpThread>>>>,
        on_user_message: Option<
            Rc<
                dyn Fn(
                        acp::PromptRequest,
                        WeakEntity<AcpThread>,
                        AsyncApp,
                    ) -> LocalBoxFuture<'static, Result<acp::PromptResponse>>
                    + 'static,
            >,
        >,
    }

    impl FakeAgentConnection {
        fn new() -> Self {
            Self {
                auth_methods: Vec::new(),
                on_user_message: None,
                sessions: Arc::default(),
            }
        }

        #[expect(unused)]
        fn with_auth_methods(mut self, auth_methods: Vec<acp::AuthMethod>) -> Self {
            self.auth_methods = auth_methods;
            self
        }

        fn on_user_message(
            mut self,
            handler: impl Fn(
                acp::PromptRequest,
                WeakEntity<AcpThread>,
                AsyncApp,
            ) -> LocalBoxFuture<'static, Result<acp::PromptResponse>>
            + 'static,
        ) -> Self {
            self.on_user_message.replace(Rc::new(handler));
            self
        }
    }

    impl AgentConnection for FakeAgentConnection {
        fn auth_methods(&self) -> &[acp::AuthMethod] {
            &self.auth_methods
        }

        fn new_thread(
            self: Rc<Self>,
            project: Entity<Project>,
            _cwd: &Path,
            cx: &mut App,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            let session_id = acp::SessionId(
                rand::rng()
                    .sample_iter(&distr::Alphanumeric)
                    .take(7)
                    .map(char::from)
                    .collect::<String>()
                    .into(),
            );
            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            let thread = cx.new(|cx| {
                AcpThread::new(
                    "Test",
                    self.clone(),
                    project,
                    action_log,
                    session_id.clone(),
                    watch::Receiver::constant(acp::PromptCapabilities {
                        image: true,
                        audio: true,
                        embedded_context: true,
                        meta: None,
                    }),
                    cx,
                )
            });
            self.sessions.lock().insert(session_id, thread.downgrade());
            Task::ready(Ok(thread))
        }

        fn authenticate(&self, method: acp::AuthMethodId, _cx: &mut App) -> Task<gpui::Result<()>> {
            if self.auth_methods().iter().any(|m| m.id == method) {
                Task::ready(Ok(()))
            } else {
                Task::ready(Err(anyhow!("Invalid Auth Method")))
            }
        }

        fn prompt(
            &self,
            _id: Option<UserMessageId>,
            params: acp::PromptRequest,
            cx: &mut App,
        ) -> Task<gpui::Result<acp::PromptResponse>> {
            let sessions = self.sessions.lock();
            let thread = sessions.get(&params.session_id).unwrap();
            if let Some(handler) = &self.on_user_message {
                let handler = handler.clone();
                let thread = thread.clone();
                cx.spawn(async move |cx| handler(params, thread, cx.clone()).await)
            } else {
                Task::ready(Ok(acp::PromptResponse {
                    stop_reason: acp::StopReason::EndTurn,
                    meta: None,
                }))
            }
        }

        fn cancel(&self, session_id: &acp::SessionId, cx: &mut App) {
            let sessions = self.sessions.lock();
            let thread = sessions.get(session_id).unwrap().clone();

            cx.spawn(async move |cx| {
                thread
                    .update(cx, |thread, cx| thread.cancel(cx))
                    .unwrap()
                    .await
            })
            .detach();
        }

        fn truncate(
            &self,
            session_id: &acp::SessionId,
            _cx: &App,
        ) -> Option<Rc<dyn AgentSessionTruncate>> {
            Some(Rc::new(FakeAgentSessionEditor {
                _session_id: session_id.clone(),
            }))
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    struct FakeAgentSessionEditor {
        _session_id: acp::SessionId,
    }

    impl AgentSessionTruncate for FakeAgentSessionEditor {
        fn run(&self, _message_id: UserMessageId, _cx: &mut App) -> Task<Result<()>> {
            Task::ready(Ok(()))
        }
    }

    #[gpui::test]
    async fn test_tool_call_not_found_creates_failed_entry(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .update(|cx| connection.new_thread(project, Path::new(path!("/test")), cx))
            .await
            .unwrap();

        // Try to update a tool call that doesn't exist
        let nonexistent_id = acp::ToolCallId("nonexistent-tool-call".into());
        thread.update(cx, |thread, cx| {
            let result = thread.handle_session_update(
                acp::SessionUpdate::ToolCallUpdate(acp::ToolCallUpdate {
                    id: nonexistent_id.clone(),
                    fields: acp::ToolCallUpdateFields {
                        status: Some(acp::ToolCallStatus::Completed),
                        ..Default::default()
                    },
                    meta: None,
                }),
                cx,
            );

            // The update should succeed (not return an error)
            assert!(result.is_ok());

            // There should now be exactly one entry in the thread
            assert_eq!(thread.entries.len(), 1);

            // The entry should be a failed tool call
            if let AgentThreadEntry::ToolCall(tool_call) = &thread.entries[0] {
                assert_eq!(tool_call.id, nonexistent_id);
                assert!(matches!(tool_call.status, ToolCallStatus::Failed));
                assert_eq!(tool_call.kind, acp::ToolKind::Fetch);

                // Check that the content contains the error message
                assert_eq!(tool_call.content.len(), 1);
                if let ToolCallContent::ContentBlock(content_block) = &tool_call.content[0] {
                    match content_block {
                        ContentBlock::Markdown { markdown } => {
                            let markdown_text = markdown.read(cx).source();
                            assert!(markdown_text.contains("Tool call not found"));
                        }
                        ContentBlock::Empty => panic!("Expected markdown content, got empty"),
                        ContentBlock::ResourceLink { .. } => {
                            panic!("Expected markdown content, got resource link")
                        }
                    }
                } else {
                    panic!("Expected ContentBlock, got: {:?}", tool_call.content[0]);
                }
            } else {
                panic!("Expected ToolCall entry, got: {:?}", thread.entries[0]);
            }
        });
    }
}
