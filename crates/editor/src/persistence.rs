use std::path::PathBuf;

use crate::Editor;
use db::sqlez_macros::sql;
use db::{connection, query};
use sqlez::domain::Domain;
use workspace::{ItemId, Workspace, WorkspaceId};

connection!(DB: EditorDb<(Workspace, Editor)>);

impl Domain for Editor {
    fn name() -> &'static str {
        "editor"
    }

    fn migrations() -> &'static [&'static str] {
        &[sql! (
            CREATE TABLE editors(
                item_id INTEGER NOT NULL,
                workspace_id INTEGER NOT NULL,
                path BLOB NOT NULL,
                PRIMARY KEY(item_id, workspace_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            ) STRICT;
        )]
    }
}

impl EditorDb {
    query! {
        pub fn get_path(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
            SELECT path FROM editors
            WHERE item_id = ? AND workspace_id = ?
        }
    }

    query! {
        pub async fn save_path(item_id: ItemId, workspace_id: WorkspaceId, path: PathBuf) -> Result<()> {
            INSERT OR REPLACE INTO editors(item_id, workspace_id, path)
            VALUES (?, ?, ?)
        }
    }
}
