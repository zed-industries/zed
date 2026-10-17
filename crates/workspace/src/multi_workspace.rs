use anyhow::{Context as _, Result};
use fs::Fs;
use futures::channel::oneshot;

use gpui::{
    AnyView, App, AsyncWindowContext, Context, DragMoveEvent, Entity, EntityId, EventEmitter,
    FocusHandle, Focusable, ManagedView, MouseButton, Pixels, Render, Subscription, Task, TaskExt,
    WeakEntity, Window, WindowId, actions, deferred, px,
};
pub use project::ProjectGroupKey;
use project::{DisableAiSettings, Project};
use remote::RemoteConnectionOptions;
use settings::Settings;
pub use settings::SidebarSide;
use std::cell::Cell;
use std::path::PathBuf;
use std::rc::{Rc, Weak};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use ui::prelude::*;
use util::ResultExt;
use util::path_list::PathList;
use zed_actions::agents_sidebar::ToggleThreadSwitcher;

use agent_settings::AgentSettings;
use settings::SidebarDockPosition;
use ui::{ContextMenu, right_click_menu};

const SIDEBAR_RESIZE_HANDLE_SIZE: Pixels = px(6.0);

use crate::open_remote_project_with_existing_connection;
use crate::{
    CloseIntent, CloseWindow, DockPosition, Event as WorkspaceEvent, Item, LocalWorkspaceSource,
    ModalView, OpenMode, Panel, PendingWorkspaceIds, Workspace, WorkspaceId,
    WorkspaceIdReservation, client_side_decorations,
    notifications::DetachAndPromptErr,
    persistence::{
        WorkspaceDb,
        model::{MultiWorkspaceState, SerializedWorkspace, SerializedWorkspaceLocation},
    },
};

actions!(
    multi_workspace,
    [
        /// Toggles the workspace switcher sidebar.
        ToggleWorkspaceSidebar,
        /// Closes the workspace sidebar.
        CloseWorkspaceSidebar,
        /// Moves focus to or from the workspace sidebar without closing it.
        FocusWorkspaceSidebar,
        /// Activates the next project in the sidebar.
        NextProject,
        /// Activates the previous project in the sidebar.
        PreviousProject,
        /// Moves the active project up in the sidebar.
        MoveProjectUp,
        /// Moves the active project down in the sidebar.
        MoveProjectDown,
        /// Activates the next thread in sidebar order.
        NextThread,
        /// Activates the previous thread in sidebar order.
        PreviousThread,
        /// Creates a new thread in the current workspace.
        NewThread,
        /// Moves the active project to a new window.
        MoveProjectToNewWindow,
    ]
);

#[derive(Default)]
pub struct SidebarRenderState {
    pub open: bool,
    pub side: SidebarSide,
}

pub fn sidebar_side_context_menu(
    id: impl Into<ElementId>,
    cx: &App,
) -> ui::RightClickMenu<ContextMenu> {
    let current_position = AgentSettings::get_global(cx).threads_sidebar.position;
    right_click_menu(id).menu(move |window, cx| {
        let fs = <dyn fs::Fs>::global(cx);
        ContextMenu::build(window, cx, move |mut menu, _, _cx| {
            let positions: [(SidebarDockPosition, &str); 2] = [
                (SidebarDockPosition::Left, "Left"),
                (SidebarDockPosition::Right, "Right"),
            ];
            for (position, label) in positions {
                let fs = fs.clone();
                menu = menu.toggleable_entry(
                    label,
                    position == current_position,
                    IconPosition::Start,
                    None,
                    move |_window, cx| {
                        let side = match position {
                            SidebarDockPosition::Left => "left",
                            SidebarDockPosition::Right => "right",
                        };
                        telemetry::event!("Sidebar Side Changed", side = side);
                        settings::update_settings_file(fs.clone(), cx, move |settings, _cx| {
                            settings
                                .agent
                                .get_or_insert_default()
                                .set_threads_sidebar_position(Some(position));
                        });
                    },
                );
            }
            menu
        })
    })
}

pub enum MultiWorkspaceEvent {
    ActiveWorkspaceChanged {
        source_workspace: Option<WeakEntity<Workspace>>,
    },
    WorkspaceAdded(Entity<Workspace>),
    WorkspaceRemoved(EntityId),
    ProjectGroupsChanged,
}

pub enum SidebarEvent {
    SerializeNeeded,
}

pub trait Sidebar: Focusable + Render + EventEmitter<SidebarEvent> + Sized {
    fn width(&self, cx: &App) -> Pixels;
    fn set_width(&mut self, width: Option<Pixels>, cx: &mut Context<Self>);
    fn has_notifications(&self, cx: &App) -> bool;
    fn side(&self, _cx: &App) -> SidebarSide;

    fn is_threads_list_view_active(&self) -> bool {
        true
    }
    /// Makes focus reset back to the search editor upon toggling the sidebar from outside
    fn prepare_for_focus(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}
    /// Opens or cycles the thread switcher popup.
    fn toggle_thread_switcher(
        &mut self,
        _select_last: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    /// Activates the next or previous project.
    fn cycle_project(&mut self, _forward: bool, _window: &mut Window, _cx: &mut Context<Self>) {}

    /// Activates the next or previous thread in sidebar order.
    fn cycle_thread(&mut self, _forward: bool, _window: &mut Window, _cx: &mut Context<Self>) {}

    /// Return an opaque JSON blob of sidebar-specific state to persist.
    fn serialized_state(&self, _cx: &App) -> Option<String> {
        None
    }

    /// Restore sidebar state from a previously-serialized blob.
    fn restore_serialized_state(
        &mut self,
        _state: &str,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}

pub trait SidebarHandle: 'static + Send + Sync {
    fn width(&self, cx: &App) -> Pixels;
    fn set_width(&self, width: Option<Pixels>, cx: &mut App);
    fn focus_handle(&self, cx: &App) -> FocusHandle;
    fn focus(&self, window: &mut Window, cx: &mut App);
    fn prepare_for_focus(&self, window: &mut Window, cx: &mut App);
    fn has_notifications(&self, cx: &App) -> bool;
    fn to_any(&self) -> AnyView;
    fn entity_id(&self) -> EntityId;
    fn toggle_thread_switcher(&self, select_last: bool, window: &mut Window, cx: &mut App);
    fn cycle_project(&self, forward: bool, window: &mut Window, cx: &mut App);
    fn cycle_thread(&self, forward: bool, window: &mut Window, cx: &mut App);

    fn is_threads_list_view_active(&self, cx: &App) -> bool;

    fn side(&self, cx: &App) -> SidebarSide;
    fn serialized_state(&self, cx: &App) -> Option<String>;
    fn restore_serialized_state(&self, state: &str, window: &mut Window, cx: &mut App);
}

#[derive(Clone)]
pub struct DraggedSidebar;

impl Render for DraggedSidebar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

impl<T: Sidebar> SidebarHandle for Entity<T> {
    fn width(&self, cx: &App) -> Pixels {
        self.read(cx).width(cx)
    }

    fn set_width(&self, width: Option<Pixels>, cx: &mut App) {
        self.update(cx, |this, cx| this.set_width(width, cx))
    }

    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.read(cx).focus_handle(cx)
    }

    fn focus(&self, window: &mut Window, cx: &mut App) {
        let handle = self.read(cx).focus_handle(cx);
        window.focus(&handle, cx);
    }

    fn prepare_for_focus(&self, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.prepare_for_focus(window, cx));
    }

    fn has_notifications(&self, cx: &App) -> bool {
        self.read(cx).has_notifications(cx)
    }

    fn to_any(&self) -> AnyView {
        self.clone().into()
    }

    fn entity_id(&self) -> EntityId {
        Entity::entity_id(self)
    }

    fn toggle_thread_switcher(&self, select_last: bool, window: &mut Window, cx: &mut App) {
        let entity = self.clone();
        window.defer(cx, move |window, cx| {
            entity.update(cx, |this, cx| {
                this.toggle_thread_switcher(select_last, window, cx);
            });
        });
    }

    fn cycle_project(&self, forward: bool, window: &mut Window, cx: &mut App) {
        let entity = self.clone();
        window.defer(cx, move |window, cx| {
            entity.update(cx, |this, cx| {
                this.cycle_project(forward, window, cx);
            });
        });
    }

    fn cycle_thread(&self, forward: bool, window: &mut Window, cx: &mut App) {
        let entity = self.clone();
        window.defer(cx, move |window, cx| {
            entity.update(cx, |this, cx| {
                this.cycle_thread(forward, window, cx);
            });
        });
    }

    fn is_threads_list_view_active(&self, cx: &App) -> bool {
        self.read(cx).is_threads_list_view_active()
    }

    fn side(&self, cx: &App) -> SidebarSide {
        self.read(cx).side(cx)
    }

    fn serialized_state(&self, cx: &App) -> Option<String> {
        self.read(cx).serialized_state(cx)
    }

    fn restore_serialized_state(&self, state: &str, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| {
            this.restore_serialized_state(state, window, cx)
        })
    }
}

#[derive(Clone)]
pub struct ProjectGroup {
    pub key: ProjectGroupKey,
    pub workspaces: Vec<Entity<Workspace>>,
    pub expanded: bool,
}

pub struct SerializedProjectGroupState {
    pub key: ProjectGroupKey,
    pub expanded: bool,
}

#[derive(Clone)]
pub struct ProjectGroupState {
    pub key: ProjectGroupKey,
    pub expanded: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RemovalIntent {
    KeepProject,
    CloseProject,
    MoveProject,
}

pub(crate) struct WorkspaceRetirement {
    pub(crate) cancelled: Arc<AtomicBool>,
    pub(crate) closing_window: Cell<bool>,
}

/// One row per workspace held by this window. The displayed workspace is
/// always one of these rows. `pinned` records whether the workspace survives
/// being navigated away from; `activated_at` records when it was last
/// displayed.
struct HeldWorkspace {
    workspace: Entity<Workspace>,
    pinned: bool,
    activated_at: Option<u64>,
}

pub struct MultiWorkspace {
    pub(crate) restore_key: Option<WorkspaceId>,
    pub(crate) serialization_fence: Arc<AtomicBool>,
    restoration_ready: bool,
    window_id: WindowId,
    serialization_cancelled: Arc<AtomicBool>,
    retirement: Weak<WorkspaceRetirement>,
    held: Vec<HeldWorkspace>,
    unloaded_workspace_ids: Vec<WorkspaceId>,
    project_groups: Vec<ProjectGroupState>,
    /// Source of truth for which workspace is presented in this window, shared
    /// with each member `Workspace` so they can tell whether they own the
    /// platform window's title and edited indicator. This only exists to prevent
    /// Workspaces from having to read their parent MultiWorkspace to check
    /// chrome ownership, as that might cause a double lease. Kept in sync with
    /// `active_workspace`.
    active_workspace_id: Rc<Cell<EntityId>>,
    sidebar: Option<Box<dyn SidebarHandle>>,
    sidebar_open: bool,
    sidebar_overlay: Option<AnyView>,
    _serialize_task: Option<Task<()>>,
    _collapse_task: Option<(Task<()>, Arc<AtomicBool>)>,
    _subscriptions: Vec<Subscription>,
    previous_focus_handle: Option<FocusHandle>,
}

struct WorkspaceRemoval {
    workspaces: Vec<Entity<Workspace>>,
    unloaded_workspace_ids: Vec<WorkspaceId>,
    project_groups: Vec<ProjectGroupKey>,
    intent: RemovalIntent,
}

impl EventEmitter<MultiWorkspaceEvent> for MultiWorkspace {}

impl MultiWorkspace {
    pub fn sidebar_side(&self, cx: &App) -> SidebarSide {
        self.sidebar
            .as_ref()
            .map_or(SidebarSide::Left, |s| s.side(cx))
    }

    pub fn sidebar_render_state(&self, cx: &App) -> SidebarRenderState {
        SidebarRenderState {
            open: self.sidebar_open() && self.multi_workspace_enabled(cx),
            side: self.sidebar_side(cx),
        }
    }

    pub fn new(workspace: Entity<Workspace>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let release_subscription = cx.on_release(|multi_workspace: &mut MultiWorkspace, _cx| {
            if let Some(task) = multi_workspace._serialize_task.take() {
                task.detach();
            }
        });
        let settings_subscription = cx.observe_global_in::<settings::SettingsStore>(window, {
            let mut previous_multi_workspace_enabled = !DisableAiSettings::get_global(cx)
                .disable_ai
                && AgentSettings::get_global(cx).enabled;
            move |this, window, cx| {
                let multi_workspace_enabled = this.multi_workspace_enabled(cx);
                if previous_multi_workspace_enabled && !multi_workspace_enabled {
                    this.collapse_to_single_workspace(window, cx);
                } else if !previous_multi_workspace_enabled && multi_workspace_enabled {
                    if let Some((task, cancelled)) = this._collapse_task.take() {
                        cancelled.store(true, Ordering::Relaxed);
                        drop(task);
                    }
                }
                previous_multi_workspace_enabled = multi_workspace_enabled;
            }
        });
        Self::subscribe_to_workspace(&workspace, window, cx);
        let weak_self = cx.weak_entity();
        let active_workspace_id = Rc::new(Cell::new(workspace.entity_id()));
        let serialization_fence = Arc::new(AtomicBool::new(false));
        workspace.update(cx, |workspace, cx| {
            workspace.serialization_fence = serialization_fence.clone();
            workspace.set_multi_workspace(weak_self, active_workspace_id.clone(), cx);
        });
        let mut multi_workspace = Self {
            restore_key: None,
            restoration_ready: false,
            window_id: window.window_handle().window_id(),
            serialization_cancelled: Arc::new(AtomicBool::new(false)),
            serialization_fence,
            retirement: Weak::new(),
            held: vec![HeldWorkspace {
                workspace,
                pinned: false,
                activated_at: Some(0),
            }],
            unloaded_workspace_ids: Vec::new(),
            project_groups: Vec::new(),
            active_workspace_id,
            sidebar: None,
            sidebar_open: false,
            sidebar_overlay: None,
            _serialize_task: None,
            _collapse_task: None,
            _subscriptions: vec![release_subscription, settings_subscription],
            previous_focus_handle: None,
        };
        multi_workspace.serialize(cx);
        multi_workspace
    }

    pub fn set_restore_key(&mut self, workspace_id: WorkspaceId) {
        if self.restore_key != Some(workspace_id) {
            self.invalidate_serialization();
            self.restore_key = Some(workspace_id);
            self.restoration_ready = false;
        }
    }

    pub fn register_sidebar<T: Sidebar>(&mut self, sidebar: Entity<T>, cx: &mut Context<Self>) {
        self._subscriptions
            .push(cx.observe(&sidebar, |_this, _, cx| {
                cx.notify();
            }));
        self._subscriptions
            .push(cx.subscribe(&sidebar, |this, _, event, cx| match event {
                SidebarEvent::SerializeNeeded => {
                    this.serialize(cx);
                }
            }));
        self.sidebar = Some(Box::new(sidebar));
    }

    pub fn sidebar(&self) -> Option<&dyn SidebarHandle> {
        self.sidebar.as_deref()
    }

    pub fn set_sidebar_overlay(&mut self, overlay: Option<AnyView>, cx: &mut Context<Self>) {
        self.sidebar_overlay = overlay;
        cx.notify();
    }

    pub fn sidebar_open(&self) -> bool {
        self.sidebar_open
    }

    pub fn sidebar_has_notifications(&self, cx: &App) -> bool {
        self.sidebar
            .as_ref()
            .map_or(false, |s| s.has_notifications(cx))
    }

    pub fn is_threads_list_view_active(&self, cx: &App) -> bool {
        self.sidebar
            .as_ref()
            .map_or(false, |s| s.is_threads_list_view_active(cx))
    }

    pub fn multi_workspace_enabled(&self, cx: &App) -> bool {
        !DisableAiSettings::get_global(cx).disable_ai && AgentSettings::get_global(cx).enabled
    }

    pub fn toggle_sidebar(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.multi_workspace_enabled(cx) {
            return;
        }

        if self.sidebar_open() {
            self.close_sidebar(window, cx);
        } else {
            self.previous_focus_handle = window.focused(cx);
            self.open_sidebar(cx);
            if let Some(sidebar) = &self.sidebar {
                sidebar.prepare_for_focus(window, cx);
                sidebar.focus(window, cx);
            }
        }
    }

    pub fn close_sidebar_action(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.multi_workspace_enabled(cx) {
            return;
        }

        if self.sidebar_open() {
            self.close_sidebar(window, cx);
        }
    }

    pub fn focus_sidebar(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.multi_workspace_enabled(cx) {
            return;
        }

        if self.sidebar_open() {
            let sidebar_is_focused = self
                .sidebar
                .as_ref()
                .is_some_and(|s| s.focus_handle(cx).contains_focused(window, cx));

            if sidebar_is_focused {
                self.restore_previous_focus(false, window, cx);
            } else {
                self.previous_focus_handle = window.focused(cx);
                if let Some(sidebar) = &self.sidebar {
                    sidebar.prepare_for_focus(window, cx);
                    sidebar.focus(window, cx);
                }
            }
        } else {
            self.previous_focus_handle = window.focused(cx);
            self.open_sidebar(cx);
            if let Some(sidebar) = &self.sidebar {
                sidebar.prepare_for_focus(window, cx);
                sidebar.focus(window, cx);
            }
        }
    }

    pub fn open_sidebar(&mut self, cx: &mut Context<Self>) {
        let side = match self.sidebar_side(cx) {
            SidebarSide::Left => "left",
            SidebarSide::Right => "right",
        };
        telemetry::event!("Sidebar Toggled", action = "open", side = side);
        self.apply_open_sidebar(cx);
    }

    /// Restores the sidebar to open state from persisted session data without
    /// firing a telemetry event, since this is not a user-initiated action.
    pub(crate) fn restore_open_sidebar(&mut self, cx: &mut Context<Self>) {
        self.apply_open_sidebar(cx);
    }

    fn apply_open_sidebar(&mut self, cx: &mut Context<Self>) {
        self.sidebar_open = true;
        self.retain_active_workspace(cx);
        let sidebar_focus_handle = self.sidebar.as_ref().map(|s| s.focus_handle(cx));
        for workspace in self.workspaces().cloned().collect::<Vec<_>>() {
            workspace.update(cx, |workspace, _cx| {
                workspace.set_sidebar_focus_handle(sidebar_focus_handle.clone());
            });
        }
        self.serialize(cx);
        cx.notify();
    }

    pub fn close_sidebar(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let side = match self.sidebar_side(cx) {
            SidebarSide::Left => "left",
            SidebarSide::Right => "right",
        };
        telemetry::event!("Sidebar Toggled", action = "close", side = side);
        self.sidebar_open = false;
        for workspace in self.workspaces().cloned().collect::<Vec<_>>() {
            workspace.update(cx, |workspace, _cx| {
                workspace.set_sidebar_focus_handle(None);
            });
        }
        let sidebar_has_focus = self
            .sidebar
            .as_ref()
            .is_some_and(|s| s.focus_handle(cx).contains_focused(window, cx));
        if sidebar_has_focus {
            self.restore_previous_focus(true, window, cx);
        } else {
            self.previous_focus_handle.take();
        }
        self.serialize(cx);
        cx.notify();
    }

    fn restore_previous_focus(&mut self, clear: bool, window: &mut Window, cx: &mut Context<Self>) {
        let focus_handle = if clear {
            self.previous_focus_handle.take()
        } else {
            self.previous_focus_handle.clone()
        };

        if let Some(previous_focus) = focus_handle {
            previous_focus.focus(window, cx);
        } else {
            let pane = self.workspace().read(cx).active_pane().clone();
            window.focus(&pane.read(cx).focus_handle(cx), cx);
        }
    }

    pub fn close_window(&mut self, _: &CloseWindow, window: &mut Window, cx: &mut Context<Self>) {
        let Some(window_handle) = window.window_handle().downcast::<Self>() else {
            log::error!("cannot close a window whose root is not a MultiWorkspace");
            return;
        };
        cx.spawn(async move |_, cx| {
            if !crate::prepare_window_to_close(window_handle, CloseIntent::CloseWindow, cx)
                .await
                .context("preparing the window to close")?
            {
                return anyhow::Ok(());
            }

            window_handle
                .update(cx, |_, window, _cx| {
                    window.remove_window();
                })
                .context("removing the closed window")?;

            anyhow::Ok(())
        })
        .detach_and_prompt_err("Failed to close window", window, cx, |_, _, _| None);
    }

    fn subscribe_to_workspace(
        workspace: &Entity<Workspace>,
        window: &Window,
        cx: &mut Context<Self>,
    ) {
        let project = workspace.read(cx).project().clone();
        cx.subscribe_in(&project, window, {
            let workspace = workspace.downgrade();
            move |this, _project, event, _window, cx| match event {
                project::Event::WorktreePathsChanged { old_worktree_paths } => {
                    if let Some(workspace) = workspace.upgrade() {
                        let host = workspace
                            .read(cx)
                            .project()
                            .read(cx)
                            .remote_connection_options(cx);
                        let old_key =
                            ProjectGroupKey::from_worktree_paths(old_worktree_paths, host);
                        this.handle_project_group_key_change(&workspace, &old_key, cx);
                    }
                }
                _ => {}
            }
        })
        .detach();

        cx.subscribe_in(workspace, window, |this, workspace, event, window, cx| {
            if let WorkspaceEvent::Activate = event {
                this.activate(workspace.clone(), None, window, cx);
            }
        })
        .detach();
    }

    fn handle_project_group_key_change(
        &mut self,
        workspace: &Entity<Workspace>,
        old_key: &ProjectGroupKey,
        cx: &mut Context<Self>,
    ) {
        if !self.is_workspace_retained(workspace) {
            return;
        }

        let new_key = workspace.read(cx).project_group_key(cx);
        if new_key.path_list().paths().is_empty() {
            return;
        }

        // The Project already emitted WorktreePathsChanged which the
        // sidebar handles for thread migration.
        self.rekey_project_group(old_key, &new_key, cx);
        self.serialize(cx);
        cx.notify();
    }

    fn held_index(&self, workspace: &Entity<Workspace>) -> Option<usize> {
        self.held
            .iter()
            .position(|held| held.workspace == *workspace)
    }

    pub fn is_workspace_retained(&self, workspace: &Entity<Workspace>) -> bool {
        self.held
            .iter()
            .any(|held| held.pinned && held.workspace == *workspace)
    }

    pub fn active_workspace_is_retained(&self) -> bool {
        self.held[self.displayed_index()].pinned
    }

    /// The displayed workspace is the most recently activated row.
    fn displayed_index(&self) -> usize {
        self.held
            .iter()
            .enumerate()
            .max_by_key(|(_, held)| held.activated_at)
            .expect("a window always holds at least one workspace")
            .0
    }

    /// Ensures a project group exists for `key`, creating one if needed.
    fn ensure_project_group_state(&mut self, key: ProjectGroupKey) {
        if key.path_list().paths().is_empty() {
            return;
        }

        if self.project_groups.iter().any(|group| group.key == key) {
            return;
        }

        self.project_groups.insert(
            0,
            ProjectGroupState {
                key,
                expanded: true,
            },
        );
    }

    /// Transitions a project group from `old_key` to `new_key`.
    ///
    /// On collision (both keys have groups), the active workspace's
    /// Re-keys a project group from `old_key` to `new_key`, handling
    /// collisions. When two groups collide, the active workspace's
    /// group always wins. Otherwise the old key's state is preserved
    /// — it represents the group the user or system just acted on.
    /// The losing group is removed, and the winner is re-keyed in
    /// place to preserve sidebar order.
    fn rekey_project_group(
        &mut self,
        old_key: &ProjectGroupKey,
        new_key: &ProjectGroupKey,
        cx: &App,
    ) {
        if old_key == new_key {
            return;
        }

        if new_key.path_list().paths().is_empty() {
            return;
        }

        let old_key_exists = self.project_groups.iter().any(|g| g.key == *old_key);
        let new_key_exists = self.project_groups.iter().any(|g| g.key == *new_key);

        if !old_key_exists {
            self.ensure_project_group_state(new_key.clone());
            return;
        }

        if new_key_exists {
            let active_key = self.workspace().read(cx).project_group_key(cx);
            if active_key == *new_key {
                self.project_groups.retain(|g| g.key != *old_key);
            } else {
                self.project_groups.retain(|g| g.key != *new_key);
                if let Some(group) = self.project_groups.iter_mut().find(|g| g.key == *old_key) {
                    group.key = new_key.clone();
                }
            }
        } else {
            if let Some(group) = self.project_groups.iter_mut().find(|g| g.key == *old_key) {
                group.key = new_key.clone();
            }
        }

        // If another retained workspace still has the old key (e.g. a
        // linked worktree workspace), re-create the old group so it
        // remains reachable in the sidebar.
        let other_workspace_needs_old_key = self
            .held
            .iter()
            .any(|held| held.pinned && held.workspace.read(cx).project_group_key(cx) == *old_key);
        if other_workspace_needs_old_key {
            self.ensure_project_group_state(old_key.clone());
        }
    }

    /// Ensures `workspace` has a row in `held`, registering it on first
    /// insert, and returns the row's index.
    fn hold(
        &mut self,
        workspace: Entity<Workspace>,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> usize {
        if let Some(index) = self.held_index(&workspace) {
            return index;
        }
        self.register_workspace(&workspace, window, cx);
        self.held.push(HeldWorkspace {
            workspace,
            pinned: false,
            activated_at: None,
        });
        self.held.len() - 1
    }

    /// Pins the row so the workspace survives navigating away, recording
    /// `group` as the project group it was pinned under. No-op if already
    /// pinned.
    fn pin(&mut self, index: usize, group: ProjectGroupKey, cx: &mut Context<Self>) {
        if self.held[index].pinned {
            return;
        }
        self.held[index].pinned = true;
        self.ensure_project_group_state(group);
        cx.emit(MultiWorkspaceEvent::WorkspaceAdded(
            self.held[index].workspace.clone(),
        ));
    }

    pub(crate) fn activate_provisional_workspace(
        &mut self,
        workspace: Entity<Workspace>,
        provisional_key: ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let index = self.hold(workspace.clone(), window, cx);
        self.pin(index, provisional_key, cx);
        self.activate(workspace, None, window, cx);
    }

    pub(crate) fn replace_disconnected_workspace(
        &mut self,
        previous: &Entity<Workspace>,
        replacement: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        anyhow::ensure!(
            previous.read(cx).project().read(cx).is_disconnected(cx)
                && previous.read(cx).database_id() == replacement.read(cx).database_id(),
            "Workspace cannot transfer its saved state to this connection"
        );
        let index = self
            .held_index(previous)
            .context("Workspace was closed during reconnection")?;
        self.held[index].pinned = true;
        self.activate(replacement, None, window, cx);
        self.release_workspace(previous, cx);
        self.serialize(cx);
        Ok(())
    }

    pub(crate) fn bind_workspace(&self, workspace: &Entity<Workspace>, cx: &mut Context<Self>) {
        let weak_self = cx.weak_entity();
        let active_workspace_id = self.active_workspace_id.clone();
        workspace.update(cx, |workspace, cx| {
            workspace.serialization_fence = self.serialization_fence.clone();
            workspace.set_multi_workspace(weak_self, active_workspace_id, cx);
        });
    }

    fn register_workspace(
        &mut self,
        workspace: &Entity<Workspace>,
        window: &Window,
        cx: &mut Context<Self>,
    ) {
        self.invalidate_serialization();
        if let Some(workspace_id) = workspace.read(cx).database_id() {
            self.unloaded_workspace_ids.retain(|id| *id != workspace_id);
        }
        Self::subscribe_to_workspace(workspace, window, cx);
        self.bind_workspace(workspace, cx);

        let entity = cx.entity();
        cx.defer({
            let workspace = workspace.clone();
            move |cx| {
                entity.update(cx, |this, cx| {
                    this.sync_sidebar_to_workspace(&workspace, cx);
                })
            }
        });
    }

    pub fn project_group_key_for_workspace(
        &self,
        workspace: &Entity<Workspace>,
        cx: &App,
    ) -> ProjectGroupKey {
        workspace.read(cx).project_group_key(cx)
    }

    pub fn restore_project_groups(
        &mut self,
        groups: Vec<SerializedProjectGroupState>,
        _cx: &mut Context<Self>,
    ) {
        self.invalidate_serialization();
        let mut restored: Vec<ProjectGroupState> = Vec::new();
        for SerializedProjectGroupState { key, expanded } in groups {
            if key.path_list().paths().is_empty() {
                continue;
            }
            if restored.iter().any(|group| group.key == key) {
                continue;
            }
            restored.push(ProjectGroupState { key, expanded });
        }
        for existing in std::mem::take(&mut self.project_groups) {
            if !restored.iter().any(|group| group.key == existing.key) {
                restored.push(existing);
            }
        }
        self.project_groups = restored;
    }

    pub fn project_group_keys(&self) -> Vec<ProjectGroupKey> {
        self.project_groups
            .iter()
            .map(|group| group.key.clone())
            .collect()
    }

    pub fn project_groups(&self, cx: &App) -> Vec<ProjectGroup> {
        self.project_groups
            .iter()
            .map(|group| ProjectGroup {
                key: group.key.clone(),
                workspaces: self.workspaces_for_project_group(&group.key, cx),
                expanded: group.expanded,
            })
            .collect()
    }

    pub fn last_active_workspace_for_group(
        &self,
        key: &ProjectGroupKey,
        cx: &App,
    ) -> Option<Entity<Workspace>> {
        self.held
            .iter()
            .filter(|held| held.workspace.read(cx).project_group_key(cx) == *key)
            .filter_map(|held| Some((held.activated_at?, &held.workspace)))
            .max_by_key(|(activated_at, _)| *activated_at)
            .map(|(_, workspace)| workspace.clone())
    }

    pub fn group_state_by_key(&self, key: &ProjectGroupKey) -> Option<&ProjectGroupState> {
        self.project_groups.iter().find(|group| group.key == *key)
    }

    pub fn group_state_by_key_mut(
        &mut self,
        key: &ProjectGroupKey,
    ) -> Option<&mut ProjectGroupState> {
        self.project_groups
            .iter_mut()
            .find(|group| group.key == *key)
    }

    pub fn set_all_groups_expanded(&mut self, expanded: bool) {
        for group in &mut self.project_groups {
            group.expanded = expanded;
        }
    }

    pub fn move_project_group_up(&mut self, key: &ProjectGroupKey, cx: &mut Context<Self>) -> bool {
        let Some(index) = self
            .project_groups
            .iter()
            .position(|group| group.key == *key)
        else {
            return false;
        };
        if index == 0 {
            return false;
        }
        self.project_groups.swap(index - 1, index);
        cx.emit(MultiWorkspaceEvent::ProjectGroupsChanged);
        self.serialize(cx);
        cx.notify();
        true
    }

    pub fn move_project_group_down(
        &mut self,
        key: &ProjectGroupKey,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(index) = self
            .project_groups
            .iter()
            .position(|group| group.key == *key)
        else {
            return false;
        };
        if index + 1 >= self.project_groups.len() {
            return false;
        }
        self.project_groups.swap(index, index + 1);
        cx.emit(MultiWorkspaceEvent::ProjectGroupsChanged);
        self.serialize(cx);
        cx.notify();
        true
    }

    pub fn workspaces_for_project_group(
        &self,
        key: &ProjectGroupKey,
        cx: &App,
    ) -> Vec<Entity<Workspace>> {
        self.held
            .iter()
            .filter(|held| held.pinned)
            .map(|held| held.workspace.clone())
            .filter(|workspace| workspace.read(cx).project_group_key(cx) == *key)
            .collect()
    }

    pub fn remove_project_group(
        &mut self,
        group_key: &ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<bool>> {
        let retirement = match self.begin_retirement() {
            Ok(retirement) => retirement,
            Err(error) => return Task::ready(Err(error)),
        };
        // The active workspace can remain unpinned while the sidebar is
        // closed. Pin it first: this puts it in the removal set below, and
        // stops `activate` from pinning it while switching to the
        // replacement, which would recreate the project group row this
        // function just deleted.
        let active_workspace = self.workspace().clone();
        if active_workspace.read(cx).project_group_key(cx) == *group_key
            && !self.is_workspace_retained(&active_workspace)
        {
            let index = self.hold(active_workspace, window, cx);
            self.pin(index, group_key.clone(), cx);
        }

        let unloaded_workspace_ids = match self.unloaded_project_group_ids(group_key, cx) {
            Ok(ids) => ids,
            Err(error) => return Task::ready(Err(error)),
        };
        self.remove_workspaces(
            WorkspaceRemoval {
                workspaces: self.workspaces_for_project_group(group_key, cx),
                unloaded_workspace_ids,
                project_groups: vec![group_key.clone()],
                intent: RemovalIntent::CloseProject,
            },
            retirement,
            window,
            cx,
        )
    }

    /// Returns the nearest retained workspace outside the project group at
    /// `group_index`.
    ///
    /// Searches project groups by increasing distance, preferring the following
    /// group over the preceding group at equal distances. Within each group,
    /// prefers its last active workspace before falling back to any retained
    /// workspace. Workspaces in `excluded_workspaces` are ignored by both
    /// lookups.
    ///
    /// The `group_index` must identify a project group that is still present in
    /// [`Self::project_groups`].
    /// The keys of the other project groups, nearest first, preferring the
    /// following group over the preceding group at equal distances. Without an
    /// index, every group key in display order.
    fn neighbor_group_keys(&self, group_index: Option<usize>) -> Vec<ProjectGroupKey> {
        let Some(index) = group_index else {
            return self
                .project_groups
                .iter()
                .map(|group| group.key.clone())
                .collect();
        };

        (1..self.project_groups.len())
            .flat_map(|distance| [index.checked_add(distance), index.checked_sub(distance)])
            .flatten()
            .filter_map(|index| self.project_groups.get(index))
            .map(|group| group.key.clone())
            .collect()
    }

    #[cfg(test)]
    pub(super) fn nearest_retained_workspace(
        &self,
        group_index: usize,
        excluded_workspaces: &[Entity<Workspace>],
        cx: &App,
    ) -> Option<Entity<Workspace>> {
        self.neighbor_group_keys(Some(group_index))
            .into_iter()
            .find_map(|key| self.live_member_for_group(&key, excluded_workspaces, cx))
    }

    /// Goes through sqlite: serialize -> close -> open new window
    /// This avoids issues with pending tasks having the wrong window
    pub fn open_project_group_in_new_window(
        &mut self,
        key: &ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let paths: Vec<PathBuf> = key.path_list().ordered_paths().cloned().collect();
        if paths.is_empty() {
            return Task::ready(Ok(()));
        }

        if key.host().is_some() {
            return Task::ready(Err(anyhow::anyhow!(
                "Moving remote project groups to a new window is not supported"
            )));
        }
        let retirement = match self.begin_retirement() {
            Ok(retirement) => retirement,
            Err(error) => return Task::ready(Err(error)),
        };
        let app_state = self.workspace().read(cx).app_state().clone();
        let key = key.clone();
        cx.spawn_in(window, async move |multi_workspace, cx| {
            let (mut reservations, remove_task) =
                multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                    let active_id = multi_workspace
                        .last_active_workspace_for_group(&key, cx)
                        .and_then(|workspace| workspace.read(cx).database_id());
                    let ids = active_id
                        .into_iter()
                        .chain(
                            multi_workspace
                                .workspaces()
                                .filter(|workspace| workspace.read(cx).project_group_key(cx) == key)
                                .filter_map(|workspace| workspace.read(cx).database_id())
                                .filter(|id| Some(*id) != active_id),
                        )
                        .chain(multi_workspace.unloaded_project_group_ids(&key, cx)?)
                        .collect::<Vec<_>>();
                    let reservations = ids
                        .into_iter()
                        .map(|id| {
                            WorkspaceIdReservation::reserve(id, cx).with_context(|| {
                                format!("Workspace {id:?} is already opening or moving")
                            })
                        })
                        .collect::<Result<Vec<_>>>()?;
                    let removal = WorkspaceRemoval {
                        workspaces: multi_workspace
                            .workspaces()
                            .filter(|workspace| workspace.read(cx).project_group_key(cx) == key)
                            .cloned()
                            .collect(),
                        unloaded_workspace_ids: multi_workspace
                            .unloaded_project_group_ids(&key, cx)?,
                        project_groups: vec![key.clone()],
                        intent: RemovalIntent::MoveProject,
                    };
                    anyhow::Ok((
                        reservations,
                        multi_workspace.remove_workspaces(removal, retirement, window, cx),
                    ))
                })??;
            if !remove_task.await? {
                return Ok(());
            }
            multi_workspace.update(cx, |multi_workspace, cx| {
                anyhow::ensure!(
                    reservations
                        .iter()
                        .all(|reservation| !multi_workspace.owns_workspace_id(reservation.id, cx)),
                    "Project group is still in use in its original window"
                );
                anyhow::Ok(())
            })??;
            for reservation in &mut reservations {
                reservation.renew();
            }
            let source = if reservations.is_empty() {
                LocalWorkspaceSource::History
            } else {
                LocalWorkspaceSource::Reserved(reservations.remove(0))
            };
            let result = cx
                .update(|_, cx| {
                    Workspace::new_local_with_source(
                        paths,
                        source,
                        None,
                        app_state,
                        None,
                        None,
                        None,
                        OpenMode::NewWindow,
                        true,
                        cx,
                    )
                })?
                .await?;
            let flush = result.window.update(cx, |multi_workspace, window, cx| {
                let ids = reservations
                    .iter()
                    .map(|reservation| reservation.id)
                    .collect::<Vec<_>>();
                multi_workspace.restore_workspace_ids(&ids, cx);
                multi_workspace.restore_project_groups(
                    vec![SerializedProjectGroupState {
                        key,
                        expanded: true,
                    }],
                    cx,
                );
                multi_workspace.flush_restoration(window, cx)
            })?;
            flush.await?;
            Ok(())
        })
    }

    /// Finds an existing workspace whose root paths and host exactly match.
    pub fn workspace_for_paths(
        &self,
        path_list: &PathList,
        host: Option<&RemoteConnectionOptions>,
        cx: &App,
    ) -> Option<Entity<Workspace>> {
        for workspace in self.workspaces() {
            let root_paths = PathList::new(&workspace.read(cx).root_paths(cx));
            let key = workspace.read(cx).project_group_key(cx);
            let host_matches = key.host().as_ref() == host;
            let paths_match = root_paths == *path_list;
            if host_matches && paths_match {
                return Some(workspace.clone());
            }
        }

        None
    }

    /// Finds an existing workspace whose paths match, or creates a new one.
    ///
    /// For local projects (`host` is `None`), this delegates to
    /// [`Self::find_or_create_local_workspace`]. For remote projects, it
    /// tries an exact path match and, if no existing workspace is found,
    /// calls `connect_remote` to establish a connection and creates a new
    /// remote workspace.
    ///
    /// The `connect_remote` closure is responsible for any user-facing
    /// connection UI (e.g. password prompts). It receives the connection
    /// options and should return a [`Task`] that resolves to the
    /// [`RemoteClient`] session, or `None` if the connection was
    /// cancelled.
    pub fn find_or_create_workspace(
        &mut self,
        paths: PathList,
        host: Option<RemoteConnectionOptions>,
        provisional_project_group_key: Option<ProjectGroupKey>,
        connect_remote: impl FnOnce(
            RemoteConnectionOptions,
            &mut Window,
            &mut Context<Self>,
        ) -> Task<Result<Option<Entity<remote::RemoteClient>>>>
        + 'static,
        init: Option<Box<dyn FnOnce(&mut Workspace, &mut Window, &mut Context<Workspace>) + Send>>,
        open_mode: OpenMode,
        source_workspace: Option<WeakEntity<Workspace>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Workspace>>> {
        if let Some(workspace) = self.workspace_for_paths(&paths, host.as_ref(), cx) {
            self.activate(workspace.clone(), source_workspace, window, cx);
            return Task::ready(Ok(workspace));
        }

        let Some(connection_options) = host else {
            return self.find_or_create_local_workspace(
                paths,
                provisional_project_group_key,
                init,
                open_mode,
                source_workspace,
                window,
                cx,
            );
        };

        let app_state = self.workspace().read(cx).app_state().clone();
        let window_handle = window.window_handle().downcast::<MultiWorkspace>();
        let connect_task = connect_remote(connection_options.clone(), window, cx);
        let paths_vec = paths.paths().to_vec();

        cx.spawn(async move |_this, cx| {
            let session = connect_task
                .await?
                .ok_or_else(|| anyhow::anyhow!("Remote connection was cancelled"))?;

            let new_project = cx.update(|cx| {
                Project::remote(
                    session,
                    app_state.client.clone(),
                    app_state.node_runtime.clone(),
                    app_state.user_store.clone(),
                    app_state.languages.clone(),
                    app_state.fs.clone(),
                    true,
                    cx,
                )
            });

            let effective_paths_vec =
                if let Some(project_group) = provisional_project_group_key.as_ref() {
                    let resolve_tasks = cx.update(|cx| {
                        let project = new_project.read(cx);
                        paths_vec
                            .iter()
                            .map(|path| project.resolve_abs_path(&path.to_string_lossy(), cx))
                            .collect::<Vec<_>>()
                    });
                    let resolved = futures::future::join_all(resolve_tasks).await;
                    // `resolve_abs_path` returns `None` for both "definitely
                    // absent" and transport errors (it swallows the error via
                    // `log_err`). This is a weaker guarantee than the local
                    // `Ok(None)` check, but it matches how the rest of the
                    // codebase consumes this API.
                    let all_paths_missing =
                        !paths_vec.is_empty() && resolved.iter().all(|resolved| resolved.is_none());

                    if all_paths_missing {
                        project_group.path_list().paths().to_vec()
                    } else {
                        paths_vec
                    }
                } else {
                    paths_vec
                };

            let window_handle =
                window_handle.ok_or_else(|| anyhow::anyhow!("Window is not a MultiWorkspace"))?;

            let (workspace, _items) = open_remote_project_with_existing_connection(
                connection_options,
                new_project,
                effective_paths_vec,
                app_state,
                window_handle,
                provisional_project_group_key,
                source_workspace,
                cx,
            )
            .await?;

            window_handle.update(cx, |multi_workspace, window, cx| {
                multi_workspace.add(workspace.clone(), window, cx);
                workspace
            })
        })
    }

    /// Finds an existing workspace in this multi-workspace whose paths match,
    /// or creates a new one (deserializing its saved state from the database).
    /// Never searches other windows or matches workspaces with a superset of
    /// the requested paths.
    pub fn find_or_create_local_workspace(
        &mut self,
        path_list: PathList,
        project_group: Option<ProjectGroupKey>,
        init: Option<Box<dyn FnOnce(&mut Workspace, &mut Window, &mut Context<Workspace>) + Send>>,
        open_mode: OpenMode,
        source_workspace: Option<WeakEntity<Workspace>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Workspace>>> {
        if open_mode != OpenMode::NewWindow
            && let Some(workspace) = self.workspace_for_paths(&path_list, None, cx)
        {
            self.activate(workspace.clone(), source_workspace, window, cx);
            return Task::ready(Ok(workspace));
        }

        let paths = path_list.paths().to_vec();
        let app_state = self.workspace().read(cx).app_state().clone();
        let requesting_window = window.window_handle().downcast::<MultiWorkspace>();
        let fs = <dyn Fs>::global(cx);

        cx.spawn(async move |_this, cx| {
            let effective_path_list = if let Some(project_group) = project_group {
                let metadata_tasks: Vec<_> = paths
                    .iter()
                    .map(|path| fs.metadata(path.as_path()))
                    .collect();
                let metadata_results = futures::future::join_all(metadata_tasks).await;
                // Only fall back when every path is definitely absent; real
                // filesystem errors should not be treated as "missing".
                let all_paths_missing = !paths.is_empty()
                    && metadata_results
                        .into_iter()
                        // Ok(None) means the path is definitely absent
                        .all(|result| matches!(result, Ok(None)));

                if all_paths_missing {
                    project_group.path_list().clone()
                } else {
                    PathList::new(&paths)
                }
            } else {
                PathList::new(&paths)
            };

            if open_mode != OpenMode::NewWindow
                && let Some(requesting_window) = requesting_window
                && let Some(workspace) = requesting_window
                    .update(cx, |multi_workspace, window, cx| {
                        multi_workspace
                            .workspace_for_paths(&effective_path_list, None, cx)
                            .inspect(|workspace| {
                                multi_workspace.activate(
                                    workspace.clone(),
                                    source_workspace.clone(),
                                    window,
                                    cx,
                                );
                            })
                    })
                    .ok()
                    .flatten()
            {
                return Ok(workspace);
            }

            let result = cx
                .update(|cx| {
                    Workspace::new_local(
                        effective_path_list.paths().to_vec(),
                        app_state,
                        requesting_window,
                        None,
                        init,
                        open_mode,
                        cx,
                    )
                })
                .await?;
            Ok(result.workspace)
        })
    }

    pub fn workspace(&self) -> &Entity<Workspace> {
        &self.held[self.displayed_index()].workspace
    }

    pub fn workspaces(&self) -> impl Iterator<Item = &Entity<Workspace>> {
        self.held.iter().map(|held| &held.workspace)
    }

    /// Adds a workspace to this window as persistent without changing which
    /// workspace is active. Unlike `activate()`, this always inserts into the
    /// persistent list regardless of sidebar state — it's used for system-
    /// initiated additions like deserialization and worktree discovery.
    pub fn add(&mut self, workspace: Entity<Workspace>, window: &Window, cx: &mut Context<Self>) {
        if self.is_workspace_retained(&workspace) {
            return;
        }
        let key = workspace.read(cx).project_group_key(cx);
        let index = self.hold(workspace, window, cx);
        self.pin(index, key, cx);
        self.serialize(cx);
        telemetry::event!(
            "Workspace Added",
            workspace_count = self.held.iter().filter(|held| held.pinned).count()
        );
        cx.notify();
    }

    /// Ensures the workspace is in the multiworkspace and makes it the active one.
    pub fn activate(
        &mut self,
        workspace: Entity<Workspace>,
        source_workspace: Option<WeakEntity<Workspace>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.workspace() == &workspace {
            self.focus_active_workspace(window, cx);
            return;
        }

        let old_active_workspace = self.workspace().clone();
        let old_active_was_retained = self.active_workspace_is_retained();
        let should_retain_workspaces = self.multi_workspace_enabled(cx);

        if should_retain_workspaces && !old_active_was_retained {
            let key = old_active_workspace.read(cx).project_group_key(cx);
            let index = self.hold(old_active_workspace, window, cx);
            self.pin(index, key, cx);
        }

        let displayed = self.hold(workspace.clone(), window, cx);
        if should_retain_workspaces {
            let key = workspace.read(cx).project_group_key(cx);
            self.pin(displayed, key, cx);
        }

        // Publish the new active workspace before anyone reads the shared cell
        // to decide who owns the window chrome.
        self.active_workspace_id.set(workspace.entity_id());

        let stamp = self
            .held
            .iter()
            .filter_map(|held| held.activated_at)
            .max()
            .map_or(0, |max| max + 1);
        self.held[displayed].activated_at = Some(stamp);

        if !should_retain_workspaces && !old_active_was_retained {
            self.collapse_to_single_workspace(window, cx);
        }

        // The platform window is shared across all workspaces in this window.
        // The previously-active workspace left the title and edited indicator
        // reflecting its own state, so re-apply them from the newly-active
        // workspace (which is now the chrome owner per `owns_window_chrome`).
        workspace.update(cx, |workspace, cx| {
            workspace.refresh_window_state(window, cx);
        });

        cx.emit(MultiWorkspaceEvent::ActiveWorkspaceChanged { source_workspace });
        self.serialize(cx);
        self.focus_active_workspace(window, cx);
        cx.notify();
    }

    /// Promotes the currently active workspace to persistent if it is
    /// transient, so it is retained across workspace switches even when
    /// the sidebar is closed. No-op if the workspace is already persistent.
    pub fn retain_active_workspace(&mut self, cx: &mut Context<Self>) {
        let index = self.displayed_index();
        if self.held[index].pinned {
            return;
        }
        let key = self.held[index].workspace.read(cx).project_group_key(cx);
        self.pin(index, key, cx);
        self.serialize(cx);
        cx.notify();
    }

    /// Collapses to a single workspace, discarding all groups.
    /// Used when multi-workspace is disabled by settings.
    fn collapse_to_single_workspace(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let retirement = match self.begin_retirement() {
            Ok(retirement) => retirement,
            Err(error) => {
                self.workspace().update(cx, |workspace, cx| {
                    workspace.show_error(error, cx);
                });
                return;
            }
        };
        if self.sidebar_open {
            self.close_sidebar(window, cx);
        }

        let displayed_workspace = self.workspace().clone();
        let workspaces = self.workspaces().cloned().collect::<Vec<_>>();
        for workspace in &workspaces {
            self.add(workspace.clone(), window, cx);
        }
        let workspaces = workspaces
            .into_iter()
            .filter(|workspace| workspace != &displayed_workspace)
            .collect::<Vec<_>>();
        let remove_task = self.remove_workspaces(
            WorkspaceRemoval {
                workspaces,
                unloaded_workspace_ids: self.unloaded_workspace_ids.clone(),
                project_groups: self
                    .project_groups
                    .iter()
                    .map(|group| group.key.clone())
                    .collect(),
                intent: RemovalIntent::CloseProject,
            },
            retirement.clone(),
            window,
            cx,
        );
        let cancelled = retirement.cancelled.clone();
        self._collapse_task = Some((
            cx.spawn_in(window, async move |multi_workspace, cx| {
                let result = remove_task.await;
                multi_workspace
                    .update(cx, |multi_workspace, cx| match result {
                        Ok(true)
                            if !multi_workspace.multi_workspace_enabled(cx)
                                && multi_workspace.held.len() == 1 =>
                        {
                            for held in &mut multi_workspace.held {
                                held.pinned = false;
                            }
                            multi_workspace.project_groups.clear();
                            multi_workspace.serialize(cx);
                            cx.notify();
                        }
                        Ok(_) => {}
                        Err(error) => {
                            multi_workspace.workspace().update(cx, |workspace, cx| {
                                workspace.show_error(error, cx);
                            });
                        }
                    })
                    .ok();
                drop(retirement);
            }),
            cancelled,
        ));
    }

    fn release_workspace(&mut self, workspace: &Entity<Workspace>, cx: &mut Context<Self>) {
        self.invalidate_serialization();
        if let Some(index) = self.held_index(workspace) {
            assert_ne!(
                index,
                self.displayed_index(),
                "the displayed workspace must be re-pointed before it is detached"
            );
            self.held.remove(index);
        }
        cx.emit(MultiWorkspaceEvent::WorkspaceRemoved(workspace.entity_id()));
        workspace.update(cx, |workspace, _cx| {
            workspace
                .restoration_cancelled
                .store(true, std::sync::atomic::Ordering::Relaxed);
            workspace.session_id.take();
            workspace._schedule_serialize_workspace.take();
            workspace._serialize_workspace_task.take();
        });
    }

    fn sync_sidebar_to_workspace(&self, workspace: &Entity<Workspace>, cx: &mut Context<Self>) {
        if self.sidebar_open() {
            let sidebar_focus_handle = self.sidebar.as_ref().map(|s| s.focus_handle(cx));
            workspace.update(cx, |workspace, _| {
                workspace.set_sidebar_focus_handle(sidebar_focus_handle);
            });
        }
    }

    pub(crate) fn restore_workspace_ids(&mut self, ids: &[WorkspaceId], cx: &mut Context<Self>) {
        for &id in ids {
            if !self.owns_workspace_id(id, cx) {
                self.unloaded_workspace_ids.push(id);
            }
        }
        self.serialize(cx);
    }

    pub(crate) fn workspace_ids(&self, cx: &App) -> Vec<WorkspaceId> {
        self.workspaces()
            .filter_map(|workspace| workspace.read(cx).database_id())
            .chain(self.unloaded_workspace_ids.iter().copied())
            .collect()
    }

    pub(crate) fn unloaded_workspace_ids(&self) -> &[WorkspaceId] {
        &self.unloaded_workspace_ids
    }

    pub(crate) fn owns_workspace_id(&self, id: WorkspaceId, cx: &App) -> bool {
        self.unloaded_workspace_ids.contains(&id)
            || self
                .workspaces()
                .any(|workspace| workspace.read(cx).database_id() == Some(id))
    }

    pub(crate) fn unloaded_workspace_for_paths(
        &self,
        paths: &PathList,
        location: &SerializedWorkspaceLocation,
        cx: &App,
    ) -> Option<SerializedWorkspace> {
        let db = WorkspaceDb::global(cx);
        self.unloaded_workspace_ids.iter().find_map(|id| {
            db.workspace_for_id(*id).filter(|workspace| {
                workspace.paths == *paths
                    && match (&workspace.location, location) {
                        (
                            SerializedWorkspaceLocation::Local,
                            SerializedWorkspaceLocation::Local,
                        ) => true,
                        (
                            SerializedWorkspaceLocation::Remote(saved),
                            SerializedWorkspaceLocation::Remote(requested),
                        ) => crate::same_remote_connection_identity(Some(saved), Some(requested)),
                        _ => false,
                    }
            })
        })
    }

    fn unloaded_project_group_ids(
        &self,
        key: &ProjectGroupKey,
        cx: &App,
    ) -> Result<Vec<WorkspaceId>> {
        let db = WorkspaceDb::global(cx);
        let mut ids = Vec::new();
        for id in &self.unloaded_workspace_ids {
            let workspace = db
                .workspace_for_id(*id)
                .with_context(|| format!("Could not read unloaded workspace {id:?}"))?;
            let host = match workspace.location {
                SerializedWorkspaceLocation::Local => None,
                SerializedWorkspaceLocation::Remote(host) => Some(host),
            };
            if ProjectGroupKey::new(host, workspace.identity_paths.unwrap_or(workspace.paths))
                == *key
            {
                ids.push(*id);
            }
        }
        Ok(ids)
    }

    pub(crate) fn invalidate_serialization(&mut self) {
        self.serialization_cancelled.store(true, Ordering::Relaxed);
        self.serialization_cancelled = Arc::new(AtomicBool::new(false));
    }

    pub(crate) fn resume_serialization(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.serialization_fence.store(true, Ordering::Relaxed);
        self.serialization_fence = Arc::new(AtomicBool::new(false));
        for workspace in self.workspaces() {
            workspace.update(cx, |workspace, cx| {
                workspace.serialization_fence = self.serialization_fence.clone();
                workspace.serialize_workspace(window, cx);
            });
        }
        self.serialize(cx);
    }

    pub fn serialize(&mut self, cx: &mut Context<Self>) {
        self.invalidate_serialization();
        self._serialize_task = Some(cx.spawn(async move |this, cx| {
            let Ok(task) = this.update(cx, |this, cx| this.serialize_now(false, cx)) else {
                return;
            };
            task.await.log_err();
        }));
    }

    fn serialize_now(
        &self,
        completing_restore: bool,
        cx: &App,
    ) -> impl Future<Output = Result<bool>> + use<> {
        let state = MultiWorkspaceState {
            active_workspace_id: self.workspace().read(cx).database_id(),
            project_groups: self
                .project_groups
                .iter()
                .map(|group| {
                    crate::persistence::model::SerializedProjectGroup::from_group(
                        &group.key,
                        group.expanded,
                    )
                })
                .collect::<Vec<_>>(),
            sidebar_open: self.sidebar_open,
            sidebar_state: self.sidebar.as_ref().and_then(|s| s.serialized_state(cx)),
        };
        let window_id = self.window_id;
        let session_id = self.workspace().read(cx).session_id();
        let cancelled = self.serialization_cancelled.clone();
        let serialization_fence = self.serialization_fence.clone();
        let workspace_ids = self.workspace_ids(cx);
        let restore_key = self.restore_key;
        let db = WorkspaceDb::global(cx);
        async move {
            db.save_multi_workspace(
                window_id,
                state,
                workspace_ids,
                session_id,
                cancelled,
                serialization_fence,
                restore_key,
                completing_restore,
            )
            .await
        }
    }

    pub fn flush_restoration(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.restoration_ready = true;
        self.flush_pending_serialization(window, cx)
    }

    pub fn flush_pending_serialization(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let completing_restore = self.restoration_ready
            && self.workspace().read(cx).session_id().is_some()
            && self
                .workspaces()
                .all(|workspace| !workspace.read(cx).is_restoring());
        self.flush_serialization_internal(completing_restore, window, cx)
    }

    pub(crate) fn flush_serialization_for_close(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.flush_serialization_internal(false, window, cx)
    }

    pub(crate) fn begin_retirement(&mut self) -> Result<Rc<WorkspaceRetirement>> {
        anyhow::ensure!(
            self.retirement.upgrade().is_none(),
            "Workspace retirement is already in progress"
        );
        let retirement = Rc::new(WorkspaceRetirement {
            cancelled: Arc::new(AtomicBool::new(false)),
            closing_window: Cell::new(false),
        });
        self.retirement = Rc::downgrade(&retirement);
        Ok(retirement)
    }

    pub(crate) fn is_closing_window(&self) -> bool {
        self.retirement
            .upgrade()
            .is_some_and(|retirement| retirement.closing_window.get())
    }

    pub(crate) fn serialization_guard(&self, cx: &App) -> impl Fn() -> Result<()> + Send + use<> {
        self.serialization_guard_for(&self.workspaces().cloned().collect::<Vec<_>>(), cx)
    }

    fn serialization_guard_for(
        &self,
        workspaces: &[Entity<Workspace>],
        cx: &App,
    ) -> impl Fn() -> Result<()> + Send + use<> {
        let cancelled = self.serialization_cancelled.clone();
        let guards = workspaces
            .iter()
            .map(|workspace| workspace.read(cx).serialization_guard(cx))
            .collect::<Vec<_>>();
        move || {
            anyhow::ensure!(
                !cancelled.load(Ordering::Relaxed),
                "Workspace membership changed during serialization flush"
            );
            anyhow::ensure!(
                guards.iter().all(|guard| guard()),
                "Workspace changed during serialization flush"
            );
            Ok(())
        }
    }

    fn flush_serialization_internal(
        &mut self,
        completing_restore: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.invalidate_serialization();
        self._serialize_task.take();
        let tasks = self
            .workspaces()
            .map(|workspace| {
                workspace.update(cx, |member, cx| {
                    member.flush_serialization_checked(window, cx)
                })
            })
            .collect::<Vec<_>>();
        let serialization = self.serialize_now(completing_restore, cx);
        let unchanged = self.serialization_guard(cx);
        cx.background_spawn(async move {
            futures::future::join_all(tasks)
                .await
                .into_iter()
                .collect::<Result<Vec<_>>>()?;
            unchanged()?;
            anyhow::ensure!(
                serialization.await?,
                "Workspace membership changed during serialization flush"
            );
            unchanged()
        })
    }

    pub fn focus_active_workspace(&self, window: &mut Window, cx: &mut App) {
        let focus_handle = self.workspace().read(cx).fallback_focus_handle(window, cx);
        window.focus(&focus_handle, cx);
    }

    pub fn panel<T: Panel>(&self, cx: &App) -> Option<Entity<T>> {
        self.workspace().read(cx).panel::<T>(cx)
    }

    pub fn active_modal<V: ManagedView + 'static>(&self, cx: &App) -> Option<Entity<V>> {
        self.workspace().read(cx).active_modal::<V>(cx)
    }

    pub fn add_panel<T: Panel>(
        &mut self,
        panel: Entity<T>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.workspace().update(cx, |workspace, cx| {
            workspace.add_panel(panel, window, cx);
        });
    }

    pub fn focus_panel<T: Panel>(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<T>> {
        self.workspace()
            .update(cx, |workspace, cx| workspace.focus_panel::<T>(window, cx))
    }

    // used in a test
    pub fn toggle_modal<V: ModalView, B>(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        build: B,
    ) where
        B: FnOnce(&mut Window, &mut gpui::Context<V>) -> V,
    {
        self.workspace().update(cx, |workspace, cx| {
            workspace.toggle_modal(window, cx, build);
        });
    }

    pub fn toggle_dock(
        &mut self,
        dock_side: DockPosition,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.workspace().update(cx, |workspace, cx| {
            workspace.toggle_dock(dock_side, window, cx);
        });
    }

    pub fn active_item_as<I: 'static>(&self, cx: &App) -> Option<Entity<I>> {
        self.workspace().read(cx).active_item_as::<I>(cx)
    }

    pub fn items_of_type<'a, T: Item>(
        &'a self,
        cx: &'a App,
    ) -> impl 'a + Iterator<Item = Entity<T>> {
        self.workspace().read(cx).items_of_type::<T>(cx)
    }

    pub fn database_id(&self, cx: &App) -> Option<WorkspaceId> {
        self.workspace().read(cx).database_id()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_expand_all_groups(&mut self) {
        self.set_all_groups_expanded(true);
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn assert_project_group_key_integrity(&self, cx: &App) -> anyhow::Result<()> {
        let mut retained_ids: collections::HashSet<EntityId> = Default::default();
        for workspace in self
            .held
            .iter()
            .filter(|held| held.pinned)
            .map(|held| &held.workspace)
        {
            anyhow::ensure!(
                retained_ids.insert(workspace.entity_id()),
                "workspace {:?} is retained more than once",
                workspace.entity_id(),
            );

            let live_key = workspace.read(cx).project_group_key(cx);
            anyhow::ensure!(
                self.project_groups
                    .iter()
                    .any(|group| group.key == live_key),
                "workspace {:?} has live key {:?} but no project-group metadata",
                workspace.entity_id(),
                live_key,
            );
        }
        Ok(())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_random_database_id(&mut self, cx: &mut Context<Self>) {
        self.workspace().update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
        Self::new(workspace, window, cx)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_add_workspace(
        &mut self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<Workspace> {
        let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
        self.activate(workspace.clone(), None, window, cx);
        workspace
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_add_project_group(&mut self, group: ProjectGroup) {
        self.project_groups.push(ProjectGroupState {
            key: group.key,
            expanded: group.expanded,
        });
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn create_test_workspace(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let app_state = self.workspace().read(cx).app_state().clone();
        let project = Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            project::LocalProjectFlags::default(),
            cx,
        );
        let new_workspace = cx.new(|cx| Workspace::new(None, project, app_state, window, cx));
        self.activate(new_workspace.clone(), None, window, cx);

        let weak_workspace = new_workspace.downgrade();
        let db = crate::persistence::WorkspaceDb::global(cx);
        cx.spawn_in(window, async move |this, cx| {
            let workspace_id = db.next_id().await.unwrap();
            let workspace = weak_workspace.upgrade().unwrap();
            let task: Task<()> = this
                .update_in(cx, |this, window, cx| {
                    let session_id = workspace.read(cx).session_id();
                    let window_id = window.window_handle().window_id().as_u64();
                    workspace.update(cx, |workspace, _cx| {
                        workspace.set_database_id(workspace_id);
                    });
                    this.serialize(cx);
                    let db = db.clone();
                    cx.background_spawn(async move {
                        db.set_session_binding(workspace_id, session_id, Some(window_id), None)
                            .await
                            .log_err();
                    })
                })
                .unwrap();
            task.await
        })
    }

    /// Assigns random database IDs to all retained workspaces, flushes
    /// workspace serialization (SQLite) and multi-workspace state (KVP),
    /// and writes session bindings so the serialized data can be read
    /// back by `last_session_workspace_locations` +
    /// `read_serialized_multi_workspaces`.
    #[cfg(any(test, feature = "test-support"))]
    pub fn flush_all_serialization(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Task<()>> {
        let workspace_ids = self
            .workspaces()
            .map(|workspace| {
                workspace.update(cx, |workspace, _| {
                    if workspace.database_id().is_none() {
                        workspace.set_random_database_id();
                    }
                    workspace.database_id().unwrap()
                })
            })
            .collect::<Vec<_>>();
        let db = WorkspaceDb::global(cx);
        vec![cx.spawn_in(window, async move |multi_workspace, cx| {
            db.write(move |connection| {
                let mut reserve = connection.exec_bound(
                    "INSERT INTO workspaces (workspace_id) VALUES (?) ON CONFLICT(workspace_id) DO NOTHING",
                )?;
                for workspace_id in workspace_ids {
                    reserve(workspace_id)?;
                }
                anyhow::Ok(())
            })
            .await
            .unwrap();
            let flush = multi_workspace
                .update_in(cx, |multi_workspace, window, cx| {
                    multi_workspace.flush_pending_serialization(window, cx)
                })
                .unwrap();
            flush.await.unwrap();
        })]
    }

    /// The best replacement candidate within one project group: its most
    /// recently displayed member, else its first pinned member, skipping
    /// `excluding` and disconnected projects.
    fn live_member_for_group(
        &self,
        key: &ProjectGroupKey,
        excluding: &[Entity<Workspace>],
        cx: &App,
    ) -> Option<Entity<Workspace>> {
        let available = |workspace: &Entity<Workspace>| {
            !excluding.contains(workspace)
                && !workspace.read(cx).project().read(cx).is_disconnected(cx)
        };
        self.last_active_workspace_for_group(key, cx)
            .filter(&available)
            .or_else(|| {
                self.held
                    .iter()
                    .filter(|held| held.pinned)
                    .map(|held| held.workspace.clone())
                    .filter(|workspace| workspace.read(cx).project_group_key(cx) == *key)
                    .find(available)
            })
    }

    /// Removes one or more workspaces from this multi-workspace.
    ///
    /// Every workspace is first asked for consent (save prompts); no state
    /// changes until all consent. The rows are then deleted and, when the
    /// displayed workspace was among them, a replacement is chosen from what
    /// remains: another workspace in the same project, then a workspace in the
    /// nearest neighboring project, then an empty workspace. When the intent is
    /// `KeepProject` and the project has no other workspace, its root worktrees
    /// are reopened afterwards; the same applies to the adjacent local project
    /// when nothing at all remains.
    ///
    /// Returns `true` if any workspaces were actually removed.
    pub fn remove(
        &mut self,
        workspaces: impl IntoIterator<Item = Entity<Workspace>>,
        intent: RemovalIntent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<bool>> {
        let workspaces = workspaces.into_iter().collect::<Vec<_>>();
        if workspaces.is_empty() {
            return Task::ready(Ok(false));
        }
        let retirement = match self.begin_retirement() {
            Ok(retirement) => retirement,
            Err(error) => return Task::ready(Err(error)),
        };
        let project_groups = if intent != RemovalIntent::KeepProject {
            workspaces
                .iter()
                .map(|workspace| workspace.read(cx).project_group_key(cx))
                .filter(|key| {
                    self.workspaces().all(|workspace| {
                        workspaces.contains(workspace)
                            || workspace.read(cx).project_group_key(cx) != *key
                    })
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        let mut unloaded_workspace_ids = Vec::new();
        for key in &project_groups {
            match self.unloaded_project_group_ids(key, cx) {
                Ok(ids) => unloaded_workspace_ids.extend(ids),
                Err(error) => return Task::ready(Err(error)),
            }
        }
        unloaded_workspace_ids.sort_unstable();
        unloaded_workspace_ids.dedup();
        self.remove_workspaces(
            WorkspaceRemoval {
                workspaces,
                unloaded_workspace_ids,
                project_groups,
                intent,
            },
            retirement,
            window,
            cx,
        )
    }

    fn remove_workspaces(
        &mut self,
        removal: WorkspaceRemoval,
        retirement: Rc<WorkspaceRetirement>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<bool>> {
        let original_active = self.workspace().clone();
        let held = self.workspaces().cloned().collect::<Vec<_>>();
        let unloaded = self.unloaded_workspace_ids.clone();
        cx.spawn_in(window, async move |multi_workspace, cx| {
            if !Self::confirm_workspace_removal(&removal, cx).await? {
                return Ok(false);
            }
            anyhow::ensure!(
                !retirement.cancelled.load(Ordering::Relaxed),
                "Workspace removal was cancelled"
            );
            multi_workspace
                .update_in(cx, |multi_workspace, window, cx| {
                    anyhow::ensure!(
                        multi_workspace.unloaded_workspace_ids == unloaded
                            && multi_workspace.workspaces().count() == held.len()
                            && multi_workspace
                                .workspaces()
                                .all(|workspace| held.contains(workspace)),
                        "Workspace membership changed while preparing to close"
                    );
                    anyhow::Ok(multi_workspace.remove_confirmed_workspaces(
                        removal,
                        retirement,
                        original_active,
                        window,
                        cx,
                    ))
                })??
                .await
        })
    }

    fn remove_confirmed_workspaces(
        &mut self,
        removal: WorkspaceRemoval,
        retirement: Rc<WorkspaceRetirement>,
        original_active: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<bool>> {
        if removal
            .workspaces
            .iter()
            .any(|workspace| self.held_index(workspace).is_none())
            || removal
                .unloaded_workspace_ids
                .iter()
                .any(|id| !self.unloaded_workspace_ids.contains(id))
        {
            return Task::ready(Err(anyhow::anyhow!(
                "Workspace membership changed while preparing to close"
            )));
        }
        let WorkspaceRemoval {
            workspaces,
            unloaded_workspace_ids,
            project_groups,
            intent,
        } = removal;
        let workspace_ids = workspaces
            .iter()
            .filter_map(|workspace| workspace.read(cx).database_id())
            .chain(unloaded_workspace_ids.iter().copied())
            .collect::<Vec<_>>();
        self.invalidate_serialization();
        self._serialize_task.take();
        let flushes = workspaces
            .iter()
            .map(|workspace| {
                workspace.update(cx, |workspace, cx| {
                    workspace.flush_serialization_checked(window, cx)
                })
            })
            .collect::<Vec<_>>();
        let unchanged = self.serialization_guard_for(&workspaces, cx);
        let retirement_guard = self.serialization_guard_for(&workspaces, cx);
        let serialization_fence = self.serialization_fence.clone();
        let db = WorkspaceDb::global(cx);
        let group_key = original_active.read(cx).project_group_key(cx);

        let group_index = self
            .project_groups
            .iter()
            .position(|group| group.key == group_key);
        let neighbor_keys = self.neighbor_group_keys(group_index);
        let adjacent_key = group_index.and_then(|index| {
            self.project_groups
                .get(index + 1)
                .or_else(|| {
                    index
                        .checked_sub(1)
                        .and_then(|previous| self.project_groups.get(previous))
                })
                .map(|group| group.key.clone())
        });

        cx.spawn_in(window, async move |multi_workspace, cx| {
            futures::future::try_join_all(flushes).await?;
            unchanged()?;
            let cancelled = retirement.cancelled.clone();
            let cancel_on_drop = util::defer({
                let cancelled = cancelled.clone();
                move || cancelled.store(true, Ordering::Relaxed)
            });
            let (completed, completion) = oneshot::channel();
            cx.spawn({
                let multi_workspace = multi_workspace.clone();
                async move |cx| {
                    let result = async {
                        let retirement_cancelled = cancelled.clone();
                        let snapshot = db
                            .close_workspaces(
                                None,
                                workspace_ids.clone(),
                                project_groups.clone(),
                                serialization_fence,
                                move || {
                                    anyhow::ensure!(
                                        !retirement_cancelled.load(Ordering::Relaxed),
                                        "Workspace removal was cancelled"
                                    );
                                    retirement_guard()
                                },
                            )
                            .await?;
                        let completion = multi_workspace
                            .update_in(cx, |multi_workspace, window, cx| {
                                anyhow::ensure!(
                                    !cancelled.load(Ordering::Relaxed),
                                    "Workspace removal was cancelled"
                                );
                                unchanged()?;
                                let mut removed_any = !unloaded_workspace_ids.is_empty()
                                    || multi_workspace
                                        .project_groups
                                        .iter()
                                        .any(|group| project_groups.contains(&group.key))
                                    || (workspaces.is_empty() && project_groups.is_empty());
                                let displayed_workspace = multi_workspace.workspace().clone();
                                multi_workspace
                                    .unloaded_workspace_ids
                                    .retain(|id| !unloaded_workspace_ids.contains(id));
                                WorkspaceIdReservation::cancel(&workspace_ids, cx);
                                if let Some(state) = cx
                                    .try_global::<PendingWorkspaceIds>()
                                    .and_then(|pending| pending.restore_plan.upgrade())
                                {
                                    for members in state.workspaces.borrow_mut().values_mut() {
                                        members.retain(|id| !workspace_ids.contains(id));
                                    }
                                }
                                if !project_groups.is_empty() {
                                    multi_workspace
                                        .project_groups
                                        .retain(|group| !project_groups.contains(&group.key));
                                    cx.emit(MultiWorkspaceEvent::ProjectGroupsChanged);
                                }

                                for workspace in &workspaces {
                                    if *workspace == displayed_workspace {
                                        continue;
                                    }
                                    if multi_workspace.held_index(workspace).is_some() {
                                        multi_workspace.release_workspace(workspace, cx);
                                        removed_any = true;
                                    }
                                }

                                let mut reopen_key = None;
                                if workspaces.contains(&displayed_workspace) {
                                    let doomed = std::slice::from_ref(&displayed_workspace);

                                    let same_group = multi_workspace
                                        .held
                                        .iter()
                                        .filter(|held| {
                                            held.pinned && held.workspace != displayed_workspace
                                        })
                                        .map(|held| held.workspace.clone())
                                        .find(|workspace| {
                                            workspace.read(cx).project_group_key(cx) == group_key
                                        });
                                    if intent == RemovalIntent::KeepProject
                                        && same_group.is_none()
                                        && group_key.host().is_none()
                                        && !group_key.path_list().is_empty()
                                    {
                                        reopen_key = Some(group_key.clone());
                                    }

                                    let replacement = same_group
                                        .or_else(|| {
                                            neighbor_keys.iter().find_map(|key| {
                                                multi_workspace
                                                    .live_member_for_group(key, doomed, cx)
                                            })
                                        })
                                        .unwrap_or_else(|| {
                                            if reopen_key.is_none() {
                                                reopen_key = adjacent_key.clone().filter(|key| {
                                                    key.host().is_none()
                                                        && !key.path_list().is_empty()
                                                });
                                            }
                                            let app_state =
                                                displayed_workspace.read(cx).app_state().clone();
                                            let project = Project::local(
                                                app_state.client.clone(),
                                                app_state.node_runtime.clone(),
                                                app_state.user_store.clone(),
                                                app_state.languages.clone(),
                                                app_state.fs.clone(),
                                                None,
                                                project::LocalProjectFlags::default(),
                                                cx,
                                            );
                                            cx.new(|cx| {
                                                Workspace::new(None, project, app_state, window, cx)
                                            })
                                        });

                                    let displayed_index = multi_workspace.displayed_index();
                                    multi_workspace.held[displayed_index].pinned = true;
                                    multi_workspace.activate(replacement, None, window, cx);
                                    multi_workspace.release_workspace(&displayed_workspace, cx);
                                    removed_any = true;
                                } else if *multi_workspace.workspace() != original_active
                                    && !workspaces.contains(&original_active)
                                {
                                    multi_workspace.activate(
                                        original_active.clone(),
                                        None,
                                        window,
                                        cx,
                                    );
                                }

                                multi_workspace.resume_serialization(window, cx);
                                if removed_any {
                                    cx.notify();
                                }

                                anyhow::Ok((removed_any, reopen_key))
                            })
                            .and_then(|result| result);
                        match completion {
                            Ok(result) => Ok(result),
                            Err(error) => {
                                let retained_ids = multi_workspace
                                    .read_with(cx, |multi_workspace, cx| {
                                        multi_workspace.workspace_ids(cx)
                                    })
                                    .unwrap_or_else(|_| snapshot.workspace_ids.clone());
                                let rollback = db
                                    .restore_workspace_close(snapshot, retained_ids)
                                    .await
                                    .with_context(|| {
                                        format!("Rolling back workspace removal: {error:#}")
                                    });
                                multi_workspace
                                    .update_in(cx, |multi_workspace, window, cx| {
                                        multi_workspace.resume_serialization(window, cx);
                                    })
                                    .ok();
                                rollback?;
                                Err(error)
                            }
                        }
                    }
                    .await;
                    if let Err(result) = completed.send(result) {
                        result.log_err();
                    }
                    drop(retirement);
                }
            })
            .detach();
            let (removed_any, reopen_key) = completion
                .await
                .context("Workspace retirement task did not complete")??;
            drop(cancel_on_drop);
            if let Some(key) = reopen_key {
                multi_workspace
                    .update_in(cx, |multi_workspace, window, cx| {
                        multi_workspace.find_or_create_local_workspace(
                            key.path_list().clone(),
                            Some(key),
                            None,
                            OpenMode::Activate,
                            None,
                            window,
                            cx,
                        )
                    })?
                    .await?;
            }

            Ok(removed_any)
        })
    }

    pub fn open_project(
        &mut self,
        paths: Vec<PathBuf>,
        open_mode: OpenMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Workspace>>> {
        let workspace = self.workspace().clone();
        let multi_workspace_enabled = self.multi_workspace_enabled(cx);
        let replace_workspace = open_mode == OpenMode::Activate
            && (!multi_workspace_enabled
                || workspace
                    .read(cx)
                    .project()
                    .read(cx)
                    .visible_worktrees(cx)
                    .next()
                    .is_none());
        let retirement = if replace_workspace {
            match self.begin_retirement() {
                Ok(retirement) => Some(retirement),
                Err(error) => return Task::ready(Err(error)),
            }
        } else {
            None
        };
        cx.spawn_in(window, async move |multi_workspace, cx| {
            let removal = WorkspaceRemoval {
                workspaces: vec![workspace.clone()],
                unloaded_workspace_ids: Vec::new(),
                project_groups: Vec::new(),
                intent: RemovalIntent::KeepProject,
            };
            if replace_workspace && !Self::confirm_workspace_removal(&removal, cx).await? {
                return Ok(workspace);
            }
            let previous_state = workspace.read_with(cx, |workspace, cx| {
                (
                    workspace.serialization_revision(cx),
                    workspace
                        .items(cx)
                        .map(|item| item.item_id())
                        .collect::<Vec<_>>(),
                )
            });
            let new_workspace = multi_workspace
                .update_in(cx, |multi_workspace, window, cx| {
                    if replace_workspace && let Some(index) = multi_workspace.held_index(&workspace)
                    {
                        multi_workspace.held[index].pinned = true;
                    }
                    if multi_workspace_enabled || open_mode == OpenMode::NewWindow {
                        multi_workspace.find_or_create_local_workspace(
                            PathList::new(&paths),
                            None,
                            None,
                            open_mode,
                            None,
                            window,
                            cx,
                        )
                    } else {
                        workspace.update(cx, |workspace, cx| {
                            workspace.open_workspace_for_paths(open_mode, paths, window, cx)
                        })
                    }
                })?
                .await?;
            if let Some(retirement) = retirement
                && workspace != new_workspace
            {
                multi_workspace
                    .update_in(cx, |multi_workspace, window, cx| {
                        let current_state = workspace.read_with(cx, |workspace, cx| {
                            (
                                workspace.serialization_revision(cx),
                                workspace
                                    .items(cx)
                                    .map(|item| item.item_id())
                                    .collect::<Vec<_>>(),
                            )
                        });
                        anyhow::ensure!(
                            previous_state == current_state,
                            "Previous workspace changed while opening its replacement"
                        );
                        anyhow::Ok(multi_workspace.remove_confirmed_workspaces(
                            removal, retirement, workspace, window, cx,
                        ))
                    })??
                    .await?;
            }
            Ok(new_workspace)
        })
    }

    async fn confirm_workspace_removal(
        removal: &WorkspaceRemoval,
        cx: &mut AsyncWindowContext,
    ) -> Result<bool> {
        if removal.intent != RemovalIntent::MoveProject
            && !cx
                .update(|window, cx| {
                    crate::confirm_workspace_close(
                        &removal.workspaces,
                        &removal.unloaded_workspace_ids,
                        window,
                        cx,
                    )
                })?
                .await?
        {
            return Ok(false);
        }
        for workspace in &removal.workspaces {
            if !workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace.prepare_to_close_internal(CloseIntent::ReplaceWindow, window, cx)
                })?
                .await?
            {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

impl Render for MultiWorkspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let multi_workspace_enabled = self.multi_workspace_enabled(cx);
        let sidebar_side = self.sidebar_side(cx);
        let sidebar_on_right = sidebar_side == SidebarSide::Right;

        let sidebar: Option<AnyElement> = if multi_workspace_enabled && self.sidebar_open() {
            self.sidebar.as_ref().map(|sidebar_handle| {
                let weak = cx.weak_entity();

                let sidebar_width = sidebar_handle.width(cx);
                let resize_handle = deferred(
                    div()
                        .id("sidebar-resize-handle")
                        .absolute()
                        .when(!sidebar_on_right, |el| {
                            el.right(-SIDEBAR_RESIZE_HANDLE_SIZE / 2.)
                        })
                        .when(sidebar_on_right, |el| {
                            el.left(-SIDEBAR_RESIZE_HANDLE_SIZE / 2.)
                        })
                        .top(px(0.))
                        .h_full()
                        .w(SIDEBAR_RESIZE_HANDLE_SIZE)
                        .cursor_col_resize()
                        .on_drag(DraggedSidebar, |dragged, _, _, cx| {
                            cx.stop_propagation();
                            cx.new(|_| dragged.clone())
                        })
                        .on_mouse_down(MouseButton::Left, |_, _, cx| {
                            cx.stop_propagation();
                        })
                        .on_mouse_up(MouseButton::Left, move |event, _, cx| {
                            if event.click_count == 2 {
                                weak.update(cx, |this, cx| {
                                    if let Some(sidebar) = this.sidebar.as_mut() {
                                        sidebar.set_width(None, cx);
                                    }
                                    this.serialize(cx);
                                })
                                .ok();
                                cx.stop_propagation();
                            } else {
                                weak.update(cx, |this, cx| {
                                    this.serialize(cx);
                                })
                                .ok();
                            }
                        })
                        .occlude(),
                );

                div()
                    .id("sidebar-container")
                    .relative()
                    .h_full()
                    .w(sidebar_width)
                    .flex_shrink_0()
                    .child(sidebar_handle.to_any())
                    .child(resize_handle)
                    .into_any_element()
            })
        } else {
            None
        };

        let (left_sidebar, right_sidebar) = if sidebar_on_right {
            (None, sidebar)
        } else {
            (sidebar, None)
        };

        let ui_font = theme_settings::setup_ui_font(window, cx);
        let text_color = cx.theme().colors().text;

        let workspace = self.workspace().clone();
        let workspace_key_context = workspace.update(cx, |workspace, cx| workspace.key_context(cx));
        let root = workspace.update(cx, |workspace, cx| workspace.actions(h_flex(), window, cx));

        client_side_decorations(
            root.key_context(workspace_key_context)
                .relative()
                .size_full()
                .font(ui_font)
                .text_color(text_color)
                .on_action(cx.listener(Self::close_window))
                .when(self.multi_workspace_enabled(cx), |this| {
                    this.on_action(cx.listener(
                        |this: &mut Self, _: &ToggleWorkspaceSidebar, window, cx| {
                            this.toggle_sidebar(window, cx);
                        },
                    ))
                    .on_action(cx.listener(
                        |this: &mut Self, _: &CloseWorkspaceSidebar, window, cx| {
                            this.close_sidebar_action(window, cx);
                        },
                    ))
                    .on_action(cx.listener(
                        |this: &mut Self, _: &FocusWorkspaceSidebar, window, cx| {
                            this.focus_sidebar(window, cx);
                        },
                    ))
                    .on_action(cx.listener(
                        |this: &mut Self, action: &ToggleThreadSwitcher, window, cx| {
                            if let Some(sidebar) = &this.sidebar {
                                sidebar.toggle_thread_switcher(action.select_last, window, cx);
                            }
                        },
                    ))
                    .on_action(cx.listener(|this: &mut Self, _: &NextProject, window, cx| {
                        if let Some(sidebar) = &this.sidebar {
                            sidebar.cycle_project(true, window, cx);
                        }
                    }))
                    .on_action(
                        cx.listener(|this: &mut Self, _: &PreviousProject, window, cx| {
                            if let Some(sidebar) = &this.sidebar {
                                sidebar.cycle_project(false, window, cx);
                            }
                        }),
                    )
                    .on_action(
                        cx.listener(|this: &mut Self, _: &MoveProjectUp, _window, cx| {
                            let key = this.project_group_key_for_workspace(this.workspace(), cx);
                            this.move_project_group_up(&key, cx);
                        }),
                    )
                    .on_action(
                        cx.listener(|this: &mut Self, _: &MoveProjectDown, _window, cx| {
                            let key = this.project_group_key_for_workspace(this.workspace(), cx);
                            this.move_project_group_down(&key, cx);
                        }),
                    )
                    .on_action(cx.listener(|this: &mut Self, _: &NextThread, window, cx| {
                        if let Some(sidebar) = &this.sidebar {
                            sidebar.cycle_thread(true, window, cx);
                        }
                    }))
                    .on_action(
                        cx.listener(|this: &mut Self, _: &PreviousThread, window, cx| {
                            if let Some(sidebar) = &this.sidebar {
                                sidebar.cycle_thread(false, window, cx);
                            }
                        }),
                    )
                    .when(self.project_group_keys().len() >= 2, |el| {
                        el.on_action(cx.listener(
                            |this: &mut Self, _: &MoveProjectToNewWindow, window, cx| {
                                let key =
                                    this.project_group_key_for_workspace(this.workspace(), cx);
                                this.open_project_group_in_new_window(&key, window, cx)
                                    .detach_and_log_err(cx);
                            },
                        ))
                    })
                })
                .when(
                    self.sidebar_open() && self.multi_workspace_enabled(cx),
                    |this| {
                        this.on_drag_move(cx.listener(
                            move |this: &mut Self,
                                  e: &DragMoveEvent<DraggedSidebar>,
                                  window,
                                  cx| {
                                if let Some(sidebar) = &this.sidebar {
                                    let new_width = if sidebar_on_right {
                                        window.bounds().size.width - e.event.position.x
                                    } else {
                                        e.event.position.x
                                    };
                                    sidebar.set_width(Some(new_width), cx);
                                }
                            },
                        ))
                    },
                )
                .children(left_sidebar)
                .child(
                    div()
                        .flex()
                        .flex_1()
                        .size_full()
                        .overflow_hidden()
                        .child(self.workspace().clone()),
                )
                .children(right_sidebar)
                .child(self.workspace().read(cx).modal_layer.clone())
                .children(self.sidebar_overlay.as_ref().map(|view| {
                    deferred(div().absolute().size_full().inset_0().occlude().child(
                        v_flex().h(px(0.0)).top_20().items_center().child(
                            h_flex().occlude().child(view.clone()).on_mouse_down(
                                MouseButton::Left,
                                |_, _, cx| {
                                    cx.stop_propagation();
                                },
                            ),
                        ),
                    ))
                    .with_priority(2)
                })),
            window,
            cx,
        )
    }
}
