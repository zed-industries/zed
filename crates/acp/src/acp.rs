mod server;
mod thread_view;

use agentic_coding_protocol::{self as acp};
use anyhow::{Context as _, Result};
use buffer_diff::BufferDiff;
use chrono::{DateTime, Utc};
use editor::{MultiBuffer, PathKey};
use futures::channel::oneshot;
use gpui::{AppContext, Context, Entity, EventEmitter, SharedString, Task};
use language::{Anchor, Buffer, Capability, LanguageRegistry, OffsetRangeExt as _};
use markdown::Markdown;
use project::Project;
use std::{mem, ops::Range, path::PathBuf, sync::Arc};
use ui::{App, IconName};
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
pub struct UserMessage {
    pub chunks: Vec<UserMessageChunk>,
}

impl UserMessage {
    fn into_acp(self, cx: &App) -> acp::UserMessage {
        acp::UserMessage {
            chunks: self
                .chunks
                .into_iter()
                .map(|chunk| chunk.into_acp(cx))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserMessageChunk {
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

impl UserMessageChunk {
    pub fn into_acp(self, cx: &App) -> acp::UserMessageChunk {
        match self {
            Self::Text { chunk } => acp::UserMessageChunk::Text {
                chunk: chunk.read(cx).source().to_string(),
            },
            Self::File { .. } => todo!(),
            Self::Directory { .. } => todo!(),
            Self::Symbol { .. } => todo!(),
            Self::Fetch { .. } => todo!(),
        }
    }

    pub fn from_str(chunk: &str, language_registry: Arc<LanguageRegistry>, cx: &mut App) -> Self {
        Self::Text {
            chunk: cx.new(|cx| {
                Markdown::new(chunk.to_owned().into(), Some(language_registry), None, cx)
            }),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssistantMessage {
    pub chunks: Vec<AssistantMessageChunk>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssistantMessageChunk {
    Text { chunk: Entity<Markdown> },
    Thought { chunk: Entity<Markdown> },
}

impl AssistantMessageChunk {
    pub fn from_acp(
        chunk: acp::AssistantMessageChunk,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        match chunk {
            acp::AssistantMessageChunk::Text { chunk } => Self::Text {
                chunk: cx.new(|cx| Markdown::new(chunk.into(), Some(language_registry), None, cx)),
            },
            acp::AssistantMessageChunk::Thought { chunk } => Self::Thought {
                chunk: cx.new(|cx| Markdown::new(chunk.into(), Some(language_registry), None, cx)),
            },
        }
    }

    pub fn from_str(chunk: &str, language_registry: Arc<LanguageRegistry>, cx: &mut App) -> Self {
        Self::Text {
            chunk: cx.new(|cx| {
                Markdown::new(chunk.to_owned().into(), Some(language_registry), None, cx)
            }),
        }
    }
}

#[derive(Debug)]
pub enum AgentThreadEntryContent {
    UserMessage(UserMessage),
    AssistantMessage(AssistantMessage),
    ToolCall(ToolCall),
}

#[derive(Debug)]
pub struct ToolCall {
    id: ToolCallId,
    label: Entity<Markdown>,
    icon: IconName,
    content: Option<ToolCallContent>,
    status: ToolCallStatus,
}

#[derive(Debug)]
pub enum ToolCallStatus {
    WaitingForConfirmation {
        confirmation: ToolCallConfirmation,
        respond_tx: oneshot::Sender<acp::ToolCallConfirmationOutcome>,
    },
    Allowed {
        status: acp::ToolCallStatus,
    },
    Rejected,
}

#[derive(Debug)]
pub enum ToolCallConfirmation {
    Edit {
        description: Option<Entity<Markdown>>,
    },
    Execute {
        command: String,
        root_command: String,
        description: Option<Entity<Markdown>>,
    },
    Mcp {
        server_name: String,
        tool_name: String,
        tool_display_name: String,
        description: Option<Entity<Markdown>>,
    },
    Fetch {
        urls: Vec<String>,
        description: Option<Entity<Markdown>>,
    },
    Other {
        description: Entity<Markdown>,
    },
}

impl ToolCallConfirmation {
    pub fn from_acp(
        confirmation: acp::ToolCallConfirmation,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        let to_md = |description: String, cx: &mut App| -> Entity<Markdown> {
            cx.new(|cx| {
                Markdown::new(
                    description.into(),
                    Some(language_registry.clone()),
                    None,
                    cx,
                )
            })
        };

        match confirmation {
            acp::ToolCallConfirmation::Edit { description } => Self::Edit {
                description: description.map(|description| to_md(description, cx)),
            },
            acp::ToolCallConfirmation::Execute {
                command,
                root_command,
                description,
            } => Self::Execute {
                command,
                root_command,
                description: description.map(|description| to_md(description, cx)),
            },
            acp::ToolCallConfirmation::Mcp {
                server_name,
                tool_name,
                tool_display_name,
                description,
            } => Self::Mcp {
                server_name,
                tool_name,
                tool_display_name,
                description: description.map(|description| to_md(description, cx)),
            },
            acp::ToolCallConfirmation::Fetch { urls, description } => Self::Fetch {
                urls,
                description: description.map(|description| to_md(description, cx)),
            },
            acp::ToolCallConfirmation::Other { description } => Self::Other {
                description: to_md(description, cx),
            },
        }
    }
}

#[derive(Debug)]
pub enum ToolCallContent {
    Markdown { markdown: Entity<Markdown> },
    Diff { diff: Diff },
}

impl ToolCallContent {
    pub fn from_acp(
        content: acp::ToolCallContent,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        match content {
            acp::ToolCallContent::Markdown { markdown } => Self::Markdown {
                markdown: cx.new(|cx| Markdown::new_text(markdown.into(), cx)),
            },
            acp::ToolCallContent::Diff { diff } => Self::Diff {
                diff: Diff::from_acp(diff, language_registry, cx),
            },
        }
    }
}

#[derive(Debug)]
pub struct Diff {
    multibuffer: Entity<MultiBuffer>,
    path: PathBuf,
    _task: Task<Result<()>>,
}

impl Diff {
    pub fn from_acp(
        diff: acp::Diff,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        let acp::Diff {
            path,
            old_text,
            new_text,
        } = diff;

        let multibuffer = cx.new(|_cx| MultiBuffer::without_headers(Capability::ReadOnly));

        let new_buffer = cx.new(|cx| Buffer::local(new_text, cx));
        let old_buffer = cx.new(|cx| Buffer::local(old_text.unwrap_or("".into()), cx));
        let new_buffer_snapshot = new_buffer.read(cx).text_snapshot();
        let old_buffer_snapshot = old_buffer.read(cx).snapshot();
        let buffer_diff = cx.new(|cx| BufferDiff::new(&new_buffer_snapshot, cx));
        let diff_task = buffer_diff.update(cx, |diff, cx| {
            diff.set_base_text(
                old_buffer_snapshot,
                Some(language_registry.clone()),
                new_buffer_snapshot,
                cx,
            )
        });

        let task = cx.spawn({
            let multibuffer = multibuffer.clone();
            let path = path.clone();
            async move |cx| {
                diff_task.await?;

                multibuffer
                    .update(cx, |multibuffer, cx| {
                        let hunk_ranges = {
                            let buffer = new_buffer.read(cx);
                            let diff = buffer_diff.read(cx);
                            diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &buffer, cx)
                                .map(|diff_hunk| diff_hunk.buffer_range.to_point(&buffer))
                                .collect::<Vec<_>>()
                        };

                        multibuffer.set_excerpts_for_path(
                            PathKey::for_buffer(&new_buffer, cx),
                            new_buffer.clone(),
                            hunk_ranges,
                            editor::DEFAULT_MULTIBUFFER_CONTEXT,
                            cx,
                        );
                        multibuffer.add_diff(buffer_diff.clone(), cx);
                    })
                    .log_err();

                if let Some(language) = language_registry
                    .language_for_file_path(&path)
                    .await
                    .log_err()
                {
                    new_buffer.update(cx, |buffer, cx| buffer.set_language(Some(language), cx))?;
                }

                anyhow::Ok(())
            }
        });

        Self {
            multibuffer,
            path,
            _task: task,
        }
    }
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

    pub fn push_assistant_chunk(
        &mut self,
        chunk: acp::AssistantMessageChunk,
        cx: &mut Context<Self>,
    ) {
        let entries_len = self.entries.len();
        if let Some(last_entry) = self.entries.last_mut()
            && let AgentThreadEntryContent::AssistantMessage(AssistantMessage { ref mut chunks }) =
                last_entry.content
        {
            cx.emit(AcpThreadEvent::EntryUpdated(entries_len - 1));

            match (chunks.last_mut(), &chunk) {
                (
                    Some(AssistantMessageChunk::Text { chunk: old_chunk }),
                    acp::AssistantMessageChunk::Text { chunk: new_chunk },
                )
                | (
                    Some(AssistantMessageChunk::Thought { chunk: old_chunk }),
                    acp::AssistantMessageChunk::Thought { chunk: new_chunk },
                ) => {
                    old_chunk.update(cx, |old_chunk, cx| {
                        old_chunk.append(&new_chunk, cx);
                    });
                }
                _ => {
                    chunks.push(AssistantMessageChunk::from_acp(
                        chunk,
                        self.project.read(cx).languages().clone(),
                        cx,
                    ));
                }
            }
        } else {
            let chunk = AssistantMessageChunk::from_acp(
                chunk,
                self.project.read(cx).languages().clone(),
                cx,
            );

            self.push_entry(
                AgentThreadEntryContent::AssistantMessage(AssistantMessage {
                    chunks: vec![chunk],
                }),
                cx,
            );
        }
    }

    pub fn request_tool_call(
        &mut self,
        label: String,
        icon: acp::Icon,
        content: Option<acp::ToolCallContent>,
        confirmation: acp::ToolCallConfirmation,
        cx: &mut Context<Self>,
    ) -> ToolCallRequest {
        let (tx, rx) = oneshot::channel();

        let status = ToolCallStatus::WaitingForConfirmation {
            confirmation: ToolCallConfirmation::from_acp(
                confirmation,
                self.project.read(cx).languages().clone(),
                cx,
            ),
            respond_tx: tx,
        };

        let id = self.insert_tool_call(label, status, icon, content, cx);
        ToolCallRequest { id, outcome: rx }
    }

    pub fn push_tool_call(
        &mut self,
        label: String,
        icon: acp::Icon,
        content: Option<acp::ToolCallContent>,
        cx: &mut Context<Self>,
    ) -> ToolCallId {
        let status = ToolCallStatus::Allowed {
            status: acp::ToolCallStatus::Running,
        };

        self.insert_tool_call(label, status, icon, content, cx)
    }

    fn insert_tool_call(
        &mut self,
        label: String,
        status: ToolCallStatus,
        icon: acp::Icon,
        content: Option<acp::ToolCallContent>,
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
                icon: acp_icon_to_ui_icon(icon),
                content: content
                    .map(|content| ToolCallContent::from_acp(content, language_registry, cx)),
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
            AgentThreadEntryContent::ToolCall(call) => {
                call.content = new_content.map(|new_content| {
                    ToolCallContent::from_acp(new_content, language_registry, cx)
                });

                match &mut call.status {
                    ToolCallStatus::Allowed { status } => {
                        *status = new_status;
                    }
                    ToolCallStatus::WaitingForConfirmation { .. } => {
                        anyhow::bail!("Tool call hasn't been authorized yet")
                    }
                    ToolCallStatus::Rejected => {
                        anyhow::bail!("Tool call was rejected and therefore can't be updated")
                    }
                }
            }
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
                AgentThreadEntryContent::UserMessage(_)
                | AgentThreadEntryContent::AssistantMessage(_) => {
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
        let chunk =
            UserMessageChunk::from_str(message, self.project.read(cx).languages().clone(), cx);
        let message = UserMessage {
            chunks: vec![chunk],
        };
        self.push_entry(AgentThreadEntryContent::UserMessage(message.clone()), cx);
        let acp_message = message.into_acp(cx);
        cx.spawn(async move |_, cx| {
            agent.send_message(id, acp_message, cx).await?;
            Ok(())
        })
    }
}

fn acp_icon_to_ui_icon(icon: acp::Icon) -> IconName {
    match icon {
        acp::Icon::FileSearch => IconName::FileSearch,
        acp::Icon::Folder => IconName::Folder,
        acp::Icon::Globe => IconName::Globe,
        acp::Icon::Hammer => IconName::Hammer,
        acp::Icon::LightBulb => IconName::LightBulb,
        acp::Icon::Pencil => IconName::Pencil,
        acp::Icon::Regex => IconName::Regex,
        acp::Icon::Terminal => IconName::Terminal,
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
    use gpui::TestAppContext;
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
        let server = gemini_acp_server(project.clone(), cx).await;
        let thread = server.create_thread(&mut cx.to_async()).await.unwrap();
        thread
            .update(cx, |thread, cx| thread.send("Hello from Zed!", cx))
            .await
            .unwrap();

        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.entries.len(), 2);
            assert!(matches!(
                thread.entries[0].content,
                AgentThreadEntryContent::UserMessage(_)
            ));
            assert!(matches!(
                thread.entries[1].content,
                AgentThreadEntryContent::AssistantMessage(_)
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
        let server = gemini_acp_server(project.clone(), cx).await;
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
        thread.read_with(cx, |thread, _cx| {
            assert!(matches!(
                &thread.entries()[2].content,
                AgentThreadEntryContent::ToolCall(ToolCall {
                    status: ToolCallStatus::Allowed { .. },
                    ..
                })
            ));

            assert!(matches!(
                thread.entries[3].content,
                AgentThreadEntryContent::AssistantMessage(_)
            ));
        });
    }

    #[gpui::test]
    async fn test_gemini_tool_call_with_confirmation(cx: &mut TestAppContext) {
        init_test(cx);

        cx.executor().allow_parking();

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [path!("/private/tmp").as_ref()], cx).await;
        let server = gemini_acp_server(project.clone(), cx).await;
        let thread = server.create_thread(&mut cx.to_async()).await.unwrap();
        let full_turn = thread.update(cx, |thread, cx| {
            thread.send(r#"Run `echo "Hello, world!"`"#, cx)
        });

        run_until_tool_call(&thread, cx).await;

        let tool_call_id = thread.read_with(cx, |thread, _cx| {
            let AgentThreadEntryContent::ToolCall(ToolCall {
                id,
                status:
                    ToolCallStatus::WaitingForConfirmation {
                        confirmation: ToolCallConfirmation::Execute { root_command, .. },
                        ..
                    },
                ..
            }) = &thread.entries()[2].content
            else {
                panic!();
            };

            assert_eq!(root_command, "echo");

            *id
        });

        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(tool_call_id, acp::ToolCallConfirmationOutcome::Allow, cx);

            assert!(matches!(
                &thread.entries()[2].content,
                AgentThreadEntryContent::ToolCall(ToolCall {
                    status: ToolCallStatus::Allowed { .. },
                    ..
                })
            ));
        });

        full_turn.await.unwrap();

        thread.read_with(cx, |thread, cx| {
            let AgentThreadEntryContent::ToolCall(ToolCall {
                content: Some(ToolCallContent::Markdown { markdown }),
                status: ToolCallStatus::Allowed { .. },
                ..
            }) = &thread.entries()[2].content
            else {
                panic!();
            };

            markdown.read_with(cx, |md, _cx| {
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

    pub async fn gemini_acp_server(
        project: Entity<Project>,
        cx: &mut TestAppContext,
    ) -> Arc<AcpServer> {
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
        let server = cx.update(|cx| AcpServer::stdio(child, project, cx));
        server.initialize().await.unwrap();
        server
    }
}
