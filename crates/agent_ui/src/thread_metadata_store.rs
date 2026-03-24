use std::{path::Path, sync::Arc};

use acp_thread::AgentSessionInfo;
use agent::{ThreadStore, ZED_AGENT_ID};
use agent_client_protocol as acp;
use anyhow::{Context as _, Result};
use chrono::{DateTime, Utc};
use collections::HashMap;
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
    SidebarThreadMetadataStore::init_global(cx);

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
/// We migrate the last 10 threads per project and skip threads that do not have a project.
///
/// TODO: Remove this after N weeks of shipping the sidebar
fn migrate_thread_metadata(cx: &mut App) {
    const MAX_MIGRATED_THREADS_PER_PROJECT: usize = 10;

    let store = SidebarThreadMetadataStore::global(cx);
    let db = store.read(cx).db.clone();

    cx.spawn(async move |cx| {
        if !db.is_empty()? {
            return Ok::<(), anyhow::Error>(());
        }

        let metadata = store.read_with(cx, |_store, app| {
            let mut migrated_threads_per_project = HashMap::default();

            ThreadStore::global(app)
                .read(app)
                .entries()
                .filter_map(|entry| {
                    if entry.folder_paths.is_empty() {
                        return None;
                    }

                    let migrated_thread_count = migrated_threads_per_project
                        .entry(entry.folder_paths.clone())
                        .or_insert(0);
                    if *migrated_thread_count >= MAX_MIGRATED_THREADS_PER_PROJECT {
                        return None;
                    }
                    *migrated_thread_count += 1;

                    Some(ThreadMetadata {
                        session_id: entry.id,
                        agent_id: None,
                        title: entry.title,
                        updated_at: entry.updated_at,
                        created_at: entry.created_at,
                        folder_paths: entry.folder_paths,
                    })
                })
                .collect::<Vec<_>>()
        });

        log::info!("Migrating {} thread store entries", metadata.len());

        // Manually save each entry to the database and call reload, otherwise
        // we'll end up triggering lots of reloads after each save
        for entry in metadata {
            db.save(entry).await?;
        }

        log::info!("Finished migrating thread store entries");

        let _ = store.update(cx, |store, cx| store.reload(cx));
        Ok(())
    })
    .detach_and_log_err(cx);
}

struct GlobalThreadMetadataStore(Entity<SidebarThreadMetadataStore>);
impl Global for GlobalThreadMetadataStore {}

/// Lightweight metadata for any thread (native or ACP), enough to populate
/// the sidebar list and route to the correct load path when clicked.
#[derive(Debug, Clone)]
pub struct ThreadMetadata {
    pub session_id: acp::SessionId,
    /// `None` for native Zed threads, `Some("claude-code")` etc. for ACP agents.
    pub agent_id: Option<AgentId>,
    pub title: SharedString,
    pub updated_at: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
    pub folder_paths: PathList,
}

impl ThreadMetadata {
    pub fn from_session_info(agent_id: AgentId, session: &AgentSessionInfo) -> Self {
        let session_id = session.session_id.clone();
        let title = session.title.clone().unwrap_or_default();
        let updated_at = session.updated_at.unwrap_or_else(|| Utc::now());
        let created_at = session.created_at.unwrap_or(updated_at);
        let folder_paths = session.work_dirs.clone().unwrap_or_default();
        let agent_id = if agent_id.as_ref() == ZED_AGENT_ID.as_ref() {
            None
        } else {
            Some(agent_id)
        };
        Self {
            session_id,
            agent_id,
            title,
            updated_at,
            created_at: Some(created_at),
            folder_paths,
        }
    }

    pub fn from_thread(thread: &Entity<acp_thread::AcpThread>, cx: &App) -> Self {
        let thread_ref = thread.read(cx);
        let session_id = thread_ref.session_id().clone();
        let title = thread_ref
            .title()
            .unwrap_or_else(|| DEFAULT_THREAD_TITLE.into());
        let updated_at = Utc::now();

        let agent_id = thread_ref.connection().agent_id();

        let agent_id = if agent_id.as_ref() == ZED_AGENT_ID.as_ref() {
            None
        } else {
            Some(agent_id)
        };

        let folder_paths = {
            let project = thread_ref.project().read(cx);
            let paths: Vec<Arc<Path>> = project
                .visible_worktrees(cx)
                .map(|worktree| worktree.read(cx).abs_path())
                .collect();
            PathList::new(&paths)
        };

        Self {
            session_id,
            agent_id,
            title,
            created_at: Some(updated_at), // handled by db `ON CONFLICT`
            updated_at,
            folder_paths,
        }
    }
}

/// The store holds all metadata needed to show threads in the sidebar.
/// Effectively, all threads stored in here are "non-archived".
///
/// Automatically listens to AcpThread events and updates metadata if it has changed.
pub struct SidebarThreadMetadataStore {
    db: ThreadMetadataDb,
    threads: Vec<ThreadMetadata>,
    threads_by_paths: HashMap<PathList, Vec<ThreadMetadata>>,
    reload_task: Option<Shared<Task<()>>>,
    session_subscriptions: HashMap<acp::SessionId, Subscription>,
}

impl SidebarThreadMetadataStore {
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

    pub fn entries(&self) -> impl Iterator<Item = ThreadMetadata> + '_ {
        self.threads.iter().cloned()
    }

    pub fn entry_ids(&self) -> impl Iterator<Item = acp::SessionId> + '_ {
        self.threads.iter().map(|thread| thread.session_id.clone())
    }

    pub fn entries_for_path(
        &self,
        path_list: &PathList,
    ) -> impl Iterator<Item = ThreadMetadata> + '_ {
        self.threads_by_paths
            .get(path_list)
            .into_iter()
            .flatten()
            .cloned()
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
                            .push(row.clone());
                        this.threads.push(row);
                    }

                    cx.notify();
                })
                .ok();
            })
            .shared();
        self.reload_task = Some(reload_task.clone());
        reload_task
    }

    pub fn save(&mut self, metadata: ThreadMetadata, cx: &mut Context<Self>) -> Task<Result<()>> {
        if !cx.has_flag::<AgentV2FeatureFlag>() {
            return Task::ready(Ok(()));
        }

        let db = self.db.clone();
        cx.spawn(async move |this, cx| {
            db.save(metadata).await?;
            let reload_task = this.update(cx, |this, cx| this.reload(cx))?;
            reload_task.await;
            Ok(())
        })
    }

    pub fn delete(
        &mut self,
        session_id: acp::SessionId,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if !cx.has_flag::<AgentV2FeatureFlag>() {
            return Task::ready(Ok(()));
        }

        let db = self.db.clone();
        cx.spawn(async move |this, cx| {
            db.delete(session_id).await?;
            let reload_task = this.update(cx, |this, cx| this.reload(cx))?;
            reload_task.await;
            Ok(())
        })
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
                        .update(cx, |store, _cx| {
                            store.session_subscriptions.remove(thread.session_id());
                        })
                        .ok();
                }
            })
            .detach();

            weak_store
                .update(cx, |this, cx| {
                    let subscription = cx.subscribe(&thread_entity, Self::handle_thread_update);
                    this.session_subscriptions
                        .insert(thread.session_id().clone(), subscription);
                })
                .ok();
        })
        .detach();

        let mut this = Self {
            db,
            threads: Vec::new(),
            threads_by_paths: HashMap::default(),
            reload_task: None,
            session_subscriptions: HashMap::default(),
        };
        let _ = this.reload(cx);
        this
    }

    fn handle_thread_update(
        &mut self,
        thread: Entity<acp_thread::AcpThread>,
        event: &acp_thread::AcpThreadEvent,
        cx: &mut Context<Self>,
    ) {
        // Don't track subagent threads in the sidebar.
        if thread.read(cx).parent_session_id().is_some() {
            return;
        }

        match event {
            acp_thread::AcpThreadEvent::NewEntry
            | acp_thread::AcpThreadEvent::TitleUpdated
            | acp_thread::AcpThreadEvent::EntryUpdated(_)
            | acp_thread::AcpThreadEvent::EntriesRemoved(_)
            | acp_thread::AcpThreadEvent::ToolAuthorizationRequested(_)
            | acp_thread::AcpThreadEvent::ToolAuthorizationReceived(_)
            | acp_thread::AcpThreadEvent::Retry(_)
            | acp_thread::AcpThreadEvent::Stopped(_)
            | acp_thread::AcpThreadEvent::Error
            | acp_thread::AcpThreadEvent::LoadError(_)
            | acp_thread::AcpThreadEvent::Refusal => {
                let metadata = ThreadMetadata::from_thread(&thread, cx);
                self.save(metadata, cx).detach_and_log_err(cx);
            }
            _ => {}
        }
    }
}

impl Global for SidebarThreadMetadataStore {}

struct ThreadMetadataDb(ThreadSafeConnection);

impl Domain for ThreadMetadataDb {
    const NAME: &str = stringify!(ThreadMetadataDb);

    const MIGRATIONS: &[&str] = &[sql!(
        CREATE TABLE IF NOT EXISTS sidebar_threads(
            session_id TEXT PRIMARY KEY,
            agent_id TEXT,
            title TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            created_at TEXT,
            folder_paths TEXT,
            folder_paths_order TEXT
        ) STRICT;
    )];
}

db::static_connection!(ThreadMetadataDb, []);

impl ThreadMetadataDb {
    pub fn is_empty(&self) -> anyhow::Result<bool> {
        self.select::<i64>("SELECT COUNT(*) FROM sidebar_threads")?()
            .map(|counts| counts.into_iter().next().unwrap_or_default() == 0)
    }

    /// List all sidebar thread metadata, ordered by updated_at descending.
    pub fn list(&self) -> anyhow::Result<Vec<ThreadMetadata>> {
        self.select::<ThreadMetadata>(
            "SELECT session_id, agent_id, title, updated_at, created_at, folder_paths, folder_paths_order \
             FROM sidebar_threads \
             ORDER BY updated_at DESC"
        )?()
    }

    /// Upsert metadata for a thread.
    pub async fn save(&self, row: ThreadMetadata) -> anyhow::Result<()> {
        let id = row.session_id.0.clone();
        let agent_id = row.agent_id.as_ref().map(|id| id.0.to_string());
        let title = row.title.to_string();
        let updated_at = row.updated_at.to_rfc3339();
        let created_at = row.created_at.map(|dt| dt.to_rfc3339());
        let serialized = row.folder_paths.serialize();
        let (folder_paths, folder_paths_order) = if row.folder_paths.is_empty() {
            (None, None)
        } else {
            (Some(serialized.paths), Some(serialized.order))
        };

        self.write(move |conn| {
            let sql = "INSERT INTO sidebar_threads(session_id, agent_id, title, updated_at, created_at, folder_paths, folder_paths_order) \
                       VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) \
                       ON CONFLICT(session_id) DO UPDATE SET \
                           agent_id = excluded.agent_id, \
                           title = excluded.title, \
                           updated_at = excluded.updated_at, \
                           folder_paths = excluded.folder_paths, \
                           folder_paths_order = excluded.folder_paths_order";
            let mut stmt = Statement::prepare(conn, sql)?;
            let mut i = stmt.bind(&id, 1)?;
            i = stmt.bind(&agent_id, i)?;
            i = stmt.bind(&title, i)?;
            i = stmt.bind(&updated_at, i)?;
            i = stmt.bind(&created_at, i)?;
            i = stmt.bind(&folder_paths, i)?;
            stmt.bind(&folder_paths_order, i)?;
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
                agent_id: agent_id.map(|id| AgentId::new(id)),
                title: title.into(),
                updated_at,
                created_at,
                folder_paths,
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
            session_id: acp::SessionId::new(session_id),
            agent_id: None,
            title: title.to_string().into(),
            updated_at,
            created_at: Some(updated_at),
            folder_paths,
        }
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
            SidebarThreadMetadataStore::init_global(cx);
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = SidebarThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let entry_ids = store
                .entry_ids()
                .map(|session_id| session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(entry_ids, vec!["session-1", "session-2"]);

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
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            cx.update_flags(true, vec!["agent-v2".to_string()]);
            SidebarThreadMetadataStore::init_global(cx);
        });

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
            let store = SidebarThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.save(initial_metadata, cx).detach();
                store.save(second_metadata, cx).detach();
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = SidebarThreadMetadataStore::global(cx);
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
            let store = SidebarThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.save(moved_metadata, cx).detach();
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = SidebarThreadMetadataStore::global(cx);
            let store = store.read(cx);

            let entry_ids = store
                .entry_ids()
                .map(|session_id| session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(entry_ids, vec!["session-1", "session-2"]);

            let first_path_entries = store
                .entries_for_path(&first_paths)
                .map(|entry| entry.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert!(first_path_entries.is_empty());

            let second_path_entries = store
                .entries_for_path(&second_paths)
                .map(|entry| entry.session_id.0.to_string())
                .collect::<Vec<_>>();
            assert_eq!(second_path_entries, vec!["session-1", "session-2"]);
        });

        cx.update(|cx| {
            let store = SidebarThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.delete(acp::SessionId::new("session-2"), cx).detach();
            });
        });

        cx.run_until_parked();

        cx.update(|cx| {
            let store = SidebarThreadMetadataStore::global(cx);
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
    async fn test_migrate_thread_metadata(cx: &mut TestAppContext) {
        cx.update(|cx| {
            ThreadStore::init_global(cx);
            SidebarThreadMetadataStore::init_global(cx);
        });

        // Verify the cache is empty before migration
        let list = cx.update(|cx| {
            let store = SidebarThreadMetadataStore::global(cx);
            store.read(cx).entries().collect::<Vec<_>>()
        });
        assert_eq!(list.len(), 0);

        let project_a_paths = PathList::new(&[Path::new("/project-a")]);
        let project_b_paths = PathList::new(&[Path::new("/project-b")]);
        let now = Utc::now();

        for index in 0..12 {
            let updated_at = now + chrono::Duration::seconds(index as i64);
            let session_id = format!("project-a-session-{index}");
            let title = format!("Project A Thread {index}");

            let save_task = cx.update(|cx| {
                let thread_store = ThreadStore::global(cx);
                let session_id = session_id.clone();
                let title = title.clone();
                let project_a_paths = project_a_paths.clone();
                thread_store.update(cx, |store, cx| {
                    store.save_thread(
                        acp::SessionId::new(session_id),
                        make_db_thread(&title, updated_at),
                        project_a_paths,
                        cx,
                    )
                })
            });
            save_task.await.unwrap();
            cx.run_until_parked();
        }

        for index in 0..3 {
            let updated_at = now + chrono::Duration::seconds(100 + index as i64);
            let session_id = format!("project-b-session-{index}");
            let title = format!("Project B Thread {index}");

            let save_task = cx.update(|cx| {
                let thread_store = ThreadStore::global(cx);
                let session_id = session_id.clone();
                let title = title.clone();
                let project_b_paths = project_b_paths.clone();
                thread_store.update(cx, |store, cx| {
                    store.save_thread(
                        acp::SessionId::new(session_id),
                        make_db_thread(&title, updated_at),
                        project_b_paths,
                        cx,
                    )
                })
            });
            save_task.await.unwrap();
            cx.run_until_parked();
        }

        let save_projectless = cx.update(|cx| {
            let thread_store = ThreadStore::global(cx);
            thread_store.update(cx, |store, cx| {
                store.save_thread(
                    acp::SessionId::new("projectless-session"),
                    make_db_thread("Projectless Thread", now + chrono::Duration::seconds(200)),
                    PathList::default(),
                    cx,
                )
            })
        });
        save_projectless.await.unwrap();
        cx.run_until_parked();

        // Run migration
        cx.update(|cx| {
            migrate_thread_metadata(cx);
        });

        cx.run_until_parked();

        // Verify the metadata was migrated, limited to 10 per project, and
        // projectless threads were skipped.
        let list = cx.update(|cx| {
            let store = SidebarThreadMetadataStore::global(cx);
            store.read(cx).entries().collect::<Vec<_>>()
        });
        assert_eq!(list.len(), 13);

        assert!(
            list.iter()
                .all(|metadata| !metadata.folder_paths.is_empty())
        );
        assert!(
            list.iter()
                .all(|metadata| metadata.session_id.0.as_ref() != "projectless-session")
        );

        let project_a_entries = list
            .iter()
            .filter(|metadata| metadata.folder_paths == project_a_paths)
            .collect::<Vec<_>>();
        assert_eq!(project_a_entries.len(), 10);
        assert_eq!(
            project_a_entries
                .iter()
                .map(|metadata| metadata.session_id.0.as_ref())
                .collect::<Vec<_>>(),
            vec![
                "project-a-session-11",
                "project-a-session-10",
                "project-a-session-9",
                "project-a-session-8",
                "project-a-session-7",
                "project-a-session-6",
                "project-a-session-5",
                "project-a-session-4",
                "project-a-session-3",
                "project-a-session-2",
            ]
        );
        assert!(
            project_a_entries
                .iter()
                .all(|metadata| metadata.agent_id.is_none())
        );

        let project_b_entries = list
            .iter()
            .filter(|metadata| metadata.folder_paths == project_b_paths)
            .collect::<Vec<_>>();
        assert_eq!(project_b_entries.len(), 3);
        assert_eq!(
            project_b_entries
                .iter()
                .map(|metadata| metadata.session_id.0.as_ref())
                .collect::<Vec<_>>(),
            vec![
                "project-b-session-2",
                "project-b-session-1",
                "project-b-session-0",
            ]
        );
        assert!(
            project_b_entries
                .iter()
                .all(|metadata| metadata.agent_id.is_none())
        );
    }

    #[gpui::test]
    async fn test_migrate_thread_metadata_skips_when_data_exists(cx: &mut TestAppContext) {
        cx.update(|cx| {
            ThreadStore::init_global(cx);
            SidebarThreadMetadataStore::init_global(cx);
        });

        // Pre-populate the metadata store with existing data
        let existing_metadata = ThreadMetadata {
            session_id: acp::SessionId::new("existing-session"),
            agent_id: None,
            title: "Existing Thread".into(),
            updated_at: Utc::now(),
            created_at: Some(Utc::now()),
            folder_paths: PathList::default(),
        };

        cx.update(|cx| {
            let store = SidebarThreadMetadataStore::global(cx);
            store.update(cx, |store, cx| {
                store.save(existing_metadata, cx).detach();
            });
        });

        cx.run_until_parked();

        // Add an entry to native thread store that should NOT be migrated
        let save_task = cx.update(|cx| {
            let thread_store = ThreadStore::global(cx);
            thread_store.update(cx, |store, cx| {
                store.save_thread(
                    acp::SessionId::new("native-session"),
                    make_db_thread("Native Thread", Utc::now()),
                    PathList::default(),
                    cx,
                )
            })
        });
        save_task.await.unwrap();
        cx.run_until_parked();

        // Run migration - should skip because metadata store is not empty
        cx.update(|cx| {
            migrate_thread_metadata(cx);
        });

        cx.run_until_parked();

        // Verify only the existing metadata is present (migration was skipped)
        let list = cx.update(|cx| {
            let store = SidebarThreadMetadataStore::global(cx);
            store.read(cx).entries().collect::<Vec<_>>()
        });
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].session_id.0.as_ref(), "existing-session");
    }

    #[gpui::test]
    async fn test_subagent_threads_excluded_from_sidebar_metadata(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            cx.update_flags(true, vec!["agent-v2".to_string()]);
            ThreadStore::init_global(cx);
            SidebarThreadMetadataStore::init_global(cx);
        });

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
            let store = SidebarThreadMetadataStore::global(cx);
            store.read(cx).entries().collect::<Vec<_>>()
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
}
