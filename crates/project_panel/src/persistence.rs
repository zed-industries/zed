use anyhow::{Context as _, Result};
use collections::HashMap;
use db::sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection};
use db::sqlez_macros::sql;
use std::{path::Path, sync::Arc};
use workspace::{WorkspaceDb, WorkspaceId};

pub struct ProjectPanelDb(ThreadSafeConnection);

impl Domain for ProjectPanelDb {
    const NAME: &str = stringify!(ProjectPanelDb);

    // Each row records the saved expanded entries for a single worktree as a
    // JSON array of relative paths. The empty path (`""`) represents the
    // worktree root entry itself; an empty array represents "saved but
    // nothing expanded" (e.g. the user collapsed the worktree root). This
    // representation lets us distinguish "never saved" from "saved with no
    // expanded entries" — something a multi-row schema cannot encode.
    const MIGRATIONS: &[&str] = &[sql!(
        CREATE TABLE project_panel_collapse_state(
            workspace_id INTEGER NOT NULL,
            worktree_root_path TEXT NOT NULL,
            expanded_paths TEXT NOT NULL,
            PRIMARY KEY (workspace_id, worktree_root_path),
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
                ON UPDATE CASCADE
        ) STRICT;
    )];
}

db::static_connection!(ProjectPanelDb, [WorkspaceDb]);

impl ProjectPanelDb {
    /// Returns the saved expanded entries grouped by worktree absolute path.
    /// The empty relative path (`""`) represents the worktree root entry; an
    /// empty `Vec` represents a worktree explicitly saved as collapsed.
    pub fn expanded_entries(
        &self,
        workspace_id: WorkspaceId,
    ) -> Result<HashMap<Arc<Path>, Vec<String>>> {
        let rows: Vec<(String, String)> = self
            .select_bound(sql! {
                SELECT worktree_root_path, expanded_paths
                FROM project_panel_collapse_state
                WHERE workspace_id = ?
            })
            .and_then(|mut statement| statement(workspace_id))?;

        let mut result: HashMap<Arc<Path>, Vec<String>> = HashMap::default();
        for (worktree_root_path, expanded_paths) in rows {
            let paths: Vec<String> = serde_json::from_str(&expanded_paths)
                .with_context(|| format!("decoding expanded_paths for {worktree_root_path}"))?;
            let key: Arc<Path> = Arc::from(Path::new(&worktree_root_path));
            result.insert(key, paths);
        }
        Ok(result)
    }

    /// Upserts the saved expanded entries for each worktree in `entries`.
    /// Worktrees absent from the map are left untouched, so a save that is
    /// scheduled while some worktrees are still pending (e.g. during async
    /// worktree load) cannot accidentally drop their saved state. Worktrees
    /// present with an empty `Vec` are recorded as "saved with no expanded
    /// entries" rather than treated as "never saved".
    pub async fn save_expanded_entries(
        &self,
        workspace_id: WorkspaceId,
        entries: HashMap<Arc<Path>, Vec<String>>,
    ) -> Result<()> {
        self.write(move |conn| {
            conn.with_savepoint("project_panel_save_expanded_entries", || {
                for (worktree_root_path, paths) in &entries {
                    let worktree_root_str = worktree_root_path.to_string_lossy();
                    let serialized = serde_json::to_string(paths)
                        .context("serializing expanded paths")?;
                    conn.exec_bound(sql!(
                        INSERT INTO project_panel_collapse_state
                            (workspace_id, worktree_root_path, expanded_paths)
                        VALUES (?1, ?2, ?3)
                        ON CONFLICT(workspace_id, worktree_root_path)
                        DO UPDATE SET expanded_paths = excluded.expanded_paths;
                    ))?((
                        workspace_id,
                        worktree_root_str.as_ref(),
                        serialized.as_str(),
                    ))
                    .with_context(|| {
                        format!(
                            "saving collapse state for worktree {worktree_root_str}"
                        )
                    })?;
                }
                Ok(())
            })
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[gpui::test]
    async fn test_save_and_load_expanded_entries(cx: &mut gpui::TestAppContext) {
        let workspace_db = cx.update(|cx| workspace::WorkspaceDb::global(cx));
        let workspace_id = workspace_db.next_id().await.unwrap();
        let db = cx.update(|cx| ProjectPanelDb::global(cx));

        let worktree_a: Arc<Path> = Arc::from(Path::new("/foo/a"));
        let worktree_b: Arc<Path> = Arc::from(Path::new("/foo/b"));
        let worktree_c: Arc<Path> = Arc::from(Path::new("/foo/c"));
        let mut entries: HashMap<Arc<Path>, Vec<String>> = HashMap::default();
        entries.insert(
            worktree_a.clone(),
            vec!["".to_string(), "src".to_string(), "src/lib".to_string()],
        );
        entries.insert(worktree_b.clone(), vec!["".to_string()]);
        // Worktree c was explicitly collapsed (saved with empty expanded set).
        entries.insert(worktree_c.clone(), Vec::new());

        db.save_expanded_entries(workspace_id, entries.clone())
            .await
            .unwrap();

        let loaded = db.expanded_entries(workspace_id).unwrap();
        assert_eq!(loaded.len(), 3);
        let mut a = loaded.get(&worktree_a).cloned().unwrap();
        a.sort();
        assert_eq!(
            a,
            vec!["".to_string(), "src".to_string(), "src/lib".to_string()]
        );
        assert_eq!(
            loaded.get(&worktree_b).cloned().unwrap(),
            vec!["".to_string()]
        );
        assert_eq!(loaded.get(&worktree_c).cloned().unwrap(), Vec::<String>::new());

        // Saving with a subset of worktrees should upsert only those
        // worktrees, leaving rows for the others untouched. This guarantees
        // that a save scheduled before async worktree load completes cannot
        // accidentally drop saved state for not-yet-loaded worktrees.
        let mut entries: HashMap<Arc<Path>, Vec<String>> = HashMap::default();
        entries.insert(worktree_a.clone(), vec!["docs".to_string()]);
        db.save_expanded_entries(workspace_id, entries)
            .await
            .unwrap();

        let loaded = db.expanded_entries(workspace_id).unwrap();
        assert_eq!(loaded.len(), 3);
        assert_eq!(
            loaded.get(&worktree_a).cloned().unwrap(),
            vec!["docs".to_string()]
        );
        assert_eq!(
            loaded.get(&worktree_b).cloned().unwrap(),
            vec!["".to_string()]
        );
        assert_eq!(
            loaded.get(&worktree_c).cloned().unwrap(),
            Vec::<String>::new()
        );
    }
}
