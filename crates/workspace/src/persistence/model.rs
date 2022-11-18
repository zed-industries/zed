use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Result;

use async_recursion::async_recursion;
use gpui::{AsyncAppContext, Axis, ModelHandle, Task, ViewHandle};

use project::Project;
use settings::DockAnchor;
use sqlez::{
    bindable::{Bind, Column},
    statement::Statement,
};
use util::ResultExt;

use crate::{
    dock::DockPosition, item::ItemHandle, ItemDeserializers, Member, Pane, PaneAxis, Workspace,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceId(Arc<Vec<PathBuf>>);

impl WorkspaceId {
    pub fn paths(&self) -> Arc<Vec<PathBuf>> {
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
    pub workspace_id: WorkspaceId,
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
        Self::Pane(SerializedPane {
            children: Vec::new(),
        })
    }
}

impl SerializedPaneGroup {
    #[async_recursion(?Send)]
    pub(crate) async fn deserialize(
        &self,
        project: &ModelHandle<Project>,
        workspace_id: &WorkspaceId,
        workspace: &ViewHandle<Workspace>,
        cx: &mut AsyncAppContext,
    ) -> Member {
        match self {
            SerializedPaneGroup::Group { axis, children } => {
                let mut members = Vec::new();
                for child in children {
                    let new_member = child
                        .deserialize(project, workspace_id, workspace, cx)
                        .await;
                    members.push(new_member);
                }
                Member::Axis(PaneAxis {
                    axis: *axis,
                    members,
                })
            }
            SerializedPaneGroup::Pane(serialized_pane) => {
                let pane = workspace.update(cx, |workspace, cx| workspace.add_pane(cx));
                serialized_pane
                    .deserialize_to(project, &pane, workspace_id, workspace, cx)
                    .await;
                Member::Pane(pane)
            }
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

    pub async fn deserialize_to(
        &self,
        project: &ModelHandle<Project>,
        pane_handle: &ViewHandle<Pane>,
        workspace_id: &WorkspaceId,
        workspace: &ViewHandle<Workspace>,
        cx: &mut AsyncAppContext,
    ) {
        for item in self.children.iter() {
            let project = project.clone();
            let workspace_id = workspace_id.clone();
            let item_handle = pane_handle
                .update(cx, |_, cx| {
                    if let Some(deserializer) = cx.global::<ItemDeserializers>().get(&item.kind) {
                        deserializer(
                            project,
                            workspace.downgrade(),
                            workspace_id,
                            item.item_id,
                            cx,
                        )
                    } else {
                        Task::ready(Err(anyhow::anyhow!(
                            "Deserializer does not exist for item kind: {}",
                            item.kind
                        )))
                    }
                })
                .await
                .log_err();
            if let Some(item_handle) = item_handle {
                workspace.update(cx, |workspace, cx| {
                    Pane::add_item(workspace, &pane_handle, item_handle, false, false, None, cx);
                })
            }
        }
    }
}

pub type GroupId = i64;
pub type PaneId = i64;
pub type ItemId = usize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SerializedItem {
    pub kind: Arc<str>,
    pub item_id: ItemId,
}

impl SerializedItem {
    pub fn new(kind: impl AsRef<str>, item_id: ItemId) -> Self {
        Self {
            kind: Arc::from(kind.as_ref()),
            item_id,
        }
    }
}

impl Bind for &SerializedItem {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let next_index = statement.bind(self.kind.clone(), start_index)?;
        statement.bind(self.item_id, next_index)
    }
}

impl Column for SerializedItem {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (kind, next_index) = Arc::<str>::column(statement, start_index)?;
        let (item_id, next_index) = ItemId::column(statement, next_index)?;
        Ok((SerializedItem { kind, item_id }, next_index))
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
        let db = Connection::open_memory(Some("workspace_id_round_trips"));

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
