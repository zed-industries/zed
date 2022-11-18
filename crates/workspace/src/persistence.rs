#![allow(dead_code)]

pub mod model;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, bail, Result, Context};
use db::connection;
use gpui::Axis;
use indoc::indoc;
use lazy_static::lazy_static;


use sqlez::domain::Domain;
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
                timestamp TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL
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
                parent_group_id INTEGER, -- NULL means that this is a dock pane
                position INTEGER, -- NULL means that this is a dock pane
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) 
                    ON DELETE CASCADE 
                    ON UPDATE CASCADE,
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
            if let Some(old_id) = old_id {
                self.exec_bound(indoc! {"
                    DELETE FROM pane_groups WHERE workspace_id = ?"})?(&old_id)?;
                
                // If collision, delete
                
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
                ))?;
            } else {
                self.exec_bound(indoc! {"
                    DELETE FROM pane_groups WHERE workspace_id = ?"})?(&workspace.workspace_id)?;
                self.exec_bound(
                    "INSERT OR REPLACE INTO workspaces(workspace_id, dock_visible, dock_anchor) VALUES (?, ?, ?)",
                )?((&workspace.workspace_id, workspace.dock_position))?;
            }
            
            // Save center pane group and dock pane
            self.save_pane_group(&workspace.workspace_id, &workspace.center_group, None)?;
            self.save_pane(&workspace.workspace_id, &workspace.dock_pane, None)?;

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
        self.get_pane_group_children(workspace_id, None)?
            .into_iter()
            .next()
            .context("No center pane group")
            .map(|pane_group| {
                // Rewrite the special case of the root being a leaf node
                if let SerializedPaneGroup::Group { axis: Axis::Horizontal, ref children } = pane_group {
                    if children.len() == 1 {
                        if let Some(SerializedPaneGroup::Pane(pane)) = children.get(0) {
                            return SerializedPaneGroup::Pane(pane.clone())
                        }
                    }
                }
                pane_group
            })
    }

    fn get_pane_group_children<'a>(
        &self,
        workspace_id: &WorkspaceId,
        group_id: Option<GroupId>,
    ) -> Result<Vec<SerializedPaneGroup>> {
        self.select_bound::<(Option<GroupId>, &WorkspaceId), (Option<GroupId>, Option<Axis>, Option<PaneId>)>(indoc! {"
            SELECT group_id, axis, pane_id
            FROM (SELECT group_id, axis, NULL as pane_id, position,  parent_group_id, workspace_id
                  FROM pane_groups
                 UNION
                  SELECT NULL, NULL,  pane_id,  position,  parent_group_id, workspace_id
                  FROM panes
                  -- Remove the dock panes from the union
                  WHERE parent_group_id IS NOT NULL and position IS NOT NULL) 
            WHERE parent_group_id IS ? AND workspace_id = ?
            ORDER BY position
            "})?((group_id, workspace_id))?
        .into_iter()
        .map(|(group_id, axis, pane_id)| {
            if let Some((group_id, axis)) = group_id.zip(axis) {
                Ok(SerializedPaneGroup::Group {
                    axis,
                    children: self.get_pane_group_children(
                        workspace_id,
                        Some(group_id),
                    )?,
                })
            } else if let Some(pane_id) = pane_id {
                Ok(SerializedPaneGroup::Pane(SerializedPane {
                    children: self.get_items( pane_id)?,
                }))
            } else {
                bail!("Pane Group Child was neither a pane group or a pane");
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
        // Rewrite the root node to fit with the database
        let pane_group = if parent.is_none() && matches!(pane_group, SerializedPaneGroup::Pane { .. }) {
            SerializedPaneGroup::Group { axis: Axis::Horizontal, children: vec![pane_group.clone()] }
        } else {
            pane_group.clone()
        };

        match pane_group {
            SerializedPaneGroup::Group { axis, children } => {
                let (parent_id, position) = unzip_option(parent);

                let group_id = self.select_row_bound::<_, i64>(indoc!{"
                    INSERT INTO pane_groups(workspace_id, parent_group_id, position, axis) 
                    VALUES (?, ?, ?, ?) 
                    RETURNING group_id"})?
                    ((workspace_id, parent_id, position, axis))?
                    .ok_or_else(|| anyhow!("Couldn't retrieve group_id from inserted pane_group"))?;
                
                for (position, group) in children.iter().enumerate() {
                    self.save_pane_group(workspace_id, group, Some((group_id, position)))?
                }
                Ok(())
            }
            SerializedPaneGroup::Pane(pane) => {
                self.save_pane(workspace_id, &pane, parent)
            },
        }
    }

    pub(crate) fn get_dock_pane(&self, workspace_id: &WorkspaceId) -> Result<SerializedPane> {
        let pane_id = self.select_row_bound(indoc! {"
            SELECT pane_id FROM panes 
            WHERE workspace_id = ? AND parent_group_id IS NULL AND position IS NULL"})?(
            workspace_id,
        )?
        .context("No dock pane for workspace")?;

        Ok(SerializedPane::new(
            self.get_items(pane_id).context("Reading items")?,
        ))
    }

    pub(crate) fn save_pane(
        &self,
        workspace_id: &WorkspaceId,
        pane: &SerializedPane,
        parent: Option<(GroupId, usize)>,
    ) -> Result<()> {
        let (parent_id, order) = unzip_option(parent);
        
        let pane_id = self.select_row_bound::<_, i64>(indoc!{"
            INSERT INTO panes(workspace_id, parent_group_id, position) 
            VALUES (?, ?, ?) 
            RETURNING pane_id"},
        )?((workspace_id, parent_id, order))?
        .ok_or_else(|| anyhow!("Could not retrieve inserted pane_id"))?;

        self.save_items(workspace_id, pane_id, &pane.children)
            .context("Saving items")
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
    use db::{open_memory_db, write_db_to};
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
                        SerializedPaneGroup::Pane(SerializedPane {
                            children: vec![
                                SerializedItem::new("Terminal", 5),
                                SerializedItem::new("Terminal", 6),
                            ],
                        }),
                        SerializedPaneGroup::Pane(SerializedPane {
                            children: vec![
                                SerializedItem::new("Terminal", 7),
                                SerializedItem::new("Terminal", 8),

                            ],
                        }),
                    ],
                },
                SerializedPaneGroup::Pane(SerializedPane {
                    children: vec![
                        SerializedItem::new("Terminal", 9),
                        SerializedItem::new("Terminal", 10),

                    ],
                }),
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

        let dock_pane = crate::persistence::model::SerializedPane {
            children: vec![
                SerializedItem::new("Terminal", 1),
                SerializedItem::new("Terminal", 4),
                SerializedItem::new("Terminal", 2),
                SerializedItem::new("Terminal", 3),
            ],
        };

        let workspace = default_workspace(&["/tmp"], dock_pane, &Default::default());

        db.save_workspace(None, &workspace);
        write_db_to(&db, "dest.db").unwrap();
        let new_workspace = db.workspace_for_roots(&["/tmp"]).unwrap();

        assert_eq!(workspace.dock_pane, new_workspace.dock_pane);
    }

    #[test]
    fn test_simple_split() {
        // env_logger::try_init().ok();

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
                        SerializedPaneGroup::Pane(SerializedPane {
                            children: vec![
                                SerializedItem::new("Terminal", 1),
                                SerializedItem::new("Terminal", 2),
                            ],
                        }),
                        SerializedPaneGroup::Pane(SerializedPane {
                            children: vec![
                                SerializedItem::new("Terminal", 4),
                                SerializedItem::new("Terminal", 3),
                            ],
                        }),
                    ],
                },
                SerializedPaneGroup::Pane(SerializedPane {
                    children: vec![
                        SerializedItem::new("Terminal", 5),
                        SerializedItem::new("Terminal", 6),
                    ],
                }),
            ],
        };

        let workspace = default_workspace(&["/tmp"], Default::default(), &center_pane);

        db.save_workspace(None, &workspace);

        assert_eq!(workspace.center_group, center_pane);
    }
}
