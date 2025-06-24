use crate::{Agent, AgentThread, AgentThreadEntry, AgentThreadSummary, ResponseEvent, ThreadId};
use agentic_coding_protocol as acp;
use anyhow::{Context as _, Result};
use async_trait::async_trait;
use futures::channel::mpsc::UnboundedReceiver;
use gpui::{AppContext, AsyncApp, Entity, Task};
use project::Project;
use smol::process::Child;
use util::ResultExt;

pub struct AcpAgent {
    connection: acp::Connection,
    _handler_task: Task<()>,
    _io_task: Task<()>,
}

struct AcpClientDelegate {
    project: Entity<Project>,
    cx: AsyncApp,
    // sent_buffer_versions: HashMap<Entity<Buffer>, HashMap<u64, BufferSnapshot>>,
}

#[async_trait]
impl acp::Client for AcpClientDelegate {
    async fn read_file(&self, request: acp::ReadFileParams) -> Result<acp::ReadFileResponse> {
        let cx = &mut self.cx.clone();
        let buffer = self
            .project
            .update(cx, |project, cx| {
                let path = project
                    .project_path_for_absolute_path(request.path, cx)
                    .context("Failed to get project path")?;
                project.open_buffer(path, cx)
            })?
            .await?;

        anyhow::Ok(buffer.update(cx, |buffer, cx| acp::ReadFileResponse {
            content: buffer.text(),
            // todo!
            version: 0,
        }))
    }
}

impl AcpAgent {
    pub fn stdio(process: Child, project: Entity<Project>, cx: AsyncApp) -> Self {
        let stdin = process.stdin.expect("process didn't have stdin");
        let stdout = process.stdout.expect("process didn't have stdout");

        let (connection, handler_fut, io_fut) =
            acp::Connection::client_to_agent(AcpClientDelegate { project, cx }, stdin, stdout);

        let io_task = cx.background_spawn(async move {
            io_fut.await.log_err();
        });

        Self {
            connection,
            _handler_task: cx.foreground_executor().spawn(handler_fut),
            _io_task: io_task,
        }
    }
}

impl Agent for AcpAgent {
    type Thread = AcpAgentThread;

    async fn threads(&self) -> Result<Vec<AgentThreadSummary>> {
        let threads = self.connection.request(acp::ListThreadsParams).await?;
        threads
            .threads
            .into_iter()
            .map(|thread| {
                Ok(AgentThreadSummary {
                    id: ThreadId(thread.id.0),
                    title: thread.title,
                    created_at: thread.created_at,
                })
            })
            .collect()
    }

    async fn create_thread(&self) -> Result<Self::Thread> {
        todo!()
    }

    async fn open_thread(&self, id: crate::ThreadId) -> Result<Self::Thread> {
        todo!()
    }
}

struct AcpAgentThread {}

impl AgentThread for AcpAgentThread {
    async fn entries(&self) -> Result<Vec<AgentThreadEntry>> {
        todo!()
    }

    async fn send(
        &self,
        message: crate::Message,
    ) -> Result<UnboundedReceiver<Result<ResponseEvent>>> {
        todo!()
    }
}
