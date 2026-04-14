use std::{path::PathBuf, sync::Arc};

use agent::{ThreadStore, ZED_AGENT_ID};
use agent_client_protocol as acp;
use anyhow::Context as _;
use chrono::{DateTime, Utc};
use collections::{HashMap, HashSet};
use db::{
    kvp::KeyValueStore,
    sqlez::{
        bindable::{Bind, Column},
        domain::Domain,
        statement::Statement,
        thread_safe_connection::ThreadSafeConnection,
    },
    sqlez_macros::sql,
};
use fs::Fs;
use futures::{FutureExt, future::Shared};
use gpui::{AppContext as _, Entity, Global, Subscription, Task};
use project::AgentId;
pub use project::WorktreePaths;
use remote::RemoteConnectionOptions;
use ui::{App, Context, SharedString};
use util::ResultExt as _;
use workspace::{PathList, SerializedWorkspaceLocation, WorkspaceDb};

use crate::DEFAULT_THREAD_TITLE;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub struct ThreadId(uuid::Uuid);

impl ThreadId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Bind for ThreadId {
    fn bind(&self, statement: &Statement, start_index: i32) -> anyhow::Result<i32> {
        self.0.bind(statement, start_index)
    }
}

impl Column for ThreadId {
    fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
        let (uuid, next) = Column::column(statement, start_index)?;
        Ok((ThreadId(uuid), next))
    }
}

const THREAD_REMOTE_CONNECTION_MIGRATION_KEY: &str = "thread-metadata-remote-connection-backfill";
const THREAD_ID_MIGRATION_KEY: &str = "thread-metadata-thread-id-backfill";

pub fn init(cx: &mut App) {
    ThreadMetadataStore::init_global(cx);
    let migration_task = migrate_thread_metadata(cx);
    migrate_thread_remote_connections(cx, migration_task);
    migrate_thread_ids(cx);
}

/// Migrate existing thread metadata from native agent thread store to the new metadata storage.
/// We skip migrating threads that do not have a project.
///
/// TODO: Remove this after N weeks of shipping the sidebar
fn migrate_thread_metadata(cx: &mut App) -> Task<anyhow::Result<()>> {
    let store = ThreadMetadataStore::global(cx);
    let db = store.read(cx).db.clone();

    cx.spawn(async move |cx| {
        let existing_list = db.list()?;
        let is_first_migration = existing_list.is_empty();
        let existing_session_ids: HashSet<Arc<str>> = existing_list
            .into_iter()
            .filter_map(|m| m.session_id.map(|s| s.0))
            .collect();

        let mut to_migrate = store.read_with(cx, |_store, cx| {
            ThreadStore::global(cx)
                .read(cx)
                .entries()
                .filter_map(|entry| {
                    if existing_session_ids.contains(&entry.id.0) {
                        return None;
                    }

                    Some(ThreadMetadata {
                        thread_id: ThreadId::new(),
                        session_id: Some(entry.id),
                        agent_id: ZED_AGENT_ID.clone(),
                        title: if entry.title.is_empty()
                            || entry.title.as_ref() == DEFAULT_THREAD_TITLE
                        {
                            None
                        } else {
                            Some(entry.title)
                        },
                        updated_at: entry.updated_at,
                        created_at: entry.created_at,
                        worktree_paths: WorktreePaths::from_folder_paths(&entry.folder_paths),
                        remote_connection: None,
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
                if entry.worktree_paths.is_empty() {
                    continue;
                }
                per_project
                    .entry(entry.worktree_paths.folder_path_list().clone())
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
}

fn migrate_thread_remote_connections(cx: &mut App, migration_task: Task<anyhow::Result<()>>) {
    let store = ThreadMetadataStore::global(cx);
    let db = store.read(cx).db.clone();
    let kvp = KeyValueStore::global(cx);
    let workspace_db = WorkspaceDb::global(cx);
    let fs = <dyn Fs>::global(cx);

    cx.spawn(async move |cx| -> anyhow::Result<()> {
        migration_task.await?;

        if kvp
            .read_kvp(THREAD_REMOTE_CONNECTION_MIGRATION_KEY)?
            .is_some()
        {
            return Ok(());
        }

        let recent_workspaces = workspace_db.recent_workspaces_on_disk(fs.as_ref()).await?;

        let mut local_path_lists = HashSet::<PathList>::default();
        let mut remote_path_lists = HashMap::<PathList, RemoteConnectionOptions>::default();

        recent_workspaces
            .iter()
            .filter(|(_, location, path_list, _)| {
                !path_list.is_empty() && matches!(location, &SerializedWorkspaceLocation::Local)
            })
            .for_each(|(_, _, path_list, _)| {
                local_path_lists.insert(path_list.clone());
            });

        for (_, location, path_list, _) in recent_workspaces {
            match location {
                SerializedWorkspaceLocation::Remote(remote_connection)
                    if !local_path_lists.contains(&path_list) =>
                {
                    remote_path_lists
                        .entry(path_list)
                        .or_insert(remote_connection);
                }
                _ => {}
            }
        }

        let mut reloaded = false;
        for metadata in db.list()? {
            if metadata.remote_connection.is_some() {
                continue;
            }

            if let Some(remote_connection) = remote_path_lists
                .get(metadata.folder_paths())
                .or_else(|| remote_path_lists.get(metadata.main_worktree_paths()))
            {
                db.save(ThreadMetadata {
                    remote_connection: Some(remote_connection.clone()),
                    ..metadata
                })
                .await?;
                reloaded = true;
            }
        }

        let reloaded_task = reloaded
            .then_some(store.update(cx, |store, cx| store.reload(cx)))
            .unwrap_or(Task::ready(()).shared());

        kvp.write_kvp(
            THREAD_REMOTE_CONNECTION_MIGRATION_KEY.to_string(),
            "1".to_string(),
        )
        .await?;
        reloaded_task.await;

        Ok(())
    })
    .detach_and_log_err(cx);
}

fn migrate_thread_ids(cx: &mut App) {
    let store = ThreadMetadataStore::global(cx);
    let db = store.read(cx).db.clone();
    let kvp = KeyValueStore::global(cx);

    cx.spawn(async move |cx| -> anyhow::Result<()> {
        if kvp.read_kvp(THREAD_ID_MIGRATION_KEY)?.is_some() {
            return Ok(());
        }

        let mut reloaded = false;
        for metadata in db.list()? {
            db.save(metadata).await?;
            reloaded = true;
        }

        let reloaded_task = reloaded
            .then_some(store.update(cx, |store, cx| store.reload(cx)))
            .unwrap_or(Task::ready(()).shared());

        kvp.write_kvp(THREAD_ID_MIGRATION_KEY.to_string(), "1".to_string())
            .await?;
        reloaded_task.await;

        Ok(())
    })
    .detach_and_log_err(cx);
}

struct GlobalThreadMetadataStore(Entity<ThreadMetadataStore>);
impl Global for GlobalThreadMetadataStore {}

/// Lightweight metadata for any thread (native or ACP), enough to populate
/// the sidebar list and route to the correct load path when clicked.
#[derive(Debug, Clone, PartialEq)]
pub struct ThreadMetadata {
    pub thread_id: ThreadId,
    pub session_id: Option<acp::SessionId>,
    pub agent_id: AgentId,
    pub title: Option<SharedString>,
    pub updated_at: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
    pub worktree_paths: WorktreePaths,
    pub remote_connection: Option<RemoteConnectionOptions>,
    pub archived: bool,
}

impl ThreadMetadata {
    pub fn new_draft(
        thread_id: ThreadId,
        agent_id: AgentId,
        title: Option<SharedString>,
        worktree_paths: WorktreePaths,
        remote_connection: Option<RemoteConnectionOptions>,
    ) -> Self {
        let now = Utc::now();
        Self {
            thread_id,
            session_id: None,
            agent_id,
            title,
            updated_at: now,
            created_at: Some(now),
            worktree_paths: worktree_paths.clone(),
            remote_connection,
            archived: worktree_paths.is_empty(),
        }
    }

    pub fn is_draft(&self) -> bool {
        self.session_id.is_none()
    }

    pub fn display_title(&self) -> SharedString {
        self.title
            .clone()
            .unwrap_or_else(|| crate::DEFAULT_THREAD_TITLE.into())
    }

    pub fn folder_paths(&self) -> &PathList {
        self.worktree_paths.folder_path_list()
    }
    pub fn main_worktree_paths(&self) -> &PathList {
        self.worktree_paths.main_worktree_path_list()
    }
}

impl From<&ThreadMetadata> for acp_thread::AgentSessionInfo {
    fn from(meta: &ThreadMetadata) -> Self {
        let session_id = meta
            .session_id
            .clone()
            .unwrap_or_else(|| acp::SessionId::new(meta.thread_id.0.to_string()));
        Self {
            session_id,
            work_dirs: Some(meta.folder_paths().clone()),
            title: meta.title.clone(),
            updated_at: Some(meta.updated_at),
            created_at: meta.created_at,
            meta: None,
        }
    }
}

/// Record of a git worktree that was archived (deleted from disk) when its
/// last thread was archived.
pub struct ArchivedGitWorktree {
    /// Auto-incrementing primary key.
    pub id: i64,
    /// Absolute path to the directory of the worktree before it was deleted.
    /// Used when restoring, to put the recreated worktree back where it was.
    /// If the path already exists on disk, the worktree is assumed to be
    /// already restored and is used as-is.
    pub worktree_path: PathBuf,
    /// Absolute path of the main repository ("main worktree") that owned this worktree.
    /// Used when restoring, to reattach the recreated worktree to the correct main repo.
    /// If the main repo isn't found on disk, unarchiving fails because we only store
    /// commit hashes, and without the actual git repo being available, we can't restore
    /// the files.
    pub main_repo_path: PathBuf,
    /// Branch that was checked out in the worktree at archive time. `None` if
    /// the worktree was in detached HEAD state, which isn't supported in Zed, but
    /// could happen if the user made a detached one outside of Zed.
    /// On restore, we try to switch to this branch. If that fails (e.g. it's
    /// checked out elsewhere), we auto-generate a new one.
    pub branch_name: Option<String>,
    /// SHA of the WIP commit that captures files that were staged (but not yet
    /// committed) at the time of archiving. This commit can be empty if the
    /// user had no staged files at the time. It sits directly on top of whatever
    /// the user's last actual commit was.
    pub staged_commit_hash: String,
    /// SHA of the WIP commit that captures files that were unstaged (including
    /// untracked) at the time of archiving. This commit can be empty if the user
    /// had no unstaged files at the time. It sits on top of `staged_commit_hash`.
    /// After doing `git reset` past both of these commits, we're back in the state
    /// we had before archiving, including what was staged, what was unstaged, and
    /// what was committed.
    pub unstaged_commit_hash: String,
    /// SHA of the commit that HEAD pointed at before we created the two WIP
    /// commits during archival. After resetting past the WIP commits during
    /// restore, HEAD should land back on this commit. It also serves as a
    /// pre-restore sanity check (abort if this commit no longer exists in the
    /// repo) and as a fallback target if the WIP resets fail.
    pub original_commit_hash: String,
}

/// The store holds all metadata needed to show threads in the sidebar/the archive.
///
/// Listens to ConversationView events and updates metadata when the root thread changes.
pub struct ThreadMetadataStore {
    db: ThreadMetadataDb,
    threads: HashMap<ThreadId, ThreadMetadata>,
    threads_by_paths: HashMap<PathList, HashSet<ThreadId>>,
    threads_by_main_paths: HashMap<PathList, HashSet<ThreadId>>,
    threads_by_session: HashMap<acp::SessionId, ThreadId>,
    reload_task: Option<Shared<Task<()>>>,
    conversation_subscriptions: HashMap<gpui::EntityId, Subscription>,
    pending_thread_ops_tx: smol::channel::Sender<DbOperation>,
    in_flight_archives: HashMap<ThreadId, (Task<()>, smol::channel::Sender<()>)>,
    _db_operations_task: Task<()>,
}

#[derive(Debug, PartialEq)]
enum DbOperation {
    Upsert(ThreadMetadata),
    Delete(ThreadId),
}

impl DbOperation {
    fn id(&self) -> ThreadId {
        match self {
            DbOperation::Upsert(thread) => thread.thread_id,
            DbOperation::Delete(thread_id) => *thread_id,
        }
    }
}

/// Override for the test DB name used by `ThreadMetadataStore::init_global`.
/// When set as a GPUI global, `init_global` uses this name instead of
/// deriving one from the thread name. This prevents data from leaking
/// across proptest cases that share a thread name.
#[cfg(any(test, feature = "test-support"))]
pub struct TestMetadataDbName(pub String);
#[cfg(any(test, feature = "test-support"))]
impl gpui::Global for TestMetadataDbName {}

#[cfg(any(test, feature = "test-support"))]
impl TestMetadataDbName {
    pub fn global(cx: &App) -> String {
        cx.try_global::<Self>()
            .map(|g| g.0.clone())
            .unwrap_or_else(|| {
                let thread = std::thread::current();
                let test_name = thread.name().unwrap_or("unknown_test");
                format!("THREAD_METADATA_DB_{}", test_name)
            })
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
        let db_name = TestMetadataDbName::global(cx);
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
    pub fn entry_ids(&self) -> impl Iterator<Item = ThreadId> + '_ {
        self.threads.keys().copied()
    }

    /// Returns the metadata for a specific thread, if it exists.
    pub fn entry(&self, thread_id: ThreadId) -> Option<&ThreadMetadata> {
        self.threads.get(&thread_id)
    }

    /// Returns the metadata for a thread identified by its ACP session ID.
    pub fn entry_by_session(&self, session_id: &acp::SessionId) -> Option<&ThreadMetadata> {
        let thread_id = self.threads_by_session.get(session_id)?;
        self.threads.get(thread_id)
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

    /// Returns threads whose `main_worktree_paths` matches the given path list,
    /// excluding archived threads. This finds threads that were opened in a
    /// linked worktree but are associated with the given main worktree.
    pub fn entries_for_main_worktree_path(
        &self,
        path_list: &PathList,
    ) -> impl Iterator<Item = &ThreadMetadata> + '_ {
        self.threads_by_main_paths
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
                    this.threads_by_main_paths.clear();
                    this.threads_by_session.clear();

                    for row in rows {
                        if let Some(sid) = &row.session_id {
                            this.threads_by_session.insert(sid.clone(), row.thread_id);
                        }
                        this.threads_by_paths
                            .entry(row.folder_paths().clone())
                            .or_default()
                            .insert(row.thread_id);
                        if !row.main_worktree_paths().is_empty() {
                            this.threads_by_main_paths
                                .entry(row.main_worktree_paths().clone())
                                .or_default()
                                .insert(row.thread_id);
                        }
                        this.threads.insert(row.thread_id, row);
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
        for metadata in metadata {
            self.save_internal(metadata);
        }
        cx.notify();
    }

    pub fn save(&mut self, metadata: ThreadMetadata, cx: &mut Context<Self>) {
        self.save_internal(metadata);
        cx.notify();
    }

    fn save_internal(&mut self, metadata: ThreadMetadata) {
        if let Some(thread) = self.threads.get(&metadata.thread_id) {
            if thread.folder_paths() != metadata.folder_paths() {
                if let Some(thread_ids) = self.threads_by_paths.get_mut(thread.folder_paths()) {
                    thread_ids.remove(&metadata.thread_id);
                }
            }
            if thread.main_worktree_paths() != metadata.main_worktree_paths()
                && !thread.main_worktree_paths().is_empty()
            {
                if let Some(thread_ids) = self
                    .threads_by_main_paths
                    .get_mut(thread.main_worktree_paths())
                {
                    thread_ids.remove(&metadata.thread_id);
                }
            }
        }

        if let Some(sid) = &metadata.session_id {
            self.threads_by_session
                .insert(sid.clone(), metadata.thread_id);
        }

        self.threads.insert(metadata.thread_id, metadata.clone());

        self.threads_by_paths
            .entry(metadata.folder_paths().clone())
            .or_default()
            .insert(metadata.thread_id);

        if !metadata.main_worktree_paths().is_empty() {
            self.threads_by_main_paths
                .entry(metadata.main_worktree_paths().clone())
                .or_default()
                .insert(metadata.thread_id);
        }

        self.pending_thread_ops_tx
            .try_send(DbOperation::Upsert(metadata))
            .log_err();
    }

    pub fn update_working_directories(
        &mut self,
        thread_id: ThreadId,
        work_dirs: PathList,
        cx: &mut Context<Self>,
    ) {
        if let Some(thread) = self.threads.get(&thread_id) {
            self.save_internal(ThreadMetadata {
                worktree_paths: WorktreePaths::from_path_lists(
                    thread.main_worktree_paths().clone(),
                    work_dirs.clone(),
                )
                .unwrap_or_else(|_| WorktreePaths::from_folder_paths(&work_dirs)),
                ..thread.clone()
            });
            cx.notify();
        }
    }

    pub fn update_worktree_paths(
        &mut self,
        thread_ids: &[ThreadId],
        worktree_paths: WorktreePaths,
        cx: &mut Context<Self>,
    ) {
        let mut changed = false;
        for &thread_id in thread_ids {
            let Some(thread) = self.threads.get(&thread_id) else {
                continue;
            };
            if thread.worktree_paths == worktree_paths {
                continue;
            }
            self.save_internal(ThreadMetadata {
                worktree_paths: worktree_paths.clone(),
                ..thread.clone()
            });
            changed = true;
        }
        if changed {
            cx.notify();
        }
    }

    pub fn archive(
        &mut self,
        thread_id: ThreadId,
        archive_job: Option<(Task<()>, smol::channel::Sender<()>)>,
        cx: &mut Context<Self>,
    ) {
        self.update_archived(thread_id, true, cx);

        if let Some(job) = archive_job {
            self.in_flight_archives.insert(thread_id, job);
        }
    }

    pub fn unarchive(&mut self, thread_id: ThreadId, cx: &mut Context<Self>) {
        self.update_archived(thread_id, false, cx);
        // Dropping the Sender triggers cancellation in the background task.
        self.in_flight_archives.remove(&thread_id);
    }

    pub fn cleanup_completed_archive(&mut self, thread_id: ThreadId) {
        self.in_flight_archives.remove(&thread_id);
    }

    /// Updates a thread's `folder_paths` after an archived worktree has been
    /// restored to disk. The restored worktree may land at a different path
    /// than it had before archival, so each `(old_path, new_path)` pair in
    /// `path_replacements` is applied to the thread's stored folder paths.
    pub fn update_restored_worktree_paths(
        &mut self,
        thread_id: ThreadId,
        path_replacements: &[(PathBuf, PathBuf)],
        cx: &mut Context<Self>,
    ) {
        if let Some(thread) = self.threads.get(&thread_id).cloned() {
            let mut paths: Vec<PathBuf> = thread.folder_paths().paths().to_vec();
            for (old_path, new_path) in path_replacements {
                if let Some(pos) = paths.iter().position(|p| p == old_path) {
                    paths[pos] = new_path.clone();
                }
            }
            let new_folder_paths = PathList::new(&paths);
            self.save_internal(ThreadMetadata {
                worktree_paths: WorktreePaths::from_path_lists(
                    thread.main_worktree_paths().clone(),
                    new_folder_paths.clone(),
                )
                .unwrap_or_else(|_| WorktreePaths::from_folder_paths(&new_folder_paths)),
                ..thread
            });
            cx.notify();
        }
    }

    pub fn complete_worktree_restore(
        &mut self,
        thread_id: ThreadId,
        path_replacements: &[(PathBuf, PathBuf)],
        cx: &mut Context<Self>,
    ) {
        if let Some(thread) = self.threads.get(&thread_id).cloned() {
            let mut paths: Vec<PathBuf> = thread.folder_paths().paths().to_vec();
            for (old_path, new_path) in path_replacements {
                for path in &mut paths {
                    if path == old_path {
                        *path = new_path.clone();
                    }
                }
            }
            let new_folder_paths = PathList::new(&paths);
            self.save_internal(ThreadMetadata {
                worktree_paths: WorktreePaths::from_path_lists(
                    thread.main_worktree_paths().clone(),
                    new_folder_paths.clone(),
                )
                .unwrap_or_else(|_| WorktreePaths::from_folder_paths(&new_folder_paths)),
                ..thread
            });
            cx.notify();
        }
    }

    /// Apply a mutation to the worktree paths of all threads whose current
    /// `folder_paths` matches `current_folder_paths`, then re-index.
    /// When `remote_connection` is provided, only threads with a matching
    /// remote connection are affected.
    pub fn change_worktree_paths(
        &mut self,
        current_folder_paths: &PathList,
        remote_connection: Option<&RemoteConnectionOptions>,
        mutate: impl Fn(&mut WorktreePaths),
        cx: &mut Context<Self>,
    ) {
        let thread_ids: Vec<_> = self
            .threads_by_paths
            .get(current_folder_paths)
            .into_iter()
            .flatten()
            .filter(|id| {
                remote_connection.is_none()
                    || self
                        .threads
                        .get(id)
                        .and_then(|t| t.remote_connection.as_ref())
                        == remote_connection
            })
            .copied()
            .collect();

        self.mutate_thread_paths(&thread_ids, mutate, cx);
    }

    /// Like `change_worktree_paths`, but looks up threads by their
    /// `main_worktree_paths` instead of `folder_paths`. Used when
    /// migrating threads for project group key changes where the
    /// lookup key is the group key's main paths.
    /// When `remote_connection` is provided, only threads with a matching
    /// remote connection are affected.
    pub fn change_worktree_paths_by_main(
        &mut self,
        current_main_paths: &PathList,
        remote_connection: Option<&RemoteConnectionOptions>,
        mutate: impl Fn(&mut WorktreePaths),
        cx: &mut Context<Self>,
    ) {
        let thread_ids: Vec<_> = self
            .threads_by_main_paths
            .get(current_main_paths)
            .into_iter()
            .flatten()
            .filter(|id| {
                remote_connection.is_none()
                    || self
                        .threads
                        .get(id)
                        .and_then(|t| t.remote_connection.as_ref())
                        == remote_connection
            })
            .copied()
            .collect();

        self.mutate_thread_paths(&thread_ids, mutate, cx);
    }

    fn mutate_thread_paths(
        &mut self,
        thread_ids: &[ThreadId],
        mutate: impl Fn(&mut WorktreePaths),
        cx: &mut Context<Self>,
    ) {
        if thread_ids.is_empty() {
            return;
        }

        for thread_id in thread_ids {
            if let Some(thread) = self.threads.get_mut(thread_id) {
                if let Some(ids) = self
                    .threads_by_main_paths
                    .get_mut(thread.main_worktree_paths())
                {
                    ids.remove(thread_id);
                }
                if let Some(ids) = self.threads_by_paths.get_mut(thread.folder_paths()) {
                    ids.remove(thread_id);
                }

                mutate(&mut thread.worktree_paths);

                self.threads_by_main_paths
                    .entry(thread.main_worktree_paths().clone())
                    .or_default()
                    .insert(*thread_id);
                self.threads_by_paths
                    .entry(thread.folder_paths().clone())
                    .or_default()
                    .insert(*thread_id);

                self.pending_thread_ops_tx
                    .try_send(DbOperation::Upsert(thread.clone()))
                    .log_err();
            }
        }

        cx.notify();
    }

    pub fn create_archived_worktree(
        &self,
        worktree_path: String,
        main_repo_path: String,
        branch_name: Option<String>,
        staged_commit_hash: String,
        unstaged_commit_hash: String,
        original_commit_hash: String,
        cx: &App,
    ) -> Task<anyhow::Result<i64>> {
        let db = self.db.clone();
        cx.background_spawn(async move {
            db.create_archived_worktree(
                worktree_path,
                main_repo_path,
                branch_name,
                staged_commit_hash,
                unstaged_commit_hash,
                original_commit_hash,
            )
            .await
        })
    }

    pub fn link_thread_to_archived_worktree(
        &self,
        thread_id: ThreadId,
        archived_worktree_id: i64,
        cx: &App,
    ) -> Task<anyhow::Result<()>> {
        let db = self.db.clone();
        cx.background_spawn(async move {
            db.link_thread_to_archived_worktree(thread_id, archived_worktree_id)
                .await
        })
    }

    pub fn get_archived_worktrees_for_thread(
        &self,
        thread_id: ThreadId,
        cx: &App,
    ) -> Task<anyhow::Result<Vec<ArchivedGitWorktree>>> {
        let db = self.db.clone();
        cx.background_spawn(async move { db.get_archived_worktrees_for_thread(thread_id).await })
    }

    pub fn delete_archived_worktree(&self, id: i64, cx: &App) -> Task<anyhow::Result<()>> {
        let db = self.db.clone();
        cx.background_spawn(async move { db.delete_archived_worktree(id).await })
    }

    pub fn unlink_thread_from_all_archived_worktrees(
        &self,
        thread_id: ThreadId,
        cx: &App,
    ) -> Task<anyhow::Result<()>> {
        let db = self.db.clone();
        cx.background_spawn(async move {
            db.unlink_thread_from_all_archived_worktrees(thread_id)
                .await
        })
    }

    pub fn is_archived_worktree_referenced(
        &self,
        archived_worktree_id: i64,
        cx: &App,
    ) -> Task<anyhow::Result<bool>> {
        let db = self.db.clone();
        cx.background_spawn(async move {
            db.is_archived_worktree_referenced(archived_worktree_id)
                .await
        })
    }

    fn update_archived(&mut self, thread_id: ThreadId, archived: bool, cx: &mut Context<Self>) {
        if let Some(thread) = self.threads.get(&thread_id) {
            self.save_internal(ThreadMetadata {
                archived,
                ..thread.clone()
            });
            cx.notify();
        }
    }

    pub fn delete(&mut self, thread_id: ThreadId, cx: &mut Context<Self>) {
        if let Some(thread) = self.threads.get(&thread_id) {
            if let Some(sid) = &thread.session_id {
                self.threads_by_session.remove(sid);
            }
            if let Some(thread_ids) = self.threads_by_paths.get_mut(thread.folder_paths()) {
                thread_ids.remove(&thread_id);
            }
            if !thread.main_worktree_paths().is_empty() {
                if let Some(thread_ids) = self
                    .threads_by_main_paths
                    .get_mut(thread.main_worktree_paths())
                {
                    thread_ids.remove(&thread_id);
                }
            }
        }
        self.threads.remove(&thread_id);
        self.pending_thread_ops_tx
            .try_send(DbOperation::Delete(thread_id))
            .log_err();
        cx.notify();
    }

    fn new(db: ThreadMetadataDb, cx: &mut Context<Self>) -> Self {
        let weak_store = cx.weak_entity();

        cx.observe_new::<crate::ConversationView>(move |_view, _window, cx| {
            let view_entity = cx.entity();
            let entity_id = view_entity.entity_id();

            cx.on_release({
                let weak_store = weak_store.clone();
                move |_view, cx| {
                    weak_store
                        .update(cx, |store, _cx| {
                            store.conversation_subscriptions.remove(&entity_id);
                        })
                        .ok();
                }
            })
            .detach();

            weak_store
                .update(cx, |this, cx| {
                    let subscription = cx.subscribe(&view_entity, Self::handle_conversation_event);
                    this.conversation_subscriptions
                        .insert(entity_id, subscription);
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
                            DbOperation::Delete(thread_id) => {
                                db.delete(thread_id).await.log_err();
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
            threads_by_main_paths: HashMap::default(),
            threads_by_session: HashMap::default(),
            reload_task: None,
            conversation_subscriptions: HashMap::default(),
            pending_thread_ops_tx: tx,
            in_flight_archives: HashMap::default(),
            _db_operations_task,
        };
        let _ = this.reload(cx);
        this
    }

    fn dedup_db_operations(operations: Vec<DbOperation>) -> Vec<DbOperation> {
        let mut ops = HashMap::default();
        for operation in operations.into_iter().rev() {
            if ops.contains_key(&operation.id()) {
                continue;
            }
            ops.insert(operation.id(), operation);
        }
        ops.into_values().collect()
    }

    fn handle_conversation_event(
        &mut self,
        conversation_view: Entity<crate::ConversationView>,
        _event: &crate::conversation_view::RootThreadUpdated,
        cx: &mut Context<Self>,
    ) {
        let view = conversation_view.read(cx);
        let thread_id = view.thread_id;
        let Some(thread) = view.root_acp_thread(cx) else {
            return;
        };

        let thread_ref = thread.read(cx);
        if thread_ref.entries().is_empty() {
            return;
        }

        let existing_thread = self.entry(thread_id);
        let session_id = Some(thread_ref.session_id().clone());
        let title = thread_ref.title();

        let updated_at = Utc::now();

        let created_at = existing_thread
            .and_then(|t| t.created_at)
            .unwrap_or_else(|| updated_at);

        let agent_id = thread_ref.connection().agent_id();

        let project = thread_ref.project().read(cx);
        let worktree_paths = project.worktree_paths(cx);

        let remote_connection = project.remote_connection_options(cx);

        // Threads without a folder path (e.g. started in an empty
        // window) are archived by default so they don't get lost,
        // because they won't show up in the sidebar. Users can reload
        // them from the archive.
        let archived = existing_thread
            .map(|t| t.archived)
            .unwrap_or(worktree_paths.is_empty());

        let metadata = ThreadMetadata {
            thread_id,
            session_id,
            agent_id,
            title,
            created_at: Some(created_at),
            updated_at,
            worktree_paths,
            remote_connection,
            archived,
        };

        self.save(metadata, cx);
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
        sql!(ALTER TABLE sidebar_threads ADD COLUMN main_worktree_paths TEXT),
        sql!(ALTER TABLE sidebar_threads ADD COLUMN main_worktree_paths_order TEXT),
        sql!(
            CREATE TABLE IF NOT EXISTS archived_git_worktrees(
                id INTEGER PRIMARY KEY,
                worktree_path TEXT NOT NULL,
                main_repo_path TEXT NOT NULL,
                branch_name TEXT,
                staged_commit_hash TEXT,
                unstaged_commit_hash TEXT,
                original_commit_hash TEXT
            ) STRICT;

            CREATE TABLE IF NOT EXISTS thread_archived_worktrees(
                session_id TEXT NOT NULL,
                archived_worktree_id INTEGER NOT NULL REFERENCES archived_git_worktrees(id),
                PRIMARY KEY (session_id, archived_worktree_id)
            ) STRICT;
        ),
        sql!(ALTER TABLE sidebar_threads ADD COLUMN remote_connection TEXT),
        sql!(ALTER TABLE sidebar_threads ADD COLUMN thread_id BLOB),
        sql!(
            UPDATE sidebar_threads SET thread_id = randomblob(16) WHERE thread_id IS NULL;

            CREATE TABLE thread_archived_worktrees_v2(
                thread_id BLOB NOT NULL,
                archived_worktree_id INTEGER NOT NULL REFERENCES archived_git_worktrees(id),
                PRIMARY KEY (thread_id, archived_worktree_id)
            ) STRICT;

            INSERT INTO thread_archived_worktrees_v2(thread_id, archived_worktree_id)
            SELECT s.thread_id, t.archived_worktree_id
            FROM thread_archived_worktrees t
            JOIN sidebar_threads s ON s.session_id = t.session_id;

            DROP TABLE thread_archived_worktrees;
            ALTER TABLE thread_archived_worktrees_v2 RENAME TO thread_archived_worktrees;

            CREATE TABLE sidebar_threads_v2(
                thread_id BLOB PRIMARY KEY,
                session_id TEXT,
                agent_id TEXT,
                title TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                created_at TEXT,
                folder_paths TEXT,
                folder_paths_order TEXT,
                archived INTEGER DEFAULT 0,
                main_worktree_paths TEXT,
                main_worktree_paths_order TEXT,
                remote_connection TEXT
            ) STRICT;

            INSERT INTO sidebar_threads_v2(thread_id, session_id, agent_id, title, updated_at, created_at, folder_paths, folder_paths_order, archived, main_worktree_paths, main_worktree_paths_order, remote_connection)
            SELECT thread_id, session_id, agent_id, title, updated_at, created_at, folder_paths, folder_paths_order, archived, main_worktree_paths, main_worktree_paths_order, remote_connection
            FROM sidebar_threads;

            DROP TABLE sidebar_threads;
            ALTER TABLE sidebar_threads_v2 RENAME TO sidebar_threads;
        ),
    ];
}

db::static_connection!(ThreadMetadataDb, []);

impl ThreadMetadataDb {
    #[allow(dead_code)]
    pub fn list_ids(&self) -> anyhow::Result<Vec<ThreadId>> {
        self.select::<ThreadId>(
            "SELECT thread_id FROM sidebar_threads \
             ORDER BY updated_at DESC",
        )?()
    }

    /// List all sidebar thread metadata, ordered by updated_at descending.
    pub fn list(&self) -> anyhow::Result<Vec<ThreadMetadata>> {
        self.select::<ThreadMetadata>(
            "SELECT thread_id, session_id, agent_id, title, updated_at, created_at, folder_paths, folder_paths_order, archived, main_worktree_paths, main_worktree_paths_order, remote_connection \
             FROM sidebar_threads \
             ORDER BY updated_at DESC"
        )?()
    }

    /// Upsert metadata for a thread.
    pub async fn save(&self, row: ThreadMetadata) -> anyhow::Result<()> {
        let session_id = row.session_id.as_ref().map(|s| s.0.clone());
        let agent_id = if row.agent_id.as_ref() == ZED_AGENT_ID.as_ref() {
            None
        } else {
            Some(row.agent_id.to_string())
        };
        let title = row
            .title
            .as_ref()
            .map(|t| t.to_string())
            .unwrap_or_default();
        let updated_at = row.updated_at.to_rfc3339();
        let created_at = row.created_at.map(|dt| dt.to_rfc3339());
        let serialized = row.folder_paths().serialize();
        let (folder_paths, folder_paths_order) = if row.folder_paths().is_empty() {
            (None, None)
        } else {
            (Some(serialized.paths), Some(serialized.order))
        };
        let main_serialized = row.main_worktree_paths().serialize();
        let (main_worktree_paths, main_worktree_paths_order) =
            if row.main_worktree_paths().is_empty() {
                (None, None)
            } else {
                (Some(main_serialized.paths), Some(main_serialized.order))
            };
        let remote_connection = row
            .remote_connection
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .context("serialize thread metadata remote connection")?;
        let thread_id = row.thread_id;
        let archived = row.archived;

        self.write(move |conn| {
            let sql = "INSERT INTO sidebar_threads(thread_id, session_id, agent_id, title, updated_at, created_at, folder_paths, folder_paths_order, archived, main_worktree_paths, main_worktree_paths_order, remote_connection) \
                       VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12) \
                       ON CONFLICT(thread_id) DO UPDATE SET \
                           session_id = excluded.session_id, \
                           agent_id = excluded.agent_id, \
                           title = excluded.title, \
                           updated_at = excluded.updated_at, \
                           created_at = excluded.created_at, \
                           folder_paths = excluded.folder_paths, \
                           folder_paths_order = excluded.folder_paths_order, \
                           archived = excluded.archived, \
                           main_worktree_paths = excluded.main_worktree_paths, \
                           main_worktree_paths_order = excluded.main_worktree_paths_order, \
                           remote_connection = excluded.remote_connection";
            let mut stmt = Statement::prepare(conn, sql)?;
            let mut i = stmt.bind(&thread_id, 1)?;
            i = stmt.bind(&session_id, i)?;
            i = stmt.bind(&agent_id, i)?;
            i = stmt.bind(&title, i)?;
            i = stmt.bind(&updated_at, i)?;
            i = stmt.bind(&created_at, i)?;
            i = stmt.bind(&folder_paths, i)?;
            i = stmt.bind(&folder_paths_order, i)?;
            i = stmt.bind(&archived, i)?;
            i = stmt.bind(&main_worktree_paths, i)?;
            i = stmt.bind(&main_worktree_paths_order, i)?;
            stmt.bind(&remote_connection, i)?;
            stmt.exec()
        })
        .await
    }

    /// Delete metadata for a single thread.
    pub async fn delete(&self, thread_id: ThreadId) -> anyhow::Result<()> {
        self.write(move |conn| {
            let mut stmt =
                Statement::prepare(conn, "DELETE FROM sidebar_threads WHERE thread_id = ?")?;
            stmt.bind(&thread_id, 1)?;
            stmt.exec()
        })
        .await
    }

    pub async fn create_archived_worktree(
        &self,
        worktree_path: String,
        main_repo_path: String,
        branch_name: Option<String>,
        staged_commit_hash: String,
        unstaged_commit_hash: String,
        original_commit_hash: String,
    ) -> anyhow::Result<i64> {
        self.write(move |conn| {
            let mut stmt = Statement::prepare(
                conn,
                "INSERT INTO archived_git_worktrees(worktree_path, main_repo_path, branch_name, staged_commit_hash, unstaged_commit_hash, original_commit_hash) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6) \
                 RETURNING id",
            )?;
            let mut i = stmt.bind(&worktree_path, 1)?;
            i = stmt.bind(&main_repo_path, i)?;
            i = stmt.bind(&branch_name, i)?;
            i = stmt.bind(&staged_commit_hash, i)?;
            i = stmt.bind(&unstaged_commit_hash, i)?;
            stmt.bind(&original_commit_hash, i)?;
            stmt.maybe_row::<i64>()?.context("expected RETURNING id")
        })
        .await
    }

    pub async fn link_thread_to_archived_worktree(
        &self,
        thread_id: ThreadId,
        archived_worktree_id: i64,
    ) -> anyhow::Result<()> {
        self.write(move |conn| {
            let mut stmt = Statement::prepare(
                conn,
                "INSERT INTO thread_archived_worktrees(thread_id, archived_worktree_id) \
                 VALUES (?1, ?2)",
            )?;
            let i = stmt.bind(&thread_id, 1)?;
            stmt.bind(&archived_worktree_id, i)?;
            stmt.exec()
        })
        .await
    }

    pub async fn get_archived_worktrees_for_thread(
        &self,
        thread_id: ThreadId,
    ) -> anyhow::Result<Vec<ArchivedGitWorktree>> {
        self.select_bound::<ThreadId, ArchivedGitWorktree>(
            "SELECT a.id, a.worktree_path, a.main_repo_path, a.branch_name, a.staged_commit_hash, a.unstaged_commit_hash, a.original_commit_hash \
             FROM archived_git_worktrees a \
             JOIN thread_archived_worktrees t ON a.id = t.archived_worktree_id \
             WHERE t.thread_id = ?1",
        )?(thread_id)
    }

    pub async fn delete_archived_worktree(&self, id: i64) -> anyhow::Result<()> {
        self.write(move |conn| {
            let mut stmt = Statement::prepare(
                conn,
                "DELETE FROM thread_archived_worktrees WHERE archived_worktree_id = ?",
            )?;
            stmt.bind(&id, 1)?;
            stmt.exec()?;

            let mut stmt =
                Statement::prepare(conn, "DELETE FROM archived_git_worktrees WHERE id = ?")?;
            stmt.bind(&id, 1)?;
            stmt.exec()
        })
        .await
    }

    pub async fn unlink_thread_from_all_archived_worktrees(
        &self,
        thread_id: ThreadId,
    ) -> anyhow::Result<()> {
        self.write(move |conn| {
            let mut stmt = Statement::prepare(
                conn,
                "DELETE FROM thread_archived_worktrees WHERE thread_id = ?",
            )?;
            stmt.bind(&thread_id, 1)?;
            stmt.exec()
        })
        .await
    }

    pub async fn is_archived_worktree_referenced(
        &self,
        archived_worktree_id: i64,
    ) -> anyhow::Result<bool> {
        self.select_row_bound::<i64, i64>(
            "SELECT COUNT(*) FROM thread_archived_worktrees WHERE archived_worktree_id = ?1",
        )?(archived_worktree_id)
        .map(|count| count.unwrap_or(0) > 0)
    }
}

impl Column for ThreadMetadata {
    fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
        let (thread_id_uuid, next): (uuid::Uuid, i32) = Column::column(statement, start_index)?;
        let (id, next): (Option<Arc<str>>, i32) = Column::column(statement, next)?;
        let (agent_id, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (title, next): (String, i32) = Column::column(statement, next)?;
        let (updated_at_str, next): (String, i32) = Column::column(statement, next)?;
        let (created_at_str, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (folder_paths_str, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (folder_paths_order_str, next): (Option<String>, i32) =
            Column::column(statement, next)?;
        let (archived, next): (bool, i32) = Column::column(statement, next)?;
        let (main_worktree_paths_str, next): (Option<String>, i32) =
            Column::column(statement, next)?;
        let (main_worktree_paths_order_str, next): (Option<String>, i32) =
            Column::column(statement, next)?;
        let (remote_connection_json, next): (Option<String>, i32) =
            Column::column(statement, next)?;

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

        let main_worktree_paths = main_worktree_paths_str
            .map(|paths| {
                PathList::deserialize(&util::path_list::SerializedPathList {
                    paths,
                    order: main_worktree_paths_order_str.unwrap_or_default(),
                })
            })
            .unwrap_or_default();

        let remote_connection = remote_connection_json
            .as_deref()
            .map(serde_json::from_str::<RemoteConnectionOptions>)
            .transpose()
            .context("deserialize thread metadata remote connection")?;

        let worktree_paths = WorktreePaths::from_path_lists(main_worktree_paths, folder_paths)
            .unwrap_or_else(|_| WorktreePaths::default());

        let thread_id = ThreadId(thread_id_uuid);

        Ok((
            ThreadMetadata {
                thread_id,
                session_id: id.map(acp::SessionId::new),
                agent_id,
                title: if title.is_empty() || title == DEFAULT_THREAD_TITLE {
                    None
                } else {
                    Some(title.into())
                },
                updated_at,
                created_at,
                worktree_paths,
                remote_connection,
                archived,
            },
            next,
        ))
    }
}

impl Column for ArchivedGitWorktree {
    fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
        let (id, next): (i64, i32) = Column::column(statement, start_index)?;
        let (worktree_path_str, next): (String, i32) = Column::column(statement, next)?;
        let (main_repo_path_str, next): (String, i32) = Column::column(statement, next)?;
        let (branch_name, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (staged_commit_hash, next): (String, i32) = Column::column(statement, next)?;
        let (unstaged_commit_hash, next): (String, i32) = Column::column(statement, next)?;
        let (original_commit_hash, next): (String, i32) = Column::column(statement, next)?;

        Ok((
            ArchivedGitWorktree {
                id,
                worktree_path: PathBuf::from(worktree_path_str),
                main_repo_path: PathBuf::from(main_repo_path_str),
                branch_name,
                staged_commit_hash,
                unstaged_commit_hash,
                original_commit_hash,
            },
            next,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use acp_thread::StubAgentConnection;
    use action_log::ActionLog;
    use agent::DbThread;
    use agent_client_protocol as acp;

    use gpui::{TestAppContext, VisualTestContext};
    use project::FakeFs;
    use project::Project;
    use remote::WslConnectionOptions;
    use std::path::Path;
    use std::rc::Rc;
    use workspace::MultiWorkspace;

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
            thread_id: ThreadId::new(),
            archived: false,
            session_id: Some(acp::SessionId::new(session_id)),
            agent_id: agent::ZED_AGENT_ID.clone(),
            title: if title.is_empty() {
                None
            } else {
                Some(title.to_string().into())
            },
            updated_at,
            created_at: Some(updated_at),
            worktree_paths: WorktreePaths::from_folder_paths(&folder_paths),
            remote_connection: None,
        }
    }

    fn init_test(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            release_channel::init("0.0.0".parse().unwrap(), cx);
            prompt_store::init(cx);
            <dyn Fs>::set_global(fs, cx);
            ThreadMetadataStore::init_global(cx);
            ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });
        cx.run_until_parked();
    }

    fn setup_panel_with_project(
        project: Entity<Project>,
        cx: &mut TestAppContext,
    ) -> (Entity<crate::AgentPanel>, VisualTestContext) {
        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace_entity = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();
        let mut vcx = VisualTestContext::from_window(multi_workspace.into(), cx);
        let panel = workspace_entity.update_in(&mut vcx, |workspace, window, cx| {
            cx.new(|cx| crate::AgentPanel::new(workspace, None, window, cx))
        });
        (panel, vcx)
    }

    fn clear_thread_metadata_remote_connection_backfill(cx: &mut TestAppContext) {
        let kvp = cx.update(|cx| KeyValueStore::global(cx));
        smol::block_on(kvp.delete_kvp("thread-metadata-remote-connection-backfill".to_string()))
            .unwrap();
    }

    fn run_thread_metadata_migrations(cx: &mut TestAppContext) {
        clear_thread_metadata_remote_connection_backfill(cx);
        cx.update(|cx| {
            let migration_task = migrate_thread_metadata(cx);
            migrate_thread_remote_connections(cx, migration_task);
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
            ThreadMetadataStore::init_global(cx);
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            assert_eq!(store.entry_ids().count(), 2);
            assert!(
                store
                    .entry_by_session(&acp::SessionId::new("session-1"))
                    .is_some()
            );
            assert!(
                store
                    .entry_by_session(&acp::SessionId::new("session-2"))
                    .is_some()
            );

            let first_path_entries: Vec<_> = store
                .entries_for_path(&first_paths)
                .filter_map(|entry| entry.session_id.as_ref().map(|s| s.0.to_string()))
                .collect();
            assert_eq!(first_path_entries, vec!["session-1"]);

            let second_path_entries: Vec<_> = store
                .entries_for_path(&second_paths)
                .filter_map(|entry| entry.session_id.as_ref().map(|s| s.0.to_string()))
                .collect();
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
        let session1_thread_id = initial_metadata.thread_id;

        let second_metadata = make_metadata(
            "session-2",
            "Second Thread",
            initial_time,
            second_paths.clone(),
        );
        let session2_thread_id = second_metadata.thread_id;

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

            let first_path_entries: Vec<_> = store
                .entries_for_path(&first_paths)
                .filter_map(|entry| entry.session_id.as_ref().map(|s| s.0.to_string()))
                .collect();
            assert_eq!(first_path_entries, vec!["session-1"]);

            let second_path_entries: Vec<_> = store
                .entries_for_path(&second_paths)
                .filter_map(|entry| entry.session_id.as_ref().map(|s| s.0.to_string()))
                .collect();
            assert_eq!(second_path_entries, vec!["session-2"]);
        });

        let moved_metadata = ThreadMetadata {
            thread_id: session1_thread_id,
            session_id: Some(acp::SessionId::new("session-1")),
            agent_id: agent::ZED_AGENT_ID.clone(),
            title: Some("First Thread".into()),
            updated_at: updated_time,
            created_at: Some(updated_time),
            worktree_paths: WorktreePaths::from_folder_paths(&second_paths),
            remote_connection: None,
            archived: false,
        };

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

            assert_eq!(store.entry_ids().count(), 2);
            assert!(
                store
                    .entry_by_session(&acp::SessionId::new("session-1"))
                    .is_some()
            );
            assert!(
                store
                    .entry_by_session(&acp::SessionId::new("session-2"))
                    .is_some()
            );

            let first_path_entries: Vec<_> = store
                .entries_for_path(&first_paths)
                .filter_map(|entry| entry.session_id.as_ref().map(|s| s.0.to_string()))
                .collect();
            assert!(first_path_entries.is_empty());

            let second_path_entries: Vec<_> = store
                .entries_for_path(&second_paths)
                .filter_map(|entry| entry.session_id.as_ref().map(|s| s.0.to_string()))
                .collect();
            assert_eq!(second_path_entries.len(), 2);
            assert!(second_path_entries.contains(&"session-1".to_string()));
            assert!(second_path_entries.contains(&"session-2".to_string()));
        });

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.delete(session2_thread_id, cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            assert_eq!(store.entry_ids().count(), 1);

            let second_path_entries: Vec<_> = store
                .entries_for_path(&second_paths)
                .filter_map(|entry| entry.session_id.as_ref().map(|s| s.0.to_string()))
                .collect();
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
            thread_id: ThreadId::new(),
            session_id: Some(acp::SessionId::new("a-session-0")),
            agent_id: agent::ZED_AGENT_ID.clone(),
            title: Some("Existing Metadata".into()),
            updated_at: now - chrono::Duration::seconds(10),
            created_at: Some(now - chrono::Duration::seconds(10)),
            worktree_paths: WorktreePaths::from_folder_paths(&project_a_paths),
            remote_connection: None,
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

        run_thread_metadata_migrations(cx);

        let list = cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.read(cx).entries().cloned().collect::<Vec<_>>()
        });

        assert_eq!(list.len(), 4);
        assert!(
            list.iter()
                .all(|metadata| metadata.agent_id.as_ref() == agent::ZED_AGENT_ID.as_ref())
        );

        let existing_metadata = list
            .iter()
            .find(|metadata| {
                metadata
                    .session_id
                    .as_ref()
                    .is_some_and(|s| s.0.as_ref() == "a-session-0")
            })
            .unwrap();
        assert_eq!(existing_metadata.display_title(), "Existing Metadata");
        assert!(!existing_metadata.archived);

        let migrated_session_ids: Vec<_> = list
            .iter()
            .filter_map(|metadata| metadata.session_id.as_ref().map(|s| s.0.to_string()))
            .collect();
        assert!(migrated_session_ids.iter().any(|s| s == "a-session-1"));
        assert!(migrated_session_ids.iter().any(|s| s == "b-session-0"));
        assert!(migrated_session_ids.iter().any(|s| s == "projectless"));

        let migrated_entries: Vec<_> = list
            .iter()
            .filter(|metadata| {
                !metadata
                    .session_id
                    .as_ref()
                    .is_some_and(|s| s.0.as_ref() == "a-session-0")
            })
            .collect();
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
            thread_id: ThreadId::new(),
            session_id: Some(acp::SessionId::new("existing-session")),
            agent_id: agent::ZED_AGENT_ID.clone(),
            title: Some("Existing Metadata".into()),
            updated_at: existing_updated_at,
            created_at: Some(existing_updated_at),
            worktree_paths: WorktreePaths::from_folder_paths(&project_paths),
            remote_connection: None,
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

        run_thread_metadata_migrations(cx);

        let list = cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.read(cx).entries().cloned().collect::<Vec<_>>()
        });

        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0].session_id.as_ref().unwrap().0.as_ref(),
            "existing-session"
        );
    }

    #[gpui::test]
    async fn test_migrate_thread_remote_connections_backfills_from_workspace_db(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let folder_paths = PathList::new(&[Path::new("/remote-project")]);
        let updated_at = Utc::now();
        let metadata = make_metadata(
            "remote-session",
            "Remote Thread",
            updated_at,
            folder_paths.clone(),
        );

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.save(metadata, cx);
            });
        });
        cx.run_until_parked();

        let workspace_db = cx.update(|cx| WorkspaceDb::global(cx));
        let workspace_id = workspace_db.next_id().await.unwrap();
        let serialized_paths = folder_paths.serialize();
        let remote_connection_id = 1_i64;
        workspace_db
            .write(move |conn| {
                let mut stmt = Statement::prepare(
                    conn,
                    "INSERT INTO remote_connections(id, kind, user, distro) VALUES (?1, ?2, ?3, ?4)",
                )?;
                let mut next_index = stmt.bind(&remote_connection_id, 1)?;
                next_index = stmt.bind(&"wsl", next_index)?;
                next_index = stmt.bind(&Some("anth".to_string()), next_index)?;
                stmt.bind(&Some("Ubuntu".to_string()), next_index)?;
                stmt.exec()?;

                let mut stmt = Statement::prepare(
                    conn,
                    "UPDATE workspaces SET paths = ?2, paths_order = ?3, remote_connection_id = ?4, timestamp = CURRENT_TIMESTAMP WHERE workspace_id = ?1",
                )?;
                let mut next_index = stmt.bind(&workspace_id, 1)?;
                next_index = stmt.bind(&serialized_paths.paths, next_index)?;
                next_index = stmt.bind(&serialized_paths.order, next_index)?;
                stmt.bind(&Some(remote_connection_id as i32), next_index)?;
                stmt.exec()
            })
            .await
            .unwrap();

        clear_thread_metadata_remote_connection_backfill(cx);
        cx.update(|cx| {
            migrate_thread_remote_connections(cx, Task::ready(Ok(())));
        });
        cx.run_until_parked();

        let metadata = cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store
                .read(cx)
                .entry_by_session(&acp::SessionId::new("remote-session"))
                .cloned()
                .expect("expected migrated metadata row")
        });

        assert_eq!(
            metadata.remote_connection,
            Some(RemoteConnectionOptions::Wsl(WslConnectionOptions {
                distro_name: "Ubuntu".to_string(),
                user: Some("anth".to_string()),
            }))
        );
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

        run_thread_metadata_migrations(cx);

        let list = cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.read(cx).entries().cloned().collect::<Vec<_>>()
        });

        assert_eq!(list.len(), 10);

        // Project A: 5 most recent should be unarchived, 2 oldest should be archived
        let mut project_a_entries: Vec<_> = list
            .iter()
            .filter(|m| *m.folder_paths() == project_a_paths)
            .collect();
        assert_eq!(project_a_entries.len(), 7);
        project_a_entries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        for entry in &project_a_entries[..5] {
            assert!(
                !entry.archived,
                "Expected {:?} to be unarchived (top 5 most recent)",
                entry.session_id
            );
        }
        for entry in &project_a_entries[5..] {
            assert!(
                entry.archived,
                "Expected {:?} to be archived (older than top 5)",
                entry.session_id
            );
        }

        // Project B: all 3 should be unarchived (under the limit)
        let project_b_entries: Vec<_> = list
            .iter()
            .filter(|m| *m.folder_paths() == project_b_paths)
            .collect();
        assert_eq!(project_b_entries.len(), 3);
        assert!(project_b_entries.iter().all(|m| !m.archived));
    }

    #[gpui::test]
    async fn test_empty_thread_events_do_not_create_metadata(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, None::<&Path>, cx).await;
        let connection = StubAgentConnection::new();

        let (panel, mut vcx) = setup_panel_with_project(project, cx);
        crate::test_support::open_thread_with_connection(&panel, connection, &mut vcx);

        let thread = panel.read_with(&vcx, |panel, cx| panel.active_agent_thread(cx).unwrap());
        let session_id = thread.read_with(&vcx, |t, _| t.session_id().clone());
        let thread_id = crate::test_support::active_thread_id(&panel, &vcx);

        // Initial metadata was created by the panel with session_id: None.
        cx.read(|cx| {
            let store = ThreadMetadataStore::global(cx).read(cx);
            assert_eq!(store.entry_ids().count(), 1);
            assert!(
                store.entry(thread_id).unwrap().session_id.is_none(),
                "expected initial panel metadata to have no session_id"
            );
        });

        // Setting a title on an empty thread should be ignored by the
        // event handler (entries are empty), leaving session_id as None.
        thread.update_in(&mut vcx, |thread, _window, cx| {
            thread.set_title("Draft Thread".into(), cx).detach();
        });
        vcx.run_until_parked();

        cx.read(|cx| {
            let store = ThreadMetadataStore::global(cx).read(cx);
            assert!(
                store.entry(thread_id).unwrap().session_id.is_none(),
                "expected title updates on empty thread to be ignored by event handler"
            );
        });

        // Pushing content makes entries non-empty, so the event handler
        // should now update metadata with the real session_id.
        thread.update_in(&mut vcx, |thread, _window, cx| {
            thread.push_user_content_block(None, "Hello".into(), cx);
        });
        vcx.run_until_parked();

        cx.read(|cx| {
            let store = ThreadMetadataStore::global(cx).read(cx);
            assert_eq!(store.entry_ids().count(), 1);
            assert_eq!(
                store.entry(thread_id).unwrap().session_id.as_ref(),
                Some(&session_id),
            );
        });
    }

    #[gpui::test]
    async fn test_nonempty_thread_metadata_preserved_when_thread_released(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, None::<&Path>, cx).await;
        let connection = StubAgentConnection::new();

        let (panel, mut vcx) = setup_panel_with_project(project, cx);
        crate::test_support::open_thread_with_connection(&panel, connection, &mut vcx);

        let session_id = crate::test_support::active_session_id(&panel, &vcx);
        let thread = panel.read_with(&vcx, |panel, cx| panel.active_agent_thread(cx).unwrap());

        thread.update_in(&mut vcx, |thread, _window, cx| {
            thread.push_user_content_block(None, "Hello".into(), cx);
        });
        vcx.run_until_parked();

        cx.read(|cx| {
            let store = ThreadMetadataStore::global(cx).read(cx);
            assert_eq!(store.entry_ids().count(), 1);
            assert!(store.entry_by_session(&session_id).is_some());
        });

        // Dropping the panel releases the ConversationView and its thread.
        drop(panel);
        cx.update(|_| {});
        cx.run_until_parked();

        cx.read(|cx| {
            let store = ThreadMetadataStore::global(cx).read(cx);
            assert_eq!(store.entry_ids().count(), 1);
            assert!(store.entry_by_session(&session_id).is_some());
        });
    }

    #[gpui::test]
    async fn test_threads_without_project_association_are_archived_by_default(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project_without_worktree = Project::test(fs.clone(), None::<&Path>, cx).await;
        let project_with_worktree = Project::test(fs, [Path::new("/project-a")], cx).await;

        // Thread in project without worktree
        let (panel_no_wt, mut vcx_no_wt) = setup_panel_with_project(project_without_worktree, cx);
        crate::test_support::open_thread_with_connection(
            &panel_no_wt,
            StubAgentConnection::new(),
            &mut vcx_no_wt,
        );
        let thread_no_wt = panel_no_wt.read_with(&vcx_no_wt, |panel, cx| {
            panel.active_agent_thread(cx).unwrap()
        });
        thread_no_wt.update_in(&mut vcx_no_wt, |thread, _window, cx| {
            thread.push_user_content_block(None, "content".into(), cx);
            thread.set_title("No Project Thread".into(), cx).detach();
        });
        vcx_no_wt.run_until_parked();
        let session_without_worktree =
            crate::test_support::active_session_id(&panel_no_wt, &vcx_no_wt);

        // Thread in project with worktree
        let (panel_wt, mut vcx_wt) = setup_panel_with_project(project_with_worktree, cx);
        crate::test_support::open_thread_with_connection(
            &panel_wt,
            StubAgentConnection::new(),
            &mut vcx_wt,
        );
        let thread_wt =
            panel_wt.read_with(&vcx_wt, |panel, cx| panel.active_agent_thread(cx).unwrap());
        thread_wt.update_in(&mut vcx_wt, |thread, _window, cx| {
            thread.push_user_content_block(None, "content".into(), cx);
            thread.set_title("Project Thread".into(), cx).detach();
        });
        vcx_wt.run_until_parked();
        let session_with_worktree = crate::test_support::active_session_id(&panel_wt, &vcx_wt);

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let without_worktree = store
                .entry_by_session(&session_without_worktree)
                .expect("missing metadata for thread without project association");
            assert!(without_worktree.folder_paths().is_empty());
            assert!(
                without_worktree.archived,
                "expected thread without project association to be archived"
            );

            let with_worktree = store
                .entry_by_session(&session_with_worktree)
                .expect("missing metadata for thread with project association");
            assert_eq!(
                *with_worktree.folder_paths(),
                PathList::new(&[Path::new("/project-a")])
            );
            assert!(
                !with_worktree.archived,
                "expected thread with project association to remain unarchived"
            );
        });
    }

    #[gpui::test]
    async fn test_subagent_threads_excluded_from_sidebar_metadata(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, None::<&Path>, cx).await;
        let connection = Rc::new(StubAgentConnection::new());

        // Create a regular (non-subagent) thread through the panel.
        let (panel, mut vcx) = setup_panel_with_project(project.clone(), cx);
        crate::test_support::open_thread_with_connection(&panel, (*connection).clone(), &mut vcx);

        let regular_thread =
            panel.read_with(&vcx, |panel, cx| panel.active_agent_thread(cx).unwrap());
        let regular_session_id = regular_thread.read_with(&vcx, |t, _| t.session_id().clone());

        regular_thread.update_in(&mut vcx, |thread, _window, cx| {
            thread.push_user_content_block(None, "content".into(), cx);
            thread.set_title("Regular Thread".into(), cx).detach();
        });
        vcx.run_until_parked();

        // Create a standalone subagent AcpThread (not wrapped in a
        // ConversationView). The ThreadMetadataStore only observes
        // ConversationView events, so this thread's events should
        // have no effect on sidebar metadata.
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

        cx.update(|cx| {
            subagent_thread.update(cx, |thread, cx| {
                thread
                    .set_title("Subagent Thread Title".into(), cx)
                    .detach();
            });
        });
        cx.run_until_parked();

        // Only the regular thread should appear in sidebar metadata.
        // The subagent thread is excluded because the metadata store
        // only observes ConversationView events.
        let list = cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.read(cx).entries().cloned().collect::<Vec<_>>()
        });

        assert_eq!(
            list.len(),
            1,
            "Expected only the regular thread in sidebar metadata, \
             but found {} entries (subagent threads are leaking into the sidebar)",
            list.len(),
        );
        assert_eq!(list[0].session_id.as_ref().unwrap(), &regular_session_id);
        assert_eq!(list[0].display_title(), "Regular Thread");
    }

    #[test]
    fn test_dedup_db_operations_keeps_latest_operation_for_session() {
        let now = Utc::now();

        let meta = make_metadata("session-1", "First Thread", now, PathList::default());
        let thread_id = meta.thread_id;
        let operations = vec![DbOperation::Upsert(meta), DbOperation::Delete(thread_id)];

        let deduped = ThreadMetadataStore::dedup_db_operations(operations);

        assert_eq!(deduped.len(), 1);
        assert_eq!(deduped[0], DbOperation::Delete(thread_id));
    }

    #[test]
    fn test_dedup_db_operations_keeps_latest_insert_for_same_session() {
        let now = Utc::now();
        let later = now + chrono::Duration::seconds(1);

        let old_metadata = make_metadata("session-1", "Old Title", now, PathList::default());
        let shared_thread_id = old_metadata.thread_id;
        let new_metadata = ThreadMetadata {
            thread_id: shared_thread_id,
            ..make_metadata("session-1", "New Title", later, PathList::default())
        };

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
        let thread_id = metadata.thread_id;

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

            let path_entries: Vec<_> = store
                .entries_for_path(&paths)
                .filter_map(|e| e.session_id.as_ref().map(|s| s.0.to_string()))
                .collect();
            assert_eq!(path_entries, vec!["session-1"]);

            assert_eq!(store.archived_entries().count(), 0);
        });

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.archive(thread_id, None, cx);
            });
        });

        // Thread 1 should now be archived
        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let path_entries: Vec<_> = store
                .entries_for_path(&paths)
                .filter_map(|e| e.session_id.as_ref().map(|s| s.0.to_string()))
                .collect();
            assert!(path_entries.is_empty());

            let archived: Vec<_> = store.archived_entries().collect();
            assert_eq!(archived.len(), 1);
            assert_eq!(
                archived[0].session_id.as_ref().unwrap().0.as_ref(),
                "session-1"
            );
            assert!(archived[0].archived);
        });

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.unarchive(thread_id, cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let path_entries: Vec<_> = store
                .entries_for_path(&paths)
                .filter_map(|e| e.session_id.as_ref().map(|s| s.0.to_string()))
                .collect();
            assert_eq!(path_entries, vec!["session-1"]);

            assert_eq!(store.archived_entries().count(), 0);
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
        let session2_thread_id = metadata2.thread_id;

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
                store.archive(session2_thread_id, None, cx);
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let path_entries: Vec<_> = store
                .entries_for_path(&paths)
                .filter_map(|e| e.session_id.as_ref().map(|s| s.0.to_string()))
                .collect();
            assert_eq!(path_entries, vec!["session-1"]);

            assert_eq!(store.entries().count(), 2);

            let archived: Vec<_> = store
                .archived_entries()
                .filter_map(|e| e.session_id.as_ref().map(|s| s.0.to_string()))
                .collect();
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

            assert_eq!(store.entries().count(), 3);
            assert!(
                store
                    .entry_by_session(&acp::SessionId::new("session-1"))
                    .is_some()
            );
            assert!(
                store
                    .entry_by_session(&acp::SessionId::new("session-2"))
                    .is_some()
            );
            assert!(
                store
                    .entry_by_session(&acp::SessionId::new("session-3"))
                    .is_some()
            );

            assert_eq!(store.entry_ids().count(), 3);
        });
    }

    #[gpui::test]
    async fn test_archived_flag_persists_across_reload(cx: &mut TestAppContext) {
        init_test(cx);

        let paths = PathList::new(&[Path::new("/project-a")]);
        let now = Utc::now();
        let metadata = make_metadata("session-1", "Thread 1", now, paths.clone());
        let thread_id = metadata.thread_id;

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
                store.archive(thread_id, None, cx);
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
                .entry_by_session(&acp::SessionId::new("session-1"))
                .expect("thread should exist after reload");
            assert!(thread.archived);

            let path_entries: Vec<_> = store
                .entries_for_path(&paths)
                .filter_map(|e| e.session_id.as_ref().map(|s| s.0.to_string()))
                .collect();
            assert!(path_entries.is_empty());

            let archived: Vec<_> = store
                .archived_entries()
                .filter_map(|e| e.session_id.as_ref().map(|s| s.0.to_string()))
                .collect();
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
                store.archive(ThreadId::new(), None, cx);
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
        let thread_id = metadata.thread_id;

        cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.save(metadata.clone(), cx);
                store.archive(thread_id, None, cx);
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

    #[gpui::test]
    async fn test_create_and_retrieve_archived_worktree(cx: &mut TestAppContext) {
        init_test(cx);
        let store = cx.update(|cx| ThreadMetadataStore::global(cx));

        let id = store
            .read_with(cx, |store, cx| {
                store.create_archived_worktree(
                    "/tmp/worktree".to_string(),
                    "/home/user/repo".to_string(),
                    Some("feature-branch".to_string()),
                    "staged_aaa".to_string(),
                    "unstaged_bbb".to_string(),
                    "original_000".to_string(),
                    cx,
                )
            })
            .await
            .unwrap();

        let thread_id_1 = ThreadId::new();

        store
            .read_with(cx, |store, cx| {
                store.link_thread_to_archived_worktree(thread_id_1, id, cx)
            })
            .await
            .unwrap();

        let worktrees = store
            .read_with(cx, |store, cx| {
                store.get_archived_worktrees_for_thread(thread_id_1, cx)
            })
            .await
            .unwrap();

        assert_eq!(worktrees.len(), 1);
        let wt = &worktrees[0];
        assert_eq!(wt.id, id);
        assert_eq!(wt.worktree_path, PathBuf::from("/tmp/worktree"));
        assert_eq!(wt.main_repo_path, PathBuf::from("/home/user/repo"));
        assert_eq!(wt.branch_name.as_deref(), Some("feature-branch"));
        assert_eq!(wt.staged_commit_hash, "staged_aaa");
        assert_eq!(wt.unstaged_commit_hash, "unstaged_bbb");
        assert_eq!(wt.original_commit_hash, "original_000");
    }

    #[gpui::test]
    async fn test_delete_archived_worktree(cx: &mut TestAppContext) {
        init_test(cx);
        let store = cx.update(|cx| ThreadMetadataStore::global(cx));

        let id = store
            .read_with(cx, |store, cx| {
                store.create_archived_worktree(
                    "/tmp/worktree".to_string(),
                    "/home/user/repo".to_string(),
                    Some("main".to_string()),
                    "deadbeef".to_string(),
                    "deadbeef".to_string(),
                    "original_000".to_string(),
                    cx,
                )
            })
            .await
            .unwrap();

        let thread_id_1 = ThreadId::new();

        store
            .read_with(cx, |store, cx| {
                store.link_thread_to_archived_worktree(thread_id_1, id, cx)
            })
            .await
            .unwrap();

        store
            .read_with(cx, |store, cx| store.delete_archived_worktree(id, cx))
            .await
            .unwrap();

        let worktrees = store
            .read_with(cx, |store, cx| {
                store.get_archived_worktrees_for_thread(thread_id_1, cx)
            })
            .await
            .unwrap();
        assert!(worktrees.is_empty());
    }

    #[gpui::test]
    async fn test_link_multiple_threads_to_archived_worktree(cx: &mut TestAppContext) {
        init_test(cx);
        let store = cx.update(|cx| ThreadMetadataStore::global(cx));

        let id = store
            .read_with(cx, |store, cx| {
                store.create_archived_worktree(
                    "/tmp/worktree".to_string(),
                    "/home/user/repo".to_string(),
                    None,
                    "abc123".to_string(),
                    "abc123".to_string(),
                    "original_000".to_string(),
                    cx,
                )
            })
            .await
            .unwrap();

        let thread_id_1 = ThreadId::new();
        let thread_id_2 = ThreadId::new();

        store
            .read_with(cx, |store, cx| {
                store.link_thread_to_archived_worktree(thread_id_1, id, cx)
            })
            .await
            .unwrap();

        store
            .read_with(cx, |store, cx| {
                store.link_thread_to_archived_worktree(thread_id_2, id, cx)
            })
            .await
            .unwrap();

        let wt1 = store
            .read_with(cx, |store, cx| {
                store.get_archived_worktrees_for_thread(thread_id_1, cx)
            })
            .await
            .unwrap();

        let wt2 = store
            .read_with(cx, |store, cx| {
                store.get_archived_worktrees_for_thread(thread_id_2, cx)
            })
            .await
            .unwrap();

        assert_eq!(wt1.len(), 1);
        assert_eq!(wt2.len(), 1);
        assert_eq!(wt1[0].id, wt2[0].id);
    }

    #[gpui::test]
    async fn test_complete_worktree_restore_multiple_paths(cx: &mut TestAppContext) {
        init_test(cx);
        let store = cx.update(|cx| ThreadMetadataStore::global(cx));

        let original_paths = PathList::new(&[
            Path::new("/projects/worktree-a"),
            Path::new("/projects/worktree-b"),
            Path::new("/other/unrelated"),
        ]);
        let meta = make_metadata("session-multi", "Multi Thread", Utc::now(), original_paths);
        let thread_id = meta.thread_id;

        store.update(cx, |store, cx| {
            store.save(meta, cx);
        });

        let replacements = vec![
            (
                PathBuf::from("/projects/worktree-a"),
                PathBuf::from("/restored/worktree-a"),
            ),
            (
                PathBuf::from("/projects/worktree-b"),
                PathBuf::from("/restored/worktree-b"),
            ),
        ];

        store.update(cx, |store, cx| {
            store.complete_worktree_restore(thread_id, &replacements, cx);
        });

        let entry = store.read_with(cx, |store, _cx| store.entry(thread_id).cloned());
        let entry = entry.unwrap();
        let paths = entry.folder_paths().paths();
        assert_eq!(paths.len(), 3);
        assert!(paths.contains(&PathBuf::from("/restored/worktree-a")));
        assert!(paths.contains(&PathBuf::from("/restored/worktree-b")));
        assert!(paths.contains(&PathBuf::from("/other/unrelated")));
    }

    #[gpui::test]
    async fn test_complete_worktree_restore_preserves_unmatched_paths(cx: &mut TestAppContext) {
        init_test(cx);
        let store = cx.update(|cx| ThreadMetadataStore::global(cx));

        let original_paths =
            PathList::new(&[Path::new("/projects/worktree-a"), Path::new("/other/path")]);
        let meta = make_metadata("session-partial", "Partial", Utc::now(), original_paths);
        let thread_id = meta.thread_id;

        store.update(cx, |store, cx| {
            store.save(meta, cx);
        });

        let replacements = vec![
            (
                PathBuf::from("/projects/worktree-a"),
                PathBuf::from("/new/worktree-a"),
            ),
            (
                PathBuf::from("/nonexistent/path"),
                PathBuf::from("/should/not/appear"),
            ),
        ];

        store.update(cx, |store, cx| {
            store.complete_worktree_restore(thread_id, &replacements, cx);
        });

        let entry = store.read_with(cx, |store, _cx| store.entry(thread_id).cloned());
        let entry = entry.unwrap();
        let paths = entry.folder_paths().paths();
        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&PathBuf::from("/new/worktree-a")));
        assert!(paths.contains(&PathBuf::from("/other/path")));
        assert!(!paths.contains(&PathBuf::from("/should/not/appear")));
    }

    #[gpui::test]
    async fn test_update_restored_worktree_paths_multiple(cx: &mut TestAppContext) {
        init_test(cx);
        let store = cx.update(|cx| ThreadMetadataStore::global(cx));

        let original_paths = PathList::new(&[
            Path::new("/projects/worktree-a"),
            Path::new("/projects/worktree-b"),
            Path::new("/other/unrelated"),
        ]);
        let meta = make_metadata("session-multi", "Multi Thread", Utc::now(), original_paths);
        let thread_id = meta.thread_id;

        store.update(cx, |store, cx| {
            store.save(meta, cx);
        });

        let replacements = vec![
            (
                PathBuf::from("/projects/worktree-a"),
                PathBuf::from("/restored/worktree-a"),
            ),
            (
                PathBuf::from("/projects/worktree-b"),
                PathBuf::from("/restored/worktree-b"),
            ),
        ];

        store.update(cx, |store, cx| {
            store.update_restored_worktree_paths(thread_id, &replacements, cx);
        });

        let entry = store.read_with(cx, |store, _cx| store.entry(thread_id).cloned());
        let entry = entry.unwrap();
        let paths = entry.folder_paths().paths();
        assert_eq!(paths.len(), 3);
        assert!(paths.contains(&PathBuf::from("/restored/worktree-a")));
        assert!(paths.contains(&PathBuf::from("/restored/worktree-b")));
        assert!(paths.contains(&PathBuf::from("/other/unrelated")));
    }

    #[gpui::test]
    async fn test_update_restored_worktree_paths_preserves_unmatched(cx: &mut TestAppContext) {
        init_test(cx);
        let store = cx.update(|cx| ThreadMetadataStore::global(cx));

        let original_paths =
            PathList::new(&[Path::new("/projects/worktree-a"), Path::new("/other/path")]);
        let meta = make_metadata("session-partial", "Partial", Utc::now(), original_paths);
        let thread_id = meta.thread_id;

        store.update(cx, |store, cx| {
            store.save(meta, cx);
        });

        let replacements = vec![
            (
                PathBuf::from("/projects/worktree-a"),
                PathBuf::from("/new/worktree-a"),
            ),
            (
                PathBuf::from("/nonexistent/path"),
                PathBuf::from("/should/not/appear"),
            ),
        ];

        store.update(cx, |store, cx| {
            store.update_restored_worktree_paths(thread_id, &replacements, cx);
        });

        let entry = store.read_with(cx, |store, _cx| store.entry(thread_id).cloned());
        let entry = entry.unwrap();
        let paths = entry.folder_paths().paths();
        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&PathBuf::from("/new/worktree-a")));
        assert!(paths.contains(&PathBuf::from("/other/path")));
        assert!(!paths.contains(&PathBuf::from("/should/not/appear")));
    }

    #[gpui::test]
    async fn test_multiple_archived_worktrees_per_thread(cx: &mut TestAppContext) {
        init_test(cx);
        let store = cx.update(|cx| ThreadMetadataStore::global(cx));

        let id1 = store
            .read_with(cx, |store, cx| {
                store.create_archived_worktree(
                    "/projects/worktree-a".to_string(),
                    "/home/user/repo".to_string(),
                    Some("branch-a".to_string()),
                    "staged_a".to_string(),
                    "unstaged_a".to_string(),
                    "original_000".to_string(),
                    cx,
                )
            })
            .await
            .unwrap();

        let id2 = store
            .read_with(cx, |store, cx| {
                store.create_archived_worktree(
                    "/projects/worktree-b".to_string(),
                    "/home/user/repo".to_string(),
                    Some("branch-b".to_string()),
                    "staged_b".to_string(),
                    "unstaged_b".to_string(),
                    "original_000".to_string(),
                    cx,
                )
            })
            .await
            .unwrap();

        let thread_id_1 = ThreadId::new();

        store
            .read_with(cx, |store, cx| {
                store.link_thread_to_archived_worktree(thread_id_1, id1, cx)
            })
            .await
            .unwrap();

        store
            .read_with(cx, |store, cx| {
                store.link_thread_to_archived_worktree(thread_id_1, id2, cx)
            })
            .await
            .unwrap();

        let worktrees = store
            .read_with(cx, |store, cx| {
                store.get_archived_worktrees_for_thread(thread_id_1, cx)
            })
            .await
            .unwrap();

        assert_eq!(worktrees.len(), 2);

        let paths: Vec<&Path> = worktrees
            .iter()
            .map(|w| w.worktree_path.as_path())
            .collect();
        assert!(paths.contains(&Path::new("/projects/worktree-a")));
        assert!(paths.contains(&Path::new("/projects/worktree-b")));
    }

    // ── Migration tests ────────────────────────────────────────────────

    #[test]
    fn test_thread_id_primary_key_migration_backfills_null_thread_ids() {
        use db::sqlez::connection::Connection;

        let connection =
            Connection::open_memory(Some("test_thread_id_pk_migration_backfills_nulls"));

        // Run migrations 0-6 (the old schema, before the thread_id PK migration).
        let old_migrations: &[&str] = &ThreadMetadataDb::MIGRATIONS[..7];
        connection
            .migrate(ThreadMetadataDb::NAME, old_migrations, &mut |_, _, _| false)
            .expect("old migrations should succeed");

        // Insert rows: one with a thread_id, two without.
        connection
            .exec(
                "INSERT INTO sidebar_threads \
                 (session_id, title, updated_at, thread_id) \
                 VALUES ('has-tid', 'Has ThreadId', '2025-01-01T00:00:00Z', X'0102030405060708090A0B0C0D0E0F10')",
            )
            .unwrap()()
            .unwrap();
        connection
            .exec(
                "INSERT INTO sidebar_threads \
                 (session_id, title, updated_at) \
                 VALUES ('no-tid-1', 'No ThreadId 1', '2025-01-02T00:00:00Z')",
            )
            .unwrap()()
        .unwrap();
        connection
            .exec(
                "INSERT INTO sidebar_threads \
                 (session_id, title, updated_at) \
                 VALUES ('no-tid-2', 'No ThreadId 2', '2025-01-03T00:00:00Z')",
            )
            .unwrap()()
        .unwrap();

        // Set up archived_git_worktrees + thread_archived_worktrees rows
        // referencing the session without a thread_id.
        connection
            .exec(
                "INSERT INTO archived_git_worktrees \
                 (id, worktree_path, main_repo_path, staged_commit_hash, unstaged_commit_hash, original_commit_hash) \
                 VALUES (1, '/wt', '/main', 'abc', 'def', '000')",
            )
            .unwrap()()
            .unwrap();
        connection
            .exec(
                "INSERT INTO thread_archived_worktrees \
                 (session_id, archived_worktree_id) \
                 VALUES ('no-tid-1', 1)",
            )
            .unwrap()()
        .unwrap();

        // Run all migrations (0-7). sqlez skips 0-6 and runs only migration 7.
        connection
            .migrate(
                ThreadMetadataDb::NAME,
                ThreadMetadataDb::MIGRATIONS,
                &mut |_, _, _| false,
            )
            .expect("new migration should succeed");

        // All 3 rows should survive with non-NULL thread_ids.
        let count: i64 = connection
            .select_row_bound::<(), i64>("SELECT COUNT(*) FROM sidebar_threads")
            .unwrap()(())
        .unwrap()
        .unwrap();
        assert_eq!(count, 3, "all 3 rows should survive the migration");

        let null_count: i64 = connection
            .select_row_bound::<(), i64>(
                "SELECT COUNT(*) FROM sidebar_threads WHERE thread_id IS NULL",
            )
            .unwrap()(())
        .unwrap()
        .unwrap();
        assert_eq!(
            null_count, 0,
            "no rows should have NULL thread_id after migration"
        );

        // The row that already had a thread_id should keep its original value.
        let original_tid: Vec<u8> = connection
            .select_row_bound::<&str, Vec<u8>>(
                "SELECT thread_id FROM sidebar_threads WHERE session_id = ?",
            )
            .unwrap()("has-tid")
        .unwrap()
        .unwrap();
        assert_eq!(
            original_tid,
            vec![
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
                0x0F, 0x10
            ],
            "pre-existing thread_id should be preserved"
        );

        // The two rows that had NULL thread_ids should now have distinct non-empty blobs.
        let generated_tid_1: Vec<u8> = connection
            .select_row_bound::<&str, Vec<u8>>(
                "SELECT thread_id FROM sidebar_threads WHERE session_id = ?",
            )
            .unwrap()("no-tid-1")
        .unwrap()
        .unwrap();
        let generated_tid_2: Vec<u8> = connection
            .select_row_bound::<&str, Vec<u8>>(
                "SELECT thread_id FROM sidebar_threads WHERE session_id = ?",
            )
            .unwrap()("no-tid-2")
        .unwrap()
        .unwrap();
        assert_eq!(
            generated_tid_1.len(),
            16,
            "generated thread_id should be 16 bytes"
        );
        assert_eq!(
            generated_tid_2.len(),
            16,
            "generated thread_id should be 16 bytes"
        );
        assert_ne!(
            generated_tid_1, generated_tid_2,
            "each generated thread_id should be unique"
        );

        // The thread_archived_worktrees join row should have migrated
        // using the backfilled thread_id from the session without a
        // pre-existing thread_id.
        let archived_count: i64 = connection
            .select_row_bound::<(), i64>("SELECT COUNT(*) FROM thread_archived_worktrees")
            .unwrap()(())
        .unwrap()
        .unwrap();
        assert_eq!(
            archived_count, 1,
            "thread_archived_worktrees row should survive migration"
        );

        // The thread_archived_worktrees row should reference the
        // backfilled thread_id of the 'no-tid-1' session.
        let archived_tid: Vec<u8> = connection
            .select_row_bound::<(), Vec<u8>>(
                "SELECT thread_id FROM thread_archived_worktrees LIMIT 1",
            )
            .unwrap()(())
        .unwrap()
        .unwrap();
        assert_eq!(
            archived_tid, generated_tid_1,
            "thread_archived_worktrees should reference the backfilled thread_id"
        );
    }

    // ── ThreadWorktreePaths tests ──────────────────────────────────────

    /// Helper to build a `ThreadWorktreePaths` from (main, folder) pairs.
    fn make_worktree_paths(pairs: &[(&str, &str)]) -> WorktreePaths {
        let (mains, folders): (Vec<&Path>, Vec<&Path>) = pairs
            .iter()
            .map(|(m, f)| (Path::new(*m), Path::new(*f)))
            .unzip();
        WorktreePaths::from_path_lists(PathList::new(&mains), PathList::new(&folders)).unwrap()
    }

    #[test]
    fn test_thread_worktree_paths_full_add_then_remove_cycle() {
        // Full scenario from the issue:
        //   1. Start with linked worktree selectric → zed
        //   2. Add cloud
        //   3. Remove zed

        let mut paths = make_worktree_paths(&[("/projects/zed", "/worktrees/selectric/zed")]);

        // Step 2: add cloud
        paths.add_path(Path::new("/projects/cloud"), Path::new("/projects/cloud"));

        assert_eq!(paths.ordered_pairs().count(), 2);
        assert_eq!(
            paths.folder_path_list(),
            &PathList::new(&[
                Path::new("/worktrees/selectric/zed"),
                Path::new("/projects/cloud"),
            ])
        );
        assert_eq!(
            paths.main_worktree_path_list(),
            &PathList::new(&[Path::new("/projects/zed"), Path::new("/projects/cloud"),])
        );

        // Step 3: remove zed
        paths.remove_main_path(Path::new("/projects/zed"));

        assert_eq!(paths.ordered_pairs().count(), 1);
        assert_eq!(
            paths.folder_path_list(),
            &PathList::new(&[Path::new("/projects/cloud")])
        );
        assert_eq!(
            paths.main_worktree_path_list(),
            &PathList::new(&[Path::new("/projects/cloud")])
        );
    }

    #[test]
    fn test_thread_worktree_paths_add_is_idempotent() {
        let mut paths = make_worktree_paths(&[("/projects/zed", "/projects/zed")]);

        paths.add_path(Path::new("/projects/zed"), Path::new("/projects/zed"));

        assert_eq!(paths.ordered_pairs().count(), 1);
    }

    #[test]
    fn test_thread_worktree_paths_remove_nonexistent_is_noop() {
        let mut paths = make_worktree_paths(&[("/projects/zed", "/worktrees/selectric/zed")]);

        paths.remove_main_path(Path::new("/projects/nonexistent"));

        assert_eq!(paths.ordered_pairs().count(), 1);
    }

    #[test]
    fn test_thread_worktree_paths_from_path_lists_preserves_association() {
        let folder = PathList::new(&[
            Path::new("/worktrees/selectric/zed"),
            Path::new("/projects/cloud"),
        ]);
        let main = PathList::new(&[Path::new("/projects/zed"), Path::new("/projects/cloud")]);

        let paths = WorktreePaths::from_path_lists(main, folder).unwrap();

        let pairs: Vec<_> = paths
            .ordered_pairs()
            .map(|(m, f)| (m.clone(), f.clone()))
            .collect();
        assert_eq!(pairs.len(), 2);
        assert!(pairs.contains(&(
            PathBuf::from("/projects/zed"),
            PathBuf::from("/worktrees/selectric/zed")
        )));
        assert!(pairs.contains(&(
            PathBuf::from("/projects/cloud"),
            PathBuf::from("/projects/cloud")
        )));
    }

    #[test]
    fn test_thread_worktree_paths_main_deduplicates_linked_worktrees() {
        // Two linked worktrees of the same main repo: the main_worktree_path_list
        // deduplicates because PathList stores unique sorted paths, but
        // ordered_pairs still has both entries.
        let paths = make_worktree_paths(&[
            ("/projects/zed", "/worktrees/selectric/zed"),
            ("/projects/zed", "/worktrees/feature/zed"),
        ]);

        // main_worktree_path_list has the duplicate main path twice
        // (PathList keeps all entries from its input)
        assert_eq!(paths.ordered_pairs().count(), 2);
        assert_eq!(
            paths.folder_path_list(),
            &PathList::new(&[
                Path::new("/worktrees/selectric/zed"),
                Path::new("/worktrees/feature/zed"),
            ])
        );
        assert_eq!(
            paths.main_worktree_path_list(),
            &PathList::new(&[Path::new("/projects/zed"), Path::new("/projects/zed"),])
        );
    }

    #[test]
    fn test_thread_worktree_paths_mismatched_lengths_returns_error() {
        let folder = PathList::new(&[
            Path::new("/worktrees/selectric/zed"),
            Path::new("/projects/cloud"),
        ]);
        let main = PathList::new(&[Path::new("/projects/zed")]);

        let result = WorktreePaths::from_path_lists(main, folder);
        assert!(result.is_err());
    }
}
