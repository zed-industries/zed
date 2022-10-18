mod items;
mod kvp;
mod migrations;

use anyhow::Result;
use migrations::MIGRATIONS;
use parking_lot::Mutex;
use rusqlite::Connection;
use std::path::Path;
use std::sync::Arc;

pub use kvp::*;

pub struct Db {
    connection: Mutex<Connection>,
    in_memory: bool,
}

// To make a migration:
// Add to the migrations directory, a file with the name:
//  <NUMBER>_<DESCRIPTION>.sql. Migrations are executed in order of number

impl Db {
    /// Open or create a database at the given file path. Falls back to in memory database if the
    /// database at the given path is corrupted
    pub fn open(path: &Path) -> Result<Arc<Self>> {
        let conn = Connection::open(path)?;

        Self::initialize(conn, false).or_else(|_| Self::open_in_memory())
    }

    /// Open a in memory database for testing and as a fallback.
    pub fn open_in_memory() -> Result<Arc<Self>> {
        let conn = Connection::open_in_memory()?;

        Self::initialize(conn, true)
    }

    fn initialize(mut conn: Connection, in_memory: bool) -> Result<Arc<Self>> {
        MIGRATIONS.to_latest(&mut conn)?;

        Ok(Arc::new(Self {
            connection: Mutex::new(conn),
            in_memory,
        }))
    }
}
