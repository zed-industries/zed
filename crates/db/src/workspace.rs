mod items;
pub mod model;
pub(crate) mod pane;

use anyhow::Context;
use util::{iife, ResultExt};

use std::path::{Path, PathBuf};

use indoc::indoc;
use sqlez::migrations::Migration;

pub(crate) const WORKSPACES_MIGRATION: Migration = Migration::new(
    "workspace",
    &[indoc! {"
        CREATE TABLE workspaces(
            workspace_id BLOB PRIMARY KEY,
            dock_anchor TEXT, -- Enum: 'Bottom' / 'Right' / 'Expanded'
            dock_visible INTEGER, -- Boolean
            timestamp TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL
        ) STRICT;
    "}],
);

use self::model::{SerializedWorkspace, WorkspaceId, WorkspaceRow};

use super::Db;

impl Db {
    /// Returns a serialized workspace for the given worktree_roots. If the passed array
    /// is empty, the most recent workspace is returned instead. If no workspace for the
    /// passed roots is stored, returns none.
    pub fn workspace_for_roots<P: AsRef<Path>>(
        &self,
        worktree_roots: &[P],
    ) -> Option<SerializedWorkspace> {
        let workspace_id: WorkspaceId = worktree_roots.into();

        // Note that we re-assign the workspace_id here in case it's empty
        // and we've grabbed the most recent workspace
        let (workspace_id, dock_anchor, dock_visible) = iife!({
            if worktree_roots.len() == 0 {
                self.prepare(indoc! {"
                        SELECT workspace_id, dock_anchor, dock_visible 
                        FROM workspaces 
                        ORDER BY timestamp DESC LIMIT 1"})?
                    .maybe_row::<WorkspaceRow>()
            } else {
                self.prepare(indoc! {"
                        SELECT workspace_id, dock_anchor, dock_visible 
                        FROM workspaces 
                        WHERE workspace_id = ?"})?
                    .with_bindings(&workspace_id)?
                    .maybe_row::<WorkspaceRow>()
            }
        })
        .log_err()
        .flatten()?;

        Some(SerializedWorkspace {
            dock_pane: self.get_dock_pane(&workspace_id)?,
            center_group: self.get_center_group(&workspace_id),
            dock_anchor,
            dock_visible,
        })
    }

    /// Saves a workspace using the worktree roots. Will garbage collect any workspaces
    /// that used this workspace previously
    pub fn save_workspace<P: AsRef<Path>>(
        &self,
        worktree_roots: &[P],
        workspace: SerializedWorkspace,
    ) {
        let workspace_id: WorkspaceId = worktree_roots.into();

        self.with_savepoint("update_worktrees", |conn| {
            // Delete any previous workspaces with the same roots. This cascades to all
            // other tables that are based on the same roots set.
            // Insert new workspace into workspaces table if none were found
            self.prepare(indoc!{"
                DELETE FROM workspaces WHERE workspace_id = ?1;
                INSERT INTO workspaces(workspace_id, dock_anchor, dock_visible) VALUES (?1, ?, ?)"})?
            .with_bindings((&workspace_id, workspace.dock_anchor, workspace.dock_visible))?
            .exec()?;
            
            // Save center pane group and dock pane
            Self::save_center_group(&workspace_id, &workspace.center_group, conn)?;
            Self::save_dock_pane(&workspace_id, &workspace.dock_pane, conn)?;

            Ok(())
        })
        .with_context(|| format!("Update workspace with roots {:?}", worktree_roots.iter().map(|p| p.as_ref()).collect::<Vec<_>>()))
        .log_err();
    }

    /// Returns the previous workspace ids sorted by last modified along with their opened worktree roots
    pub fn recent_workspaces(&self, limit: usize) -> Vec<Vec<PathBuf>> {
        iife!({
            Ok::<_, anyhow::Error>(self.prepare("SELECT workspace_id FROM workspaces ORDER BY timestamp DESC LIMIT ?")?
                .with_bindings(limit)?
                .rows::<WorkspaceId>()?
                .into_iter().map(|id| id.0)
                .collect::<Vec<Vec<PathBuf>>>())
            
        }).log_err().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {

    // use std::{path::PathBuf, thread::sleep, time::Duration};

    // use crate::Db;

    // use super::WorkspaceId;

    // #[test]
    // fn test_workspace_saving() {
    //     env_logger::init();
    //     let db = Db::open_in_memory("test_new_worktrees_for_roots");

    //     // Test nothing returned with no roots at first
    //     assert_eq!(db.workspace_for_roots::<String>(&[]), None);

    //     // Test creation
    //     let workspace_1 = db.workspace_for_roots::<String>(&[]);
    //     assert_eq!(workspace_1.workspace_id, WorkspaceId(1));

    //     // Ensure the timestamps are different
    //     sleep(Duration::from_secs(1));
    //     db.make_new_workspace::<String>(&[]);

    //     // Test pulling another value from recent workspaces
    //     let workspace_2 = db.workspace_for_roots::<String>(&[]);
    //     assert_eq!(workspace_2.workspace_id, WorkspaceId(2));

    //     // Ensure the timestamps are different
    //     sleep(Duration::from_secs(1));

    //     // Test creating a new workspace that doesn't exist already
    //     let workspace_3 = db.workspace_for_roots(&["/tmp", "/tmp2"]);
    //     assert_eq!(workspace_3.workspace_id, WorkspaceId(3));

    //     // Make sure it's in the recent workspaces....
    //     let workspace_3 = db.workspace_for_roots::<String>(&[]);
    //     assert_eq!(workspace_3.workspace_id, WorkspaceId(3));

    //     // And that it can be pulled out again
    //     let workspace_3 = db.workspace_for_roots(&["/tmp", "/tmp2"]);
    //     assert_eq!(workspace_3.workspace_id, WorkspaceId(3));
    // }

    // #[test]
    // fn test_empty_worktrees() {
    //     let db = Db::open_in_memory("test_empty_worktrees");

    //     assert_eq!(None, db.workspace::<String>(&[]));

    //     db.make_new_workspace::<String>(&[]); //ID 1
    //     db.make_new_workspace::<String>(&[]); //ID 2
    //     db.update_worktrees(&WorkspaceId(1), &["/tmp", "/tmp2"]);

    //     // Sanity check
    //     assert_eq!(db.workspace(&["/tmp", "/tmp2"]).unwrap().0, WorkspaceId(1));

    //     db.update_worktrees::<String>(&WorkspaceId(1), &[]);

    //     // Make sure 'no worktrees' fails correctly. returning [1, 2] from this
    //     // call would be semantically correct (as those are the workspaces that
    //     // don't have roots) but I'd prefer that this API to either return exactly one
    //     // workspace, and None otherwise
    //     assert_eq!(db.workspace::<String>(&[]), None,);

    //     assert_eq!(db.last_workspace().unwrap().0, WorkspaceId(1));

    //     assert_eq!(
    //         db.recent_workspaces(2),
    //         vec![Vec::<PathBuf>::new(), Vec::<PathBuf>::new()],
    //     )
    // }

    // #[test]
    // fn test_more_workspace_ids() {
    //     let data = &[
    //         (WorkspaceId(1), vec!["/tmp1"]),
    //         (WorkspaceId(2), vec!["/tmp1", "/tmp2"]),
    //         (WorkspaceId(3), vec!["/tmp1", "/tmp2", "/tmp3"]),
    //         (WorkspaceId(4), vec!["/tmp2", "/tmp3"]),
    //         (WorkspaceId(5), vec!["/tmp2", "/tmp3", "/tmp4"]),
    //         (WorkspaceId(6), vec!["/tmp2", "/tmp4"]),
    //         (WorkspaceId(7), vec!["/tmp2"]),
    //     ];

    //     let db = Db::open_in_memory("test_more_workspace_ids");

    //     for (workspace_id, entries) in data {
    //         db.make_new_workspace::<String>(&[]);
    //         db.update_worktrees(workspace_id, entries);
    //     }

    //     assert_eq!(WorkspaceId(1), db.workspace(&["/tmp1"]).unwrap().0);
    //     assert_eq!(db.workspace(&["/tmp1", "/tmp2"]).unwrap().0, WorkspaceId(2));
    //     assert_eq!(
    //         db.workspace(&["/tmp1", "/tmp2", "/tmp3"]).unwrap().0,
    //         WorkspaceId(3)
    //     );
    //     assert_eq!(db.workspace(&["/tmp2", "/tmp3"]).unwrap().0, WorkspaceId(4));
    //     assert_eq!(
    //         db.workspace(&["/tmp2", "/tmp3", "/tmp4"]).unwrap().0,
    //         WorkspaceId(5)
    //     );
    //     assert_eq!(db.workspace(&["/tmp2", "/tmp4"]).unwrap().0, WorkspaceId(6));
    //     assert_eq!(db.workspace(&["/tmp2"]).unwrap().0, WorkspaceId(7));

    //     assert_eq!(db.workspace(&["/tmp1", "/tmp5"]), None);
    //     assert_eq!(db.workspace(&["/tmp5"]), None);
    //     assert_eq!(db.workspace(&["/tmp2", "/tmp3", "/tmp4", "/tmp5"]), None);
    // }

    // #[test]
    // fn test_detect_workspace_id() {
    //     let data = &[
    //         (WorkspaceId(1), vec!["/tmp"]),
    //         (WorkspaceId(2), vec!["/tmp", "/tmp2"]),
    //         (WorkspaceId(3), vec!["/tmp", "/tmp2", "/tmp3"]),
    //     ];

    //     let db = Db::open_in_memory("test_detect_workspace_id");

    //     for (workspace_id, entries) in data {
    //         db.make_new_workspace::<String>(&[]);
    //         db.update_worktrees(workspace_id, entries);
    //     }

    //     assert_eq!(db.workspace(&["/tmp2"]), None);
    //     assert_eq!(db.workspace(&["/tmp2", "/tmp3"]), None);
    //     assert_eq!(db.workspace(&["/tmp"]).unwrap().0, WorkspaceId(1));
    //     assert_eq!(db.workspace(&["/tmp", "/tmp2"]).unwrap().0, WorkspaceId(2));
    //     assert_eq!(
    //         db.workspace(&["/tmp", "/tmp2", "/tmp3"]).unwrap().0,
    //         WorkspaceId(3)
    //     );
    // }

    // #[test]
    // fn test_tricky_overlapping_updates() {
    //     // DB state:
    //     // (/tree) -> ID: 1
    //     // (/tree, /tree2) -> ID: 2
    //     // (/tree2, /tree3) -> ID: 3

    //     // -> User updates 2 to: (/tree2, /tree3)

    //     // DB state:
    //     // (/tree) -> ID: 1
    //     // (/tree2, /tree3) -> ID: 2
    //     // Get rid of 3 for garbage collection

    //     let data = &[
    //         (WorkspaceId(1), vec!["/tmp"]),
    //         (WorkspaceId(2), vec!["/tmp", "/tmp2"]),
    //         (WorkspaceId(3), vec!["/tmp2", "/tmp3"]),
    //     ];

    //     let db = Db::open_in_memory("test_tricky_overlapping_update");

    //     // Load in the test data
    //     for (workspace_id, entries) in data {
    //         db.make_new_workspace::<String>(&[]);
    //         db.update_worktrees(workspace_id, entries);
    //     }

    //     sleep(Duration::from_secs(1));
    //     // Execute the update
    //     db.update_worktrees(&WorkspaceId(2), &["/tmp2", "/tmp3"]);

    //     // Make sure that workspace 3 doesn't exist
    //     assert_eq!(db.workspace(&["/tmp2", "/tmp3"]).unwrap().0, WorkspaceId(2));

    //     // And that workspace 1 was untouched
    //     assert_eq!(db.workspace(&["/tmp"]).unwrap().0, WorkspaceId(1));

    //     // And that workspace 2 is no longer registered under these roots
    //     assert_eq!(db.workspace(&["/tmp", "/tmp2"]), None);

    //     assert_eq!(db.last_workspace().unwrap().0, WorkspaceId(2));

    //     let recent_workspaces = db.recent_workspaces(10);
    //     assert_eq!(
    //         recent_workspaces.get(0).unwrap(),
    //         &vec![PathBuf::from("/tmp2"), PathBuf::from("/tmp3")]
    //     );
    //     assert_eq!(
    //         recent_workspaces.get(1).unwrap(),
    //         &vec![PathBuf::from("/tmp")]
    //     );
    // }
}
