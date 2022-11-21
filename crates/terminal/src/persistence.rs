use std::path::{Path, PathBuf};

use db::{connection, exec_method, indoc, select_row_method, sqlez::domain::Domain};

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
                item_id INTEGER,
                working_directory BLOB,
                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
            ) STRICT;
        "}]
    }
}

impl TerminalDb {
    exec_method!(
        save_working_directory(item_id: ItemId, workspace_id: WorkspaceId, working_directory: &Path):
            "INSERT OR REPLACE INTO terminals(item_id, workspace_id, working_directory)
             VALUES (?1, ?2, ?3)"
    );

    select_row_method!(
        get_working_directory(item_id: ItemId, workspace_id: WorkspaceId) -> PathBuf:
            "SELECT working_directory
             FROM terminals 
             WHERE item_id = ? AND workspace_id = ?"
    );
}
