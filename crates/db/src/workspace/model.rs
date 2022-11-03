use anyhow::{bail, Result};

use gpui::Axis;
use sqlez::{
    bindable::{Bind, Column},
    statement::Statement,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub(crate) struct WorkspaceId(pub(crate) i64);

impl Bind for WorkspaceId {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        self.0.bind(statement, start_index)
    }
}

impl Column for WorkspaceId {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        i64::column(statement, start_index).map(|(id, next_index)| (Self(id), next_index))
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DockAnchor {
    #[default]
    Bottom,
    Right,
    Expanded,
}

impl Bind for DockAnchor {
    fn bind(&self, statement: &Statement, start_index: i32) -> anyhow::Result<i32> {
        match self {
            DockAnchor::Bottom => "Bottom",
            DockAnchor::Right => "Right",
            DockAnchor::Expanded => "Expanded",
        }
        .bind(statement, start_index)
    }
}

impl Column for DockAnchor {
    fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
        String::column(statement, start_index).and_then(|(anchor_text, next_index)| {
            Ok((
                match anchor_text.as_ref() {
                    "Bottom" => DockAnchor::Bottom,
                    "Right" => DockAnchor::Right,
                    "Expanded" => DockAnchor::Expanded,
                    _ => bail!("Stored dock anchor is incorrect"),
                },
                next_index,
            ))
        })
    }
}

pub(crate) type WorkspaceRow = (WorkspaceId, DockAnchor, bool);

#[derive(Default, Debug)]
pub struct SerializedWorkspace {
    pub center_group: SerializedPaneGroup,
    pub dock_anchor: DockAnchor,
    pub dock_visible: bool,
    pub dock_pane: SerializedDockPane,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PaneId {
    workspace_id: WorkspaceId,
    pane_id: usize,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PaneGroupId {
    workspace_id: WorkspaceId,
}

impl PaneGroupId {
    pub fn root(workspace_id: WorkspaceId) -> Self {
        Self {
            workspace_id,
            // group_id: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct SerializedPaneGroup {
    axis: Axis,
    children: Vec<PaneGroupChild>,
}

impl SerializedPaneGroup {
    pub(crate) fn empty_root(_workspace_id: WorkspaceId) -> Self {
        Self {
            // group_id: PaneGroupId::root(workspace_id),
            axis: Default::default(),
            children: Default::default(),
        }
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
            (self.workspace_id, self.anchor_position, self.visible),
            start_index,
        )
    }
}

impl Column for DockRow {
    fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
        <(WorkspaceId, DockAnchor, bool) as Column>::column(statement, start_index).map(
            |((workspace_id, anchor_position, visible), next_index)| {
                (
                    DockRow {
                        workspace_id,
                        anchor_position,
                        visible,
                    },
                    next_index,
                )
            },
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ItemId {
    pub item_id: usize,
}
