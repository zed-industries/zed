use anyhow::Result;
use rusqlite::OptionalExtension;

use super::Db;

impl Db {
    pub fn read_kvp(&self, key: &str) -> Result<Option<String>> {
        let lock = self.connection.lock();
        let mut stmt = lock.prepare_cached("SELECT value FROM kv_store WHERE key = (?)")?;

        Ok(stmt.query_row([key], |row| row.get(0)).optional()?)
    }

    pub fn delete_kvp(&self, key: &str) -> Result<()> {
        let lock = self.connection.lock();

        let mut stmt = lock.prepare_cached("SELECT value FROM kv_store WHERE key = (?)")?;

        stmt.execute([key])?;

        Ok(())
    }

    pub fn write_kvp(&self, key: &str, value: &str) -> Result<()> {
        let lock = self.connection.lock();

        let mut stmt =
            lock.prepare_cached("INSERT OR REPLACE INTO kv_store(key, value) VALUES ((?), (?))")?;

        stmt.execute([key, value])?;

        Ok(())
    }
}
