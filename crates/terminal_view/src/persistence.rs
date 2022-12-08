use std::path::PathBuf;

use db::{define_connection, query, sqlez_macros::sql};
use workspace::{WorkspaceDb, WorkspaceId};

type ModelId = usize;

define_connection! {
    pub static ref TERMINAL_DB: TerminalDb<WorkspaceDb> =
        &[sql!(
            CREATE TABLE terminals (
                workspace_id INTEGER,
                model_id INTEGER UNIQUE,
                working_directory BLOB,
                PRIMARY KEY(workspace_id, model_id),
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
            item_id: ModelId
        ) -> Result<()> {
            UPDATE terminals
            SET workspace_id = ?
            WHERE workspace_id = ? AND item_id = ?
        }
    }

    query! {
        pub async fn save_working_directory(
            item_id: ModelId,
            workspace_id: i64,
            working_directory: PathBuf
        ) -> Result<()> {
            INSERT OR REPLACE INTO terminals(item_id, workspace_id, working_directory)
            VALUES (?, ?, ?)
        }
    }

    query! {
        pub fn get_working_directory(item_id: ModelId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
            SELECT working_directory
            FROM terminals
            WHERE item_id = ? AND workspace_id = ?
        }
    }
}
