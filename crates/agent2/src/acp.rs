use std::{io::Write as _, path::Path, sync::Arc};

use crate::{
    Agent, AgentThreadEntryContent, AgentThreadSummary, Message, MessageChunk, Role, Thread,
    ThreadEntryId, ThreadId,
};
use agentic_coding_protocol as acp;
use anyhow::{Context as _, Result};
use async_trait::async_trait;
use collections::HashMap;
use gpui::{App, AppContext, AsyncApp, Context, Entity, Task, WeakEntity};
use parking_lot::Mutex;
use project::Project;
use smol::process::Child;
use util::ResultExt;

pub struct AcpAgent {
    connection: Arc<acp::AgentConnection>,
    threads: Arc<Mutex<HashMap<ThreadId, WeakEntity<Thread>>>>,
    project: Entity<Project>,
    _handler_task: Task<()>,
    _io_task: Task<()>,
}

struct AcpClientDelegate {
    project: Entity<Project>,
    threads: Arc<Mutex<HashMap<ThreadId, WeakEntity<Thread>>>>,
    cx: AsyncApp,
    // sent_buffer_versions: HashMap<Entity<Buffer>, HashMap<u64, BufferSnapshot>>,
}

impl AcpClientDelegate {
    fn new(
        project: Entity<Project>,
        threads: Arc<Mutex<HashMap<ThreadId, WeakEntity<Thread>>>>,
        cx: AsyncApp,
    ) -> Self {
        Self {
            project,
            threads,
            cx: cx,
        }
    }

    fn update_thread<R>(
        &self,
        thread_id: &ThreadId,
        cx: &mut App,
        callback: impl FnMut(&mut Thread, &mut Context<Thread>) -> R,
    ) -> Option<R> {
        let thread = self.threads.lock().get(&thread_id)?.clone();
        let Some(thread) = thread.upgrade() else {
            self.threads.lock().remove(&thread_id);
            return None;
        };
        Some(thread.update(cx, callback))
    }
}

#[async_trait(?Send)]
impl acp::Client for AcpClientDelegate {
    async fn stat(&self, params: acp::StatParams) -> Result<acp::StatResponse> {
        let cx = &mut self.cx.clone();
        self.project.update(cx, |project, cx| {
            let path = project
                .project_path_for_absolute_path(Path::new(&params.path), cx)
                .context("Failed to get project path")?;

            match project.entry_for_path(&path, cx) {
                // todo! refresh entry?
                None => Ok(acp::StatResponse {
                    exists: false,
                    is_directory: false,
                }),
                Some(entry) => Ok(acp::StatResponse {
                    exists: entry.is_created(),
                    is_directory: entry.is_dir(),
                }),
            }
        })?
    }

    async fn stream_message_chunk(
        &self,
        params: acp::StreamMessageChunkParams,
    ) -> Result<acp::StreamMessageChunkResponse> {
        let cx = &mut self.cx.clone();

        cx.update(|cx| {
            self.update_thread(&params.thread_id.into(), cx, |thread, cx| {
                let acp::MessageChunk::Text { chunk } = &params.chunk;
                thread.push_assistant_chunk(
                    MessageChunk::Text {
                        chunk: chunk.into(),
                    },
                    cx,
                )
            });
        })?;

        Ok(acp::StreamMessageChunkResponse)
    }

    async fn read_text_file(
        &self,
        request: acp::ReadTextFileParams,
    ) -> Result<acp::ReadTextFileResponse> {
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

        buffer.update(cx, |buffer, cx| {
            let start = language::Point::new(request.line_offset.unwrap_or(0), 0);
            let end = match request.line_limit {
                None => buffer.max_point(),
                Some(limit) => start + language::Point::new(limit + 1, 0),
            };

            let content: String = buffer.text_for_range(start..end).collect();
            self.update_thread(&request.thread_id.into(), cx, |thread, cx| {
                thread.push_entry(
                    AgentThreadEntryContent::ReadFile {
                        path: request.path.clone(),
                        content: content.clone(),
                    },
                    cx,
                );
            });

            acp::ReadTextFileResponse {
                content,
                version: acp::FileVersion(0),
            }
        })
    }

    async fn read_binary_file(
        &self,
        request: acp::ReadBinaryFileParams,
    ) -> Result<acp::ReadBinaryFileResponse> {
        let cx = &mut self.cx.clone();
        let file = self
            .project
            .update(cx, |project, cx| {
                let (worktree, path) = project
                    .find_worktree(Path::new(&request.path), cx)
                    .context("Failed to get project path")?;

                let task = worktree.update(cx, |worktree, cx| worktree.load_binary_file(&path, cx));
                anyhow::Ok(task)
            })??
            .await?;

        // todo! test
        let content = cx
            .background_spawn(async move {
                let start = request.byte_offset.unwrap_or(0) as usize;
                let end = request
                    .byte_limit
                    .map(|limit| (start + limit as usize).min(file.content.len()))
                    .unwrap_or(file.content.len());

                let range_content = &file.content[start..end];

                let mut base64_content = Vec::new();
                let mut base64_encoder = base64::write::EncoderWriter::new(
                    std::io::Cursor::new(&mut base64_content),
                    &base64::engine::general_purpose::STANDARD,
                );
                base64_encoder.write_all(range_content)?;
                drop(base64_encoder);

                // SAFETY: The base64 encoder should not produce non-UTF8.
                unsafe { anyhow::Ok(String::from_utf8_unchecked(base64_content)) }
            })
            .await?;

        Ok(acp::ReadBinaryFileResponse {
            content,
            // todo!
            version: acp::FileVersion(0),
        })
    }

    async fn glob_search(&self, request: acp::GlobSearchParams) -> Result<acp::GlobSearchResponse> {
        todo!()
    }
}

impl AcpAgent {
    pub fn stdio(mut process: Child, project: Entity<Project>, cx: &mut AsyncApp) -> Arc<Self> {
        let stdin = process.stdin.take().expect("process didn't have stdin");
        let stdout = process.stdout.take().expect("process didn't have stdout");

        let threads: Arc<Mutex<HashMap<ThreadId, WeakEntity<Thread>>>> = Default::default();
        let (connection, handler_fut, io_fut) = acp::AgentConnection::connect_to_agent(
            AcpClientDelegate::new(project.clone(), threads.clone(), cx.clone()),
            stdin,
            stdout,
        );

        let io_task = cx.background_spawn(async move {
            io_fut.await.log_err();
            process.status().await.log_err();
        });

        Arc::new(Self {
            project,
            connection: Arc::new(connection),
            threads,
            _handler_task: cx.foreground_executor().spawn(handler_fut),
            _io_task: io_task,
        })
    }
}

#[async_trait(?Send)]
impl Agent for AcpAgent {
    async fn threads(&self, cx: &mut AsyncApp) -> Result<Vec<AgentThreadSummary>> {
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

    async fn create_thread(self: Arc<Self>, cx: &mut AsyncApp) -> Result<Entity<Thread>> {
        let response = self.connection.request(acp::CreateThreadParams).await?;
        let thread_id: ThreadId = response.thread_id.into();
        let agent = self.clone();
        let thread = cx.new(|_| Thread {
            title: "The agent2 thread".into(),
            id: thread_id.clone(),
            next_entry_id: ThreadEntryId(0),
            entries: Vec::default(),
            project: self.project.clone(),
            agent,
        })?;
        self.threads.lock().insert(thread_id, thread.downgrade());
        Ok(thread)
    }

    async fn open_thread(&self, id: ThreadId, cx: &mut AsyncApp) -> Result<Entity<Thread>> {
        todo!()
    }

    async fn thread_entries(
        &self,
        thread_id: ThreadId,
        cx: &mut AsyncApp,
    ) -> Result<Vec<AgentThreadEntryContent>> {
        let response = self
            .connection
            .request(acp::GetThreadEntriesParams {
                thread_id: thread_id.clone().into(),
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
                                acp::MessageChunk::Text { chunk } => MessageChunk::Text {
                                    chunk: chunk.into(),
                                },
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

    async fn send_thread_message(
        &self,
        thread_id: ThreadId,
        message: crate::Message,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        self.connection
            .request(acp::SendMessageParams {
                thread_id: thread_id.clone().into(),
                message: acp::Message {
                    role: match message.role {
                        Role::User => acp::Role::User,
                        Role::Assistant => acp::Role::Assistant,
                    },
                    chunks: message
                        .chunks
                        .into_iter()
                        .map(|chunk| match chunk {
                            MessageChunk::Text { chunk } => acp::MessageChunk::Text {
                                chunk: chunk.into(),
                            },
                            MessageChunk::File { .. } => todo!(),
                            MessageChunk::Directory { .. } => todo!(),
                            MessageChunk::Symbol { .. } => todo!(),
                            MessageChunk::Fetch { .. } => todo!(),
                        })
                        .collect(),
                },
            })
            .await?;
        Ok(())
    }
}

impl From<acp::ThreadId> for ThreadId {
    fn from(thread_id: acp::ThreadId) -> Self {
        Self(thread_id.0.into())
    }
}

impl From<ThreadId> for acp::ThreadId {
    fn from(thread_id: ThreadId) -> Self {
        acp::ThreadId(thread_id.0.to_string())
    }
}
