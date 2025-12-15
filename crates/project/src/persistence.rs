use collections::{HashMap, HashSet};
use gpui::{App, Entity, SharedString};
use std::path::PathBuf;

use db::{
    query,
    sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
    sqlez_macros::sql,
};

use crate::{
    trusted_worktrees::{PathTrust, RemoteHostLocation, find_worktree_in_store},
    worktree_store::WorktreeStore,
};

// https://www.sqlite.org/limits.html
// > <..> the maximum value of a host parameter number is SQLITE_MAX_VARIABLE_NUMBER,
// > which defaults to <..> 32766 for SQLite versions after 3.32.0.
#[allow(unused)]
const MAX_QUERY_PLACEHOLDERS: usize = 32000;

#[allow(unused)]
pub struct ProjectDb(ThreadSafeConnection);

impl Domain for ProjectDb {
    const NAME: &str = stringify!(ProjectDb);

    const MIGRATIONS: &[&str] = &[sql!(
        CREATE TABLE IF NOT EXISTS trusted_worktrees (
            trust_id INTEGER PRIMARY KEY AUTOINCREMENT,
            absolute_path TEXT,
            user_name TEXT,
            host_name TEXT
        ) STRICT;
    )];
}

db::static_connection!(PROJECT_DB, ProjectDb, []);

impl ProjectDb {
    pub(crate) async fn save_trusted_worktrees(
        &self,
        trusted_worktrees: HashMap<Option<RemoteHostLocation>, HashSet<PathBuf>>,
        trusted_workspaces: HashSet<Option<RemoteHostLocation>>,
    ) -> anyhow::Result<()> {
        use anyhow::Context as _;
        use db::sqlez::statement::Statement;
        use itertools::Itertools as _;

        PROJECT_DB
            .clear_trusted_worktrees()
            .await
            .context("clearing previous trust state")?;

        let trusted_worktrees = trusted_worktrees
            .into_iter()
            .flat_map(|(host, abs_paths)| {
                abs_paths
                    .into_iter()
                    .map(move |abs_path| (Some(abs_path), host.clone()))
            })
            .chain(trusted_workspaces.into_iter().map(|host| (None, host)))
            .collect::<Vec<_>>();
        let mut first_worktree;
        let mut last_worktree = 0_usize;
        for (count, placeholders) in std::iter::once("(?, ?, ?)")
            .cycle()
            .take(trusted_worktrees.len())
            .chunks(MAX_QUERY_PLACEHOLDERS / 3)
            .into_iter()
            .map(|chunk| {
                let mut count = 0;
                let placeholders = chunk
                    .inspect(|_| {
                        count += 1;
                    })
                    .join(", ");
                (count, placeholders)
            })
            .collect::<Vec<_>>()
        {
            first_worktree = last_worktree;
            last_worktree = last_worktree + count;
            let query = format!(
                r#"INSERT INTO trusted_worktrees(absolute_path, user_name, host_name)
VALUES {placeholders};"#
            );

            let trusted_worktrees = trusted_worktrees[first_worktree..last_worktree].to_vec();
            self.write(move |conn| {
                let mut statement = Statement::prepare(conn, query)?;
                let mut next_index = 1;
                for (abs_path, host) in trusted_worktrees {
                    let abs_path = abs_path.as_ref().map(|abs_path| abs_path.to_string_lossy());
                    next_index = statement.bind(
                        &abs_path.as_ref().map(|abs_path| abs_path.as_ref()),
                        next_index,
                    )?;
                    next_index = statement.bind(
                        &host
                            .as_ref()
                            .and_then(|host| Some(host.user_name.as_ref()?.as_str())),
                        next_index,
                    )?;
                    next_index = statement.bind(
                        &host.as_ref().map(|host| host.host_identifier.as_str()),
                        next_index,
                    )?;
                }
                statement.exec()
            })
            .await
            .context("inserting new trusted state")?;
        }
        Ok(())
    }

    pub(crate) fn fetch_trusted_worktrees(
        &self,
        worktree_store: Option<Entity<WorktreeStore>>,
        host: Option<RemoteHostLocation>,
        cx: &App,
    ) -> anyhow::Result<HashMap<Option<RemoteHostLocation>, HashSet<PathTrust>>> {
        let trusted_worktrees = PROJECT_DB.trusted_worktrees()?;
        Ok(trusted_worktrees
            .into_iter()
            .map(|(abs_path, user_name, host_name)| {
                let db_host = match (user_name, host_name) {
                    (_, None) => None,
                    (None, Some(host_name)) => Some(RemoteHostLocation {
                        user_name: None,
                        host_identifier: SharedString::new(host_name),
                    }),
                    (Some(user_name), Some(host_name)) => Some(RemoteHostLocation {
                        user_name: Some(SharedString::new(user_name)),
                        host_identifier: SharedString::new(host_name),
                    }),
                };

                match abs_path {
                    Some(abs_path) => {
                        if db_host != host {
                            (db_host, PathTrust::AbsPath(abs_path))
                        } else if let Some(worktree_store) = &worktree_store {
                            find_worktree_in_store(worktree_store.read(cx), &abs_path, cx)
                                .map(PathTrust::Worktree)
                                .map(|trusted_worktree| (host.clone(), trusted_worktree))
                                .unwrap_or_else(|| (db_host.clone(), PathTrust::AbsPath(abs_path)))
                        } else {
                            (db_host, PathTrust::AbsPath(abs_path))
                        }
                    }
                    None => (db_host, PathTrust::Workspace),
                }
            })
            .fold(HashMap::default(), |mut acc, (remote_host, path_trust)| {
                acc.entry(remote_host)
                    .or_insert_with(HashSet::default)
                    .insert(path_trust);
                acc
            }))
    }

    query! {
        fn trusted_worktrees() -> Result<Vec<(Option<PathBuf>, Option<String>, Option<String>)>> {
            SELECT absolute_path, user_name, host_name
            FROM trusted_worktrees
        }
    }

    query! {
        pub async fn clear_trusted_worktrees() -> Result<()> {
            DELETE FROM trusted_worktrees
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use collections::{HashMap, HashSet};
    use gpui::{SharedString, TestAppContext};
    use serde_json::json;
    use settings::SettingsStore;
    use smol::lock::Mutex;
    use util::path;

    use crate::{
        FakeFs, Project,
        persistence::PROJECT_DB,
        trusted_worktrees::{PathTrust, RemoteHostLocation},
    };

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[gpui::test]
    async fn test_save_and_fetch_trusted_worktrees(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let _guard = TEST_LOCK.lock().await;
        PROJECT_DB.clear_trusted_worktrees().await.unwrap();
        cx.update(|cx| {
            if cx.try_global::<SettingsStore>().is_none() {
                let settings = SettingsStore::test(cx);
                cx.set_global(settings);
            }
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/"),
            json!({
                "project_a": { "main.rs": "" },
                "project_b": { "lib.rs": "" }
            }),
        )
        .await;

        let project = Project::test(
            fs,
            [path!("/project_a").as_ref(), path!("/project_b").as_ref()],
            cx,
        )
        .await;
        let worktree_store = project.read_with(cx, |p, _| p.worktree_store());

        let mut trusted_paths: HashMap<Option<RemoteHostLocation>, HashSet<PathBuf>> =
            HashMap::default();
        trusted_paths.insert(
            None,
            HashSet::from_iter([
                PathBuf::from(path!("/project_a")),
                PathBuf::from(path!("/project_b")),
            ]),
        );

        PROJECT_DB
            .save_trusted_worktrees(trusted_paths, HashSet::default())
            .await
            .unwrap();

        let fetched = cx.update(|cx| {
            PROJECT_DB.fetch_trusted_worktrees(Some(worktree_store.clone()), None, cx)
        });
        let fetched = fetched.unwrap();

        let local_trust = fetched.get(&None).expect("should have local host entry");
        assert_eq!(local_trust.len(), 2);
        assert!(
            local_trust
                .iter()
                .all(|p| matches!(p, PathTrust::Worktree(_)))
        );

        let fetched_no_store = cx
            .update(|cx| PROJECT_DB.fetch_trusted_worktrees(None, None, cx))
            .unwrap();
        let local_trust_no_store = fetched_no_store
            .get(&None)
            .expect("should have local host entry");
        assert_eq!(local_trust_no_store.len(), 2);
        assert!(
            local_trust_no_store
                .iter()
                .all(|p| matches!(p, PathTrust::AbsPath(_)))
        );
    }

    #[gpui::test]
    async fn test_save_and_fetch_workspace_trust(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let _guard = TEST_LOCK.lock().await;
        PROJECT_DB.clear_trusted_worktrees().await.unwrap();
        cx.update(|cx| {
            if cx.try_global::<SettingsStore>().is_none() {
                let settings = SettingsStore::test(cx);
                cx.set_global(settings);
            }
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "main.rs": "" }))
            .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_store = project.read_with(cx, |p, _| p.worktree_store());

        let trusted_workspaces = HashSet::from_iter([None]);
        PROJECT_DB
            .save_trusted_worktrees(HashMap::default(), trusted_workspaces)
            .await
            .unwrap();

        let fetched = cx.update(|cx| {
            PROJECT_DB.fetch_trusted_worktrees(Some(worktree_store.clone()), None, cx)
        });
        let fetched = fetched.unwrap();

        let local_trust = fetched.get(&None).expect("should have local host entry");
        assert!(local_trust.contains(&PathTrust::Workspace));

        let fetched_no_store = cx
            .update(|cx| PROJECT_DB.fetch_trusted_worktrees(None, None, cx))
            .unwrap();
        let local_trust_no_store = fetched_no_store
            .get(&None)
            .expect("should have local host entry");
        assert!(local_trust_no_store.contains(&PathTrust::Workspace));
    }

    #[gpui::test]
    async fn test_save_and_fetch_remote_host_trust(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let _guard = TEST_LOCK.lock().await;
        PROJECT_DB.clear_trusted_worktrees().await.unwrap();
        cx.update(|cx| {
            if cx.try_global::<SettingsStore>().is_none() {
                let settings = SettingsStore::test(cx);
                cx.set_global(settings);
            }
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "main.rs": "" }))
            .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_store = project.read_with(cx, |p, _| p.worktree_store());

        let remote_host = Some(RemoteHostLocation {
            user_name: Some(SharedString::from("testuser")),
            host_identifier: SharedString::from("remote.example.com"),
        });

        let mut trusted_paths: HashMap<Option<RemoteHostLocation>, HashSet<PathBuf>> =
            HashMap::default();
        trusted_paths.insert(
            remote_host.clone(),
            HashSet::from_iter([PathBuf::from("/home/testuser/project")]),
        );

        PROJECT_DB
            .save_trusted_worktrees(trusted_paths, HashSet::default())
            .await
            .unwrap();

        let fetched = cx.update(|cx| {
            PROJECT_DB.fetch_trusted_worktrees(Some(worktree_store.clone()), None, cx)
        });
        let fetched = fetched.unwrap();

        let remote_trust = fetched
            .get(&remote_host)
            .expect("should have remote host entry");
        assert_eq!(remote_trust.len(), 1);
        assert!(remote_trust
            .iter()
            .any(|p| matches!(p, PathTrust::AbsPath(path) if path == &PathBuf::from("/home/testuser/project"))));

        let fetched_no_store = cx
            .update(|cx| PROJECT_DB.fetch_trusted_worktrees(None, None, cx))
            .unwrap();
        let remote_trust_no_store = fetched_no_store
            .get(&remote_host)
            .expect("should have remote host entry");
        assert_eq!(remote_trust_no_store.len(), 1);
        assert!(remote_trust_no_store
            .iter()
            .any(|p| matches!(p, PathTrust::AbsPath(path) if path == &PathBuf::from("/home/testuser/project"))));
    }

    #[gpui::test]
    async fn test_clear_trusted_worktrees(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let _guard = TEST_LOCK.lock().await;
        PROJECT_DB.clear_trusted_worktrees().await.unwrap();
        cx.update(|cx| {
            if cx.try_global::<SettingsStore>().is_none() {
                let settings = SettingsStore::test(cx);
                cx.set_global(settings);
            }
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "main.rs": "" }))
            .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_store = project.read_with(cx, |p, _| p.worktree_store());

        let trusted_workspaces = HashSet::from_iter([None]);
        PROJECT_DB
            .save_trusted_worktrees(HashMap::default(), trusted_workspaces)
            .await
            .unwrap();

        PROJECT_DB.clear_trusted_worktrees().await.unwrap();

        let fetched = cx.update(|cx| {
            PROJECT_DB.fetch_trusted_worktrees(Some(worktree_store.clone()), None, cx)
        });
        let fetched = fetched.unwrap();

        assert!(fetched.is_empty(), "should be empty after clear");

        let fetched_no_store = cx
            .update(|cx| PROJECT_DB.fetch_trusted_worktrees(None, None, cx))
            .unwrap();
        assert!(fetched_no_store.is_empty(), "should be empty after clear");
    }
}
