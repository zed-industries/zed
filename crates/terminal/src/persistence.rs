use std::path::{Path, PathBuf};

use db::{connection, indoc, sqlez::domain::Domain};
use util::{iife, ResultExt};
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
                item_id INTEGER,
                workspace_id BLOB,
                working_directory BLOB,
                PRIMARY KEY(item_id, workspace_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            ) STRICT;
        "}]
    }
}

impl TerminalDb {
    pub fn save_working_directory(
        &self,
        item_id: ItemId,
        workspace_id: &WorkspaceId,
        working_directory: &Path,
    ) {
        iife!({
            self.exec_bound::<(ItemId, &WorkspaceId, &Path)>(indoc! {"
                INSERT OR REPLACE INTO terminals(item_id, workspace_id, working_directory) 
                VALUES (?, ?, ?)  
            "})?((item_id, workspace_id, working_directory))
        })
        .log_err();
    }

    pub fn get_working_directory(
        &self,
        item_id: ItemId,
        workspace_id: &WorkspaceId,
    ) -> Option<PathBuf> {
        iife!({
            self.select_row_bound::<(ItemId, &WorkspaceId), PathBuf>(indoc! {"
            SELECT working_directory 
            FROM terminals 
            WHERE item_id = ? workspace_id = ?"})?((item_id, workspace_id))
        })
        .log_err()
        .flatten()
    }
}
