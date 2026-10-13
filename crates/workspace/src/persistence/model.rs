use super::{SerializedAxis, SerializedWindowBounds, UNAVAILABLE_SESSION_WORKSPACE_RETENTION};
use crate::{
    Member, Pane, PaneAxis, SerializableItemRegistry, Workspace, WorkspaceId, item::ItemHandle,
    multi_workspace::SerializedProjectGroupState, path_list::PathList,
};
use anyhow::Result;
use collections::{HashSet, IndexSet};
use db::sqlez::{
    bindable::{Bind, Column, StaticColumnCount},
    statement::Statement,
};
use gpui::{App, Context, Entity, Task, WeakEntity, Window, WindowId};

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
use util::path_list::SerializedPathList;
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
    pub session_id: Option<String>,
    pub window_id: Option<WindowId>,
}

impl SessionWorkspace {
    pub fn session_window(&self) -> Option<SessionWindow> {
        self.binding().window()
    }

    pub fn binding(&self) -> SessionBinding {
        SessionBinding {
            session_id: self.session_id.clone(),
            window_id: self.window_id.map(|window_id| window_id.as_u64()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SessionWindow {
    pub session_id: String,
    pub window_id: u64,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SessionBinding {
    pub session_id: Option<String>,
    pub window_id: Option<u64>,
}

impl SessionBinding {
    pub fn window(&self) -> Option<SessionWindow> {
        Some(SessionWindow {
            session_id: self.session_id.clone()?,
            window_id: self.window_id?,
        })
    }
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
    pub(crate) unserialized_root_files: Option<UnserializedRootFiles>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub(crate) struct UnserializedRootFiles {
    pub(crate) paths: PathList,
    pub(crate) active: Option<PathBuf>,
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
    pub(crate) fn restorable_panes(&self) -> Vec<&SerializedPane> {
        match self {
            SerializedPaneGroup::Group { children, .. } => children
                .iter()
                .flat_map(SerializedPaneGroup::restorable_panes)
                .collect(),
            SerializedPaneGroup::Pane(serialized_pane) => {
                if serialized_pane.children.is_empty() {
                    Vec::new()
                } else {
                    vec![serialized_pane]
                }
            }
        }
    }

    pub(crate) fn build_restored_member(
        &self,
        mut pane_for_leaf: impl FnMut(&SerializedPane) -> Entity<Pane>,
    ) -> Option<Member> {
        self.build_member(&mut pane_for_leaf)
    }

    fn build_member(
        &self,
        pane_for_leaf: &mut dyn FnMut(&SerializedPane) -> Entity<Pane>,
    ) -> Option<Member> {
        match self {
            SerializedPaneGroup::Group {
                axis,
                children,
                flexes,
            } => {
                let mut members = children
                    .iter()
                    .filter_map(|child| child.build_member(pane_for_leaf))
                    .collect::<Vec<_>>();
                match members.len() {
                    0 => None,
                    1 => members.pop(),
                    _ => Some(Member::Axis(PaneAxis::load(
                        axis.0,
                        members,
                        flexes.clone(),
                    ))),
                }
            }
            SerializedPaneGroup::Pane(serialized_pane) => {
                if serialized_pane.children.is_empty() {
                    None
                } else {
                    Some(Member::Pane(pane_for_leaf(serialized_pane)))
                }
            }
        }
    }

    pub(crate) fn item_count(&self) -> usize {
        match self {
            SerializedPaneGroup::Group { children, .. } => {
                children.iter().map(SerializedPaneGroup::item_count).sum()
            }
            SerializedPaneGroup::Pane(serialized_pane) => serialized_pane.children.len(),
        }
    }

    pub(crate) fn item_keys(&self) -> HashSet<(Arc<str>, ItemId)> {
        match self {
            SerializedPaneGroup::Group { children, .. } => children
                .iter()
                .flat_map(SerializedPaneGroup::item_keys)
                .collect(),
            SerializedPaneGroup::Pane(serialized_pane) => serialized_pane
                .children
                .iter()
                .map(SerializedItem::key)
                .collect(),
        }
    }

    pub(crate) fn item_ids(&self) -> Vec<ItemId> {
        match self {
            SerializedPaneGroup::Group { children, .. } => children
                .iter()
                .flat_map(SerializedPaneGroup::item_ids)
                .collect(),
            SerializedPaneGroup::Pane(serialized_pane) => serialized_pane
                .children
                .iter()
                .map(|item| item.item_id)
                .collect(),
        }
    }

    pub(crate) fn append_unrestored_items(&mut self, live_items: Vec<SerializedItem>) {
        let saved_keys = self.item_keys();
        let items = live_items
            .into_iter()
            .filter(|item| !saved_keys.contains(&item.key()))
            .collect::<Vec<_>>();
        if items.is_empty() {
            return;
        }
        let has_active_pane = self.has_active_pane();
        let Some(pane) = self.first_pane_mut(has_active_pane) else {
            return;
        };
        if items.iter().any(|item| item.active) {
            for child in &mut pane.children {
                child.active = false;
            }
        }
        pane.children.extend(items);
    }

    fn has_active_pane(&self) -> bool {
        match self {
            SerializedPaneGroup::Group { children, .. } => {
                children.iter().any(SerializedPaneGroup::has_active_pane)
            }
            SerializedPaneGroup::Pane(serialized_pane) => serialized_pane.active,
        }
    }

    fn first_pane_mut(&mut self, active_only: bool) -> Option<&mut SerializedPane> {
        match self {
            SerializedPaneGroup::Group { children, .. } => children
                .iter_mut()
                .find_map(|child| child.first_pane_mut(active_only)),
            SerializedPaneGroup::Pane(serialized_pane) => {
                (!active_only || serialized_pane.active).then_some(serialized_pane)
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

    pub(crate) fn deserialize_items(
        &self,
        project: &Entity<Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<Task<Result<Box<dyn ItemHandle>>>> {
        self.children
            .iter()
            .map(|item| {
                SerializableItemRegistry::deserialize(
                    &item.kind,
                    project.clone(),
                    workspace.clone(),
                    workspace_id,
                    item.item_id,
                    window,
                    cx,
                )
            })
            .collect()
    }

    pub(crate) fn place_restored_items(
        &self,
        pane: &mut Pane,
        items: &[Option<Box<dyn ItemHandle>>],
        window: &mut Window,
        cx: &mut Context<Pane>,
    ) -> Vec<Option<Box<dyn ItemHandle>>> {
        let pane_had_items = pane.items_len() > 0;
        let first_index = pane.pinned_count();
        let mut next_index = first_index;
        let mut restored_pinned_count = 0;
        let mut active_item = None;
        let mut preview_item = None;
        let mut placed_items = Vec::with_capacity(items.len());
        for (child_index, (serialized_item, item)) in self.children.iter().zip(items).enumerate() {
            let Some(item) = item else {
                placed_items.push(None);
                continue;
            };
            let (item, inserted) = if pane_had_items {
                pane.insert_restored_item(item.boxed_clone(), next_index, cx)
            } else {
                pane.add_item_inner(
                    item.boxed_clone(),
                    false,
                    false,
                    true,
                    Some(next_index),
                    window,
                    cx,
                );
                (item.boxed_clone(), true)
            };
            if inserted {
                next_index += 1;
                if child_index < self.pinned_count {
                    restored_pinned_count += 1;
                }
                if serialized_item.preview {
                    preview_item = Some(item.item_id());
                }
            }
            if serialized_item.active {
                active_item = Some(item.boxed_clone());
            }
            placed_items.push(Some(item));
        }

        if !pane_had_items
            && let Some(index) = active_item
                .as_ref()
                .and_then(|item| pane.index_for_item(item.as_ref()))
        {
            pane.activate_item(index, false, false, window, cx);
        }
        if let Some(preview_item) = preview_item
            && (!pane_had_items || pane.preview_item_id().is_none())
        {
            pane.set_preview_item_id(Some(preview_item), cx);
        }
        pane.set_pinned_count((first_index + restored_pinned_count).min(pane.items_len()));
        placed_items
    }

    pub(crate) fn unrestored(
        &self,
        items: &[Option<Box<dyn ItemHandle>>],
        now: i64,
    ) -> Option<SerializedPane> {
        let mut pinned_count = 0;
        let children = self
            .children
            .iter()
            .zip(items)
            .enumerate()
            .filter(|(_, (_, item))| item.is_none())
            .filter_map(|(index, (serialized_item, _))| {
                let unrestored_since = serialized_item.unrestored_since.unwrap_or(now);
                if now - unrestored_since >= UNAVAILABLE_SESSION_WORKSPACE_RETENTION.num_seconds() {
                    return None;
                }
                if index < self.pinned_count {
                    pinned_count += 1;
                }
                Some(SerializedItem {
                    unrestored_since: Some(unrestored_since),
                    ..serialized_item.clone()
                })
            })
            .collect::<Vec<_>>();
        (!children.is_empty()).then(|| SerializedPane::new(children, self.active, pinned_count))
    }

    pub(crate) fn merge_pending(&mut self, pending: &SerializedPane) {
        let live_keys = self
            .children
            .iter()
            .map(SerializedItem::key)
            .collect::<HashSet<_>>();
        let live_has_active = self.children.iter().any(|item| item.active);
        let live_has_preview = self.children.iter().any(|item| item.preview);
        let insertion_index = self.pinned_count.min(self.children.len());
        let mut pending_pinned_count = 0;
        let pending_items = pending
            .children
            .iter()
            .enumerate()
            .filter(|(_, item)| !live_keys.contains(&item.key()))
            .map(|(index, item)| {
                if index < pending.pinned_count {
                    pending_pinned_count += 1;
                }
                SerializedItem {
                    active: item.active && !live_has_active,
                    preview: item.preview && !live_has_preview,
                    ..item.clone()
                }
            })
            .collect::<Vec<_>>();
        self.children
            .splice(insertion_index..insertion_index, pending_items);
        self.pinned_count = insertion_index + pending_pinned_count;
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
    pub(crate) unrestored_since: Option<i64>,
}

impl SerializedItem {
    pub fn new(kind: impl AsRef<str>, item_id: ItemId, active: bool, preview: bool) -> Self {
        Self {
            kind: Arc::from(kind.as_ref()),
            item_id,
            active,
            preview,
            unrestored_since: None,
        }
    }

    pub(crate) fn key(&self) -> (Arc<str>, ItemId) {
        (self.kind.clone(), self.item_id)
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
            unrestored_since: None,
        }
    }
}

impl StaticColumnCount for SerializedItem {
    fn column_count() -> usize {
        5
    }
}
impl Bind for &SerializedItem {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let next_index = statement.bind(&self.kind, start_index)?;
        let next_index = statement.bind(&self.item_id, next_index)?;
        let next_index = statement.bind(&self.active, next_index)?;
        let next_index = statement.bind(&self.preview, next_index)?;
        statement.bind(&self.unrestored_since, next_index)
    }
}

impl Column for SerializedItem {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (kind, next_index) = Arc::<str>::column(statement, start_index)?;
        let (item_id, next_index) = ItemId::column(statement, next_index)?;
        let (active, next_index) = bool::column(statement, next_index)?;
        let (preview, next_index) = bool::column(statement, next_index)?;
        let (unrestored_since, next_index) = Option::<i64>::column(statement, next_index)?;
        Ok((
            SerializedItem {
                kind,
                item_id,
                active,
                preview,
                unrestored_since,
            },
            next_index,
        ))
    }
}
