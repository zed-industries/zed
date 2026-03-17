use std::{path::Path, sync::Arc};

use agent::{ThreadStore, ZED_AGENT_ID};
use agent_client_protocol as acp;
use anyhow::Result;
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
use gpui::{AppContext as _, Entity, Global, Subscription, Task};
use project::AgentId;
use ui::{App, Context, SharedString};
use workspace::PathList;

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
///
/// TODO: Remove this after N weeks of shipping the sidebar
fn migrate_thread_metadata(cx: &mut App) {
    ThreadMetadataStore::global(cx).update(cx, |store, cx| {
        let list = store.list(cx);
        cx.spawn(async move |this, cx| {
            let Ok(list) = list.await else {
                return;
            };
            if list.is_empty() {
                this.update(cx, |this, cx| {
                    let metadata = ThreadStore::global(cx)
                        .read(cx)
                        .entries()
                        .map(|entry| ThreadMetadata {
                            session_id: entry.id,
                            agent_id: None,
                            title: entry.title,
                            updated_at: entry.updated_at,
                            created_at: entry.created_at,
                            folder_paths: entry.folder_paths,
                        })
                        .collect::<Vec<_>>();
                    for entry in metadata {
                        this.save(entry, cx).detach_and_log_err(cx);
                    }
                })
                .ok();
            }
        })
        .detach();
    });
}

struct GlobalThreadMetadataStore(Entity<ThreadMetadataStore>);
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

pub struct ThreadMetadataStore {
    db: ThreadMetadataDb,
    session_subscriptions: HashMap<acp::SessionId, Subscription>,
}

impl ThreadMetadataStore {
    #[cfg(not(any(test, feature = "test-support")))]
    pub fn init_global(cx: &mut App) {
        if cx.has_global::<Self>() {
            return;
        }

        let db = THREAD_METADATA_DB.clone();
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

    pub fn list(&self, cx: &App) -> Task<Result<Vec<ThreadMetadata>>> {
        let db = self.db.clone();
        cx.background_spawn(async move {
            let s = db.list()?;
            Ok(s)
        })
    }

    pub fn save(&mut self, metadata: ThreadMetadata, cx: &mut Context<Self>) -> Task<Result<()>> {
        if !cx.has_flag::<AgentV2FeatureFlag>() {
            return Task::ready(Ok(()));
        }

        let db = self.db.clone();
        cx.spawn(async move |this, cx| {
            db.save(metadata).await?;
            this.update(cx, |_this, cx| cx.notify())
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
            this.update(cx, |_this, cx| cx.notify())
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

        Self {
            db,
            session_subscriptions: HashMap::default(),
        }
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
            | acp_thread::AcpThreadEvent::EntryUpdated(_)
            | acp_thread::AcpThreadEvent::TitleUpdated => {
                let metadata = Self::metadata_for_acp_thread(thread.read(cx), cx);
                self.save(metadata, cx).detach_and_log_err(cx);
            }
            _ => {}
        }
    }

    fn metadata_for_acp_thread(thread: &acp_thread::AcpThread, cx: &App) -> ThreadMetadata {
        let session_id = thread.session_id().clone();
        let title = thread.title();
        let updated_at = Utc::now();

        let agent_id = thread.connection().agent_id();

        let agent_id = if agent_id.as_ref() == ZED_AGENT_ID.as_ref() {
            None
        } else {
            Some(agent_id)
        };

        let folder_paths = {
            let project = thread.project().read(cx);
            let paths: Vec<Arc<Path>> = project
                .visible_worktrees(cx)
                .map(|worktree| worktree.read(cx).abs_path())
                .collect();
            PathList::new(&paths)
        };

        ThreadMetadata {
            session_id,
            agent_id,
            title,
            created_at: Some(updated_at), // handled by db `ON CONFLICT`
            updated_at,
            folder_paths,
        }
    }
}

impl Global for ThreadMetadataStore {}

#[derive(Clone)]
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

db::static_connection!(THREAD_METADATA_DB, ThreadMetadataDb, []);

impl ThreadMetadataDb {
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
    use util::path_list::PathList;

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

    #[gpui::test]
    async fn test_migrate_thread_metadata(cx: &mut TestAppContext) {
        cx.update(|cx| {
            ThreadStore::init_global(cx);
            ThreadMetadataStore::init_global(cx);
        });

        // Verify the list is empty before migration
        let metadata_list = cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.read(cx).list(cx)
        });

        let list = metadata_list.await.unwrap();
        assert_eq!(list.len(), 0);

        let now = Utc::now();

        // Populate the native ThreadStore via save_thread
        let save1 = cx.update(|cx| {
            let thread_store = ThreadStore::global(cx);
            thread_store.update(cx, |store, cx| {
                store.save_thread(
                    acp::SessionId::new("session-1"),
                    make_db_thread("Thread 1", now),
                    PathList::default(),
                    cx,
                )
            })
        });
        save1.await.unwrap();
        cx.run_until_parked();

        let save2 = cx.update(|cx| {
            let thread_store = ThreadStore::global(cx);
            thread_store.update(cx, |store, cx| {
                store.save_thread(
                    acp::SessionId::new("session-2"),
                    make_db_thread("Thread 2", now),
                    PathList::default(),
                    cx,
                )
            })
        });
        save2.await.unwrap();
        cx.run_until_parked();

        // Run migration
        cx.update(|cx| {
            migrate_thread_metadata(cx);
        });

        cx.run_until_parked();

        // Verify the metadata was migrated
        let metadata_list = cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.read(cx).list(cx)
        });

        let list = metadata_list.await.unwrap();
        assert_eq!(list.len(), 2);

        let metadata1 = list
            .iter()
            .find(|m| m.session_id.0.as_ref() == "session-1")
            .expect("session-1 should be in migrated metadata");
        assert_eq!(metadata1.title.as_ref(), "Thread 1");
        assert!(metadata1.agent_id.is_none());

        let metadata2 = list
            .iter()
            .find(|m| m.session_id.0.as_ref() == "session-2")
            .expect("session-2 should be in migrated metadata");
        assert_eq!(metadata2.title.as_ref(), "Thread 2");
        assert!(metadata2.agent_id.is_none());
    }

    #[gpui::test]
    async fn test_migrate_thread_metadata_skips_when_data_exists(cx: &mut TestAppContext) {
        cx.update(|cx| {
            ThreadStore::init_global(cx);
            ThreadMetadataStore::init_global(cx);
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
            let store = ThreadMetadataStore::global(cx);
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
        let metadata_list = cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.read(cx).list(cx)
        });

        let list = metadata_list.await.unwrap();
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
            ThreadMetadataStore::init_global(cx);
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
                    "Subagent Thread",
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

        // List all metadata from the store.
        let metadata_list = cx.update(|cx| {
            let store = ThreadMetadataStore::global(cx);
            store.read(cx).list(cx)
        });

        let list = metadata_list.await.unwrap();

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
