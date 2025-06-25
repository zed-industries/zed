use std::path::Path;

use crate::{
    Agent, AgentThread, AgentThreadEntryContent, AgentThreadSummary, ResponseEvent, ThreadId,
};
use agentic_coding_protocol::{self as acp};
use anyhow::{Context as _, Result};
use async_trait::async_trait;
use futures::channel::mpsc::UnboundedReceiver;
use gpui::{AppContext, AsyncApp, Entity, Task};
use project::Project;
use smol::process::Child;
use util::ResultExt;

pub struct AcpAgent {
    connection: acp::AgentConnection,
    _handler_task: Task<()>,
    _io_task: Task<()>,
}

struct AcpClientDelegate {
    project: Entity<Project>,
    cx: AsyncApp,
    // sent_buffer_versions: HashMap<Entity<Buffer>, HashMap<u64, BufferSnapshot>>,
}

#[async_trait(?Send)]
impl acp::Client for AcpClientDelegate {
    async fn read_file(&self, request: acp::ReadFileParams) -> Result<acp::ReadFileResponse> {
        let cx = &mut self.cx.clone();
        let buffer = self
            .project
            .update(cx, |project, cx| {
                let path = project
                    .project_path_for_absolute_path(Path::new(&request.path), cx)
                    .context("Failed to get project path")?;
                anyhow::Ok(project.open_buffer(path, cx))
            })??
            .await?;

        buffer.update(cx, |buffer, _| acp::ReadFileResponse {
            content: buffer.text(),
            version: acp::FileVersion(0),
        })
    }

    async fn glob_search(&self, request: acp::GlobSearchParams) -> Result<acp::GlobSearchResponse> {
        todo!()
    }

    async fn end_turn(&self, request: acp::EndTurnParams) -> Result<acp::EndTurnResponse> {
        todo!()
    }
}

impl AcpAgent {
    pub fn stdio(mut process: Child, project: Entity<Project>, cx: AsyncApp) -> Self {
        let stdin = process.stdin.take().expect("process didn't have stdin");
        let stdout = process.stdout.take().expect("process didn't have stdout");

        let (connection, handler_fut, io_fut) = acp::AgentConnection::connect_to_agent(
            AcpClientDelegate {
                project,
                cx: cx.clone(),
            },
            stdin,
            stdout,
        );

        let io_task = cx.background_spawn(async move {
            io_fut.await.log_err();
            process.status().await.log_err();
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
        let response = self.connection.request(acp::GetThreadsParams).await?;
        response
            .threads
            .into_iter()
            .map(|thread| {
                Ok(AgentThreadSummary {
                    id: thread.id.into(),
                    title: thread.title,
                    created_at: thread.modified_at,
                })
            })
            .collect()
    }

    async fn create_thread(&self) -> Result<Self::Thread> {
        let response = self.connection.request(acp::CreateThreadParams).await?;
        Ok(AcpAgentThread {
            id: response.thread_id,
        })
    }

    async fn open_thread(&self, id: ThreadId) -> Result<Self::Thread> {
        todo!()
    }
}

pub struct AcpAgentThread {
    id: acp::ThreadId,
}

impl AgentThread for AcpAgentThread {
    async fn entries(&self) -> Result<Vec<AgentThreadEntryContent>> {
        todo!()
    }

    async fn send(
        &self,
        message: crate::Message,
    ) -> Result<UnboundedReceiver<Result<ResponseEvent>>> {
        todo!()
    }
}

impl From<acp::ThreadId> for ThreadId {
    fn from(thread_id: acp::ThreadId) -> Self {
        Self(thread_id.0)
    }
}

impl From<ThreadId> for acp::ThreadId {
    fn from(thread_id: ThreadId) -> Self {
        acp::ThreadId(thread_id.0)
    }
}
