mod permission_mcp_server;

use collections::HashMap;
use project::Project;
use std::cell::RefCell;
use std::fmt::Display;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;

use agentic_coding_protocol::{
    self as acp, AnyAgentRequest, AnyAgentResult, Client, ProtocolVersion, PushToolCallParams,
    StreamAssistantMessageChunkParams, ToolCallContent, UpdateToolCallParams,
};
use anyhow::{Result, anyhow};
use futures::channel::oneshot;
use futures::future::LocalBoxFuture;
use futures::{AsyncBufReadExt, AsyncWriteExt};
use futures::{
    AsyncRead, AsyncWrite, FutureExt, StreamExt,
    channel::mpsc::{self, UnboundedReceiver, UnboundedSender},
    io::BufReader,
    select_biased,
};
use gpui::{App, AppContext, Entity, Task};
use serde::{Deserialize, Serialize};
use util::ResultExt;

use crate::AgentServer;
use crate::claude::permission_mcp_server::PermissionMcpServer;
use acp_thread::{AcpClientDelegate, AcpThread, AgentConnection};

impl AgentConnection for ClaudeAgentConnection {
    /// Send a request to the agent and wait for a response.

    fn request_any(
        &self,
        params: AnyAgentRequest,
    ) -> LocalBoxFuture<'static, Result<acp::AnyAgentResult>> {
        let end_turn_tx = self.end_turn_tx.clone();
        let outgoing_tx = self.outgoing_tx.clone();
        async move {
            match params {
                // todo: consider sending an empty request so we get the init response?
                AnyAgentRequest::InitializeParams(_) => Ok(AnyAgentResult::InitializeResponse(
                    acp::InitializeResponse {
                        is_authenticated: true,
                        protocol_version: ProtocolVersion::latest(),
                    },
                )),
                AnyAgentRequest::AuthenticateParams(_) => {
                    Err(anyhow!("Authentication not supported"))
                }
                AnyAgentRequest::SendUserMessageParams(message) => {
                    let (tx, rx) = oneshot::channel();
                    end_turn_tx.borrow_mut().replace(tx);
                    let mut content = String::new();
                    for chunk in message.chunks {
                        match chunk {
                            agentic_coding_protocol::UserMessageChunk::Text { text } => {
                                content.push_str(&text)
                            }
                            agentic_coding_protocol::UserMessageChunk::Path { path } => {
                                content.push_str(&format!("@{path:?}"))
                            }
                        }
                    }
                    outgoing_tx.unbounded_send(SdkMessage::User {
                        message: Message {
                            role: Role::User,
                            content: vec![MessageContent::Text { text: content }],
                            id: None,
                            model: None,
                            stop_reason: None,
                            stop_sequence: None,
                            usage: None,
                        },
                        session_id: None,
                    })?;
                    rx.await??;
                    Ok(AnyAgentResult::SendUserMessageResponse(
                        acp::SendUserMessageResponse,
                    ))
                }
                AnyAgentRequest::CancelSendMessageParams(_) => Ok(
                    AnyAgentResult::CancelSendMessageResponse(acp::CancelSendMessageResponse),
                ),
            }
        }
        .boxed_local()
    }
}

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
        ui::IconName::AiAnthropic
    }

    fn new_thread(
        &self,
        root_dir: &Path,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        let project = project.clone();
        let root_dir = root_dir.to_path_buf();
        let title = self.name().into();
        cx.spawn(async move |cx| {
            let delegate_rc = Rc::new(RefCell::new(None));
            let tool_id_map = Rc::new(RefCell::new(HashMap::default()));

            let permission_mcp_server =
                PermissionMcpServer::new(delegate_rc.clone(), tool_id_map.clone(), cx).await?;

            let mut mcp_servers = HashMap::default();
            mcp_servers.insert(
                permission_mcp_server::SERVER_NAME.to_string(),
                permission_mcp_server.server_config()?,
            );
            let mcp_config = McpConfig { mcp_servers };

            let mut mcp_config_file = tempfile::Builder::new().tempfile()?;
            // todo! async
            mcp_config_file.write_all(serde_json::to_string(&mcp_config)?.as_bytes())?;
            mcp_config_file.flush()?;
            let mcp_config_path = mcp_config_file.into_temp_path();

            let command = which::which("claude")?;

            let mut child = util::command::new_smol_command(&command)
                .args([
                    "--input-format",
                    "stream-json",
                    "--output-format",
                    "stream-json",
                    "--print",
                    "--verbose",
                    "--mcp-config",
                    &mcp_config_path.to_string_lossy().to_string(),
                    "--permission-prompt-tool",
                    &format!(
                        "mcp__{}__{}",
                        permission_mcp_server::SERVER_NAME,
                        permission_mcp_server::TOOL_NAME
                    ),
                    "--allowedTools",
                    "mcp__zed__Read,mcp__zed__Edit",
                    "--disallowedTools",
                    "Read,Edit",
                ])
                .current_dir(root_dir)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::inherit())
                .kill_on_drop(true)
                .spawn()?;

            let stdin = child.stdin.take().unwrap();
            let stdout = child.stdout.take().unwrap();

            let (incoming_message_tx, mut incoming_message_rx) = mpsc::unbounded();
            let (outgoing_tx, outgoing_rx) = mpsc::unbounded();

            let io_task =
                ClaudeAgentConnection::handle_io(outgoing_rx, incoming_message_tx, stdin, stdout);
            cx.background_spawn(async move {
                io_task.await.log_err();
                drop(mcp_config_path);
                drop(child);
            })
            .detach();

            cx.new(|cx| {
                let end_turn_tx = Rc::new(RefCell::new(None));
                let delegate = AcpClientDelegate::new(cx.entity().downgrade(), cx.to_async());
                delegate_rc.borrow_mut().replace(delegate.clone());

                let handler_task = cx.foreground_executor().spawn({
                    let end_turn_tx = end_turn_tx.clone();
                    let tool_id_map = tool_id_map.clone();
                    async move {
                        while let Some(message) = incoming_message_rx.next().await {
                            ClaudeAgentConnection::handle_message(
                                delegate.clone(),
                                message,
                                end_turn_tx.clone(),
                                tool_id_map.clone(),
                            )
                            .await
                        }
                    }
                });

                let mut connection = ClaudeAgentConnection {
                    outgoing_tx,
                    end_turn_tx,
                    _handler_task: handler_task,
                    _permissions_mcp_server: None,
                };

                connection._permissions_mcp_server = Some(permission_mcp_server);
                acp_thread::AcpThread::new(connection, title, None, project.clone(), cx)
            })
        })
    }
}

struct ClaudeAgentConnection {
    outgoing_tx: UnboundedSender<SdkMessage>,
    end_turn_tx: Rc<RefCell<Option<oneshot::Sender<Result<()>>>>>,
    _permissions_mcp_server: Option<PermissionMcpServer>,
    _handler_task: Task<()>,
}

impl ClaudeAgentConnection {
    async fn handle_message(
        delegate: AcpClientDelegate,
        message: SdkMessage,
        end_turn_tx: Rc<RefCell<Option<oneshot::Sender<Result<()>>>>>,
        tool_id_map: Rc<RefCell<HashMap<String, acp::ToolCallId>>>,
    ) {
        match message {
            SdkMessage::Assistant { message, .. } | SdkMessage::User { message, .. } => {
                for chunk in message.content {
                    match chunk {
                        MessageContent::Text { text } => {
                            delegate
                                .stream_assistant_message_chunk(StreamAssistantMessageChunkParams {
                                    chunk: acp::AssistantMessageChunk::Text { text },
                                })
                                .await
                                .log_err();
                        }
                        MessageContent::ToolUse { id, name, input } => {
                            let formatted = serde_json::to_string_pretty(&input).unwrap();
                            let markdown = format!("```json\n{}\n```", formatted);
                            if let Some(resp) = delegate
                                .push_tool_call(PushToolCallParams {
                                    label: name,
                                    icon: acp::Icon::Hammer,
                                    content: Some(ToolCallContent::Markdown { markdown }),
                                    locations: Vec::default(),
                                })
                                .await
                                .log_err()
                            {
                                tool_id_map.borrow_mut().insert(id, resp.id);
                            }
                        }
                        MessageContent::ToolResult {
                            content,
                            tool_use_id,
                        } => {
                            if let Some(id) = tool_id_map.borrow_mut().remove(&tool_use_id) {
                                delegate
                                    .update_tool_call(UpdateToolCallParams {
                                        tool_call_id: id,
                                        status: acp::ToolCallStatus::Finished,
                                        content: Some(ToolCallContent::Markdown {
                                            markdown: content,
                                        }),
                                    })
                                    .await
                                    .log_err();
                            }
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
    ) -> Result<()> {
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
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    role: Role,
    content: Vec<MessageContent>,
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
#[serde(tag = "type", rename_all = "snake_case")]
enum MessageContent {
    Text {
        text: String,
    },
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    ToolResult {
        content: String,
        tool_use_id: String,
    },
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
        api_key_source: String,
        cwd: String,
        session_id: String,
        tools: Vec<String>,
        mcp_servers: Vec<McpServer>,
        model: String,
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
enum PermissionMode {
    Default,
    AcceptEdits,
    BypassPermissions,
    Plan,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct McpConfig {
    mcp_servers: HashMap<String, McpServerConfig>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct McpServerConfig {
    command: String,
    args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<HashMap<String, String>>,
}
