use std::{path::Path, sync::Arc};

use acp_thread::AcpThreadEvent;
use agent::{ThreadStore, ZED_AGENT_ID};
use agent_client_protocol as acp;
use anyhow::Context as _;
use chrono::{DateTime, Utc};
use collections::{HashMap, HashSet};
use db::{
    sqlez::{
        bindable::Column, domain::Domain, statement::Statement,
        thread_safe_connection::ThreadSafeConnection,
    },
    sqlez_macros::sql,
};
use feature_flags::{AgentV2FeatureFlag, FeatureFlagAppExt};
use futures::{FutureExt as _, future::Shared};
use gpui::{AppContext as _, Entity, Global, Subscription, Task};
use project::AgentId;
use ui::{App, Context, SharedString};
use util::ResultExt as _;
use workspace::PathList;

use crate::DEFAULT_THREAD_TITLE;

pub fn init(cx: &mut App) {
    ThreadMetadataStore::init_global(cx);

    if cx.has_flag::<AgentV2FeatureFlag>() {
        migrate_thread_metadata(cx);
    }
    cx.observe_flag::<AgentV2FeatureFlag, _>(|has_flag, cx| {
        if has_flag {
            migrate_thread_metadata(cx);
        }
    })
    .detach();
}

/// Migrate existing thread metadata from native agent thread store to the new metadata storage.
/// We skip migrating threads that do not have a project.
///
/// TODO: Remove this after N weeks of shipping the sidebar
fn migrate_thread_metadata(cx: &mut App) {
    let store = ThreadMetadataStore::global(cx);
    let db = store.read(cx).db.clone();

    cx.spawn(async move |cx| {
        let existing_entries = db.list_ids()?.into_iter().collect::<HashSet<_>>();

        let is_first_migration = existing_entries.is_empty();

        let mut to_migrate = store.read_with(cx, |_store, cx| {
            ThreadStore::global(cx)
                .read(cx)
                .entries()
                .filter_map(|entry| {
                    if existing_entries.contains(&entry.id.0) || entry.folder_paths.is_empty() {
                        return None;
                    }

                    Some(ThreadMetadata {
                        session_id: entry.id,
                        agent_id: ZED_AGENT_ID.clone(),
                        title: entry.title,
                        updated_at: entry.updated_at,
                        created_at: entry.created_at,
                        folder_paths: entry.folder_paths,
                        archived: true,
                    })
                })
                .collect::<Vec<_>>()
        });

        if to_migrate.is_empty() {
            return anyhow::Ok(());
        }

        // On the first migration (no entries in DB yet), keep the 5 most
        // recent threads per project unarchived.
        if is_first_migration {
            let mut per_project: HashMap<PathList, Vec<&mut ThreadMetadata>> = HashMap::default();
            for entry in &mut to_migrate {
                per_project
                    .entry(entry.folder_paths.clone())
                    .or_default()
                    .push(entry);
            }
            for entries in per_project.values_mut() {
                entries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
                for entry in entries.iter_mut().take(5) {
                    entry.archived = false;
                }
            }
        }

        log::info!("Migrating {} thread store entries", to_migrate.len());

        // Manually save each entry to the database and call reload, otherwise
        // we'll end up triggering lots of reloads after each save
        for entry in to_migrate {
            db.save(entry).await?;
        }

        log::info!("Finished migrating thread store entries");

        let _ = store.update(cx, |store, cx| store.reload(cx));
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

struct GlobalThreadMetadataStore(Entity<ThreadMetadataStore>);
impl Global for GlobalThreadMetadataStore {}

/// Lightweight metadata for any thread (native or ACP), enough to populate
/// the sidebar list and route to the correct load path when clicked.
#[derive(Debug, Clone, PartialEq)]
pub struct ThreadMetadata {
    pub session_id: acp::SessionId,
    pub agent_id: AgentId,
    pub title: SharedString,
    pub updated_at: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
    pub folder_paths: PathList,
    pub archived: bool,
}

impl From<&ThreadMetadata> for acp_thread::AgentSessionInfo {
    fn from(meta: &ThreadMetadata) -> Self {
        Self {
            session_id: meta.session_id.clone(),
            work_dirs: Some(meta.folder_paths.clone()),
            title: Some(meta.title.clone()),
            updated_at: Some(meta.updated_at),
            created_at: meta.created_at,
            meta: None,
        }
    }
}

/// The store holds all metadata needed to show threads in the sidebar/the archive.
///
/// Automatically listens to AcpThread events and updates metadata if it has changed.
pub struct ThreadMetadataStore {
    db: ThreadMetadataDb,
    threads: HashMap<acp::SessionId, ThreadMetadata>,
    threads_by_paths: HashMap<PathList, HashSet<acp::SessionId>>,
    reload_task: Option<Shared<Task<()>>>,
    session_subscriptions: HashMap<acp::SessionId, Subscription>,
    pending_thread_ops_tx: smol::channel::Sender<DbOperation>,
    _db_operations_task: Task<()>,
}

#[derive(Debug, PartialEq)]
enum DbOperation {
    Upsert(ThreadMetadata),
    Delete(acp::SessionId),
}

impl DbOperation {
    fn id(&self) -> &acp::SessionId {
        match self {
            DbOperation::Upsert(thread) => &thread.session_id,
            DbOperation::Delete(session_id) => session_id,
        }
    }
}

impl ThreadMetadataStore {
    #[cfg(not(any(test, feature = "test-support")))]
    pub fn init_global(cx: &mut App) {
        if cx.has_global::<Self>() {
            return;
        }

        let db = ThreadMetadataDb::global(cx);
        let thread_store = cx.new(|cx| Self::new(db, cx));
        cx.set_global(GlobalThreadMetadataStore(thread_store));
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn init_global(cx: &mut App) {
        let thread = std::thread::current();
        let test_name = thread.name().unwrap_or("unknown_test");
        let db_name = format!("THREAD_METADATA_DB_{}", test_name);
        let db = smol::block_on(db::open_test_db::<ThreadMetadataDb>(&db_name));
        let thread_store = cx.new(|cx| Self::new(ThreadMetadataDb(db), cx));
        cx.set_global(GlobalThreadMetadataStore(thread_store));
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalThreadMetadataStore>()
            .map(|store| store.0.clone())
    }

    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalThreadMetadataStore>().0.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.threads.is_empty()
    }

    /// Returns all thread IDs.
    pub fn entry_ids(&self) -> impl Iterator<Item = acp::SessionId> + '_ {
        self.threads.keys().cloned()
    }

    /// Returns the metadata for a specific thread, if it exists.
    pub fn entry(&self, session_id: &acp::SessionId) -> Option<&ThreadMetadata> {
        self.threads.get(session_id)
    }

    /// Returns all threads.
    pub fn entries(&self) -> impl Iterator<Item = &ThreadMetadata> + '_ {
        self.threads.values()
    }

    /// Returns all archived threads.
    pub fn archived_entries(&self) -> impl Iterator<Item = &ThreadMetadata> + '_ {
        self.entries().filter(|t| t.archived)
    }

    /// Returns all threads for the given path list, excluding archived threads.
    pub fn entries_for_path(
        &self,
        path_list: &PathList,
    ) -> impl Iterator<Item = &ThreadMetadata> + '_ {
        self.threads_by_paths
            .get(path_list)
            .into_iter()
            .flatten()
            .filter_map(|s| self.threads.get(s))
            .filter(|s| !s.archived)
    }

    fn reload(&mut self, cx: &mut Context<Self>) -> Shared<Task<()>> {
        let db = self.db.clone();
        self.reload_task.take();

        let list_task = cx
            .background_spawn(async move { db.list().context("Failed to fetch sidebar metadata") });

        let reload_task = cx
            .spawn(async move |this, cx| {
                let Some(rows) = list_task.await.log_err() else {
                    return;
                };

                this.update(cx, |this, cx| {
                    this.threads.clear();
                    this.threads_by_paths.clear();

                    for row in rows {
                        this.threads_by_paths
                            .entry(row.folder_paths.clone())
                            .or_default()
                            .insert(row.session_id.clone());
                        this.threads.insert(row.session_id.clone(), row);
                    }

                    cx.notify();
                })
                .ok();
            })
            .shared();
        self.reload_task = Some(reload_task.clone());
        reload_task
    }

    pub fn save_all(&mut self, metadata: Vec<ThreadMetadata>, cx: &mut Context<Self>) {
        if !cx.has_flag::<AgentV2FeatureFlag>() {
            return;
        }

        for metadata in metadata {
            self.save_internal(metadata);
        }
        cx.notify();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn save_manually(&mut self, metadata: ThreadMetadata, cx: &mut Context<Self>) {
        self.save(metadata, cx)
    }

    fn save(&mut self, metadata: ThreadMetadata, cx: &mut Context<Self>) {
        if !cx.has_flag::<AgentV2FeatureFlag>() {
            return;
        }

        self.save_internal(metadata);
        cx.notify();
    }

    fn save_internal(&mut self, metadata: ThreadMetadata) {
        // If the folder paths have changed, we need to clear the old entry
        if let Some(thread) = self.threads.get(&metadata.session_id)
            && thread.folder_paths != metadata.folder_paths
            && let Some(session_ids) = self.threads_by_paths.get_mut(&thread.folder_paths)
        {
            session_ids.remove(&metadata.session_id);
        }

        self.threads
            .insert(metadata.session_id.clone(), metadata.clone());

        self.threads_by_paths
            .entry(metadata.folder_paths.clone())
            .or_default()
            .insert(metadata.session_id.clone());

        self.pending_thread_ops_tx
            .try_send(DbOperation::Upsert(metadata))
            .log_err();
    }

    pub fn archive(&mut self, session_id: &acp::SessionId, cx: &mut Context<Self>) {
        self.update_archived(session_id, true, cx);
    }

    pub fn unarchive(&mut self, session_id: &acp::SessionId, cx: &mut Context<Self>) {
        self.update_archived(session_id, false, cx);
    }

    fn update_archived(
        &mut self,
        session_id: &acp::SessionId,
        archived: bool,
        cx: &mut Context<Self>,
    ) {
        if !cx.has_flag::<AgentV2FeatureFlag>() {
            return;
        }

        if let Some(thread) = self.threads.get(session_id) {
            self.save_internal(ThreadMetadata {
                archived,
                ..thread.clone()
            });
            cx.notify();
        }
    }

    pub fn delete(&mut self, session_id: acp::SessionId, cx: &mut Context<Self>) {
        if !cx.has_flag::<AgentV2FeatureFlag>() {
            return;
        }

        if let Some(thread) = self.threads.get(&session_id)
            && let Some(session_ids) = self.threads_by_paths.get_mut(&thread.folder_paths)
        {
            session_ids.remove(&session_id);
        }
        self.threads.remove(&session_id);
        self.pending_thread_ops_tx
            .try_send(DbOperation::Delete(session_id))
            .log_err();
        cx.notify();
    }

    fn new(db: ThreadMetadataDb, cx: &mut Context<Self>) -> Self {
        let weak_store = cx.weak_entity();

        cx.observe_new::<acp_thread::AcpThread>(move |thread, _window, cx| {
            // Don't track subagent threads in the sidebar.
            if thread.parent_session_id().is_some() {
                return;
            }

            let thread_entity = cx.entity();

            cx.on_release({
                let weak_store = weak_store.clone();
                move |thread, cx| {
                    weak_store
                        .update(cx, |store, cx| {
                            let session_id = thread.session_id().clone();
                            store.session_subscriptions.remove(&session_id);
                            if thread.entries().is_empty() {
                                // Empty threads can be unloaded without ever being
                                // durably persisted by the underlying agent.
                                store.delete(session_id, cx);
                            }
                        })
                        .ok();
                }
            })
            .detach();

            weak_store
                .update(cx, |this, cx| {
                    let subscription = cx.subscribe(&thread_entity, Self::handle_thread_event);
                    this.session_subscriptions
                        .insert(thread.session_id().clone(), subscription);
                })
                .ok();
        })
        .detach();

        let (tx, rx) = smol::channel::unbounded();
        let _db_operations_task = cx.background_spawn({
            let db = db.clone();
            async move {
                while let Ok(first_update) = rx.recv().await {
                    let mut updates = vec![first_update];
                    while let Ok(update) = rx.try_recv() {
                        updates.push(update);
                    }
                    let updates = Self::dedup_db_operations(updates);
                    for operation in updates {
                        match operation {
                            DbOperation::Upsert(metadata) => {
                                db.save(metadata).await.log_err();
                            }
                            DbOperation::Delete(session_id) => {
                                db.delete(session_id).await.log_err();
                            }
                        }
                    }
                }
            }
        });

        let mut this = Self {
            db,
            threads: HashMap::default(),
            threads_by_paths: HashMap::default(),
            reload_task: None,
            session_subscriptions: HashMap::default(),
            pending_thread_ops_tx: tx,
            _db_operations_task,
        };
        let _ = this.reload(cx);
        this
    }

    fn dedup_db_operations(operations: Vec<DbOperation>) -> Vec<DbOperation> {
        let mut ops = HashMap::default();
        for operation in operations.into_iter().rev() {
            if ops.contains_key(operation.id()) {
                continue;
            }
            ops.insert(operation.id().clone(), operation);
        }
        ops.into_values().collect()
    }

    fn handle_thread_event(
        &mut self,
        thread: Entity<acp_thread::AcpThread>,
        event: &AcpThreadEvent,
        cx: &mut Context<Self>,
    ) {
        // Don't track subagent threads in the sidebar.
        if thread.read(cx).parent_session_id().is_some() {
            return;
        }

        match event {
            AcpThreadEvent::NewEntry
            | AcpThreadEvent::TitleUpdated
            | AcpThreadEvent::EntryUpdated(_)
            | AcpThreadEvent::EntriesRemoved(_)
            | AcpThreadEvent::ToolAuthorizationRequested(_)
            | AcpThreadEvent::ToolAuthorizationReceived(_)
            | AcpThreadEvent::Retry(_)
            | AcpThreadEvent::Stopped(_)
            | AcpThreadEvent::Error
            | AcpThreadEvent::LoadError(_)
            | AcpThreadEvent::Refusal
            | AcpThreadEvent::WorkingDirectoriesUpdated => {
                let thread_ref = thread.read(cx);
                let existing_thread = self.threads.get(thread_ref.session_id());
                let session_id = thread_ref.session_id().clone();
                let title = thread_ref
                    .title()
                    .unwrap_or_else(|| DEFAULT_THREAD_TITLE.into());

                let updated_at = Utc::now();

                let created_at = existing_thread
                    .and_then(|t| t.created_at)
                    .unwrap_or_else(|| updated_at);

                let agent_id = thread_ref.connection().agent_id();

                let folder_paths = {
                    let project = thread_ref.project().read(cx);
                    let paths: Vec<Arc<Path>> = project
                        .visible_worktrees(cx)
                        .map(|worktree| worktree.read(cx).abs_path())
                        .collect();
                    PathList::new(&paths)
                };

                let archived = existing_thread.map(|t| t.archived).unwrap_or(false);

                let metadata = ThreadMetadata {
                    session_id,
                    agent_id,
                    title,
                    created_at: Some(created_at),
                    updated_at,
                    folder_paths,
                    archived,
                };

                self.save(metadata, cx);
            }
            AcpThreadEvent::TokenUsageUpdated
            | AcpThreadEvent::SubagentSpawned(_)
            | AcpThreadEvent::PromptCapabilitiesUpdated
            | AcpThreadEvent::AvailableCommandsUpdated(_)
            | AcpThreadEvent::ModeUpdated(_)
            | AcpThreadEvent::ConfigOptionsUpdated(_) => {}
        }
    }
}

impl Global for ThreadMetadataStore {}

struct ThreadMetadataDb(ThreadSafeConnection);

impl Domain for ThreadMetadataDb {
    const NAME: &str = stringify!(ThreadMetadataDb);

    const MIGRATIONS: &[&str] = &[
        sql!(
            CREATE TABLE IF NOT EXISTS sidebar_threads(
                session_id TEXT PRIMARY KEY,
                agent_id TEXT,
                title TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                created_at TEXT,
                folder_paths TEXT,
                folder_paths_order TEXT
            ) STRICT;
        ),
        sql!(ALTER TABLE sidebar_threads ADD COLUMN archived INTEGER DEFAULT 0),
    ];
}

db::static_connection!(ThreadMetadataDb, []);

impl ThreadMetadataDb {
    pub fn list_ids(&self) -> anyhow::Result<Vec<Arc<str>>> {
        self.select::<Arc<str>>(
            "SELECT session_id FROM sidebar_threads \
             ORDER BY updated_at DESC",
        )?()
    }

    /// List all sidebar thread metadata, ordered by updated_at descending.
    pub fn list(&self) -> anyhow::Result<Vec<ThreadMetadata>> {
        self.select::<ThreadMetadata>(
            "SELECT session_id, agent_id, title, updated_at, created_at, folder_paths, folder_paths_order, archived \
             FROM sidebar_threads \
             ORDER BY updated_at DESC"
        )?()
    }

    /// Upsert metadata for a thread.
    pub async fn save(&self, row: ThreadMetadata) -> anyhow::Result<()> {
        let id = row.session_id.0.clone();
        let agent_id = if row.agent_id.as_ref() == ZED_AGENT_ID.as_ref() {
            None
        } else {
            Some(row.agent_id.to_string())
        };
        let title = row.title.to_string();
        let updated_at = row.updated_at.to_rfc3339();
        let created_at = row.created_at.map(|dt| dt.to_rfc3339());
        let serialized = row.folder_paths.serialize();
        let (folder_paths, folder_paths_order) = if row.folder_paths.is_empty() {
            (None, None)
        } else {
            (Some(serialized.paths), Some(serialized.order))
        };
        let archived = row.archived;

        self.write(move |conn| {
            let sql = "INSERT INTO sidebar_threads(session_id, agent_id, title, updated_at, created_at, folder_paths, folder_paths_order, archived) \
                       VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8) \
                       ON CONFLICT(session_id) DO UPDATE SET \
                           agent_id = excluded.agent_id, \
                           title = excluded.title, \
                           updated_at = excluded.updated_at, \
                           created_at = excluded.created_at, \
                           folder_paths = excluded.folder_paths, \
                           folder_paths_order = excluded.folder_paths_order, \
                           archived = excluded.archived";
            let mut stmt = Statement::prepare(conn, sql)?;
            let mut i = stmt.bind(&id, 1)?;
            i = stmt.bind(&agent_id, i)?;
            i = stmt.bind(&title, i)?;
            i = stmt.bind(&updated_at, i)?;
            i = stmt.bind(&created_at, i)?;
            i = stmt.bind(&folder_paths, i)?;
            i = stmt.bind(&folder_paths_order, i)?;
            stmt.bind(&archived, i)?;
            stmt.exec()
        })
        .await
    }

    /// Delete metadata for a single thread.
    pub async fn delete(&self, session_id: acp::SessionId) -> anyhow::Result<()> {
        let id = session_id.0.clone();
        self.write(move |conn| {
            let mut stmt =
                Statement::prepare(conn, "DELETE FROM sidebar_threads WHERE session_id = ?")?;
            stmt.bind(&id, 1)?;
            stmt.exec()
        })
        .await
    }
}

impl Column for ThreadMetadata {
    fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
        let (id, next): (Arc<str>, i32) = Column::column(statement, start_index)?;
        let (agent_id, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (title, next): (String, i32) = Column::column(statement, next)?;
        let (updated_at_str, next): (String, i32) = Column::column(statement, next)?;
        let (created_at_str, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (folder_paths_str, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (folder_paths_order_str, next): (Option<String>, i32) =
            Column::column(statement, next)?;
        let (archived, next): (bool, i32) = Column::column(statement, next)?;

        let agent_id = agent_id
            .map(|id| AgentId::new(id))
            .unwrap_or(ZED_AGENT_ID.clone());

        let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc);
        let created_at = created_at_str
            .as_deref()
            .map(DateTime::parse_from_rfc3339)
            .transpose()?
            .map(|dt| dt.with_timezone(&Utc));

        let folder_paths = folder_paths_str
            .map(|paths| {
                PathList::deserialize(&util::path_list::SerializedPathList {
                    paths,
                    order: folder_paths_order_str.unwrap_or_default(),
                })
            })
            .unwrap_or_default();

        Ok((
            ThreadMetadata {
                session_id: acp::SessionId::new(id),
                agent_id,
                title: title.into(),
                updated_at,
                created_at,
                folder_paths,
                archived,
            },
            next,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use acp_thread::{AgentConnection, StubAgentConnection};
    use action_log::ActionLog;
    use agent::DbThread;
    use agent_client_protocol as acp;
    use feature_flags::FeatureFlagAppExt;
    use gpui::TestAppContext;
    use project::FakeFs;
    use project::Project;
    use std::path::Path;
    use std::rc::Rc;

    fn make_db_thread(title: &str, updated_at: DateTime<Utc>) -> DbThread {
        DbThread {
            title: title.to_string().into(),
            messages: Vec::new(),
            updated_at,
            detailed_summary: None,
            initial_project_snapshot: None,
            cumulative_token_usage: Default::default(),
            request_token_usage: Default::default(),
            model: None,
            profile: None,
            imported: false,
            subagent_context: None,
            speed: None,
            thinking_enabled: false,
            thinking_effort: None,
            draft_prompt: None,
            ui_scroll_position: None,
        }
    }

    fn make_metadata(
        session_id: &str,
        title: &str,
        updated_at: DateTime<Utc>,
        folder_paths: PathList,
    ) -> ThreadMetadata {
        ThreadMetadata {
            archived: false,
            session_id: acp::SessionId::new(session_id),
            agent_id: agent::ZED_AGENT_ID.clone(),
            title: title.to_string().into(),
            updated_at,
            created_at: Some(updated_at),
            folder_paths,
        }
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            cx.update_flags(true, vec!["agent-v2".to_string()]);
            ThreadMetadataStore::init_global(cx);
            ThreadStore::init_global(cx);
        });
        cx.run_until_parked();
    }

    #[gpui::test]
    async fn test_store_initializes_cache_from_database(cx: &mut TestAppContext) {
        let first_paths = PathList::new(&[Path::new("/project-a")]);
        let second_paths = PathList::new(&[Path::new("/project-b")]);
        let now = Utc::now();
        let older = now - chrono::Duration::seconds(1);

        let thread = std::thread::current();
        let test_name = thread.name().unwrap_or("unknown_test");
        let db_name = format!("THREAD_METADATA_DB_{}", test_name);
        let db = ThreadMetadataDb(smol::block_on(db::open_test_db::<ThreadMetadataDb>(
            &db_name,
        )));

        db.save(make_metadata(
            "session-1",
            "First Thread",
            now,
            first_paths.clone(),
        ))
        .await
        .unwrap();
        db.save(make_metadata(
            "session-2",
            "Second Thread",
            older,
            second_paths.clone(),
        ))
        .await
        .unwrap();

        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            cx.update_flags(true, vec!["agent-v2".to_string()]);
            ThreadMetadataStore::init_global(cx);
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let entry_ids = store
                .entry_ids()
                .map(|session_id| session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(entry_ids.len(), 2);
            assert!(entry_ids.contains(&"session-1".to_string()));
            assert!(entry_ids.contains(&"session-2".to_string()));

            let first_path_entries = store
                .entries_for_path(&first_paths)
                .map(|entry| entry.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(first_path_entries, vec!["session-1"]);

            let second_path_entries = store
                .entries_for_path(&second_paths)
                .map(|entry| entry.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(second_path_entries, vec!["session-2"]);
        });
    }

    #[gpui::test]
    async fn test_store_cache_updates_after_save_and_delete(cx: &mut TestAppContext) {
        init_test(cx);

        let first_paths = PathList::new(&[Path::new("/project-a")]);
        let second_paths = PathList::new(&[Path::new("/project-b")]);
        let initial_time = Utc::now();
        let updated_time = initial_time + chrono::Duration::seconds(1);

        let initial_metadata = make_metadata(
            "session-1",
            "First Thread",
            initial_time,
            first_paths.clone(),
        );

        let second_metadata = make_metadata(
            "session-2",
            "Second Thread",
            initial_time,
            second_paths.clone(),
        );

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.save(initial_metadata, cx);
                store.save(second_metadata, cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let first_path_entries = store
                .entries_for_path(&first_paths)
                .map(|entry| entry.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(first_path_entries, vec!["session-1"]);

            let second_path_entries = store
                .entries_for_path(&second_paths)
                .map(|entry| entry.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(second_path_entries, vec!["session-2"]);
        });

        let moved_metadata = make_metadata(
            "session-1",
            "First Thread",
            updated_time,
            second_paths.clone(),
        );

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.save(moved_metadata, cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let entry_ids = store
                .entry_ids()
                .map(|session_id| session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(entry_ids.len(), 2);
            assert!(entry_ids.contains(&"session-1".to_string()));
            assert!(entry_ids.contains(&"session-2".to_string()));

            let first_path_entries = store
                .entries_for_path(&first_paths)
                .map(|entry| entry.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert!(first_path_entries.is_empty());

            let second_path_entries = store
                .entries_for_path(&second_paths)
                .map(|entry| entry.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(second_path_entries.len(), 2);
            assert!(second_path_entries.contains(&"session-1".to_string()));
            assert!(second_path_entries.contains(&"session-2".to_string()));
        });

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.delete(acp::SessionId::new("session-2"), cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let entry_ids = store
                .entry_ids()
                .map(|session_id| session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(entry_ids, vec!["session-1"]);

            let second_path_entries = store
                .entries_for_path(&second_paths)
                .map(|entry| entry.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(second_path_entries, vec!["session-1"]);
        });
    }

    #[gpui::test]
    async fn test_migrate_thread_metadata_migrates_only_missing_threads(cx: &mut TestAppContext) {
        init_test(cx);

        let project_a_paths = PathList::new(&[Path::new("/project-a")]);
        let project_b_paths = PathList::new(&[Path::new("/project-b")]);
        let now = Utc::now();

        let existing_metadata = ThreadMetadata {
            session_id: acp::SessionId::new("a-session-0"),
            agent_id: agent::ZED_AGENT_ID.clone(),
            title: "Existing Metadata".into(),
            updated_at: now - chrono::Duration::seconds(10),
            created_at: Some(now - chrono::Duration::seconds(10)),
            folder_paths: project_a_paths.clone(),
            archived: false,
        };

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.save(existing_metadata, cx);
            });
        });
        cx.run_until_parked();

        let threads_to_save = vec![
            (
                "a-session-0",
                "Thread A0 From Native Store",
                project_a_paths.clone(),
                now,
            ),
            (
                "a-session-1",
                "Thread A1",
                project_a_paths.clone(),
                now + chrono::Duration::seconds(1),
            ),
            (
                "b-session-0",
                "Thread B0",
                project_b_paths.clone(),
                now + chrono::Duration::seconds(2),
            ),
            (
                "projectless",
                "Projectless",
                PathList::default(),
                now + chrono::Duration::seconds(3),
            ),
        ];

        for (session_id, title, paths, updated_at) in &threads_to_save {
            let save_task = cx.update(|cx| {
                let thread_store = ThreadStore::global(cx);
                let session_id = session_id.to_string();
                let title = title.to_string();
                let paths = paths.clone();
                thread_store.update(cx, |store, cx| {
                    store.save_thread(
                        acp::SessionId::new(session_id),
                        make_db_thread(&title, *updated_at),
                        paths,
                        cx,
                    )
                })
            });
            save_task.await.unwrap();
            cx.run_until_parked();
        }

        cx.update(|cx| migrate_thread_metadata(cx));
        cx.run_until_parked();

        let list = cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.read(cx).entries().cloned().collect::<Vec<_>>()
        });

        assert_eq!(list.len(), 3);
        assert!(
            list.iter()
                .all(|metadata| metadata.agent_id.as_ref() == agent::ZED_AGENT_ID.as_ref())
        );

        let existing_metadata = list
            .iter()
            .find(|metadata| metadata.session_id.0.as_ref() == "a-session-0")
            .unwrap();
        assert_eq!(existing_metadata.title.as_ref(), "Existing Metadata");
        assert!(!existing_metadata.archived);

        let migrated_session_ids = list
            .iter()
            .map(|metadata| metadata.session_id.0.as_ref())
            .collect::<Vec<_>>();
        assert!(migrated_session_ids.contains(&"a-session-1"));
        assert!(migrated_session_ids.contains(&"b-session-0"));
        assert!(!migrated_session_ids.contains(&"projectless"));

        let migrated_entries = list
            .iter()
            .filter(|metadata| metadata.session_id.0.as_ref() != "a-session-0")
            .collect::<Vec<_>>();
        assert!(
            migrated_entries
                .iter()
                .all(|metadata| !metadata.folder_paths.is_empty())
        );
        assert!(migrated_entries.iter().all(|metadata| metadata.archived));
    }

    #[gpui::test]
    async fn test_migrate_thread_metadata_noops_when_all_threads_already_exist(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let project_paths = PathList::new(&[Path::new("/project-a")]);
        let existing_updated_at = Utc::now();

        let existing_metadata = ThreadMetadata {
            session_id: acp::SessionId::new("existing-session"),
            agent_id: agent::ZED_AGENT_ID.clone(),
            title: "Existing Metadata".into(),
            updated_at: existing_updated_at,
            created_at: Some(existing_updated_at),
            folder_paths: project_paths.clone(),
            archived: false,
        };

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.save(existing_metadata, cx);
            });
        });
        cx.run_until_parked();

        let save_task = cx.update(|cx| {
            let thread_store = ThreadStore::global(cx);
            thread_store.update(cx, |store, cx| {
                store.save_thread(
                    acp::SessionId::new("existing-session"),
                    make_db_thread(
                        "Updated Native Thread Title",
                        existing_updated_at + chrono::Duration::seconds(1),
                    ),
                    project_paths.clone(),
                    cx,
                )
            })
        });
        save_task.await.unwrap();
        cx.run_until_parked();

        cx.update(|cx| migrate_thread_metadata(cx));
        cx.run_until_parked();

        let list = cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.read(cx).entries().cloned().collect::<Vec<_>>()
        });

        assert_eq!(list.len(), 1);
        assert_eq!(list[0].session_id.0.as_ref(), "existing-session");
    }

    #[gpui::test]
    async fn test_migrate_thread_metadata_archives_beyond_five_most_recent_per_project(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let project_a_paths = PathList::new(&[Path::new("/project-a")]);
        let project_b_paths = PathList::new(&[Path::new("/project-b")]);
        let now = Utc::now();

        // Create 7 threads for project A and 3 for project B
        let mut threads_to_save = Vec::new();
        for i in 0..7 {
            threads_to_save.push((
                format!("a-session-{i}"),
                format!("Thread A{i}"),
                project_a_paths.clone(),
                now + chrono::Duration::seconds(i as i64),
            ));
        }
        for i in 0..3 {
            threads_to_save.push((
                format!("b-session-{i}"),
                format!("Thread B{i}"),
                project_b_paths.clone(),
                now + chrono::Duration::seconds(i as i64),
            ));
        }

        for (session_id, title, paths, updated_at) in &threads_to_save {
            let save_task = cx.update(|cx| {
                let thread_store = ThreadStore::global(cx);
                let session_id = session_id.to_string();
                let title = title.to_string();
                let paths = paths.clone();
                thread_store.update(cx, |store, cx| {
                    store.save_thread(
                        acp::SessionId::new(session_id),
                        make_db_thread(&title, *updated_at),
                        paths,
                        cx,
                    )
                })
            });
            save_task.await.unwrap();
            cx.run_until_parked();
        }

        cx.update(|cx| migrate_thread_metadata(cx));
        cx.run_until_parked();

        let list = cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.read(cx).entries().cloned().collect::<Vec<_>>()
        });

        assert_eq!(list.len(), 10);

        // Project A: 5 most recent should be unarchived, 2 oldest should be archived
        let mut project_a_entries: Vec<_> = list
            .iter()
            .filter(|m| m.folder_paths == project_a_paths)
            .collect();
        assert_eq!(project_a_entries.len(), 7);
        project_a_entries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        for entry in &project_a_entries[..5] {
            assert!(
                !entry.archived,
                "Expected {} to be unarchived (top 5 most recent)",
                entry.session_id.0
            );
        }
        for entry in &project_a_entries[5..] {
            assert!(
                entry.archived,
                "Expected {} to be archived (older than top 5)",
                entry.session_id.0
            );
        }

        // Project B: all 3 should be unarchived (under the limit)
        let project_b_entries: Vec<_> = list
            .iter()
            .filter(|m| m.folder_paths == project_b_paths)
            .collect();
        assert_eq!(project_b_entries.len(), 3);
        assert!(project_b_entries.iter().all(|m| !m.archived));
    }

    #[gpui::test]
    async fn test_empty_thread_metadata_deleted_when_thread_released(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, None::<&Path>, cx).await;
        let connection = Rc::new(StubAgentConnection::new());

        let thread = cx
            .update(|cx| {
                connection
                    .clone()
                    .new_session(project.clone(), PathList::default(), cx)
            })
            .await
            .unwrap();
        let session_id = cx.read(|cx| thread.read(cx).session_id().clone());

        cx.update(|cx| {
            thread.update(cx, |thread, cx| {
                thread.set_title("Draft Thread".into(), cx).detach();
            });
        });
        cx.run_until_parked();

        let metadata_ids = cx.update(|cx| {
            ThreadMetadataStore::global(cx)
                .read(cx)
                .entry_ids()
                .collect::<Vec<_>>()
        });
        assert_eq!(metadata_ids, vec![session_id]);

        drop(thread);
        cx.update(|_| {});
        cx.run_until_parked();
        cx.run_until_parked();

        let metadata_ids = cx.update(|cx| {
            ThreadMetadataStore::global(cx)
                .read(cx)
                .entry_ids()
                .collect::<Vec<_>>()
        });
        assert!(
            metadata_ids.is_empty(),
            "expected empty draft thread metadata to be deleted on release"
        );
    }

    #[gpui::test]
    async fn test_nonempty_thread_metadata_preserved_when_thread_released(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, None::<&Path>, cx).await;
        let connection = Rc::new(StubAgentConnection::new());

        let thread = cx
            .update(|cx| {
                connection
                    .clone()
                    .new_session(project.clone(), PathList::default(), cx)
            })
            .await
            .unwrap();
        let session_id = cx.read(|cx| thread.read(cx).session_id().clone());

        cx.update(|cx| {
            thread.update(cx, |thread, cx| {
                thread.push_user_content_block(None, "Hello".into(), cx);
            });
        });
        cx.run_until_parked();

        let metadata_ids = cx.update(|cx| {
            ThreadMetadataStore::global(cx)
                .read(cx)
                .entry_ids()
                .collect::<Vec<_>>()
        });
        assert_eq!(metadata_ids, vec![session_id.clone()]);

        drop(thread);
        cx.update(|_| {});
        cx.run_until_parked();

        let metadata_ids = cx.update(|cx| {
            ThreadMetadataStore::global(cx)
                .read(cx)
                .entry_ids()
                .collect::<Vec<_>>()
        });
        assert_eq!(metadata_ids, vec![session_id]);
    }

    #[gpui::test]
    async fn test_subagent_threads_excluded_from_sidebar_metadata(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, None::<&Path>, cx).await;
        let connection = Rc::new(StubAgentConnection::new());

        // Create a regular (non-subagent) AcpThread.
        let regular_thread = cx
            .update(|cx| {
                connection
                    .clone()
                    .new_session(project.clone(), PathList::default(), cx)
            })
            .await
            .unwrap();

        let regular_session_id = cx.read(|cx| regular_thread.read(cx).session_id().clone());

        // Set a title on the regular thread to trigger a save via handle_thread_update.
        cx.update(|cx| {
            regular_thread.update(cx, |thread, cx| {
                thread.set_title("Regular Thread".into(), cx).detach();
            });
        });
        cx.run_until_parked();

        // Create a subagent AcpThread
        let subagent_session_id = acp::SessionId::new("subagent-session");
        let subagent_thread = cx.update(|cx| {
            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            cx.new(|cx| {
                acp_thread::AcpThread::new(
                    Some(regular_session_id.clone()),
                    Some("Subagent Thread".into()),
                    None,
                    connection.clone(),
                    project.clone(),
                    action_log,
                    subagent_session_id.clone(),
                    watch::Receiver::constant(acp::PromptCapabilities::new()),
                    cx,
                )
            })
        });

        // Set a title on the subagent thread to trigger handle_thread_update.
        cx.update(|cx| {
            subagent_thread.update(cx, |thread, cx| {
                thread
                    .set_title("Subagent Thread Title".into(), cx)
                    .detach();
            });
        });
        cx.run_until_parked();

        // List all metadata from the store cache.
        let list = cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.read(cx).entries().cloned().collect::<Vec<_>>()
        });

        // The subagent thread should NOT appear in the sidebar metadata.
        // Only the regular thread should be listed.
        assert_eq!(
            list.len(),
            1,
            "Expected only the regular thread in sidebar metadata, \
             but found {} entries (subagent threads are leaking into the sidebar)",
            list.len(),
        );
        assert_eq!(list[0].session_id, regular_session_id);
        assert_eq!(list[0].title.as_ref(), "Regular Thread");
    }

    #[test]
    fn test_dedup_db_operations_keeps_latest_operation_for_session() {
        let now = Utc::now();

        let operations = vec![
            DbOperation::Upsert(make_metadata(
                "session-1",
                "First Thread",
                now,
                PathList::default(),
            )),
            DbOperation::Delete(acp::SessionId::new("session-1")),
        ];

        let deduped = ThreadMetadataStore::dedup_db_operations(operations);

        assert_eq!(deduped.len(), 1);
        assert_eq!(
            deduped[0],
            DbOperation::Delete(acp::SessionId::new("session-1"))
        );
    }

    #[test]
    fn test_dedup_db_operations_keeps_latest_insert_for_same_session() {
        let now = Utc::now();
        let later = now + chrono::Duration::seconds(1);

        let old_metadata = make_metadata("session-1", "Old Title", now, PathList::default());
        let new_metadata = make_metadata("session-1", "New Title", later, PathList::default());

        let deduped = ThreadMetadataStore::dedup_db_operations(vec![
            DbOperation::Upsert(old_metadata),
            DbOperation::Upsert(new_metadata.clone()),
        ]);

        assert_eq!(deduped.len(), 1);
        assert_eq!(deduped[0], DbOperation::Upsert(new_metadata));
    }

    #[test]
    fn test_dedup_db_operations_preserves_distinct_sessions() {
        let now = Utc::now();

        let metadata1 = make_metadata("session-1", "First Thread", now, PathList::default());
        let metadata2 = make_metadata("session-2", "Second Thread", now, PathList::default());
        let deduped = ThreadMetadataStore::dedup_db_operations(vec![
            DbOperation::Upsert(metadata1.clone()),
            DbOperation::Upsert(metadata2.clone()),
        ]);

        assert_eq!(deduped.len(), 2);
        assert!(deduped.contains(&DbOperation::Upsert(metadata1)));
        assert!(deduped.contains(&DbOperation::Upsert(metadata2)));
    }

    #[gpui::test]
    async fn test_archive_and_unarchive_thread(cx: &mut TestAppContext) {
        init_test(cx);

        let paths = PathList::new(&[Path::new("/project-a")]);
        let now = Utc::now();
        let metadata = make_metadata("session-1", "Thread 1", now, paths.clone());

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.save(metadata, cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let path_entries = store
                .entries_for_path(&paths)
                .map(|e| e.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(path_entries, vec!["session-1"]);

            let archived = store
                .archived_entries()
                .map(|e| e.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert!(archived.is_empty());
        });

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.archive(&acp::SessionId::new("session-1"), cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let path_entries = store
                .entries_for_path(&paths)
                .map(|e| e.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert!(path_entries.is_empty());

            let archived = store.archived_entries().collect::<Vec<_>>();
            assert_eq!(archived.len(), 1);
            assert_eq!(archived[0].session_id.0.as_ref(), "session-1");
            assert!(archived[0].archived);
        });

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.unarchive(&acp::SessionId::new("session-1"), cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let path_entries = store
                .entries_for_path(&paths)
                .map(|e| e.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(path_entries, vec!["session-1"]);

            let archived = store
                .archived_entries()
                .map(|e| e.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert!(archived.is_empty());
        });
    }

    #[gpui::test]
    async fn test_entries_for_path_excludes_archived(cx: &mut TestAppContext) {
        init_test(cx);

        let paths = PathList::new(&[Path::new("/project-a")]);
        let now = Utc::now();

        let metadata1 = make_metadata("session-1", "Active Thread", now, paths.clone());
        let metadata2 = make_metadata(
            "session-2",
            "Archived Thread",
            now - chrono::Duration::seconds(1),
            paths.clone(),
        );

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.save(metadata1, cx);
                store.save(metadata2, cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.archive(&acp::SessionId::new("session-2"), cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let path_entries = store
                .entries_for_path(&paths)
                .map(|e| e.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(path_entries, vec!["session-1"]);

            let all_entries = store
                .entries()
                .map(|e| e.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(all_entries.len(), 2);
            assert!(all_entries.contains(&"session-1".to_string()));
            assert!(all_entries.contains(&"session-2".to_string()));

            let archived = store
                .archived_entries()
                .map(|e| e.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(archived, vec!["session-2"]);
        });
    }

    #[gpui::test]
    async fn test_save_all_persists_multiple_threads(cx: &mut TestAppContext) {
        init_test(cx);

        let paths = PathList::new(&[Path::new("/project-a")]);
        let now = Utc::now();

        let m1 = make_metadata("session-1", "Thread One", now, paths.clone());
        let m2 = make_metadata(
            "session-2",
            "Thread Two",
            now - chrono::Duration::seconds(1),
            paths.clone(),
        );
        let m3 = make_metadata(
            "session-3",
            "Thread Three",
            now - chrono::Duration::seconds(2),
            paths,
        );

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.save_all(vec![m1, m2, m3], cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let all_entries = store
                .entries()
                .map(|e| e.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(all_entries.len(), 3);
            assert!(all_entries.contains(&"session-1".to_string()));
            assert!(all_entries.contains(&"session-2".to_string()));
            assert!(all_entries.contains(&"session-3".to_string()));

            let entry_ids = store.entry_ids().collect::<Vec<_>>();
            assert_eq!(entry_ids.len(), 3);
        });
    }

    #[gpui::test]
    async fn test_archived_flag_persists_across_reload(cx: &mut TestAppContext) {
        init_test(cx);

        let paths = PathList::new(&[Path::new("/project-a")]);
        let now = Utc::now();
        let metadata = make_metadata("session-1", "Thread 1", now, paths.clone());

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.save(metadata, cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.archive(&acp::SessionId::new("session-1"), cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                let _ = store.reload(cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let thread = store
                .entries()
                .find(|e| e.session_id.0.as_ref() == "session-1")
                .expect("thread should exist after reload");
            assert!(thread.archived);

            let path_entries = store
                .entries_for_path(&paths)
                .map(|e| e.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert!(path_entries.is_empty());

            let archived = store
                .archived_entries()
                .map(|e| e.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(archived, vec!["session-1"]);
        });
    }

    #[gpui::test]
    async fn test_archive_nonexistent_thread_is_noop(cx: &mut TestAppContext) {
        init_test(cx);

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.archive(&acp::SessionId::new("nonexistent"), cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            assert!(store.is_empty());
            assert_eq!(store.entries().count(), 0);
            assert_eq!(store.archived_entries().count(), 0);
        });
    }

    #[gpui::test]
    async fn test_save_followed_by_archiving_without_parking(cx: &mut TestAppContext) {
        init_test(cx);

        let paths = PathList::new(&[Path::new("/project-a")]);
        let now = Utc::now();
        let metadata = make_metadata("session-1", "Thread 1", now, paths);
        let session_id = metadata.session_id.clone();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.save(metadata.clone(), cx);
                store.archive(&session_id, cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let entries: Vec<ThreadMetadata> = store.entries().cloned().collect();
            pretty_assertions::assert_eq!(
                entries,
                vec![ThreadMetadata {
                    archived: true,
                    ..metadata
                }]
            );
        });
    }
}
