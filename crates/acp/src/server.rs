use crate::{AcpThread, AgentThreadEntryContent, ThreadEntryId, ThreadId, ToolCallId};
use agentic_coding_protocol as acp;
use anyhow::{Context as _, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::channel::oneshot;
use gpui::{App, AppContext, AsyncApp, Context, Entity, Task, WeakEntity};
use parking_lot::Mutex;
use project::Project;
use smol::process::Child;
use std::{io::Write as _, path::Path, sync::Arc};
use util::ResultExt;

pub struct AcpServer {
    connection: Arc<acp::AgentConnection>,
    threads: Arc<Mutex<HashMap<ThreadId, WeakEntity<AcpThread>>>>,
    project: Entity<Project>,
    _handler_task: Task<()>,
    _io_task: Task<()>,
}

struct AcpClientDelegate {
    project: Entity<Project>,
    threads: Arc<Mutex<HashMap<ThreadId, WeakEntity<AcpThread>>>>,
    cx: AsyncApp,
    // sent_buffer_versions: HashMap<Entity<Buffer>, HashMap<u64, BufferSnapshot>>,
}

impl AcpClientDelegate {
    fn new(
        project: Entity<Project>,
        threads: Arc<Mutex<HashMap<ThreadId, WeakEntity<AcpThread>>>>,
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
        callback: impl FnOnce(&mut AcpThread, &mut Context<AcpThread>) -> R,
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
                thread.push_assistant_chunk(params.chunk, cx)
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

    async fn glob_search(
        &self,
        _request: acp::GlobSearchParams,
    ) -> Result<acp::GlobSearchResponse> {
        todo!()
    }

    async fn request_tool_call(
        &self,
        request: acp::RequestToolCallParams,
    ) -> Result<acp::RequestToolCallResponse> {
        let (tx, rx) = oneshot::channel();

        let cx = &mut self.cx.clone();
        let entry_id = cx
            .update(|cx| {
                self.update_thread(&request.thread_id.into(), cx, |thread, cx| {
                    // todo! tools that don't require confirmation
                    thread.push_tool_call(request.tool_name, request.description, tx, cx)
                })
            })?
            .context("Failed to update thread")?;

        if dbg!(rx.await)? {
            Ok(acp::RequestToolCallResponse::Allowed {
                id: entry_id.into(),
            })
        } else {
            Ok(acp::RequestToolCallResponse::Rejected)
        }
    }
}

impl AcpServer {
    pub fn stdio(mut process: Child, project: Entity<Project>, cx: &mut AsyncApp) -> Arc<Self> {
        let stdin = process.stdin.take().expect("process didn't have stdin");
        let stdout = process.stdout.take().expect("process didn't have stdout");

        let threads: Arc<Mutex<HashMap<ThreadId, WeakEntity<AcpThread>>>> = Default::default();
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

impl AcpServer {
    pub async fn create_thread(self: Arc<Self>, cx: &mut AsyncApp) -> Result<Entity<AcpThread>> {
        let response = self.connection.request(acp::CreateThreadParams).await?;
        let thread_id: ThreadId = response.thread_id.into();
        let server = self.clone();
        let thread = cx.new(|_| AcpThread {
            title: "The agent2 thread".into(),
            id: thread_id.clone(),
            next_entry_id: ThreadEntryId(0),
            entries: Vec::default(),
            project: self.project.clone(),
            server,
        })?;
        self.threads.lock().insert(thread_id, thread.downgrade());
        Ok(thread)
    }

    pub async fn send_message(
        &self,
        thread_id: ThreadId,
        message: acp::Message,
        _cx: &mut AsyncApp,
    ) -> Result<()> {
        self.connection
            .request(acp::SendMessageParams {
                thread_id: thread_id.clone().into(),
                message,
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

impl From<acp::ToolCallId> for ToolCallId {
    fn from(tool_call_id: acp::ToolCallId) -> Self {
        Self(ThreadEntryId(tool_call_id.0.into()))
    }
}

impl From<ToolCallId> for acp::ToolCallId {
    fn from(tool_call_id: ToolCallId) -> Self {
        acp::ToolCallId(tool_call_id.0.0)
    }
}
