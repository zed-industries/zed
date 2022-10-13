mod kvp;

use anyhow::Result;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Pool, Sqlite};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

pub use kvp::*;

pub struct Db {
    pool: Pool<Sqlite>,
    in_memory: bool,
}

// To make a migration:
// Add to the migrations directory, a file with the name:
//  <NUMBER>_<DESCRIPTION>.sql. Migrations are executed in order of number

impl Db {
    /// Open or create a database at the given file path. Falls back to in memory database if the
    /// database at the given path is corrupted
    pub async fn open(path: &Path) -> Arc<Self> {
        let options = SqliteConnectOptions::from_str(&format!(
            "sqlite://{}",
            path.to_string_lossy().to_string()
        ))
        .expect("database path should always be well formed")
        .create_if_missing(true);

        Self::initialize(options, false)
            .await
            .unwrap_or(Self::open_in_memory().await)
    }

    /// Open a in memory database for testing and as a fallback.
    pub async fn open_in_memory() -> Arc<Self> {
        let options = SqliteConnectOptions::from_str(":memory:")
            .expect("Should always be able to create in memory database options");

        Self::initialize(options, true)
            .await
            .expect("Should always be able to create an in memory database")
    }

    async fn initialize(options: SqliteConnectOptions, in_memory: bool) -> Result<Arc<Self>> {
        let pool = Pool::<Sqlite>::connect_with(options).await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Arc::new(Self { pool, in_memory }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[gpui::test]
    fn test_db() {
        let dir = TempDir::new("db-test").unwrap();
        let fake_db = Db::open_fake();
        let real_db = Db::open(&dir.path().join("test.db")).unwrap();

        for db in [&real_db, &fake_db] {
            assert_eq!(
                db.read(["key-1", "key-2", "key-3"]).unwrap(),
                &[None, None, None]
            );

            db.write([("key-1", "one"), ("key-3", "three")]).unwrap();
            assert_eq!(
                db.read(["key-1", "key-2", "key-3"]).unwrap(),
                &[
                    Some("one".as_bytes().to_vec()),
                    None,
                    Some("three".as_bytes().to_vec())
                ]
            );

            db.delete(["key-3", "key-4"]).unwrap();
            assert_eq!(
                db.read(["key-1", "key-2", "key-3"]).unwrap(),
                &[Some("one".as_bytes().to_vec()), None, None,]
            );
        }

        drop(real_db);

        let real_db = Db::open(&dir.path().join("test.db")).unwrap();
        assert_eq!(
            real_db.read(["key-1", "key-2", "key-3"]).unwrap(),
            &[Some("one".as_bytes().to_vec()), None, None,]
        );
    }
}
