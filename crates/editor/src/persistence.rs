use std::path::PathBuf;

use db::{connection, sql_method};
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
                workspace_id INTEGER NOT NULL,
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
    sql_method! {
        get_path(item_id: ItemId, workspace_id: WorkspaceId) -> Result<PathBuf>:
            indoc! {"
                SELECT path FROM editors 
                WHERE item_id = ? AND workspace_id = ?"}
    }

    sql_method! {
        async save_path(item_id: ItemId, workspace_id: WorkspaceId, path: PathBuf) -> Result<()>:
            indoc! {"
                INSERT OR REPLACE INTO editors(item_id, workspace_id, path)
                VALUES (?, ?, ?)"}
    }
}
