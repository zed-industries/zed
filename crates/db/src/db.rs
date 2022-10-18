mod kvp;
mod migrations;

use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use log::error;
use parking_lot::Mutex;
use rusqlite::Connection;

use migrations::MIGRATIONS;

pub enum Db {
    Real {
        connection: Mutex<Connection>,
        in_memory: bool,
    },
    Null,
}

// To make a migration:
// Add to the migrations directory, a file with the name:
//  <NUMBER>_<DESCRIPTION>.sql. Migrations are executed in order of number

impl Db {
    /// Open or create a database at the given file path. Falls back to in memory database if the
    /// database at the given path is corrupted
    pub fn open(path: &Path) -> Arc<Self> {
        Connection::open(path)
            .map_err(Into::into)
            .and_then(|connection| Self::initialize(connection, false))
            .unwrap_or_else(|e| {
                error!(
                    "Connecting to db failed. Falling back to in memory db. {}",
                    e
                );
                Self::open_in_memory()
            })
    }

    /// Open a in memory database for testing and as a fallback.
    pub fn open_in_memory() -> Arc<Self> {
        Connection::open_in_memory()
            .map_err(Into::into)
            .and_then(|connection| Self::initialize(connection, true))
            .unwrap_or_else(|e| {
                error!("Connecting to in memory db failed. Reverting to null db. {}");
                Arc::new(Self::Null)
            })
    }

    fn initialize(mut conn: Connection, in_memory: bool) -> Result<Arc<Self>> {
        MIGRATIONS.to_latest(&mut conn)?;

        Ok(Arc::new(Self::Real {
            connection: Mutex::new(conn),
            in_memory,
        }))
    }

    fn persisting(&self) -> bool {
        match self {
            Db::Real { in_memory, .. } => *in_memory,
            _ => false,
        }
    }
}
