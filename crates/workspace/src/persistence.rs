#![allow(dead_code)]

pub mod model;

use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use db::open_file_db;
use gpui::Axis;
use indoc::indoc;
use lazy_static::lazy_static;

use sqlez::thread_safe_connection::ThreadSafeConnection;
use sqlez::{connection::Connection, domain::Domain, migrations::Migration};
use util::{iife, unzip_option, ResultExt};

use super::Workspace;

use model::{
    GroupId, PaneId, SerializedItem, SerializedItemKind, SerializedPane, SerializedPaneGroup,
    SerializedWorkspace, WorkspaceId,
};

lazy_static! {
    pub static ref DB: WorkspaceDb = WorkspaceDb(open_file_db());
}

pub struct WorkspaceDb(ThreadSafeConnection<Workspace>);

impl Deref for WorkspaceDb {
    type Target = ThreadSafeConnection<Workspace>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) const WORKSPACES_MIGRATION: Migration = Migration::new(
    "workspace",
    &[indoc! {"
        CREATE TABLE workspaces(
            workspace_id BLOB PRIMARY KEY,
            dock_anchor TEXT, -- Enum: 'Bottom' / 'Right' / 'Expanded'
            dock_visible INTEGER, -- Boolean
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
            parent_group_id INTEGER, -- NULL, this is a dock pane
            position INTEGER, -- NULL, this is a dock pane
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
    "}],
);

impl Domain for Workspace {
    fn migrate(conn: &Connection) -> anyhow::Result<()> {
        WORKSPACES_MIGRATION.run(&conn)
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
        let (workspace_id, dock_position) = iife!({
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
    pub fn save_workspace<P: AsRef<Path>>(
        &self,
        worktree_roots: &[P],
        old_roots: Option<&[P]>,
        workspace: &SerializedWorkspace,
    ) {
        let workspace_id: WorkspaceId = worktree_roots.into();

        self.with_savepoint("update_worktrees", || {
            if let Some(old_roots) = old_roots {
                let old_id: WorkspaceId = old_roots.into();

                self.exec_bound("DELETE FROM WORKSPACES WHERE workspace_id = ?")?(&old_id)?;
            }

            // Delete any previous workspaces with the same roots. This cascades to all
            // other tables that are based on the same roots set.
            // Insert new workspace into workspaces table if none were found
            self.exec_bound("DELETE FROM workspaces WHERE workspace_id = ?;")?(&workspace_id)?;

            self.exec_bound(
                "INSERT INTO workspaces(workspace_id, dock_visible, dock_anchor) VALUES (?, ?, ?)",
            )?((&workspace_id, workspace.dock_position))?;

            // Save center pane group and dock pane
            self.save_pane_group(&workspace_id, &workspace.center_group, None)?;
            self.save_pane(&workspace_id, &workspace.dock_pane, None)?;

            Ok(())
        })
        .with_context(|| {
            format!(
                "Update workspace with roots {:?}",
                worktree_roots
                    .iter()
                    .map(|p| p.as_ref())
                    .collect::<Vec<_>>()
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
        if parent.is_none() && !matches!(pane_group, SerializedPaneGroup::Group { .. }) {
            bail!("Pane groups must have a SerializedPaneGroup::Group at the root")
        }

        let (parent_id, position) = unzip_option(parent);

        match pane_group {
            SerializedPaneGroup::Group { axis, children } => {
                let parent_id = self.insert_bound("INSERT INTO pane_groups(workspace_id, parent_group_id, position, axis) VALUES (?, ?, ?, ?)")?
                    ((workspace_id, parent_id, position, *axis))?;

                for (position, group) in children.iter().enumerate() {
                    self.save_pane_group(workspace_id, group, Some((parent_id, position)))?
                }
                Ok(())
            }
            SerializedPaneGroup::Pane(pane) => self.save_pane(workspace_id, pane, parent),
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

        let pane_id = self.insert_bound(
            "INSERT INTO panes(workspace_id, parent_group_id, position) VALUES (?, ?, ?)",
        )?((workspace_id, parent_id, order))?;

        self.save_items(workspace_id, pane_id, &pane.children)
            .context("Saving items")
    }

    pub(crate) fn get_items(&self, pane_id: PaneId) -> Result<Vec<SerializedItem>> {
        Ok(self.select_bound(indoc! {"
            SELECT item_id, kind FROM items
            WHERE pane_id = ?
            ORDER BY position"})?(pane_id)?
        .into_iter()
        .map(|(item_id, kind)| match kind {
            SerializedItemKind::Terminal => SerializedItem::Terminal { item_id },
            _ => unimplemented!(),
        })
        .collect())
    }

    pub(crate) fn save_items(
        &self,
        workspace_id: &WorkspaceId,
        pane_id: PaneId,
        items: &[SerializedItem],
    ) -> Result<()> {
        let mut delete_old = self
            .exec_bound("DELETE FROM items WHERE workspace_id = ? AND pane_id = ? AND item_id = ?")
            .context("Preparing deletion")?;
        let mut insert_new = self.exec_bound(
            "INSERT INTO items(item_id, workspace_id, pane_id, kind, position) VALUES (?, ?, ?, ?, ?)",
        ).context("Preparing insertion")?;
        for (position, item) in items.iter().enumerate() {
            delete_old((workspace_id, pane_id, item.item_id()))?;
            insert_new((item.item_id(), workspace_id, pane_id, item.kind(), position))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use db::open_memory_db;
    use settings::DockAnchor;

    use super::*;

    #[test]
    fn test_workspace_assignment() {
        // env_logger::try_init().ok();

        let db = WorkspaceDb(open_memory_db("test_basic_functionality"));

        let workspace_1 = SerializedWorkspace {
            dock_position: crate::dock::DockPosition::Shown(DockAnchor::Bottom),
            center_group: Default::default(),
            dock_pane: Default::default(),
        };

        let workspace_2 = SerializedWorkspace {
            dock_position: crate::dock::DockPosition::Hidden(DockAnchor::Expanded),
            center_group: Default::default(),
            dock_pane: Default::default(),
        };

        let workspace_3 = SerializedWorkspace {
            dock_position: crate::dock::DockPosition::Shown(DockAnchor::Right),
            center_group: Default::default(),
            dock_pane: Default::default(),
        };

        db.save_workspace(&["/tmp", "/tmp2"], None, &workspace_1);
        db.save_workspace(&["/tmp"], None, &workspace_2);

        db::write_db_to(&db, "test.db").unwrap();

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
        db.save_workspace(&["/tmp", "/tmp2"], Some(&["/tmp", "/tmp2"]), &workspace_2);
        assert_eq!(
            db.workspace_for_roots(&["/tmp", "/tmp2"]).unwrap(),
            workspace_2
        );

        // Test other mechanism for mutating
        db.save_workspace(&["/tmp", "/tmp2"], None, &workspace_3);
        assert_eq!(
            db.workspace_for_roots(&["/tmp", "/tmp2"]).unwrap(),
            workspace_3
        );

        // Make sure that updating paths differently also works
        db.save_workspace(
            &["/tmp3", "/tmp4", "/tmp2"],
            Some(&["/tmp", "/tmp2"]),
            &workspace_3,
        );
        assert_eq!(db.workspace_for_roots(&["/tmp2", "tmp"]), None);
        assert_eq!(
            db.workspace_for_roots(&["/tmp2", "/tmp3", "/tmp4"])
                .unwrap(),
            workspace_3
        );
    }

    use crate::persistence::model::SerializedWorkspace;
    use crate::persistence::model::{SerializedItem, SerializedPane, SerializedPaneGroup};

    fn default_workspace(
        dock_pane: SerializedPane,
        center_group: &SerializedPaneGroup,
    ) -> SerializedWorkspace {
        SerializedWorkspace {
            dock_position: crate::dock::DockPosition::Hidden(DockAnchor::Right),
            center_group: center_group.clone(),
            dock_pane,
        }
    }

    #[test]
    fn test_basic_dock_pane() {
        // env_logger::try_init().ok();

        let db = WorkspaceDb(open_memory_db("basic_dock_pane"));

        let dock_pane = crate::persistence::model::SerializedPane {
            children: vec![
                SerializedItem::Terminal { item_id: 1 },
                SerializedItem::Terminal { item_id: 4 },
                SerializedItem::Terminal { item_id: 2 },
                SerializedItem::Terminal { item_id: 3 },
            ],
        };

        let workspace = default_workspace(dock_pane, &Default::default());

        db.save_workspace(&["/tmp"], None, &workspace);

        let new_workspace = db.workspace_for_roots(&["/tmp"]).unwrap();

        assert_eq!(workspace.dock_pane, new_workspace.dock_pane);
    }

    #[test]
    fn test_simple_split() {
        // env_logger::try_init().ok();

        let db = WorkspaceDb(open_memory_db("simple_split"));

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
                                SerializedItem::Terminal { item_id: 1 },
                                SerializedItem::Terminal { item_id: 2 },
                            ],
                        }),
                        SerializedPaneGroup::Pane(SerializedPane {
                            children: vec![
                                SerializedItem::Terminal { item_id: 4 },
                                SerializedItem::Terminal { item_id: 3 },
                            ],
                        }),
                    ],
                },
                SerializedPaneGroup::Pane(SerializedPane {
                    children: vec![
                        SerializedItem::Terminal { item_id: 5 },
                        SerializedItem::Terminal { item_id: 6 },
                    ],
                }),
            ],
        };

        let workspace = default_workspace(Default::default(), &center_pane);

        db.save_workspace(&["/tmp"], None, &workspace);

        assert_eq!(workspace.center_group, center_pane);
    }
}
