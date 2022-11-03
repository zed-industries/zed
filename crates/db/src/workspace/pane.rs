use gpui::Axis;
use indoc::indoc;
use sqlez::migrations::Migration;
use util::{iife, ResultExt};

use super::{
    model::{PaneGroupId, PaneId, SerializedDockPane, SerializedPaneGroup, WorkspaceId},
    Db,
};

pub(crate) const PANE_MIGRATIONS: Migration = Migration::new(
    "pane",
    &[indoc! {"
        CREATE TABLE pane_groups(
            group_id INTEGER PRIMARY KEY,
            workspace_id INTEGER NOT NULL,
            parent_group INTEGER, -- NULL indicates that this is a root node
            axis TEXT NOT NULL, -- Enum:  'Vertical' / 'Horizontal'
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
            FOREIGN KEY(parent_group) REFERENCES pane_groups(group_id) ON DELETE CASCADE
        ) STRICT;
        
        CREATE TABLE panes(
            pane_id INTEGER PRIMARY KEY,
            workspace_id INTEGER NOT NULL,
            group_id INTEGER, -- If null, this is a dock pane
            idx INTEGER NOT NULL,
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
            FOREIGN KEY(group_id) REFERENCES pane_groups(group_id) ON DELETE CASCADE
        ) STRICT;
        
        CREATE TABLE items(
            item_id INTEGER NOT NULL, -- This is the item's view id, so this is not unique
            pane_id INTEGER NOT NULL,
            workspace_id INTEGER NOT NULL,
            kind TEXT NOT NULL,
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE
            FOREIGN KEY(pane_id) REFERENCES panes(pane_id) ON DELETE CASCADE
            PRIMARY KEY(item_id, workspace_id)
        ) STRICT;
    "}],
);

impl Db {
    pub(crate) fn get_center_group(&self, _workspace: WorkspaceId) -> SerializedPaneGroup {
        unimplemented!()
    }

    pub(crate) fn get_pane_group(&self, _pane_group_id: PaneGroupId) -> SerializedPaneGroup {
        unimplemented!()
        // let axis = self.get_pane_group_axis(pane_group_id);
        // let mut children: Vec<(usize, PaneGroupChild)> = Vec::new();
        // for child_row in self.get_pane_group_children(pane_group_id) {
        //     if let Some(child_pane_id) = child_row.child_pane_id {
        //         children.push((
        //             child_row.index,
        //             PaneGroupChild::Pane(self.get_pane(PaneId {
        //                 workspace_id: pane_group_id.workspace_id,
        //                 pane_id: child_pane_id,
        //             })),
        //         ));
        //     } else if let Some(child_group_id) = child_row.child_group_id {
        //         children.push((
        //             child_row.index,
        //             PaneGroupChild::Group(self.get_pane_group(PaneGroupId {
        //                 workspace_id: pane_group_id.workspace_id,
        //                 group_id: child_group_id,
        //             })),
        //         ));
        //     }
        // }
        // children.sort_by_key(|(index, _)| *index);

        // SerializedPaneGroup {
        //     group_id: pane_group_id,
        //     axis,
        //     children: children.into_iter().map(|(_, child)| child).collect(),
        // }
    }

    // fn _get_pane_group_children(
    //     &self,
    //     _pane_group_id: PaneGroupId,
    // ) -> impl Iterator<Item = PaneGroupChildRow> {
    //     Vec::new().into_iter()
    // }

    pub(crate) fn save_pane_splits(
        &self,
        _workspace: &WorkspaceId,
        _center_pane_group: &SerializedPaneGroup,
    ) {
        // Delete the center pane group for this workspace and any of its children
        // Generate new pane group IDs as we go through
        // insert them
    }

    pub(crate) fn _get_pane(&self, _pane_id: PaneId) -> SerializedPane {
        unimplemented!();
    }

    pub(crate) fn get_dock_pane(&self, workspace: WorkspaceId) -> Option<SerializedDockPane> {
        iife!({
            self.prepare("SELECT anchor_position, visible FROM dock_panes WHERE workspace_id = ?")?
                .with_bindings(workspace)?
                .maybe_row::<SerializedDockPane>()
        })
        .log_err()
        .flatten()
    }

    pub(crate) fn save_dock_pane(&self, workspace: &WorkspaceId, dock_pane: &SerializedDockPane) {
        // iife!({
        //     self.prepare(
        //         "INSERT INTO dock_panes (workspace_id, anchor_position, visible) VALUES (?, ?, ?);",
        //     )?
        //     .with_bindings(dock_pane.to_row(workspace))?
        //     .insert()
        // })
        // .log_err();
    }
}

#[cfg(test)]
mod tests {

    // use crate::{items::ItemId, pane::SerializedPane, Db, DockAnchor};

    // use super::{PaneGroupChild, SerializedDockPane, SerializedPaneGroup};

    // #[test]
    // fn test_basic_dock_pane() {
    //     let db = Db::open_in_memory("basic_dock_pane");

    //     let workspace = db.workspace_for_roots(&["/tmp"]);

    //     let dock_pane = SerializedDockPane {
    //         anchor_position: DockAnchor::Expanded,
    //         visible: true,
    //     };

    //     db.save_dock_pane(&workspace.workspace_id, &dock_pane);

    //     let new_workspace = db.workspace_for_roots(&["/tmp"]);

    //     assert_eq!(new_workspace.dock_pane.unwrap(), dock_pane);
    // }

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
