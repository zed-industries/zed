use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{bail, Result};

use gpui::Axis;

use settings::DockAnchor;
use sqlez::{
    bindable::{Bind, Column},
    statement::Statement,
};

use crate::dock::DockPosition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorkspaceId(Arc<Vec<PathBuf>>);

impl WorkspaceId {
    pub fn paths(self) -> Arc<Vec<PathBuf>> {
        self.0.clone()
    }
}

impl<P: AsRef<Path>, T: IntoIterator<Item = P>> From<T> for WorkspaceId {
    fn from(iterator: T) -> Self {
        let mut roots = iterator
            .into_iter()
            .map(|p| p.as_ref().to_path_buf())
            .collect::<Vec<_>>();
        roots.sort();
        Self(Arc::new(roots))
    }
}

impl Bind for &WorkspaceId {
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

#[derive(Debug, PartialEq, Eq)]
pub struct SerializedWorkspace {
    pub dock_position: DockPosition,
    pub center_group: SerializedPaneGroup,
    pub dock_pane: SerializedPane,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SerializedPaneGroup {
    Group {
        axis: Axis,
        children: Vec<SerializedPaneGroup>,
    },
    Pane(SerializedPane),
}

impl Default for SerializedPaneGroup {
    fn default() -> Self {
        Self::Group {
            axis: Axis::Horizontal,
            children: vec![Self::Pane(Default::default())],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SerializedPane {
    pub(crate) children: Vec<SerializedItem>,
}

impl SerializedPane {
    pub fn new(children: Vec<SerializedItem>) -> Self {
        SerializedPane { children }
    }
}

pub type GroupId = i64;
pub type PaneId = i64;
pub type ItemId = usize;

pub(crate) enum SerializedItemKind {
    Editor,
    Diagnostics,
    ProjectSearch,
    Terminal,
}

impl Bind for SerializedItemKind {
    fn bind(&self, statement: &Statement, start_index: i32) -> anyhow::Result<i32> {
        match self {
            SerializedItemKind::Editor => "Editor",
            SerializedItemKind::Diagnostics => "Diagnostics",
            SerializedItemKind::ProjectSearch => "ProjectSearch",
            SerializedItemKind::Terminal => "Terminal",
        }
        .bind(statement, start_index)
    }
}

impl Column for SerializedItemKind {
    fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
        String::column(statement, start_index).and_then(|(kind_text, next_index)| {
            Ok((
                match kind_text.as_ref() {
                    "Editor" => SerializedItemKind::Editor,
                    "Diagnostics" => SerializedItemKind::Diagnostics,
                    "ProjectSearch" => SerializedItemKind::ProjectSearch,
                    "Terminal" => SerializedItemKind::Terminal,
                    _ => bail!("Stored serialized item kind is incorrect"),
                },
                next_index,
            ))
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SerializedItem {
    Editor { item_id: usize, path: Arc<Path> },
    Diagnostics { item_id: usize },
    ProjectSearch { item_id: usize, query: String },
    Terminal { item_id: usize },
}

impl SerializedItem {
    pub fn item_id(&self) -> usize {
        match self {
            SerializedItem::Editor { item_id, .. } => *item_id,
            SerializedItem::Diagnostics { item_id } => *item_id,
            SerializedItem::ProjectSearch { item_id, .. } => *item_id,
            SerializedItem::Terminal { item_id } => *item_id,
        }
    }

    pub(crate) fn kind(&self) -> SerializedItemKind {
        match self {
            SerializedItem::Editor { .. } => SerializedItemKind::Editor,
            SerializedItem::Diagnostics { .. } => SerializedItemKind::Diagnostics,
            SerializedItem::ProjectSearch { .. } => SerializedItemKind::ProjectSearch,
            SerializedItem::Terminal { .. } => SerializedItemKind::Terminal,
        }
    }
}

impl Bind for DockPosition {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let next_index = statement.bind(self.is_visible(), start_index)?;
        statement.bind(self.anchor(), next_index)
    }
}

impl Column for DockPosition {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (visible, next_index) = bool::column(statement, start_index)?;
        let (dock_anchor, next_index) = DockAnchor::column(statement, next_index)?;
        let position = if visible {
            DockPosition::Shown(dock_anchor)
        } else {
            DockPosition::Hidden(dock_anchor)
        };
        Ok((position, next_index))
    }
}

#[cfg(test)]
mod tests {
    use settings::DockAnchor;
    use sqlez::connection::Connection;

    use super::WorkspaceId;

    #[test]
    fn test_workspace_round_trips() {
        let db = Connection::open_memory("workspace_id_round_trips");

        db.exec(indoc::indoc! {"
                CREATE TABLE workspace_id_test(
                workspace_id BLOB,
                dock_anchor TEXT
                );"})
            .unwrap()()
        .unwrap();

        let workspace_id: WorkspaceId = WorkspaceId::from(&["\test2", "\test1"]);

        db.exec_bound("INSERT INTO workspace_id_test(workspace_id, dock_anchor) VALUES (?,?)")
            .unwrap()((&workspace_id, DockAnchor::Bottom))
        .unwrap();

        assert_eq!(
            db.select_row("SELECT workspace_id, dock_anchor FROM workspace_id_test LIMIT 1")
                .unwrap()()
            .unwrap(),
            Some((WorkspaceId::from(&["\test1", "\test2"]), DockAnchor::Bottom))
        );
    }
}
