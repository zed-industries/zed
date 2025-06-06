use super::{SerialiCodeOrbitAxis, SerialiCodeOrbitWindowBounds};
use crate::{
    Member, Pane, PaneAxis, SerializableItemRegistry, Workspace, WorkspaceId, item::ItemHandle,
};
use anyhow::{Context as _, Result};
use async_recursion::async_recursion;
use db::sqlez::{
    bindable::{Bind, Column, StaticColumnCount},
    statement::Statement,
};
use gpui::{AsyncWindowContext, Entity, WeakEntity};
use itertools::Itertools as _;
use project::{Project, debugger::breakpoint_store::SourceBreakpoint};
use remote::ssh_session::SshProjectId;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{ResultExt, paths::SanitiCodeOrbitPath};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SerialiCodeOrbitSshProject {
    pub id: SshProjectId,
    pub host: String,
    pub port: Option<u16>,
    pub paths: Vec<String>,
    pub user: Option<String>,
}

impl SerialiCodeOrbitSshProject {
    pub fn ssh_urls(&self) -> Vec<PathBuf> {
        self.paths
            .iter()
            .map(|path| {
                let mut result = String::new();
                if let Some(user) = &self.user {
                    result.push_str(user);
                    result.push('@');
                }
                result.push_str(&self.host);
                if let Some(port) = &self.port {
                    result.push(':');
                    result.push_str(&port.to_string());
                }
                result.push_str(path);
                PathBuf::from(result)
            })
            .collect()
    }
}

impl StaticColumnCount for SerialiCodeOrbitSshProject {
    fn column_count() -> usize {
        5
    }
}

impl Bind for &SerialiCodeOrbitSshProject {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let next_index = statement.bind(&self.id.0, start_index)?;
        let next_index = statement.bind(&self.host, next_index)?;
        let next_index = statement.bind(&self.port, next_index)?;
        let raw_paths = serde_json::to_string(&self.paths)?;
        let next_index = statement.bind(&raw_paths, next_index)?;
        statement.bind(&self.user, next_index)
    }
}

impl Column for SerialiCodeOrbitSshProject {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let id = statement.column_int64(start_index)?;
        let host = statement.column_text(start_index + 1)?.to_string();
        let (port, _) = Option::<u16>::column(statement, start_index + 2)?;
        let raw_paths = statement.column_text(start_index + 3)?.to_string();
        let paths: Vec<String> = serde_json::from_str(&raw_paths)?;

        let (user, _) = Option::<String>::column(statement, start_index + 4)?;

        Ok((
            Self {
                id: SshProjectId(id as u64),
                host,
                port,
                paths,
                user,
            },
            start_index + 5,
        ))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LocalPaths(Arc<Vec<PathBuf>>);

impl LocalPaths {
    pub fn new<P: AsRef<Path>>(paths: impl IntoIterator<Item = P>) -> Self {
        let mut paths: Vec<PathBuf> = paths
            .into_iter()
            .map(|p| SanitiCodeOrbitPath::from(p).into())
            .collect();
        // Ensure all future `CodeOrbit workspace1 workspace2` and `CodeOrbit workspace2 workspace1` calls are using the same workspace.
        // The actual workspace order is stored in the `LocalPathsOrder` struct.
        paths.sort();
        Self(Arc::new(paths))
    }

    pub fn paths(&self) -> &Arc<Vec<PathBuf>> {
        &self.0
    }
}

impl StaticColumnCount for LocalPaths {}
impl Bind for &LocalPaths {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        statement.bind(&bincode::serialize(&self.0)?, start_index)
    }
}

impl Column for LocalPaths {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let path_blob = statement.column_blob(start_index)?;
        let paths: Arc<Vec<PathBuf>> = if path_blob.is_empty() {
            Default::default()
        } else {
            bincode::deserialize(path_blob).context("Bincode deserialization of paths failed")?
        };

        Ok((Self(paths), start_index + 1))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LocalPathsOrder(Vec<usize>);

impl LocalPathsOrder {
    pub fn new(order: impl IntoIterator<Item = usize>) -> Self {
        Self(order.into_iter().collect())
    }

    pub fn order(&self) -> &[usize] {
        self.0.as_slice()
    }

    pub fn default_for_paths(paths: &LocalPaths) -> Self {
        Self::new(0..paths.0.len())
    }
}

impl StaticColumnCount for LocalPathsOrder {}
impl Bind for &LocalPathsOrder {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        statement.bind(&bincode::serialize(&self.0)?, start_index)
    }
}

impl Column for LocalPathsOrder {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let order_blob = statement.column_blob(start_index)?;
        let order = if order_blob.is_empty() {
            Vec::new()
        } else {
            bincode::deserialize(order_blob).context("deserializing workspace root order")?
        };

        Ok((Self(order), start_index + 1))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SerialiCodeOrbitWorkspaceLocation {
    Local(LocalPaths, LocalPathsOrder),
    Ssh(SerialiCodeOrbitSshProject),
}

impl SerialiCodeOrbitWorkspaceLocation {
    /// Create a new `SerialiCodeOrbitWorkspaceLocation` from a list of local paths.
    ///
    /// The paths will be sorted and the order will be stored in the `LocalPathsOrder` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use codeorbit_workspace::SerialiCodeOrbitWorkspaceLocation;
    ///
    /// let location = SerialiCodeOrbitWorkspaceLocation::from_local_paths(vec![
    ///     Path::new("path/to/workspace1"),
    ///     Path::new("path/to/workspace2"),
    /// ]);
    /// assert_eq!(location, SerialiCodeOrbitWorkspaceLocation::Local(
    ///    LocalPaths::new(vec![
    ///         Path::new("path/to/workspace1"),
    ///         Path::new("path/to/workspace2"),
    ///    ]),
    ///   LocalPathsOrder::new(vec![0, 1]),
    /// ));
    /// ```
    ///
    /// ```
    /// use std::path::Path;
    /// use codeorbit_workspace::SerialiCodeOrbitWorkspaceLocation;
    ///
    /// let location = SerialiCodeOrbitWorkspaceLocation::from_local_paths(vec![
    ///     Path::new("path/to/workspace2"),
    ///     Path::new("path/to/workspace1"),
    /// ]);
    ///
    /// assert_eq!(location, SerialiCodeOrbitWorkspaceLocation::Local(
    ///    LocalPaths::new(vec![
    ///         Path::new("path/to/workspace1"),
    ///         Path::new("path/to/workspace2"),
    ///   ]),
    ///  LocalPathsOrder::new(vec![1, 0]),
    /// ));
    /// ```
    pub fn from_local_paths<P: AsRef<Path>>(paths: impl IntoIterator<Item = P>) -> Self {
        let mut indexed_paths: Vec<_> = paths
            .into_iter()
            .map(|p| p.as_ref().to_path_buf())
            .enumerate()
            .collect();

        indexed_paths.sort_by(|(_, a), (_, b)| a.cmp(b));

        let sorted_paths: Vec<_> = indexed_paths.iter().map(|(_, path)| path.clone()).collect();
        let order: Vec<_> = indexed_paths.iter().map(|(index, _)| *index).collect();

        Self::Local(LocalPaths::new(sorted_paths), LocalPathsOrder::new(order))
    }

    /// Get sorted paths
    pub fn sorted_paths(&self) -> Arc<Vec<PathBuf>> {
        match self {
            SerialiCodeOrbitWorkspaceLocation::Local(paths, order) => {
                if order.order().len() == 0 {
                    paths.paths().clone()
                } else {
                    Arc::new(
                        order
                            .order()
                            .iter()
                            .zip(paths.paths().iter())
                            .sorted_by_key(|(i, _)| **i)
                            .map(|(_, p)| p.clone())
                            .collect(),
                    )
                }
            }
            SerialiCodeOrbitWorkspaceLocation::Ssh(ssh_project) => Arc::new(ssh_project.ssh_urls()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct SerialiCodeOrbitWorkspace {
    pub(crate) id: WorkspaceId,
    pub(crate) location: SerialiCodeOrbitWorkspaceLocation,
    pub(crate) center_group: SerialiCodeOrbitPaneGroup,
    pub(crate) window_bounds: Option<SerialiCodeOrbitWindowBounds>,
    pub(crate) centered_layout: bool,
    pub(crate) display: Option<Uuid>,
    pub(crate) docks: DockStructure,
    pub(crate) session_id: Option<String>,
    pub(crate) breakpoints: BTreeMap<Arc<Path>, Vec<SourceBreakpoint>>,
    pub(crate) window_id: Option<u64>,
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
    pub(crate) active_panel: Option<String>,
    pub(crate) zoom: bool,
}

impl Column for DockData {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (visible, next_index) = Option::<bool>::column(statement, start_index)?;
        let (active_panel, next_index) = Option::<String>::column(statement, next_index)?;
        let (zoom, next_index) = Option::<bool>::column(statement, next_index)?;
        Ok((
            DockData {
                visible: visible.unwrap_or(false),
                active_panel,
                zoom: zoom.unwrap_or(false),
            },
            next_index,
        ))
    }
}

impl Bind for DockData {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let next_index = statement.bind(&self.visible, start_index)?;
        let next_index = statement.bind(&self.active_panel, next_index)?;
        statement.bind(&self.zoom, next_index)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum SerialiCodeOrbitPaneGroup {
    Group {
        axis: SerialiCodeOrbitAxis,
        flexes: Option<Vec<f32>>,
        children: Vec<SerialiCodeOrbitPaneGroup>,
    },
    Pane(SerialiCodeOrbitPane),
}

#[cfg(test)]
impl Default for SerialiCodeOrbitPaneGroup {
    fn default() -> Self {
        Self::Pane(SerialiCodeOrbitPane {
            children: vec![Serialicodeorbit-editem::default()],
            active: false,
            pinned_count: 0,
        })
    }
}

impl SerialiCodeOrbitPaneGroup {
    #[async_recursion(?Send)]
    pub(crate) async fn deserialize(
        self,
        project: &Entity<Project>,
        workspace_id: WorkspaceId,
        workspace: WeakEntity<Workspace>,
        cx: &mut AsyncWindowContext,
    ) -> Option<(
        Member,
        Option<Entity<Pane>>,
        Vec<Option<Box<dyn ItemHandle>>>,
    )> {
        match self {
            SerialiCodeOrbitPaneGroup::Group {
                axis,
                children,
                flexes,
            } => {
                let mut current_active_pane = None;
                let mut members = Vec::new();
                let mut items = Vec::new();
                for child in children {
                    if let Some((new_member, active_pane, new_items)) = child
                        .deserialize(project, workspace_id, workspace.clone(), cx)
                        .await
                    {
                        members.push(new_member);
                        items.extend(new_items);
                        current_active_pane = current_active_pane.or(active_pane);
                    }
                }

                if members.is_empty() {
                    return None;
                }

                if members.len() == 1 {
                    return Some((members.remove(0), current_active_pane, items));
                }

                Some((
                    Member::Axis(PaneAxis::load(axis.0, members, flexes)),
                    current_active_pane,
                    items,
                ))
            }
            SerialiCodeOrbitPaneGroup::Pane(serialiCodeOrbit_pane) => {
                let pane = workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace.add_pane(window, cx).downgrade()
                    })
                    .log_err()?;
                let active = serialiCodeOrbit_pane.active;
                let new_items = serialiCodeOrbit_pane
                    .deserialize_to(project, &pane, workspace_id, workspace.clone(), cx)
                    .await
                    .log_err()?;

                if pane
                    .read_with(cx, |pane, _| pane.items_len() != 0)
                    .log_err()?
                {
                    let pane = pane.upgrade()?;
                    Some((
                        Member::Pane(pane.clone()),
                        active.then_some(pane),
                        new_items,
                    ))
                } else {
                    let pane = pane.upgrade()?;
                    workspace
                        .update_in(cx, |workspace, window, cx| {
                            workspace.force_remove_pane(&pane, &None, window, cx)
                        })
                        .log_err()?;
                    None
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SerialiCodeOrbitPane {
    pub(crate) active: bool,
    pub(crate) children: Vec<Serialicodeorbit-editem>,
    pub(crate) pinned_count: usize,
}

impl SerialiCodeOrbitPane {
    pub fn new(children: Vec<Serialicodeorbit-editem>, active: bool, pinned_count: usize) -> Self {
        SerialiCodeOrbitPane {
            children,
            active,
            pinned_count,
        }
    }

    pub async fn deserialize_to(
        &self,
        project: &Entity<Project>,
        pane: &WeakEntity<Pane>,
        workspace_id: WorkspaceId,
        workspace: WeakEntity<Workspace>,
        cx: &mut AsyncWindowContext,
    ) -> Result<Vec<Option<Box<dyn ItemHandle>>>> {
        let mut item_tasks = Vec::new();
        let mut active_item_index = None;
        let mut preview_item_index = None;
        for (index, item) in self.children.iter().enumerate() {
            let project = project.clone();
            item_tasks.push(pane.update_in(cx, |_, window, cx| {
                SerializableItemRegistry::deserialize(
                    &item.kind,
                    project,
                    workspace.clone(),
                    workspace_id,
                    item.item_id,
                    window,
                    cx,
                )
            })?);
            if item.active {
                active_item_index = Some(index);
            }
            if item.preview {
                preview_item_index = Some(index);
            }
        }

        let mut items = Vec::new();
        for item_handle in futures::future::join_all(item_tasks).await {
            let item_handle = item_handle.log_err();
            items.push(item_handle.clone());

            if let Some(item_handle) = item_handle {
                pane.update_in(cx, |pane, window, cx| {
                    pane.add_item(item_handle.clone(), true, true, None, window, cx);
                })?;
            }
        }

        if let Some(active_item_index) = active_item_index {
            pane.update_in(cx, |pane, window, cx| {
                pane.activate_item(active_item_index, false, false, window, cx);
            })?;
        }

        if let Some(preview_item_index) = preview_item_index {
            pane.update(cx, |pane, cx| {
                if let Some(item) = pane.item_for_index(preview_item_index) {
                    pane.set_preview_item_id(Some(item.item_id()), cx);
                }
            })?;
        }
        pane.update(cx, |pane, _| {
            pane.set_pinned_count(self.pinned_count.min(items.len()));
        })?;

        anyhow::Ok(items)
    }
}

pub type GroupId = i64;
pub type PaneId = i64;
pub type ItemId = u64;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Serialicodeorbit-editem {
    pub kind: Arc<str>,
    pub item_id: ItemId,
    pub active: bool,
    pub preview: bool,
}

impl Serialicodeorbit-editem {
    pub fn new(kind: impl AsRef<str>, item_id: ItemId, active: bool, preview: bool) -> Self {
        Self {
            kind: Arc::from(kind.as_ref()),
            item_id,
            active,
            preview,
        }
    }
}

#[cfg(test)]
impl Default for Serialicodeorbit-editem {
    fn default() -> Self {
        Serialicodeorbit-editem {
            kind: Arc::from("Terminal"),
            item_id: 100000,
            active: false,
            preview: false,
        }
    }
}

impl StaticColumnCount for Serialicodeorbit-editem {
    fn column_count() -> usize {
        4
    }
}
impl Bind for &Serialicodeorbit-editem {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let next_index = statement.bind(&self.kind, start_index)?;
        let next_index = statement.bind(&self.item_id, next_index)?;
        let next_index = statement.bind(&self.active, next_index)?;
        statement.bind(&self.preview, next_index)
    }
}

impl Column for Serialicodeorbit-editem {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (kind, next_index) = Arc::<str>::column(statement, start_index)?;
        let (item_id, next_index) = ItemId::column(statement, next_index)?;
        let (active, next_index) = bool::column(statement, next_index)?;
        let (preview, next_index) = bool::column(statement, next_index)?;
        Ok((
            Serialicodeorbit-editem {
                kind,
                item_id,
                active,
                preview,
            },
            next_index,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_local_paths() {
        let paths = vec!["b", "a", "c"];
        let serialiCodeOrbit = SerialiCodeOrbitWorkspaceLocation::from_local_paths(paths);

        assert_eq!(
            serialiCodeOrbit,
            SerialiCodeOrbitWorkspaceLocation::Local(
                LocalPaths::new(vec!["a", "b", "c"]),
                LocalPathsOrder::new(vec![1, 0, 2])
            )
        );
    }

    #[test]
    fn test_sorted_paths() {
        let paths = vec!["b", "a", "c"];
        let serialiCodeOrbit = SerialiCodeOrbitWorkspaceLocation::from_local_paths(paths);
        assert_eq!(
            serialiCodeOrbit.sorted_paths(),
            Arc::new(vec![
                PathBuf::from("b"),
                PathBuf::from("a"),
                PathBuf::from("c"),
            ])
        );

        let paths = Arc::new(vec![
            PathBuf::from("a"),
            PathBuf::from("b"),
            PathBuf::from("c"),
        ]);
        let order = vec![2, 0, 1];
        let serialiCodeOrbit =
            SerialiCodeOrbitWorkspaceLocation::Local(LocalPaths(paths.clone()), LocalPathsOrder(order));
        assert_eq!(
            serialiCodeOrbit.sorted_paths(),
            Arc::new(vec![
                PathBuf::from("b"),
                PathBuf::from("c"),
                PathBuf::from("a"),
            ])
        );

        let paths = Arc::new(vec![
            PathBuf::from("a"),
            PathBuf::from("b"),
            PathBuf::from("c"),
        ]);
        let order = vec![];
        let serialiCodeOrbit =
            SerialiCodeOrbitWorkspaceLocation::Local(LocalPaths(paths.clone()), LocalPathsOrder(order));
        assert_eq!(serialiCodeOrbit.sorted_paths(), paths);

        let urls = ["/a", "/b", "/c"];
        let serialiCodeOrbit = SerialiCodeOrbitWorkspaceLocation::Ssh(SerialiCodeOrbitSshProject {
            id: SshProjectId(0),
            host: "host".to_string(),
            port: Some(22),
            paths: urls.iter().map(|s| s.to_string()).collect(),
            user: Some("user".to_string()),
        });
        assert_eq!(
            serialiCodeOrbit.sorted_paths(),
            Arc::new(
                urls.iter()
                    .map(|p| PathBuf::from(format!("user@host:22{}", p)))
                    .collect()
            )
        );
    }
}
