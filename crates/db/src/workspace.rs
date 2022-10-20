use std::{path::Path, sync::Arc};

use super::Db;

pub(crate) const WORKSPACE_M_1: &str = "
CREATE TABLE workspaces(
    workspace_id INTEGER PRIMARY KEY,
    center_group INTEGER NOT NULL,
    dock_pane INTEGER NOT NULL,
    timestamp INTEGER,
    FOREIGN KEY(center_group) REFERENCES pane_groups(group_id)
    FOREIGN KEY(dock_pane) REFERENCES pane_items(pane_id)
) STRICT;

CREATE TABLE worktree_roots(
    worktree_root BLOB NOT NULL,
    workspace_id INTEGER NOT NULL,
    FOREIGN KEY(workspace_id) REFERENCES workspace_ids(workspace_id)
) STRICT;

CREATE TABLE pane_groups(
    workspace_id INTEGER,
    group_id INTEGER,
    split_direction STRING, -- 'Vertical' / 'Horizontal' /
    PRIMARY KEY (workspace_id, group_id)
) STRICT;

CREATE TABLE pane_group_children(
    workspace_id INTEGER,
    group_id INTEGER,
    child_pane_id INTEGER,  -- Nullable
    child_group_id INTEGER, -- Nullable
    PRIMARY KEY (workspace_id, group_id)
) STRICT;

CREATE TABLE pane_items(
    workspace_id INTEGER,
    pane_id INTEGER,
    item_id INTEGER, -- Array
    PRIMARY KEY (workspace_id, pane_id)
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

#[derive(Debug, PartialEq, Eq)]
pub struct WorkspaceId(usize);

impl Db {
    /// Finds or creates a workspace id for the given set of worktree roots. If the passed worktree roots is empty, return the
    /// the last workspace id
    pub fn workspace_id(&self, worktree_roots: &[Arc<Path>]) -> WorkspaceId {
        // Find the workspace id which is uniquely identified by this set of paths return it if found
        // Otherwise:
        //   Find the max workspace_id and increment it as our new workspace id
        //   Store in the worktrees table the mapping from this new id to the set of worktree roots
        unimplemented!();
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

    /// Returns the previous workspace ids sorted by last modified
    pub fn recent_workspaces(&self) -> Vec<(WorkspaceId, Vec<Arc<Path>>)> {
        // Return all the workspace ids and their associated paths ordered by the access timestamp
        //ORDER BY timestamps
        unimplemented!();
    }

    pub fn center_pane(&self, workspace: WorkspaceId) -> SerializedPaneGroup {}

    pub fn dock_pane(&self, workspace: WorkspaceId) -> SerializedPane {}
}

#[cfg(test)]
mod tests {

    use std::{
        path::{Path, PathBuf},
        sync::Arc,
    };

    use crate::Db;

    use super::WorkspaceId;

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

        fn arc_path(path: &'static str) -> Arc<Path> {
            PathBuf::from(path).into()
        }

        let data = &[
            (WorkspaceId(1), vec![arc_path("/tmp")]),
            (WorkspaceId(2), vec![arc_path("/tmp"), arc_path("/tmp2")]),
            (WorkspaceId(3), vec![arc_path("/tmp2"), arc_path("/tmp3")]),
        ];

        let db = Db::open_in_memory();

        for (workspace_id, entries) in data {
            db.update_worktree_roots(workspace_id, entries); //??
            assert_eq!(&db.workspace_id(&[]), workspace_id)
        }

        for (workspace_id, entries) in data {
            assert_eq!(&db.workspace_id(entries.as_slice()), workspace_id);
        }

        db.update_worktree_roots(&WorkspaceId(2), &[arc_path("/tmp2")]);
        // todo!(); // make sure that 3 got garbage collected

        assert_eq!(db.workspace_id(&[arc_path("/tmp2")]), WorkspaceId(2));
        assert_eq!(db.workspace_id(&[arc_path("/tmp")]), WorkspaceId(1));

        let recent_workspaces = db.recent_workspaces();
        assert_eq!(recent_workspaces.get(0).unwrap().0, WorkspaceId(2));
        assert_eq!(recent_workspaces.get(1).unwrap().0, WorkspaceId(3));
        assert_eq!(recent_workspaces.get(2).unwrap().0, WorkspaceId(1));
    }
}

// [/tmp, /tmp2] -> ID1?
// [/tmp] -> ID2?

/*
path | id
/tmp   ID1
/tmp   ID2
/tmp2  ID1


SELECT id
FROM workspace_ids
WHERE path IN (path1, path2)
INTERSECT
SELECT id
FROM workspace_ids
WHERE path = path_2
... and etc. for each element in path array

If contains row, yay! If not,
SELECT max(id) FROm workspace_ids

Select id WHERE path IN paths

SELECT MAX(id)

*/
