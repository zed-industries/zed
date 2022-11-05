use anyhow::{bail, Context, Result};
use indoc::indoc;
use sqlez::{migrations::Migration, statement::Statement};
use util::unzip_option;

use crate::model::{Axis, GroupId, PaneId, SerializedPane};

use super::{
    model::{SerializedPaneGroup, WorkspaceId},
    Db,
};

pub(crate) const PANE_MIGRATIONS: Migration = Migration::new(
    "pane",
    &[indoc! {"
        CREATE TABLE pane_groups(
            group_id INTEGER PRIMARY KEY,
            workspace_id BLOB NOT NULL,
            parent_group_id INTEGER, -- NULL indicates that this is a root node
            position INTEGER, -- NULL indicates that this is a root node
            axis TEXT NOT NULL, -- Enum:  'Vertical' / 'Horizontal'
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
            FOREIGN KEY(parent_group_id) REFERENCES pane_groups(group_id) ON DELETE CASCADE
        ) STRICT;
        
        CREATE TABLE panes(
            pane_id INTEGER PRIMARY KEY,
            workspace_id BLOB NOT NULL,
            parent_group_id INTEGER, -- NULL, this is a dock pane
            position INTEGER, -- NULL, this is a dock pane
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
            FOREIGN KEY(parent_group_id) REFERENCES pane_groups(group_id) ON DELETE CASCADE
        ) STRICT;
    "}],
);

impl Db {
    pub(crate) fn get_center_pane_group(
        &self,
        workspace_id: &WorkspaceId,
    ) -> Result<SerializedPaneGroup> {
        let mut query = self.prepare(indoc! {"
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
            "})?;

        self.get_pane_group_children(workspace_id, None, &mut query)?
            .into_iter()
            .next()
            .context("No center pane group")
    }

    fn get_pane_group_children(
        &self,
        workspace_id: &WorkspaceId,
        group_id: Option<GroupId>,
        query: &mut Statement,
    ) -> Result<Vec<SerializedPaneGroup>> {
        let children = query.with_bindings((group_id, workspace_id))?.rows::<(
            Option<GroupId>,
            Option<Axis>,
            Option<PaneId>,
        )>()?;

        children
            .into_iter()
            .map(|(group_id, axis, pane_id)| {
                if let Some((group_id, axis)) = group_id.zip(axis) {
                    Ok(SerializedPaneGroup::Group {
                        axis,
                        children: self.get_pane_group_children(
                            workspace_id,
                            Some(group_id),
                            query,
                        )?,
                    })
                } else if let Some(pane_id) = pane_id {
                    Ok(SerializedPaneGroup::Pane(SerializedPane {
                        children: self.get_items(pane_id)?,
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
                let parent_id = self.prepare("INSERT INTO pane_groups(workspace_id, parent_group_id, position, axis) VALUES (?, ?, ?, ?)")?
                    .with_bindings((workspace_id, parent_id, position, *axis))?
                    .insert()? as GroupId;

                for (position, group) in children.iter().enumerate() {
                    self.save_pane_group(workspace_id, group, Some((parent_id, position)))?
                }
                Ok(())
            }
            SerializedPaneGroup::Pane(pane) => self.save_pane(workspace_id, pane, parent),
        }
    }

    pub(crate) fn get_dock_pane(&self, workspace_id: &WorkspaceId) -> Result<SerializedPane> {
        let pane_id = self
            .prepare(indoc! {"
                SELECT pane_id FROM panes 
                WHERE workspace_id = ? AND parent_group_id IS NULL AND position IS NULL"})?
            .with_bindings(workspace_id)?
            .row::<PaneId>()?;

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

        let pane_id = self
            .prepare("INSERT INTO panes(workspace_id, parent_group_id, position) VALUES (?, ?, ?)")?
            .with_bindings((workspace_id, parent_id, order))?
            .insert()? as PaneId;

        self.save_items(workspace_id, pane_id, &pane.children)
            .context("Saving items")
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        model::{SerializedItem, SerializedPane, SerializedPaneGroup, SerializedWorkspace},
        Db,
    };

    fn default_workspace(
        dock_pane: SerializedPane,
        center_group: &SerializedPaneGroup,
    ) -> SerializedWorkspace {
        SerializedWorkspace {
            dock_anchor: crate::model::DockAnchor::Right,
            dock_visible: false,
            center_group: center_group.clone(),
            dock_pane,
        }
    }

    #[test]
    fn test_basic_dock_pane() {
        env_logger::try_init().ok();

        let db = Db::open_in_memory("basic_dock_pane");

        let dock_pane = crate::model::SerializedPane {
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
        env_logger::try_init().ok();

        let db = Db::open_in_memory("simple_split");

        //  -----------------
        //  | 1,2   | 5,6   |
        //  | - - - |       |
        //  | 3,4   |       |
        //  -----------------
        let center_pane = SerializedPaneGroup::Group {
            axis: crate::model::Axis::Horizontal,
            children: vec![
                SerializedPaneGroup::Group {
                    axis: crate::model::Axis::Vertical,
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
