use collections::HashMap;
use context_server::types::requests::CallTool;
use context_server::types::{CallToolParams, ToolResponseContent};
use context_server::{ContextServer, ContextServerCommand, ContextServerId};
use futures::channel::{mpsc, oneshot};
use project::Project;
use settings::SettingsStore;
use smol::stream::StreamExt;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use uuid::Uuid;

use agentic_coding_protocol::{
    self as acp, AnyAgentRequest, AnyAgentResult, Client as _, ProtocolVersion,
};
use anyhow::{Context, Result, anyhow};
use futures::future::LocalBoxFuture;
use futures::{FutureExt, SinkExt as _};
use gpui::{App, AppContext, Entity, Task};
use serde::{Deserialize, Serialize};
use util::ResultExt;

use crate::mcp_server::{self, McpServerConfig, ZedMcpServer};
use crate::tools::{EditToolParams, ReadToolParams};
use crate::{AgentServer, AgentServerCommand, AllAgentServersSettings};
use acp_thread::{AcpClientDelegate, AcpThread, AgentConnection};

#[derive(Clone)]
pub struct Codex;

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

            let zed_mcp_server = ZedMcpServer::new(
                delegate_rx,
                tool_id_map.clone(),
                mcp_server::EnabledTools {
                    permission: false,
                    ..Default::default()
                },
                cx,
            )
            .await?;

            let settings = cx.read_global(|settings: &SettingsStore, _| {
                settings.get::<AllAgentServersSettings>(None).codex.clone()
            })?;

            let Some(command) =
                AgentServerCommand::resolve("codex", &["mcp"], settings, &project, cx).await
            else {
                anyhow::bail!("Failed to find codex binary");
            };

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
            let (request_tx, mut request_rx) = mpsc::unbounded();

            let client = codex_mcp_client
                .client()
                .context("Failed to subscribe to server")?;

            client.on_notification("codex/event", {
                move |event, cx| {
                    let mut notification_tx = notification_tx.clone();
                    cx.background_spawn(async move {
                        log::trace!("Notification: {:?}", serde_json::to_string_pretty(&event));
                        if let Some(event) = serde_json::from_value::<CodexEvent>(event).log_err() {
                            notification_tx.send(event.msg).await.log_err();
                        }
                    })
                    .detach();
                }
            });

            client.on_request::<CodexApproval, _>({
                move |elicitation, cx| {
                    let (tx, rx) = oneshot::channel::<Result<CodexApprovalResponse>>();
                    let mut request_tx = request_tx.clone();
                    cx.background_spawn(async move {
                        log::trace!("Elicitation: {:?}", elicitation);
                        request_tx.send((elicitation, tx)).await?;
                        rx.await?
                    })
                }
            });

            let requested_call_id = Rc::new(RefCell::new(None));
            let session_id = Rc::new(RefCell::new(None));

            cx.new(|cx| {
                let delegate = AcpClientDelegate::new(cx.entity().downgrade(), cx.to_async());
                delegate_tx.send(Some(delegate.clone())).log_err();

                let handler_task = cx.spawn({
                    let delegate = delegate.clone();
                    let tool_id_map = tool_id_map.clone();
                    let requested_call_id = requested_call_id.clone();
                    let session_id = session_id.clone();
                    async move |_, _cx| {
                        while let Some(notification) = notification_rx.next().await {
                            CodexAgentConnection::handle_acp_notification(
                                &delegate,
                                notification,
                                &session_id,
                                &tool_id_map,
                                &requested_call_id,
                            )
                            .await
                            .log_err();
                        }
                    }
                });

                let request_task = cx.spawn({
                    let delegate = delegate.clone();
                    async move |_, _cx| {
                        while let Some((elicitation, respond)) = request_rx.next().await {
                            if let Some((id, decision)) =
                                CodexAgentConnection::handle_elicitation(&delegate, elicitation)
                                    .await
                                    .log_err()
                            {
                                requested_call_id.replace(Some(id));

                                respond
                                    .send(Ok(CodexApprovalResponse { decision }))
                                    .log_err();
                            }
                        }
                    }
                });

                let connection = CodexAgentConnection {
                    root_dir,
                    codex_mcp: codex_mcp_client,
                    cancel_request_tx: Default::default(),
                    session_id,
                    zed_mcp_server,
                    _handler_task: handler_task,
                    _request_task: request_task,
                };

                acp_thread::AcpThread::new(connection, title, None, project.clone(), cx)
            })
        })
    }
}

struct CodexAgentConnection {
    codex_mcp: Arc<context_server::ContextServer>,
    root_dir: PathBuf,
    cancel_request_tx: Rc<RefCell<Option<oneshot::Sender<()>>>>,
    session_id: Rc<RefCell<Option<Uuid>>>,
    zed_mcp_server: ZedMcpServer,
    _handler_task: Task<()>,
    _request_task: Task<()>,
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
        let mcp_config = self.zed_mcp_server.server_config();
        let session_id = self.session_id.clone();
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

                    let prompt = message
                        .chunks
                        .into_iter()
                        .filter_map(|chunk| match chunk {
                            acp::UserMessageChunk::Text { text } => Some(text),
                            acp::UserMessageChunk::Path { .. } => {
                                // todo!
                                None
                            }
                        })
                        .collect();

                    let params = if let Some(session_id) = *session_id.borrow() {
                        CallToolParams {
                            name: "codex-reply".into(),
                            arguments: Some(serde_json::to_value(CodexToolCallReplyParam {
                                prompt,
                                session_id,
                            })?),
                            meta: None,
                        }
                    } else {
                        CallToolParams {
                            name: "codex".into(),
                            arguments: Some(serde_json::to_value(CodexToolCallParam {
                                prompt,
                                cwd: root_dir,
                                config: Some(CodexConfig {
                                    mcp_servers: Some(
                                        mcp_config
                                            .into_iter()
                                            .map(|config| {
                                                (mcp_server::SERVER_NAME.to_string(), config)
                                            })
                                            .collect(),
                                    ),
                                }),
                            })?),
                            meta: None,
                        }
                    };

                    client
                        .request_with::<CallTool>(params, Some(cancel_rx), None)
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CodexConfig {
    mcp_servers: Option<HashMap<String, McpServerConfig>>,
}

impl CodexAgentConnection {
    async fn handle_elicitation(
        delegate: &AcpClientDelegate,
        elicitation: CodexElicitation,
    ) -> Result<(acp::ToolCallId, ReviewDecision)> {
        let confirmation = match elicitation {
            CodexElicitation::ExecApproval(exec) => {
                let inner_command = strip_bash_lc_and_escape(&exec.codex_command);

                acp::RequestToolCallConfirmationParams {
                    tool_call: acp::PushToolCallParams {
                        label: format!("`{inner_command}`"),
                        icon: acp::Icon::Terminal,
                        content: None,
                        locations: vec![],
                    },
                    confirmation: acp::ToolCallConfirmation::Execute {
                        root_command: inner_command
                            .split(" ")
                            .next()
                            .unwrap_or_default()
                            .to_string(),
                        command: inner_command,
                        description: Some(exec.message),
                    },
                }
            }
            CodexElicitation::PatchApproval(patch) => {
                acp::RequestToolCallConfirmationParams {
                    tool_call: acp::PushToolCallParams {
                        label: "Edit".to_string(),
                        icon: acp::Icon::Pencil,
                        content: None, // todo!()
                        locations: patch
                            .codex_changes
                            .keys()
                            .map(|path| acp::ToolCallLocation {
                                path: path.clone(),
                                line: None,
                            })
                            .collect(),
                    },
                    confirmation: acp::ToolCallConfirmation::Edit {
                        description: Some(patch.message),
                    },
                }
            }
        };

        let response = delegate
            .request_tool_call_confirmation(confirmation)
            .await?;

        let decision = match response.outcome {
            acp::ToolCallConfirmationOutcome::Allow => ReviewDecision::Approved,
            acp::ToolCallConfirmationOutcome::AlwaysAllow
            | acp::ToolCallConfirmationOutcome::AlwaysAllowMcpServer
            | acp::ToolCallConfirmationOutcome::AlwaysAllowTool => {
                ReviewDecision::ApprovedForSession
            }
            acp::ToolCallConfirmationOutcome::Reject => ReviewDecision::Denied,
            acp::ToolCallConfirmationOutcome::Cancel => ReviewDecision::Abort,
        };

        Ok((response.id, decision))
    }

    async fn handle_acp_notification(
        delegate: &AcpClientDelegate,
        event: AcpNotification,
        session_id: &Rc<RefCell<Option<Uuid>>>,
        tool_id_map: &Rc<RefCell<HashMap<String, acp::ToolCallId>>>,
        requested_call_id: &Rc<RefCell<Option<acp::ToolCallId>>>,
    ) -> Result<()> {
        match event {
            AcpNotification::SessionConfigured(sesh) => {
                session_id.replace(Some(sesh.session_id));
            }
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
            AcpNotification::McpToolCallBegin(mut event) => {
                if let Some(requested_tool_id) = requested_call_id.take() {
                    tool_id_map
                        .borrow_mut()
                        .insert(event.call_id, requested_tool_id);
                } else {
                    let mut tool_call = acp::PushToolCallParams {
                        label: format!("`{}: {}`", event.server, event.tool),
                        icon: acp::Icon::Hammer,
                        content: event.arguments.as_ref().and_then(|args| {
                            Some(acp::ToolCallContent::Markdown {
                                markdown: md_codeblock(
                                    "json",
                                    &serde_json::to_string_pretty(args).ok()?,
                                ),
                            })
                        }),
                        locations: vec![],
                    };

                    if event.server == mcp_server::SERVER_NAME
                        && event.tool == mcp_server::EDIT_TOOL
                        && let Some(params) = event.arguments.take().and_then(|args| {
                            serde_json::from_value::<EditToolParams>(args).log_err()
                        })
                    {
                        tool_call = acp::PushToolCallParams {
                            label: "Edit".into(),
                            icon: acp::Icon::Pencil,
                            content: Some(acp::ToolCallContent::Diff {
                                diff: acp::Diff {
                                    path: params.abs_path.clone(),
                                    old_text: Some(params.old_text),
                                    new_text: params.new_text,
                                },
                            }),
                            locations: vec![acp::ToolCallLocation {
                                path: params.abs_path,
                                line: None,
                            }],
                        };
                    } else if event.server == mcp_server::SERVER_NAME
                        && event.tool == mcp_server::READ_TOOL
                        && let Some(params) = event.arguments.take().and_then(|args| {
                            serde_json::from_value::<ReadToolParams>(args).log_err()
                        })
                    {
                        tool_call = acp::PushToolCallParams {
                            label: "Read".into(),
                            icon: acp::Icon::FileSearch,
                            content: None,
                            locations: vec![acp::ToolCallLocation {
                                path: params.abs_path,
                                line: params.offset,
                            }],
                        }
                    }

                    let result = delegate.push_tool_call(tool_call).await?;

                    tool_id_map.borrow_mut().insert(event.call_id, result.id);
                }
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
                if let Some(requested_tool_id) = requested_call_id.take() {
                    tool_id_map
                        .borrow_mut()
                        .insert(event.call_id, requested_tool_id);
                } else {
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
            AcpNotification::Other => {}
        }

        Ok(())
    }
}

/// todo! use types from h2a crate when we have one

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct CodexToolCallParam {
    pub prompt: String,
    pub cwd: PathBuf,
    pub config: Option<CodexConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CodexToolCallReplyParam {
    pub session_id: Uuid,
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CodexEvent {
    pub msg: AcpNotification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AcpNotification {
    SessionConfigured(SessionConfiguredEvent),
    AgentMessage(AgentMessageEvent),
    AgentReasoning(AgentReasoningEvent),
    McpToolCallBegin(McpToolCallBeginEvent),
    McpToolCallEnd(McpToolCallEndEvent),
    ExecCommandBegin(ExecCommandBeginEvent),
    ExecCommandEnd(ExecCommandEndEvent),
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

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SessionConfiguredEvent {
    pub session_id: Uuid,
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

pub struct CodexApproval;
impl context_server::types::Request for CodexApproval {
    type Params = CodexElicitation;
    type Response = CodexApprovalResponse;
    const METHOD: &'static str = "elicitation/create";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecApprovalRequest {
    // These fields are required so that `params`
    // conforms to ElicitRequestParams.
    pub message: String,
    // #[serde(rename = "requestedSchema")]
    // pub requested_schema: ElicitRequestParamsRequestedSchema,

    // // These are additional fields the client can use to
    // // correlate the request with the codex tool call.
    pub codex_mcp_tool_call_id: String,
    // pub codex_event_id: String,
    pub codex_command: Vec<String>,
    pub codex_cwd: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchApprovalRequest {
    pub message: String,
    // #[serde(rename = "requestedSchema")]
    // pub requested_schema: ElicitRequestParamsRequestedSchema,
    pub codex_mcp_tool_call_id: String,
    pub codex_event_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codex_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codex_grant_root: Option<PathBuf>,
    pub codex_changes: HashMap<PathBuf, FileChange>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "codex_elicitation", rename_all = "kebab-case")]
pub enum CodexElicitation {
    ExecApproval(ExecApprovalRequest),
    PatchApproval(PatchApprovalRequest),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FileChange {
    Add {
        content: String,
    },
    Delete,
    Update {
        unified_diff: String,
        move_path: Option<PathBuf>,
    },
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
