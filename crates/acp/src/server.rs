use crate::{AcpThread, ThreadEntryId, ToolCallId, ToolCallRequest};
use agentic_coding_protocol as acp;
use anyhow::{Context as _, Result};
use async_trait::async_trait;
use gpui::{App, AppContext, AsyncApp, Entity, Task, WeakEntity};
use parking_lot::Mutex;
use project::Project;
use smol::process::Child;
use std::{process::ExitStatus, sync::Arc};
use util::ResultExt;

pub struct AcpServer {
    thread: WeakEntity<AcpThread>,
    project: Entity<Project>,
    connection: Arc<acp::AgentConnection>,
    exit_status: Arc<Mutex<Option<ExitStatus>>>,
    _handler_task: Task<()>,
    _io_task: Task<()>,
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

#[async_trait(?Send)]
impl acp::Client for AcpClientDelegate {
    async fn stream_assistant_message_chunk(
        &self,
        params: acp::StreamAssistantMessageChunkParams,
    ) -> Result<acp::StreamAssistantMessageChunkResponse> {
        let cx = &mut self.cx.clone();

        cx.update(|cx| {
            self.thread.update(cx, |thread, cx| {
                thread.push_assistant_chunk(params.chunk, cx)
            });
        })?;

        Ok(acp::StreamAssistantMessageChunkResponse)
    }

    async fn request_tool_call_confirmation(
        &self,
        request: acp::RequestToolCallConfirmationParams,
    ) -> Result<acp::RequestToolCallConfirmationResponse> {
        let cx = &mut self.cx.clone();
        let ToolCallRequest { id, outcome } = cx
            .update(|cx| {
                self.thread.update(cx, |thread, cx| {
                    thread.request_tool_call(
                        request.label,
                        request.icon,
                        request.content,
                        request.confirmation,
                        cx,
                    )
                })
            })?
            .context("Failed to update thread")?;

        Ok(acp::RequestToolCallConfirmationResponse {
            id: id.into(),
            outcome: outcome.await?,
        })
    }

    async fn push_tool_call(
        &self,
        request: acp::PushToolCallParams,
    ) -> Result<acp::PushToolCallResponse> {
        let cx = &mut self.cx.clone();
        let entry_id = cx
            .update(|cx| {
                self.thread.update(cx, |thread, cx| {
                    thread.push_tool_call(request.label, request.icon, request.content, cx)
                })
            })?
            .context("Failed to update thread")?;

        Ok(acp::PushToolCallResponse {
            id: entry_id.into(),
        })
    }

    async fn update_tool_call(
        &self,
        request: acp::UpdateToolCallParams,
    ) -> Result<acp::UpdateToolCallResponse> {
        let cx = &mut self.cx.clone();

        cx.update(|cx| {
            self.thread.update(cx, |thread, cx| {
                thread.update_tool_call(
                    request.tool_call_id.into(),
                    request.status,
                    request.content,
                    cx,
                )
            })
        })?
        .context("Failed to update thread")??;

        Ok(acp::UpdateToolCallResponse)
    }
}

impl AcpServer {
    pub fn stdio(mut process: Child, project: Entity<Project>, cx: &mut App) -> Arc<Self> {
        let stdin = process.stdin.take().expect("process didn't have stdin");
        let stdout = process.stdout.take().expect("process didn't have stdout");

        let mut connection = None;
        cx.new(|cx| {
            let (conn, handler_fut, io_fut) = acp::AgentConnection::connect_to_agent(
                AcpClientDelegate::new(cx.entity().downgrade(), cx.to_async()),
                stdin,
                stdout,
            );

            let exit_status: Arc<Mutex<Option<ExitStatus>>> = Default::default();
            let io_task = cx.background_spawn({
                let exit_status = exit_status.clone();
                async move {
                    io_fut.await.log_err();
                    let result = process.status().await.log_err();
                    *exit_status.lock() = result;
                }
            });

            connection.replace(Arc::new(Self {
                project: project.clone(),
                connection: Arc::new(conn),
                thread: cx.entity().downgrade(),
                exit_status,
                _handler_task: cx.foreground_executor().spawn(handler_fut),
                _io_task: io_task,
            }));

            AcpThread::new(
                connection.clone().unwrap(),
                Vec::default(),
                project.clone(),
                cx,
            )
        });

        connection.unwrap()
    }

    #[cfg(test)]
    pub fn fake(
        stdin: async_pipe::PipeWriter,
        stdout: async_pipe::PipeReader,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Arc<Self> {
        let mut connection = None;
        cx.new(|cx| {
            let (conn, handler_fut, io_fut) = acp::AgentConnection::connect_to_agent(
                AcpClientDelegate::new(cx.entity().downgrade(), cx.to_async()),
                stdin,
                stdout,
            );

            let exit_status: Arc<Mutex<Option<ExitStatus>>> = Default::default();
            let io_task = cx.background_spawn({
                async move {
                    io_fut.await.log_err();
                    // todo!() exit status?
                }
            });

            connection.replace(Arc::new(Self {
                project: project.clone(),
                connection: Arc::new(conn),
                thread: cx.entity().downgrade(),
                exit_status,
                _handler_task: cx.foreground_executor().spawn(handler_fut),
                _io_task: io_task,
            }));

            AcpThread::new(
                connection.clone().unwrap(),
                Vec::default(),
                project.clone(),
                cx,
            )
        });

        connection.unwrap()
    }

    pub async fn initialize(&self) -> Result<acp::InitializeResponse> {
        self.connection
            .request(acp::InitializeParams)
            .await
            .map_err(to_anyhow)
    }

    pub async fn authenticate(&self) -> Result<()> {
        self.connection
            .request(acp::AuthenticateParams)
            .await
            .map_err(to_anyhow)?;

        Ok(())
    }

    pub async fn send_message(&self, message: acp::UserMessage, _cx: &mut AsyncApp) -> Result<()> {
        self.connection
            .request(acp::SendUserMessageParams { message })
            .await
            .map_err(to_anyhow)?;
        Ok(())
    }

    pub async fn cancel_send_message(&self, _cx: &mut AsyncApp) -> Result<()> {
        self.connection
            .request(acp::CancelSendMessageParams)
            .await
            .map_err(to_anyhow)?;
        Ok(())
    }

    pub fn exit_status(&self) -> Option<ExitStatus> {
        *self.exit_status.lock()
    }
}

#[track_caller]
fn to_anyhow(e: acp::Error) -> anyhow::Error {
    log::error!(
        "failed to send message: {code}: {message}",
        code = e.code,
        message = e.message
    );
    anyhow::anyhow!(e.message)
}

impl From<acp::ToolCallId> for ToolCallId {
    fn from(tool_call_id: acp::ToolCallId) -> Self {
        Self(ThreadEntryId(tool_call_id.0))
    }
}

impl From<ToolCallId> for acp::ToolCallId {
    fn from(tool_call_id: ToolCallId) -> Self {
        acp::ToolCallId(tool_call_id.as_u64())
    }
}
