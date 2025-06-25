mod acp;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::{
    FutureExt, StreamExt,
    channel::{mpsc, oneshot},
    select_biased,
    stream::{BoxStream, FuturesUnordered},
};
use gpui::{AppContext, AsyncApp, Context, Entity, SharedString, Task};
use project::Project;
use std::{future, ops::Range, path::PathBuf, pin::pin, sync::Arc};

#[async_trait(?Send)]
pub trait Agent: 'static {
    async fn threads(&self, cx: &mut AsyncApp) -> Result<Vec<AgentThreadSummary>>;
    async fn create_thread(self: Arc<Self>, cx: &mut AsyncApp) -> Result<Entity<Thread>>;
    async fn open_thread(&self, id: ThreadId, cx: &mut AsyncApp) -> Result<Entity<Thread>>;
    async fn thread_entries(
        &self,
        id: ThreadId,
        cx: &mut AsyncApp,
    ) -> Result<Vec<AgentThreadEntryContent>>;
    async fn send_thread_message(
        &self,
        thread_id: ThreadId,
        message: Message,
        cx: &mut AsyncApp,
    ) -> Result<mpsc::UnboundedReceiver<Result<ResponseEvent>>>;
}

pub enum ResponseEvent {
    MessageResponse(MessageResponse),
    ReadFileRequest(ReadFileRequest),
    // GlobSearchRequest(SearchRequest),
    // RegexSearchRequest(RegexSearchRequest),
    // RunCommandRequest(RunCommandRequest),
    // WebSearchResponse(WebSearchResponse),
}

pub struct MessageResponse {
    role: Role,
    chunks: BoxStream<'static, Result<MessageChunk>>,
}

#[derive(Debug)]
pub struct ReadFileRequest {
    path: PathBuf,
    range: Range<usize>,
    response_tx: oneshot::Sender<Result<FileContent>>,
}

impl ReadFileRequest {
    pub fn respond(self, content: Result<FileContent>) {
        self.response_tx.send(content).ok();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThreadId(SharedString);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FileVersion(u64);

#[derive(Debug)]
pub struct AgentThreadSummary {
    pub id: ThreadId,
    pub title: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileContent {
    pub path: PathBuf,
    pub version: FileVersion,
    pub content: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Message {
    pub role: Role,
    pub chunks: Vec<MessageChunk>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum MessageChunk {
    Text {
        chunk: String,
    },
    File {
        content: FileContent,
    },
    Directory {
        path: PathBuf,
        contents: Vec<FileContent>,
    },
    Symbol {
        path: PathBuf,
        range: Range<u64>,
        version: FileVersion,
        name: String,
        content: String,
    },
    Thread {
        title: String,
        content: Vec<AgentThreadEntryContent>,
    },
    Fetch {
        url: String,
        content: String,
    },
}

impl From<&str> for MessageChunk {
    fn from(chunk: &str) -> Self {
        MessageChunk::Text {
            chunk: chunk.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum AgentThreadEntryContent {
    Message(Message),
    ReadFile { path: PathBuf, content: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ThreadEntryId(usize);

impl ThreadEntryId {
    pub fn post_inc(&mut self) -> Self {
        let id = *self;
        self.0 += 1;
        id
    }
}

#[derive(Debug)]
pub struct ThreadEntry {
    pub id: ThreadEntryId,
    pub content: AgentThreadEntryContent,
}

pub struct ThreadStore {
    threads: Vec<AgentThreadSummary>,
    agent: Arc<dyn Agent>,
    project: Entity<Project>,
}

impl ThreadStore {
    pub async fn load(
        agent: Arc<dyn Agent>,
        project: Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<Entity<Self>> {
        let threads = agent.threads(cx).await?;
        cx.new(|_cx| Self {
            threads,
            agent,
            project,
        })
    }

    /// Returns the threads in reverse chronological order.
    pub fn threads(&self) -> &[AgentThreadSummary] {
        &self.threads
    }

    /// Opens a thread with the given ID.
    pub fn open_thread(
        &self,
        id: ThreadId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Thread>>> {
        let agent = self.agent.clone();
        cx.spawn(async move |_, cx| agent.open_thread(id, cx).await)
    }

    /// Creates a new thread.
    pub fn create_thread(&self, cx: &mut Context<Self>) -> Task<Result<Entity<Thread>>> {
        let agent = self.agent.clone();
        cx.spawn(async move |_, cx| agent.create_thread(cx).await)
    }
}

pub struct Thread {
    id: ThreadId,
    next_entry_id: ThreadEntryId,
    entries: Vec<ThreadEntry>,
    agent: Arc<dyn Agent>,
    project: Entity<Project>,
}

impl Thread {
    pub async fn load(
        agent: Arc<dyn Agent>,
        thread_id: ThreadId,
        project: Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<Entity<Self>> {
        let entries = agent.thread_entries(thread_id.clone(), cx).await?;
        cx.new(|cx| Self::new(agent, thread_id, entries, project, cx))
    }

    pub fn new(
        agent: Arc<dyn Agent>,
        thread_id: ThreadId,
        entries: Vec<AgentThreadEntryContent>,
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut next_entry_id = ThreadEntryId(0);
        Self {
            entries: entries
                .into_iter()
                .map(|entry| ThreadEntry {
                    id: next_entry_id.post_inc(),
                    content: entry,
                })
                .collect(),
            agent,
            id: thread_id,
            next_entry_id,
            project,
        }
    }

    pub fn entries(&self) -> &[ThreadEntry] {
        &self.entries
    }

    pub fn push_entry(&mut self, entry: AgentThreadEntryContent, cx: &mut Context<Self>) {
        self.entries.push(ThreadEntry {
            id: self.next_entry_id.post_inc(),
            content: entry,
        });
        cx.notify();
    }

    pub fn send(&mut self, message: Message, cx: &mut Context<Self>) -> Task<Result<()>> {
        let agent = self.agent.clone();
        let id = self.id.clone();
        cx.spawn(async move |this, cx| {
            let mut events = agent.send_thread_message(id, message, cx).await?;
            let mut pending_event_handlers = FuturesUnordered::new();

            loop {
                let mut next_event_handler_result = pin!(
                    async {
                        if pending_event_handlers.is_empty() {
                            future::pending::<()>().await;
                        }

                        pending_event_handlers.next().await
                    }
                    .fuse()
                );

                select_biased! {
                    event = events.next() => {
                        let Some(event) = event else {
                            while let Some(result) = pending_event_handlers.next().await {
                                result?;
                            }

                            break;
                        };

                        let task = match event {
                            Ok(ResponseEvent::MessageResponse(message)) => {
                                this.update(cx, |this, cx| this.handle_message_response(message, cx))?
                            }
                            Ok(ResponseEvent::ReadFileRequest(request)) => {
                                this.update(cx, |this, cx| this.handle_read_file_request(request, cx))?
                            }
                            Err(_) => todo!(),
                        };
                        pending_event_handlers.push(task);
                    }
                    result = next_event_handler_result => {
                        // Event handlers should only return errors that are
                        // unrecoverable and should therefore stop this turn of
                        // the agentic loop.
                        result.unwrap()?;
                    }
                }
            }

            Ok(())
        })
    }

    fn handle_message_response(
        &mut self,
        mut message: MessageResponse,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let entry_id = self.next_entry_id.post_inc();
        self.entries.push(ThreadEntry {
            id: entry_id,
            content: AgentThreadEntryContent::Message(Message {
                role: message.role,
                chunks: Vec::new(),
            }),
        });
        cx.notify();

        cx.spawn(async move |this, cx| {
            while let Some(chunk) = message.chunks.next().await {
                match chunk {
                    Ok(chunk) => {
                        this.update(cx, |this, cx| {
                            let ix = this
                                .entries
                                .binary_search_by_key(&entry_id, |entry| entry.id)
                                .map_err(|_| anyhow!("message not found"))?;
                            let AgentThreadEntryContent::Message(message) =
                                &mut this.entries[ix].content
                            else {
                                unreachable!()
                            };
                            message.chunks.push(chunk);
                            cx.notify();
                            anyhow::Ok(())
                        })??;
                    }
                    Err(err) => todo!("show error"),
                }
            }

            Ok(())
        })
    }

    fn handle_read_file_request(
        &mut self,
        request: ReadFileRequest,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acp::AcpAgent;
    use gpui::TestAppContext;
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use std::{env, process::Stdio};
    use util::path;

    fn init_test(cx: &mut TestAppContext) {
        env_logger::init();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
        });
    }

    #[gpui::test]
    async fn test_gemini(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            json!({"foo": "Lorem ipsum dolor", "bar": "bar", "baz": "baz"}),
        )
        .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
        let agent = gemini_agent(project.clone(), cx.to_async()).unwrap();
        let thread_store = ThreadStore::load(Arc::new(agent), project, &mut cx.to_async())
            .await
            .unwrap();
        let thread = thread_store
            .update(cx, |thread_store, cx| {
                assert_eq!(thread_store.threads().len(), 0);
                thread_store.create_thread(cx)
            })
            .await
            .unwrap();
        thread
            .update(cx, |thread, cx| {
                thread.send(
                    Message {
                        role: Role::User,
                        chunks: vec![
                            "Read the 'test/foo' file and output all of its contents.".into(),
                        ],
                    },
                    cx,
                )
            })
            .await
            .unwrap();
        thread.read_with(cx, |thread, _| {
            assert!(
                thread.entries().iter().any(|entry| {
                    entry.content
                        == AgentThreadEntryContent::ReadFile {
                            path: "test/foo".into(),
                            content: "Lorem ipsum dolor".into(),
                        }
                }),
                "Thread does not contain entry. Actual: {:?}",
                thread.entries()
            );
        });
    }

    pub fn gemini_agent(project: Entity<Project>, cx: AsyncApp) -> Result<AcpAgent> {
        let child = util::command::new_smol_command("node")
            .arg("../../../gemini-cli/packages/cli")
            .arg("--acp")
            .args(["--model", "gemini-2.5-flash"])
            .env("GEMINI_API_KEY", env::var("GEMINI_API_KEY").unwrap())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true)
            .spawn()
            .unwrap();

        Ok(AcpAgent::stdio(child, project, cx))
    }
}
