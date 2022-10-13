use anyhow::Result;

use super::Db;

impl Db {
    pub async fn read_kvp(&self, key: &str) -> Result<String> {
        let value = sqlx::query!("SELECT value FROM kv_store WHERE key = ?", key)
            .fetch_one(&self.pool)
            .await
            .map(|res| res.value)?;

        Ok(value)
    }

    pub async fn delete_kvp(&self, key: &str) -> Result<()> {
        sqlx::query!("DELETE FROM kv_store WHERE key = ?", key)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn write_kvp(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO kv_store(key, value) VALUES (?, ?)",
            key,
            value
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
