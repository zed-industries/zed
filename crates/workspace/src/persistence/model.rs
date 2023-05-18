use crate::{ItemDeserializers, Member, Pane, PaneAxis, Workspace, WorkspaceId};
use anyhow::{anyhow, Context, Result};
use async_recursion::async_recursion;
use db::sqlez::{
    bindable::{Bind, Column, StaticColumnCount},
    statement::Statement,
};
use gpui::{
    platform::WindowBounds, AsyncAppContext, Axis, ModelHandle, Task, ViewHandle, WeakViewHandle,
};
use project::Project;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use util::ResultExt;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceLocation(Arc<Vec<PathBuf>>);

impl WorkspaceLocation {
    pub fn paths(&self) -> Arc<Vec<PathBuf>> {
        self.0.clone()
    }
}

impl<P: AsRef<Path>, T: IntoIterator<Item = P>> From<T> for WorkspaceLocation {
    fn from(iterator: T) -> Self {
        let mut roots = iterator
            .into_iter()
            .map(|p| p.as_ref().to_path_buf())
            .collect::<Vec<_>>();
        roots.sort();
        Self(Arc::new(roots))
    }
}

impl StaticColumnCount for WorkspaceLocation {}
impl Bind for &WorkspaceLocation {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        bincode::serialize(&self.0)
            .expect("Bincode serialization of paths should not fail")
            .bind(statement, start_index)
    }
}

impl Column for WorkspaceLocation {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let blob = statement.column_blob(start_index)?;
        Ok((
            WorkspaceLocation(bincode::deserialize(blob).context("Bincode failed")?),
            start_index + 1,
        ))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SerializedWorkspace {
    pub id: WorkspaceId,
    pub location: WorkspaceLocation,
    pub center_group: SerializedPaneGroup,
    pub left_sidebar_open: bool,
    pub bounds: Option<WindowBounds>,
    pub display: Option<Uuid>,
    pub docks: DockStructure,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct DockStructure {
    pub(crate) left: DockData,
    pub(crate) right: DockData,
    pub(crate) bottom: DockData,
}

impl Column for DockStructure {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (left, next_index) = DockData::column(statement, start_index)?;
        let (right, next_index) = DockData::column(statement, next_index)?;
        let (bottom, next_index) = DockData::column(statement, next_index)?;
        Ok((
            DockStructure {
                left,
                right,
                bottom,
            },
            next_index,
        ))
    }
}

impl Bind for DockStructure {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let next_index = statement.bind(&self.left, start_index)?;
        let next_index = statement.bind(&self.right, next_index)?;
        statement.bind(&self.bottom, next_index)
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct DockData {
    pub(crate) visible: bool,
    pub(crate) size: Option<f32>,
}

impl Column for DockData {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (visible, next_index) = Option::<bool>::column(statement, start_index)?;
        let (size, next_index) = Option::<f32>::column(statement, next_index)?;
        Ok((
            DockData {
                visible: visible.unwrap_or(false),
                size,
            },
            next_index,
        ))
    }
}

impl Bind for DockData {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let next_index = statement.bind(&self.visible, start_index)?;
        statement.bind(&self.size, next_index)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SerializedPaneGroup {
    Group {
        axis: Axis,
        children: Vec<SerializedPaneGroup>,
    },
    Pane(SerializedPane),
}

#[cfg(test)]
impl Default for SerializedPaneGroup {
    fn default() -> Self {
        Self::Pane(SerializedPane {
            children: vec![SerializedItem::default()],
            active: false,
        })
    }
}

impl SerializedPaneGroup {
    #[async_recursion(?Send)]
    pub(crate) async fn deserialize(
        &self,
        project: &ModelHandle<Project>,
        workspace_id: WorkspaceId,
        workspace: &WeakViewHandle<Workspace>,
        cx: &mut AsyncAppContext,
    ) -> Option<(Member, Option<ViewHandle<Pane>>)> {
        match self {
            SerializedPaneGroup::Group { axis, children } => {
                let mut current_active_pane = None;
                let mut members = Vec::new();
                for child in children {
                    if let Some((new_member, active_pane)) = child
                        .deserialize(project, workspace_id, workspace, cx)
                        .await
                    {
                        members.push(new_member);
                        current_active_pane = current_active_pane.or(active_pane);
                    }
                }

                if members.is_empty() {
                    return None;
                }

                if members.len() == 1 {
                    return Some((members.remove(0), current_active_pane));
                }

                Some((
                    Member::Axis(PaneAxis {
                        axis: *axis,
                        members,
                    }),
                    current_active_pane,
                ))
            }
            SerializedPaneGroup::Pane(serialized_pane) => {
                let pane = workspace
                    .update(cx, |workspace, cx| workspace.add_pane(cx).downgrade())
                    .log_err()?;
                let active = serialized_pane.active;
                serialized_pane
                    .deserialize_to(project, &pane, workspace_id, workspace, cx)
                    .await
                    .log_err()?;

                if pane
                    .read_with(cx, |pane, _| pane.items_len() != 0)
                    .log_err()?
                {
                    let pane = pane.upgrade(cx)?;
                    Some((Member::Pane(pane.clone()), active.then(|| pane)))
                } else {
                    let pane = pane.upgrade(cx)?;
                    workspace
                        .update(cx, |workspace, cx| workspace.force_remove_pane(&pane, cx))
                        .log_err()?;
                    None
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SerializedPane {
    pub(crate) active: bool,
    pub(crate) children: Vec<SerializedItem>,
}

impl SerializedPane {
    pub fn new(children: Vec<SerializedItem>, active: bool) -> Self {
        SerializedPane { children, active }
    }

    pub async fn deserialize_to(
        &self,
        project: &ModelHandle<Project>,
        pane_handle: &WeakViewHandle<Pane>,
        workspace_id: WorkspaceId,
        workspace: &WeakViewHandle<Workspace>,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        let mut active_item_index = None;
        for (index, item) in self.children.iter().enumerate() {
            let project = project.clone();
            let item_handle = pane_handle
                .update(cx, |_, cx| {
                    if let Some(deserializer) = cx.global::<ItemDeserializers>().get(&item.kind) {
                        deserializer(project, workspace.clone(), workspace_id, item.item_id, cx)
                    } else {
                        Task::ready(Err(anyhow::anyhow!(
                            "Deserializer does not exist for item kind: {}",
                            item.kind
                        )))
                    }
                })?
                .await
                .log_err();

            if let Some(item_handle) = item_handle {
                workspace.update(cx, |workspace, cx| {
                    let pane_handle = pane_handle
                        .upgrade(cx)
                        .ok_or_else(|| anyhow!("pane was dropped"))?;
                    Pane::add_item(workspace, &pane_handle, item_handle, true, true, None, cx);
                    anyhow::Ok(())
                })??;
            }

            if item.active {
                active_item_index = Some(index);
            }
        }

        if let Some(active_item_index) = active_item_index {
            pane_handle.update(cx, |pane, cx| {
                pane.activate_item(active_item_index, false, false, cx);
            })?;
        }

        anyhow::Ok(())
    }
}

pub type GroupId = i64;
pub type PaneId = i64;
pub type ItemId = usize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SerializedItem {
    pub kind: Arc<str>,
    pub item_id: ItemId,
    pub active: bool,
}

impl SerializedItem {
    pub fn new(kind: impl AsRef<str>, item_id: ItemId, active: bool) -> Self {
        Self {
            kind: Arc::from(kind.as_ref()),
            item_id,
            active,
        }
    }
}

#[cfg(test)]
impl Default for SerializedItem {
    fn default() -> Self {
        SerializedItem {
            kind: Arc::from("Terminal"),
            item_id: 100000,
            active: false,
        }
    }
}

impl StaticColumnCount for SerializedItem {
    fn column_count() -> usize {
        3
    }
}
impl Bind for &SerializedItem {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let next_index = statement.bind(&self.kind, start_index)?;
        let next_index = statement.bind(&self.item_id, next_index)?;
        statement.bind(&self.active, next_index)
    }
}

impl Column for SerializedItem {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (kind, next_index) = Arc::<str>::column(statement, start_index)?;
        let (item_id, next_index) = ItemId::column(statement, next_index)?;
        let (active, next_index) = bool::column(statement, next_index)?;
        Ok((
            SerializedItem {
                kind,
                item_id,
                active,
            },
            next_index,
        ))
    }
}

#[cfg(test)]
mod tests {
    use db::sqlez::connection::Connection;

    // use super::WorkspaceLocation;

    #[test]
    fn test_workspace_round_trips() {
        let _db = Connection::open_memory(Some("workspace_id_round_trips"));

        todo!();
        // db.exec(indoc::indoc! {"
        //         CREATE TABLE workspace_id_test(
        //             workspace_id INTEGER,
        //             dock_anchor TEXT
        //         );"})
        //     .unwrap()()
        // .unwrap();

        // let workspace_id: WorkspaceLocation = WorkspaceLocation::from(&["\test2", "\test1"]);

        // db.exec_bound("INSERT INTO workspace_id_test(workspace_id, dock_anchor) VALUES (?,?)")
        //     .unwrap()((&workspace_id, DockAnchor::Bottom))
        // .unwrap();

        // assert_eq!(
        //     db.select_row("SELECT workspace_id, dock_anchor FROM workspace_id_test LIMIT 1")
        //         .unwrap()()
        //     .unwrap(),
        //     Some((
        //         WorkspaceLocation::from(&["\test1", "\test2"]),
        //         DockAnchor::Bottom
        //     ))
        // );
    }
}
