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
use util::{ResultExt, debug_panic};

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
        "How can I help you today?"
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

struct ClaudeAgentConnection {
    sessions: Rc<RefCell<HashMap<acp::SessionId, ClaudeAgentSession>>>,
}

impl AgentConnection for ClaudeAgentConnection {
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

            let Some(command) = AgentServerCommand::resolve(
                "claude",
                &[],
                Some(&util::paths::home_dir().join(".claude/local/claude")),
                settings,
                &project,
                cx,
            )
            .await
            else {
                anyhow::bail!("Failed to find claude binary");
            };

            let (incoming_message_tx, mut incoming_message_rx) = mpsc::unbounded();
            let (outgoing_tx, outgoing_rx) = mpsc::unbounded();

            let session_id = acp::SessionId(Uuid::new_v4().to_string().into());

            log::trace!("Starting session with id: {}", session_id);

            let mut child = spawn_claude(
                &command,
                ClaudeSessionMode::Start,
                session_id.clone(),
                &mcp_config_path,
                &cwd,
            )?;

            let stdin = child.stdin.take().unwrap();
            let stdout = child.stdout.take().unwrap();

            let pid = child.id();
            log::trace!("Spawned (pid: {})", pid);

            cx.background_spawn(async move {
                let mut outgoing_rx = Some(outgoing_rx);

                ClaudeAgentSession::handle_io(
                    outgoing_rx.take().unwrap(),
                    incoming_message_tx.clone(),
                    stdin,
                    stdout,
                )
                .await?;

                log::trace!("Stopped (pid: {})", pid);

                drop(mcp_config_path);
                anyhow::Ok(())
            })
            .detach();

            let turn_state = Rc::new(RefCell::new(TurnState::None));

            let handler_task = cx.spawn({
                let turn_state = turn_state.clone();
                let mut thread_rx = thread_rx.clone();
                async move |cx| {
                    while let Some(message) = incoming_message_rx.next().await {
                        ClaudeAgentSession::handle_message(
                            thread_rx.clone(),
                            message,
                            turn_state.clone(),
                            cx,
                        )
                        .await
                    }

                    if let Some(status) = child.status().await.log_err() {
                        if let Some(thread) = thread_rx.recv().await.ok() {
                            thread
                                .update(cx, |thread, cx| {
                                    thread.emit_server_exited(status, cx);
                                })
                                .ok();
                        }
                    }
                }
            });

            let thread = cx.new(|cx| {
                AcpThread::new("Claude Code", self.clone(), project, session_id.clone(), cx)
            })?;

            thread_tx.send(thread.downgrade())?;

            let session = ClaudeAgentSession {
                outgoing_tx,
                turn_state,
                _handler_task: handler_task,
                _mcp_server: Some(permission_mcp_server),
            };

            self.sessions.borrow_mut().insert(session_id, session);

            Ok(thread)
        })
    }

    fn auth_methods(&self) -> &[acp::AuthMethod] {
        &[]
    }

    fn authenticate(&self, _: acp::AuthMethodId, _cx: &mut App) -> Task<Result<()>> {
        Task::ready(Err(anyhow!("Authentication not supported")))
    }

    fn prompt(
        &self,
        params: acp::PromptRequest,
        cx: &mut App,
    ) -> Task<Result<acp::PromptResponse>> {
        let sessions = self.sessions.borrow();
        let Some(session) = sessions.get(&params.session_id) else {
            return Task::ready(Err(anyhow!(
                "Attempted to send message to nonexistent session {}",
                params.session_id
            )));
        };

        let (end_tx, end_rx) = oneshot::channel();
        session.turn_state.replace(TurnState::InProgress { end_tx });

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

        cx.foreground_executor().spawn(async move { end_rx.await? })
    }

    fn cancel(&self, session_id: &acp::SessionId, _cx: &mut App) {
        let sessions = self.sessions.borrow();
        let Some(session) = sessions.get(&session_id) else {
            log::warn!("Attempted to cancel nonexistent session {}", session_id);
            return;
        };

        let request_id = new_request_id();

        let turn_state = session.turn_state.take();
        let TurnState::InProgress { end_tx } = turn_state else {
            // Already cancelled or idle, put it back
            session.turn_state.replace(turn_state);
            return;
        };

        session.turn_state.replace(TurnState::CancelRequested {
            end_tx,
            request_id: request_id.clone(),
        });

        session
            .outgoing_tx
            .unbounded_send(SdkMessage::ControlRequest {
                request_id,
                request: ControlRequest::Interrupt,
            })
            .log_err();
    }
}

#[derive(Clone, Copy)]
enum ClaudeSessionMode {
    Start,
    #[expect(dead_code)]
    Resume,
}

fn spawn_claude(
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
    turn_state: Rc<RefCell<TurnState>>,
    _mcp_server: Option<ClaudeZedMcpServer>,
    _handler_task: Task<()>,
}

#[derive(Debug, Default)]
enum TurnState {
    #[default]
    None,
    InProgress {
        end_tx: oneshot::Sender<Result<acp::PromptResponse>>,
    },
    CancelRequested {
        end_tx: oneshot::Sender<Result<acp::PromptResponse>>,
        request_id: String,
    },
    CancelConfirmed {
        end_tx: oneshot::Sender<Result<acp::PromptResponse>>,
    },
}

impl TurnState {
    fn is_cancelled(&self) -> bool {
        matches!(self, TurnState::CancelConfirmed { .. })
    }

    fn end_tx(self) -> Option<oneshot::Sender<Result<acp::PromptResponse>>> {
        match self {
            TurnState::None => None,
            TurnState::InProgress { end_tx, .. } => Some(end_tx),
            TurnState::CancelRequested { end_tx, .. } => Some(end_tx),
            TurnState::CancelConfirmed { end_tx } => Some(end_tx),
        }
    }

    fn confirm_cancellation(self, id: &str) -> Self {
        match self {
            TurnState::CancelRequested { request_id, end_tx } if request_id == id => {
                TurnState::CancelConfirmed { end_tx }
            }
            _ => self,
        }
    }
}

impl ClaudeAgentSession {
    async fn handle_message(
        mut thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
        message: SdkMessage,
        turn_state: Rc<RefCell<TurnState>>,
        cx: &mut AsyncApp,
    ) {
        match message {
            // we should only be sending these out, they don't need to be in the thread
            SdkMessage::ControlRequest { .. } => {}
            SdkMessage::User {
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
                            if !turn_state.borrow().is_cancelled() {
                                thread
                                    .update(cx, |thread, cx| {
                                        thread.push_user_content_block(text.into(), cx)
                                    })
                                    .log_err();
                            }
                        }
                        ContentChunk::ToolResult {
                            content,
                            tool_use_id,
                        } => {
                            let content = content.to_string();
                            thread
                                .update(cx, |thread, cx| {
                                    thread.update_tool_call(
                                        acp::ToolCallUpdate {
                                            id: acp::ToolCallId(tool_use_id.into()),
                                            fields: acp::ToolCallUpdateFields {
                                                status: if turn_state.borrow().is_cancelled() {
                                                    // Do not set to completed if turn was cancelled
                                                    None
                                                } else {
                                                    Some(acp::ToolCallStatus::Completed)
                                                },
                                                content: (!content.is_empty())
                                                    .then(|| vec![content.into()]),
                                                ..Default::default()
                                            },
                                        },
                                        cx,
                                    )
                                })
                                .log_err();
                        }
                        ContentChunk::Thinking { .. }
                        | ContentChunk::RedactedThinking
                        | ContentChunk::ToolUse { .. } => {
                            debug_panic!(
                                "Should not get {:?} with role: assistant. should we handle this?",
                                chunk
                            );
                        }

                        ContentChunk::Image
                        | ContentChunk::Document
                        | ContentChunk::WebSearchToolResult => {
                            thread
                                .update(cx, |thread, cx| {
                                    thread.push_assistant_content_block(
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
            SdkMessage::Assistant {
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
                                    thread.push_assistant_content_block(text.into(), false, cx)
                                })
                                .log_err();
                        }
                        ContentChunk::Thinking { thinking } => {
                            thread
                                .update(cx, |thread, cx| {
                                    thread.push_assistant_content_block(thinking.into(), true, cx)
                                })
                                .log_err();
                        }
                        ContentChunk::RedactedThinking => {
                            thread
                                .update(cx, |thread, cx| {
                                    thread.push_assistant_content_block(
                                        "[REDACTED]".into(),
                                        true,
                                        cx,
                                    )
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
                        ContentChunk::ToolResult { .. } | ContentChunk::WebSearchToolResult => {
                            debug_panic!(
                                "Should not get tool results with role: assistant. should we handle this?"
                            );
                        }
                        ContentChunk::Image | ContentChunk::Document => {
                            thread
                                .update(cx, |thread, cx| {
                                    thread.push_assistant_content_block(
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
                is_error,
                subtype,
                result,
                ..
            } => {
                let turn_state = turn_state.take();
                let was_cancelled = turn_state.is_cancelled();
                let Some(end_turn_tx) = turn_state.end_tx() else {
                    debug_panic!("Received `SdkMessage::Result` but there wasn't an active turn");
                    return;
                };

                if is_error || (!was_cancelled && subtype == ResultErrorType::ErrorDuringExecution)
                {
                    end_turn_tx
                        .send(Err(anyhow!(
                            "Error: {}",
                            result.unwrap_or_else(|| subtype.to_string())
                        )))
                        .ok();
                } else {
                    let stop_reason = match subtype {
                        ResultErrorType::Success => acp::StopReason::EndTurn,
                        ResultErrorType::ErrorMaxTurns => acp::StopReason::MaxTurnRequests,
                        ResultErrorType::ErrorDuringExecution => acp::StopReason::Cancelled,
                    };
                    end_turn_tx
                        .send(Ok(acp::PromptResponse { stop_reason }))
                        .ok();
                }
            }
            SdkMessage::ControlResponse { response } => {
                if matches!(response.subtype, ResultErrorType::Success) {
                    let new_state = turn_state.take().confirm_cancellation(&response.request_id);
                    turn_state.replace(new_state);
                } else {
                    log::error!("Control response error: {:?}", response);
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
    Thinking {
        thinking: String,
    },
    RedactedThinking,
    // TODO
    Image,
    Document,
    WebSearchToolResult,
    #[serde(untagged)]
    UntaggedText(String),
}

impl Display for ContentChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentChunk::Text { text } => write!(f, "{}", text),
            ContentChunk::Thinking { thinking } => write!(f, "Thinking: {}", thinking),
            ContentChunk::RedactedThinking => write!(f, "Thinking: [REDACTED]"),
            ContentChunk::UntaggedText(text) => write!(f, "{}", text),
            ContentChunk::ToolResult { content, .. } => write!(f, "{}", content),
            ContentChunk::Image
            | ContentChunk::Document
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
    /// Messages used to control the conversation, outside of chat messages to the model
    ControlRequest {
        request_id: String,
        request: ControlRequest,
    },
    /// Response to a control request
    ControlResponse { response: ControlResponse },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "subtype", rename_all = "snake_case")]
enum ControlRequest {
    /// Cancel the current conversation
    Interrupt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ControlResponse {
    request_id: String,
    subtype: ResultErrorType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
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

fn new_request_id() -> String {
    use rand::Rng;
    // In the Claude Code TS SDK they just generate a random 12 character string,
    // `Math.random().toString(36).substring(2, 15)`
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
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
    use crate::e2e_tests;
    use gpui::TestAppContext;
    use serde_json::json;

    crate::common_e2e_tests!(ClaudeCode, allow_option_id = "allow");

    pub fn local_command() -> AgentServerCommand {
        AgentServerCommand {
            path: "claude".into(),
            args: vec![],
            env: None,
        }
    }

    #[gpui::test]
    #[cfg_attr(not(feature = "e2e"), ignore)]
    async fn test_todo_plan(cx: &mut TestAppContext) {
        let fs = e2e_tests::init_test(cx).await;
        let project = Project::test(fs, [], cx).await;
        let thread =
            e2e_tests::new_test_thread(ClaudeCode, project.clone(), "/private/tmp", cx).await;

        thread
            .update(cx, |thread, cx| {
                thread.send_raw(
                    "Create a todo plan for initializing a new React app. I'll follow it myself, do not execute on it.",
                    cx,
                )
            })
            .await
            .unwrap();

        let mut entries_len = 0;

        thread.read_with(cx, |thread, _| {
            entries_len = thread.plan().entries.len();
            assert!(thread.plan().entries.len() > 0, "Empty plan");
        });

        thread
            .update(cx, |thread, cx| {
                thread.send_raw(
                    "Mark the first entry status as in progress without acting on it.",
                    cx,
                )
            })
            .await
            .unwrap();

        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                thread.plan().entries[0].status,
                acp::PlanEntryStatus::InProgress
            ));
            assert_eq!(thread.plan().entries.len(), entries_len);
        });

        thread
            .update(cx, |thread, cx| {
                thread.send_raw(
                    "Now mark the first entry as completed without acting on it.",
                    cx,
                )
            })
            .await
            .unwrap();

        thread.read_with(cx, |thread, _| {
            assert!(matches!(
                thread.plan().entries[0].status,
                acp::PlanEntryStatus::Completed
            ));
            assert_eq!(thread.plan().entries.len(), entries_len);
        });
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
