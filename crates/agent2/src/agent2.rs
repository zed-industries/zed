mod acp;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use gpui::{AppContext, AsyncApp, Context, Entity, SharedString, Task};
use project::Project;
use std::{ops::Range, path::PathBuf, sync::Arc};

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
    ) -> Result<()>;
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
        _: &mut Context<Self>,
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
        cx.spawn(async move |_, cx| {
            agent.send_thread_message(id, message, cx).await?;
            Ok(())
        })
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
    use std::{env, path::Path, process::Stdio};
    use util::path;

    fn init_test(cx: &mut TestAppContext) {
        env_logger::init();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
            language::init(cx);
        });
    }

    #[gpui::test]
    async fn test_gemini(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/private/tmp"),
            json!({"foo": "Lorem ipsum dolor", "bar": "bar", "baz": "baz"}),
        )
        .await;
        let project = Project::test(fs, [path!("/private/tmp").as_ref()], cx).await;
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
                            "Read the '/private/tmp/foo' file and output all of its contents."
                                .into(),
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
                            path: "/private/tmp/foo".into(),
                            content: "Lorem ipsum dolor".into(),
                        }
                }),
                "Thread does not contain entry. Actual: {:?}",
                thread.entries()
            );
        });
    }

    pub fn gemini_agent(project: Entity<Project>, mut cx: AsyncApp) -> Result<Arc<AcpAgent>> {
        let cli_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../gemini-cli/packages/cli");
        let child = util::command::new_smol_command("node")
            .arg(cli_path)
            .arg("--acp")
            .args(["--model", "gemini-2.5-flash"])
            .current_dir("/private/tmp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true)
            .spawn()
            .unwrap();

        Ok(AcpAgent::stdio(child, project, &mut cx))
    }
}
