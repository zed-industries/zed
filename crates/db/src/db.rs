pub mod items;
pub mod kvp;
mod migrations;
pub mod pane;
pub mod workspace;

use std::fs;
use std::ops::Deref;
use std::path::Path;

use anyhow::Result;
use indoc::indoc;
use kvp::KVP_MIGRATION;
use pane::PANE_MIGRATIONS;
use sqlez::connection::Connection;
use sqlez::thread_safe_connection::ThreadSafeConnection;

pub use workspace::*;

#[derive(Clone)]
pub struct Db(ThreadSafeConnection);

impl Deref for Db {
    type Target = sqlez::connection::Connection;

    fn deref(&self) -> &Self::Target {
        &self.0.deref()
    }
}

impl Db {
    /// Open or create a database at the given directory path.
    pub fn open(db_dir: &Path, channel: &'static str) -> Self {
        // Use 0 for now. Will implement incrementing and clearing of old db files soon TM
        let current_db_dir = db_dir.join(Path::new(&format!("0-{}", channel)));
        fs::create_dir_all(&current_db_dir)
            .expect("Should be able to create the database directory");
        let db_path = current_db_dir.join(Path::new("db.sqlite"));

        Db(
            ThreadSafeConnection::new(db_path.to_string_lossy().as_ref(), true)
                .with_initialize_query(indoc! {"
                    PRAGMA journal_mode=WAL;
                    PRAGMA synchronous=NORMAL;
                    PRAGMA foreign_keys=TRUE;
                    PRAGMA case_sensitive_like=TRUE;
                "})
                .with_migrations(&[KVP_MIGRATION, WORKSPACES_MIGRATION, PANE_MIGRATIONS]),
        )
    }

    pub fn persisting(&self) -> bool {
        self.persistent()
    }

    /// Open a in memory database for testing and as a fallback.
    pub fn open_in_memory(db_name: &str) -> Self {
        Db(ThreadSafeConnection::new(db_name, false)
            .with_initialize_query(indoc! {"
                    PRAGMA journal_mode=WAL;
                    PRAGMA synchronous=NORMAL;
                    PRAGMA foreign_keys=TRUE;
                    PRAGMA case_sensitive_like=TRUE;
                    "})
            .with_migrations(&[KVP_MIGRATION, WORKSPACES_MIGRATION, PANE_MIGRATIONS]))
    }

    pub fn write_to<P: AsRef<Path>>(&self, dest: P) -> Result<()> {
        let destination = Connection::open_file(dest.as_ref().to_string_lossy().as_ref());
        self.backup_main(&destination)
    }
}

impl Drop for Db {
    fn drop(&mut self) {
        self.exec(indoc! {"
            PRAGMA analysis_limit=500;
            PRAGMA optimize"})
            .ok();
    }
}
