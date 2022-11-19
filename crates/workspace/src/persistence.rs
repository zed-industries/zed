#![allow(dead_code)]

pub mod model;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, bail, Result, Context};
use db::connection;
use gpui::Axis;
use indoc::indoc;


use db::sqlez::domain::Domain;
use util::{iife, unzip_option, ResultExt};

use crate::dock::DockPosition;

use super::Workspace;

use model::{
    GroupId, PaneId, SerializedItem, SerializedPane, SerializedPaneGroup,
    SerializedWorkspace, WorkspaceId,
};

connection!(DB: WorkspaceDb<Workspace>);


impl Domain for Workspace {
    fn name() -> &'static str {
        "workspace"
    }
    
    fn migrations() -> &'static [&'static str] {
        &[indoc! {"
            CREATE TABLE workspaces(
                workspace_id BLOB PRIMARY KEY,
                dock_visible INTEGER, -- Boolean
                dock_anchor TEXT, -- Enum: 'Bottom' / 'Right' / 'Expanded'
                dock_pane INTEGER, -- NULL indicates that we don't have a dock pane yet
                timestamp TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL,
                FOREIGN KEY(dock_pane) REFERENCES panes(pane_id)
            ) STRICT;
            
            CREATE TABLE pane_groups(
                group_id INTEGER PRIMARY KEY,
                workspace_id BLOB NOT NULL,
                parent_group_id INTEGER, -- NULL indicates that this is a root node
                position INTEGER, -- NULL indicates that this is a root node
                axis TEXT NOT NULL, -- Enum:  'Vertical' / 'Horizontal'
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) 
                    ON DELETE CASCADE 
                    ON UPDATE CASCADE,
                FOREIGN KEY(parent_group_id) REFERENCES pane_groups(group_id) ON DELETE CASCADE
            ) STRICT;
            
            CREATE TABLE panes(
                pane_id INTEGER PRIMARY KEY,
                workspace_id BLOB NOT NULL,
                active INTEGER NOT NULL, -- Boolean
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) 
                    ON DELETE CASCADE 
                    ON UPDATE CASCADE
            ) STRICT;
            
            CREATE TABLE center_panes(
                pane_id INTEGER PRIMARY KEY,
                parent_group_id INTEGER, -- NULL means that this is a root pane
                position INTEGER, -- NULL means that this is a root pane
                FOREIGN KEY(pane_id) REFERENCES panes(pane_id) 
                    ON DELETE CASCADE,
                FOREIGN KEY(parent_group_id) REFERENCES pane_groups(group_id) ON DELETE CASCADE
            ) STRICT;
                            
            CREATE TABLE items(
                item_id INTEGER NOT NULL, -- This is the item's view id, so this is not unique
                workspace_id BLOB NOT NULL,
                pane_id INTEGER NOT NULL,
                kind TEXT NOT NULL,
                position INTEGER NOT NULL,
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                    ON UPDATE CASCADE,
                FOREIGN KEY(pane_id) REFERENCES panes(pane_id)
                    ON DELETE CASCADE,
                PRIMARY KEY(item_id, workspace_id)
            ) STRICT;
        "}]
    }
}

impl WorkspaceDb {
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
        let (workspace_id, dock_position): (WorkspaceId, DockPosition) = iife!({
            if worktree_roots.len() == 0 {
                self.select_row(indoc! {"
                    SELECT workspace_id, dock_visible, dock_anchor
                    FROM workspaces 
                    ORDER BY timestamp DESC LIMIT 1"})?()?
            } else {
                self.select_row_bound(indoc! {"
                    SELECT workspace_id, dock_visible, dock_anchor
                    FROM workspaces 
                    WHERE workspace_id = ?"})?(&workspace_id)?
            }
            .context("No workspaces found")
        })
        .warn_on_err()
        .flatten()?;

        Some(SerializedWorkspace {
            workspace_id: workspace_id.clone(),
            dock_pane: self
                .get_dock_pane(&workspace_id)
                .context("Getting dock pane")
                .log_err()?,
            center_group: self
                .get_center_pane_group(&workspace_id)
                .context("Getting center group")
                .log_err()?,
            dock_position,
        })
    }

    /// Saves a workspace using the worktree roots. Will garbage collect any workspaces
    /// that used this workspace previously
    pub fn save_workspace(
        &self,
        old_id: Option<WorkspaceId>,
        workspace: &SerializedWorkspace,
    ) {
        self.with_savepoint("update_worktrees", || {
            self.exec_bound(indoc! {"
                UPDATE workspaces SET dock_pane = NULL WHERE workspace_id = ?1;
                DELETE FROM pane_groups WHERE workspace_id = ?1;
                DELETE FROM panes WHERE workspace_id = ?1;"})?
            (old_id.as_ref().unwrap_or(&workspace.workspace_id)).context("Clearing old panes")?;
            
            if let Some(old_id) = old_id {
                self.exec_bound(indoc! {"
                    UPDATE OR REPLACE workspaces
                    SET workspace_id = ?,
                        dock_visible = ?,
                        dock_anchor = ?,
                        timestamp = CURRENT_TIMESTAMP
                    WHERE workspace_id = ?"})?((
                    &workspace.workspace_id,
                    workspace.dock_position,
                    &old_id,
                )).context("Updating workspace with new worktree roots")?;
            } else {
                self.exec_bound(
                    "INSERT OR REPLACE INTO workspaces(workspace_id, dock_visible, dock_anchor) VALUES (?, ?, ?)",
                )?((&workspace.workspace_id, workspace.dock_position)).context("Uodating workspace")?;
            }
            
            // Save center pane group and dock pane
            self.save_pane_group(&workspace.workspace_id, &workspace.center_group, None).context("save pane group in save workspace")?;
            
            let dock_id = self.save_pane(&workspace.workspace_id, &workspace.dock_pane, None, true).context("save pane in save workspace")?;
        
            // Complete workspace initialization
            self.exec_bound(indoc! {"
                UPDATE workspaces
                SET dock_pane = ?
                WHERE workspace_id = ?"})?((
                dock_id,
                &workspace.workspace_id,
            )).context("Finishing initialization with dock pane")?;

            Ok(())
        })
        .with_context(|| {
            format!(
                "Update workspace with roots {:?} failed.",
                workspace.workspace_id.paths()
            )
        })
        .log_err();
    }

    /// Returns the previous workspace ids sorted by last modified along with their opened worktree roots
    pub fn recent_workspaces(&self, limit: usize) -> Vec<Arc<Vec<PathBuf>>> {
        iife!({
            // TODO, upgrade anyhow: https://docs.rs/anyhow/1.0.66/anyhow/fn.Ok.html
            Ok::<_, anyhow::Error>(
                self.select_bound::<usize, WorkspaceId>(
                    "SELECT workspace_id FROM workspaces ORDER BY timestamp DESC LIMIT ?",
                )?(limit)?
                .into_iter()
                .map(|id| id.paths())
                .collect::<Vec<Arc<Vec<PathBuf>>>>(),
            )
        })
        .log_err()
        .unwrap_or_default()
    }

    pub(crate) fn get_center_pane_group(
        &self,
        workspace_id: &WorkspaceId,
    ) -> Result<SerializedPaneGroup> {
        self.get_pane_group(workspace_id, None)?
            .into_iter()
            .next()
            .context("No center pane group")
    }

    fn get_pane_group(
        &self,
        workspace_id: &WorkspaceId,
        group_id: Option<GroupId>,
    ) -> Result<Vec<SerializedPaneGroup>> {
        type GroupKey<'a> = (Option<GroupId>, &'a WorkspaceId);
        type GroupOrPane = (Option<GroupId>, Option<Axis>, Option<PaneId>, Option<bool>);
        self.select_bound::<GroupKey, GroupOrPane>(indoc! {"
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
            "})?((group_id, workspace_id))?
        .into_iter()
        .map(|(group_id, axis, pane_id, active)| {
            if let Some((group_id, axis)) = group_id.zip(axis) {
                Ok(SerializedPaneGroup::Group {
                    axis,
                    children: self.get_pane_group(
                        workspace_id,
                        Some(group_id),
                    )?,
                })
            } else if let Some((pane_id, active)) = pane_id.zip(active) {
                Ok(SerializedPaneGroup::Pane(SerializedPane::new(self.get_items( pane_id)?, active)))
            } else {
                bail!("Pane Group Child was neither a pane group or a pane");
            }
        })
        // Filter out panes and pane groups which don't have any children or items
        .filter(|pane_group| {
            match pane_group {
                Ok(SerializedPaneGroup::Group { children, .. }) => !children.is_empty(),
                Ok(SerializedPaneGroup::Pane(pane)) => !pane.children.is_empty(),
                _ => true,
            }
        })
        .collect::<Result<_>>()
    }

    pub(crate) fn save_pane_group(
        &self,
        workspace_id: &WorkspaceId,
        pane_group: &SerializedPaneGroup,
        parent: Option<(GroupId, usize)>,
    ) -> Result<()> {
        match pane_group {
            SerializedPaneGroup::Group { axis, children } => {
                let (parent_id, position) = unzip_option(parent);

                let group_id = self.select_row_bound::<_, i64>(indoc!{"
                        INSERT INTO pane_groups(workspace_id, parent_group_id, position, axis) 
                        VALUES (?, ?, ?, ?) 
                        RETURNING group_id"})?
                    ((workspace_id, parent_id, position, *axis))?
                    .ok_or_else(|| anyhow!("Couldn't retrieve group_id from inserted pane_group"))?;
                
                for (position, group) in children.iter().enumerate() {
                    self.save_pane_group(workspace_id, group, Some((group_id, position)))?
                }
                Ok(())
            }
            SerializedPaneGroup::Pane(pane) => {
                self.save_pane(workspace_id, &pane, parent, false)?;
                Ok(())
            },
        }
    }

    pub(crate) fn get_dock_pane(&self, workspace_id: &WorkspaceId) -> Result<SerializedPane> {
        let (pane_id, active) = self.select_row_bound(indoc! {"
            SELECT pane_id, active
            FROM panes
            WHERE pane_id = (SELECT dock_pane FROM workspaces WHERE workspace_id = ?)"})?(
            workspace_id,
        )?
        .context("No dock pane for workspace")?;

        Ok(SerializedPane::new(
            self.get_items(pane_id).context("Reading items")?,
            active
        ))
    }

    pub(crate) fn save_pane(
        &self,
        workspace_id: &WorkspaceId,
        pane: &SerializedPane,
        parent: Option<(GroupId, usize)>, // None indicates BOTH dock pane AND center_pane
        dock: bool,
    ) -> Result<PaneId> {
        let pane_id = self.select_row_bound::<_, i64>(indoc!{"
            INSERT INTO panes(workspace_id, active) 
            VALUES (?, ?) 
            RETURNING pane_id"},
        )?((workspace_id, pane.active))?
        .ok_or_else(|| anyhow!("Could not retrieve inserted pane_id"))?;
        
        if !dock {
            let (parent_id, order) = unzip_option(parent);
            self.exec_bound(indoc! {"
                INSERT INTO center_panes(pane_id, parent_group_id, position)
                VALUES (?, ?, ?)"})?((
                    pane_id, parent_id, order
                ))?;
        }

        self.save_items(workspace_id, pane_id, &pane.children)
            .context("Saving items")?;
        
        Ok(pane_id)
    }
    
  

    pub(crate) fn get_items(&self, pane_id: PaneId) -> Result<Vec<SerializedItem>> {
        Ok(self.select_bound(indoc! {"
            SELECT kind, item_id FROM items
            WHERE pane_id = ?
            ORDER BY position"})?(pane_id)?)
    }

    pub(crate) fn save_items(
        &self,
        workspace_id: &WorkspaceId,
        pane_id: PaneId,
        items: &[SerializedItem],
    ) -> Result<()> {
        let mut insert = self.exec_bound(
            "INSERT INTO items(workspace_id, pane_id, position, kind, item_id) VALUES (?, ?, ?, ?, ?)",
        ).context("Preparing insertion")?;
        for (position, item) in items.iter().enumerate() {
            insert((workspace_id, pane_id, position, item))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use db::{open_memory_db};
    use settings::DockAnchor;

    use super::*;

    #[test]
    fn test_full_workspace_serialization() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_memory_db(Some("test_full_workspace_serialization")));

        let dock_pane = crate::persistence::model::SerializedPane {
            
            children: vec![
                SerializedItem::new("Terminal", 1),
                SerializedItem::new("Terminal", 2),
                SerializedItem::new("Terminal", 3),
                SerializedItem::new("Terminal", 4),

            ],
            active: false
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
                                SerializedItem::new("Terminal", 5),
                                SerializedItem::new("Terminal", 6),
                            ],
                            false)
                        ),
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 7),
                                SerializedItem::new("Terminal", 8),
                            ],
                            false,
                        )),
                    ],
                },
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 9),
                        SerializedItem::new("Terminal", 10),

                    ],
                    false,
                )),
            ],
        };

        let workspace = SerializedWorkspace {
            workspace_id: (["/tmp", "/tmp2"]).into(),
            dock_position:  DockPosition::Shown(DockAnchor::Bottom),
            center_group,
            dock_pane,
        };
        
        db.save_workspace(None, &workspace);
        let round_trip_workspace = db.workspace_for_roots(&["/tmp2", "/tmp"]);
        
        assert_eq!(workspace, round_trip_workspace.unwrap());

        // Test guaranteed duplicate IDs
        db.save_workspace(None, &workspace);
        db.save_workspace(None, &workspace);
        
        let round_trip_workspace = db.workspace_for_roots(&["/tmp", "/tmp2"]);
        assert_eq!(workspace, round_trip_workspace.unwrap());
        
        
    }

    #[test]
    fn test_workspace_assignment() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_memory_db(Some("test_basic_functionality")));

        let workspace_1 = SerializedWorkspace {
            workspace_id: (["/tmp", "/tmp2"]).into(),
            dock_position: crate::dock::DockPosition::Shown(DockAnchor::Bottom),
            center_group: Default::default(),
            dock_pane: Default::default(),
        };

        let mut workspace_2 = SerializedWorkspace {
            workspace_id: (["/tmp"]).into(),
            dock_position: crate::dock::DockPosition::Hidden(DockAnchor::Expanded),
            center_group: Default::default(),
            dock_pane: Default::default(),
        };

        db.save_workspace(None, &workspace_1);
        db.save_workspace(None, &workspace_2);

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
        workspace_2.workspace_id = (["/tmp", "/tmp2"]).into();
        db.save_workspace(Some((&["/tmp"]).into()), &workspace_2);
        assert_eq!(
            db.workspace_for_roots(&["/tmp", "/tmp2"]).unwrap(),
            workspace_2
        );

        // Test other mechanism for mutating
        let mut workspace_3 = SerializedWorkspace {
            workspace_id: (&["/tmp", "/tmp2"]).into(),
            dock_position: DockPosition::Shown(DockAnchor::Right),
            center_group: Default::default(),
            dock_pane: Default::default(),
        };

        
        db.save_workspace(None, &workspace_3);
        assert_eq!(
            db.workspace_for_roots(&["/tmp", "/tmp2"]).unwrap(),
            workspace_3
        );

        // Make sure that updating paths differently also works
        workspace_3.workspace_id = (["/tmp3", "/tmp4", "/tmp2"]).into();
        db.save_workspace(
            Some((&["/tmp", "/tmp2"]).into()),
            &workspace_3,
        );
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
            workspace_id: workspace_id.into(),
            dock_position: crate::dock::DockPosition::Hidden(DockAnchor::Right),
            center_group: center_group.clone(),
            dock_pane,
        }
    }

    #[test]
    fn test_basic_dock_pane() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_memory_db(Some("basic_dock_pane")));

        let dock_pane = crate::persistence::model::SerializedPane::new(
            vec![
                SerializedItem::new("Terminal", 1),
                SerializedItem::new("Terminal", 4),
                SerializedItem::new("Terminal", 2),
                SerializedItem::new("Terminal", 3),
            ], false
        );

        let workspace = default_workspace(&["/tmp"], dock_pane, &Default::default());

        db.save_workspace(None, &workspace);

        let new_workspace = db.workspace_for_roots(&["/tmp"]).unwrap();

        assert_eq!(workspace.dock_pane, new_workspace.dock_pane);
    }

    #[test]
    fn test_simple_split() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_memory_db(Some("simple_split")));

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
                            SerializedItem::new("Terminal", 1),
                            SerializedItem::new("Terminal", 2),
                        ],
                        false)),
                    SerializedPaneGroup::Pane(SerializedPane::new(vec![
                            SerializedItem::new("Terminal", 4),
                            SerializedItem::new("Terminal", 3),
                        ], true)),  
                    ],
                },
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 5),
                        SerializedItem::new("Terminal", 6),
                    ],
                    false)),
            ],
        };

        let workspace = default_workspace(&["/tmp"], Default::default(), &center_pane);

        db.save_workspace(None, &workspace);
                
        let new_workspace = db.workspace_for_roots(&["/tmp"]).unwrap();

        assert_eq!(workspace.center_group, new_workspace.center_group);
    }
    
    #[test]
    fn test_cleanup_panes() {
        env_logger::try_init().ok();
        
        let db = WorkspaceDb(open_memory_db(Some("test_cleanup_panes")));
        
        let center_pane = SerializedPaneGroup::Group {
            axis: gpui::Axis::Horizontal,
            children: vec![
                SerializedPaneGroup::Group {
                    axis: gpui::Axis::Vertical,
                    children: vec![
                    SerializedPaneGroup::Pane(SerializedPane::new( 
                        vec![
                            SerializedItem::new("Terminal", 1),
                            SerializedItem::new("Terminal", 2),
                        ],
                        false)),
                    SerializedPaneGroup::Pane(SerializedPane::new(vec![
                            SerializedItem::new("Terminal", 4),
                            SerializedItem::new("Terminal", 3),
                        ], true)),  
                    ],
                },
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 5),
                        SerializedItem::new("Terminal", 6),
                    ],
                    false)),
            ],
        };

        let id = &["/tmp"];
        
        let mut workspace = default_workspace(id, Default::default(), &center_pane);

        db.save_workspace(None, &workspace);
        
        workspace.center_group = SerializedPaneGroup::Group {
            axis: gpui::Axis::Vertical,
            children: vec![
            SerializedPaneGroup::Pane(SerializedPane::new( 
                vec![
                    SerializedItem::new("Terminal", 1),
                    SerializedItem::new("Terminal", 2),
                ],
                false)),
            SerializedPaneGroup::Pane(SerializedPane::new(vec![
                    SerializedItem::new("Terminal", 4),
                    SerializedItem::new("Terminal", 3),
                ], true)),  
            ],
        };
        
        db.save_workspace(None, &workspace);
                
        let new_workspace = db.workspace_for_roots(id).unwrap();

        assert_eq!(workspace.center_group, new_workspace.center_group);

    }
}
