use crate::{DbThread, DbThreadMetadata, ThreadsDatabase};
use acp_thread::MentionUri;
use agent_client_protocol as acp;
use anyhow::{Context as _, Result, anyhow};
use assistant_text_thread::{SavedTextThreadMetadata, TextThread};
use chrono::{DateTime, Utc};
use db::kvp::KEY_VALUE_STORE;
use gpui::{App, AsyncApp, Entity, SharedString, Task, prelude::*};
use itertools::Itertools;
use paths::text_threads_dir;
use project::Project;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, path::Path, rc::Rc, sync::Arc, time::Duration};
use ui::ElementId;
use util::ResultExt as _;

const MAX_RECENTLY_OPENED_ENTRIES: usize = 6;
const RECENTLY_OPENED_THREADS_KEY: &str = "recent-agent-threads";
const SAVE_RECENTLY_OPENED_ENTRIES_DEBOUNCE: Duration = Duration::from_millis(50);

const DEFAULT_TITLE: &SharedString = &SharedString::new_static("New Thread");

//todo: We should remove this function once we support loading all acp thread
pub fn load_agent_thread(
    session_id: acp::SessionId,
    history_store: Entity<HistoryStore>,
    project: Entity<Project>,
    cx: &mut App,
) -> Task<Result<Entity<crate::Thread>>> {
    use agent_servers::{AgentServer, AgentServerDelegate};

    let server = Rc::new(crate::NativeAgentServer::new(
        project.read(cx).fs().clone(),
        history_store,
    ));
    let delegate = AgentServerDelegate::new(
        project.read(cx).agent_server_store().clone(),
        project.clone(),
        None,
        None,
    );
    let connection = server.connect(None, delegate, cx);
    cx.spawn(async move |cx| {
        let (agent, _) = connection.await?;
        let agent = agent.downcast::<crate::NativeAgentConnection>().unwrap();
        cx.update(|cx| agent.load_thread(session_id, cx)).await
    })
}

#[derive(Clone, Debug)]
pub enum HistoryEntry {
    AcpThread(DbThreadMetadata),
    TextThread(SavedTextThreadMetadata),
}

impl HistoryEntry {
    pub fn updated_at(&self) -> DateTime<Utc> {
        match self {
            HistoryEntry::AcpThread(thread) => thread.updated_at,
            HistoryEntry::TextThread(text_thread) => text_thread.mtime.to_utc(),
        }
    }

    pub fn id(&self) -> HistoryEntryId {
        match self {
            HistoryEntry::AcpThread(thread) => HistoryEntryId::AcpThread(thread.id.clone()),
            HistoryEntry::TextThread(text_thread) => {
                HistoryEntryId::TextThread(text_thread.path.clone())
            }
        }
    }

    pub fn mention_uri(&self) -> MentionUri {
        match self {
            HistoryEntry::AcpThread(thread) => MentionUri::Thread {
                id: thread.id.clone(),
                name: thread.title.to_string(),
            },
            HistoryEntry::TextThread(text_thread) => MentionUri::TextThread {
                path: text_thread.path.as_ref().to_owned(),
                name: text_thread.title.to_string(),
            },
        }
    }

    pub fn title(&self) -> &SharedString {
        match self {
            HistoryEntry::AcpThread(thread) => {
                if thread.title.is_empty() {
                    DEFAULT_TITLE
                } else {
                    &thread.title
                }
            }
            HistoryEntry::TextThread(text_thread) => &text_thread.title,
        }
    }
}

/// Generic identifier for a history entry.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum HistoryEntryId {
    AcpThread(acp::SessionId),
    TextThread(Arc<Path>),
}

impl Into<ElementId> for HistoryEntryId {
    fn into(self) -> ElementId {
        match self {
            HistoryEntryId::AcpThread(session_id) => ElementId::Name(session_id.0.into()),
            HistoryEntryId::TextThread(path) => ElementId::Path(path),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum SerializedRecentOpen {
    AcpThread(String),
    TextThread(String),
}

pub struct HistoryStore {
    threads: Vec<DbThreadMetadata>,
    entries: Vec<HistoryEntry>,
    text_thread_store: Entity<assistant_text_thread::TextThreadStore>,
    recently_opened_entries: VecDeque<HistoryEntryId>,
    _subscriptions: Vec<gpui::Subscription>,
    _save_recently_opened_entries_task: Task<()>,
}

impl HistoryStore {
    pub fn new(
        text_thread_store: Entity<assistant_text_thread::TextThreadStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions =
            vec![cx.observe(&text_thread_store, |this, _, cx| this.update_entries(cx))];

        cx.spawn(async move |this, cx| {
            let entries = Self::load_recently_opened_entries(cx).await;
            this.update(cx, |this, cx| {
                if let Some(entries) = entries.log_err() {
                    this.recently_opened_entries = entries;
                }

                this.reload(cx);
            })
            .ok();
        })
        .detach();

        Self {
            text_thread_store,
            recently_opened_entries: VecDeque::default(),
            threads: Vec::default(),
            entries: Vec::default(),
            _subscriptions: subscriptions,
            _save_recently_opened_entries_task: Task::ready(()),
        }
    }

    pub fn thread_from_session_id(&self, session_id: &acp::SessionId) -> Option<&DbThreadMetadata> {
        self.threads.iter().find(|thread| &thread.id == session_id)
    }

    pub fn load_thread(
        &mut self,
        id: acp::SessionId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<DbThread>>> {
        let database_future = ThreadsDatabase::connect(cx);
        cx.background_spawn(async move {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.load_thread(id).await
        })
    }

    pub fn save_thread(
        &mut self,
        id: acp::SessionId,
        thread: crate::DbThread,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let database_future = ThreadsDatabase::connect(cx);
        cx.spawn(async move |this, cx| {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.save_thread(id, thread).await?;
            this.update(cx, |this, cx| this.reload(cx))
        })
    }

    pub fn delete_thread(
        &mut self,
        id: acp::SessionId,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let database_future = ThreadsDatabase::connect(cx);
        cx.spawn(async move |this, cx| {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.delete_thread(id.clone()).await?;
            this.update(cx, |this, cx| this.reload(cx))
        })
    }

    pub fn delete_threads(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let database_future = ThreadsDatabase::connect(cx);
        cx.spawn(async move |this, cx| {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            database.delete_threads().await?;
            this.update(cx, |this, cx| this.reload(cx))
        })
    }

    pub fn delete_text_thread(
        &mut self,
        path: Arc<Path>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.text_thread_store
            .update(cx, |store, cx| store.delete_local(path, cx))
    }

    pub fn load_text_thread(
        &self,
        path: Arc<Path>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<TextThread>>> {
        self.text_thread_store
            .update(cx, |store, cx| store.open_local(path, cx))
    }

    pub fn reload(&self, cx: &mut Context<Self>) {
        let database_connection = ThreadsDatabase::connect(cx);
        cx.spawn(async move |this, cx| {
            let database = database_connection.await;
            let threads = database.map_err(|err| anyhow!(err))?.list_threads().await?;
            this.update(cx, |this, cx| {
                if this.recently_opened_entries.len() < MAX_RECENTLY_OPENED_ENTRIES {
                    for thread in threads
                        .iter()
                        .take(MAX_RECENTLY_OPENED_ENTRIES - this.recently_opened_entries.len())
                        .rev()
                    {
                        this.push_recently_opened_entry(
                            HistoryEntryId::AcpThread(thread.id.clone()),
                            cx,
                        )
                    }
                }
                this.threads = threads;
                this.update_entries(cx);
            })
        })
        .detach_and_log_err(cx);
    }

    fn update_entries(&mut self, cx: &mut Context<Self>) {
        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_THREAD_HISTORY").is_ok() {
            return;
        }
        let mut history_entries = Vec::new();
        history_entries.extend(self.threads.iter().cloned().map(HistoryEntry::AcpThread));
        history_entries.extend(
            self.text_thread_store
                .read(cx)
                .unordered_text_threads()
                .cloned()
                .map(HistoryEntry::TextThread),
        );

        history_entries.sort_unstable_by_key(|entry| std::cmp::Reverse(entry.updated_at()));
        self.entries = history_entries;
        cx.notify()
    }

    pub fn is_empty(&self, _cx: &App) -> bool {
        self.entries.is_empty()
    }

    pub fn recently_opened_entries(&self, cx: &App) -> Vec<HistoryEntry> {
        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_THREAD_HISTORY").is_ok() {
            return Vec::new();
        }

        let thread_entries = self.threads.iter().flat_map(|thread| {
            self.recently_opened_entries
                .iter()
                .enumerate()
                .flat_map(|(index, entry)| match entry {
                    HistoryEntryId::AcpThread(id) if &thread.id == id => {
                        Some((index, HistoryEntry::AcpThread(thread.clone())))
                    }
                    _ => None,
                })
        });

        let context_entries = self
            .text_thread_store
            .read(cx)
            .unordered_text_threads()
            .flat_map(|text_thread| {
                self.recently_opened_entries
                    .iter()
                    .enumerate()
                    .flat_map(|(index, entry)| match entry {
                        HistoryEntryId::TextThread(path) if &text_thread.path == path => {
                            Some((index, HistoryEntry::TextThread(text_thread.clone())))
                        }
                        _ => None,
                    })
            });

        thread_entries
            .chain(context_entries)
            // optimization to halt iteration early
            .take(self.recently_opened_entries.len())
            .sorted_unstable_by_key(|(index, _)| *index)
            .map(|(_, entry)| entry)
            .collect()
    }

    fn save_recently_opened_entries(&mut self, cx: &mut Context<Self>) {
        let serialized_entries = self
            .recently_opened_entries
            .iter()
            .filter_map(|entry| match entry {
                HistoryEntryId::TextThread(path) => path.file_name().map(|file| {
                    SerializedRecentOpen::TextThread(file.to_string_lossy().into_owned())
                }),
                HistoryEntryId::AcpThread(id) => {
                    Some(SerializedRecentOpen::AcpThread(id.to_string()))
                }
            })
            .collect::<Vec<_>>();

        self._save_recently_opened_entries_task = cx.spawn(async move |_, cx| {
            let content = serde_json::to_string(&serialized_entries).unwrap();
            cx.background_executor()
                .timer(SAVE_RECENTLY_OPENED_ENTRIES_DEBOUNCE)
                .await;

            if cfg!(any(feature = "test-support", test)) {
                return;
            }
            KEY_VALUE_STORE
                .write_kvp(RECENTLY_OPENED_THREADS_KEY.to_owned(), content)
                .await
                .log_err();
        });
    }

    fn load_recently_opened_entries(cx: &AsyncApp) -> Task<Result<VecDeque<HistoryEntryId>>> {
        cx.background_spawn(async move {
            if cfg!(any(feature = "test-support", test)) {
                log::warn!("history store does not persist in tests");
                return Ok(VecDeque::new());
            }
            let json = KEY_VALUE_STORE
                .read_kvp(RECENTLY_OPENED_THREADS_KEY)?
                .unwrap_or("[]".to_string());
            let entries = serde_json::from_str::<Vec<SerializedRecentOpen>>(&json)
                .context("deserializing persisted agent panel navigation history")?
                .into_iter()
                .take(MAX_RECENTLY_OPENED_ENTRIES)
                .flat_map(|entry| match entry {
                    SerializedRecentOpen::AcpThread(id) => {
                        Some(HistoryEntryId::AcpThread(acp::SessionId::new(id.as_str())))
                    }
                    SerializedRecentOpen::TextThread(file_name) => Some(
                        HistoryEntryId::TextThread(text_threads_dir().join(file_name).into()),
                    ),
                })
                .collect();
            Ok(entries)
        })
    }

    pub fn push_recently_opened_entry(&mut self, entry: HistoryEntryId, cx: &mut Context<Self>) {
        self.recently_opened_entries
            .retain(|old_entry| old_entry != &entry);
        self.recently_opened_entries.push_front(entry);
        self.recently_opened_entries
            .truncate(MAX_RECENTLY_OPENED_ENTRIES);
        self.save_recently_opened_entries(cx);
    }

    pub fn remove_recently_opened_thread(&mut self, id: acp::SessionId, cx: &mut Context<Self>) {
        self.recently_opened_entries.retain(
            |entry| !matches!(entry, HistoryEntryId::AcpThread(thread_id) if thread_id == &id),
        );
        self.save_recently_opened_entries(cx);
    }

    pub fn replace_recently_opened_text_thread(
        &mut self,
        old_path: &Path,
        new_path: &Arc<Path>,
        cx: &mut Context<Self>,
    ) {
        for entry in &mut self.recently_opened_entries {
            match entry {
                HistoryEntryId::TextThread(path) if path.as_ref() == old_path => {
                    *entry = HistoryEntryId::TextThread(new_path.clone());
                    break;
                }
                _ => {}
            }
        }
        self.save_recently_opened_entries(cx);
    }

    pub fn remove_recently_opened_entry(&mut self, entry: &HistoryEntryId, cx: &mut Context<Self>) {
        self.recently_opened_entries
            .retain(|old_entry| old_entry != entry);
        self.save_recently_opened_entries(cx);
    }

    pub fn entries(&self) -> impl Iterator<Item = HistoryEntry> {
        self.entries.iter().cloned()
    }
}
