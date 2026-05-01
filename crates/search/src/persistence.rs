use anyhow::Result;
use db::{
    query,
    sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
    sqlez_macros::sql,
};

const MAX_BUFFER_SEARCH_HISTORY_PER_KIND: usize = 50;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HistoryKind {
    Find,
    Replace,
}

impl HistoryKind {
    fn as_str(self) -> &'static str {
        match self {
            HistoryKind::Find => "find",
            HistoryKind::Replace => "replace",
        }
    }
}

pub struct BufferSearchHistoryDB(ThreadSafeConnection);

impl Domain for BufferSearchHistoryDB {
    const NAME: &str = stringify!(BufferSearchHistoryDB);
    const MIGRATIONS: &[&str] = &[sql!(
        CREATE TABLE IF NOT EXISTS search_history(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            kind TEXT NOT NULL,
            query TEXT NOT NULL
        ) STRICT;
        CREATE INDEX IF NOT EXISTS search_history_kind_id ON search_history(kind, id);
    )];
}

db::static_connection!(BufferSearchHistoryDB, []);

impl BufferSearchHistoryDB {
    pub async fn record(&self, kind: HistoryKind, query: String) -> Result<()> {
        let kind = kind.as_str().to_string();
        self.record_internal(kind, query, MAX_BUFFER_SEARCH_HISTORY_PER_KIND as i64)
            .await
    }

    pub fn list(&self, kind: HistoryKind) -> Result<Vec<String>> {
        self.list_internal(kind.as_str().to_string())
    }

    pub async fn clear_all(&self) -> Result<()> {
        self.clear_all_internal().await
    }

    query! {
        async fn record_internal(kind: String, query: String, max_per_kind: i64) -> Result<()> {
            INSERT INTO search_history (kind, query) VALUES ((?), (?));
            DELETE FROM search_history
            WHERE kind = (?1)
            AND id NOT IN (
                SELECT id FROM search_history
                WHERE kind = (?1)
                ORDER BY id DESC
                LIMIT (?3)
            )
        }
    }

    query! {
        fn list_internal(kind: String) -> Result<Vec<String>> {
            SELECT query FROM search_history
            WHERE kind = (?)
            ORDER BY id ASC
        }
    }

    query! {
        async fn clear_all_internal() -> Result<()> {
            DELETE FROM search_history
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BufferSearchHistoryDB, HistoryKind, MAX_BUFFER_SEARCH_HISTORY_PER_KIND};

    #[gpui::test]
    async fn test_records_and_lists_history() {
        let db = BufferSearchHistoryDB::open_test_db("test_records_and_lists_history").await;

        assert!(db.list(HistoryKind::Find).unwrap().is_empty());
        assert!(db.list(HistoryKind::Replace).unwrap().is_empty());

        db.record(HistoryKind::Find, "foo".into()).await.unwrap();
        db.record(HistoryKind::Find, "bar".into()).await.unwrap();
        db.record(HistoryKind::Replace, "baz".into()).await.unwrap();

        assert_eq!(
            db.list(HistoryKind::Find).unwrap(),
            vec!["foo".to_string(), "bar".to_string()]
        );
        assert_eq!(
            db.list(HistoryKind::Replace).unwrap(),
            vec!["baz".to_string()]
        );
    }

    #[gpui::test]
    async fn test_caps_at_max_per_kind() {
        let db = BufferSearchHistoryDB::open_test_db("test_caps_at_max_per_kind").await;

        for i in 0..(MAX_BUFFER_SEARCH_HISTORY_PER_KIND + 5) {
            db.record(HistoryKind::Find, format!("q{i}")).await.unwrap();
        }

        let entries = db.list(HistoryKind::Find).unwrap();
        assert_eq!(entries.len(), MAX_BUFFER_SEARCH_HISTORY_PER_KIND);
        assert_eq!(entries.first().unwrap(), "q5");
        assert_eq!(
            entries.last().unwrap(),
            &format!("q{}", MAX_BUFFER_SEARCH_HISTORY_PER_KIND + 4)
        );
    }

    #[gpui::test]
    async fn test_clear_all_removes_every_kind() {
        let db = BufferSearchHistoryDB::open_test_db("test_clear_all_removes_every_kind").await;

        db.record(HistoryKind::Find, "foo".into()).await.unwrap();
        db.record(HistoryKind::Replace, "bar".into()).await.unwrap();

        db.clear_all().await.unwrap();

        assert!(db.list(HistoryKind::Find).unwrap().is_empty());
        assert!(db.list(HistoryKind::Replace).unwrap().is_empty());
    }
}
