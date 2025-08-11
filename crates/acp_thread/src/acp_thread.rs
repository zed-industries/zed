mod connection;
mod diff;

pub use connection::*;
pub use diff::*;

use action_log::ActionLog;
use agent_client_protocol as acp;
use anyhow::{Context as _, Result};
use editor::Bias;
use futures::{FutureExt, channel::oneshot, future::BoxFuture};
use gpui::{AppContext, Context, Entity, EventEmitter, SharedString, Task};
use itertools::Itertools;
use language::{Anchor, Buffer, BufferSnapshot, LanguageRegistry, Point, text_diff};
use markdown::Markdown;
use project::{AgentLocation, Project};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Formatter;
use std::process::ExitStatus;
use std::rc::Rc;
use std::{
    fmt::Display,
    mem,
    path::{Path, PathBuf},
    sync::Arc,
};
use ui::App;
use util::ResultExt;

#[derive(Debug)]
pub struct UserMessage {
    pub content: ContentBlock,
}

impl UserMessage {
    pub fn from_acp(
        message: impl IntoIterator<Item = acp::ContentBlock>,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        let mut content = ContentBlock::Empty;
        for chunk in message {
            content.append(chunk, &language_registry, cx)
        }
        Self { content: content }
    }

    fn to_markdown(&self, cx: &App) -> String {
        format!("## User\n\n{}\n\n", self.content.to_markdown(cx))
    }
}

#[derive(Debug)]
pub struct MentionPath<'a>(&'a Path);

impl<'a> MentionPath<'a> {
    const PREFIX: &'static str = "@file:";

    pub fn new(path: &'a Path) -> Self {
        MentionPath(path)
    }

    pub fn try_parse(url: &'a str) -> Option<Self> {
        let path = url.strip_prefix(Self::PREFIX)?;
        Some(MentionPath(Path::new(path)))
    }

    pub fn path(&self) -> &Path {
        self.0
    }
}

impl Display for MentionPath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[@{}]({}{})",
            self.0.file_name().unwrap_or_default().display(),
            Self::PREFIX,
            self.0.display()
        )
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
    pub fn from_str(chunk: &str, language_registry: &Arc<LanguageRegistry>, cx: &mut App) -> Self {
        Self::Message {
            block: ContentBlock::new(chunk.into(), language_registry, cx),
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
    fn to_markdown(&self, cx: &App) -> String {
        match self {
            Self::UserMessage(message) => message.to_markdown(cx),
            Self::AssistantMessage(message) => message.to_markdown(cx),
            Self::ToolCall(tool_call) => tool_call.to_markdown(cx),
        }
    }

    pub fn diffs(&self) -> impl Iterator<Item = &Entity<Diff>> {
        if let AgentThreadEntry::ToolCall(call) = self {
            itertools::Either::Left(call.diffs())
        } else {
            itertools::Either::Right(std::iter::empty())
        }
    }

    pub fn locations(&self) -> Option<&[acp::ToolCallLocation]> {
        if let AgentThreadEntry::ToolCall(ToolCall { locations, .. }) = self {
            Some(locations)
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
    pub raw_input: Option<serde_json::Value>,
    pub raw_output: Option<serde_json::Value>,
}

impl ToolCall {
    fn from_acp(
        tool_call: acp::ToolCall,
        status: ToolCallStatus,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        Self {
            id: tool_call.id,
            label: cx.new(|cx| {
                Markdown::new(
                    tool_call.title.into(),
                    Some(language_registry.clone()),
                    None,
                    cx,
                )
            }),
            kind: tool_call.kind,
            content: tool_call
                .content
                .into_iter()
                .map(|content| ToolCallContent::from_acp(content, language_registry.clone(), cx))
                .collect(),
            locations: tool_call.locations,
            status,
            raw_input: tool_call.raw_input,
            raw_output: tool_call.raw_output,
        }
    }

    fn update_fields(
        &mut self,
        fields: acp::ToolCallUpdateFields,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) {
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
            self.status = ToolCallStatus::Allowed { status };
        }

        if let Some(title) = title {
            self.label.update(cx, |label, cx| {
                label.replace(title, cx);
            });
        }

        if let Some(content) = content {
            self.content = content
                .into_iter()
                .map(|chunk| ToolCallContent::from_acp(chunk, language_registry.clone(), cx))
                .collect();
        }

        if let Some(locations) = locations {
            self.locations = locations;
        }

        if let Some(raw_input) = raw_input {
            self.raw_input = Some(raw_input);
        }

        if let Some(raw_output) = raw_output {
            self.raw_output = Some(raw_output);
        }
    }

    pub fn diffs(&self) -> impl Iterator<Item = &Entity<Diff>> {
        self.content.iter().filter_map(|content| match content {
            ToolCallContent::ContentBlock { .. } => None,
            ToolCallContent::Diff { diff } => Some(diff),
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
}

#[derive(Debug)]
pub enum ToolCallStatus {
    WaitingForConfirmation {
        options: Vec<acp::PermissionOption>,
        respond_tx: oneshot::Sender<acp::PermissionOptionId>,
    },
    Allowed {
        status: acp::ToolCallStatus,
    },
    Rejected,
    Canceled,
}

impl Display for ToolCallStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ToolCallStatus::WaitingForConfirmation { .. } => "Waiting for confirmation",
                ToolCallStatus::Allowed { status } => match status {
                    acp::ToolCallStatus::Pending => "Pending",
                    acp::ToolCallStatus::InProgress => "In Progress",
                    acp::ToolCallStatus::Completed => "Completed",
                    acp::ToolCallStatus::Failed => "Failed",
                },
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
}

impl ContentBlock {
    pub fn new(
        block: acp::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        let mut this = Self::Empty;
        this.append(block, language_registry, cx);
        this
    }

    pub fn new_combined(
        blocks: impl IntoIterator<Item = acp::ContentBlock>,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        let mut this = Self::Empty;
        for block in blocks {
            this.append(block, &language_registry, cx);
        }
        this
    }

    pub fn append(
        &mut self,
        block: acp::ContentBlock,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) {
        let new_content = match block {
            acp::ContentBlock::Text(text_content) => text_content.text.clone(),
            acp::ContentBlock::ResourceLink(resource_link) => {
                if let Some(path) = resource_link.uri.strip_prefix("file://") {
                    format!("{}", MentionPath(path.as_ref()))
                } else {
                    resource_link.uri.clone()
                }
            }
            acp::ContentBlock::Image(_)
            | acp::ContentBlock::Audio(_)
            | acp::ContentBlock::Resource(_) => String::new(),
        };

        match self {
            ContentBlock::Empty => {
                *self = ContentBlock::Markdown {
                    markdown: cx.new(|cx| {
                        Markdown::new(
                            new_content.into(),
                            Some(language_registry.clone()),
                            None,
                            cx,
                        )
                    }),
                };
            }
            ContentBlock::Markdown { markdown } => {
                markdown.update(cx, |markdown, cx| markdown.append(&new_content, cx));
            }
        }
    }

    fn to_markdown<'a>(&'a self, cx: &'a App) -> &'a str {
        match self {
            ContentBlock::Empty => "",
            ContentBlock::Markdown { markdown } => markdown.read(cx).source(),
        }
    }

    pub fn markdown(&self) -> Option<&Entity<Markdown>> {
        match self {
            ContentBlock::Empty => None,
            ContentBlock::Markdown { markdown } => Some(markdown),
        }
    }
}

#[derive(Debug)]
pub enum ToolCallContent {
    ContentBlock { content: ContentBlock },
    Diff { diff: Entity<Diff> },
}

impl ToolCallContent {
    pub fn from_acp(
        content: acp::ToolCallContent,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        match content {
            acp::ToolCallContent::Content { content } => Self::ContentBlock {
                content: ContentBlock::new(content, &language_registry, cx),
            },
            acp::ToolCallContent::Diff { diff } => Self::Diff {
                diff: cx.new(|cx| Diff::from_acp(diff, language_registry, cx)),
            },
        }
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        match self {
            Self::ContentBlock { content } => content.to_markdown(cx).to_string(),
            Self::Diff { diff } => diff.read(cx).to_markdown(cx),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ToolCallUpdate {
    UpdateFields(acp::ToolCallUpdate),
    UpdateDiff(ToolCallUpdateDiff),
}

impl ToolCallUpdate {
    fn id(&self) -> &acp::ToolCallId {
        match self {
            Self::UpdateFields(update) => &update.id,
            Self::UpdateDiff(diff) => &diff.id,
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
}

pub enum AcpThreadEvent {
    NewEntry,
    EntryUpdated(usize),
    ToolAuthorizationRequired,
    Stopped,
    Error,
    ServerExited(ExitStatus),
}

impl EventEmitter<AcpThreadEvent> for AcpThread {}

#[derive(PartialEq, Eq)]
pub enum ThreadStatus {
    Idle,
    WaitingForToolConfirmation,
    Generating,
}

#[derive(Debug, Clone)]
pub enum LoadError {
    Unsupported {
        error_message: SharedString,
        upgrade_message: SharedString,
        upgrade_command: String,
    },
    Exited(i32),
    Other(SharedString),
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Unsupported { error_message, .. } => write!(f, "{}", error_message),
            LoadError::Exited(status) => write!(f, "Server exited with status {}", status),
            LoadError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for LoadError {}

impl AcpThread {
    pub fn new(
        title: impl Into<SharedString>,
        connection: Rc<dyn AgentConnection>,
        project: Entity<Project>,
        session_id: acp::SessionId,
        cx: &mut Context<Self>,
    ) -> Self {
        let action_log = cx.new(|_| ActionLog::new(project.clone()));

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
        }
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
            if self.waiting_for_tool_confirmation() {
                ThreadStatus::WaitingForToolConfirmation
            } else {
                ThreadStatus::Generating
            }
        } else {
            ThreadStatus::Idle
        }
    }

    pub fn has_pending_edit_tool_calls(&self) -> bool {
        for entry in self.entries.iter().rev() {
            match entry {
                AgentThreadEntry::UserMessage(_) => return false,
                AgentThreadEntry::ToolCall(
                    call @ ToolCall {
                        status:
                            ToolCallStatus::Allowed {
                                status:
                                    acp::ToolCallStatus::InProgress | acp::ToolCallStatus::Pending,
                            },
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
    ) -> Result<()> {
        match update {
            acp::SessionUpdate::UserMessageChunk { content } => {
                self.push_user_content_block(content, cx);
            }
            acp::SessionUpdate::AgentMessageChunk { content } => {
                self.push_assistant_content_block(content, false, cx);
            }
            acp::SessionUpdate::AgentThoughtChunk { content } => {
                self.push_assistant_content_block(content, true, cx);
            }
            acp::SessionUpdate::ToolCall(tool_call) => {
                self.upsert_tool_call(tool_call, cx);
            }
            acp::SessionUpdate::ToolCallUpdate(tool_call_update) => {
                self.update_tool_call(tool_call_update, cx)?;
            }
            acp::SessionUpdate::Plan(plan) => {
                self.update_plan(plan, cx);
            }
        }
        Ok(())
    }

    pub fn push_user_content_block(&mut self, chunk: acp::ContentBlock, cx: &mut Context<Self>) {
        let language_registry = self.project.read(cx).languages().clone();
        let entries_len = self.entries.len();

        if let Some(last_entry) = self.entries.last_mut()
            && let AgentThreadEntry::UserMessage(UserMessage { content }) = last_entry
        {
            content.append(chunk, &language_registry, cx);
            cx.emit(AcpThreadEvent::EntryUpdated(entries_len - 1));
        } else {
            let content = ContentBlock::new(chunk, &language_registry, cx);
            self.push_entry(AgentThreadEntry::UserMessage(UserMessage { content }), cx);
        }
    }

    pub fn push_assistant_content_block(
        &mut self,
        chunk: acp::ContentBlock,
        is_thought: bool,
        cx: &mut Context<Self>,
    ) {
        let language_registry = self.project.read(cx).languages().clone();
        let entries_len = self.entries.len();
        if let Some(last_entry) = self.entries.last_mut()
            && let AgentThreadEntry::AssistantMessage(AssistantMessage { chunks }) = last_entry
        {
            cx.emit(AcpThreadEvent::EntryUpdated(entries_len - 1));
            match (chunks.last_mut(), is_thought) {
                (Some(AssistantMessageChunk::Message { block }), false)
                | (Some(AssistantMessageChunk::Thought { block }), true) => {
                    block.append(chunk, &language_registry, cx)
                }
                _ => {
                    let block = ContentBlock::new(chunk, &language_registry, cx);
                    if is_thought {
                        chunks.push(AssistantMessageChunk::Thought { block })
                    } else {
                        chunks.push(AssistantMessageChunk::Message { block })
                    }
                }
            }
        } else {
            let block = ContentBlock::new(chunk, &language_registry, cx);
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

    pub fn update_tool_call(
        &mut self,
        update: impl Into<ToolCallUpdate>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let update = update.into();
        let languages = self.project.read(cx).languages().clone();

        let (ix, current_call) = self
            .tool_call_mut(update.id())
            .context("Tool call not found")?;
        match update {
            ToolCallUpdate::UpdateFields(update) => {
                current_call.update_fields(update.fields, languages, cx);
            }
            ToolCallUpdate::UpdateDiff(update) => {
                current_call.content.clear();
                current_call
                    .content
                    .push(ToolCallContent::Diff { diff: update.diff });
            }
        }

        cx.emit(AcpThreadEvent::EntryUpdated(ix));

        Ok(())
    }

    /// Updates a tool call if id matches an existing entry, otherwise inserts a new one.
    pub fn upsert_tool_call(&mut self, tool_call: acp::ToolCall, cx: &mut Context<Self>) {
        let status = ToolCallStatus::Allowed {
            status: tool_call.status,
        };
        self.upsert_tool_call_inner(tool_call, status, cx)
    }

    pub fn upsert_tool_call_inner(
        &mut self,
        tool_call: acp::ToolCall,
        status: ToolCallStatus,
        cx: &mut Context<Self>,
    ) {
        let language_registry = self.project.read(cx).languages().clone();
        let call = ToolCall::from_acp(tool_call, status, language_registry, cx);

        let location = call.locations.last().cloned();

        if let Some((ix, current_call)) = self.tool_call_mut(&call.id) {
            *current_call = call;

            cx.emit(AcpThreadEvent::EntryUpdated(ix));
        } else {
            self.push_entry(AgentThreadEntry::ToolCall(call), cx);
        }

        if let Some(location) = location {
            self.set_project_location(location, cx)
        }
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

    pub fn set_project_location(&self, location: acp::ToolCallLocation, cx: &mut Context<Self>) {
        self.project.update(cx, |project, cx| {
            let Some(path) = project.project_path_for_absolute_path(&location.path, cx) else {
                return;
            };
            let buffer = project.open_buffer(path, cx);
            cx.spawn(async move |project, cx| {
                let buffer = buffer.await?;

                project.update(cx, |project, cx| {
                    let position = if let Some(line) = location.line {
                        let snapshot = buffer.read(cx).snapshot();
                        let point = snapshot.clip_point(Point::new(line, 0), Bias::Left);
                        snapshot.anchor_before(point)
                    } else {
                        Anchor::MIN
                    };

                    project.set_agent_location(
                        Some(AgentLocation {
                            buffer: buffer.downgrade(),
                            position,
                        }),
                        cx,
                    );
                })
            })
            .detach_and_log_err(cx);
        });
    }

    pub fn request_tool_call_authorization(
        &mut self,
        tool_call: acp::ToolCall,
        options: Vec<acp::PermissionOption>,
        cx: &mut Context<Self>,
    ) -> oneshot::Receiver<acp::PermissionOptionId> {
        let (tx, rx) = oneshot::channel();

        let status = ToolCallStatus::WaitingForConfirmation {
            options,
            respond_tx: tx,
        };

        self.upsert_tool_call_inner(tool_call, status, cx);
        cx.emit(AcpThreadEvent::ToolAuthorizationRequired);
        rx
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
                ToolCallStatus::Allowed {
                    status: acp::ToolCallStatus::InProgress,
                }
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

    /// Returns true if the last turn is awaiting tool authorization
    pub fn waiting_for_tool_confirmation(&self) -> bool {
        for entry in self.entries.iter().rev() {
            match &entry {
                AgentThreadEntry::ToolCall(call) => match call.status {
                    ToolCallStatus::WaitingForConfirmation { .. } => return true,
                    ToolCallStatus::Allowed { .. }
                    | ToolCallStatus::Rejected
                    | ToolCallStatus::Canceled => continue,
                },
                AgentThreadEntry::UserMessage(_) | AgentThreadEntry::AssistantMessage(_) => {
                    // Reached the beginning of the turn
                    return false;
                }
            }
        }
        false
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
            cx,
        );
        self.push_entry(
            AgentThreadEntry::UserMessage(UserMessage { content: block }),
            cx,
        );
        self.clear_completed_plan_entries(cx);

        let (tx, rx) = oneshot::channel();
        let cancel_task = self.cancel(cx);

        self.send_task = Some(cx.spawn(async move |this, cx| {
            async {
                cancel_task.await;

                let result = this
                    .update(cx, |this, cx| {
                        this.connection.prompt(
                            acp::PromptRequest {
                                prompt: message,
                                session_id: this.session_id.clone(),
                            },
                            cx,
                        )
                    })?
                    .await;

                tx.send(result).log_err();

                anyhow::Ok(())
            }
            .await
            .log_err();
        }));

        cx.spawn(async move |this, cx| match rx.await {
            Ok(Err(e)) => {
                this.update(cx, |_, cx| cx.emit(AcpThreadEvent::Error))
                    .log_err();
                Err(e)?
            }
            result => {
                let cancelled = matches!(
                    result,
                    Ok(Ok(acp::PromptResponse {
                        stop_reason: acp::StopReason::Cancelled
                    }))
                );

                // We only take the task if the current prompt wasn't cancelled.
                //
                // This prompt may have been cancelled because another one was sent
                // while it was still generating. In these cases, dropping `send_task`
                // would cause the next generation to be cancelled.
                if !cancelled {
                    this.update(cx, |this, _cx| this.send_task.take()).ok();
                }

                this.update(cx, |_, cx| cx.emit(AcpThreadEvent::Stopped))
                    .log_err();
                Ok(())
            }
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
                    ToolCallStatus::WaitingForConfirmation { .. }
                        | ToolCallStatus::Allowed {
                            status: acp::ToolCallStatus::InProgress
                        }
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

    pub fn read_text_file(
        &self,
        path: PathBuf,
        line: Option<u32>,
        limit: Option<u32>,
        reuse_shared_snapshot: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<String>> {
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
                project.update(cx, |project, cx| {
                    let position = buffer
                        .read(cx)
                        .snapshot()
                        .anchor_before(Point::new(line.unwrap_or_default(), 0));
                    project.set_agent_location(
                        Some(AgentLocation {
                            buffer: buffer.downgrade(),
                            position,
                        }),
                        cx,
                    );
                })?;

                buffer.update(cx, |buffer, _| buffer.snapshot())?
            };

            this.update(cx, |this, _| {
                let text = snapshot.text();
                this.shared_buffers.insert(buffer.clone(), snapshot);
                if line.is_none() && limit.is_none() {
                    return Ok(text);
                }
                let limit = limit.unwrap_or(u32::MAX) as usize;
                let Some(line) = line else {
                    return Ok(text.lines().take(limit).collect::<String>());
                };

                let count = text.lines().count();
                if count < line as usize {
                    anyhow::bail!("There are only {} lines", count);
                }
                Ok(text
                    .lines()
                    .skip(line as usize + 1)
                    .take(limit)
                    .collect::<String>())
            })?
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
            cx.update(|cx| {
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
                });

                action_log.update(cx, |action_log, cx| {
                    action_log.buffer_read(buffer.clone(), cx);
                });
                buffer.update(cx, |buffer, cx| {
                    buffer.edit(edits, None, cx);
                });
                action_log.update(cx, |action_log, cx| {
                    action_log.buffer_edited(buffer.clone(), cx);
                });
            })?;
            project
                .update(cx, |project, cx| project.save_buffer(buffer, cx))?
                .await
        })
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        self.entries.iter().map(|e| e.to_markdown(cx)).collect()
    }

    pub fn emit_server_exited(&mut self, status: ExitStatus, cx: &mut Context<Self>) {
        cx.emit(AcpThreadEvent::ServerExited(status));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use futures::{channel::mpsc, future::LocalBoxFuture, select};
    use gpui::{AsyncApp, TestAppContext, WeakEntity};
    use indoc::indoc;
    use project::FakeFs;
    use rand::Rng as _;
    use serde_json::json;
    use settings::SettingsStore;
    use smol::stream::StreamExt as _;
    use std::{cell::RefCell, rc::Rc, time::Duration};

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
    async fn test_push_user_content_block(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let connection = Rc::new(FakeAgentConnection::new());
        let thread = cx
            .spawn(async move |mut cx| {
                connection
                    .new_thread(project, Path::new(path!("/test")), &mut cx)
                    .await
            })
            .await
            .unwrap();

        // Test creating a new user message
        thread.update(cx, |thread, cx| {
            thread.push_user_content_block(
                acp::ContentBlock::Text(acp::TextContent {
                    annotations: None,
                    text: "Hello, ".to_string(),
                }),
                cx,
            );
        });

        thread.update(cx, |thread, cx| {
            assert_eq!(thread.entries.len(), 1);
            if let AgentThreadEntry::UserMessage(user_msg) = &thread.entries[0] {
                assert_eq!(user_msg.content.to_markdown(cx), "Hello, ");
            } else {
                panic!("Expected UserMessage");
            }
        });

        // Test appending to existing user message
        thread.update(cx, |thread, cx| {
            thread.push_user_content_block(
                acp::ContentBlock::Text(acp::TextContent {
                    annotations: None,
                    text: "world!".to_string(),
                }),
                cx,
            );
        });

        thread.update(cx, |thread, cx| {
            assert_eq!(thread.entries.len(), 1);
            if let AgentThreadEntry::UserMessage(user_msg) = &thread.entries[0] {
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
                }),
                false,
                cx,
            );
        });

        thread.update(cx, |thread, cx| {
            thread.push_user_content_block(
                acp::ContentBlock::Text(acp::TextContent {
                    annotations: None,
                    text: "New user message".to_string(),
                }),
                cx,
            );
        });

        thread.update(cx, |thread, cx| {
            assert_eq!(thread.entries.len(), 3);
            if let AgentThreadEntry::UserMessage(user_msg) = &thread.entries[2] {
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
                                acp::SessionUpdate::AgentThoughtChunk {
                                    content: "Thinking ".into(),
                                },
                                cx,
                            )
                            .unwrap();
                        thread
                            .handle_session_update(
                                acp::SessionUpdate::AgentThoughtChunk {
                                    content: "hard!".into(),
                                },
                                cx,
                            )
                            .unwrap();
                    })?;
                    Ok(acp::PromptResponse {
                        stop_reason: acp::StopReason::EndTurn,
                    })
                }
                .boxed_local()
            },
        ));

        let thread = cx
            .spawn(async move |mut cx| {
                connection
                    .new_thread(project, Path::new(path!("/test")), &mut cx)
                    .await
            })
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
            .spawn(|mut cx| connection.new_thread(project, Path::new(path!("/tmp")), &mut cx))
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
                                }),
                                cx,
                            )
                        })
                        .unwrap()
                        .unwrap();
                    Ok(acp::PromptResponse {
                        stop_reason: acp::StopReason::EndTurn,
                    })
                }
                .boxed_local()
            }
        }));

        let thread = cx
            .spawn(async move |mut cx| {
                connection
                    .new_thread(project, Path::new(path!("/test")), &mut cx)
                    .await
            })
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
                    status: ToolCallStatus::Allowed {
                        status: acp::ToolCallStatus::InProgress,
                        ..
                    },
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
                    status: ToolCallStatus::Allowed {
                        status: acp::ToolCallStatus::Completed,
                        ..
                    },
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
                                        },
                                    }],
                                    locations: vec![],
                                    raw_input: None,
                                    raw_output: None,
                                }),
                                cx,
                            )
                        })
                        .unwrap()
                        .unwrap();
                    Ok(acp::PromptResponse {
                        stop_reason: acp::StopReason::EndTurn,
                    })
                }
                .boxed_local()
            }
        }));

        let thread = connection
            .new_thread(project, Path::new(path!("/test")), &mut cx.to_async())
            .await
            .unwrap();
        cx.update(|cx| thread.update(cx, |thread, cx| thread.send(vec!["Hi".into()], cx)))
            .await
            .unwrap();

        assert!(cx.read(|cx| !thread.read(cx).has_pending_edit_tool_calls()));
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
            cx: &mut gpui::AsyncApp,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            let session_id = acp::SessionId(
                rand::thread_rng()
                    .sample_iter(&rand::distributions::Alphanumeric)
                    .take(7)
                    .map(char::from)
                    .collect::<String>()
                    .into(),
            );
            let thread = cx
                .new(|cx| AcpThread::new("Test", self.clone(), project, session_id.clone(), cx))
                .unwrap();
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
                }))
            }
        }

        fn cancel(&self, session_id: &acp::SessionId, cx: &mut App) {
            let sessions = self.sessions.lock();
            let thread = sessions.get(&session_id).unwrap().clone();

            cx.spawn(async move |cx| {
                thread
                    .update(cx, |thread, cx| thread.cancel(cx))
                    .unwrap()
                    .await
            })
            .detach();
        }
    }
}
