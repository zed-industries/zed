use gpui::Axis;

use crate::{items::ItemId, workspace::WorkspaceId};

use super::Db;

pub(crate) const PANE_M_1: &str = "
CREATE TABLE pane_groups(
    workspace_id INTEGER,
    group_id INTEGER,
    axis STRING NOT NULL, -- 'Vertical' / 'Horizontal'
    PRIMARY KEY (workspace_id, group_id)
) STRICT;

CREATE TABLE pane_group_children(
    workspace_id INTEGER,
    group_id INTEGER,
    child_pane_id INTEGER,  -- Nullable
    child_group_id INTEGER, -- Nullable
    index INTEGER,
    PRIMARY KEY (workspace_id, group_id)
) STRICT;

CREATE TABLE pane_items(
    workspace_id INTEGER,
    pane_id INTEGER,
    item_id INTEGER, -- Array
    index INTEGER,
    KEY (workspace_id, pane_id)
) STRICT;

ALTER TABLE WORKSPACE
ADD THESE COLS:
center_group INTEGER NOT NULL,
dock_pane INTEGER NOT NULL,
--    FOREIGN KEY(center_group) REFERENCES pane_groups(group_id)
--    FOREIGN KEY(dock_pane) REFERENCES pane_items(pane_id)
";

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PaneId {
    workspace_id: WorkspaceId,
    pane_id: usize,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PaneGroupId {
    workspace_id: WorkspaceId,
    group_id: usize,
}

impl PaneGroupId {
    pub(crate) fn root(workspace_id: WorkspaceId) -> Self {
        Self {
            workspace_id,
            group_id: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SerializedPaneGroup {
    group_id: PaneGroupId,
    axis: Axis,
    children: Vec<PaneGroupChild>,
}

impl SerializedPaneGroup {
    pub(crate) fn empty_root(workspace_id: WorkspaceId) -> Self {
        Self {
            group_id: PaneGroupId::root(workspace_id),
            axis: Default::default(),
            children: Default::default(),
        }
    }
}

struct PaneGroupChildRow {
    child_pane_id: Option<usize>,
    child_group_id: Option<usize>,
    index: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PaneGroupChild {
    Pane(SerializedPane),
    Group(SerializedPaneGroup),
}

#[derive(Debug, PartialEq, Eq)]
pub struct SerializedPane {
    pane_id: PaneId,
    children: Vec<ItemId>,
}

impl Db {
    pub(crate) fn get_pane_group(&self, pane_group_id: PaneGroupId) -> SerializedPaneGroup {
        let axis = self.get_pane_group_axis(pane_group_id);
        let mut children: Vec<(usize, PaneGroupChild)> = Vec::new();
        for child_row in self.get_pane_group_children(pane_group_id) {
            if let Some(child_pane_id) = child_row.child_pane_id {
                children.push((
                    child_row.index,
                    PaneGroupChild::Pane(self.get_pane(PaneId {
                        workspace_id: pane_group_id.workspace_id,
                        pane_id: child_pane_id,
                    })),
                ));
            } else if let Some(child_group_id) = child_row.child_group_id {
                children.push((
                    child_row.index,
                    PaneGroupChild::Group(self.get_pane_group(PaneGroupId {
                        workspace_id: pane_group_id.workspace_id,
                        group_id: child_group_id,
                    })),
                ));
            }
        }
        children.sort_by_key(|(index, _)| *index);

        SerializedPaneGroup {
            group_id: pane_group_id,
            axis,
            children: children.into_iter().map(|(_, child)| child).collect(),
        }
    }

    fn get_pane_group_children(
        &self,
        pane_group_id: PaneGroupId,
    ) -> impl Iterator<Item = PaneGroupChildRow> {
        Vec::new().into_iter()
    }

    fn get_pane_group_axis(&self, pane_group_id: PaneGroupId) -> Axis {
        unimplemented!();
    }

    pub fn save_pane_splits(&self, center_pane_group: SerializedPaneGroup) {
        // Delete the center pane group for this workspace and any of its children
        // Generate new pane group IDs as we go through
        // insert them
        // Items garbage collect themselves when dropped
    }

    pub(crate) fn get_pane(&self, pane_id: PaneId) -> SerializedPane {
        unimplemented!();
    }

    pub fn save_pane(&self, pane: SerializedPane) {}
}
