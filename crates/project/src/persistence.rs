use anyhow::Context;
use collections::{HashMap, HashSet};
use gpui::{App, Entity, SharedString};
use itertools::Itertools as _;
use std::path::PathBuf;

use db::{
    query,
    sqlez::{domain::Domain, statement::Statement, thread_safe_connection::ThreadSafeConnection},
    sqlez_macros::sql,
};

use crate::{
    trusted_worktrees::{PathTrust, RemoteHostLocation, find_worktree_in_store},
    worktree_store::WorktreeStore,
};

// https://www.sqlite.org/limits.html
// > <..> the maximum value of a host parameter number is SQLITE_MAX_VARIABLE_NUMBER,
// > which defaults to <..> 32766 for SQLite versions after 3.32.0.
const MAX_QUERY_PLACEHOLDERS: usize = 32000;

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
        trusted_globals: HashSet<Option<RemoteHostLocation>>,
    ) -> anyhow::Result<()> {
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
            .chain(trusted_globals.into_iter().map(|host| (None, host)))
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
                        &host.as_ref().map(|host| host.host_name.as_str()),
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
        worktree_store: Entity<WorktreeStore>,
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
                        host_name: SharedString::new(host_name),
                    }),
                    (Some(user_name), Some(host_name)) => Some(RemoteHostLocation {
                        user_name: Some(SharedString::new(user_name)),
                        host_name: SharedString::new(host_name),
                    }),
                };

                match abs_path {
                    Some(abs_path) => {
                        if db_host != host {
                            (db_host, PathTrust::AbsPath(abs_path))
                        } else {
                            find_worktree_in_store(worktree_store.read(cx), &abs_path, cx)
                                .map(PathTrust::Worktree)
                                .map(|trusted_worktree| (host.clone(), trusted_worktree))
                                .unwrap_or_else(|| (db_host.clone(), PathTrust::AbsPath(abs_path)))
                        }
                    }
                    None => (db_host, PathTrust::Global),
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
