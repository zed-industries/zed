pub mod kvp;

use std::fs;
use std::ops::Deref;
use std::path::Path;

use anyhow::Result;
use indoc::indoc;
use sqlez::connection::Connection;
use sqlez::domain::Domain;
use sqlez::thread_safe_connection::ThreadSafeConnection;

const INITIALIZE_QUERY: &'static str = indoc! {"
    PRAGMA journal_mode=WAL;
    PRAGMA synchronous=NORMAL;
    PRAGMA foreign_keys=TRUE;
    PRAGMA case_sensitive_like=TRUE;
"};

#[derive(Clone)]
pub struct Db<D: Domain>(ThreadSafeConnection<D>);

impl<D: Domain> Deref for Db<D> {
    type Target = sqlez::connection::Connection;

    fn deref(&self) -> &Self::Target {
        &self.0.deref()
    }
}

impl<D: Domain> Db<D> {
    /// Open or create a database at the given directory path.
    pub fn open(db_dir: &Path, channel: &'static str) -> Self {
        // Use 0 for now. Will implement incrementing and clearing of old db files soon TM
        let current_db_dir = db_dir.join(Path::new(&format!("0-{}", channel)));
        fs::create_dir_all(&current_db_dir)
            .expect("Should be able to create the database directory");
        let db_path = current_db_dir.join(Path::new("db.sqlite"));

        Db(
            ThreadSafeConnection::new(db_path.to_string_lossy().as_ref(), true)
                .with_initialize_query(INITIALIZE_QUERY),
        )
    }

    /// Open a in memory database for testing and as a fallback.
    pub fn open_in_memory(db_name: &str) -> Self {
        Db(ThreadSafeConnection::new(db_name, false).with_initialize_query(INITIALIZE_QUERY))
    }

    pub fn persisting(&self) -> bool {
        self.persistent()
    }

    pub fn write_to<P: AsRef<Path>>(&self, dest: P) -> Result<()> {
        let destination = Connection::open_file(dest.as_ref().to_string_lossy().as_ref());
        self.backup_main(&destination)
    }

    pub fn open_as<D2: Domain>(&self) -> Db<D2> {
        Db(self.0.for_domain())
    }
}
