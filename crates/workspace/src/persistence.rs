#![allow(dead_code)]

pub mod model;

use std::path::Path;

use anyhow::{anyhow, bail, Context, Result};
use db::{define_connection, query, sqlez::connection::Connection, sqlez_macros::sql};
use gpui::Axis;

use util::{ unzip_option, ResultExt};

use crate::dock::DockPosition;
use crate::WorkspaceId;

use model::{
    GroupId, PaneId, SerializedItem, SerializedPane, SerializedPaneGroup, SerializedWorkspace,
    WorkspaceLocation,
};

define_connection! {
    pub static ref DB: WorkspaceDb<()> =
        &[sql!(
            CREATE TABLE workspaces(
                workspace_id INTEGER PRIMARY KEY,
                workspace_location BLOB UNIQUE,
                dock_visible INTEGER, // Boolean
                dock_anchor TEXT, // Enum: 'Bottom' / 'Right' / 'Expanded'
                dock_pane INTEGER, // NULL indicates that we don't have a dock pane yet
                left_sidebar_open INTEGER, //Boolean
                timestamp TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL,
                FOREIGN KEY(dock_pane) REFERENCES panes(pane_id)
            ) STRICT;
            
            CREATE TABLE pane_groups(
                group_id INTEGER PRIMARY KEY,
                workspace_id INTEGER NOT NULL,
                parent_group_id INTEGER, // NULL indicates that this is a root node
                position INTEGER, // NULL indicates that this is a root node
                axis TEXT NOT NULL, // Enum: 'Vertical' / 'Horizontal'
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
                ON UPDATE CASCADE,
                FOREIGN KEY(parent_group_id) REFERENCES pane_groups(group_id) ON DELETE CASCADE
            ) STRICT;
            
            CREATE TABLE panes(
                pane_id INTEGER PRIMARY KEY,
                workspace_id INTEGER NOT NULL,
                active INTEGER NOT NULL, // Boolean
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
                ON UPDATE CASCADE
            ) STRICT;
            
            CREATE TABLE center_panes(
                pane_id INTEGER PRIMARY KEY,
                parent_group_id INTEGER, // NULL means that this is a root pane
                position INTEGER, // NULL means that this is a root pane
                FOREIGN KEY(pane_id) REFERENCES panes(pane_id)
                ON DELETE CASCADE,
                FOREIGN KEY(parent_group_id) REFERENCES pane_groups(group_id) ON DELETE CASCADE
            ) STRICT;
            
            CREATE TABLE items(
                item_id INTEGER NOT NULL, // This is the item's view id, so this is not unique
                workspace_id INTEGER NOT NULL,
                pane_id INTEGER NOT NULL,
                kind TEXT NOT NULL,
                position INTEGER NOT NULL,
                active INTEGER NOT NULL,
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
                ON UPDATE CASCADE,
                FOREIGN KEY(pane_id) REFERENCES panes(pane_id)
                ON DELETE CASCADE,
                PRIMARY KEY(item_id, workspace_id)
            ) STRICT;
        )];
}

impl WorkspaceDb {
    /// Returns a serialized workspace for the given worktree_roots. If the passed array
    /// is empty, the most recent workspace is returned instead. If no workspace for the
    /// passed roots is stored, returns none.
    pub fn workspace_for_roots<P: AsRef<Path>>(
        &self,
        worktree_roots: &[P],
    ) -> Option<SerializedWorkspace> {
        let workspace_location: WorkspaceLocation = worktree_roots.into();

        // Note that we re-assign the workspace_id here in case it's empty
        // and we've grabbed the most recent workspace
        let (workspace_id, workspace_location, left_sidebar_open, dock_position): (
            WorkspaceId,
            WorkspaceLocation,
            bool,
            DockPosition,
        ) = 
            self.select_row_bound(sql!{
                SELECT workspace_id, workspace_location, left_sidebar_open, dock_visible, dock_anchor
                FROM workspaces 
                WHERE workspace_location = ?
            })
            .and_then(|mut prepared_statement| (prepared_statement)(&workspace_location))
            .context("No workspaces found")
            .warn_on_err()
            .flatten()?;

        Some(SerializedWorkspace {
            id: workspace_id,
            location: workspace_location.clone(),
            dock_pane: self
                .get_dock_pane(workspace_id)
                .context("Getting dock pane")
                .log_err()?,
            center_group: self
                .get_center_pane_group(workspace_id)
                .context("Getting center group")
                .log_err()?,
            dock_position,
            left_sidebar_open
        })
    }

    /// Saves a workspace using the worktree roots. Will garbage collect any workspaces
    /// that used this workspace previously
    pub async fn save_workspace(&self, workspace: SerializedWorkspace) {
        self.write(move |conn| {
            conn.with_savepoint("update_worktrees", || {
                // Clear out panes and pane_groups
                conn.exec_bound(sql!(
                    UPDATE workspaces SET dock_pane = NULL WHERE workspace_id = ?1;
                    DELETE FROM pane_groups WHERE workspace_id = ?1;
                    DELETE FROM panes WHERE workspace_id = ?1;))?(workspace.id)
                .expect("Clearing old panes");

                conn.exec_bound(sql!(
                    DELETE FROM workspaces WHERE workspace_location = ? AND workspace_id != ?
                ))?((&workspace.location, workspace.id.clone()))
                .context("clearing out old locations")?;

                // Upsert
                conn.exec_bound(sql!(
                        INSERT INTO workspaces(
                            workspace_id,
                            workspace_location,
                            left_sidebar_open,
                            dock_visible,
                            dock_anchor,
                            timestamp
                        )
                        VALUES (?1, ?2, ?3, ?4, ?5, CURRENT_TIMESTAMP)
                        ON CONFLICT DO
                            UPDATE SET
                            workspace_location = ?2,
                            left_sidebar_open = ?3,
                            dock_visible = ?4,
                            dock_anchor = ?5,
                            timestamp = CURRENT_TIMESTAMP
                ))?((workspace.id, &workspace.location, workspace.left_sidebar_open, workspace.dock_position))
                .context("Updating workspace")?;

                // Save center pane group and dock pane
                Self::save_pane_group(conn, workspace.id, &workspace.center_group, None)
                    .context("save pane group in save workspace")?;

                let dock_id = Self::save_pane(conn, workspace.id, &workspace.dock_pane, None, true)
                    .context("save pane in save workspace")?;

                // Complete workspace initialization
                conn.exec_bound(sql!(
                    UPDATE workspaces
                    SET dock_pane = ?
                    WHERE workspace_id = ?
                ))?((dock_id, workspace.id))
                .context("Finishing initialization with dock pane")?;

                Ok(())
            })
            .log_err();
        })
        .await;
    }

    query! {
        pub async fn next_id() -> Result<WorkspaceId> {
            INSERT INTO workspaces DEFAULT VALUES RETURNING workspace_id
        }
    }

    query! {
        pub fn recent_workspaces(limit: usize) -> Result<Vec<(WorkspaceId, WorkspaceLocation)>> {
            SELECT workspace_id, workspace_location 
            FROM workspaces
            WHERE workspace_location IS NOT NULL
            ORDER BY timestamp DESC 
            LIMIT ?
        }
    }

    query! {
        pub fn last_workspace() -> Result<Option<WorkspaceLocation>> {
            SELECT workspace_location
            FROM workspaces
            WHERE workspace_location IS NOT NULL
            ORDER BY timestamp DESC
            LIMIT 1
        }
    }

    fn get_center_pane_group(&self, workspace_id: WorkspaceId) -> Result<SerializedPaneGroup> {
        Ok(self.get_pane_group(workspace_id, None)?
            .into_iter()
            .next()
            .unwrap_or_else(|| SerializedPaneGroup::Pane(SerializedPane { active: true, children: vec![] })))
    }

    fn get_pane_group(
        &self,
        workspace_id: WorkspaceId,
        group_id: Option<GroupId>,
    ) -> Result<Vec<SerializedPaneGroup>> {
        type GroupKey = (Option<GroupId>, WorkspaceId);
        type GroupOrPane = (Option<GroupId>, Option<Axis>, Option<PaneId>, Option<bool>);
        self.select_bound::<GroupKey, GroupOrPane>(sql!(
            SELECT group_id, axis, pane_id, active
                FROM (SELECT 
                        group_id,
                        axis,
                        NULL as pane_id,
                        NULL as active,
                        position,
                        parent_group_id,
                        workspace_id
                      FROM pane_groups 
                     UNION
                      SELECT 
                        NULL,
                        NULL,  
                        center_panes.pane_id,
                        panes.active as active,
                        position,
                        parent_group_id,
                        panes.workspace_id as workspace_id
                      FROM center_panes
                      JOIN panes ON center_panes.pane_id = panes.pane_id) 
            WHERE parent_group_id IS ? AND workspace_id = ?
            ORDER BY position
        ))?((group_id, workspace_id))?
        .into_iter()
        .map(|(group_id, axis, pane_id, active)| {
            if let Some((group_id, axis)) = group_id.zip(axis) {
                Ok(SerializedPaneGroup::Group {
                    axis,
                    children: self.get_pane_group(workspace_id, Some(group_id))?,
                })
            } else if let Some((pane_id, active)) = pane_id.zip(active) {
                Ok(SerializedPaneGroup::Pane(SerializedPane::new(
                    self.get_items(pane_id)?,
                    active,
                )))
            } else {
                bail!("Pane Group Child was neither a pane group or a pane");
            }
        })
        // Filter out panes and pane groups which don't have any children or items
        .filter(|pane_group| match pane_group {
            Ok(SerializedPaneGroup::Group { children, .. }) => !children.is_empty(),
            Ok(SerializedPaneGroup::Pane(pane)) => !pane.children.is_empty(), 
            _ => true,
        })
        .collect::<Result<_>>()
    }

   
    fn save_pane_group(
        conn: &Connection,
        workspace_id: WorkspaceId,
        pane_group: &SerializedPaneGroup,
        parent: Option<(GroupId, usize)>,
    ) -> Result<()> {
        match pane_group {
            SerializedPaneGroup::Group { axis, children } => {
                let (parent_id, position) = unzip_option(parent);

                let group_id = conn.select_row_bound::<_, i64>(sql!(
                        INSERT INTO pane_groups(workspace_id, parent_group_id, position, axis) 
                        VALUES (?, ?, ?, ?) 
                        RETURNING group_id
                ))?((
                    workspace_id,
                    parent_id,
                    position,
                    *axis,
                ))?
                .ok_or_else(|| anyhow!("Couldn't retrieve group_id from inserted pane_group"))?;

                for (position, group) in children.iter().enumerate() {
                    Self::save_pane_group(conn, workspace_id, group, Some((group_id, position)))?
                }

                Ok(())
            }
            SerializedPaneGroup::Pane(pane) => {
                Self::save_pane(conn, workspace_id, &pane, parent, false)?;
                Ok(())
            }
        }
    }

    fn get_dock_pane(&self, workspace_id: WorkspaceId) -> Result<SerializedPane> {
        let (pane_id, active) = self.select_row_bound(sql!(
            SELECT pane_id, active
            FROM panes
            WHERE pane_id = (SELECT dock_pane FROM workspaces WHERE workspace_id = ?)
        ))?(
            workspace_id,
        )?
        .context("No dock pane for workspace")?;

        Ok(SerializedPane::new(
            self.get_items(pane_id).context("Reading items")?,
            active,
        ))
    }

    fn save_pane(
        conn: &Connection,
        workspace_id: WorkspaceId,
        pane: &SerializedPane,
        parent: Option<(GroupId, usize)>, // None indicates BOTH dock pane AND center_pane
        dock: bool,
    ) -> Result<PaneId> {
        let pane_id = conn.select_row_bound::<_, i64>(sql!(
            INSERT INTO panes(workspace_id, active) 
            VALUES (?, ?) 
            RETURNING pane_id
        ))?((workspace_id, pane.active))?
        .ok_or_else(|| anyhow!("Could not retrieve inserted pane_id"))?;

        if !dock {
            let (parent_id, order) = unzip_option(parent);
            conn.exec_bound(sql!(
                INSERT INTO center_panes(pane_id, parent_group_id, position)
                VALUES (?, ?, ?)
            ))?((pane_id, parent_id, order))?;
        }

        Self::save_items(conn, workspace_id, pane_id, &pane.children).context("Saving items")?;

        Ok(pane_id)
    }

    fn get_items(&self, pane_id: PaneId) -> Result<Vec<SerializedItem>> {
        Ok(self.select_bound(sql!(
            SELECT kind, item_id, active FROM items
            WHERE pane_id = ?
            ORDER BY position
        ))?(pane_id)?)
    }

    fn save_items(
        conn: &Connection,
        workspace_id: WorkspaceId,
        pane_id: PaneId,
        items: &[SerializedItem],
    ) -> Result<()> {
        let mut insert = conn.exec_bound(sql!(
            INSERT INTO items(workspace_id, pane_id, position, kind, item_id, active) VALUES (?, ?, ?, ?, ?, ?)
        )).context("Preparing insertion")?;
        for (position, item) in items.iter().enumerate() {
            insert((workspace_id, pane_id, position, item))?;
        }

        Ok(())
    }

    query!{
        pub async fn update_timestamp(workspace_id: WorkspaceId) -> Result<()> {
            UPDATE workspaces
            SET timestamp = CURRENT_TIMESTAMP
            WHERE workspace_id = ?
        }
    }
    
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use db::open_test_db;
    use settings::DockAnchor;

    use super::*;

    #[gpui::test]
    async fn test_next_id_stability() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_test_db("test_next_id_stability").await);

        db.write(|conn| {
            conn.migrate(
                "test_table",
                &[sql!(
                    CREATE TABLE test_table(
                        text TEXT,
                        workspace_id INTEGER,
                        FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                            ON DELETE CASCADE
                    ) STRICT;
                )],
            )
            .unwrap();
        })
        .await;

        let id = db.next_id().await.unwrap();
        // Assert the empty row got inserted
        assert_eq!(
            Some(id),
            db.select_row_bound::<WorkspaceId, WorkspaceId>(sql!(
                SELECT workspace_id FROM workspaces WHERE workspace_id = ?
            ))
            .unwrap()(id)
            .unwrap()
        );

        db.write(move |conn| {
            conn.exec_bound(sql!(INSERT INTO test_table(text, workspace_id) VALUES (?, ?)))
                .unwrap()(("test-text-1", id))
            .unwrap()
        })
        .await;

        let test_text_1 = db
            .select_row_bound::<_, String>(sql!(SELECT text FROM test_table WHERE workspace_id = ?))
            .unwrap()(1)
        .unwrap()
        .unwrap();
        assert_eq!(test_text_1, "test-text-1");
    }

    #[gpui::test]
    async fn test_workspace_id_stability() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_test_db("test_workspace_id_stability").await);

        db.write(|conn| {
            conn.migrate(
                "test_table",
                &[sql!(
                    CREATE TABLE test_table(
                        text TEXT,
                        workspace_id INTEGER,
                        FOREIGN KEY(workspace_id) 
                            REFERENCES workspaces(workspace_id)
                            ON DELETE CASCADE
                    ) STRICT;)],
            )
        })
        .await
        .unwrap();

        let mut workspace_1 = SerializedWorkspace {
            id: 1,
            location: (["/tmp", "/tmp2"]).into(),
            dock_position: crate::dock::DockPosition::Shown(DockAnchor::Bottom),
            center_group: Default::default(),
            dock_pane: Default::default(),
            left_sidebar_open: true
        };

        let mut workspace_2 = SerializedWorkspace {
            id: 2,
            location: (["/tmp"]).into(),
            dock_position: crate::dock::DockPosition::Hidden(DockAnchor::Expanded),
            center_group: Default::default(),
            dock_pane: Default::default(),
            left_sidebar_open: false
        };

        db.save_workspace(workspace_1.clone()).await;

        db.write(|conn| {
            conn.exec_bound(sql!(INSERT INTO test_table(text, workspace_id) VALUES (?, ?)))
                .unwrap()(("test-text-1", 1))
            .unwrap();
        })
        .await;

        db.save_workspace(workspace_2.clone()).await;

        db.write(|conn| {
            conn.exec_bound(sql!(INSERT INTO test_table(text, workspace_id) VALUES (?, ?)))
                .unwrap()(("test-text-2", 2))
            .unwrap();
        })
        .await;

        workspace_1.location = (["/tmp", "/tmp3"]).into();
        db.save_workspace(workspace_1.clone()).await;
        db.save_workspace(workspace_1).await;

        workspace_2.dock_pane.children.push(SerializedItem {
            kind: Arc::from("Test"),
            item_id: 10,
            active: true,
        });
        db.save_workspace(workspace_2).await;

        let test_text_2 = db
            .select_row_bound::<_, String>(sql!(SELECT text FROM test_table WHERE workspace_id = ?))
            .unwrap()(2)
        .unwrap()
        .unwrap();
        assert_eq!(test_text_2, "test-text-2");

        let test_text_1 = db
            .select_row_bound::<_, String>(sql!(SELECT text FROM test_table WHERE workspace_id = ?))
            .unwrap()(1)
        .unwrap()
        .unwrap();
        assert_eq!(test_text_1, "test-text-1");
    }

    #[gpui::test]
    async fn test_full_workspace_serialization() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_test_db("test_full_workspace_serialization").await);

        let dock_pane = crate::persistence::model::SerializedPane {
            children: vec![
                SerializedItem::new("Terminal", 1, false),
                SerializedItem::new("Terminal", 2, false),
                SerializedItem::new("Terminal", 3, true),
                SerializedItem::new("Terminal", 4, false),
            ],
            active: false,
        };

        //  -----------------
        //  | 1,2   | 5,6   |
        //  | - - - |       |
        //  | 3,4   |       |
        //  -----------------
        let center_group = SerializedPaneGroup::Group {
            axis: gpui::Axis::Horizontal,
            children: vec![
                SerializedPaneGroup::Group {
                    axis: gpui::Axis::Vertical,
                    children: vec![
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 5, false),
                                SerializedItem::new("Terminal", 6, true),
                            ],
                            false,
                        )),
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 7, true),
                                SerializedItem::new("Terminal", 8, false),
                            ],
                            false,
                        )),
                    ],
                },
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 9, false),
                        SerializedItem::new("Terminal", 10, true),
                    ],
                    false,
                )),
            ],
        };

        let workspace = SerializedWorkspace {
            id: 5,
            location: (["/tmp", "/tmp2"]).into(),
            dock_position: DockPosition::Shown(DockAnchor::Bottom),
            center_group,
            dock_pane,
            left_sidebar_open: true
        };

        db.save_workspace(workspace.clone()).await;
        let round_trip_workspace = db.workspace_for_roots(&["/tmp2", "/tmp"]);

        assert_eq!(workspace, round_trip_workspace.unwrap());

        // Test guaranteed duplicate IDs
        db.save_workspace(workspace.clone()).await;
        db.save_workspace(workspace.clone()).await;

        let round_trip_workspace = db.workspace_for_roots(&["/tmp", "/tmp2"]);
        assert_eq!(workspace, round_trip_workspace.unwrap());
    }

    #[gpui::test]
    async fn test_workspace_assignment() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_test_db("test_basic_functionality").await);

        let workspace_1 = SerializedWorkspace {
            id: 1,
            location: (["/tmp", "/tmp2"]).into(),
            dock_position: crate::dock::DockPosition::Shown(DockAnchor::Bottom),
            center_group: Default::default(),
            dock_pane: Default::default(),
            left_sidebar_open: true,
        };

        let mut workspace_2 = SerializedWorkspace {
            id: 2,
            location: (["/tmp"]).into(),
            dock_position: crate::dock::DockPosition::Hidden(DockAnchor::Expanded),
            center_group: Default::default(),
            dock_pane: Default::default(),
            left_sidebar_open: false,
        };

        db.save_workspace(workspace_1.clone()).await;
        db.save_workspace(workspace_2.clone()).await;

        // Test that paths are treated as a set
        assert_eq!(
            db.workspace_for_roots(&["/tmp", "/tmp2"]).unwrap(),
            workspace_1
        );
        assert_eq!(
            db.workspace_for_roots(&["/tmp2", "/tmp"]).unwrap(),
            workspace_1
        );

        // Make sure that other keys work
        assert_eq!(db.workspace_for_roots(&["/tmp"]).unwrap(), workspace_2);
        assert_eq!(db.workspace_for_roots(&["/tmp3", "/tmp2", "/tmp4"]), None);

        // Test 'mutate' case of updating a pre-existing id
        workspace_2.location = (["/tmp", "/tmp2"]).into();

        db.save_workspace(workspace_2.clone()).await;
        assert_eq!(
            db.workspace_for_roots(&["/tmp", "/tmp2"]).unwrap(),
            workspace_2
        );

        // Test other mechanism for mutating
        let mut workspace_3 = SerializedWorkspace {
            id: 3,
            location: (&["/tmp", "/tmp2"]).into(),
            dock_position: DockPosition::Shown(DockAnchor::Right),
            center_group: Default::default(),
            dock_pane: Default::default(),
            left_sidebar_open: false
        };

        db.save_workspace(workspace_3.clone()).await;
        assert_eq!(
            db.workspace_for_roots(&["/tmp", "/tmp2"]).unwrap(),
            workspace_3
        );

        // Make sure that updating paths differently also works
        workspace_3.location = (["/tmp3", "/tmp4", "/tmp2"]).into();
        db.save_workspace(workspace_3.clone()).await;
        assert_eq!(db.workspace_for_roots(&["/tmp2", "tmp"]), None);
        assert_eq!(
            db.workspace_for_roots(&["/tmp2", "/tmp3", "/tmp4"])
                .unwrap(),
            workspace_3
        );
    }

    use crate::dock::DockPosition;
    use crate::persistence::model::SerializedWorkspace;
    use crate::persistence::model::{SerializedItem, SerializedPane, SerializedPaneGroup};

    fn default_workspace<P: AsRef<Path>>(
        workspace_id: &[P],
        dock_pane: SerializedPane,
        center_group: &SerializedPaneGroup,
    ) -> SerializedWorkspace {
        SerializedWorkspace {
            id: 4,
            location: workspace_id.into(),
            dock_position: crate::dock::DockPosition::Hidden(DockAnchor::Right),
            center_group: center_group.clone(),
            dock_pane,
            left_sidebar_open: true
        }
    }

    #[gpui::test]
    async fn test_basic_dock_pane() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_test_db("basic_dock_pane").await);

        let dock_pane = crate::persistence::model::SerializedPane::new(
            vec![
                SerializedItem::new("Terminal", 1, false),
                SerializedItem::new("Terminal", 4, false),
                SerializedItem::new("Terminal", 2, false),
                SerializedItem::new("Terminal", 3, true),
            ],
            false,
        );

        let workspace = default_workspace(&["/tmp"], dock_pane, &Default::default());

        db.save_workspace(workspace.clone()).await;

        let new_workspace = db.workspace_for_roots(&["/tmp"]).unwrap();

        assert_eq!(workspace.dock_pane, new_workspace.dock_pane);
    }

    #[gpui::test]
    async fn test_simple_split() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_test_db("simple_split").await);

        //  -----------------
        //  | 1,2   | 5,6   |
        //  | - - - |       |
        //  | 3,4   |       |
        //  -----------------
        let center_pane = SerializedPaneGroup::Group {
            axis: gpui::Axis::Horizontal,
            children: vec![
                SerializedPaneGroup::Group {
                    axis: gpui::Axis::Vertical,
                    children: vec![
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 1, false),
                                SerializedItem::new("Terminal", 2, true),
                            ],
                            false,
                        )),
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 4, false),
                                SerializedItem::new("Terminal", 3, true),
                            ],
                            true,
                        )),
                    ],
                },
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 5, true),
                        SerializedItem::new("Terminal", 6, false),
                    ],
                    false,
                )),
            ],
        };

        let workspace = default_workspace(&["/tmp"], Default::default(), &center_pane);

        db.save_workspace(workspace.clone()).await;

        let new_workspace = db.workspace_for_roots(&["/tmp"]).unwrap();

        assert_eq!(workspace.center_group, new_workspace.center_group);
    }

    #[gpui::test]
    async fn test_cleanup_panes() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_test_db("test_cleanup_panes").await);

        let center_pane = SerializedPaneGroup::Group {
            axis: gpui::Axis::Horizontal,
            children: vec![
                SerializedPaneGroup::Group {
                    axis: gpui::Axis::Vertical,
                    children: vec![
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 1, false),
                                SerializedItem::new("Terminal", 2, true),
                            ],
                            false,
                        )),
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 4, false),
                                SerializedItem::new("Terminal", 3, true),
                            ],
                            true,
                        )),
                    ],
                },
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 5, false),
                        SerializedItem::new("Terminal", 6, true),
                    ],
                    false,
                )),
            ],
        };

        let id = &["/tmp"];

        let mut workspace = default_workspace(id, Default::default(), &center_pane);

        db.save_workspace(workspace.clone()).await;

        workspace.center_group = SerializedPaneGroup::Group {
            axis: gpui::Axis::Vertical,
            children: vec![
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 1, false),
                        SerializedItem::new("Terminal", 2, true),
                    ],
                    false,
                )),
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 4, true),
                        SerializedItem::new("Terminal", 3, false),
                    ],
                    true,
                )),
            ],
        };

        db.save_workspace(workspace.clone()).await;

        let new_workspace = db.workspace_for_roots(id).unwrap();

        assert_eq!(workspace.center_group, new_workspace.center_group);
    }
}
