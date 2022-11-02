use anyhow::{Result, anyhow};

use std::{
    ffi::OsStr,
    fmt::Debug,
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
    sync::Arc,
};

use indoc::indoc;
use sqlez::{
    connection::Connection, migrations::Migration, bindable::{Column, Bind},
};

use crate::pane::SerializedDockPane;

use super::Db;

// If you need to debug the worktree root code, change 'BLOB' here to 'TEXT' for easier debugging
// you might want to update some of the parsing code as well, I've left the variations in but commented
// out. This will panic if run on an existing db that has already been migrated
pub(crate) const WORKSPACES_MIGRATION: Migration = Migration::new(
    "workspace",
    &[indoc! {"
        CREATE TABLE workspaces(
            workspace_id INTEGER PRIMARY KEY,
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

impl WorkspaceId {
    pub fn raw_id(&self) -> i64 {
        self.0
    }
}

impl Bind for WorkspaceId {
    fn bind(&self, statement: &sqlez::statement::Statement, start_index: i32) -> Result<i32> {
        todo!();
    }
}

impl Column for WorkspaceId {
    fn column(statement: &mut sqlez::statement::Statement, start_index: i32) -> Result<(Self, i32)> {
       todo!();
    }
}

#[derive(Default, Debug)]
pub struct SerializedWorkspace {
    pub workspace_id: WorkspaceId,
    // pub center_group: SerializedPaneGroup,
    pub dock_pane: Option<SerializedDockPane>,
}

impl Db {
    /// Finds or creates a workspace id for the given set of worktree roots. If the passed worktree roots is empty,
    /// returns the last workspace which was updated
    pub fn workspace_for_roots<P>(&self, worktree_roots: &[P]) -> SerializedWorkspace
    where
        P: AsRef<Path> + Debug,
    {
        // Find the workspace id which is uniquely identified by this set of paths
        // return it if found
        let mut workspace_id = self.workspace_id(worktree_roots);
        if workspace_id.is_none() && worktree_roots.len() == 0 {
            workspace_id = self.last_workspace_id();
        }

        if let Some(workspace_id) = workspace_id {
            SerializedWorkspace {
                workspace_id,
                dock_pane: self.get_dock_pane(workspace_id),
            }
        } else {
            self.make_new_workspace(worktree_roots)
        }
    }

    fn make_new_workspace<P>(&self, worktree_roots: &[P]) -> SerializedWorkspace
    where
        P: AsRef<Path> + Debug,
    {
        let res = self.with_savepoint("make_new_workspace", |conn| {
            let workspace_id = WorkspaceId(
                conn.prepare("INSERT INTO workspaces DEFAULT VALUES")?
                    .insert()?,
            );

            update_worktree_roots(conn, &workspace_id, worktree_roots)?;

            Ok(SerializedWorkspace {
                workspace_id,
                dock_pane: None,
            })
        });

        match res {
            Ok(serialized_workspace) => serialized_workspace,
            Err(err) => {
                log::error!("Failed to insert new workspace into DB: {}", err);
                Default::default()
            }
        }
    }

    fn workspace_id<P>(&self, worktree_roots: &[P]) -> Option<WorkspaceId>
    where
        P: AsRef<Path> + Debug,
    {
        match get_workspace_id(worktree_roots, &self) {
            Ok(workspace_id) => workspace_id,
            Err(err) => {
                log::error!("Failed to get workspace_id: {}", err);
                None
            }
        }
    }

    // fn get_workspace_row(&self, workspace_id: WorkspaceId) -> WorkspaceRow {
    //     unimplemented!()
    // }

    /// Updates the open paths for the given workspace id. Will garbage collect items from
    /// any workspace ids which are no replaced by the new workspace id. Updates the timestamps
    /// in the workspace id table
    pub fn update_worktrees<P>(&self, workspace_id: &WorkspaceId, worktree_roots: &[P])
    where
        P: AsRef<Path> + Debug,
    {
        match self.with_savepoint("update_worktrees", |conn| {
            update_worktree_roots(conn, workspace_id, worktree_roots)
        }) {
            Ok(_) => {}
            Err(err) => log::error!(
                "Failed to update workspace {:?} with roots {:?}, error: {}",
                workspace_id,
                worktree_roots,
                err
            ),
        }
    }

    fn last_workspace_id(&self) -> Option<WorkspaceId> {
        let res = self
            .prepare(
                "SELECT workspace_id FROM workspaces ORDER BY last_opened_timestamp DESC LIMIT 1",
            )
            .and_then(|stmt| stmt.maybe_row())
            .map(|row| row.map(|id| WorkspaceId(id)));

        match res {
            Ok(result) => result,
            Err(err) => {
                log::error!("Failed to get last workspace id, err: {}", err);
                return None;
            }
        }
    }

    /// Returns the previous workspace ids sorted by last modified along with their opened worktree roots
    pub fn recent_workspaces(&self, limit: usize) -> Vec<(WorkspaceId, Vec<Arc<Path>>)> {
        self.with_savepoint("recent_workspaces", |conn| {
            let ids = conn.prepare("SELECT workspace_id FROM workspaces ORDER BY last_opened_timestamp DESC LIMIT ?")?
                .with_bindings(limit)?
                .rows::<i64>()?
                .iter()
                .map(|row| WorkspaceId(*row));
            
            let result = Vec::new();
            
            let stmt = conn.prepare("SELECT worktree_root FROM worktree_roots WHERE workspace_id = ?")?;
            for workspace_id in ids {
                let roots = stmt.with_bindings(workspace_id.0)?
                    .rows::<Vec<u8>>()?
                    .iter()
                    .map(|row| {
                        PathBuf::from(OsStr::from_bytes(&row)).into()
                    })
                    .collect();
                result.push((workspace_id, roots))
            }
            
            Ok(result)
        }).unwrap_or_else(|err| {
            log::error!("Failed to get recent workspaces, err: {}", err);
            Vec::new()
        })
    }
}

fn update_worktree_roots<P>(
    connection: &Connection,
    workspace_id: &WorkspaceId,
    worktree_roots: &[P],
) -> Result<()>
where
    P: AsRef<Path> + Debug,
{
    // Lookup any old WorkspaceIds which have the same set of roots, and delete them.
    let preexisting_id = get_workspace_id(worktree_roots, &connection)?;
    if let Some(preexisting_id) = preexisting_id {
        if preexisting_id != *workspace_id {
            // Should also delete fields in other tables with cascading updates
            connection.prepare(
                "DELETE FROM workspaces WHERE workspace_id = ?",
            )?
            .with_bindings(preexisting_id.0)?
            .exec()?;
        }
    }

    connection
        .prepare("DELETE FROM worktree_roots WHERE workspace_id = ?")?
        .with_bindings(workspace_id.0)?
        .exec()?;

    for root in worktree_roots {
        let path = root.as_ref().as_os_str().as_bytes();
        // If you need to debug this, here's the string parsing:
        // let path = root.as_ref().to_string_lossy().to_string();

        connection.prepare("INSERT INTO worktree_roots(workspace_id, worktree_root) VALUES (?, ?)")?
            .with_bindings((workspace_id.0, path))?
            .exec()?;
    }

    connection.prepare("UPDATE workspaces SET last_opened_timestamp = CURRENT_TIMESTAMP WHERE workspace_id = ?")?
        .with_bindings(workspace_id.0)?
        .exec()?;

    Ok(())
}

fn get_workspace_id<P>(worktree_roots: &[P], connection: &Connection) -> Result<Option<WorkspaceId>>
where
    P: AsRef<Path> + Debug,
{
    // Short circuit if we can
    if worktree_roots.len() == 0 {
        return Ok(None);
    }

    // Prepare the array binding string. SQL doesn't have syntax for this, so
    // we have to do it ourselves.
    let mut array_binding_stmt = "(".to_string();
    for i in 0..worktree_roots.len() {
        // This uses ?NNN for numbered placeholder syntax
        array_binding_stmt.push_str(&format!("?{}", (i + 1))); //sqlite is 1-based
        if i < worktree_roots.len() - 1 {
            array_binding_stmt.push(',');
            array_binding_stmt.push(' ');
        }
    }
    array_binding_stmt.push(')');
    
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
            SELECT workspace_id 
            FROM (SELECT count(workspace_id) as num_matching, workspace_id FROM worktree_roots
                  WHERE worktree_root in {array_bind} AND workspace_id NOT IN
                    (SELECT wt1.workspace_id FROM worktree_roots as wt1
                     JOIN worktree_roots as wt2
                     ON wt1.workspace_id = wt2.workspace_id
                     WHERE wt1.worktree_root NOT in {array_bind} AND wt2.worktree_root in {array_bind})
                  GROUP BY workspace_id)
            WHERE num_matching = ?
        "#,
        array_bind = array_binding_stmt
    );

    // This will only be called on start up and when root workspaces change, no need to waste memory
    // caching it.
    let mut stmt = connection.prepare(&query)?;
    // Make sure we bound the parameters correctly
    debug_assert!(worktree_roots.len() as i32 + 1 == stmt.parameter_count());

    let root_bytes: Vec<&[u8]> = worktree_roots.iter()
        .map(|root| root.as_ref().as_os_str().as_bytes()).collect();
    
    stmt.with_bindings((root_bytes, root_bytes.len()))?
        .maybe_row()
        .map(|row| row.map(|id| WorkspaceId(id)))
}

#[cfg(test)]
mod tests {

    use std::{
        path::{Path, PathBuf},
        sync::Arc,
        thread::sleep,
        time::Duration,
    };

    use crate::Db;

    use super::WorkspaceId;

    #[test]
    fn test_new_worktrees_for_roots() {
        let db = Db::open_in_memory();

        // Test creation in 0 case
        let workspace_1 = db.workspace_for_roots::<String>(&[]);
        assert_eq!(workspace_1.workspace_id, WorkspaceId(1));

        // Test pulling from recent workspaces
        let workspace_1 = db.workspace_for_roots::<String>(&[]);
        assert_eq!(workspace_1.workspace_id, WorkspaceId(1));

        // Ensure the timestamps are different
        sleep(Duration::from_millis(20));
        db.make_new_workspace::<String>(&[]);

        // Test pulling another value from recent workspaces
        let workspace_2 = db.workspace_for_roots::<String>(&[]);
        assert_eq!(workspace_2.workspace_id, WorkspaceId(2));

        // Ensure the timestamps are different
        sleep(Duration::from_millis(20));

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
        let db = Db::open_in_memory();

        assert_eq!(None, db.workspace_id::<String>(&[]));

        db.make_new_workspace::<String>(&[]); //ID 1
        db.make_new_workspace::<String>(&[]); //ID 2
        db.update_worktrees(&WorkspaceId(1), &["/tmp", "/tmp2"]);

        db.write_to("test.db").unwrap();
        // Sanity check
        assert_eq!(db.workspace_id(&["/tmp", "/tmp2"]), Some(WorkspaceId(1)));

        db.update_worktrees::<String>(&WorkspaceId(1), &[]);

        // Make sure 'no worktrees' fails correctly. returning [1, 2] from this
        // call would be semantically correct (as those are the workspaces that
        // don't have roots) but I'd prefer that this API to either return exactly one
        // workspace, and None otherwise
        assert_eq!(db.workspace_id::<String>(&[]), None,);

        assert_eq!(db.last_workspace_id(), Some(WorkspaceId(1)));

        assert_eq!(
            db.recent_workspaces(2),
            vec![(WorkspaceId(1), vec![]), (WorkspaceId(2), vec![]),],
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

        let db = Db::open_in_memory();

        for (workspace_id, entries) in data {
            db.make_new_workspace::<String>(&[]);
            db.update_worktrees(workspace_id, entries);
        }

        assert_eq!(Some(WorkspaceId(1)), db.workspace_id(&["/tmp1"]));
        assert_eq!(db.workspace_id(&["/tmp1", "/tmp2"]), Some(WorkspaceId(2)));
        assert_eq!(
            db.workspace_id(&["/tmp1", "/tmp2", "/tmp3"]),
            Some(WorkspaceId(3))
        );
        assert_eq!(db.workspace_id(&["/tmp2", "/tmp3"]), Some(WorkspaceId(4)));
        assert_eq!(
            db.workspace_id(&["/tmp2", "/tmp3", "/tmp4"]),
            Some(WorkspaceId(5))
        );
        assert_eq!(db.workspace_id(&["/tmp2", "/tmp4"]), Some(WorkspaceId(6)));
        assert_eq!(db.workspace_id(&["/tmp2"]), Some(WorkspaceId(7)));

        assert_eq!(db.workspace_id(&["/tmp1", "/tmp5"]), None);
        assert_eq!(db.workspace_id(&["/tmp5"]), None);
        assert_eq!(db.workspace_id(&["/tmp2", "/tmp3", "/tmp4", "/tmp5"]), None);
    }

    #[test]
    fn test_detect_workspace_id() {
        let data = &[
            (WorkspaceId(1), vec!["/tmp"]),
            (WorkspaceId(2), vec!["/tmp", "/tmp2"]),
            (WorkspaceId(3), vec!["/tmp", "/tmp2", "/tmp3"]),
        ];

        let db = Db::open_in_memory();

        for (workspace_id, entries) in data {
            db.make_new_workspace::<String>(&[]);
            db.update_worktrees(workspace_id, entries);
        }

        assert_eq!(db.workspace_id(&["/tmp2"]), None);
        assert_eq!(db.workspace_id(&["/tmp2", "/tmp3"]), None);
        assert_eq!(db.workspace_id(&["/tmp"]), Some(WorkspaceId(1)));
        assert_eq!(db.workspace_id(&["/tmp", "/tmp2"]), Some(WorkspaceId(2)));
        assert_eq!(
            db.workspace_id(&["/tmp", "/tmp2", "/tmp3"]),
            Some(WorkspaceId(3))
        );
    }

    fn arc_path(path: &'static str) -> Arc<Path> {
        PathBuf::from(path).into()
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

        let db = Db::open_in_memory();

        // Load in the test data
        for (workspace_id, entries) in data {
            db.make_new_workspace::<String>(&[]);
            db.update_worktrees(workspace_id, entries);
        }

        // Execute the update
        db.update_worktrees(&WorkspaceId(2), &["/tmp2", "/tmp3"]);

        // Make sure that workspace 3 doesn't exist
        assert_eq!(db.workspace_id(&["/tmp2", "/tmp3"]), Some(WorkspaceId(2)));

        // And that workspace 1 was untouched
        assert_eq!(db.workspace_id(&["/tmp"]), Some(WorkspaceId(1)));

        // And that workspace 2 is no longer registered under these roots
        assert_eq!(db.workspace_id(&["/tmp", "/tmp2"]), None);

        assert_eq!(Some(WorkspaceId(2)), db.last_workspace_id());

        let recent_workspaces = db.recent_workspaces(10);
        assert_eq!(
            recent_workspaces.get(0).unwrap(),
            &(WorkspaceId(2), vec![arc_path("/tmp2"), arc_path("/tmp3")])
        );
        assert_eq!(
            recent_workspaces.get(1).unwrap(),
            &(WorkspaceId(1), vec![arc_path("/tmp")])
        );
    }
}
