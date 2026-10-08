use super::{SerializedAxis, SerializedWindowBounds};
use crate::{
    Member, Pane, PaneAxis, SerializableItemRegistry, Workspace, WorkspaceId, item::ItemHandle,
    multi_workspace::SerializedProjectGroupState, path_list::PathList,
};
use anyhow::{Context, Result};
use async_recursion::async_recursion;
use collections::IndexSet;
use db::sqlez::{
    bindable::{Bind, Column, StaticColumnCount},
    statement::Statement,
};
use gpui::{AsyncWindowContext, Entity, WeakEntity, WindowId};

use language::{Toolchain, ToolchainScope};
use project::{
    Project, ProjectGroupKey, bookmark_store::SerializedBookmark,
    debugger::breakpoint_store::SourceBreakpoint,
};
use remote::RemoteConnectionOptions;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{ResultExt, path_list::SerializedPathList};
use uuid::Uuid;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Serialize, serde::Deserialize,
)]
pub(crate) struct RemoteConnectionId(pub u64);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum RemoteConnectionKind {
    Ssh,
    Wsl,
    Docker,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum SerializedWorkspaceLocation {
    Local,
    Remote(RemoteConnectionOptions),
}

impl SerializedWorkspaceLocation {
    /// Get sorted paths
    pub fn sorted_paths(&self) -> Arc<Vec<PathBuf>> {
        unimplemented!()
    }
}

/// A workspace entry from a previous session, containing all the info needed
/// to restore it including which window it belonged to (for MultiWorkspace grouping).
#[derive(Debug, PartialEq, Clone)]
pub struct SessionWorkspace {
    pub workspace_id: WorkspaceId,
    pub location: SerializedWorkspaceLocation,
    pub paths: PathList,
    pub window_id: Option<WindowId>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SerializedProjectGroup {
    pub path_list: SerializedPathList,
    pub(crate) location: SerializedWorkspaceLocation,
    #[serde(default = "default_expanded")]
    pub expanded: bool,
}

fn default_expanded() -> bool {
    true
}

impl SerializedProjectGroup {
    pub fn from_group(key: &ProjectGroupKey, expanded: bool) -> Self {
        Self {
            path_list: key.path_list().serialize(),
            location: match key.host() {
                Some(host) => SerializedWorkspaceLocation::Remote(host),
                None => SerializedWorkspaceLocation::Local,
            },
            expanded,
        }
    }

    pub fn into_restored_state(self) -> SerializedProjectGroupState {
        let path_list = PathList::deserialize(&self.path_list);
        let host = match self.location {
            SerializedWorkspaceLocation::Local => None,
            SerializedWorkspaceLocation::Remote(opts) => Some(opts),
        };
        SerializedProjectGroupState {
            key: ProjectGroupKey::new(host, path_list),
            expanded: self.expanded,
        }
    }
}

impl From<SerializedProjectGroup> for ProjectGroupKey {
    fn from(value: SerializedProjectGroup) -> Self {
        value.into_restored_state().key
    }
}

/// Per-window state for a MultiWorkspace, persisted to KVP.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MultiWorkspaceState {
    pub active_workspace_id: Option<WorkspaceId>,
    pub sidebar_open: bool,
    #[serde(alias = "project_group_keys")]
    pub project_groups: Vec<SerializedProjectGroup>,
    #[serde(default)]
    pub sidebar_state: Option<String>,
}

/// The serialized state of a single MultiWorkspace window from a previous session:
/// the active workspace to restore plus window-level state (project group keys,
/// sidebar).
#[derive(Debug, Clone)]
pub struct SerializedMultiWorkspace {
    pub active_workspace: SessionWorkspace,
    pub state: MultiWorkspaceState,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct SerializedWorkspace {
    pub(crate) id: WorkspaceId,
    pub(crate) location: SerializedWorkspaceLocation,
    pub(crate) paths: PathList,
    /// The workspace's main worktree paths at the time this workspace was saved.
    ///
    /// These paths are used for grouping, deduping, and display in recent-workspace
    /// UIs. They are not authoritative for reopening the workspace, because they may
    /// become stale if the repository layout changes after the save. Use `paths` when
    /// reopening the workspace.
    pub(crate) identity_paths: Option<PathList>,
    pub(crate) center_group: SerializedPaneGroup,
    pub(crate) window_bounds: Option<SerializedWindowBounds>,
    pub(crate) centered_layout: bool,
    pub(crate) display: Option<Uuid>,
    pub(crate) docks: DockStructure,
    pub(crate) session_id: Option<String>,
    pub(crate) bookmarks: BTreeMap<Arc<Path>, Vec<SerializedBookmark>>,
    pub(crate) breakpoints: BTreeMap<Arc<Path>, Vec<SourceBreakpoint>>,
    pub(crate) user_toolchains: BTreeMap<ToolchainScope, IndexSet<Toolchain>>,
    pub(crate) recent_navigation_history: Vec<PathBuf>,
    pub(crate) window_id: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct DockStructure {
    pub left: DockData,
    pub right: DockData,
    pub bottom: DockData,
}

impl RemoteConnectionKind {
    pub(crate) fn serialize(&self) -> &'static str {
        match self {
            RemoteConnectionKind::Ssh => "ssh",
            RemoteConnectionKind::Wsl => "wsl",
            RemoteConnectionKind::Docker => "docker",
        }
    }

    pub(crate) fn deserialize(text: &str) -> Option<Self> {
        match text {
            "ssh" => Some(Self::Ssh),
            "wsl" => Some(Self::Wsl),
            "docker" => Some(Self::Docker),
            _ => None,
        }
    }
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

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct DockData {
    pub visible: bool,
    pub active_panel: Option<String>,
    pub zoom: bool,
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
pub(crate) enum SerializedPaneGroup {
    Group {
        axis: SerializedAxis,
        flexes: Option<Vec<f32>>,
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
            pinned_count: 0,
        })
    }
}

impl SerializedPaneGroup {
    /// The items of every pane in this group, in pane order.
    pub(crate) fn into_items(self) -> Vec<SerializedItem> {
        match self {
            SerializedPaneGroup::Group { children, .. } => children
                .into_iter()
                .flat_map(SerializedPaneGroup::into_items)
                .collect(),
            SerializedPaneGroup::Pane(pane) => pane.children,
        }
    }

    /// Removes every item the group lists with one of `item_ids`.
    ///
    /// While a restore is in flight two id spaces meet: the layout being restored
    /// names items minted by the process that saved it, and the items the user
    /// opens are minted by this one. An item is persisted as
    /// `(item_id, workspace_id)`, so an id both spaces name can be listed only
    /// once; a listing that is superseded this way has to go.
    pub(crate) fn remove_items(&mut self, item_ids: &[ItemId]) {
        match self {
            SerializedPaneGroup::Group { children, .. } => {
                for child in children {
                    child.remove_items(item_ids);
                }
            }
            SerializedPaneGroup::Pane(pane) => {
                // Pinned tabs are the leading tabs of a pane, so the pinned count
                // has to shrink along with every pinned listing that is dropped
                // here. Otherwise a tab that was not pinned would take the dropped
                // item's slot and come back pinned on the next restore.
                let pinned_region = 0..pane.pinned_count;
                let mut pinned_count = pane.pinned_count;
                pane.children = std::mem::take(&mut pane.children)
                    .into_iter()
                    .enumerate()
                    .filter_map(|(index, item)| {
                        if !item_ids.contains(&item.item_id) {
                            return Some(item);
                        }
                        if pinned_region.contains(&index) {
                            pinned_count -= 1;
                        }
                        None
                    })
                    .collect();
                pane.pinned_count = pinned_count;
            }
        }
    }

    /// Appends `items` to the group's active pane, or to its first pane when no
    /// pane is marked active.
    ///
    /// This is where items a user opened while a restore was in flight belong in
    /// the restored layout: the restore installs them into its active pane, so
    /// the persisted form of that layout has to place them the same way.
    pub(crate) fn mount_items(&mut self, items: Vec<SerializedItem>) {
        if items.is_empty() {
            return;
        }

        let target = match self.active_pane_mut() {
            Some(pane) => Some(pane),
            None => self.first_pane_mut(),
        };
        if let Some(pane) = target {
            pane.children.extend(items);
        }
    }

    fn active_pane_mut(&mut self) -> Option<&mut SerializedPane> {
        match self {
            SerializedPaneGroup::Pane(pane) => pane.active.then_some(pane),
            SerializedPaneGroup::Group { children, .. } => children
                .iter_mut()
                .find_map(|child| child.active_pane_mut()),
        }
    }

    fn first_pane_mut(&mut self) -> Option<&mut SerializedPane> {
        match self {
            SerializedPaneGroup::Pane(pane) => Some(pane),
            SerializedPaneGroup::Group { children, .. } => {
                children.iter_mut().find_map(|child| child.first_pane_mut())
            }
        }
    }

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
            SerializedPaneGroup::Group {
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
            SerializedPaneGroup::Pane(serialized_pane) => {
                let pane = workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace.add_restore_pane(window, cx).downgrade()
                    })
                    .log_err()?;
                let active = serialized_pane.active;
                let new_items = serialized_pane
                    .deserialize_to(project, &pane, workspace_id, workspace.clone(), cx)
                    .await
                    .context("Could not deserialize pane)")
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
pub struct SerializedPane {
    pub(crate) active: bool,
    pub(crate) children: Vec<SerializedItem>,
    pub(crate) pinned_count: usize,
}

impl SerializedPane {
    pub fn new(children: Vec<SerializedItem>, active: bool, pinned_count: usize) -> Self {
        SerializedPane {
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
                    // `activate_pane` is false: the pane is not attached to the
                    // center until the restored layout is installed, so making it
                    // the workspace's active pane would point the workspace -- and
                    // the items the user opens while it restores -- at a pane the
                    // user cannot see. The pane the layout marks active is
                    // activated with the rest of the layout at installation.
                    pane.add_item(item_handle.clone(), false, true, None, window, cx);
                })?;
            }
        }

        if let Some(active_item) = active_item_index.and_then(|index| items.get(index)?.clone()) {
            pane.update_in(cx, |pane, window, cx| {
                if let Some(index) = pane.index_for_item(active_item.as_ref()) {
                    pane.activate_item(index, false, false, window, cx);
                }
            })?;
        }

        if let Some(preview_item) = preview_item_index.and_then(|index| items.get(index)?.clone()) {
            pane.update(cx, |pane, cx| {
                pane.set_preview_item_id(Some(preview_item.item_id()), cx);
            })?;
        }

        // `items` keeps a `None` for every item that failed to deserialize, and those
        // were never added to the pane. Counting them would leave the pinned count
        // pointing past the pinned tabs and pin unpinned ones in their place.
        let pinned_count = items
            .iter()
            .take(self.pinned_count)
            .filter(|item| item.is_some())
            .count();
        pane.update(cx, |pane, _| {
            pane.set_pinned_count(pinned_count);
        })?;

        anyhow::Ok(items)
    }
}

pub type GroupId = i64;
pub type PaneId = i64;
pub type ItemId = u64;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SerializedItem {
    pub kind: Arc<str>,
    pub item_id: ItemId,
    pub active: bool,
    pub preview: bool,
}

impl SerializedItem {
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
impl Default for SerializedItem {
    fn default() -> Self {
        SerializedItem {
            kind: Arc::from("Terminal"),
            item_id: 100000,
            active: false,
            preview: false,
        }
    }
}

impl StaticColumnCount for SerializedItem {
    fn column_count() -> usize {
        4
    }
}
impl Bind for &SerializedItem {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let next_index = statement.bind(&self.kind, start_index)?;
        let next_index = statement.bind(&self.item_id, next_index)?;
        let next_index = statement.bind(&self.active, next_index)?;
        statement.bind(&self.preview, next_index)
    }
}

impl Column for SerializedItem {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (kind, next_index) = Arc::<str>::column(statement, start_index)?;
        let (item_id, next_index) = ItemId::column(statement, next_index)?;
        let (active, next_index) = bool::column(statement, next_index)?;
        let (preview, next_index) = bool::column(statement, next_index)?;
        Ok((
            SerializedItem {
                kind,
                item_id,
                active,
                preview,
            },
            next_index,
        ))
    }
}
