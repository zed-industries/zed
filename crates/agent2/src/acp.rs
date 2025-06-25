use std::{
    path::Path,
    sync::{Arc, Weak},
};

use crate::{
    Agent, AgentThread, AgentThreadEntryContent, AgentThreadSummary, Message, MessageChunk,
    ResponseEvent, Role, ThreadId,
};
use agentic_coding_protocol::{self as acp};
use anyhow::{Context as _, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::channel::mpsc::UnboundedReceiver;
use gpui::{AppContext, AsyncApp, Entity, Task};
use parking_lot::Mutex;
use project::Project;
use smol::process::Child;
use util::ResultExt;

pub struct AcpAgent {
    connection: Arc<acp::AgentConnection>,
    threads: Mutex<HashMap<acp::ThreadId, Weak<AcpAgentThread>>>,
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
    async fn stream_message_chunk(
        &self,
        request: acp::StreamMessageChunkParams,
    ) -> Result<acp::StreamMessageChunkResponse> {
        Ok(acp::StreamMessageChunkResponse)
    }

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
            connection: Arc::new(connection),
            threads: Mutex::default(),
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

    async fn create_thread(&self) -> Result<Arc<Self::Thread>> {
        let response = self.connection.request(acp::CreateThreadParams).await?;
        let thread = Arc::new(AcpAgentThread {
            id: response.thread_id.clone(),
            connection: self.connection.clone(),
            state: Mutex::new(AcpAgentThreadState { turn: None }),
        });
        self.threads
            .lock()
            .insert(response.thread_id, Arc::downgrade(&thread));
        Ok(thread)
    }

    async fn open_thread(&self, id: ThreadId) -> Result<Arc<Self::Thread>> {
        todo!()
    }
}

pub struct AcpAgentThread {
    id: acp::ThreadId,
    connection: Arc<acp::AgentConnection>,
    state: Mutex<AcpAgentThreadState>,
}

struct AcpAgentThreadState {
    turn: Option<AcpAgentThreadTurn>,
}

struct AcpAgentThreadTurn {}

impl AgentThread for AcpAgentThread {
    async fn entries(&self) -> Result<Vec<AgentThreadEntryContent>> {
        let response = self
            .connection
            .request(acp::GetThreadEntriesParams {
                thread_id: self.id.clone(),
            })
            .await?;

        Ok(response
            .entries
            .into_iter()
            .map(|entry| match entry {
                acp::ThreadEntry::Message { message } => {
                    AgentThreadEntryContent::Message(Message {
                        role: match message.role {
                            acp::Role::User => Role::User,
                            acp::Role::Assistant => Role::Assistant,
                        },
                        chunks: message
                            .chunks
                            .into_iter()
                            .map(|chunk| match chunk {
                                acp::MessageChunk::Text { chunk } => MessageChunk::Text { chunk },
                            })
                            .collect(),
                    })
                }
                acp::ThreadEntry::ReadFile { path, content } => {
                    AgentThreadEntryContent::ReadFile { path, content }
                }
            })
            .collect())
    }

    async fn send(
        &self,
        message: crate::Message,
    ) -> Result<UnboundedReceiver<Result<ResponseEvent>>> {
        let response = self
            .connection
            .request(acp::SendMessageParams {
                thread_id: self.id.clone(),
                message: acp::Message {
                    role: match message.role {
                        Role::User => acp::Role::User,
                        Role::Assistant => acp::Role::Assistant,
                    },
                    chunks: message
                        .chunks
                        .into_iter()
                        .map(|chunk| match chunk {
                            MessageChunk::Text { chunk } => acp::MessageChunk::Text { chunk },
                            MessageChunk::File { content } => todo!(),
                            MessageChunk::Directory { path, contents } => todo!(),
                            MessageChunk::Symbol {
                                path,
                                range,
                                version,
                                name,
                                content,
                            } => todo!(),
                            MessageChunk::Thread { title, content } => todo!(),
                            MessageChunk::Fetch { url, content } => todo!(),
                        })
                        .collect(),
                },
            })
            .await?;
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
