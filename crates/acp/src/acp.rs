mod server;
mod thread_view;

use anyhow::Result;
use chrono::{DateTime, Utc};
use gpui::{Context, Entity, SharedString, Task};
use project::Project;
use std::{ops::Range, path::PathBuf, sync::Arc};

pub use server::AcpServer;
pub use thread_view::AcpThreadView;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileContent {
    pub path: PathBuf,
    pub version: FileVersion,
    pub content: SharedString,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Role {
    User,
    Assistant,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub role: Role,
    pub chunks: Vec<MessageChunk>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessageChunk {
    Text {
        // todo! should it be shared string? what about streaming?
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
        name: SharedString,
        content: SharedString,
    },
    Fetch {
        url: SharedString,
        content: SharedString,
    },
}

impl From<&str> for MessageChunk {
    fn from(chunk: &str) -> Self {
        MessageChunk::Text {
            chunk: chunk.to_string().into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

pub struct AcpThread {
    id: ThreadId,
    next_entry_id: ThreadEntryId,
    entries: Vec<ThreadEntry>,
    server: Arc<AcpServer>,
    title: SharedString,
    project: Entity<Project>,
}

enum AcpThreadEvent {
    NewEntry,
    LastEntryUpdated,
}

impl EventEmitter<AcpThreadEvent> for AcpThread {}

impl AcpThread {
    pub fn new(
        server: Arc<AcpServer>,
        thread_id: ThreadId,
        entries: Vec<AgentThreadEntryContent>,
        project: Entity<Project>,
        _: &mut Context<Self>,
    ) -> Self {
        let mut next_entry_id = ThreadEntryId(0);
        Self {
            title: "A new agent2 thread".into(),
            entries: entries
                .into_iter()
                .map(|entry| ThreadEntry {
                    id: next_entry_id.post_inc(),
                    content: entry,
                })
                .collect(),
            server,
            id: thread_id,
            next_entry_id,
            project,
        }
    }

    pub fn title(&self) -> SharedString {
        self.title.clone()
    }

    pub fn entries(&self) -> &[ThreadEntry] {
        &self.entries
    }

    pub fn push_entry(&mut self, entry: AgentThreadEntryContent, cx: &mut Context<Self>) {
        self.entries.push(ThreadEntry {
            id: self.next_entry_id.post_inc(),
            content: entry,
        });
        cx.emit(AcpThreadEvent::NewEntry)
    }

    pub fn push_assistant_chunk(&mut self, chunk: MessageChunk, cx: &mut Context<Self>) {
        if let Some(last_entry) = self.entries.last_mut()
            && let AgentThreadEntryContent::Message(Message {
                ref mut chunks,
                role: Role::Assistant,
            }) = last_entry.content
        {
            cx.emit(AcpThreadEvent::LastEntryUpdated);

            if let (
                Some(MessageChunk::Text { chunk: old_chunk }),
                MessageChunk::Text { chunk: new_chunk },
            ) = (chunks.last_mut(), &chunk)
            {
                old_chunk.push_str(&new_chunk);
            } else {
                chunks.push(chunk);
                return cx.notify();
            }

            return;
        }

        self.push_entry(
            AgentThreadEntryContent::Message(Message {
                role: Role::Assistant,
                chunks: vec![chunk],
            }),
        });
        cx.notify();
    }

    pub fn send(&mut self, message: Message, cx: &mut Context<Self>) -> Task<Result<()>> {
        let agent = self.server.clone();
        let id = self.id.clone();
        self.push_entry(AgentThreadEntryContent::Message(message.clone()), cx);
        cx.spawn(async move |_, cx| {
            agent.send_message(id, message, cx).await?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AsyncApp, TestAppContext};
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
        let server = gemini_acp_server(project.clone(), cx.to_async()).unwrap();
        let thread = server.create_thread(&mut cx.to_async()).await.unwrap();
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
            assert!(matches!(
                thread.entries[0].content,
                AgentThreadEntryContent::Message(Message {
                    role: Role::User,
                    ..
                })
            ));
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

    pub fn gemini_acp_server(project: Entity<Project>, mut cx: AsyncApp) -> Result<Arc<AcpServer>> {
        let cli_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../gemini-cli/packages/cli");
        let mut command = util::command::new_smol_command("node");
        command
            .arg(cli_path)
            .arg("--acp")
            .args(["--model", "gemini-2.5-flash"])
            .current_dir("/private/tmp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true);

        if let Ok(gemini_key) = std::env::var("GEMINI_API_KEY") {
            command.env("GEMINI_API_KEY", gemini_key);
        }

        let child = command.spawn().unwrap();

        Ok(AcpServer::stdio(child, project, &mut cx))
    }
}
