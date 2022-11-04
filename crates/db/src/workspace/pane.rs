use anyhow::{Context, Result};
use indoc::indoc;
use sqlez::migrations::Migration;
use util::unzip_option;

use crate::model::{GroupId, PaneId, SerializedPane};

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
            parent_group INTEGER, -- NULL indicates that this is a root node
            axis TEXT NOT NULL, -- Enum:  'Vertical' / 'Horizontal'
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
            FOREIGN KEY(parent_group) REFERENCES pane_groups(group_id) ON DELETE CASCADE
        ) STRICT;
        
        CREATE TABLE panes(
            pane_id INTEGER PRIMARY KEY,
            workspace_id BLOB NOT NULL,
            group_id INTEGER, -- If null, this is a dock pane
            position INTEGER, -- If null, this is a dock pane
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
            FOREIGN KEY(group_id) REFERENCES pane_groups(group_id) ON DELETE CASCADE
        ) STRICT;
    "}],
);

impl Db {
    pub(crate) fn get_center_group(
        &self,
        _workspace_id: &WorkspaceId,
    ) -> Result<SerializedPaneGroup> {
        Ok(SerializedPaneGroup::new())
    }

    pub(crate) fn save_center_group(
        &self,
        _workspace_id: &WorkspaceId,
        _center_pane_group: &SerializedPaneGroup,
    ) -> Result<()> {
        // Delete the center pane group for this workspace and any of its children
        // Generate new pane group IDs as we go through
        // insert them
        Ok(())
    }

    pub(crate) fn get_dock_pane(&self, workspace_id: &WorkspaceId) -> Result<SerializedPane> {
        let pane_id = self
            .prepare(indoc! {"
                SELECT pane_id FROM panes 
                WHERE workspace_id = ? AND group_id IS NULL AND position IS NULL"})?
            .with_bindings(workspace_id)?
            .row::<PaneId>()?;

        Ok(SerializedPane::new(
            self.get_items(pane_id).context("Reading items")?,
        ))
    }

    pub(crate) fn save_dock_pane(
        &self,
        workspace: &WorkspaceId,
        dock_pane: &SerializedPane,
    ) -> Result<()> {
        self.save_pane(workspace, &dock_pane, None)
    }

    pub(crate) fn save_pane(
        &self,
        workspace_id: &WorkspaceId,
        pane: &SerializedPane,
        parent: Option<(GroupId, usize)>,
    ) -> Result<()> {
        let (parent_id, order) = unzip_option(parent);

        let pane_id = self
            .prepare("INSERT INTO panes(workspace_id, group_id, position) VALUES (?, ?, ?)")?
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
        center_group: SerializedPaneGroup,
    ) -> SerializedWorkspace {
        SerializedWorkspace {
            dock_anchor: crate::model::DockAnchor::Right,
            dock_visible: false,
            center_group,
            dock_pane,
        }
    }

    #[test]
    fn test_basic_dock_pane() {
        let db = Db::open_in_memory("basic_dock_pane");

        let dock_pane = crate::model::SerializedPane {
            children: vec![
                SerializedItem::Terminal { item_id: 1 },
                SerializedItem::Terminal { item_id: 4 },
                SerializedItem::Terminal { item_id: 2 },
                SerializedItem::Terminal { item_id: 3 },
            ],
        };

        let workspace = default_workspace(dock_pane, SerializedPaneGroup::new());

        db.save_workspace(&["/tmp"], None, &workspace);

        let new_workspace = db.workspace_for_roots(&["/tmp"]).unwrap();

        assert_eq!(workspace.dock_pane, new_workspace.dock_pane);
    }

    // #[test]
    // fn test_dock_simple_split() {
    //     let db = Db::open_in_memory("simple_split");

    //     let workspace = db.workspace_for_roots(&["/tmp"]);

    //     // Pane group -> Pane -> 10 , 20
    //     let center_pane = SerializedPaneGroup {
    //         axis: gpui::Axis::Horizontal,
    //         children: vec![PaneGroupChild::Pane(SerializedPane {
    //             items: vec![ItemId { item_id: 10 }, ItemId { item_id: 20 }],
    //         })],
    //     };

    //     db.save_pane_splits(&workspace.workspace_id, &center_pane);

    //     // let new_workspace = db.workspace_for_roots(&["/tmp"]);

    //     // assert_eq!(new_workspace.center_group, center_pane);
    // }
}
