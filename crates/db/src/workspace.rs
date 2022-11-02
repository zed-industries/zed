use anyhow::{bail, Context, Result};
use util::{iife, ResultExt};

use std::{
    fmt::Debug,
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
};

use indoc::indoc;
use sqlez::{
    bindable::{Bind, Column},
    connection::Connection,
    migrations::Migration,
    statement::Statement,
};

use crate::pane::{SerializedDockPane, SerializedPaneGroup};

use super::Db;

// If you need to debug the worktree root code, change 'BLOB' here to 'TEXT' for easier debugging
// you might want to update some of the parsing code as well, I've left the variations in but commented
// out. This will panic if run on an existing db that has already been migrated
pub(crate) const WORKSPACES_MIGRATION: Migration = Migration::new(
    "workspace",
    &[indoc! {"
        CREATE TABLE workspaces(
            workspace_id INTEGER PRIMARY KEY,
            dock_anchor TEXT, -- Enum: 'Bottom' / 'Right' / 'Expanded'
            dock_visible INTEGER, -- Boolean
            timestamp TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL
        ) STRICT;
        
        CREATE TABLE worktree_roots(
            worktree_root BLOB NOT NULL,
            workspace_id INTEGER NOT NULL,
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE
            PRIMARY KEY(worktree_root, workspace_id)
        ) STRICT;"}],
);

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub struct WorkspaceId(i64);

impl Bind for WorkspaceId {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        self.0.bind(statement, start_index)
    }
}

impl Column for WorkspaceId {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        i64::column(statement, start_index).map(|(id, next_index)| (Self(id), next_index))
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DockAnchor {
    #[default]
    Bottom,
    Right,
    Expanded,
}

impl Bind for DockAnchor {
    fn bind(&self, statement: &Statement, start_index: i32) -> anyhow::Result<i32> {
        match self {
            DockAnchor::Bottom => "Bottom",
            DockAnchor::Right => "Right",
            DockAnchor::Expanded => "Expanded",
        }
        .bind(statement, start_index)
    }
}

impl Column for DockAnchor {
    fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
        String::column(statement, start_index).and_then(|(anchor_text, next_index)| {
            Ok((
                match anchor_text.as_ref() {
                    "Bottom" => DockAnchor::Bottom,
                    "Right" => DockAnchor::Right,
                    "Expanded" => DockAnchor::Expanded,
                    _ => bail!("Stored dock anchor is incorrect"),
                },
                next_index,
            ))
        })
    }
}

type WorkspaceRow = (WorkspaceId, DockAnchor, bool);

#[derive(Default, Debug)]
pub struct SerializedWorkspace {
    pub center_group: SerializedPaneGroup,
    pub dock_anchor: DockAnchor,
    pub dock_visible: bool,
    pub dock_pane: SerializedDockPane,
}

impl Db {
    /// Finds or creates a workspace id for the given set of worktree roots. If the passed worktree roots is empty,
    /// returns the last workspace which was updated

    pub fn workspace_for_roots<P>(&self, worktree_roots: &[P]) -> Option<SerializedWorkspace>
    where
        P: AsRef<Path> + Debug,
    {
        // Find the workspace id which is uniquely identified by this set of paths
        // return it if found
        let mut workspace_row = get_workspace(worktree_roots, &self)
            .log_err()
            .unwrap_or_default();
        if workspace_row.is_none() && worktree_roots.len() == 0 {
            workspace_row = self.last_workspace();
        }

        workspace_row.and_then(|(workspace_id, dock_anchor, dock_visible)| {
            Some(SerializedWorkspace {
                dock_pane: self.get_dock_pane(workspace_id)?,
                center_group: self.get_center_group(workspace_id),
                dock_anchor,
                dock_visible,
            })
        })
    }

    /// Updates the open paths for the given workspace id. Will garbage collect items from
    /// any workspace ids which are no replaced by the new workspace id. Updates the timestamps
    /// in the workspace id table
    pub fn update_worktrees<P>(&self, workspace_id: &WorkspaceId, worktree_roots: &[P])
    where
        P: AsRef<Path> + Debug,
    {
        self.with_savepoint("update_worktrees", |conn| {
            // Lookup any old WorkspaceIds which have the same set of roots, and delete them.
            let preexisting_workspace = get_workspace(worktree_roots, &conn)?;
            if let Some((preexisting_workspace_id, _, _)) = preexisting_workspace {
                if preexisting_workspace_id != *workspace_id {
                    // Should also delete fields in other tables with cascading updates
                    conn.prepare("DELETE FROM workspaces WHERE workspace_id = ?")?
                        .with_bindings(preexisting_workspace_id)?
                        .exec()?;
                }
            }

            conn.prepare("DELETE FROM worktree_roots WHERE workspace_id = ?")?
                .with_bindings(workspace_id.0)?
                .exec()?;

            for root in worktree_roots {
                let path = root.as_ref().as_os_str().as_bytes();
                // If you need to debug this, here's the string parsing:
                // let path = root.as_ref().to_string_lossy().to_string();

                conn.prepare(
                    "INSERT INTO worktree_roots(workspace_id, worktree_root) VALUES (?, ?)",
                )?
                .with_bindings((workspace_id.0, path))?
                .exec()?;
            }

            conn.prepare(
                "UPDATE workspaces SET timestamp = CURRENT_TIMESTAMP WHERE workspace_id = ?",
            )?
            .with_bindings(workspace_id.0)?
            .exec()?;

            Ok(())
        })
        .context("Update workspace {workspace_id:?} with roots {worktree_roots:?}")
        .log_err();
    }

    fn last_workspace(&self) -> Option<WorkspaceRow> {
        iife! ({
            self.prepare("SELECT workspace_id, dock_anchor, dock_visible FROM workspaces ORDER BY timestamp DESC LIMIT 1")?
                .maybe_row::<WorkspaceRow>()
        }).log_err()?
    }

    /// Returns the previous workspace ids sorted by last modified along with their opened worktree roots
    pub fn recent_workspaces(&self, limit: usize) -> Vec<Vec<PathBuf>> {
        self.with_savepoint("recent_workspaces", |conn| {
            let mut stmt =
                conn.prepare("SELECT worktree_root FROM worktree_roots WHERE workspace_id = ?")?;

            conn.prepare("SELECT workspace_id FROM workspaces ORDER BY timestamp DESC LIMIT ?")?
                .with_bindings(limit)?
                .rows::<WorkspaceId>()?
                .iter()
                .map(|workspace_id| stmt.with_bindings(workspace_id.0)?.rows::<PathBuf>())
                .collect::<Result<_>>()
        })
        .log_err()
        .unwrap_or_default()
    }
}

fn get_workspace<P>(worktree_roots: &[P], connection: &Connection) -> Result<Option<WorkspaceRow>>
where
    P: AsRef<Path> + Debug,
{
    // Short circuit if we can
    if worktree_roots.len() == 0 {
        return Ok(None);
    }

    // Prepare the array binding string. SQL doesn't have syntax for this, so
    // we have to do it ourselves.
    let array_binding_stmt = format!(
        "({})",
        (0..worktree_roots.len())
            .map(|index| format!("?{}", index + 1))
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Any workspace can have multiple independent paths, and these paths
    // can overlap in the database. Take this test data for example:
    //
    // [/tmp, /tmp2] -> 1
    // [/tmp] -> 2
    // [/tmp2, /tmp3] -> 3
    //
    // This would be stred in the database like so:
    //
    // ID PATH
    // 1  /tmp
    // 1  /tmp2
    // 2  /tmp
    // 3  /tmp2
    // 3  /tmp3
    //
    // Note how both /tmp and /tmp2 are associated with multiple workspace IDs.
    // So, given an array of worktree roots, how can we find the exactly matching ID?
    // Let's analyze what happens when querying for [/tmp, /tmp2], from the inside out:
    //  - We start with a join of this table on itself, generating every possible
    //    pair of ((path, ID), (path, ID)), and filtering the join down to just the
    //    *overlapping but non-matching* workspace IDs. For this small data set,
    //    this would look like:
    //
    //    wt1.ID wt1.PATH | wt2.ID wt2.PATH
    //    3      /tmp3      3      /tmp2
    //
    //  - Moving one SELECT out, we use the first pair's ID column to invert the selection,
    //    meaning we now have a list of all the entries for our array, minus overlapping sets,
    //    but including *subsets* of our worktree roots:
    //
    //    ID PATH
    //    1  /tmp
    //    1  /tmp2
    //    2  /tmp
    //
    // - To trim out the subsets, we can to exploit the PRIMARY KEY constraint that there are no
    //   duplicate entries in this table. Using a GROUP BY and a COUNT we can find the subsets of
    //   our keys:
    //
    //    ID num_matching
    //    1  2
    //    2  1
    //
    // - And with one final WHERE num_matching = $num_of_worktree_roots, we're done! We've found the
    //   matching ID correctly :D
    //
    // Note: due to limitations in SQLite's query binding, we have to generate the prepared
    //       statement with string substitution (the {array_bind}) below, and then bind the
    //       parameters by number.
    let query = format!(
        r#"
        SELECT workspaces.workspace_id, workspaces.dock_anchor, workspaces.dock_visible
        FROM (SELECT workspace_id
              FROM (SELECT count(workspace_id) as num_matching, workspace_id FROM worktree_roots
                    WHERE worktree_root in {array_bind} AND workspace_id NOT IN
                      (SELECT wt1.workspace_id FROM worktree_roots as wt1
                       JOIN worktree_roots as wt2
                       ON wt1.workspace_id = wt2.workspace_id
                       WHERE wt1.worktree_root NOT in {array_bind} AND wt2.worktree_root in {array_bind})
                    GROUP BY workspace_id)
              WHERE num_matching = ?) as matching_workspace
        JOIN workspaces ON workspaces.workspace_id = matching_workspace.workspace_id
        "#,
        array_bind = array_binding_stmt
    );

    // This will only be called on start up and when root workspaces change, no need to waste memory
    // caching it.
    let mut stmt = connection.prepare(&query)?;

    // Make sure we bound the parameters correctly
    debug_assert!(worktree_roots.len() as i32 + 1 == stmt.parameter_count());

    let root_bytes: Vec<&[u8]> = worktree_roots
        .iter()
        .map(|root| root.as_ref().as_os_str().as_bytes())
        .collect();

    let num_of_roots = root_bytes.len();

    stmt.with_bindings((root_bytes, num_of_roots))?
        .maybe_row::<WorkspaceRow>()
}

#[cfg(test)]
mod tests {

    use std::{path::PathBuf, thread::sleep, time::Duration};

    use crate::Db;

    use super::WorkspaceId;

    #[test]
    fn test_new_worktrees_for_roots() {
        env_logger::init();
        let db = Db::open_in_memory("test_new_worktrees_for_roots");

        // Test creation in 0 case
        let workspace_1 = db.workspace_for_roots::<String>(&[]);
        assert_eq!(workspace_1.workspace_id, WorkspaceId(1));

        // Test pulling from recent workspaces
        let workspace_1 = db.workspace_for_roots::<String>(&[]);
        assert_eq!(workspace_1.workspace_id, WorkspaceId(1));

        // Ensure the timestamps are different
        sleep(Duration::from_secs(1));
        db.make_new_workspace::<String>(&[]);

        // Test pulling another value from recent workspaces
        let workspace_2 = db.workspace_for_roots::<String>(&[]);
        assert_eq!(workspace_2.workspace_id, WorkspaceId(2));

        // Ensure the timestamps are different
        sleep(Duration::from_secs(1));

        // Test creating a new workspace that doesn't exist already
        let workspace_3 = db.workspace_for_roots(&["/tmp", "/tmp2"]);
        assert_eq!(workspace_3.workspace_id, WorkspaceId(3));

        // Make sure it's in the recent workspaces....
        let workspace_3 = db.workspace_for_roots::<String>(&[]);
        assert_eq!(workspace_3.workspace_id, WorkspaceId(3));

        // And that it can be pulled out again
        let workspace_3 = db.workspace_for_roots(&["/tmp", "/tmp2"]);
        assert_eq!(workspace_3.workspace_id, WorkspaceId(3));
    }

    #[test]
    fn test_empty_worktrees() {
        let db = Db::open_in_memory("test_empty_worktrees");

        assert_eq!(None, db.workspace::<String>(&[]));

        db.make_new_workspace::<String>(&[]); //ID 1
        db.make_new_workspace::<String>(&[]); //ID 2
        db.update_worktrees(&WorkspaceId(1), &["/tmp", "/tmp2"]);

        // Sanity check
        assert_eq!(db.workspace(&["/tmp", "/tmp2"]).unwrap().0, WorkspaceId(1));

        db.update_worktrees::<String>(&WorkspaceId(1), &[]);

        // Make sure 'no worktrees' fails correctly. returning [1, 2] from this
        // call would be semantically correct (as those are the workspaces that
        // don't have roots) but I'd prefer that this API to either return exactly one
        // workspace, and None otherwise
        assert_eq!(db.workspace::<String>(&[]), None,);

        assert_eq!(db.last_workspace().unwrap().0, WorkspaceId(1));

        assert_eq!(
            db.recent_workspaces(2),
            vec![Vec::<PathBuf>::new(), Vec::<PathBuf>::new()],
        )
    }

    #[test]
    fn test_more_workspace_ids() {
        let data = &[
            (WorkspaceId(1), vec!["/tmp1"]),
            (WorkspaceId(2), vec!["/tmp1", "/tmp2"]),
            (WorkspaceId(3), vec!["/tmp1", "/tmp2", "/tmp3"]),
            (WorkspaceId(4), vec!["/tmp2", "/tmp3"]),
            (WorkspaceId(5), vec!["/tmp2", "/tmp3", "/tmp4"]),
            (WorkspaceId(6), vec!["/tmp2", "/tmp4"]),
            (WorkspaceId(7), vec!["/tmp2"]),
        ];

        let db = Db::open_in_memory("test_more_workspace_ids");

        for (workspace_id, entries) in data {
            db.make_new_workspace::<String>(&[]);
            db.update_worktrees(workspace_id, entries);
        }

        assert_eq!(WorkspaceId(1), db.workspace(&["/tmp1"]).unwrap().0);
        assert_eq!(db.workspace(&["/tmp1", "/tmp2"]).unwrap().0, WorkspaceId(2));
        assert_eq!(
            db.workspace(&["/tmp1", "/tmp2", "/tmp3"]).unwrap().0,
            WorkspaceId(3)
        );
        assert_eq!(db.workspace(&["/tmp2", "/tmp3"]).unwrap().0, WorkspaceId(4));
        assert_eq!(
            db.workspace(&["/tmp2", "/tmp3", "/tmp4"]).unwrap().0,
            WorkspaceId(5)
        );
        assert_eq!(db.workspace(&["/tmp2", "/tmp4"]).unwrap().0, WorkspaceId(6));
        assert_eq!(db.workspace(&["/tmp2"]).unwrap().0, WorkspaceId(7));

        assert_eq!(db.workspace(&["/tmp1", "/tmp5"]), None);
        assert_eq!(db.workspace(&["/tmp5"]), None);
        assert_eq!(db.workspace(&["/tmp2", "/tmp3", "/tmp4", "/tmp5"]), None);
    }

    #[test]
    fn test_detect_workspace_id() {
        let data = &[
            (WorkspaceId(1), vec!["/tmp"]),
            (WorkspaceId(2), vec!["/tmp", "/tmp2"]),
            (WorkspaceId(3), vec!["/tmp", "/tmp2", "/tmp3"]),
        ];

        let db = Db::open_in_memory("test_detect_workspace_id");

        for (workspace_id, entries) in data {
            db.make_new_workspace::<String>(&[]);
            db.update_worktrees(workspace_id, entries);
        }

        assert_eq!(db.workspace(&["/tmp2"]), None);
        assert_eq!(db.workspace(&["/tmp2", "/tmp3"]), None);
        assert_eq!(db.workspace(&["/tmp"]).unwrap().0, WorkspaceId(1));
        assert_eq!(db.workspace(&["/tmp", "/tmp2"]).unwrap().0, WorkspaceId(2));
        assert_eq!(
            db.workspace(&["/tmp", "/tmp2", "/tmp3"]).unwrap().0,
            WorkspaceId(3)
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
            (WorkspaceId(1), vec!["/tmp"]),
            (WorkspaceId(2), vec!["/tmp", "/tmp2"]),
            (WorkspaceId(3), vec!["/tmp2", "/tmp3"]),
        ];

        let db = Db::open_in_memory("test_tricky_overlapping_update");

        // Load in the test data
        for (workspace_id, entries) in data {
            db.make_new_workspace::<String>(&[]);
            db.update_worktrees(workspace_id, entries);
        }

        sleep(Duration::from_secs(1));
        // Execute the update
        db.update_worktrees(&WorkspaceId(2), &["/tmp2", "/tmp3"]);

        // Make sure that workspace 3 doesn't exist
        assert_eq!(db.workspace(&["/tmp2", "/tmp3"]).unwrap().0, WorkspaceId(2));

        // And that workspace 1 was untouched
        assert_eq!(db.workspace(&["/tmp"]).unwrap().0, WorkspaceId(1));

        // And that workspace 2 is no longer registered under these roots
        assert_eq!(db.workspace(&["/tmp", "/tmp2"]), None);

        assert_eq!(db.last_workspace().unwrap().0, WorkspaceId(2));

        let recent_workspaces = db.recent_workspaces(10);
        assert_eq!(
            recent_workspaces.get(0).unwrap(),
            &vec![PathBuf::from("/tmp2"), PathBuf::from("/tmp3")]
        );
        assert_eq!(
            recent_workspaces.get(1).unwrap(),
            &vec![PathBuf::from("/tmp")]
        );
    }
}
