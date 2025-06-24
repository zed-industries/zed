mod acp;

use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use futures::{
    FutureExt, StreamExt,
    channel::{mpsc, oneshot},
    select_biased,
    stream::{BoxStream, FuturesUnordered},
};
use gpui::{AppContext, AsyncApp, Context, Entity, Task, WeakEntity};
use project::Project;
use std::{future, ops::Range, path::PathBuf, pin::pin, sync::Arc};

pub trait Agent: 'static {
    type Thread: AgentThread;

    fn threads(&self) -> impl Future<Output = Result<Vec<AgentThreadSummary>>>;
    fn create_thread(&self) -> impl Future<Output = Result<Self::Thread>>;
    fn open_thread(&self, id: ThreadId) -> impl Future<Output = Result<Self::Thread>>;
}

pub trait AgentThread: 'static {
    fn entries(&self) -> impl Future<Output = Result<Vec<AgentThreadEntry>>>;
    fn send(
        &self,
        message: Message,
    ) -> impl Future<Output = Result<mpsc::UnboundedReceiver<Result<ResponseEvent>>>>;
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

pub struct ThreadId(String);

pub struct FileVersion(u64);

pub struct AgentThreadSummary {
    pub id: ThreadId,
    pub title: String,
    pub created_at: DateTime<Utc>,
}

pub struct FileContent {
    pub path: PathBuf,
    pub version: FileVersion,
    pub content: String,
}

pub enum Role {
    User,
    Assistant,
}

pub struct Message {
    pub role: Role,
    pub chunks: Vec<MessageChunk>,
}

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
        content: Vec<AgentThreadEntry>,
    },
    Fetch {
        url: String,
        content: String,
    },
}

pub enum AgentThreadEntry {
    Message(Message),
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

pub struct ThreadEntry {
    pub id: ThreadEntryId,
    pub entry: AgentThreadEntry,
}

pub struct ThreadStore<T: Agent> {
    threads: Vec<AgentThreadSummary>,
    agent: Arc<T>,
    project: Entity<Project>,
}

impl<T: Agent> ThreadStore<T> {
    pub async fn load(
        agent: Arc<T>,
        project: Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<Entity<Self>> {
        let threads = agent.threads().await?;
        cx.new(|cx| Self {
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
    ) -> Task<Result<Entity<Thread<T::Thread>>>> {
        let agent = self.agent.clone();
        let project = self.project.clone();
        cx.spawn(async move |_, cx| {
            let agent_thread = agent.open_thread(id).await?;
            Thread::load(Arc::new(agent_thread), project, cx).await
        })
    }

    /// Creates a new thread.
    pub fn create_thread(&self, cx: &mut Context<Self>) -> Task<Result<Entity<Thread<T::Thread>>>> {
        let agent = self.agent.clone();
        let project = self.project.clone();
        cx.spawn(async move |_, cx| {
            let agent_thread = agent.create_thread().await?;
            Thread::load(Arc::new(agent_thread), project, cx).await
        })
    }
}

pub struct Thread<T: AgentThread> {
    next_entry_id: ThreadEntryId,
    entries: Vec<ThreadEntry>,
    agent_thread: Arc<T>,
    project: Entity<Project>,
}

impl<T: AgentThread> Thread<T> {
    pub async fn load(
        agent_thread: Arc<T>,
        project: Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<Entity<Self>> {
        let entries = agent_thread.entries().await?;
        cx.new(|cx| Self::new(agent_thread, entries, project, cx))
    }

    pub fn new(
        agent_thread: Arc<T>,
        entries: Vec<AgentThreadEntry>,
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut next_entry_id = ThreadEntryId(0);
        Self {
            entries: entries
                .into_iter()
                .map(|entry| ThreadEntry {
                    id: next_entry_id.post_inc(),
                    entry,
                })
                .collect(),
            next_entry_id,
            agent_thread,
            project,
        }
    }

    async fn handle_message(
        this: WeakEntity<Self>,
        role: Role,
        mut chunks: BoxStream<'static, Result<MessageChunk>>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let entry_id = this.update(cx, |this, cx| {
            let entry_id = this.next_entry_id.post_inc();
            this.entries.push(ThreadEntry {
                id: entry_id,
                entry: AgentThreadEntry::Message(Message {
                    role,
                    chunks: Vec::new(),
                }),
            });
            cx.notify();
            entry_id
        })?;

        while let Some(chunk) = chunks.next().await {
            match chunk {
                Ok(chunk) => {
                    this.update(cx, |this, cx| {
                        let ix = this
                            .entries
                            .binary_search_by_key(&entry_id, |entry| entry.id)
                            .map_err(|_| anyhow!("message not found"))?;
                        let AgentThreadEntry::Message(message) = &mut this.entries[ix].entry else {
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
    }

    pub fn entries(&self) -> &[ThreadEntry] {
        &self.entries
    }

    pub fn send(&mut self, message: Message, cx: &mut Context<Self>) -> Task<Result<()>> {
        let agent_thread = self.agent_thread.clone();
        cx.spawn(async move |this, cx| {
            let mut events = agent_thread.send(message).await?;
            let mut pending_event_handlers = FuturesUnordered::new();

            loop {
                let mut next_event_handler_result = pin!(async {
                    if pending_event_handlers.is_empty() {
                        future::pending::<()>().await;
                    }

                    pending_event_handlers.next().await
                }.fuse());

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
            entry: AgentThreadEntry::Message(Message {
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
                            let AgentThreadEntry::Message(message) = &mut this.entries[ix].entry
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
    use agentic_coding_protocol::Client;
    use gpui::{BackgroundExecutor, TestAppContext};
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use smol::process::Child;
    use std::env;
    use util::path;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
        });
    }

    #[gpui::test]
    async fn test_basic(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            json!({"foo": "foo", "bar": "bar", "baz": "baz"}),
        )
        .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
        let agent = GeminiAgent::start(&cx.executor()).await.unwrap();
        let thread_store = ThreadStore::load(Arc::new(agent), project, &mut cx.to_async())
            .await
            .unwrap();
    }

    struct TestClient;

    #[async_trait]
    impl Client for TestClient {
        async fn read_file(&self, _request: ReadFileParams) -> Result<ReadFileResponse> {
            Ok(ReadFileResponse {
                version: FileVersion(0),
                content: "the content".into(),
            })
        }
    }

    struct GeminiAgent {
        child: Child,
        _task: Task<()>,
    }

    impl GeminiAgent {
        pub fn start(executor: &BackgroundExecutor) -> Task<Result<Self>> {
            executor.spawn(async move {
                // todo!
                let child = util::command::new_smol_command("node")
                    .arg("../gemini-cli/packages/cli")
                    .arg("--acp")
                    .env("GEMINI_API_KEY", env::var("GEMINI_API_KEY").unwrap())
                    .kill_on_drop(true)
                    .spawn()
                    .unwrap();

                Ok(GeminiAgent { child })
            })
        }
    }

    impl Agent for GeminiAgent {
        type Thread = GeminiAgentThread;

        async fn threads(&self) -> Result<Vec<AgentThreadSummary>> {
            todo!()
        }

        async fn create_thread(&self) -> Result<Self::Thread> {
            todo!()
        }

        async fn open_thread(&self, id: ThreadId) -> Result<Self::Thread> {
            todo!()
        }
    }

    struct GeminiAgentThread {}

    impl AgentThread for GeminiAgentThread {
        async fn entries(&self) -> Result<Vec<AgentThreadEntry>> {
            todo!()
        }

        async fn send(
            &self,
            _message: Message,
        ) -> Result<mpsc::UnboundedReceiver<Result<ResponseEvent>>> {
            todo!()
        }
    }
}
