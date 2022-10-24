use anyhow::Result;

use std::{path::Path, sync::Arc};

use crate::pane::{PaneGroupId, PaneId, SerializedPane, SerializedPaneGroup};

use super::Db;

pub(crate) const WORKSPACE_M_1: &str = "
CREATE TABLE workspaces(
    workspace_id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT DEFAULT CURRENT_TIMESTAMP,
    dummy_data INTEGER
) STRICT;

CREATE TABLE worktree_roots(
    worktree_root BLOB NOT NULL,
    workspace_id INTEGER NOT NULL,
    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
) STRICT;
";

// Zed stores items with ids which are a combination of a view id during a given run and a workspace id. This

//      Case 1: Starting Zed Contextless
//          > Zed -> Reopen the last
//      Case 2: Starting Zed with a project folder
//          > Zed ~/projects/Zed
//      Case 3: Starting Zed with a file
//          > Zed ~/projects/Zed/cargo.toml
//      Case 4: Starting Zed with multiple project folders
//          > Zed ~/projects/Zed ~/projects/Zed.dev

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub struct WorkspaceId(i64);

struct WorkspaceRow {
    pub center_group_id: PaneGroupId,
    pub dock_pane_id: PaneId,
}

#[derive(Default)]
pub struct SerializedWorkspace {
    pub workspace_id: WorkspaceId,
    // pub center_group: SerializedPaneGroup,
    // pub dock_pane: Option<SerializedPane>,
}

impl Db {
    /// Finds or creates a workspace id for the given set of worktree roots. If the passed worktree roots is empty, return the
    /// the last workspace id
    pub fn workspace_for_worktree_roots(
        &self,
        worktree_roots: &[Arc<Path>],
    ) -> SerializedWorkspace {
        // Find the workspace id which is uniquely identified by this set of paths return it if found
        if let Some(workspace_id) = self.workspace_id(worktree_roots) {
            // TODO
            // let workspace_row = self.get_workspace_row(workspace_id);
            // let center_group = self.get_pane_group(workspace_row.center_group_id);
            // let dock_pane = self.get_pane(workspace_row.dock_pane_id);

            SerializedWorkspace {
                workspace_id,
                // center_group,
                // dock_pane: Some(dock_pane),
            }
        } else {
            self.make_new_workspace()
        }
    }

    fn make_new_workspace(&self) -> SerializedWorkspace {
        self.real()
            .map(|db| {
                let lock = db.connection.lock();
                match lock.execute("INSERT INTO workspaces(dummy_data) VALUES(1);", []) {
                    Ok(_) => SerializedWorkspace {
                        workspace_id: WorkspaceId(lock.last_insert_rowid()),
                    },
                    Err(_) => Default::default(),
                }
            })
            .unwrap_or_default()
    }

    fn workspace_id(&self, worktree_roots: &[Arc<Path>]) -> Option<WorkspaceId> {
        unimplemented!()
    }

    fn get_workspace_row(&self, workspace_id: WorkspaceId) -> WorkspaceRow {
        unimplemented!()
    }

    /// Updates the open paths for the given workspace id. Will garbage collect items from
    /// any workspace ids which are no replaced by the new workspace id. Updates the timestamps
    /// in the workspace id table
    pub fn update_worktree_roots(&self, workspace_id: &WorkspaceId, worktree_roots: &[Arc<Path>]) {
        // Lookup any WorkspaceIds which have the same set of roots, and delete them. (NOTE: this should garbage collect other tables)
        // Remove the old rows which contain workspace_id
        // Add rows for the new worktree_roots

        // zed /tree
        // -> add tree2
        //   -> udpate_worktree_roots() -> ADDs entries for /tree and /tree2, LEAVING BEHIND, the initial entry for /tree
        unimplemented!();
    }

    /// Returns the previous workspace ids sorted by last modified along with their opened worktree roots
    pub fn recent_workspaces(&self) -> Vec<(WorkspaceId, Vec<Arc<Path>>)> {
        // Return all the workspace ids and their associated paths ordered by the access timestamp
        //ORDER BY timestamps
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {

    use std::{
        path::{Path, PathBuf},
        sync::Arc,
    };

    use crate::Db;

    use super::WorkspaceId;

    fn arc_path(path: &'static str) -> Arc<Path> {
        PathBuf::from(path).into()
    }

    #[test]
    fn test_detect_workspace_id() {
        let data = &[
            (WorkspaceId(1), vec![arc_path("/tmp")]),
            (WorkspaceId(2), vec![arc_path("/tmp"), arc_path("/tmp2")]),
            (
                WorkspaceId(3),
                vec![arc_path("/tmp"), arc_path("/tmp2"), arc_path("/tmp3")],
            ),
        ];

        let db = Db::open_in_memory();

        for (workspace_id, entries) in data {
            db.update_worktree_roots(workspace_id, entries); //??
        }

        assert_eq!(None, db.workspace_id(&[arc_path("/tmp2")]));
        assert_eq!(
            None,
            db.workspace_id(&[arc_path("/tmp2"), arc_path("/tmp3")])
        );
        assert_eq!(Some(WorkspaceId(1)), db.workspace_id(&[arc_path("/tmp")]));
        assert_eq!(
            Some(WorkspaceId(2)),
            db.workspace_id(&[arc_path("/tmp"), arc_path("/tmp2")])
        );
        assert_eq!(
            Some(WorkspaceId(3)),
            db.workspace_id(&[arc_path("/tmp"), arc_path("/tmp2"), arc_path("/tmp3")])
        );
    }

    #[test]
    fn test_tricky_overlapping_updates() {
        // DB state:
        // (/tree) -> ID: 1
        // (/tree, /tree2) -> ID: 2
        // (/tree2, /tree3) -> ID: 3

        // -> User updates 2 to: (/tree2, /tree3)

        // DB state:
        // (/tree) -> ID: 1
        // (/tree2, /tree3) -> ID: 2
        // Get rid of 3 for garbage collection

        let data = &[
            (WorkspaceId(1), vec![arc_path("/tmp")]),
            (WorkspaceId(2), vec![arc_path("/tmp"), arc_path("/tmp2")]),
            (WorkspaceId(3), vec![arc_path("/tmp2"), arc_path("/tmp3")]),
        ];

        let db = Db::open_in_memory();

        for (workspace_id, entries) in data {
            db.update_worktree_roots(workspace_id, entries); //??
            assert_eq!(&db.workspace_id(&[]), &Some(*workspace_id))
        }

        for (workspace_id, entries) in data {
            assert_eq!(&db.workspace_id(entries.as_slice()), &Some(*workspace_id));
        }

        db.update_worktree_roots(&WorkspaceId(2), &[arc_path("/tmp2")]);
        // todo!(); // make sure that 3 got garbage collected

        assert_eq!(db.workspace_id(&[arc_path("/tmp2")]), Some(WorkspaceId(2)));
        assert_eq!(db.workspace_id(&[arc_path("/tmp")]), Some(WorkspaceId(1)));

        let recent_workspaces = db.recent_workspaces();
        assert_eq!(recent_workspaces.get(0).unwrap().0, WorkspaceId(2));
        assert_eq!(recent_workspaces.get(1).unwrap().0, WorkspaceId(3));
        assert_eq!(recent_workspaces.get(2).unwrap().0, WorkspaceId(1));
    }
}
