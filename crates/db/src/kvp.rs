use anyhow::Result;

use super::Db;

impl Db {
    pub fn read_kvp(&self, key: &str) -> Result<String> {
        let mut stmt = self
            .connecion
            .prepare_cached("SELECT value FROM kv_store WHERE key = (?)")?;

        Ok(stmt.query_row([key], |row| row.get(0))?)
    }

    pub fn delete_kvp(&self, key: &str) -> Result<()> {
        let mut stmt = self
            .connecion
            .prepare_cached("SELECT value FROM kv_store WHERE key = (?)")?;

        stmt.execute([key])?;

        Ok(())
    }

    pub fn write_kvp(&self, key: &str, value: &str) -> Result<()> {
        let mut stmt = self
            .connecion
            .prepare_cached("INSERT OR REPLACE INTO kv_store(key, value) VALUES ((?), (?))")?;

        stmt.execute([key, value])?;

        Ok(())
    }
}
