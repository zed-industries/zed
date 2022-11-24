use std::path::PathBuf;

use db::{connection, indoc, query, sqlez::domain::Domain};

use workspace::{ItemId, Workspace, WorkspaceId};

use crate::Terminal;

connection!(TERMINAL_CONNECTION: TerminalDb<(Workspace, Terminal)>);

impl Domain for Terminal {
    fn name() -> &'static str {
        "terminal"
    }

    fn migrations() -> &'static [&'static str] {
        &[indoc! {"
            CREATE TABLE terminals (
                workspace_id INTEGER,
                item_id INTEGER UNIQUE,
                working_directory BLOB,
                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
            ) STRICT;
        "}]
    }
}

impl TerminalDb {
    query! {
       pub async fn update_workspace_id(
            new_id: WorkspaceId,
            old_id: WorkspaceId,
            item_id: ItemId
        ) -> Result<()> {
            indoc!{"
                UPDATE terminals
                SET workspace_id = ?
                WHERE workspace_id = ? AND item_id = ?
            "}
        }
    }

    query! {
        pub async fn save_working_directory(
            item_id: ItemId,
            workspace_id: WorkspaceId,
            working_directory: PathBuf
        ) -> Result<()> {
            indoc!{"
                INSERT OR REPLACE INTO terminals(item_id, workspace_id, working_directory)
                VALUES (?1, ?2, ?3)
            "}
        }
    }

    query! {
        pub fn get_working_directory(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
            indoc!{"
                SELECT working_directory
                FROM terminals
                WHERE item_id = ? AND workspace_id = ?
            "}
        }
    }
}
