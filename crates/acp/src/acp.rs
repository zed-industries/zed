pub use acp::ToolCallId;
use agent_servers::AgentServer;
use agentic_coding_protocol::{self as acp, UserMessageChunk};
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::ActionLog;
use buffer_diff::BufferDiff;
use editor::{MultiBuffer, PathKey};
use futures::{FutureExt, channel::oneshot, future::BoxFuture};
use gpui::{AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task, WeakEntity};
use itertools::Itertools;
use language::{
    Anchor, Buffer, BufferSnapshot, Capability, LanguageRegistry, OffsetRangeExt as _, Point,
    text_diff,
};
use markdown::Markdown;
use project::{AgentLocation, Project};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Formatter, Write};
use std::{
    fmt::Display,
    mem,
    path::{Path, PathBuf},
    sync::Arc,
};
use ui::{App, IconName};
use util::ResultExt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserMessage {
    pub content: Entity<Markdown>,
}

impl UserMessage {
    pub fn from_acp(
        message: &acp::SendUserMessageParams,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        let mut md_source = String::new();

        for chunk in &message.chunks {
            match chunk {
                UserMessageChunk::Text { text } => md_source.push_str(&text),
                UserMessageChunk::Path { path } => {
                    write!(&mut md_source, "{}", MentionPath(&path)).unwrap()
                }
            }
        }

        Self {
            content: cx
                .new(|cx| Markdown::new(md_source.into(), Some(language_registry), None, cx)),
        }
    }

    fn to_markdown(&self, cx: &App) -> String {
        format!("## User\n\n{}\n\n", self.content.read(cx).source())
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssistantMessage {
    pub chunks: Vec<AssistantMessageChunk>,
}

impl AssistantMessage {
    fn to_markdown(&self, cx: &App) -> String {
        format!(
            "## Assistant\n\n{}\n\n",
            self.chunks
                .iter()
                .map(|chunk| chunk.to_markdown(cx))
                .join("\n\n")
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssistantMessageChunk {
    Text { chunk: Entity<Markdown> },
    Thought { chunk: Entity<Markdown> },
}

impl AssistantMessageChunk {
    pub fn from_acp(
        chunk: acp::AssistantMessageChunk,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        match chunk {
            acp::AssistantMessageChunk::Text { text } => Self::Text {
                chunk: cx.new(|cx| Markdown::new(text.into(), Some(language_registry), None, cx)),
            },
            acp::AssistantMessageChunk::Thought { thought } => Self::Thought {
                chunk: cx
                    .new(|cx| Markdown::new(thought.into(), Some(language_registry), None, cx)),
            },
        }
    }

    pub fn from_str(chunk: &str, language_registry: Arc<LanguageRegistry>, cx: &mut App) -> Self {
        Self::Text {
            chunk: cx.new(|cx| {
                Markdown::new(chunk.to_owned().into(), Some(language_registry), None, cx)
            }),
        }
    }

    fn to_markdown(&self, cx: &App) -> String {
        match self {
            Self::Text { chunk } => chunk.read(cx).source().to_string(),
            Self::Thought { chunk } => {
                format!("<thinking>\n{}\n</thinking>", chunk.read(cx).source())
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
            Self::ToolCall(too_call) => too_call.to_markdown(cx),
        }
    }

    pub fn diff(&self) -> Option<&Diff> {
        if let AgentThreadEntry::ToolCall(ToolCall {
            content: Some(ToolCallContent::Diff { diff }),
            ..
        }) = self
        {
            Some(&diff)
        } else {
            None
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
    pub icon: IconName,
    pub content: Option<ToolCallContent>,
    pub status: ToolCallStatus,
    pub locations: Vec<acp::ToolCallLocation>,
}

impl ToolCall {
    fn to_markdown(&self, cx: &App) -> String {
        let mut markdown = format!(
            "**Tool Call: {}**\nStatus: {}\n\n",
            self.label.read(cx).source(),
            self.status
        );
        if let Some(content) = &self.content {
            markdown.push_str(content.to_markdown(cx).as_str());
            markdown.push_str("\n\n");
        }
        markdown
    }
}

#[derive(Debug)]
pub enum ToolCallStatus {
    WaitingForConfirmation {
        confirmation: ToolCallConfirmation,
        respond_tx: oneshot::Sender<acp::ToolCallConfirmationOutcome>,
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
                    acp::ToolCallStatus::Running => "Running",
                    acp::ToolCallStatus::Finished => "Finished",
                    acp::ToolCallStatus::Error => "Error",
                },
                ToolCallStatus::Rejected => "Rejected",
                ToolCallStatus::Canceled => "Canceled",
            }
        )
    }
}

#[derive(Debug)]
pub enum ToolCallConfirmation {
    Edit {
        description: Option<Entity<Markdown>>,
    },
    Execute {
        command: String,
        root_command: String,
        description: Option<Entity<Markdown>>,
    },
    Mcp {
        server_name: String,
        tool_name: String,
        tool_display_name: String,
        description: Option<Entity<Markdown>>,
    },
    Fetch {
        urls: Vec<SharedString>,
        description: Option<Entity<Markdown>>,
    },
    Other {
        description: Entity<Markdown>,
    },
}

impl ToolCallConfirmation {
    pub fn from_acp(
        confirmation: acp::ToolCallConfirmation,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        let to_md = |description: String, cx: &mut App| -> Entity<Markdown> {
            cx.new(|cx| {
                Markdown::new(
                    description.into(),
                    Some(language_registry.clone()),
                    None,
                    cx,
                )
            })
        };

        match confirmation {
            acp::ToolCallConfirmation::Edit { description } => Self::Edit {
                description: description.map(|description| to_md(description, cx)),
            },
            acp::ToolCallConfirmation::Execute {
                command,
                root_command,
                description,
            } => Self::Execute {
                command,
                root_command,
                description: description.map(|description| to_md(description, cx)),
            },
            acp::ToolCallConfirmation::Mcp {
                server_name,
                tool_name,
                tool_display_name,
                description,
            } => Self::Mcp {
                server_name,
                tool_name,
                tool_display_name,
                description: description.map(|description| to_md(description, cx)),
            },
            acp::ToolCallConfirmation::Fetch { urls, description } => Self::Fetch {
                urls: urls.iter().map(|url| url.into()).collect(),
                description: description.map(|description| to_md(description, cx)),
            },
            acp::ToolCallConfirmation::Other { description } => Self::Other {
                description: to_md(description, cx),
            },
        }
    }
}

#[derive(Debug)]
pub enum ToolCallContent {
    Markdown { markdown: Entity<Markdown> },
    Diff { diff: Diff },
}

impl ToolCallContent {
    pub fn from_acp(
        content: acp::ToolCallContent,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        match content {
            acp::ToolCallContent::Markdown { markdown } => Self::Markdown {
                markdown: cx.new(|cx| Markdown::new_text(markdown.into(), cx)),
            },
            acp::ToolCallContent::Diff { diff } => Self::Diff {
                diff: Diff::from_acp(diff, language_registry, cx),
            },
        }
    }

    fn to_markdown(&self, cx: &App) -> String {
        match self {
            Self::Markdown { markdown } => markdown.read(cx).source().to_string(),
            Self::Diff { diff } => diff.to_markdown(cx),
        }
    }
}

#[derive(Debug)]
pub struct Diff {
    pub multibuffer: Entity<MultiBuffer>,
    pub path: PathBuf,
    pub new_buffer: Entity<Buffer>,
    pub old_buffer: Entity<Buffer>,
    _task: Task<Result<()>>,
}

impl Diff {
    pub fn from_acp(
        diff: acp::Diff,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        let acp::Diff {
            path,
            old_text,
            new_text,
        } = diff;

        let multibuffer = cx.new(|_cx| MultiBuffer::without_headers(Capability::ReadOnly));

        let new_buffer = cx.new(|cx| Buffer::local(new_text, cx));
        let old_buffer = cx.new(|cx| Buffer::local(old_text.unwrap_or("".into()), cx));
        let new_buffer_snapshot = new_buffer.read(cx).text_snapshot();
        let old_buffer_snapshot = old_buffer.read(cx).snapshot();
        let buffer_diff = cx.new(|cx| BufferDiff::new(&new_buffer_snapshot, cx));
        let diff_task = buffer_diff.update(cx, |diff, cx| {
            diff.set_base_text(
                old_buffer_snapshot,
                Some(language_registry.clone()),
                new_buffer_snapshot,
                cx,
            )
        });

        let task = cx.spawn({
            let multibuffer = multibuffer.clone();
            let path = path.clone();
            let new_buffer = new_buffer.clone();
            async move |cx| {
                diff_task.await?;

                multibuffer
                    .update(cx, |multibuffer, cx| {
                        let hunk_ranges = {
                            let buffer = new_buffer.read(cx);
                            let diff = buffer_diff.read(cx);
                            diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &buffer, cx)
                                .map(|diff_hunk| diff_hunk.buffer_range.to_point(&buffer))
                                .collect::<Vec<_>>()
                        };

                        multibuffer.set_excerpts_for_path(
                            PathKey::for_buffer(&new_buffer, cx),
                            new_buffer.clone(),
                            hunk_ranges,
                            editor::DEFAULT_MULTIBUFFER_CONTEXT,
                            cx,
                        );
                        multibuffer.add_diff(buffer_diff.clone(), cx);
                    })
                    .log_err();

                if let Some(language) = language_registry
                    .language_for_file_path(&path)
                    .await
                    .log_err()
                {
                    new_buffer.update(cx, |buffer, cx| buffer.set_language(Some(language), cx))?;
                }

                anyhow::Ok(())
            }
        });

        Self {
            multibuffer,
            path,
            new_buffer,
            old_buffer,
            _task: task,
        }
    }

    fn to_markdown(&self, cx: &App) -> String {
        let buffer_text = self
            .multibuffer
            .read(cx)
            .all_buffers()
            .iter()
            .map(|buffer| buffer.read(cx).text())
            .join("\n");
        format!("Diff: {}\n```\n{}\n```\n", self.path.display(), buffer_text)
    }
}

pub struct AcpThread {
    entries: Vec<AgentThreadEntry>,
    title: SharedString,
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
    shared_buffers: HashMap<Entity<Buffer>, BufferSnapshot>,
    send_task: Option<Task<()>>,
    connection: Arc<acp::AgentConnection>,
    child_status: Option<Task<Result<()>>>,
    _io_task: Task<()>,
}

pub enum AcpThreadEvent {
    NewEntry,
    EntryUpdated(usize),
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
    Unsupported { current_version: SharedString },
    Exited(i32),
    Other(SharedString),
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Unsupported { current_version } => {
                write!(
                    f,
                    "Your installed version of Gemini {} doesn't support the Agentic Coding Protocol (ACP).",
                    current_version
                )
            }
            LoadError::Exited(status) => write!(f, "Server exited with status {}", status),
            LoadError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for LoadError {}

impl AcpThread {
    pub async fn spawn(
        server: impl AgentServer + 'static,
        root_dir: &Path,
        project: Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<Entity<Self>> {
        let command = match server.command(&project, cx).await {
            Ok(command) => command,
            Err(e) => return Err(anyhow!(LoadError::Other(format!("{e}").into()))),
        };

        let mut child = util::command::new_smol_command(&command.path)
            .args(command.args.iter())
            .current_dir(root_dir)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .kill_on_drop(true)
            .spawn()?;

        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        cx.new(|cx| {
            let foreground_executor = cx.foreground_executor().clone();

            let (connection, io_fut) = acp::AgentConnection::connect_to_agent(
                AcpClientDelegate::new(cx.entity().downgrade(), cx.to_async()),
                stdin,
                stdout,
                move |fut| foreground_executor.spawn(fut).detach(),
            );

            let io_task = cx.background_spawn(async move {
                io_fut.await.log_err();
            });

            let child_status = cx.background_spawn(async move {
                match child.status().await {
                    Err(e) => Err(anyhow!(e)),
                    Ok(result) if result.success() => Ok(()),
                    Ok(result) => {
                        if let Some(version) = server.version(&command).await.log_err()
                            && !version.supported
                        {
                            Err(anyhow!(LoadError::Unsupported {
                                current_version: version.current_version
                            }))
                        } else {
                            Err(anyhow!(LoadError::Exited(result.code().unwrap_or(-127))))
                        }
                    }
                }
            });

            let action_log = cx.new(|_| ActionLog::new(project.clone()));

            Self {
                action_log,
                shared_buffers: Default::default(),
                entries: Default::default(),
                title: "ACP Thread".into(),
                project,
                send_task: None,
                connection: Arc::new(connection),
                child_status: Some(child_status),
                _io_task: io_task,
            }
        })
    }

    pub fn action_log(&self) -> &Entity<ActionLog> {
        &self.action_log
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    #[cfg(test)]
    pub fn fake(
        stdin: async_pipe::PipeWriter,
        stdout: async_pipe::PipeReader,
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Self {
        let foreground_executor = cx.foreground_executor().clone();

        let (connection, io_fut) = acp::AgentConnection::connect_to_agent(
            AcpClientDelegate::new(cx.entity().downgrade(), cx.to_async()),
            stdin,
            stdout,
            move |fut| {
                foreground_executor.spawn(fut).detach();
            },
        );

        let io_task = cx.background_spawn({
            async move {
                io_fut.await.log_err();
            }
        });

        let action_log = cx.new(|_| ActionLog::new(project.clone()));

        Self {
            action_log,
            shared_buffers: Default::default(),
            entries: Default::default(),
            title: "ACP Thread".into(),
            project,
            send_task: None,
            connection: Arc::new(connection),
            child_status: None,
            _io_task: io_task,
        }
    }

    pub fn title(&self) -> SharedString {
        self.title.clone()
    }

    pub fn entries(&self) -> &[AgentThreadEntry] {
        &self.entries
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
                AgentThreadEntry::ToolCall(ToolCall {
                    status:
                        ToolCallStatus::Allowed {
                            status: acp::ToolCallStatus::Running,
                            ..
                        },
                    content: Some(ToolCallContent::Diff { .. }),
                    ..
                }) => return true,
                AgentThreadEntry::ToolCall(_) | AgentThreadEntry::AssistantMessage(_) => {}
            }
        }

        false
    }

    pub fn push_entry(&mut self, entry: AgentThreadEntry, cx: &mut Context<Self>) {
        self.entries.push(entry);
        cx.emit(AcpThreadEvent::NewEntry);
    }

    pub fn push_assistant_chunk(
        &mut self,
        chunk: acp::AssistantMessageChunk,
        cx: &mut Context<Self>,
    ) {
        let entries_len = self.entries.len();
        if let Some(last_entry) = self.entries.last_mut()
            && let AgentThreadEntry::AssistantMessage(AssistantMessage { chunks }) = last_entry
        {
            cx.emit(AcpThreadEvent::EntryUpdated(entries_len - 1));

            match (chunks.last_mut(), &chunk) {
                (
                    Some(AssistantMessageChunk::Text { chunk: old_chunk }),
                    acp::AssistantMessageChunk::Text { text: new_chunk },
                )
                | (
                    Some(AssistantMessageChunk::Thought { chunk: old_chunk }),
                    acp::AssistantMessageChunk::Thought { thought: new_chunk },
                ) => {
                    old_chunk.update(cx, |old_chunk, cx| {
                        old_chunk.append(&new_chunk, cx);
                    });
                }
                _ => {
                    chunks.push(AssistantMessageChunk::from_acp(
                        chunk,
                        self.project.read(cx).languages().clone(),
                        cx,
                    ));
                }
            }
        } else {
            let chunk = AssistantMessageChunk::from_acp(
                chunk,
                self.project.read(cx).languages().clone(),
                cx,
            );

            self.push_entry(
                AgentThreadEntry::AssistantMessage(AssistantMessage {
                    chunks: vec![chunk],
                }),
                cx,
            );
        }
    }

    pub fn request_tool_call(
        &mut self,
        tool_call: acp::RequestToolCallConfirmationParams,
        cx: &mut Context<Self>,
    ) -> ToolCallRequest {
        let (tx, rx) = oneshot::channel();

        let status = ToolCallStatus::WaitingForConfirmation {
            confirmation: ToolCallConfirmation::from_acp(
                tool_call.confirmation,
                self.project.read(cx).languages().clone(),
                cx,
            ),
            respond_tx: tx,
        };

        let id = self.insert_tool_call(tool_call.tool_call, status, cx);
        ToolCallRequest { id, outcome: rx }
    }

    pub fn push_tool_call(
        &mut self,
        request: acp::PushToolCallParams,
        cx: &mut Context<Self>,
    ) -> acp::ToolCallId {
        let status = ToolCallStatus::Allowed {
            status: acp::ToolCallStatus::Running,
        };

        self.insert_tool_call(request, status, cx)
    }

    fn insert_tool_call(
        &mut self,
        tool_call: acp::PushToolCallParams,
        status: ToolCallStatus,
        cx: &mut Context<Self>,
    ) -> acp::ToolCallId {
        let language_registry = self.project.read(cx).languages().clone();
        let id = acp::ToolCallId(self.entries.len() as u64);
        let call = ToolCall {
            id,
            label: cx.new(|cx| {
                Markdown::new(
                    tool_call.label.into(),
                    Some(language_registry.clone()),
                    None,
                    cx,
                )
            }),
            icon: acp_icon_to_ui_icon(tool_call.icon),
            content: tool_call
                .content
                .map(|content| ToolCallContent::from_acp(content, language_registry, cx)),
            locations: tool_call.locations,
            status,
        };

        self.push_entry(AgentThreadEntry::ToolCall(call), cx);

        id
    }

    pub fn authorize_tool_call(
        &mut self,
        id: acp::ToolCallId,
        outcome: acp::ToolCallConfirmationOutcome,
        cx: &mut Context<Self>,
    ) {
        let Some((ix, call)) = self.tool_call_mut(id) else {
            return;
        };

        let new_status = if outcome == acp::ToolCallConfirmationOutcome::Reject {
            ToolCallStatus::Rejected
        } else {
            ToolCallStatus::Allowed {
                status: acp::ToolCallStatus::Running,
            }
        };

        let curr_status = mem::replace(&mut call.status, new_status);

        if let ToolCallStatus::WaitingForConfirmation { respond_tx, .. } = curr_status {
            respond_tx.send(outcome).log_err();
        } else if cfg!(debug_assertions) {
            panic!("tried to authorize an already authorized tool call");
        }

        cx.emit(AcpThreadEvent::EntryUpdated(ix));
    }

    pub fn update_tool_call(
        &mut self,
        id: acp::ToolCallId,
        new_status: acp::ToolCallStatus,
        new_content: Option<acp::ToolCallContent>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let language_registry = self.project.read(cx).languages().clone();
        let (ix, call) = self.tool_call_mut(id).context("Entry not found")?;

        call.content = new_content
            .map(|new_content| ToolCallContent::from_acp(new_content, language_registry, cx));

        match &mut call.status {
            ToolCallStatus::Allowed { status } => {
                *status = new_status;
            }
            ToolCallStatus::WaitingForConfirmation { .. } => {
                anyhow::bail!("Tool call hasn't been authorized yet")
            }
            ToolCallStatus::Rejected => {
                anyhow::bail!("Tool call was rejected and therefore can't be updated")
            }
            ToolCallStatus::Canceled => {
                call.status = ToolCallStatus::Allowed { status: new_status };
            }
        }

        cx.emit(AcpThreadEvent::EntryUpdated(ix));
        Ok(())
    }

    fn tool_call_mut(&mut self, id: acp::ToolCallId) -> Option<(usize, &mut ToolCall)> {
        let entry = self.entries.get_mut(id.0 as usize);
        debug_assert!(
            entry.is_some(),
            "We shouldn't give out ids to entries that don't exist"
        );
        match entry {
            Some(AgentThreadEntry::ToolCall(call)) if call.id == id => Some((id.0 as usize, call)),
            _ => {
                if cfg!(debug_assertions) {
                    panic!("entry is not a tool call");
                }
                None
            }
        }
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

    pub fn initialize(
        &self,
    ) -> impl use<> + Future<Output = Result<acp::InitializeResponse, acp::Error>> {
        let connection = self.connection.clone();
        async move { connection.request(acp::InitializeParams).await }
    }

    pub fn authenticate(&self) -> impl use<> + Future<Output = Result<(), acp::Error>> {
        let connection = self.connection.clone();
        async move { connection.request(acp::AuthenticateParams).await }
    }

    #[cfg(test)]
    pub fn send_raw(
        &mut self,
        message: &str,
        cx: &mut Context<Self>,
    ) -> BoxFuture<'static, Result<(), acp::Error>> {
        self.send(
            acp::SendUserMessageParams {
                chunks: vec![acp::UserMessageChunk::Text {
                    text: message.to_string(),
                }],
            },
            cx,
        )
    }

    pub fn send(
        &mut self,
        message: acp::SendUserMessageParams,
        cx: &mut Context<Self>,
    ) -> BoxFuture<'static, Result<(), acp::Error>> {
        let agent = self.connection.clone();
        self.push_entry(
            AgentThreadEntry::UserMessage(UserMessage::from_acp(
                &message,
                self.project.read(cx).languages().clone(),
                cx,
            )),
            cx,
        );

        let (tx, rx) = oneshot::channel();
        let cancel = self.cancel(cx);

        self.send_task = Some(cx.spawn(async move |this, cx| {
            cancel.await.log_err();

            let result = agent.request(message).await;
            tx.send(result).log_err();
            this.update(cx, |this, _cx| this.send_task.take()).log_err();
        }));

        async move {
            match rx.await {
                Ok(Err(e)) => Err(e)?,
                _ => Ok(()),
            }
        }
        .boxed()
    }

    pub fn cancel(&mut self, cx: &mut Context<Self>) -> Task<Result<(), acp::Error>> {
        let agent = self.connection.clone();

        if self.send_task.take().is_some() {
            cx.spawn(async move |this, cx| {
                agent.request(acp::CancelSendMessageParams).await?;

                this.update(cx, |this, _cx| {
                    for entry in this.entries.iter_mut() {
                        if let AgentThreadEntry::ToolCall(call) = entry {
                            let cancel = matches!(
                                call.status,
                                ToolCallStatus::WaitingForConfirmation { .. }
                                    | ToolCallStatus::Allowed {
                                        status: acp::ToolCallStatus::Running
                                    }
                            );

                            if cancel {
                                let curr_status =
                                    mem::replace(&mut call.status, ToolCallStatus::Canceled);

                                if let ToolCallStatus::WaitingForConfirmation {
                                    respond_tx, ..
                                } = curr_status
                                {
                                    respond_tx
                                        .send(acp::ToolCallConfirmationOutcome::Cancel)
                                        .ok();
                                }
                            }
                        }
                    }
                })?;
                Ok(())
            })
        } else {
            Task::ready(Ok(()))
        }
    }

    pub fn read_text_file(
        &self,
        request: acp::ReadTextFileParams,
        cx: &mut Context<Self>,
    ) -> Task<Result<String>> {
        let project = self.project.clone();
        let action_log = self.action_log.clone();
        cx.spawn(async move |this, cx| {
            let load = project.update(cx, |project, cx| {
                let path = project
                    .project_path_for_absolute_path(&request.path, cx)
                    .context("invalid path")?;
                anyhow::Ok(project.open_buffer(path, cx))
            });
            let buffer = load??.await?;

            action_log.update(cx, |action_log, cx| {
                action_log.buffer_read(buffer.clone(), cx);
            })?;
            project.update(cx, |project, cx| {
                let position = buffer
                    .read(cx)
                    .snapshot()
                    .anchor_before(Point::new(request.line.unwrap_or_default(), 0));
                project.set_agent_location(
                    Some(AgentLocation {
                        buffer: buffer.downgrade(),
                        position,
                    }),
                    cx,
                );
            })?;
            let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot())?;
            this.update(cx, |this, _| {
                let text = snapshot.text();
                this.shared_buffers.insert(buffer.clone(), snapshot);
                text
            })
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

    pub fn child_status(&mut self) -> Option<Task<Result<()>>> {
        self.child_status.take()
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        self.entries.iter().map(|e| e.to_markdown(cx)).collect()
    }
}

struct AcpClientDelegate {
    thread: WeakEntity<AcpThread>,
    cx: AsyncApp,
    // sent_buffer_versions: HashMap<Entity<Buffer>, HashMap<u64, BufferSnapshot>>,
}

impl AcpClientDelegate {
    fn new(thread: WeakEntity<AcpThread>, cx: AsyncApp) -> Self {
        Self { thread, cx }
    }
}

impl acp::Client for AcpClientDelegate {
    async fn stream_assistant_message_chunk(
        &self,
        params: acp::StreamAssistantMessageChunkParams,
    ) -> Result<(), acp::Error> {
        let cx = &mut self.cx.clone();

        cx.update(|cx| {
            self.thread
                .update(cx, |thread, cx| {
                    thread.push_assistant_chunk(params.chunk, cx)
                })
                .ok();
        })?;

        Ok(())
    }

    async fn request_tool_call_confirmation(
        &self,
        request: acp::RequestToolCallConfirmationParams,
    ) -> Result<acp::RequestToolCallConfirmationResponse, acp::Error> {
        let cx = &mut self.cx.clone();
        let ToolCallRequest { id, outcome } = cx
            .update(|cx| {
                self.thread
                    .update(cx, |thread, cx| thread.request_tool_call(request, cx))
            })?
            .context("Failed to update thread")?;

        Ok(acp::RequestToolCallConfirmationResponse {
            id,
            outcome: outcome.await.map_err(acp::Error::into_internal_error)?,
        })
    }

    async fn push_tool_call(
        &self,
        request: acp::PushToolCallParams,
    ) -> Result<acp::PushToolCallResponse, acp::Error> {
        let cx = &mut self.cx.clone();
        let id = cx
            .update(|cx| {
                self.thread
                    .update(cx, |thread, cx| thread.push_tool_call(request, cx))
            })?
            .context("Failed to update thread")?;

        Ok(acp::PushToolCallResponse { id })
    }

    async fn update_tool_call(&self, request: acp::UpdateToolCallParams) -> Result<(), acp::Error> {
        let cx = &mut self.cx.clone();

        cx.update(|cx| {
            self.thread.update(cx, |thread, cx| {
                thread.update_tool_call(request.tool_call_id, request.status, request.content, cx)
            })
        })?
        .context("Failed to update thread")??;

        Ok(())
    }

    async fn read_text_file(
        &self,
        request: acp::ReadTextFileParams,
    ) -> Result<acp::ReadTextFileResponse, acp::Error> {
        let content = self
            .cx
            .update(|cx| {
                self.thread
                    .update(cx, |thread, cx| thread.read_text_file(request, cx))
            })?
            .context("Failed to update thread")?
            .await?;
        Ok(acp::ReadTextFileResponse { content })
    }

    async fn write_text_file(&self, request: acp::WriteTextFileParams) -> Result<(), acp::Error> {
        self.cx
            .update(|cx| {
                self.thread.update(cx, |thread, cx| {
                    thread.write_text_file(request.path, request.content, cx)
                })
            })?
            .context("Failed to update thread")?
            .await?;

        Ok(())
    }
}

fn acp_icon_to_ui_icon(icon: acp::Icon) -> IconName {
    match icon {
        acp::Icon::FileSearch => IconName::ToolSearch,
        acp::Icon::Folder => IconName::ToolFolder,
        acp::Icon::Globe => IconName::ToolWeb,
        acp::Icon::Hammer => IconName::ToolHammer,
        acp::Icon::LightBulb => IconName::ToolBulb,
        acp::Icon::Pencil => IconName::ToolPencil,
        acp::Icon::Regex => IconName::ToolRegex,
        acp::Icon::Terminal => IconName::ToolTerminal,
    }
}

pub struct ToolCallRequest {
    pub id: acp::ToolCallId,
    pub outcome: oneshot::Receiver<acp::ToolCallConfirmationOutcome>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent_servers::{AgentServerCommand, AgentServerVersion};
    use async_pipe::{PipeReader, PipeWriter};
    use futures::{channel::mpsc, future::LocalBoxFuture, select};
    use gpui::{AsyncApp, TestAppContext};
    use indoc::indoc;
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use smol::{future::BoxedLocal, stream::StreamExt as _};
    use std::{cell::RefCell, env, path::Path, rc::Rc, time::Duration};
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
    async fn test_thinking_concatenation(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (thread, fake_server) = fake_acp_thread(project, cx);

        fake_server.update(cx, |fake_server, _| {
            fake_server.on_user_message(move |_, server, mut cx| async move {
                server
                    .update(&mut cx, |server, _| {
                        server.send_to_zed(acp::StreamAssistantMessageChunkParams {
                            chunk: acp::AssistantMessageChunk::Thought {
                                thought: "Thinking ".into(),
                            },
                        })
                    })?
                    .await
                    .unwrap();
                server
                    .update(&mut cx, |server, _| {
                        server.send_to_zed(acp::StreamAssistantMessageChunkParams {
                            chunk: acp::AssistantMessageChunk::Thought {
                                thought: "hard!".into(),
                            },
                        })
                    })?
                    .await
                    .unwrap();

                Ok(())
            })
        });

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
        let (thread, fake_server) = fake_acp_thread(project.clone(), cx);
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

        let (read_file_tx, read_file_rx) = oneshot::channel::<()>();
        let read_file_tx = Rc::new(RefCell::new(Some(read_file_tx)));

        fake_server.update(cx, |fake_server, _| {
            fake_server.on_user_message(move |_, server, mut cx| {
                let read_file_tx = read_file_tx.clone();
                async move {
                    let content = server
                        .update(&mut cx, |server, _| {
                            server.send_to_zed(acp::ReadTextFileParams {
                                path: path!("/tmp/foo").into(),
                                line: None,
                                limit: None,
                            })
                        })?
                        .await
                        .unwrap();
                    assert_eq!(content.content, "one\ntwo\nthree\n");
                    read_file_tx.take().unwrap().send(()).unwrap();
                    server
                        .update(&mut cx, |server, _| {
                            server.send_to_zed(acp::WriteTextFileParams {
                                path: path!("/tmp/foo").into(),
                                content: "one\ntwo\nthree\nfour\nfive\n".to_string(),
                            })
                        })?
                        .await
                        .unwrap();
                    Ok(())
                }
            })
        });

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
        let (thread, fake_server) = fake_acp_thread(project, cx);

        let (end_turn_tx, end_turn_rx) = oneshot::channel::<()>();

        let tool_call_id = Rc::new(RefCell::new(None));
        let end_turn_rx = Rc::new(RefCell::new(Some(end_turn_rx)));
        fake_server.update(cx, |fake_server, _| {
            let tool_call_id = tool_call_id.clone();
            fake_server.on_user_message(move |_, server, mut cx| {
                let end_turn_rx = end_turn_rx.clone();
                let tool_call_id = tool_call_id.clone();
                async move {
                    let tool_call_result = server
                        .update(&mut cx, |server, _| {
                            server.send_to_zed(acp::PushToolCallParams {
                                label: "Fetch".to_string(),
                                icon: acp::Icon::Globe,
                                content: None,
                                locations: vec![],
                            })
                        })?
                        .await
                        .unwrap();
                    *tool_call_id.clone().borrow_mut() = Some(tool_call_result.id);
                    end_turn_rx.take().unwrap().await.ok();

                    Ok(())
                }
            })
        });

        let request = thread.update(cx, |thread, cx| {
            thread.send_raw("Fetch https://example.com", cx)
        });

        run_until_first_tool_call(&thread, cx).await;

        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                thread.entries[1],
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::Allowed {
                        status: acp::ToolCallStatus::Running,
                        ..
                    },
                    ..
                })
            ));
        });

        cx.run_until_parked();

        thread
            .update(cx, |thread, cx| thread.cancel(cx))
            .await
            .unwrap();

        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                &thread.entries[1],
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::Canceled,
                    ..
                })
            ));
        });

        fake_server
            .update(cx, |fake_server, _| {
                fake_server.send_to_zed(acp::UpdateToolCallParams {
                    tool_call_id: tool_call_id.borrow().unwrap(),
                    status: acp::ToolCallStatus::Finished,
                    content: None,
                })
            })
            .await
            .unwrap();

        drop(end_turn_tx);
        request.await.unwrap();

        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                thread.entries[1],
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::Allowed {
                        status: acp::ToolCallStatus::Finished,
                        ..
                    },
                    ..
                })
            ));
        });
    }

    #[gpui::test]
    #[cfg_attr(not(feature = "gemini"), ignore)]
    async fn test_gemini_basic(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let thread = gemini_acp_thread(project.clone(), "/private/tmp", cx).await;
        thread
            .update(cx, |thread, cx| thread.send_raw("Hello from Zed!", cx))
            .await
            .unwrap();

        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.entries.len(), 2);
            assert!(matches!(
                thread.entries[0],
                AgentThreadEntry::UserMessage(_)
            ));
            assert!(matches!(
                thread.entries[1],
                AgentThreadEntry::AssistantMessage(_)
            ));
        });
    }

    #[gpui::test]
    #[cfg_attr(not(feature = "gemini"), ignore)]
    async fn test_gemini_path_mentions(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();
        let tempdir = tempfile::tempdir().unwrap();
        std::fs::write(
            tempdir.path().join("foo.rs"),
            indoc! {"
                fn main() {
                    println!(\"Hello, world!\");
                }
            "},
        )
        .expect("failed to write file");
        let project = Project::example([tempdir.path()], &mut cx.to_async()).await;
        let thread = gemini_acp_thread(project.clone(), tempdir.path(), cx).await;
        thread
            .update(cx, |thread, cx| {
                thread.send(
                    acp::SendUserMessageParams {
                        chunks: vec![
                            acp::UserMessageChunk::Text {
                                text: "Read the file ".into(),
                            },
                            acp::UserMessageChunk::Path {
                                path: Path::new("foo.rs").into(),
                            },
                            acp::UserMessageChunk::Text {
                                text: " and tell me what the content of the println! is".into(),
                            },
                        ],
                    },
                    cx,
                )
            })
            .await
            .unwrap();

        thread.read_with(cx, |thread, cx| {
            assert_eq!(thread.entries.len(), 3);
            assert!(matches!(
                thread.entries[0],
                AgentThreadEntry::UserMessage(_)
            ));
            assert!(matches!(thread.entries[1], AgentThreadEntry::ToolCall(_)));
            let AgentThreadEntry::AssistantMessage(assistant_message) = &thread.entries[2] else {
                panic!("Expected AssistantMessage")
            };
            assert!(
                assistant_message.to_markdown(cx).contains("Hello, world!"),
                "unexpected assistant message: {:?}",
                assistant_message.to_markdown(cx)
            );
        });
    }

    #[gpui::test]
    #[cfg_attr(not(feature = "gemini"), ignore)]
    async fn test_gemini_tool_call(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/private/tmp"),
            json!({"foo": "Lorem ipsum dolor", "bar": "bar", "baz": "baz"}),
        )
        .await;
        let project = Project::test(fs, [path!("/private/tmp").as_ref()], cx).await;
        let thread = gemini_acp_thread(project.clone(), "/private/tmp", cx).await;
        thread
            .update(cx, |thread, cx| {
                thread.send_raw(
                    "Read the '/private/tmp/foo' file and tell me what you see.",
                    cx,
                )
            })
            .await
            .unwrap();
        thread.read_with(cx, |thread, _cx| {
            assert!(matches!(
                &thread.entries()[2],
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::Allowed { .. },
                    ..
                })
            ));

            assert!(matches!(
                thread.entries[3],
                AgentThreadEntry::AssistantMessage(_)
            ));
        });
    }

    #[gpui::test]
    #[cfg_attr(not(feature = "gemini"), ignore)]
    async fn test_gemini_tool_call_with_confirmation(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [path!("/private/tmp").as_ref()], cx).await;
        let thread = gemini_acp_thread(project.clone(), "/private/tmp", cx).await;
        let full_turn = thread.update(cx, |thread, cx| {
            thread.send_raw(r#"Run `echo "Hello, world!"`"#, cx)
        });

        run_until_first_tool_call(&thread, cx).await;

        let tool_call_id = thread.read_with(cx, |thread, _cx| {
            let AgentThreadEntry::ToolCall(ToolCall {
                id,
                status:
                    ToolCallStatus::WaitingForConfirmation {
                        confirmation: ToolCallConfirmation::Execute { root_command, .. },
                        ..
                    },
                ..
            }) = &thread.entries()[2]
            else {
                panic!();
            };

            assert_eq!(root_command, "echo");

            *id
        });

        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(tool_call_id, acp::ToolCallConfirmationOutcome::Allow, cx);

            assert!(matches!(
                &thread.entries()[2],
                AgentThreadEntry::ToolCall(ToolCall {
                    status: ToolCallStatus::Allowed { .. },
                    ..
                })
            ));
        });

        full_turn.await.unwrap();

        thread.read_with(cx, |thread, cx| {
            let AgentThreadEntry::ToolCall(ToolCall {
                content: Some(ToolCallContent::Markdown { markdown }),
                status: ToolCallStatus::Allowed { .. },
                ..
            }) = &thread.entries()[2]
            else {
                panic!();
            };

            markdown.read_with(cx, |md, _cx| {
                assert!(
                    md.source().contains("Hello, world!"),
                    r#"Expected '{}' to contain "Hello, world!""#,
                    md.source()
                );
            });
        });
    }

    #[gpui::test]
    #[cfg_attr(not(feature = "gemini"), ignore)]
    async fn test_gemini_cancel(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [path!("/private/tmp").as_ref()], cx).await;
        let thread = gemini_acp_thread(project.clone(), "/private/tmp", cx).await;
        let full_turn = thread.update(cx, |thread, cx| {
            thread.send_raw(r#"Run `echo "Hello, world!"`"#, cx)
        });

        let first_tool_call_ix = run_until_first_tool_call(&thread, cx).await;

        thread.read_with(cx, |thread, _cx| {
            let AgentThreadEntry::ToolCall(ToolCall {
                id,
                status:
                    ToolCallStatus::WaitingForConfirmation {
                        confirmation: ToolCallConfirmation::Execute { root_command, .. },
                        ..
                    },
                ..
            }) = &thread.entries()[first_tool_call_ix]
            else {
                panic!("{:?}", thread.entries()[1]);
            };

            assert_eq!(root_command, "echo");

            *id
        });

        thread
            .update(cx, |thread, cx| thread.cancel(cx))
            .await
            .unwrap();
        full_turn.await.unwrap();
        thread.read_with(cx, |thread, _| {
            let AgentThreadEntry::ToolCall(ToolCall {
                status: ToolCallStatus::Canceled,
                ..
            }) = &thread.entries()[first_tool_call_ix]
            else {
                panic!();
            };
        });

        thread
            .update(cx, |thread, cx| {
                thread.send_raw(r#"Stop running and say goodbye to me."#, cx)
            })
            .await
            .unwrap();
        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                &thread.entries().last().unwrap(),
                AgentThreadEntry::AssistantMessage(..),
            ))
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

    pub async fn gemini_acp_thread(
        project: Entity<Project>,
        current_dir: impl AsRef<Path>,
        cx: &mut TestAppContext,
    ) -> Entity<AcpThread> {
        struct DevGemini;

        impl agent_servers::AgentServer for DevGemini {
            async fn command(
                &self,
                _project: &Entity<Project>,
                _cx: &mut AsyncApp,
            ) -> Result<agent_servers::AgentServerCommand> {
                let cli_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../../gemini-cli/packages/cli")
                    .to_string_lossy()
                    .to_string();

                Ok(AgentServerCommand {
                    path: "node".into(),
                    args: vec![cli_path, "--acp".into()],
                    env: None,
                })
            }

            async fn version(
                &self,
                _command: &agent_servers::AgentServerCommand,
            ) -> Result<AgentServerVersion> {
                Ok(AgentServerVersion {
                    current_version: "0.1.0".into(),
                    supported: true,
                })
            }
        }

        let thread = AcpThread::spawn(DevGemini, current_dir.as_ref(), project, &mut cx.to_async())
            .await
            .unwrap();

        thread
            .update(cx, |thread, _| thread.initialize())
            .await
            .unwrap();
        thread
    }

    pub fn fake_acp_thread(
        project: Entity<Project>,
        cx: &mut TestAppContext,
    ) -> (Entity<AcpThread>, Entity<FakeAcpServer>) {
        let (stdin_tx, stdin_rx) = async_pipe::pipe();
        let (stdout_tx, stdout_rx) = async_pipe::pipe();
        let thread = cx.update(|cx| cx.new(|cx| AcpThread::fake(stdin_tx, stdout_rx, project, cx)));
        let agent = cx.update(|cx| cx.new(|cx| FakeAcpServer::new(stdin_rx, stdout_tx, cx)));
        (thread, agent)
    }

    pub struct FakeAcpServer {
        connection: acp::ClientConnection,
        _io_task: Task<()>,
        on_user_message: Option<
            Rc<
                dyn Fn(
                    acp::SendUserMessageParams,
                    Entity<FakeAcpServer>,
                    AsyncApp,
                ) -> LocalBoxFuture<'static, Result<(), acp::Error>>,
            >,
        >,
    }

    #[derive(Clone)]
    struct FakeAgent {
        server: Entity<FakeAcpServer>,
        cx: AsyncApp,
    }

    impl acp::Agent for FakeAgent {
        async fn initialize(&self) -> Result<acp::InitializeResponse, acp::Error> {
            Ok(acp::InitializeResponse {
                is_authenticated: true,
            })
        }

        async fn authenticate(&self) -> Result<(), acp::Error> {
            Ok(())
        }

        async fn cancel_send_message(&self) -> Result<(), acp::Error> {
            Ok(())
        }

        async fn send_user_message(
            &self,
            request: acp::SendUserMessageParams,
        ) -> Result<(), acp::Error> {
            let mut cx = self.cx.clone();
            let handler = self
                .server
                .update(&mut cx, |server, _| server.on_user_message.clone())
                .ok()
                .flatten();
            if let Some(handler) = handler {
                handler(request, self.server.clone(), self.cx.clone()).await
            } else {
                Err(anyhow::anyhow!("No handler for on_user_message").into())
            }
        }
    }

    impl FakeAcpServer {
        fn new(stdin: PipeReader, stdout: PipeWriter, cx: &Context<Self>) -> Self {
            let agent = FakeAgent {
                server: cx.entity(),
                cx: cx.to_async(),
            };
            let foreground_executor = cx.foreground_executor().clone();

            let (connection, io_fut) = acp::ClientConnection::connect_to_client(
                agent.clone(),
                stdout,
                stdin,
                move |fut| {
                    foreground_executor.spawn(fut).detach();
                },
            );
            FakeAcpServer {
                connection: connection,
                on_user_message: None,
                _io_task: cx.background_spawn(async move {
                    io_fut.await.log_err();
                }),
            }
        }

        fn on_user_message<F>(
            &mut self,
            handler: impl for<'a> Fn(acp::SendUserMessageParams, Entity<FakeAcpServer>, AsyncApp) -> F
            + 'static,
        ) where
            F: Future<Output = Result<(), acp::Error>> + 'static,
        {
            self.on_user_message
                .replace(Rc::new(move |request, server, cx| {
                    handler(request, server, cx).boxed_local()
                }));
        }

        fn send_to_zed<T: acp::ClientRequest + 'static>(
            &self,
            message: T,
        ) -> BoxedLocal<Result<T::Response>> {
            self.connection
                .request(message)
                .map(|f| f.map_err(|err| anyhow!(err)))
                .boxed_local()
        }
    }
}
