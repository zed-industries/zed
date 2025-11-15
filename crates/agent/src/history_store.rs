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
use std::{
    collections::{HashSet, VecDeque},
    path::Path,
    rc::Rc,
    sync::Arc,
    time::Duration,
};
use ui::ElementId;
use util::ResultExt as _;
use workspace::WorkspaceId;

const MAX_RECENTLY_OPENED_ENTRIES: usize = 6;
const MAX_TRACKED_HISTORY_ENTRIES: usize = 50;
const RECENTLY_OPENED_THREADS_KEY_PREFIX: &str = "recent-agent-threads";
const SAVE_RECENTLY_OPENED_ENTRIES_DEBOUNCE: Duration = Duration::from_millis(50);

const DEFAULT_TITLE: &SharedString = &SharedString::new_static("New Thread");

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HistoryScope {
    Global,
    Workspace { storage_key: String },
}

impl HistoryScope {
    pub fn global() -> Self {
        HistoryScope::Global
    }

    pub fn workspace_key(key: impl Into<String>) -> Self {
        HistoryScope::Workspace {
            storage_key: key.into(),
        }
    }

    pub fn workspace_id(id: WorkspaceId) -> Self {
        HistoryScope::Workspace {
            storage_key: format!("id-{}", i64::from(id)),
        }
    }

    fn storage_key(&self) -> Option<&str> {
        match self {
            HistoryScope::Global => None,
            HistoryScope::Workspace { storage_key } => Some(storage_key.as_str()),
        }
    }

    fn is_scoped(&self) -> bool {
        matches!(self, HistoryScope::Workspace { .. })
    }
}

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
        cx.update(|cx| agent.load_thread(session_id, cx))?.await
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

#[derive(Serialize, Deserialize, Debug, Clone)]
enum SerializedRecentOpen {
    AcpThread(String),
    TextThread(String),
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct PersistedHistoryPayload {
    #[serde(default)]
    recent: Vec<SerializedRecentOpen>,
    #[serde(default)]
    known: Vec<SerializedRecentOpen>,
}

struct PersistedHistoryData {
    recent: VecDeque<HistoryEntryId>,
    tracked: VecDeque<HistoryEntryId>,
}

pub struct HistoryStore {
    threads: Vec<DbThreadMetadata>,
    entries: Vec<HistoryEntry>,
    text_thread_store: Entity<assistant_text_thread::TextThreadStore>,
    recently_opened_entries: VecDeque<HistoryEntryId>,
    tracked_entries: VecDeque<HistoryEntryId>,
    scope: HistoryScope,
    _subscriptions: Vec<gpui::Subscription>,
    _save_recently_opened_entries_task: Task<()>,
}

impl HistoryStore {
    pub fn new(
        text_thread_store: Entity<assistant_text_thread::TextThreadStore>,
        scope: HistoryScope,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions =
            vec![cx.observe(&text_thread_store, |this, _, cx| this.update_entries(cx))];

        let scope_for_task = scope.clone();
        cx.spawn(async move |this, cx| {
            let persisted = Self::load_persisted_history(cx, scope_for_task).await;
            this.update(cx, |this, cx| {
                if let Some(persisted) = persisted.log_err() {
                    this.recently_opened_entries = persisted.recent;
                    this.tracked_entries = persisted.tracked;
                }

                this.reload(cx);
            })
            .ok();
        })
        .detach();

        Self {
            text_thread_store,
            recently_opened_entries: VecDeque::default(),
            tracked_entries: VecDeque::default(),
            threads: Vec::default(),
            entries: Vec::default(),
            scope,
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
        let database_future = ThreadsDatabase::connect(cx);
        cx.spawn(async move |this, cx| {
            let threads = database_future
                .await
                .map_err(|err| anyhow!(err))?
                .list_threads()
                .await?;

            this.update(cx, |this, cx| {
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
        let allowed_ids = if self.scope.is_scoped() {
            Some(
                self.tracked_entries
                    .iter()
                    .cloned()
                    .collect::<HashSet<HistoryEntryId>>(),
            )
        } else {
            None
        };
        let mut history_entries = Vec::new();
        history_entries.extend(self.threads.iter().filter_map(|thread| {
            if allowed_ids.as_ref().map_or(true, |ids| {
                ids.contains(&HistoryEntryId::AcpThread(thread.id.clone()))
            }) {
                Some(HistoryEntry::AcpThread(thread.clone()))
            } else {
                None
            }
        }));
        history_entries.extend(
            self.text_thread_store
                .read(cx)
                .unordered_text_threads()
                .cloned()
                .filter_map(|text_thread| {
                    if allowed_ids.as_ref().map_or(true, |ids| {
                        ids.contains(&HistoryEntryId::TextThread(text_thread.path.clone()))
                    }) {
                        Some(HistoryEntry::TextThread(text_thread))
                    } else {
                        None
                    }
                }),
        );

        history_entries.sort_unstable_by_key(|entry| std::cmp::Reverse(entry.updated_at()));
        self.entries = history_entries;
        cx.notify()
    }

    pub fn is_empty(&self, _cx: &App) -> bool {
        self.entries.is_empty()
    }

    pub fn recently_opened_entries(&self, cx: &App) -> Vec<HistoryEntry> {
        // Print how many entries we loaded
        println!(
            "DEBUG: Loaded {} recent entries for workspace",
            self.entries.len()
        );
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

    fn get_recently_opened_threads_key(&self) -> String {
        match self.scope.storage_key() {
            Some(key) => format!("{}-{}", RECENTLY_OPENED_THREADS_KEY_PREFIX, key),
            None => RECENTLY_OPENED_THREADS_KEY_PREFIX.to_string(),
        }
    }

    fn deserialize_entries(
        entries: Vec<SerializedRecentOpen>,
        limit: usize,
    ) -> VecDeque<HistoryEntryId> {
        let mut result = VecDeque::with_capacity(entries.len().min(limit));
        for entry in entries {
            if result.len() >= limit {
                break;
            }
            let history_entry = match entry {
                SerializedRecentOpen::AcpThread(id) => {
                    HistoryEntryId::AcpThread(acp::SessionId(id.as_str().into()))
                }
                SerializedRecentOpen::TextThread(file_name) => {
                    HistoryEntryId::TextThread(text_threads_dir().join(file_name).into())
                }
            };
            result.push_back(history_entry);
        }
        result
    }

    fn serialize_entries(entries: &VecDeque<HistoryEntryId>) -> Vec<SerializedRecentOpen> {
        entries
            .iter()
            .filter_map(|entry| match entry {
                HistoryEntryId::TextThread(path) => path.file_name().map(|file| {
                    SerializedRecentOpen::TextThread(file.to_string_lossy().into_owned())
                }),
                HistoryEntryId::AcpThread(id) => {
                    Some(SerializedRecentOpen::AcpThread(id.to_string()))
                }
            })
            .collect()
    }

    fn schedule_save_history(&mut self, cx: &mut Context<Self>) {
        let payload = PersistedHistoryPayload {
            recent: Self::serialize_entries(&self.recently_opened_entries),
            known: Self::serialize_entries(&self.tracked_entries),
        };

        let key = self.get_recently_opened_threads_key();
        println!("DEBUG: Saving recent threads with key: {}", key);

        self._save_recently_opened_entries_task = cx.spawn(async move |_, cx| {
            let content = serde_json::to_string(&payload).unwrap();
            cx.background_executor()
                .timer(SAVE_RECENTLY_OPENED_ENTRIES_DEBOUNCE)
                .await;

            if cfg!(any(feature = "test-support", test)) {
                return;
            }
            KEY_VALUE_STORE.write_kvp(key, content).await.log_err();
        });
    }

    fn load_persisted_history(
        cx: &AsyncApp,
        scope: HistoryScope,
    ) -> Task<Result<PersistedHistoryData>> {
        cx.background_spawn(async move {
            if cfg!(any(feature = "test-support", test)) {
                anyhow::bail!("history store does not persist in tests");
            }

            let (key, json, scoped) = match scope {
                HistoryScope::Global => {
                    let key = RECENTLY_OPENED_THREADS_KEY_PREFIX.to_string();
                    let json = KEY_VALUE_STORE
                        .read_kvp(&key)?
                        .unwrap_or_else(|| "[]".to_string());
                    (Some(key), json, false)
                }
                HistoryScope::Workspace { storage_key } => {
                    let key = format!("{}-{}", RECENTLY_OPENED_THREADS_KEY_PREFIX, storage_key);
                    println!("DEBUG: Loading recent threads for scoped key: {}", key);
                    let json = KEY_VALUE_STORE
                        .read_kvp(&key)?
                        .unwrap_or_else(|| "{}".to_string());
                    (Some(key), json, true)
                }
            };

            let payload = match serde_json::from_str::<PersistedHistoryPayload>(&json) {
                Ok(payload) => payload,
                Err(_) => {
                    // Support legacy payloads that stored only the recent list as a JSON array.
                    let legacy: Vec<SerializedRecentOpen> = serde_json::from_str(&json)
                        .context("deserializing persisted agent panel navigation history")?;
                    PersistedHistoryPayload {
                        recent: legacy.clone(),
                        known: legacy,
                    }
                }
            };

            let mut tracked = Self::deserialize_entries(payload.known, MAX_TRACKED_HISTORY_ENTRIES);
            if tracked.is_empty() {
                tracked =
                    Self::deserialize_entries(payload.recent.clone(), MAX_TRACKED_HISTORY_ENTRIES);
            }
            let recent = Self::deserialize_entries(payload.recent, MAX_RECENTLY_OPENED_ENTRIES);

            // Clean up legacy keys that may still use the list format.
            if let Some(key) = key {
                if scoped && !json.trim_start().starts_with('{') {
                    let payload = PersistedHistoryPayload {
                        recent: Self::serialize_entries(&recent),
                        known: Self::serialize_entries(&tracked),
                    };
                    let _ = KEY_VALUE_STORE.write_kvp(key, serde_json::to_string(&payload)?);
                }
            }

            Ok(PersistedHistoryData { recent, tracked })
        })
    }

    pub fn push_recently_opened_entry(&mut self, entry: HistoryEntryId, cx: &mut Context<Self>) {
        self.recently_opened_entries
            .retain(|old_entry| old_entry != &entry);
        self.tracked_entries.retain(|old_entry| old_entry != &entry);
        self.recently_opened_entries.push_front(entry.clone());
        self.recently_opened_entries
            .truncate(MAX_RECENTLY_OPENED_ENTRIES);
        self.tracked_entries.push_front(entry);
        self.tracked_entries.truncate(MAX_TRACKED_HISTORY_ENTRIES);
        self.schedule_save_history(cx);
    }

    pub fn remove_recently_opened_thread(&mut self, id: acp::SessionId, cx: &mut Context<Self>) {
        self.recently_opened_entries.retain(
            |entry| !matches!(entry, HistoryEntryId::AcpThread(thread_id) if thread_id == &id),
        );
        self.tracked_entries.retain(
            |entry| !matches!(entry, HistoryEntryId::AcpThread(thread_id) if thread_id == &id),
        );
        self.schedule_save_history(cx);
    }

    pub fn replace_recently_opened_text_thread(
        &mut self,
        old_path: &Path,
        new_path: &Arc<Path>,
        cx: &mut Context<Self>,
    ) {
        let replace_text_entry = |entry: &mut HistoryEntryId| match entry {
            HistoryEntryId::TextThread(path) if path.as_ref() == old_path => {
                *entry = HistoryEntryId::TextThread(new_path.clone());
                true
            }
            _ => false,
        };

        for entry in &mut self.recently_opened_entries {
            if replace_text_entry(entry) {
                break;
            }
        }

        for entry in &mut self.tracked_entries {
            replace_text_entry(entry);
        }
        self.schedule_save_history(cx);
    }

    pub fn remove_recently_opened_entry(&mut self, entry: &HistoryEntryId, cx: &mut Context<Self>) {
        self.recently_opened_entries
            .retain(|old_entry| old_entry != entry);
        self.tracked_entries.retain(|old_entry| old_entry != entry);
        self.schedule_save_history(cx);
    }

    pub fn entries(&self) -> impl Iterator<Item = HistoryEntry> {
        self.entries.iter().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent_client_protocol as acp;
    use agent_settings;
    use assistant_text_thread::TextThreadStore;
    use chrono::{Duration as ChronoDuration, Utc};
    use fs::FakeFs;
    use gpui::{SharedString, TestAppContext};
    use language;
    use language_model::LanguageModelRegistry;
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;

    fn init_history_store_test(cx: &mut TestAppContext) {
        env_logger::try_init().ok();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
            agent_settings::init(cx);
            language::init(cx);
            LanguageModelRegistry::test(cx);
        });
    }

    #[gpui::test]
    async fn filters_entries_per_workspace(cx: &mut TestAppContext) {
        init_history_store_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/", json!({})).await;
        let project = Project::test(fs.clone(), [], cx).await;

        let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| {
            HistoryStore::new(
                text_thread_store.clone(),
                HistoryScope::workspace_id(WorkspaceId::default()),
                cx,
            )
        });

        cx.run_until_parked();

        let thread_a = DbThreadMetadata {
            id: acp::SessionId("thread-a".into()),
            title: SharedString::from("Thread A"),
            updated_at: Utc::now(),
        };
        let thread_b = DbThreadMetadata {
            id: acp::SessionId("thread-b".into()),
            title: SharedString::from("Thread B"),
            updated_at: Utc::now() - ChronoDuration::minutes(5),
        };

        history_store.update(cx, |store, cx| {
            store.threads = vec![thread_a.clone(), thread_b.clone()];
            store.update_entries(cx);
        });

        let initial_entries =
            history_store.update(cx, |store, _| store.entries().collect::<Vec<_>>());
        assert!(
            initial_entries.is_empty(),
            "entries should be empty before tracking any thread"
        );

        history_store.update(cx, |store, cx| {
            store.push_recently_opened_entry(HistoryEntryId::AcpThread(thread_a.id.clone()), cx);
            store.update_entries(cx);
        });

        let filtered_entries =
            history_store.update(cx, |store, _| store.entries().collect::<Vec<_>>());

        assert_eq!(filtered_entries.len(), 1);
        match &filtered_entries[0] {
            HistoryEntry::AcpThread(metadata) => {
                assert_eq!(metadata.id, thread_a.id);
            }
            _ => panic!("expected acp thread entry"),
        }
    }
}
