use std::str::FromStr;

use gpui::Axis;
use indoc::indoc;
use sqlez::{
    bindable::{Bind, Column},
    migrations::Migration,
    statement::Statement,
};
use util::{iife, ResultExt};

use crate::{items::ItemId, workspace::WorkspaceId};

use super::Db;

pub(crate) const PANE_MIGRATIONS: Migration = Migration::new(
    "pane",
    &[indoc! {"
CREATE TABLE dock_panes(
    dock_pane_id INTEGER PRIMARY KEY,
    workspace_id INTEGER NOT NULL,
    anchor_position TEXT NOT NULL, -- Enum: 'Bottom' / 'Right' / 'Expanded'
    visible INTEGER NOT NULL, -- Boolean
    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE
) STRICT;

CREATE TABLE pane_groups( -- Inner nodes
    group_id INTEGER PRIMARY KEY,
    workspace_id INTEGER NOT NULL,
    parent_group INTEGER, -- NULL indicates that this is a root node
    axis TEXT NOT NULL, -- Enum:  'Vertical' / 'Horizontal'
    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
    FOREIGN KEY(parent_group) REFERENCES pane_groups(group_id) ON DELETE CASCADE
) STRICT;


CREATE TABLE grouped_panes( -- Leaf nodes 
    pane_id INTEGER PRIMARY KEY,
    workspace_id INTEGER NOT NULL,
    group_id INTEGER NOT NULL,
    idx INTEGER NOT NULL,
    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
    FOREIGN KEY(group_id) REFERENCES pane_groups(group_id) ON DELETE CASCADE
) STRICT;

CREATE TABLE items(
    item_id INTEGER PRIMARY KEY,
    workspace_id INTEGER NOT NULL,
    kind TEXT NOT NULL,
    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE
) STRICT;

CREATE TABLE group_items(
    workspace_id INTEGER NOT NULL,
    pane_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    idx INTEGER NOT NULL,
    PRIMARY KEY (workspace_id, pane_id, item_id)
    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
    FOREIGN KEY(pane_id) REFERENCES grouped_panes(pane_id) ON DELETE CASCADE,
    FOREIGN KEY(item_id) REFERENCES items(item_id) ON DELETE CASCADE
) STRICT;

CREATE TABLE dock_items(
    workspace_id INTEGER NOT NULL,
    dock_pane_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    idx INTEGER NOT NULL,
    PRIMARY KEY (workspace_id, dock_pane_id, item_id)
    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
    FOREIGN KEY(dock_pane_id) REFERENCES dock_panes(dock_pane_id) ON DELETE CASCADE,
    FOREIGN KEY(item_id) REFERENCES items(item_id)ON DELETE CASCADE
) STRICT;
"}],
);

// We have an many-branched, unbalanced tree with three types:
// Pane Groups
// Panes
// Items

// The root is always a Pane Group
// Pane Groups can have 0 (or more) Panes and/or Pane Groups as children
// Panes can have 0 or more items as children
// Panes can be their own root
// Items cannot have children
// References pointing down is hard (SQL doesn't like arrays)
// References pointing up is easy (1-1 item / parent relationship) but is harder to query
//

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
    pub fn root(workspace_id: WorkspaceId) -> Self {
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
    pub fn empty_root(workspace_id: WorkspaceId) -> Self {
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

//********* CURRENTLY IN USE TYPES: *********

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DockAnchor {
    #[default]
    Bottom,
    Right,
    Expanded,
}

impl ToString for DockAnchor {
    fn to_string(&self) -> String {
        match self {
            DockAnchor::Bottom => "Bottom".to_string(),
            DockAnchor::Right => "Right".to_string(),
            DockAnchor::Expanded => "Expanded".to_string(),
        }
    }
}

impl FromStr for DockAnchor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "Bottom" => Ok(DockAnchor::Bottom),
            "Right" => Ok(DockAnchor::Right),
            "Expanded" => Ok(DockAnchor::Expanded),
            _ => anyhow::bail!("Not a valid dock anchor"),
        }
    }
}

impl Bind for DockAnchor {
    fn bind(&self, statement: &Statement, start_index: i32) -> anyhow::Result<i32> {
        statement.bind(self.to_string(), start_index)
    }
}

impl Column for DockAnchor {
    fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
        <String as Column>::column(statement, start_index)
            .and_then(|(str, next_index)| Ok((DockAnchor::from_str(&str)?, next_index)))
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct SerializedDockPane {
    pub anchor_position: DockAnchor,
    pub visible: bool,
}

impl SerializedDockPane {
    fn to_row(&self, workspace: &WorkspaceId) -> DockRow {
        DockRow {
            workspace_id: *workspace,
            anchor_position: self.anchor_position,
            visible: self.visible,
        }
    }
}

impl Column for SerializedDockPane {
    fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
        <(DockAnchor, bool) as Column>::column(statement, start_index).map(
            |((anchor_position, visible), next_index)| {
                (
                    SerializedDockPane {
                        anchor_position,
                        visible,
                    },
                    next_index,
                )
            },
        )
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub(crate) struct DockRow {
    workspace_id: WorkspaceId,
    anchor_position: DockAnchor,
    visible: bool,
}

impl Bind for DockRow {
    fn bind(&self, statement: &Statement, start_index: i32) -> anyhow::Result<i32> {
        statement.bind(
            (
                self.workspace_id,
                self.anchor_position.to_string(),
                self.visible,
            ),
            start_index,
        )
    }
}

impl Db {
    pub fn get_pane_group(&self, pane_group_id: PaneGroupId) -> SerializedPaneGroup {
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
        _pane_group_id: PaneGroupId,
    ) -> impl Iterator<Item = PaneGroupChildRow> {
        Vec::new().into_iter()
    }

    fn get_pane_group_axis(&self, _pane_group_id: PaneGroupId) -> Axis {
        unimplemented!();
    }

    pub fn save_pane_splits(&self, _center_pane_group: SerializedPaneGroup) {
        // Delete the center pane group for this workspace and any of its children
        // Generate new pane group IDs as we go through
        // insert them
        // Items garbage collect themselves when dropped
    }

    pub(crate) fn get_pane(&self, _pane_id: PaneId) -> SerializedPane {
        unimplemented!();
    }

    pub fn get_dock_pane(&self, workspace: WorkspaceId) -> Option<SerializedDockPane> {
        iife!({
            self.prepare("SELECT anchor_position, visible FROM dock_panes WHERE workspace_id = ?")?
                .with_bindings(workspace)?
                .maybe_row::<SerializedDockPane>()
        })
        .log_err()
        .flatten()
    }

    pub fn save_dock_pane(&self, workspace: &WorkspaceId, dock_pane: &SerializedDockPane) {
        iife!({
            self.prepare(
                "INSERT INTO dock_panes (workspace_id, anchor_position, visible) VALUES (?, ?, ?);",
            )?
            .with_bindings(dock_pane.to_row(workspace))?
            .insert()
        })
        .log_err();
    }
}

#[cfg(test)]
mod tests {

    use crate::{pane::SerializedPane, Db};

    use super::{DockAnchor, SerializedDockPane};

    #[test]
    fn test_basic_dock_pane() {
        let db = Db::open_in_memory("basic_dock_pane");

        let workspace = db.workspace_for_roots(&["/tmp"]);

        let dock_pane = SerializedDockPane {
            anchor_position: DockAnchor::Expanded,
            visible: true,
        };

        db.save_dock_pane(&workspace.workspace_id, &dock_pane);

        let new_workspace = db.workspace_for_roots(&["/tmp"]);

        assert_eq!(new_workspace.dock_pane.unwrap(), dock_pane);
    }

    #[test]
    fn test_dock_simple_split() {
        let db = Db::open_in_memory("simple_split");

        let workspace = db.workspace_for_roots(&["/tmp"]);

        let center_pane = SerializedPane {
            pane_id: crate::pane::PaneId {
                workspace_id: workspace.workspace_id,
                pane_id: 1,
            },
            children: vec![],
        };

        db.save_dock_pane(&workspace.workspace_id, &dock_pane);

        let new_workspace = db.workspace_for_roots(&["/tmp"]);

        assert_eq!(new_workspace.dock_pane.unwrap(), dock_pane);
    }
}
