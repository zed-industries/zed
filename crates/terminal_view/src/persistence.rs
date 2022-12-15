use std::path::PathBuf;

use db::{define_connection, query, sqlez_macros::sql};
use workspace::{ItemId, WorkspaceDb, WorkspaceId};

define_connection! {
    pub static ref TERMINAL_DB: TerminalDb<WorkspaceDb> =
        &[sql!(
            CREATE TABLE terminals (
                workspace_id INTEGER,
                item_id INTEGER UNIQUE,
                working_directory BLOB,
                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
            ) STRICT;
        )];
}

impl TerminalDb {
    query! {
       pub async fn update_workspace_id(
            new_id: WorkspaceId,
            old_id: WorkspaceId,
            item_id: ItemId
        ) -> Result<()> {
            UPDATE terminals
            SET workspace_id = ?
            WHERE workspace_id = ? AND item_id = ?
        }
    }

    query! {
        pub async fn save_working_directory(
            item_id: ItemId,
            workspace_id: WorkspaceId,
            working_directory: PathBuf
        ) -> Result<()> {
            INSERT OR REPLACE INTO terminals(item_id, workspace_id, working_directory)
            VALUES (?, ?, ?)
        }
    }

    query! {
        pub async fn take_working_directory(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
            DELETE FROM terminals
            WHERE item_id = ? AND workspace_id = ?
            RETURNING working_directory
        }
    }
}
