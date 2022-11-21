use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use db::{connection, exec_method};
use indoc::indoc;
use sqlez::domain::Domain;
use workspace::{ItemId, Workspace, WorkspaceId};

use crate::Editor;

connection!(DB: EditorDb<(Workspace, Editor)>);

impl Domain for Editor {
    fn name() -> &'static str {
        "editor"
    }

    fn migrations() -> &'static [&'static str] {
        &[indoc! {"
            CREATE TABLE editors(
                item_id INTEGER NOT NULL,
                workspace_id BLOB NOT NULL,
                path BLOB NOT NULL,
                PRIMARY KEY(item_id, workspace_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                    ON UPDATE CASCADE

            ) STRICT;
        "}]
    }
}

impl EditorDb {
    pub fn get_path(&self, item_id: ItemId, workspace_id: WorkspaceId) -> Result<PathBuf> {
        self.select_row_bound(indoc! {"
            SELECT path FROM editors 
            WHERE item_id = ? AND workspace_id = ?"})?((item_id, workspace_id))?
        .context("Path not found for serialized editor")
    }

    exec_method!(save_path(item_id: ItemId, workspace_id: WorkspaceId, path: &Path):
        "INSERT OR REPLACE INTO editors(item_id, workspace_id, path)
         VALUES (?, ?, ?)"
    );
}
