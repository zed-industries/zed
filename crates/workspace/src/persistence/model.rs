use super::{SerializedAxis, SerializedWindowBounds};
use crate::{
    Member, Pane, PaneAxis, SerializableItemRegistry, Workspace, WorkspaceId,
    invalid_item_view::RestoringItem, item::ItemHandle,
    multi_workspace::SerializedProjectGroupState, path_list::PathList,
};
#[cfg(test)]
use anyhow::Context as _;
use anyhow::Result;
use collections::{HashMap, IndexSet};
use db::sqlez::{
    bindable::{Bind, Column, StaticColumnCount},
    statement::Statement,
};
#[cfg(test)]
use gpui::AsyncWindowContext;
use gpui::{
    App, AppContext as _, Context, Entity, EntityId, SharedString, Task, WeakEntity, Window,
    WindowId,
};

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
    sync::{Arc, atomic::Ordering},
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
    #[cfg(any(test, feature = "test-support"))]
    Mock,
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

impl MultiWorkspaceState {
    pub(super) fn restore_project_groups(
        &mut self,
        previous: &Self,
        removed_groups: &[ProjectGroupKey],
    ) {
        let mut insertion_index = self.project_groups.len();
        for group in previous.project_groups.iter().rev() {
            let key = ProjectGroupKey::from(group.clone());
            if let Some(index) = self
                .project_groups
                .iter()
                .position(|current| ProjectGroupKey::from(current.clone()) == key)
            {
                insertion_index = index;
            } else if removed_groups.contains(&key) {
                self.project_groups.insert(insertion_index, group.clone());
            }
        }
    }
}

/// The serialized state of a single MultiWorkspace window from a previous session:
/// the active workspace to restore plus window-level state (project group keys,
/// sidebar).
#[derive(Debug, Clone)]
pub struct SerializedMultiWorkspace {
    pub active_workspace: SessionWorkspace,
    pub workspace_ids: Vec<WorkspaceId>,
    pub scratch_workspaces: Vec<WorkspaceId>,
    pub state: MultiWorkspaceState,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct WorkspaceRestoreCheckpoint {
    active_workspace_id: WorkspaceId,
    pub(crate) workspace_ids: Vec<WorkspaceId>,
    pub(crate) order: usize,
    location: SerializedWorkspaceLocation,
    paths: SerializedPathList,
    window_id: Option<u64>,
    scratch_workspaces: Vec<WorkspaceId>,
    state: MultiWorkspaceState,
}

impl WorkspaceRestoreCheckpoint {
    pub(crate) fn new(workspace: &SerializedMultiWorkspace, order: usize) -> Self {
        Self {
            active_workspace_id: workspace.active_workspace.workspace_id,
            workspace_ids: workspace.workspace_ids.clone(),
            order,
            location: workspace.active_workspace.location.clone(),
            paths: workspace.active_workspace.paths.serialize(),
            window_id: workspace.active_workspace.window_id.map(|id| id.as_u64()),
            scratch_workspaces: workspace.scratch_workspaces.clone(),
            state: workspace.state.clone(),
        }
    }

    pub(crate) fn remove_members(
        &mut self,
        workspace_ids: &[WorkspaceId],
        groups: &[ProjectGroupKey],
    ) {
        self.workspace_ids.retain(|id| !workspace_ids.contains(id));
        self.scratch_workspaces
            .retain(|id| self.workspace_ids.contains(id));
        self.state
            .project_groups
            .retain(|group| !groups.contains(&ProjectGroupKey::from(group.clone())));
        if self
            .state
            .active_workspace_id
            .is_some_and(|id| !self.workspace_ids.contains(&id))
        {
            self.state.active_workspace_id = None;
        }
    }

    pub(crate) fn into_workspace(self) -> SerializedMultiWorkspace {
        SerializedMultiWorkspace {
            active_workspace: SessionWorkspace {
                workspace_id: self.active_workspace_id,
                location: self.location,
                paths: PathList::deserialize(&self.paths),
                window_id: self.window_id.map(WindowId::from),
            },
            workspace_ids: self.workspace_ids,
            scratch_workspaces: self.scratch_workspaces,
            state: self.state,
        }
    }
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
            #[cfg(any(test, feature = "test-support"))]
            RemoteConnectionKind::Mock => "mock",
        }
    }

    pub(crate) fn deserialize(text: &str) -> Option<Self> {
        match text {
            "ssh" => Some(Self::Ssh),
            "wsl" => Some(Self::Wsl),
            "docker" => Some(Self::Docker),
            #[cfg(any(test, feature = "test-support"))]
            "mock" => Some(Self::Mock),
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

pub(crate) type ReusableItems = HashMap<(Arc<str>, ItemId), (Entity<Pane>, EntityId)>;

impl SerializedPaneGroup {
    pub(crate) fn restore(
        self,
        project: &Entity<Project>,
        workspace_id: WorkspaceId,
        workspace: &mut Workspace,
        reusable_items: &mut ReusableItems,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> (
        Member,
        Option<Entity<Pane>>,
        Vec<Task<Option<Box<dyn ItemHandle>>>>,
    ) {
        self.normalize()
            .unwrap_or_else(|| Self::Pane(SerializedPane::new(Vec::new(), true, 0)))
            .restore_normalized(project, workspace_id, workspace, reusable_items, window, cx)
    }

    pub(super) fn normalize(self) -> Option<Self> {
        match self {
            Self::Pane(pane) => (!pane.children.is_empty()).then_some(Self::Pane(pane)),
            Self::Group {
                axis,
                children,
                flexes,
            } => {
                let original_count = children.len();
                let mut retained_flexes = Vec::new();
                let mut children = children
                    .into_iter()
                    .enumerate()
                    .filter_map(|(index, child)| {
                        let child = child.normalize()?;
                        if let Some(flex) = flexes.as_ref().and_then(|flexes| flexes.get(index)) {
                            retained_flexes.push(*flex);
                        }
                        Some(child)
                    })
                    .collect::<Vec<_>>();
                if children.len() <= 1 {
                    return children.pop();
                }
                let flexes = flexes
                    .filter(|flexes| flexes.len() == original_count)
                    .and_then(|flexes| {
                        if children.len() == original_count {
                            return Some(flexes);
                        }
                        let total = retained_flexes
                            .iter()
                            .map(|flex| f64::from(*flex))
                            .sum::<f64>();
                        if total <= 0.0
                            || retained_flexes
                                .iter()
                                .any(|flex| !flex.is_finite() || *flex < 0.0)
                        {
                            return None;
                        }
                        for flex in &mut retained_flexes {
                            *flex = (f64::from(*flex) * children.len() as f64 / total) as f32;
                        }
                        Some(retained_flexes)
                    });
                Some(Self::Group {
                    axis,
                    children,
                    flexes,
                })
            }
        }
    }

    fn restore_normalized(
        self,
        project: &Entity<Project>,
        workspace_id: WorkspaceId,
        workspace: &mut Workspace,
        reusable_items: &mut ReusableItems,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> (
        Member,
        Option<Entity<Pane>>,
        Vec<Task<Option<Box<dyn ItemHandle>>>>,
    ) {
        match self {
            SerializedPaneGroup::Group {
                axis,
                children,
                flexes,
            } => {
                let mut current_active_pane = None;
                let mut members = Vec::new();
                let mut tasks = Vec::new();
                for child in children {
                    let (member, active_pane, item_tasks) = child.restore_normalized(
                        project,
                        workspace_id,
                        workspace,
                        reusable_items,
                        window,
                        cx,
                    );
                    members.push(member);
                    tasks.extend(item_tasks);
                    current_active_pane = current_active_pane.or(active_pane);
                }
                (
                    Member::Axis(PaneAxis::load(axis.0, members, flexes)),
                    current_active_pane,
                    tasks,
                )
            }
            SerializedPaneGroup::Pane(serialized_pane) => {
                let pane = workspace.add_pane(window, cx);
                let tasks = serialized_pane.restore_to(
                    project,
                    &pane,
                    workspace_id,
                    workspace.weak_handle(),
                    reusable_items,
                    window,
                    cx,
                );
                (
                    Member::Pane(pane.clone()),
                    serialized_pane.active.then_some(pane),
                    tasks,
                )
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

    #[cfg(test)]
    pub async fn deserialize_to(
        &self,
        project: &Entity<Project>,
        pane: &WeakEntity<Pane>,
        workspace_id: WorkspaceId,
        workspace: WeakEntity<Workspace>,
        cx: &mut AsyncWindowContext,
    ) -> Result<Vec<Option<Box<dyn ItemHandle>>>> {
        let pane = pane.upgrade().context("Pane closed during restoration")?;
        let tasks = cx.update(|window, cx| {
            self.restore_to(
                project,
                &pane,
                workspace_id,
                workspace,
                &mut ReusableItems::default(),
                window,
                cx,
            )
        })?;
        Ok(futures::future::join_all(tasks).await)
    }

    fn restore_to(
        &self,
        project: &Entity<Project>,
        pane: &Entity<Pane>,
        workspace_id: WorkspaceId,
        workspace: WeakEntity<Workspace>,
        reusable_items: &mut ReusableItems,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<Task<Option<Box<dyn ItemHandle>>>> {
        pane.update(cx, |pane, cx| {
            let mut placeholders = Vec::new();
            for item in &self.children {
                let placeholder = cx.new(|cx| RestoringItem::new(item.clone(), cx));
                pane.add_restoring_item(placeholder.clone(), item.active, window, cx);
                placeholders.push(placeholder);
            }
            pane.set_pinned_count(self.pinned_count.min(placeholders.len()));

            if let Some(index) = self.children.iter().rposition(|item| item.preview) {
                pane.set_preview_item_id(placeholders.get(index).map(|item| item.entity_id()), cx);
            }

            placeholders
                .into_iter()
                .zip(&self.children)
                .map(|(placeholder, item)| {
                    if let Some((source, item_id)) =
                        reusable_items.remove(&(item.kind.clone(), item.item_id))
                        && let Some(item) = source.update(cx, |source, cx| {
                            source.take_item_for_restore(item_id, window, cx)
                        })
                    {
                        let replaced =
                            pane.replace_item(placeholder.entity_id(), item.clone(), window, cx);
                        return Task::ready(replaced.then_some(item));
                    }
                    let deserialize = SerializableItemRegistry::deserialize(
                        &item.kind,
                        project.clone(),
                        workspace.clone(),
                        workspace_id,
                        item.item_id,
                        window,
                        cx,
                    );
                    let workspace = workspace.clone();
                    let placeholder = placeholder.downgrade();
                    window.spawn(cx, async move |cx| {
                        let restored = deserialize.await;
                        let placeholder = placeholder.upgrade()?;
                        let (pane, already_open) = workspace
                            .read_with(cx, |workspace, cx| {
                                if workspace.restoration_cancelled.load(Ordering::Relaxed) {
                                    return None;
                                }
                                let pane = workspace.pane_for_item_id(placeholder.entity_id())?;
                                let already_open = restored.as_ref().is_ok_and(|item| {
                                    workspace
                                        .pane_for_item_id(item.item_id())
                                        .is_some_and(|pane| {
                                            pane.read(cx).index_for_item(item.as_ref()).is_some()
                                        })
                                });
                                Some((pane, already_open))
                            })
                            .ok()
                            .flatten()?;
                        pane.update_in(cx, |pane, window, cx| {
                            pane.index_for_item(&placeholder)?;
                            match restored {
                                Ok(item) => {
                                    if already_open {
                                        pane.remove_item(
                                            placeholder.entity_id(),
                                            false,
                                            false,
                                            window,
                                            cx,
                                        );
                                        return Some(item);
                                    }
                                    pane.replace_item(
                                        placeholder.entity_id(),
                                        item.clone(),
                                        window,
                                        cx,
                                    )
                                    .then_some(item)
                                }
                                Err(error) => {
                                    placeholder.update(cx, |placeholder, cx| {
                                        placeholder.set_error(
                                            SharedString::from(format!("{error:#}")),
                                            cx,
                                        );
                                    });
                                    None
                                }
                            }
                        })
                        .ok()
                        .flatten()
                    })
                })
                .collect()
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        SerializableItemDescriptor, SplitDirection, WorkspaceDb,
        item::{
            bind_item_persistence_id, item_persistence_id,
            test::{TestItem, TestProjectItem},
        },
        move_item, register_serializable_item,
    };
    use gpui::{Focusable, Global, TestAppContext};
    use project::FakeFs;

    #[gpui::test]
    async fn test_restore_placeholder_lifecycle(cx: &mut TestAppContext) {
        for action in [
            "hydrate",
            "close",
            "cancel",
            "move",
            "reorder",
            "duplicate",
            "already_open",
        ] {
            let app = &mut cx.new_app();
            crate::tests::init_test(app);
            let descriptor = app.update(|cx| {
                register_serializable_item::<TestItem>(cx);
                SerializableItemRegistry::descriptor("TestItem", cx).unwrap()
            });
            let project = Project::test(FakeFs::new(app.executor()), None, app).await;
            let (workspace, cx) =
                app.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
            let failed_descriptor =
                SerializedItem::new("Unrestorable", 43, action != "hydrate", action != "hydrate");
            let revision_before_restore =
                workspace.read_with(cx, |workspace, cx| workspace.serialization_revision(cx));
            let (source, target, placeholder, live, tasks) =
                workspace.update_in(cx, |workspace, window, cx| {
                    let source = workspace.active_pane().clone();
                    let project_item = TestProjectItem::new(1, "same.txt", cx);
                    cx.set_global(RestoreTest {
                        descriptor,
                        project_item: project_item.clone(),
                        open_in: (action == "already_open").then_some(source.clone()),
                    });
                    cx.global_mut::<SerializableItemRegistry>()
                        .descriptors_by_kind
                        .get_mut("TestItem")
                        .unwrap()
                        .deserialize = deserialize_test_item;
                    let tasks = SerializedPane::new(
                        vec![
                            SerializedItem::new(
                                "TestItem",
                                42,
                                action == "hydrate",
                                action == "hydrate",
                            ),
                            failed_descriptor.clone(),
                        ],
                        true,
                        1,
                    )
                    .restore_to(
                        &project,
                        &source,
                        WorkspaceId::from_i64(1),
                        workspace.weak_handle(),
                        &mut ReusableItems::default(),
                        window,
                        cx,
                    );
                    let placeholder = source
                        .read(cx)
                        .items_of_type::<RestoringItem>()
                        .next()
                        .unwrap();
                    let target = if action == "move" {
                        workspace.split_pane(source.clone(), SplitDirection::Right, window, cx)
                    } else {
                        source.clone()
                    };
                    let live = (action != "already_open" && action != "hydrate").then(|| {
                        let live = cx.new(|cx| {
                            let mut item = TestItem::new(cx).with_dirty(true);
                            item.state = String::from("live edits");
                            item.project_items.push(project_item);
                            item
                        });
                        target.update(cx, |pane, cx| {
                            pane.add_item(Box::new(live.clone()), false, false, None, window, cx);
                        });
                        live
                    });
                    match action {
                        "hydrate" => {
                            window.focus(&placeholder.focus_handle(cx), cx);
                        }
                        "close" => source.update(cx, |pane, cx| {
                            pane.remove_item(placeholder.entity_id(), false, false, window, cx);
                        }),
                        "cancel" => workspace
                            .restoration_cancelled
                            .store(true, Ordering::Relaxed),
                        "move" | "reorder" => {
                            move_item(
                                &source,
                                &target,
                                placeholder.entity_id(),
                                1,
                                false,
                                window,
                                cx,
                            );
                            target.update(cx, |pane, cx| {
                                pane.set_pinned_count(1);
                                pane.set_preview_item_id(Some(placeholder.entity_id()), cx);
                            });
                        }
                        _ => {}
                    }
                    (source, target, placeholder, live, tasks)
                });
            if action == "hydrate" {
                assert_eq!(
                    workspace.read_with(cx, |workspace, cx| workspace.serialization_revision(cx)),
                    revision_before_restore
                );
            }
            let expected_ids = target.read_with(cx, |pane, _| {
                pane.items().map(|item| item.item_id()).collect::<Vec<_>>()
            });
            let expected_state = target.read_with(cx, |pane, _| {
                (
                    pane.active_item_index(),
                    pane.pinned_count(),
                    pane.preview_item_id(),
                )
            });
            let revision =
                workspace.read_with(cx, |workspace, cx| workspace.serialization_revision(cx));
            let mut results = futures::future::join_all(tasks).await;
            if action == "hydrate" {
                assert_eq!(
                    workspace.read_with(cx, |workspace, cx| workspace.serialization_revision(cx)),
                    revision
                );
            }
            let restored = results.remove(0);
            assert!(results.into_iter().all(|item| item.is_none()));
            if action == "close" || action == "cancel" {
                assert!(restored.is_none());
                assert_eq!(
                    target.read_with(cx, |pane, _| pane
                        .items()
                        .map(|item| item.item_id())
                        .collect::<Vec<_>>()),
                    expected_ids
                );
            } else {
                let restored = restored.unwrap().downcast::<TestItem>().unwrap();
                assert_ne!(
                    cx.update(|_, cx| item_persistence_id(&restored, cx))
                        .await
                        .unwrap(),
                    42
                );
                assert_eq!(
                    workspace.read_with(cx, |workspace, _| workspace
                        .pane_for_item_id(restored.entity_id())),
                    Some(target.clone())
                );
                if action == "already_open" {
                    assert_eq!(target.read_with(cx, |pane, _| pane.items_len()), 2);
                } else {
                    assert_eq!(
                        target.read_with(cx, |pane, _| pane
                            .items()
                            .map(|item| item.item_id())
                            .collect::<Vec<_>>()),
                        expected_ids
                            .into_iter()
                            .map(|id| if id == placeholder.entity_id() {
                                restored.entity_id()
                            } else {
                                id
                            })
                            .collect::<Vec<_>>()
                    );
                    assert_eq!(
                        target.read_with(cx, |pane, _| (
                            pane.active_item_index(),
                            pane.pinned_count(),
                            pane.preview_item_id()
                        )),
                        (
                            expected_state.0,
                            expected_state.1,
                            expected_state.2.map(|id| if id == placeholder.entity_id() {
                                restored.entity_id()
                            } else {
                                id
                            })
                        )
                    );
                }
                assert!(
                    workspace
                        .read_with(cx, |workspace, _| workspace
                            .pane_for_item_id(placeholder.entity_id()))
                        .is_none()
                );
            }
            source.read_with(cx, |pane, cx| {
                let failed = pane
                    .items_of_type::<RestoringItem>()
                    .find(|item| item.read(cx).serialized_item.item_id == 43)
                    .unwrap();
                assert_eq!(failed.read(cx).serialized_item, failed_descriptor);
                assert_eq!(
                    failed.read(cx).error.as_deref(),
                    (action != "cancel")
                        .then_some("cannot deserialize Unrestorable, descriptor not found")
                );
            });
            if let Some(live) = live {
                live.read_with(cx, |item, _| assert_eq!(item.state, "live edits"));
            }
            if action == "hydrate" {
                source.update_in(cx, |pane, window, cx| {
                    let item = pane.items_of_type::<TestItem>().next().unwrap();
                    pane.remove_item(item.entity_id(), false, false, window, cx);
                });
                assert_ne!(
                    workspace.read_with(cx, |workspace, cx| workspace.serialization_revision(cx)),
                    revision
                );
            }
            app.quit();
        }
    }

    #[gpui::test]
    async fn test_reload_reuses_persisted_items(cx: &mut TestAppContext) {
        crate::tests::init_test(cx);
        cx.update(register_serializable_item::<TestItem>);
        let project = Project::test(FakeFs::new(cx.executor()), None, cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
        let database = cx.update(|_, cx| WorkspaceDb::global(cx));
        let workspace_id = database.next_id().await.unwrap();
        let (source, live) = workspace.update_in(cx, |workspace, window, cx| {
            workspace.set_database_id(workspace_id);
            let source = workspace.active_pane().clone();
            let live = cx.new(TestItem::new);
            bind_item_persistence_id(&live, 42, cx).unwrap();
            source.update(cx, |pane, cx| {
                pane.add_item(Box::new(live.clone()), true, true, None, window, cx);
                pane.set_pinned_count(1);
                pane.set_preview_item_id(Some(live.entity_id()), cx);
            });
            (source, live)
        });
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.flush_serialization_checked(window, cx)
            })
            .await
            .unwrap();
        let serialized = database.workspace_for_id(workspace_id).unwrap();
        let live_id = cx
            .update(|_, cx| item_persistence_id(&live, cx))
            .await
            .unwrap();
        assert_ne!(live_id, 42);
        live.update(cx, |item, _| {
            item.state = String::from("unsaved live edits");
            item.is_dirty = true;
        });
        let pending = cx.new(TestItem::new);
        source.update_in(cx, |pane, window, cx| {
            pane.add_item(Box::new(pending.clone()), false, false, None, window, cx);
        });
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.load_workspace(serialized, Vec::new(), window, cx)
            })
            .await
            .unwrap();
        workspace.read_with(cx, |workspace, cx| {
            let restored = workspace.active_pane();
            assert_ne!(restored, &source);
            assert_eq!(
                restored
                    .read(cx)
                    .items()
                    .map(|item| item.item_id())
                    .collect::<Vec<_>>(),
                vec![live.entity_id()]
            );
            assert_eq!(restored.read(cx).pinned_count(), 1);
            assert_eq!(restored.read(cx).preview_item_id(), Some(live.entity_id()));
            assert_eq!(
                source
                    .read(cx)
                    .items()
                    .map(|item| item.item_id())
                    .collect::<Vec<_>>(),
                vec![pending.entity_id()]
            );
            assert_eq!(
                workspace.pane_for_item_id(live.entity_id()).as_ref(),
                Some(restored)
            );
            assert_eq!(live.read(cx).state, "unsaved live edits");
            assert!(live.read(cx).is_dirty);
        });
        let pending_id = cx
            .update(|_, cx| item_persistence_id(&pending, cx))
            .await
            .unwrap();
        assert_ne!(pending_id, live_id);
        let mut expected_ids = vec![live_id, pending_id];
        expected_ids.sort_unstable();
        let mut saved_items = database
            .select_bound::<_, ItemId>(
                "SELECT item_id FROM items WHERE workspace_id = ? ORDER BY item_id",
            )
            .unwrap();
        assert_eq!(saved_items(workspace_id).unwrap(), expected_ids);
    }

    #[gpui::test]
    async fn test_restored_single_pane_cannot_be_removed(cx: &mut TestAppContext) {
        crate::tests::init_test(cx);
        cx.update(register_serializable_item::<TestItem>);
        let project = Project::test(FakeFs::new(cx.executor()), None, cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
        let database = cx.update(|_, cx| WorkspaceDb::global(cx));
        let workspace_id = database.next_id().await.unwrap();
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.set_database_id(workspace_id);
            let source = workspace.active_pane().clone();
            let item = cx.new(TestItem::new);
            workspace.add_item_to_active_pane(Box::new(item), None, true, window, cx);
            workspace.split_pane(source.clone(), SplitDirection::Right, window, cx);
            workspace.split_pane(source.clone(), SplitDirection::Down, window, cx);
            window.focus(&source.focus_handle(cx), cx);
            assert_eq!(workspace.center.panes().len(), 3);
        });
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.flush_serialization_checked(window, cx)
            })
            .await
            .unwrap();
        let serialized = database.workspace_for_id(workspace_id).unwrap();
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.load_workspace(serialized, Vec::new(), window, cx)
            })
            .await
            .unwrap();
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
        pane.update_in(cx, |pane, window, cx| {
            assert_eq!(pane.items_len(), 1);
            let item = pane.active_item().unwrap();
            pane.remove_item(item.item_id(), false, true, window, cx);
        });
        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(workspace.center.panes().len(), 1);
            assert_eq!(workspace.panes().len(), 1);
            assert_eq!(workspace.active_pane(), &pane);
            assert_eq!(pane.read(cx).items_len(), 0);
        });
    }

    struct RestoreTest {
        descriptor: SerializableItemDescriptor,
        project_item: Entity<TestProjectItem>,
        open_in: Option<Entity<Pane>>,
    }

    impl Global for RestoreTest {}

    fn deserialize_test_item(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut Context<Pane>,
    ) -> Task<Result<Box<dyn ItemHandle>>> {
        let test = cx.global::<RestoreTest>();
        let project_item = test.project_item.clone();
        let open_in = test.open_in.clone();
        let deserialize = test.descriptor.deserialize;
        let task = deserialize(project, workspace, workspace_id, item_id, window, cx);
        window.spawn(cx, async move |cx| {
            let item = task.await?;
            item.downcast::<TestItem>().unwrap().update(cx, |item, _| {
                item.project_items.push(project_item);
            });
            if let Some(pane) = open_in {
                pane.update_in(cx, |pane, window, cx| {
                    pane.add_item_inner(item.clone(), false, false, false, None, window, cx);
                })?;
            }
            Ok(item)
        })
    }
}
