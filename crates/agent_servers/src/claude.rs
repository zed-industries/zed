mod mcp_server;
pub mod tools;

use collections::HashMap;
use context_server::listener::McpServerTool;
use project::Project;
use settings::SettingsStore;
use smol::process::Child;
use std::cell::RefCell;
use std::fmt::Display;
use std::path::Path;
use std::pin::pin;
use std::rc::Rc;
use uuid::Uuid;

use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use futures::channel::oneshot;
use futures::{AsyncBufReadExt, AsyncWriteExt};
use futures::{
    AsyncRead, AsyncWrite, FutureExt, StreamExt,
    channel::mpsc::{self, UnboundedReceiver, UnboundedSender},
    io::BufReader,
    select_biased,
};
use gpui::{App, AppContext, AsyncApp, Entity, Task, WeakEntity};
use serde::{Deserialize, Serialize};
use util::ResultExt;

use crate::claude::mcp_server::{ClaudeZedMcpServer, McpConfig};
use crate::claude::tools::ClaudeTool;
use crate::{AgentServer, AgentServerCommand, AllAgentServersSettings};
use acp_thread::{AcpThread, AgentConnection};

#[derive(Clone)]
pub struct ClaudeCode;

impl AgentServer for ClaudeCode {
    fn name(&self) -> &'static str {
        "Claude Code"
    }

    fn empty_state_headline(&self) -> &'static str {
        self.name()
    }

    fn empty_state_message(&self) -> &'static str {
        ""
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiClaude
    }

    fn connect(
        &self,
        _root_dir: &Path,
        _project: &Entity<Project>,
        _cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        let connection = ClaudeAgentConnection {
            sessions: Default::default(),
        };

        Task::ready(Ok(Rc::new(connection) as _))
    }
}

#[cfg(unix)]
fn send_interrupt(pid: libc::pid_t) -> anyhow::Result<()> {
    let pid = nix::unistd::Pid::from_raw(pid);

    nix::sys::signal::kill(pid, nix::sys::signal::SIGINT)
        .map_err(|e| anyhow!("Failed to interrupt process: {}", e))
}

#[cfg(windows)]
fn send_interrupt(_pid: i32) -> anyhow::Result<()> {
    panic!("Cancel not implemented on Windows")
}

struct ClaudeAgentConnection {
    sessions: Rc<RefCell<HashMap<acp::SessionId, ClaudeAgentSession>>>,
}

impl AgentConnection for ClaudeAgentConnection {
    fn name(&self) -> &'static str {
        ClaudeCode.name()
    }

    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut AsyncApp,
    ) -> Task<Result<Entity<AcpThread>>> {
        let cwd = cwd.to_owned();
        cx.spawn(async move |cx| {
            let (mut thread_tx, thread_rx) = watch::channel(WeakEntity::new_invalid());
            let permission_mcp_server = ClaudeZedMcpServer::new(thread_rx.clone(), cx).await?;

            let mut mcp_servers = HashMap::default();
            mcp_servers.insert(
                mcp_server::SERVER_NAME.to_string(),
                permission_mcp_server.server_config()?,
            );
            let mcp_config = McpConfig { mcp_servers };

            let mcp_config_file = tempfile::NamedTempFile::new()?;
            let (mcp_config_file, mcp_config_path) = mcp_config_file.into_parts();

            let mut mcp_config_file = smol::fs::File::from(mcp_config_file);
            mcp_config_file
                .write_all(serde_json::to_string(&mcp_config)?.as_bytes())
                .await?;
            mcp_config_file.flush().await?;

            let settings = cx.read_global(|settings: &SettingsStore, _| {
                settings.get::<AllAgentServersSettings>(None).claude.clone()
            })?;

            let Some(command) =
                AgentServerCommand::resolve("claude", &[], settings, &project, cx).await
            else {
                anyhow::bail!("Failed to find claude binary");
            };

            let (incoming_message_tx, mut incoming_message_rx) = mpsc::unbounded();
            let (outgoing_tx, outgoing_rx) = mpsc::unbounded();
            let (cancel_tx, mut cancel_rx) = mpsc::unbounded::<oneshot::Sender<Result<()>>>();

            let session_id = acp::SessionId(Uuid::new_v4().to_string().into());

            log::trace!("Starting session with id: {}", session_id);

            cx.background_spawn({
                let session_id = session_id.clone();
                async move {
                    let mut outgoing_rx = Some(outgoing_rx);
                    let mut mode = ClaudeSessionMode::Start;

                    loop {
                        let mut child = spawn_claude(
                            &command,
                            mode,
                            session_id.clone(),
                            &mcp_config_path,
                            &cwd,
                        )
                        .await?;
                        mode = ClaudeSessionMode::Resume;

                        let pid = child.id();
                        log::trace!("Spawned (pid: {})", pid);

                        let mut io_fut = pin!(
                            ClaudeAgentSession::handle_io(
                                outgoing_rx.take().unwrap(),
                                incoming_message_tx.clone(),
                                child.stdin.take().unwrap(),
                                child.stdout.take().unwrap(),
                            )
                            .fuse()
                        );

                        select_biased! {
                            done_tx = cancel_rx.next() => {
                                if let Some(done_tx) = done_tx {
                                    log::trace!("Interrupted (pid: {})", pid);
                                    let result = send_interrupt(pid as i32);
                                    outgoing_rx.replace(io_fut.await?);
                                    done_tx.send(result).log_err();
                                    continue;
                                }
                            }
                            result = io_fut => {
                                result?;
                            }
                        }

                        log::trace!("Stopped (pid: {})", pid);
                        break;
                    }

                    drop(mcp_config_path);
                    anyhow::Ok(())
                }
            })
            .detach();

            let end_turn_tx = Rc::new(RefCell::new(None));
            let handler_task = cx.spawn({
                let end_turn_tx = end_turn_tx.clone();
                let thread_rx = thread_rx.clone();
                async move |cx| {
                    while let Some(message) = incoming_message_rx.next().await {
                        ClaudeAgentSession::handle_message(
                            thread_rx.clone(),
                            message,
                            end_turn_tx.clone(),
                            cx,
                        )
                        .await
                    }
                }
            });

            let thread =
                cx.new(|cx| AcpThread::new(self.clone(), project, session_id.clone(), cx))?;

            thread_tx.send(thread.downgrade())?;

            let session = ClaudeAgentSession {
                outgoing_tx,
                end_turn_tx,
                cancel_tx,
                _handler_task: handler_task,
                _mcp_server: Some(permission_mcp_server),
            };

            self.sessions.borrow_mut().insert(session_id, session);

            Ok(thread)
        })
    }

    fn authenticate(&self, _cx: &mut App) -> Task<Result<()>> {
        Task::ready(Err(anyhow!("Authentication not supported")))
    }

    fn prompt(&self, params: acp::PromptToolArguments, cx: &mut App) -> Task<Result<()>> {
        let sessions = self.sessions.borrow();
        let Some(session) = sessions.get(&params.session_id) else {
            return Task::ready(Err(anyhow!(
                "Attempted to send message to nonexistent session {}",
                params.session_id
            )));
        };

        let (tx, rx) = oneshot::channel();
        session.end_turn_tx.borrow_mut().replace(tx);

        let mut content = String::new();
        for chunk in params.prompt {
            match chunk {
                acp::ContentBlock::Text(text_content) => {
                    content.push_str(&text_content.text);
                }
                acp::ContentBlock::ResourceLink(resource_link) => {
                    content.push_str(&format!("@{}", resource_link.uri));
                }
                acp::ContentBlock::Audio(_)
                | acp::ContentBlock::Image(_)
                | acp::ContentBlock::Resource(_) => {
                    // TODO
                }
            }
        }

        if let Err(err) = session.outgoing_tx.unbounded_send(SdkMessage::User {
            message: Message {
                role: Role::User,
                content: Content::UntaggedText(content),
                id: None,
                model: None,
                stop_reason: None,
                stop_sequence: None,
                usage: None,
            },
            session_id: Some(params.session_id.to_string()),
        }) {
            return Task::ready(Err(anyhow!(err)));
        }

        cx.foreground_executor().spawn(async move {
            rx.await??;
            Ok(())
        })
    }

    fn cancel(&self, session_id: &acp::SessionId, cx: &mut App) {
        let sessions = self.sessions.borrow();
        let Some(session) = sessions.get(&session_id) else {
            log::warn!("Attempted to cancel nonexistent session {}", session_id);
            return;
        };

        let (done_tx, done_rx) = oneshot::channel();
        if session
            .cancel_tx
            .unbounded_send(done_tx)
            .log_err()
            .is_some()
        {
            let end_turn_tx = session.end_turn_tx.clone();
            cx.foreground_executor()
                .spawn(async move {
                    done_rx.await??;
                    if let Some(end_turn_tx) = end_turn_tx.take() {
                        end_turn_tx.send(Ok(())).ok();
                    }
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
        }
    }
}

#[derive(Clone, Copy)]
enum ClaudeSessionMode {
    Start,
    Resume,
}

async fn spawn_claude(
    command: &AgentServerCommand,
    mode: ClaudeSessionMode,
    session_id: acp::SessionId,
    mcp_config_path: &Path,
    root_dir: &Path,
) -> Result<Child> {
    let child = util::command::new_smol_command(&command.path)
        .args([
            "--input-format",
            "stream-json",
            "--output-format",
            "stream-json",
            "--print",
            "--verbose",
            "--mcp-config",
            mcp_config_path.to_string_lossy().as_ref(),
            "--permission-prompt-tool",
            &format!(
                "mcp__{}__{}",
                mcp_server::SERVER_NAME,
                mcp_server::PermissionTool::NAME,
            ),
            "--allowedTools",
            &format!(
                "mcp__{}__{},mcp__{}__{}",
                mcp_server::SERVER_NAME,
                mcp_server::EditTool::NAME,
                mcp_server::SERVER_NAME,
                mcp_server::ReadTool::NAME
            ),
            "--disallowedTools",
            "Read,Edit",
        ])
        .args(match mode {
            ClaudeSessionMode::Start => ["--session-id".to_string(), session_id.to_string()],
            ClaudeSessionMode::Resume => ["--resume".to_string(), session_id.to_string()],
        })
        .args(command.args.iter().map(|arg| arg.as_str()))
        .current_dir(root_dir)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .kill_on_drop(true)
        .spawn()?;

    Ok(child)
}

struct ClaudeAgentSession {
    outgoing_tx: UnboundedSender<SdkMessage>,
    end_turn_tx: Rc<RefCell<Option<oneshot::Sender<Result<()>>>>>,
    cancel_tx: UnboundedSender<oneshot::Sender<Result<()>>>,
    _mcp_server: Option<ClaudeZedMcpServer>,
    _handler_task: Task<()>,
}

impl ClaudeAgentSession {
    async fn handle_message(
        mut thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
        message: SdkMessage,
        end_turn_tx: Rc<RefCell<Option<oneshot::Sender<Result<()>>>>>,
        cx: &mut AsyncApp,
    ) {
        match message {
            SdkMessage::Assistant {
                message,
                session_id: _,
            }
            | SdkMessage::User {
                message,
                session_id: _,
            } => {
                let Some(thread) = thread_rx
                    .recv()
                    .await
                    .log_err()
                    .and_then(|entity| entity.upgrade())
                else {
                    log::error!("Received an SDK message but thread is gone");
                    return;
                };

                for chunk in message.content.chunks() {
                    match chunk {
                        ContentChunk::Text { text } | ContentChunk::UntaggedText(text) => {
                            thread
                                .update(cx, |thread, cx| {
                                    thread.push_assistant_chunk(text.into(), false, cx)
                                })
                                .log_err();
                        }
                        ContentChunk::ToolUse { id, name, input } => {
                            let claude_tool = ClaudeTool::infer(&name, input);

                            thread
                                .update(cx, |thread, cx| {
                                    if let ClaudeTool::TodoWrite(Some(params)) = claude_tool {
                                        thread.update_plan(
                                            acp::Plan {
                                                entries: params
                                                    .todos
                                                    .into_iter()
                                                    .map(Into::into)
                                                    .collect(),
                                            },
                                            cx,
                                        )
                                    } else {
                                        thread.upsert_tool_call(
                                            claude_tool.as_acp(acp::ToolCallId(id.into())),
                                            cx,
                                        );
                                    }
                                })
                                .log_err();
                        }
                        ContentChunk::ToolResult {
                            content,
                            tool_use_id,
                        } => {
                            let content = content.to_string();
                            thread
                                .update(cx, |thread, cx| {
                                    thread.update_tool_call(
                                        acp::ToolCallId(tool_use_id.into()),
                                        acp::ToolCallStatus::Completed,
                                        (!content.is_empty()).then(|| vec![content.into()]),
                                        cx,
                                    )
                                })
                                .log_err();
                        }
                        ContentChunk::Image
                        | ContentChunk::Document
                        | ContentChunk::Thinking
                        | ContentChunk::RedactedThinking
                        | ContentChunk::WebSearchToolResult => {
                            thread
                                .update(cx, |thread, cx| {
                                    thread.push_assistant_chunk(
                                        format!("Unsupported content: {:?}", chunk).into(),
                                        false,
                                        cx,
                                    )
                                })
                                .log_err();
                        }
                    }
                }
            }
            SdkMessage::Result {
                is_error, subtype, ..
            } => {
                if let Some(end_turn_tx) = end_turn_tx.borrow_mut().take() {
                    if is_error {
                        end_turn_tx.send(Err(anyhow!("Error: {subtype}"))).ok();
                    } else {
                        end_turn_tx.send(Ok(())).ok();
                    }
                }
            }
            SdkMessage::System { .. } => {}
        }
    }

    async fn handle_io(
        mut outgoing_rx: UnboundedReceiver<SdkMessage>,
        incoming_tx: UnboundedSender<SdkMessage>,
        mut outgoing_bytes: impl Unpin + AsyncWrite,
        incoming_bytes: impl Unpin + AsyncRead,
    ) -> Result<UnboundedReceiver<SdkMessage>> {
        let mut output_reader = BufReader::new(incoming_bytes);
        let mut outgoing_line = Vec::new();
        let mut incoming_line = String::new();
        loop {
            select_biased! {
                message = outgoing_rx.next() => {
                    if let Some(message) = message {
                        outgoing_line.clear();
                        serde_json::to_writer(&mut outgoing_line, &message)?;
                        log::trace!("send: {}", String::from_utf8_lossy(&outgoing_line));
                        outgoing_line.push(b'\n');
                        outgoing_bytes.write_all(&outgoing_line).await.ok();
                    } else {
                        break;
                    }
                }
                bytes_read = output_reader.read_line(&mut incoming_line).fuse() => {
                    if bytes_read? == 0 {
                        break
                    }
                    log::trace!("recv: {}", &incoming_line);
                    match serde_json::from_str::<SdkMessage>(&incoming_line) {
                        Ok(message) => {
                            incoming_tx.unbounded_send(message).log_err();
                        }
                        Err(error) => {
                            log::error!("failed to parse incoming message: {error}. Raw: {incoming_line}");
                        }
                    }
                    incoming_line.clear();
                }
            }
        }

        Ok(outgoing_rx)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    role: Role,
    content: Content,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<Usage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Content {
    UntaggedText(String),
    Chunks(Vec<ContentChunk>),
}

impl Content {
    pub fn chunks(self) -> impl Iterator<Item = ContentChunk> {
        match self {
            Self::Chunks(chunks) => chunks.into_iter(),
            Self::UntaggedText(text) => vec![ContentChunk::Text { text: text.clone() }].into_iter(),
        }
    }
}

impl Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Content::UntaggedText(txt) => write!(f, "{}", txt),
            Content::Chunks(chunks) => {
                for chunk in chunks {
                    write!(f, "{}", chunk)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ContentChunk {
    Text {
        text: String,
    },
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    ToolResult {
        content: Content,
        tool_use_id: String,
    },
    // TODO
    Image,
    Document,
    Thinking,
    RedactedThinking,
    WebSearchToolResult,
    #[serde(untagged)]
    UntaggedText(String),
}

impl Display for ContentChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentChunk::Text { text } => write!(f, "{}", text),
            ContentChunk::UntaggedText(text) => write!(f, "{}", text),
            ContentChunk::ToolResult { content, .. } => write!(f, "{}", content),
            ContentChunk::Image
            | ContentChunk::Document
            | ContentChunk::Thinking
            | ContentChunk::RedactedThinking
            | ContentChunk::ToolUse { .. }
            | ContentChunk::WebSearchToolResult => {
                write!(f, "\n{:?}\n", &self)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Usage {
    input_tokens: u32,
    cache_creation_input_tokens: u32,
    cache_read_input_tokens: u32,
    output_tokens: u32,
    service_tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Role {
    System,
    Assistant,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MessageParam {
    role: Role,
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum SdkMessage {
    // An assistant message
    Assistant {
        message: Message, // from Anthropic SDK
        #[serde(skip_serializing_if = "Option::is_none")]
        session_id: Option<String>,
    },

    // A user message
    User {
        message: Message, // from Anthropic SDK
        #[serde(skip_serializing_if = "Option::is_none")]
        session_id: Option<String>,
    },

    // Emitted as the last message in a conversation
    Result {
        subtype: ResultErrorType,
        duration_ms: f64,
        duration_api_ms: f64,
        is_error: bool,
        num_turns: i32,
        #[serde(skip_serializing_if = "Option::is_none")]
        result: Option<String>,
        session_id: String,
        total_cost_usd: f64,
    },
    // Emitted as the first message at the start of a conversation
    System {
        cwd: String,
        session_id: String,
        tools: Vec<String>,
        model: String,
        mcp_servers: Vec<McpServer>,
        #[serde(rename = "apiKeySource")]
        api_key_source: String,
        #[serde(rename = "permissionMode")]
        permission_mode: PermissionMode,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ResultErrorType {
    Success,
    ErrorMaxTurns,
    ErrorDuringExecution,
}

impl Display for ResultErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultErrorType::Success => write!(f, "success"),
            ResultErrorType::ErrorMaxTurns => write!(f, "error_max_turns"),
            ResultErrorType::ErrorDuringExecution => write!(f, "error_during_execution"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct McpServer {
    name: String,
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum PermissionMode {
    Default,
    AcceptEdits,
    BypassPermissions,
    Plan,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use serde_json::json;

    crate::common_e2e_tests!(ClaudeCode);

    pub fn local_command() -> AgentServerCommand {
        AgentServerCommand {
            path: "claude".into(),
            args: vec![],
            env: None,
        }
    }

    #[test]
    fn test_deserialize_content_untagged_text() {
        let json = json!("Hello, world!");
        let content: Content = serde_json::from_value(json).unwrap();
        match content {
            Content::UntaggedText(text) => assert_eq!(text, "Hello, world!"),
            _ => panic!("Expected UntaggedText variant"),
        }
    }

    #[test]
    fn test_deserialize_content_chunks() {
        let json = json!([
            {
                "type": "text",
                "text": "Hello"
            },
            {
                "type": "tool_use",
                "id": "tool_123",
                "name": "calculator",
                "input": {"operation": "add", "a": 1, "b": 2}
            }
        ]);
        let content: Content = serde_json::from_value(json).unwrap();
        match content {
            Content::Chunks(chunks) => {
                assert_eq!(chunks.len(), 2);
                match &chunks[0] {
                    ContentChunk::Text { text } => assert_eq!(text, "Hello"),
                    _ => panic!("Expected Text chunk"),
                }
                match &chunks[1] {
                    ContentChunk::ToolUse { id, name, input } => {
                        assert_eq!(id, "tool_123");
                        assert_eq!(name, "calculator");
                        assert_eq!(input["operation"], "add");
                        assert_eq!(input["a"], 1);
                        assert_eq!(input["b"], 2);
                    }
                    _ => panic!("Expected ToolUse chunk"),
                }
            }
            _ => panic!("Expected Chunks variant"),
        }
    }

    #[test]
    fn test_deserialize_tool_result_untagged_text() {
        let json = json!({
            "type": "tool_result",
            "content": "Result content",
            "tool_use_id": "tool_456"
        });
        let chunk: ContentChunk = serde_json::from_value(json).unwrap();
        match chunk {
            ContentChunk::ToolResult {
                content,
                tool_use_id,
            } => {
                match content {
                    Content::UntaggedText(text) => assert_eq!(text, "Result content"),
                    _ => panic!("Expected UntaggedText content"),
                }
                assert_eq!(tool_use_id, "tool_456");
            }
            _ => panic!("Expected ToolResult variant"),
        }
    }

    #[test]
    fn test_deserialize_tool_result_chunks() {
        let json = json!({
            "type": "tool_result",
            "content": [
                {
                    "type": "text",
                    "text": "Processing complete"
                },
                {
                    "type": "text",
                    "text": "Result: 42"
                }
            ],
            "tool_use_id": "tool_789"
        });
        let chunk: ContentChunk = serde_json::from_value(json).unwrap();
        match chunk {
            ContentChunk::ToolResult {
                content,
                tool_use_id,
            } => {
                match content {
                    Content::Chunks(chunks) => {
                        assert_eq!(chunks.len(), 2);
                        match &chunks[0] {
                            ContentChunk::Text { text } => assert_eq!(text, "Processing complete"),
                            _ => panic!("Expected Text chunk"),
                        }
                        match &chunks[1] {
                            ContentChunk::Text { text } => assert_eq!(text, "Result: 42"),
                            _ => panic!("Expected Text chunk"),
                        }
                    }
                    _ => panic!("Expected Chunks content"),
                }
                assert_eq!(tool_use_id, "tool_789");
            }
            _ => panic!("Expected ToolResult variant"),
        }
    }
}
