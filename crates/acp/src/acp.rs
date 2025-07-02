mod server;
mod thread_view;

use agentic_coding_protocol::{self as acp, Role};
use anyhow::{Context as _, Result};
use chrono::{DateTime, Utc};
use futures::channel::oneshot;
use gpui::{AppContext, Context, Entity, EventEmitter, SharedString, Task};
use language::LanguageRegistry;
use markdown::Markdown;
use project::Project;
use std::{mem, ops::Range, path::PathBuf, sync::Arc};
use ui::App;
use util::{ResultExt, debug_panic};

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub role: acp::Role,
    pub chunks: Vec<MessageChunk>,
}

impl Message {
    fn into_acp(self, cx: &App) -> acp::Message {
        acp::Message {
            role: self.role,
            chunks: self
                .chunks
                .into_iter()
                .map(|chunk| chunk.into_acp(cx))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessageChunk {
    Text {
        chunk: Entity<Markdown>,
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

impl MessageChunk {
    pub fn from_acp(
        chunk: acp::MessageChunk,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        match chunk {
            acp::MessageChunk::Text { chunk } => MessageChunk::Text {
                chunk: cx.new(|cx| Markdown::new(chunk.into(), Some(language_registry), None, cx)),
            },
        }
    }

    pub fn into_acp(self, cx: &App) -> acp::MessageChunk {
        match self {
            MessageChunk::Text { chunk } => acp::MessageChunk::Text {
                chunk: chunk.read(cx).source().to_string(),
            },
            MessageChunk::File { .. } => todo!(),
            MessageChunk::Directory { .. } => todo!(),
            MessageChunk::Symbol { .. } => todo!(),
            MessageChunk::Fetch { .. } => todo!(),
        }
    }

    pub fn from_str(chunk: &str, language_registry: Arc<LanguageRegistry>, cx: &mut App) -> Self {
        MessageChunk::Text {
            chunk: cx.new(|cx| {
                Markdown::new(chunk.to_owned().into(), Some(language_registry), None, cx)
            }),
        }
    }
}

#[derive(Debug)]
pub enum AgentThreadEntryContent {
    Message(Message),
    ToolCall(ToolCall),
}

#[derive(Debug)]
pub struct ToolCall {
    id: ToolCallId,
    label: Entity<Markdown>,
    status: ToolCallStatus,
}

#[derive(Debug)]
pub enum ToolCallStatus {
    WaitingForConfirmation {
        confirmation: acp::ToolCallConfirmation,
        respond_tx: oneshot::Sender<acp::ToolCallConfirmationOutcome>,
    },
    // todo! Running?
    Allowed {
        // todo! should this be variants in crate::ToolCallStatus instead?
        status: acp::ToolCallStatus,
        content: Option<Entity<Markdown>>,
    },
    Rejected,
}

/// A `ThreadEntryId` that is known to be a ToolCall
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToolCallId(ThreadEntryId);

impl ToolCallId {
    pub fn as_u64(&self) -> u64 {
        self.0.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ThreadEntryId(pub u64);

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
    EntryUpdated(usize),
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

    pub fn push_entry(
        &mut self,
        entry: AgentThreadEntryContent,
        cx: &mut Context<Self>,
    ) -> ThreadEntryId {
        let id = self.next_entry_id.post_inc();
        self.entries.push(ThreadEntry { id, content: entry });
        cx.emit(AcpThreadEvent::NewEntry);
        id
    }

    pub fn push_assistant_chunk(&mut self, chunk: acp::MessageChunk, cx: &mut Context<Self>) {
        let entries_len = self.entries.len();
        if let Some(last_entry) = self.entries.last_mut()
            && let AgentThreadEntryContent::Message(Message {
                ref mut chunks,
                role: Role::Assistant,
            }) = last_entry.content
        {
            cx.emit(AcpThreadEvent::EntryUpdated(entries_len - 1));

            if let (
                Some(MessageChunk::Text { chunk: old_chunk }),
                acp::MessageChunk::Text { chunk: new_chunk },
            ) = (chunks.last_mut(), &chunk)
            {
                old_chunk.update(cx, |old_chunk, cx| {
                    old_chunk.append(&new_chunk, cx);
                });
            } else {
                chunks.push(MessageChunk::from_acp(
                    chunk,
                    self.project.read(cx).languages().clone(),
                    cx,
                ));
            }

            return;
        }

        let chunk = MessageChunk::from_acp(chunk, self.project.read(cx).languages().clone(), cx);

        self.push_entry(
            AgentThreadEntryContent::Message(Message {
                role: Role::Assistant,
                chunks: vec![chunk],
            }),
            cx,
        );
    }

    pub fn request_tool_call(
        &mut self,
        label: String,
        confirmation: acp::ToolCallConfirmation,
        cx: &mut Context<Self>,
    ) -> ToolCallRequest {
        let (tx, rx) = oneshot::channel();

        let status = ToolCallStatus::WaitingForConfirmation {
            confirmation,
            respond_tx: tx,
        };

        let id = self.insert_tool_call(label, status, cx);
        ToolCallRequest { id, outcome: rx }
    }

    pub fn push_tool_call(&mut self, label: String, cx: &mut Context<Self>) -> ToolCallId {
        let status = ToolCallStatus::Allowed {
            status: acp::ToolCallStatus::Running,
            content: None,
        };

        self.insert_tool_call(label, status, cx)
    }

    fn insert_tool_call(
        &mut self,
        label: String,
        status: ToolCallStatus,
        cx: &mut Context<Self>,
    ) -> ToolCallId {
        let language_registry = self.project.read(cx).languages().clone();

        let entry_id = self.push_entry(
            AgentThreadEntryContent::ToolCall(ToolCall {
                // todo! clean up id creation
                id: ToolCallId(ThreadEntryId(self.entries.len() as u64)),
                label: cx.new(|cx| {
                    Markdown::new(label.into(), Some(language_registry.clone()), None, cx)
                }),
                status,
            }),
            cx,
        );

        ToolCallId(entry_id)
    }

    pub fn authorize_tool_call(
        &mut self,
        id: ToolCallId,
        outcome: acp::ToolCallConfirmationOutcome,
        cx: &mut Context<Self>,
    ) {
        let Some(entry) = self.entry_mut(id.0) else {
            return;
        };

        let AgentThreadEntryContent::ToolCall(call) = &mut entry.content else {
            debug_panic!("expected ToolCall");
            return;
        };

        let new_status = if outcome == acp::ToolCallConfirmationOutcome::Reject {
            ToolCallStatus::Rejected
        } else {
            ToolCallStatus::Allowed {
                status: acp::ToolCallStatus::Running,
                content: None,
            }
        };

        let curr_status = mem::replace(&mut call.status, new_status);

        if let ToolCallStatus::WaitingForConfirmation { respond_tx, .. } = curr_status {
            respond_tx.send(outcome).log_err();
        } else {
            debug_panic!("tried to authorize an already authorized tool call");
        }

        cx.emit(AcpThreadEvent::EntryUpdated(id.as_u64() as usize));
    }

    pub fn update_tool_call(
        &mut self,
        id: ToolCallId,
        new_status: acp::ToolCallStatus,
        new_content: Option<acp::ToolCallContent>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let language_registry = self.project.read(cx).languages().clone();
        let entry = self.entry_mut(id.0).context("Entry not found")?;

        match &mut entry.content {
            AgentThreadEntryContent::ToolCall(call) => match &mut call.status {
                ToolCallStatus::Allowed { content, status } => {
                    *content = new_content.map(|new_content| {
                        let acp::ToolCallContent::Markdown { markdown } = new_content;

                        cx.new(|cx| {
                            Markdown::new(markdown.into(), Some(language_registry), None, cx)
                        })
                    });

                    *status = new_status;
                }
                ToolCallStatus::WaitingForConfirmation { .. } => {
                    anyhow::bail!("Tool call hasn't been authorized yet")
                }
                ToolCallStatus::Rejected => {
                    anyhow::bail!("Tool call was rejected and therefore can't be updated")
                }
            },
            _ => anyhow::bail!("Entry is not a tool call"),
        }

        cx.emit(AcpThreadEvent::EntryUpdated(id.as_u64() as usize));
        Ok(())
    }

    fn entry_mut(&mut self, id: ThreadEntryId) -> Option<&mut ThreadEntry> {
        let entry = self.entries.get_mut(id.0 as usize);
        debug_assert!(
            entry.is_some(),
            "We shouldn't give out ids to entries that don't exist"
        );
        entry
    }

    /// Returns true if the last turn is awaiting tool authorization
    pub fn waiting_for_tool_confirmation(&self) -> bool {
        for entry in self.entries.iter().rev() {
            match &entry.content {
                AgentThreadEntryContent::ToolCall(call) => match call.status {
                    ToolCallStatus::WaitingForConfirmation { .. } => return true,
                    ToolCallStatus::Allowed { .. } | ToolCallStatus::Rejected => continue,
                },
                AgentThreadEntryContent::Message(_) => {
                    // Reached the beginning of the turn
                    return false;
                }
            }
        }
        false
    }

    pub fn send(&mut self, message: &str, cx: &mut Context<Self>) -> Task<Result<()>> {
        let agent = self.server.clone();
        let id = self.id.clone();
        let chunk = MessageChunk::from_str(message, self.project.read(cx).languages().clone(), cx);
        let message = Message {
            role: Role::User,
            chunks: vec![chunk],
        };
        self.push_entry(AgentThreadEntryContent::Message(message.clone()), cx);
        let acp_message = message.into_acp(cx);
        cx.spawn(async move |_, cx| {
            agent.send_message(id, acp_message, cx).await?;
            Ok(())
        })
    }
}

pub struct ToolCallRequest {
    pub id: ToolCallId,
    pub outcome: oneshot::Receiver<acp::ToolCallConfirmationOutcome>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{FutureExt as _, channel::mpsc, select};
    use gpui::{AsyncApp, TestAppContext};
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use smol::stream::StreamExt as _;
    use std::{env, path::Path, process::Stdio, time::Duration};
    use util::path;

    fn init_test(cx: &mut TestAppContext) {
        env_logger::try_init().ok();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
            language::init(cx);
        });
    }

    #[gpui::test]
    async fn test_gemini_basic(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let server = gemini_acp_server(project.clone(), cx.to_async()).unwrap();
        let thread = server.create_thread(&mut cx.to_async()).await.unwrap();
        thread
            .update(cx, |thread, cx| thread.send("Hello from Zed!", cx))
            .await
            .unwrap();

        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.entries.len(), 2);
            assert!(matches!(
                thread.entries[0].content,
                AgentThreadEntryContent::Message(Message {
                    role: Role::User,
                    ..
                })
            ));
            assert!(matches!(
                thread.entries[1].content,
                AgentThreadEntryContent::Message(Message {
                    role: Role::Assistant,
                    ..
                })
            ));
        });
    }

    #[gpui::test]
    async fn test_gemini_tool_call(cx: &mut TestAppContext) {
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
                    "Read the '/private/tmp/foo' file and tell me what you see.",
                    cx,
                )
            })
            .await
            .unwrap();
        thread.read_with(cx, |thread, cx| {
            let AgentThreadEntryContent::ToolCall(ToolCall {
                label,
                status: ToolCallStatus::Allowed { .. },
                ..
            }) = &thread.entries()[1].content
            else {
                panic!();
            };

            label.read_with(cx, |md, _cx| {
                assert_eq!(md.source(), "ReadFile");
            });

            assert!(matches!(
                thread.entries[2].content,
                AgentThreadEntryContent::Message(Message {
                    role: Role::Assistant,
                    ..
                })
            ));
        });
    }

    #[gpui::test]
    async fn test_gemini_tool_call_with_confirmation(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [path!("/private/tmp").as_ref()], cx).await;
        let server = gemini_acp_server(project.clone(), cx.to_async()).unwrap();
        let thread = server.create_thread(&mut cx.to_async()).await.unwrap();
        let full_turn = thread.update(cx, |thread, cx| {
            thread.send(r#"Run `echo "Hello, world!"`"#, cx)
        });

        run_until_tool_call(&thread, cx).await;

        let tool_call_id = thread.read_with(cx, |thread, cx| {
            let AgentThreadEntryContent::ToolCall(ToolCall {
                id,
                label,
                status:
                    ToolCallStatus::WaitingForConfirmation {
                        confirmation: acp::ToolCallConfirmation::Execute { root_command, .. },
                        ..
                    },
            }) = &thread.entries()[1].content
            else {
                panic!();
            };

            assert_eq!(root_command, "echo");

            label.read_with(cx, |md, _cx| {
                assert_eq!(md.source(), "Shell");
            });

            *id
        });

        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(tool_call_id, acp::ToolCallConfirmationOutcome::Allow, cx);

            assert!(matches!(
                &thread.entries()[1].content,
                AgentThreadEntryContent::ToolCall(ToolCall {
                    status: ToolCallStatus::Allowed { .. },
                    ..
                })
            ));
        });

        full_turn.await.unwrap();

        thread.read_with(cx, |thread, cx| {
            let AgentThreadEntryContent::ToolCall(ToolCall {
                status: ToolCallStatus::Allowed { content, .. },
                ..
            }) = &thread.entries()[1].content
            else {
                panic!();
            };

            content.as_ref().unwrap().read_with(cx, |md, _cx| {
                assert!(
                    md.source().contains("Hello, world!"),
                    r#"Expected '{}' to contain "Hello, world!""#,
                    md.source()
                );
            });
        });
    }

    async fn run_until_tool_call(thread: &Entity<AcpThread>, cx: &mut TestAppContext) {
        let (mut tx, mut rx) = mpsc::channel::<()>(1);

        let subscription = cx.update(|cx| {
            cx.subscribe(thread, move |thread, _, cx| {
                if thread
                    .read(cx)
                    .entries
                    .iter()
                    .any(|e| matches!(e.content, AgentThreadEntryContent::ToolCall(_)))
                {
                    tx.try_send(()).unwrap();
                }
            })
        });

        select! {
            _ = futures::FutureExt::fuse(smol::Timer::after(Duration::from_secs(10))) => {
                panic!("Timeout waiting for tool call")
            }
            _ = rx.next().fuse() => {
                drop(subscription);
            }
        }
    }

    pub fn gemini_acp_server(project: Entity<Project>, mut cx: AsyncApp) -> Result<Arc<AcpServer>> {
        let cli_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../gemini-cli/packages/cli");
        let mut command = util::command::new_smol_command("node");
        command
            .arg(cli_path)
            .arg("--acp")
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
