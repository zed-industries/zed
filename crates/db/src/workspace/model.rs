use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

use gpui::Axis;
use sqlez::{
    bindable::{Bind, Column},
    statement::Statement,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct WorkspaceId(Vec<PathBuf>);

impl<P: AsRef<Path>, T: IntoIterator<Item = P>> From<T> for WorkspaceId {
    fn from(iterator: T) -> Self {
        let mut roots = iterator
            .into_iter()
            .map(|p| p.as_ref().to_path_buf())
            .collect::<Vec<_>>();
        roots.sort();
        Self(roots)
    }
}

impl Bind for WorkspaceId {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        bincode::serialize(&self.0)
            .expect("Bincode serialization of paths should not fail")
            .bind(statement, start_index)
    }
}

impl Column for WorkspaceId {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let blob = statement.column_blob(start_index)?;
        Ok((WorkspaceId(bincode::deserialize(blob)?), start_index + 1))
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

#[derive(Debug)]
pub struct SerializedWorkspace {
    pub dock_anchor: DockAnchor,
    pub dock_visible: bool,
    pub center_group: SerializedPaneGroup,
    pub dock_pane: SerializedPane,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SerializedPaneGroup {
    axis: Axis,
    children: Vec<PaneGroupChild>,
}

pub struct SerializedPane {
    children: Vec<SerializedItem>,
}

pub enum SerializedItemKind {}

pub enum SerializedItem {}
