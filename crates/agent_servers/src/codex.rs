use collections::HashMap;
use context_server::types::requests::CallTool;
use context_server::types::{CallToolParams, ToolResponseContent};
use context_server::{ContextServer, ContextServerCommand, ContextServerId};
use futures::channel::{mpsc, oneshot};
use itertools::Itertools;
use project::Project;
use serde::de::DeserializeOwned;
use settings::SettingsStore;
use smol::stream::StreamExt;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use agentic_coding_protocol::{
    self as acp, AnyAgentRequest, AnyAgentResult, Client as _, ProtocolVersion,
};
use anyhow::{Context, Result, anyhow};
use futures::future::LocalBoxFuture;
use futures::{AsyncWriteExt, FutureExt, SinkExt as _};
use gpui::{App, AppContext, Entity, Task};
use serde::{Deserialize, Serialize};
use util::ResultExt;

use crate::mcp_server::{McpConfig, ZedMcpServer};
use crate::{AgentServer, AgentServerCommand, AllAgentServersSettings};
use acp_thread::{AcpClientDelegate, AcpThread, AgentConnection};

#[derive(Clone)]
pub struct Codex;

pub struct CodexApproval;
impl context_server::types::Request for CodexApproval {
    type Params = CodexApprovalRequest;
    type Response = CodexApprovalResponse;
    const METHOD: &'static str = "elicitation/create";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodexApprovalRequest {
    // These fields are required so that `params`
    // conforms to ElicitRequestParams.
    pub message: String,
    // #[serde(rename = "requestedSchema")]
    // pub requested_schema: ElicitRequestParamsRequestedSchema,

    // // These are additional fields the client can use to
    // // correlate the request with the codex tool call.
    // pub codex_elicitation: String,
    // pub codex_mcp_tool_call_id: String,
    // pub codex_event_id: String,
    // pub codex_command: Vec<String>,
    // pub codex_cwd: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodexApprovalResponse {
    pub decision: ReviewDecision,
}

/// User's decision in response to an ExecApprovalRequest.
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDecision {
    /// User has approved this command and the agent should execute it.
    Approved,

    /// User has approved this command and wants to automatically approve any
    /// future identical instances (`command` and `cwd` match exactly) for the
    /// remainder of the session.
    ApprovedForSession,

    /// User has denied this command and the agent should not execute it, but
    /// it should continue the session and try something else.
    #[default]
    Denied,

    /// User has denied this command and the agent should not do anything until
    /// the user's next command.
    Abort,
}

impl AgentServer for Codex {
    fn name(&self) -> &'static str {
        "Codex"
    }

    fn empty_state_headline(&self) -> &'static str {
        self.name()
    }

    fn empty_state_message(&self) -> &'static str {
        ""
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiOpenAi
    }

    fn supports_always_allow(&self) -> bool {
        false
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
            let (mut delegate_tx, delegate_rx) = watch::channel(None);
            let tool_id_map = Rc::new(RefCell::new(HashMap::default()));

            let zed_mcp_server = ZedMcpServer::new(delegate_rx, tool_id_map.clone(), cx).await?;
            let mcp_server_config = zed_mcp_server.server_config()?;
            // https://github.com/openai/codex/blob/main/codex-rs/config.md
            let cli_server_config = format!(
                "mcp_servers.{}={{command = \"{}\", args = [{}]}}",
                crate::mcp_server::SERVER_NAME,
                mcp_server_config.command.display(),
                mcp_server_config
                    .args
                    .iter()
                    .map(|arg| format!("\"{}\"", arg))
                    .join(", ")
            );

            let settings = cx.read_global(|settings: &SettingsStore, _| {
                settings.get::<AllAgentServersSettings>(None).codex.clone()
            })?;

            let Some(mut command) =
                AgentServerCommand::resolve("codex", &["mcp"], settings, &project, cx).await
            else {
                anyhow::bail!("Failed to find codex binary");
            };

            command
                .args
                .extend(["--config".to_string(), cli_server_config]);

            let codex_mcp_client: Arc<ContextServer> = ContextServer::stdio(
                ContextServerId("codex-mcp-server".into()),
                ContextServerCommand {
                    path: command.path,
                    args: command.args,
                    env: command.env,
                },
            )
            .into();

            ContextServer::start(codex_mcp_client.clone(), cx).await?;
            // todo! stop

            let (notification_tx, mut notification_rx) = mpsc::unbounded();

            let client = codex_mcp_client
                .client()
                .context("Failed to subscribe to server")?;
            client.on_request::<CodexApproval, _>({
                move |elicitation: CodexApprovalRequest, cx| {
                    cx.spawn(async move |cx| anyhow::bail!("oops"))
                }
            });
            client.on_notification("codex/event", {
                move |event, cx| {
                    let mut notification_tx = notification_tx.clone();
                    cx.background_spawn(async move {
                        log::trace!("Notification: {:?}", event);
                        if let Some(event) = serde_json::from_value::<CodexEvent>(event).log_err() {
                            notification_tx.send(event.msg).await.log_err();
                        }
                    })
                    .detach();
                }
            });

            cx.new(|cx| {
                let delegate = AcpClientDelegate::new(cx.entity().downgrade(), cx.to_async());
                delegate_tx.send(Some(delegate.clone())).log_err();

                let handler_task = cx.spawn({
                    let delegate = delegate.clone();
                    let tool_id_map = tool_id_map.clone();
                    async move |_, _cx| {
                        while let Some(notification) = notification_rx.next().await {
                            CodexAgentConnection::handle_acp_notification(
                                &delegate,
                                notification,
                                &tool_id_map,
                            )
                            .await
                            .log_err();
                        }
                    }
                });

                let connection = CodexAgentConnection {
                    root_dir,
                    codex_mcp: codex_mcp_client,
                    cancel_request_tx: Default::default(),
                    tool_id_map: tool_id_map.clone(),
                    _handler_task: handler_task,
                    _zed_mcp: zed_mcp_server,
                };

                acp_thread::AcpThread::new(connection, title, None, project.clone(), cx)
            })
        })
    }
}

impl AgentConnection for CodexAgentConnection {
    /// Send a request to the agent and wait for a response.
    fn request_any(
        &self,
        params: AnyAgentRequest,
    ) -> LocalBoxFuture<'static, Result<acp::AnyAgentResult>> {
        let client = self.codex_mcp.client();
        let root_dir = self.root_dir.clone();
        let cancel_request_tx = self.cancel_request_tx.clone();
        async move {
            let client = client.context("Codex MCP server is not initialized")?;

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
                    let (new_cancel_tx, cancel_rx) = oneshot::channel();
                    cancel_request_tx.borrow_mut().replace(new_cancel_tx);

                    client
                        .cancellable_request::<CallTool>(
                            CallToolParams {
                                name: "codex".into(),
                                arguments: Some(serde_json::to_value(CodexToolCallParam {
                                    prompt: message
                                        .chunks
                                        .into_iter()
                                        .filter_map(|chunk| match chunk {
                                            acp::UserMessageChunk::Text { text } => Some(text),
                                            acp::UserMessageChunk::Path { .. } => {
                                                // todo!
                                                None
                                            }
                                        })
                                        .collect(),
                                    cwd: root_dir,
                                })?),
                                meta: None,
                            },
                            cancel_rx,
                        )
                        .await?;

                    Ok(AnyAgentResult::SendUserMessageResponse(
                        acp::SendUserMessageResponse,
                    ))
                }
                AnyAgentRequest::CancelSendMessageParams(_) => {
                    if let Ok(mut borrow) = cancel_request_tx.try_borrow_mut() {
                        if let Some(cancel_tx) = borrow.take() {
                            cancel_tx.send(()).ok();
                        }
                    }

                    Ok(AnyAgentResult::CancelSendMessageResponse(
                        acp::CancelSendMessageResponse,
                    ))
                }
            }
        }
        .boxed_local()
    }
}

struct CodexAgentConnection {
    codex_mcp: Arc<context_server::ContextServer>,
    root_dir: PathBuf,
    cancel_request_tx: Rc<RefCell<Option<oneshot::Sender<()>>>>,
    tool_id_map: Rc<RefCell<HashMap<String, acp::ToolCallId>>>,
    _handler_task: Task<()>,
    _zed_mcp: ZedMcpServer,
}

impl CodexAgentConnection {
    async fn handle_acp_notification(
        delegate: &AcpClientDelegate,
        event: AcpNotification,
        tool_id_map: &Rc<RefCell<HashMap<String, acp::ToolCallId>>>,
    ) -> Result<()> {
        match event {
            AcpNotification::AgentMessage(message) => {
                delegate
                    .stream_assistant_message_chunk(acp::StreamAssistantMessageChunkParams {
                        chunk: acp::AssistantMessageChunk::Text {
                            text: message.message,
                        },
                    })
                    .await?;
            }
            AcpNotification::AgentReasoning(message) => {
                delegate
                    .stream_assistant_message_chunk(acp::StreamAssistantMessageChunkParams {
                        chunk: acp::AssistantMessageChunk::Thought {
                            thought: message.text,
                        },
                    })
                    .await?
            }
            AcpNotification::McpToolCallBegin(event) => {
                let result = delegate
                    .push_tool_call(acp::PushToolCallParams {
                        label: format!("`{}: {}`", event.server, event.tool),
                        icon: acp::Icon::Hammer,
                        content: event.arguments.and_then(|args| {
                            Some(acp::ToolCallContent::Markdown {
                                markdown: md_codeblock(
                                    "json",
                                    &serde_json::to_string_pretty(&args).ok()?,
                                ),
                            })
                        }),
                        locations: vec![],
                    })
                    .await?;

                tool_id_map.borrow_mut().insert(event.call_id, result.id);
            }
            AcpNotification::McpToolCallEnd(event) => {
                let acp_call_id = tool_id_map
                    .borrow_mut()
                    .remove(&event.call_id)
                    .context("Missing tool call")?;

                let (status, content) = match event.result {
                    Ok(value) => {
                        if let Ok(response) =
                            serde_json::from_value::<context_server::types::CallToolResponse>(value)
                        {
                            (
                                acp::ToolCallStatus::Finished,
                                mcp_tool_content_to_acp(response.content),
                            )
                        } else {
                            (
                                acp::ToolCallStatus::Error,
                                Some(acp::ToolCallContent::Markdown {
                                    markdown: "Failed to parse tool response".to_string(),
                                }),
                            )
                        }
                    }
                    Err(error) => (
                        acp::ToolCallStatus::Error,
                        Some(acp::ToolCallContent::Markdown { markdown: error }),
                    ),
                };

                delegate
                    .update_tool_call(acp::UpdateToolCallParams {
                        tool_call_id: acp_call_id,
                        status,
                        content,
                    })
                    .await?;
            }
            AcpNotification::ExecCommandBegin(event) => {
                let inner_command = strip_bash_lc_and_escape(&event.command);

                let result = delegate
                    .push_tool_call(acp::PushToolCallParams {
                        label: format!("`{}`", inner_command),
                        icon: acp::Icon::Terminal,
                        content: None,
                        locations: vec![],
                    })
                    .await?;

                tool_id_map.borrow_mut().insert(event.call_id, result.id);
            }
            AcpNotification::ExecCommandEnd(event) => {
                let acp_call_id = tool_id_map
                    .borrow_mut()
                    .remove(&event.call_id)
                    .context("Missing tool call")?;

                let mut content = String::new();
                if !event.stdout.is_empty() {
                    use std::fmt::Write;
                    writeln!(
                        &mut content,
                        "### Output\n\n{}",
                        md_codeblock("", &event.stdout)
                    )
                    .unwrap();
                }
                if !event.stdout.is_empty() && !event.stderr.is_empty() {
                    use std::fmt::Write;
                    writeln!(&mut content).unwrap();
                }
                if !event.stderr.is_empty() {
                    use std::fmt::Write;
                    writeln!(
                        &mut content,
                        "### Error\n\n{}",
                        md_codeblock("", &event.stderr)
                    )
                    .unwrap();
                }
                let success = event.exit_code == 0;
                if !success {
                    use std::fmt::Write;
                    writeln!(&mut content, "\nExit code: `{}`", event.exit_code).unwrap();
                }

                delegate
                    .update_tool_call(acp::UpdateToolCallParams {
                        tool_call_id: acp_call_id,
                        status: if success {
                            acp::ToolCallStatus::Finished
                        } else {
                            acp::ToolCallStatus::Error
                        },
                        content: Some(acp::ToolCallContent::Markdown { markdown: content }),
                    })
                    .await?;
            }
            AcpNotification::ExecApprovalRequest(event) => {
                let inner_command = strip_bash_lc_and_escape(&event.command);
                let root_command = inner_command
                    .split(" ")
                    .next()
                    .map(|s| s.to_string())
                    .unwrap_or_default();

                let response = delegate
                    .request_tool_call_confirmation(acp::RequestToolCallConfirmationParams {
                        tool_call: acp::PushToolCallParams {
                            label: format!("`{}`", inner_command),
                            icon: acp::Icon::Terminal,
                            content: None,
                            locations: vec![],
                        },
                        confirmation: acp::ToolCallConfirmation::Execute {
                            command: inner_command,
                            root_command,
                            description: event.reason,
                        },
                    })
                    .await?;

                tool_id_map.borrow_mut().insert(event.call_id, response.id);

                // todo! approval
            }
            AcpNotification::Other => {}
        }

        Ok(())
    }
}

/// todo! use types from h2a crate when we have one

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct CodexToolCallParam {
    pub prompt: String,
    pub cwd: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CodexEvent {
    pub msg: AcpNotification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AcpNotification {
    AgentMessage(AgentMessageEvent),
    AgentReasoning(AgentReasoningEvent),
    McpToolCallBegin(McpToolCallBeginEvent),
    McpToolCallEnd(McpToolCallEndEvent),
    ExecCommandBegin(ExecCommandBeginEvent),
    ExecCommandEnd(ExecCommandEndEvent),
    ExecApprovalRequest(ExecApprovalRequestEvent),
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessageEvent {
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentReasoningEvent {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolCallBeginEvent {
    pub call_id: String,
    pub server: String,
    pub tool: String,
    pub arguments: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolCallEndEvent {
    pub call_id: String,
    pub result: Result<serde_json::Value, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecCommandBeginEvent {
    pub call_id: String,
    pub command: Vec<String>,
    pub cwd: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecCommandEndEvent {
    pub call_id: String,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecApprovalRequestEvent {
    pub call_id: String,
    pub command: Vec<String>,
    pub cwd: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

// Helper functions
fn md_codeblock(lang: &str, content: &str) -> String {
    if content.ends_with('\n') {
        format!("```{}\n{}```", lang, content)
    } else {
        format!("```{}\n{}\n```", lang, content)
    }
}

fn strip_bash_lc_and_escape(command: &[String]) -> String {
    match command {
        // exactly three items
        [first, second, third]
            // first two must be "bash", "-lc"
            if first == "bash" && second == "-lc" =>
        {
            third.clone()
        }
        _ => escape_command(command),
    }
}

fn escape_command(command: &[String]) -> String {
    shlex::try_join(command.iter().map(|s| s.as_str())).unwrap_or_else(|_| command.join(" "))
}

fn mcp_tool_content_to_acp(chunks: Vec<ToolResponseContent>) -> Option<acp::ToolCallContent> {
    let mut content = String::new();

    for chunk in chunks {
        match chunk {
            ToolResponseContent::Text { text } => content.push_str(&text),
            ToolResponseContent::Image { .. } => {
                // todo!
            }
            ToolResponseContent::Audio { .. } => {
                // todo!
            }
            ToolResponseContent::Resource { .. } => {
                // todo!
            }
        }
    }

    if !content.is_empty() {
        Some(acp::ToolCallContent::Markdown { markdown: content })
    } else {
        None
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    crate::common_e2e_tests!(Codex);

    pub fn local_command() -> AgentServerCommand {
        let cli_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../codex/code-rs/target/debug/codex");

        AgentServerCommand {
            path: cli_path,
            args: vec!["mcp".into()],
            env: None,
        }
    }
}
