use db::{
    query,
    sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
    sqlez_macros::sql,
};
use workspace::WorkspaceId;

pub struct FileFinderDb(ThreadSafeConnection);

impl Domain for FileFinderDb {
    const NAME: &str = stringify!(FileFinderDb);
    const MIGRATIONS: &[&str] = &[sql!(
        CREATE TABLE IF NOT EXISTS file_history(
            workspace_id  INTEGER NOT NULL,
            file_path     TEXT    NOT NULL,
            last_accessed INTEGER NOT NULL DEFAULT (unixepoch()),
            PRIMARY KEY(workspace_id, file_path)
        ) STRICT;

        CREATE INDEX IF NOT EXISTS ix_file_history_recent
            ON file_history(workspace_id, last_accessed DESC);
    )];
}

db::static_connection!(FileFinderDb, [workspace::WorkspaceDb]);

impl FileFinderDb {
    // Combines the upsert and pruning in one write() call to avoid table-level lock
    // conflicts when the shared test database is used concurrently. The ?1 / ?2 syntax
    // lets the DELETE reuse the workspace_id binding from statement 1 without needing
    // a separate parameter slot.
    query! {
        pub async fn record_file_access(workspace_id: WorkspaceId, file_path: String) -> Result<()> {
            INSERT INTO file_history(workspace_id, file_path, last_accessed)
            VALUES (?1, ?2, unixepoch())
            ON CONFLICT(workspace_id, file_path) DO UPDATE SET last_accessed = unixepoch();
            DELETE FROM file_history
            WHERE workspace_id = ?1 AND rowid NOT IN (
                SELECT rowid FROM file_history
                WHERE workspace_id = ?1
                ORDER BY last_accessed DESC
                LIMIT 200
            )
        }
    }

    query! {
        pub fn recent_files(workspace_id: WorkspaceId) -> Result<Vec<String>> {
            SELECT file_path FROM file_history
            WHERE workspace_id = (?)
            ORDER BY last_accessed DESC
            LIMIT 20
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Each test uses open_test_db with a unique name to get a fully isolated in-memory
    // SQLite database. This avoids the shared-cache locking conflicts that occur when
    // concurrent tests share TEST_APP_DATABASE.

    fn ws() -> WorkspaceId {
        WorkspaceId::from_i64(1)
    }

    #[gpui::test]
    async fn test_record_and_retrieve_file_access(_cx: &mut gpui::TestAppContext) {
        let db = FileFinderDb::open_test_db("test_record_and_retrieve").await;
        let workspace_id = ws();
        let path = "/project/src/main.rs".to_string();

        let result = db.recent_files(workspace_id).unwrap();
        assert!(result.is_empty());

        db.record_file_access(workspace_id, path.clone())
            .await
            .unwrap();

        let result = db.recent_files(workspace_id).unwrap();
        assert_eq!(result, vec![path]);
    }

    #[gpui::test]
    async fn test_deduplicates_repeated_access(_cx: &mut gpui::TestAppContext) {
        let db = FileFinderDb::open_test_db("test_deduplicates").await;
        let workspace_id = ws();
        let path = "/project/src/main.rs".to_string();

        db.record_file_access(workspace_id, path.clone())
            .await
            .unwrap();
        db.record_file_access(workspace_id, path.clone())
            .await
            .unwrap();
        db.record_file_access(workspace_id, path.clone())
            .await
            .unwrap();

        let result = db.recent_files(workspace_id).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], path);
    }

    #[gpui::test]
    async fn test_isolates_history_by_workspace(_cx: &mut gpui::TestAppContext) {
        let db = FileFinderDb::open_test_db("test_isolates_by_workspace").await;
        let ws1 = WorkspaceId::from_i64(1);
        let ws2 = WorkspaceId::from_i64(2);

        db.record_file_access(ws1, "/project1/foo.rs".to_string())
            .await
            .unwrap();
        db.record_file_access(ws2, "/project2/bar.rs".to_string())
            .await
            .unwrap();

        let result1 = db.recent_files(ws1).unwrap();
        assert_eq!(result1, vec!["/project1/foo.rs".to_string()]);

        let result2 = db.recent_files(ws2).unwrap();
        assert_eq!(result2, vec!["/project2/bar.rs".to_string()]);
    }

    #[gpui::test]
    async fn test_recent_files_returns_inserted_files(_cx: &mut gpui::TestAppContext) {
        let db = FileFinderDb::open_test_db("test_recent_files_returns").await;
        let workspace_id = ws();

        db.record_file_access(workspace_id, "/project/a.rs".to_string())
            .await
            .unwrap();
        db.record_file_access(workspace_id, "/project/b.rs".to_string())
            .await
            .unwrap();

        let result = db.recent_files(workspace_id).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"/project/a.rs".to_string()));
        assert!(result.contains(&"/project/b.rs".to_string()));
    }

    #[gpui::test]
    async fn test_prunes_entries_beyond_limit(_cx: &mut gpui::TestAppContext) {
        let db = FileFinderDb::open_test_db("test_prunes_entries").await;
        let workspace_id = ws();

        for i in 0..=200 {
            db.record_file_access(workspace_id, format!("/project/file_{i}.rs"))
                .await
                .unwrap();
        }

        let result = db.recent_files(workspace_id).unwrap();
        // recent_files caps at 20; pruning keeps at most 200 in the table
        assert_eq!(result.len(), 20);
    }
}
