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

impl ProjectDb {}

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

    static TEST_WORKTREE_TRUST_LOCK: Mutex<()> = Mutex::new(());
}
