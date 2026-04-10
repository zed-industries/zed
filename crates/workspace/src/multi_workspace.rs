use anyhow::Result;
use gpui::PathPromptOptions;
use gpui::{
    AnyView, App, Context, DragMoveEvent, Entity, EntityId, EventEmitter, FocusHandle, Focusable,
    ManagedView, MouseButton, Pixels, Render, Subscription, Task, Tiling, Window, WindowId,
    actions, deferred, px,
};
use project::{DirectoryLister, DisableAiSettings, Project, ProjectGroupKey};
use remote::RemoteConnectionOptions;
use settings::Settings;
pub use settings::SidebarSide;
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::path::Path;
use std::path::PathBuf;
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
    CloseIntent, CloseWindow, DockPosition, Event as WorkspaceEvent, Item, ModalView, OpenMode,
    Panel, Workspace, WorkspaceId, client_side_decorations,
    persistence::model::MultiWorkspaceState,
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
        /// Activates the next thread in sidebar order.
        NextThread,
        /// Activates the previous thread in sidebar order.
        PreviousThread,
        /// Expands the thread list for the current project to show more threads.
        ShowMoreThreads,
        /// Collapses the thread list for the current project to show fewer threads.
        ShowFewerThreads,
        /// Creates a new thread in the current workspace.
        NewThread,
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
    let current_position = AgentSettings::get_global(cx).sidebar_side;
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
                        settings::update_settings_file(fs.clone(), cx, move |settings, _cx| {
                            settings
                                .agent
                                .get_or_insert_default()
                                .set_sidebar_side(position);
                        });
                    },
                );
            }
            menu
        })
    })
}

pub enum MultiWorkspaceEvent {
    ActiveWorkspaceChanged,
    WorkspaceAdded(Entity<Workspace>),
    WorkspaceRemoved(EntityId),
    ProjectGroupKeyChanged {
        old_key: ProjectGroupKey,
        new_key: ProjectGroupKey,
    },
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

/// Tracks which workspace the user is currently looking at.
///
/// `Persistent` workspaces live in the `workspaces` vec and are shown in the
/// sidebar. `Transient` workspaces exist outside the vec and are discarded
/// when the user switches away.
enum ActiveWorkspace {
    /// A persistent workspace, identified by index into the `workspaces` vec.
    Persistent(usize),
    /// A workspace not in the `workspaces` vec that will be discarded on
    /// switch or promoted to persistent when the sidebar is opened.
    Transient(Entity<Workspace>),
}

impl ActiveWorkspace {
    fn transient_workspace(&self) -> Option<&Entity<Workspace>> {
        match self {
            Self::Transient(workspace) => Some(workspace),
            Self::Persistent(_) => None,
        }
    }

    /// Sets the active workspace to transient, returning the previous
    /// transient workspace (if any).
    fn set_transient(&mut self, workspace: Entity<Workspace>) -> Option<Entity<Workspace>> {
        match std::mem::replace(self, Self::Transient(workspace)) {
            Self::Transient(old) => Some(old),
            Self::Persistent(_) => None,
        }
    }

    /// Sets the active workspace to persistent at the given index,
    /// returning the previous transient workspace (if any).
    fn set_persistent(&mut self, index: usize) -> Option<Entity<Workspace>> {
        match std::mem::replace(self, Self::Persistent(index)) {
            Self::Transient(workspace) => Some(workspace),
            Self::Persistent(_) => None,
        }
    }
}

pub struct MultiWorkspace {
    window_id: WindowId,
    workspaces: Vec<Entity<Workspace>>,
    active_workspace: ActiveWorkspace,
    project_group_keys: Vec<ProjectGroupKey>,
    workspace_group_keys: HashMap<EntityId, ProjectGroupKey>,
    sidebar: Option<Box<dyn SidebarHandle>>,
    sidebar_open: bool,
    sidebar_overlay: Option<AnyView>,
    pending_removal_tasks: Vec<Task<()>>,
    _serialize_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
    previous_focus_handle: Option<FocusHandle>,
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
        let release_subscription = cx.on_release(|this: &mut MultiWorkspace, _cx| {
            if let Some(task) = this._serialize_task.take() {
                task.detach();
            }
            for task in std::mem::take(&mut this.pending_removal_tasks) {
                task.detach();
            }
        });
        let quit_subscription = cx.on_app_quit(Self::app_will_quit);
        let settings_subscription = cx.observe_global_in::<settings::SettingsStore>(window, {
            let mut previous_disable_ai = DisableAiSettings::get_global(cx).disable_ai;
            move |this, window, cx| {
                if DisableAiSettings::get_global(cx).disable_ai != previous_disable_ai {
                    this.collapse_to_single_workspace(window, cx);
                    previous_disable_ai = DisableAiSettings::get_global(cx).disable_ai;
                }
            }
        });
        Self::subscribe_to_workspace(&workspace, window, cx);
        let weak_self = cx.weak_entity();
        workspace.update(cx, |workspace, cx| {
            workspace.set_multi_workspace(weak_self, cx);
        });
        Self {
            window_id: window.window_handle().window_id(),
            project_group_keys: Vec::new(),
            workspace_group_keys: HashMap::default(),
            workspaces: Vec::new(),
            active_workspace: ActiveWorkspace::Transient(workspace),
            sidebar: None,
            sidebar_open: false,
            sidebar_overlay: None,
            pending_removal_tasks: Vec::new(),
            _serialize_task: None,
            _subscriptions: vec![
                release_subscription,
                quit_subscription,
                settings_subscription,
            ],
            previous_focus_handle: None,
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
        !DisableAiSettings::get_global(cx).disable_ai
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
        self.sidebar_open = true;
        if let ActiveWorkspace::Transient(workspace) = &self.active_workspace {
            let workspace = workspace.clone();
            let index = self.promote_transient(workspace, cx);
            self.active_workspace = ActiveWorkspace::Persistent(index);
        }
        let sidebar_focus_handle = self.sidebar.as_ref().map(|s| s.focus_handle(cx));
        for workspace in self.workspaces.iter() {
            workspace.update(cx, |workspace, _cx| {
                workspace.set_sidebar_focus_handle(sidebar_focus_handle.clone());
            });
        }
        self.serialize(cx);
        cx.notify();
    }

    pub fn close_sidebar(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.sidebar_open = false;
        for workspace in self.workspaces.iter() {
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
        cx.spawn_in(window, async move |this, cx| {
            let workspaces = this.update(cx, |multi_workspace, _cx| {
                multi_workspace.workspaces().cloned().collect::<Vec<_>>()
            })?;

            for workspace in workspaces {
                let should_continue = workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace.prepare_to_close(CloseIntent::CloseWindow, window, cx)
                    })?
                    .await?;
                if !should_continue {
                    return anyhow::Ok(());
                }
            }

            cx.update(|window, _cx| {
                window.remove_window();
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
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
                project::Event::WorktreeAdded(_)
                | project::Event::WorktreeRemoved(_)
                | project::Event::WorktreeUpdatedRootRepoCommonDir(_) => {
                    if let Some(workspace) = workspace.upgrade() {
                        this.handle_workspace_key_change(&workspace, cx);
                    }
                }
                _ => {}
            }
        })
        .detach();

        cx.subscribe_in(workspace, window, |this, workspace, event, window, cx| {
            if let WorkspaceEvent::Activate = event {
                this.activate(workspace.clone(), window, cx);
            }
        })
        .detach();
    }

    fn handle_workspace_key_change(
        &mut self,
        workspace: &Entity<Workspace>,
        cx: &mut Context<Self>,
    ) {
        let workspace_id = workspace.entity_id();
        let old_key = self.project_group_key_for_workspace(workspace, cx);
        let new_key = workspace.read(cx).project_group_key(cx);

        if new_key.path_list().paths().is_empty() || old_key == new_key {
            return;
        }

        let active_workspace = self.workspace().clone();

        self.set_workspace_group_key(workspace, new_key.clone());

        let changed_root_paths = workspace.read(cx).root_paths(cx);
        let old_paths = old_key.path_list().paths();
        let new_paths = new_key.path_list().paths();

        // Remove workspaces that already had the new key and have the same
        // root paths (true duplicates that this workspace is replacing).
        //
        // NOTE: These are dropped without prompting for unsaved changes because
        // the user explicitly added a folder that makes this workspace
        // identical to the duplicate — they are intentionally overwriting it.
        let duplicate_workspaces: Vec<Entity<Workspace>> = self
            .workspaces
            .iter()
            .filter(|ws| {
                ws.entity_id() != workspace_id
                    && self.project_group_key_for_workspace(ws, cx) == new_key
                    && ws.read(cx).root_paths(cx) == changed_root_paths
            })
            .cloned()
            .collect();

        if duplicate_workspaces.contains(&active_workspace) {
            // The active workspace is among the duplicates — drop the
            // incoming workspace instead so the user stays where they are.
            self.detach_workspace(workspace, cx);
            self.workspaces.retain(|w| w != workspace);
        } else {
            for ws in &duplicate_workspaces {
                self.detach_workspace(ws, cx);
                self.workspaces.retain(|w| w != ws);
            }
        }

        // Propagate folder adds/removes to linked worktree siblings
        // (different root paths, same old key) so they stay in the group.
        let group_workspaces: Vec<Entity<Workspace>> = self
            .workspaces
            .iter()
            .filter(|ws| {
                ws.entity_id() != workspace_id
                    && self.project_group_key_for_workspace(ws, cx) == old_key
            })
            .cloned()
            .collect();

        for workspace in &group_workspaces {
            // Pre-set this to stop later WorktreeAdded events from triggering
            self.set_workspace_group_key(&workspace, new_key.clone());

            let project = workspace.read(cx).project().clone();

            for added_path in new_paths.iter().filter(|p| !old_paths.contains(p)) {
                project
                    .update(cx, |project, cx| {
                        project.find_or_create_worktree(added_path, true, cx)
                    })
                    .detach_and_log_err(cx);
            }

            for removed_path in old_paths.iter().filter(|p| !new_paths.contains(p)) {
                project.update(cx, |project, cx| {
                    project.remove_worktree_for_main_worktree_path(removed_path, cx);
                });
            }
        }

        // Restore the active workspace after removals may have shifted
        // the index. If the previously active workspace was removed,
        // fall back to the workspace whose key just changed.
        if let ActiveWorkspace::Persistent(_) = &self.active_workspace {
            let target = if self.workspaces.contains(&active_workspace) {
                &active_workspace
            } else {
                workspace
            };
            if let Some(new_index) = self.workspaces.iter().position(|ws| ws == target) {
                self.active_workspace = ActiveWorkspace::Persistent(new_index);
            }
        }

        self.remove_stale_project_group_keys(cx);

        cx.emit(MultiWorkspaceEvent::ProjectGroupKeyChanged { old_key, new_key });
        self.serialize(cx);
        cx.notify();
    }

    fn add_project_group_key(&mut self, project_group_key: ProjectGroupKey) {
        if project_group_key.path_list().paths().is_empty() {
            return;
        }
        if self.project_group_keys.contains(&project_group_key) {
            return;
        }
        // Store newest first so the vec is in "most recently added"
        self.project_group_keys.insert(0, project_group_key);
    }

    pub(crate) fn set_workspace_group_key(
        &mut self,
        workspace: &Entity<Workspace>,
        project_group_key: ProjectGroupKey,
    ) {
        self.workspace_group_keys
            .insert(workspace.entity_id(), project_group_key.clone());
        self.add_project_group_key(project_group_key);
    }

    pub fn project_group_key_for_workspace(
        &self,
        workspace: &Entity<Workspace>,
        cx: &App,
    ) -> ProjectGroupKey {
        self.workspace_group_keys
            .get(&workspace.entity_id())
            .cloned()
            .unwrap_or_else(|| workspace.read(cx).project_group_key(cx))
    }

    fn remove_stale_project_group_keys(&mut self, cx: &App) {
        let workspace_keys: HashSet<ProjectGroupKey> = self
            .workspaces
            .iter()
            .map(|workspace| self.project_group_key_for_workspace(workspace, cx))
            .collect();
        self.project_group_keys
            .retain(|key| workspace_keys.contains(key));
    }

    pub fn restore_project_group_keys(&mut self, keys: Vec<ProjectGroupKey>) {
        let mut restored: Vec<ProjectGroupKey> = Vec::with_capacity(keys.len());
        for key in keys {
            if key.path_list().paths().is_empty() {
                continue;
            }
            if !restored.contains(&key) {
                restored.push(key);
            }
        }
        for existing_key in &self.project_group_keys {
            if !restored.contains(existing_key) {
                restored.push(existing_key.clone());
            }
        }
        self.project_group_keys = restored;
    }

    pub fn project_group_keys(&self) -> impl Iterator<Item = &ProjectGroupKey> {
        self.project_group_keys.iter()
    }

    /// Returns the project groups, ordered by most recently added.
    pub fn project_groups(
        &self,
        cx: &App,
    ) -> impl Iterator<Item = (ProjectGroupKey, Vec<Entity<Workspace>>)> {
        let mut groups = self
            .project_group_keys
            .iter()
            .map(|key| (key.clone(), Vec::new()))
            .collect::<Vec<_>>();
        for workspace in &self.workspaces {
            let key = self.project_group_key_for_workspace(workspace, cx);
            if let Some((_, workspaces)) = groups.iter_mut().find(|(k, _)| k == &key) {
                workspaces.push(workspace.clone());
            }
        }
        groups.into_iter()
    }

    pub fn workspaces_for_project_group(
        &self,
        project_group_key: &ProjectGroupKey,
        cx: &App,
    ) -> impl Iterator<Item = &Entity<Workspace>> {
        self.workspaces.iter().filter(move |workspace| {
            self.project_group_key_for_workspace(workspace, cx) == *project_group_key
        })
    }

    pub fn remove_folder_from_project_group(
        &mut self,
        project_group_key: &ProjectGroupKey,
        path: &Path,
        cx: &mut Context<Self>,
    ) {
        let new_path_list = project_group_key.path_list().without_path(path);
        if new_path_list.is_empty() {
            return;
        }

        let new_key = ProjectGroupKey::new(project_group_key.host(), new_path_list);

        let workspaces: Vec<_> = self
            .workspaces_for_project_group(project_group_key, cx)
            .cloned()
            .collect();

        self.add_project_group_key(new_key);

        for workspace in workspaces {
            let project = workspace.read(cx).project().clone();
            project.update(cx, |project, cx| {
                project.remove_worktree_for_main_worktree_path(path, cx);
            });
        }

        self.serialize(cx);
        cx.notify();
    }

    pub fn prompt_to_add_folders_to_project_group(
        &mut self,
        key: &ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let paths = self.workspace().update(cx, |workspace, cx| {
            workspace.prompt_for_open_path(
                PathPromptOptions {
                    files: false,
                    directories: true,
                    multiple: true,
                    prompt: None,
                },
                DirectoryLister::Project(workspace.project().clone()),
                window,
                cx,
            )
        });

        let key = key.clone();
        cx.spawn_in(window, async move |this, cx| {
            if let Some(new_paths) = paths.await.ok().flatten() {
                if !new_paths.is_empty() {
                    this.update(cx, |multi_workspace, cx| {
                        multi_workspace.add_folders_to_project_group(&key, new_paths, cx);
                    })?;
                }
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub fn add_folders_to_project_group(
        &mut self,
        project_group_key: &ProjectGroupKey,
        new_paths: Vec<PathBuf>,
        cx: &mut Context<Self>,
    ) {
        let mut all_paths: Vec<PathBuf> = project_group_key.path_list().paths().to_vec();
        all_paths.extend(new_paths.iter().cloned());
        let new_path_list = PathList::new(&all_paths);
        let new_key = ProjectGroupKey::new(project_group_key.host(), new_path_list);

        let workspaces: Vec<_> = self
            .workspaces_for_project_group(project_group_key, cx)
            .cloned()
            .collect();

        self.add_project_group_key(new_key);

        for workspace in workspaces {
            let project = workspace.read(cx).project().clone();
            for path in &new_paths {
                project
                    .update(cx, |project, cx| {
                        project.find_or_create_worktree(path, true, cx)
                    })
                    .detach_and_log_err(cx);
            }
        }

        self.serialize(cx);
        cx.notify();
    }

    pub fn remove_project_group(
        &mut self,
        key: &ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<bool>> {
        let workspaces: Vec<_> = self
            .workspaces_for_project_group(key, cx)
            .cloned()
            .collect();

        // Compute the neighbor while the key is still in the list.
        let neighbor_key = {
            let pos = self.project_group_keys.iter().position(|k| k == key);
            pos.and_then(|pos| {
                // Keys are in display order, so pos+1 is below
                // and pos-1 is above. Try below first.
                self.project_group_keys.get(pos + 1).or_else(|| {
                    pos.checked_sub(1)
                        .and_then(|i| self.project_group_keys.get(i))
                })
            })
            .cloned()
        };

        // Now remove the key.
        self.project_group_keys.retain(|k| k != key);

        self.remove(
            workspaces,
            move |this, window, cx| {
                if let Some(neighbor_key) = neighbor_key {
                    return this.find_or_create_local_workspace(
                        neighbor_key.path_list().clone(),
                        window,
                        cx,
                    );
                }

                // No other project groups remain — create an empty workspace.
                let app_state = this.workspace().read(cx).app_state().clone();
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
                let new_workspace =
                    cx.new(|cx| Workspace::new(None, project, app_state, window, cx));
                Task::ready(Ok(new_workspace))
            },
            window,
            cx,
        )
    }

    /// Finds an existing workspace whose root paths and host exactly match.
    pub fn workspace_for_paths(
        &self,
        path_list: &PathList,
        host: Option<&RemoteConnectionOptions>,
        cx: &App,
    ) -> Option<Entity<Workspace>> {
        self.workspaces
            .iter()
            .find(|ws| {
                let key = ws.read(cx).project_group_key(cx);
                key.host().as_ref() == host
                    && PathList::new(&ws.read(cx).root_paths(cx)) == *path_list
            })
            .cloned()
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Workspace>>> {
        if let Some(workspace) = self.workspace_for_paths(&paths, host.as_ref(), cx) {
            self.activate(workspace.clone(), window, cx);
            return Task::ready(Ok(workspace));
        }

        let Some(connection_options) = host else {
            return self.find_or_create_local_workspace(paths, window, cx);
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

            let window_handle =
                window_handle.ok_or_else(|| anyhow::anyhow!("Window is not a MultiWorkspace"))?;

            open_remote_project_with_existing_connection(
                connection_options,
                new_project,
                paths_vec,
                app_state,
                window_handle,
                provisional_project_group_key,
                cx,
            )
            .await?;

            window_handle.update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Workspace>>> {
        if let Some(workspace) = self.workspace_for_paths(&path_list, None, cx) {
            self.activate(workspace.clone(), window, cx);
            return Task::ready(Ok(workspace));
        }

        if let Some(transient) = self.active_workspace.transient_workspace() {
            if transient.read(cx).project_group_key(cx).path_list() == &path_list {
                return Task::ready(Ok(transient.clone()));
            }
        }

        let paths = path_list.paths().to_vec();
        let app_state = self.workspace().read(cx).app_state().clone();
        let requesting_window = window.window_handle().downcast::<MultiWorkspace>();

        cx.spawn(async move |_this, cx| {
            let result = cx
                .update(|cx| {
                    Workspace::new_local(
                        paths,
                        app_state,
                        requesting_window,
                        None,
                        None,
                        OpenMode::Activate,
                        cx,
                    )
                })
                .await?;
            Ok(result.workspace)
        })
    }

    pub fn workspace(&self) -> &Entity<Workspace> {
        match &self.active_workspace {
            ActiveWorkspace::Persistent(index) => &self.workspaces[*index],
            ActiveWorkspace::Transient(workspace) => workspace,
        }
    }

    pub fn workspaces(&self) -> impl Iterator<Item = &Entity<Workspace>> {
        self.workspaces
            .iter()
            .chain(self.active_workspace.transient_workspace())
    }

    /// Adds a workspace to this window as persistent without changing which
    /// workspace is active. Unlike `activate()`, this always inserts into the
    /// persistent list regardless of sidebar state — it's used for system-
    /// initiated additions like deserialization and worktree discovery.
    pub fn add(&mut self, workspace: Entity<Workspace>, window: &Window, cx: &mut Context<Self>) {
        self.insert_workspace(workspace, window, cx);
    }

    /// Ensures the workspace is in the multiworkspace and makes it the active one.
    pub fn activate(
        &mut self,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Re-activating the current workspace is a no-op.
        if self.workspace() == &workspace {
            self.focus_active_workspace(window, cx);
            return;
        }

        // Resolve where we're going.
        let new_index = if let Some(index) = self.workspaces.iter().position(|w| *w == workspace) {
            Some(index)
        } else if self.sidebar_open {
            Some(self.insert_workspace(workspace.clone(), &*window, cx))
        } else {
            None
        };

        // Transition the active workspace.
        if let Some(index) = new_index {
            if let Some(old) = self.active_workspace.set_persistent(index) {
                if self.sidebar_open {
                    self.promote_transient(old, cx);
                } else {
                    self.detach_workspace(&old, cx);
                }
            }
        } else {
            Self::subscribe_to_workspace(&workspace, window, cx);
            let weak_self = cx.weak_entity();
            workspace.update(cx, |workspace, cx| {
                workspace.set_multi_workspace(weak_self, cx);
            });
            if let Some(old) = self.active_workspace.set_transient(workspace) {
                self.detach_workspace(&old, cx);
            }
        }

        cx.emit(MultiWorkspaceEvent::ActiveWorkspaceChanged);
        self.serialize(cx);
        self.focus_active_workspace(window, cx);
        cx.notify();
    }

    /// Promotes the currently active workspace to persistent if it is
    /// transient, so it is retained across workspace switches even when
    /// the sidebar is closed. No-op if the workspace is already persistent.
    pub fn retain_active_workspace(&mut self, cx: &mut Context<Self>) {
        if let ActiveWorkspace::Transient(workspace) = &self.active_workspace {
            let workspace = workspace.clone();
            let index = self.promote_transient(workspace, cx);
            self.active_workspace = ActiveWorkspace::Persistent(index);
            self.serialize(cx);
            cx.notify();
        }
    }

    /// Promotes a former transient workspace into the persistent list.
    /// Returns the index of the newly inserted workspace.
    fn promote_transient(&mut self, workspace: Entity<Workspace>, cx: &mut Context<Self>) -> usize {
        let project_group_key = self.project_group_key_for_workspace(&workspace, cx);
        self.set_workspace_group_key(&workspace, project_group_key);
        self.workspaces.push(workspace.clone());
        cx.emit(MultiWorkspaceEvent::WorkspaceAdded(workspace));
        self.workspaces.len() - 1
    }

    /// Collapses to a single transient workspace, discarding all persistent
    /// workspaces. Used when multi-workspace is disabled (e.g. disable_ai).
    fn collapse_to_single_workspace(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.sidebar_open {
            self.close_sidebar(window, cx);
        }
        let active = self.workspace().clone();
        for workspace in std::mem::take(&mut self.workspaces) {
            if workspace != active {
                self.detach_workspace(&workspace, cx);
            }
        }
        self.project_group_keys.clear();
        self.workspace_group_keys.clear();
        self.active_workspace = ActiveWorkspace::Transient(active);
        cx.notify();
    }

    /// Inserts a workspace into the list if not already present. Returns the
    /// index of the workspace (existing or newly inserted). Does not change
    /// the active workspace index.
    fn insert_workspace(
        &mut self,
        workspace: Entity<Workspace>,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> usize {
        if let Some(index) = self.workspaces.iter().position(|w| *w == workspace) {
            index
        } else {
            let project_group_key = self.project_group_key_for_workspace(&workspace, cx);

            Self::subscribe_to_workspace(&workspace, window, cx);
            self.sync_sidebar_to_workspace(&workspace, cx);
            let weak_self = cx.weak_entity();
            workspace.update(cx, |workspace, cx| {
                workspace.set_multi_workspace(weak_self, cx);
            });

            self.set_workspace_group_key(&workspace, project_group_key);
            self.workspaces.push(workspace.clone());
            cx.emit(MultiWorkspaceEvent::WorkspaceAdded(workspace));
            cx.notify();
            self.workspaces.len() - 1
        }
    }

    /// Detaches a workspace: clears session state, DB binding, cached
    /// group key, and emits `WorkspaceRemoved`. The DB row is preserved
    /// so the workspace still appears in the recent-projects list.
    fn detach_workspace(&mut self, workspace: &Entity<Workspace>, cx: &mut Context<Self>) {
        self.workspace_group_keys.remove(&workspace.entity_id());
        cx.emit(MultiWorkspaceEvent::WorkspaceRemoved(workspace.entity_id()));
        workspace.update(cx, |workspace, _cx| {
            workspace.session_id.take();
            workspace._schedule_serialize_workspace.take();
            workspace._serialize_workspace_task.take();
        });

        if let Some(workspace_id) = workspace.read(cx).database_id() {
            let db = crate::persistence::WorkspaceDb::global(cx);
            self.pending_removal_tasks.retain(|task| !task.is_ready());
            self.pending_removal_tasks
                .push(cx.background_spawn(async move {
                    db.set_session_binding(workspace_id, None, None)
                        .await
                        .log_err();
                }));
        }
    }

    fn sync_sidebar_to_workspace(&self, workspace: &Entity<Workspace>, cx: &mut Context<Self>) {
        if self.sidebar_open() {
            let sidebar_focus_handle = self.sidebar.as_ref().map(|s| s.focus_handle(cx));
            workspace.update(cx, |workspace, _| {
                workspace.set_sidebar_focus_handle(sidebar_focus_handle);
            });
        }
    }

    pub(crate) fn serialize(&mut self, cx: &mut Context<Self>) {
        self._serialize_task = Some(cx.spawn(async move |this, cx| {
            let Some((window_id, state)) = this
                .read_with(cx, |this, cx| {
                    let state = MultiWorkspaceState {
                        active_workspace_id: this.workspace().read(cx).database_id(),
                        project_group_keys: this
                            .project_group_keys()
                            .cloned()
                            .map(Into::into)
                            .collect::<Vec<_>>(),
                        sidebar_open: this.sidebar_open,
                        sidebar_state: this.sidebar.as_ref().and_then(|s| s.serialized_state(cx)),
                    };
                    (this.window_id, state)
                })
                .ok()
            else {
                return;
            };
            let kvp = cx.update(|cx| db::kvp::KeyValueStore::global(cx));
            crate::persistence::write_multi_workspace_state(&kvp, window_id, state).await;
        }));
    }

    /// Returns the in-flight serialization task (if any) so the caller can
    /// await it. Used by the quit handler to ensure pending DB writes
    /// complete before the process exits.
    pub fn flush_serialization(&mut self) -> Task<()> {
        self._serialize_task.take().unwrap_or(Task::ready(()))
    }

    fn app_will_quit(&mut self, _cx: &mut Context<Self>) -> impl Future<Output = ()> + use<> {
        let mut tasks: Vec<Task<()>> = Vec::new();
        if let Some(task) = self._serialize_task.take() {
            tasks.push(task);
        }
        tasks.extend(std::mem::take(&mut self.pending_removal_tasks));

        async move {
            futures::future::join_all(tasks).await;
        }
    }

    pub fn focus_active_workspace(&self, window: &mut Window, cx: &mut App) {
        // If a dock panel is zoomed, focus it instead of the center pane.
        // Otherwise, focusing the center pane triggers dismiss_zoomed_items_to_reveal
        // which closes the zoomed dock.
        let focus_handle = {
            let workspace = self.workspace().read(cx);
            let mut target = None;
            for dock in workspace.all_docks() {
                let dock = dock.read(cx);
                if dock.is_open() {
                    if let Some(panel) = dock.active_panel() {
                        if panel.is_zoomed(window, cx) {
                            target = Some(panel.panel_focus_handle(cx));
                            break;
                        }
                    }
                }
            }
            target.unwrap_or_else(|| {
                let pane = workspace.active_pane().clone();
                pane.read(cx).focus_handle(cx)
            })
        };
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

    pub fn take_pending_removal_tasks(&mut self) -> Vec<Task<()>> {
        let tasks: Vec<Task<()>> = std::mem::take(&mut self.pending_removal_tasks)
            .into_iter()
            .filter(|task| !task.is_ready())
            .collect();
        tasks
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn assert_project_group_key_integrity(&self, cx: &App) -> anyhow::Result<()> {
        let stored_keys: HashSet<&ProjectGroupKey> = self.project_group_keys().collect();

        let workspace_group_keys: HashSet<&ProjectGroupKey> =
            self.workspace_group_keys.values().collect();
        let extra_keys = &workspace_group_keys - &stored_keys;
        anyhow::ensure!(
            extra_keys.is_empty(),
            "workspace_group_keys values not in project_group_keys: {:?}",
            extra_keys,
        );

        let cached_ids: HashSet<EntityId> = self.workspace_group_keys.keys().copied().collect();
        let workspace_ids: HashSet<EntityId> =
            self.workspaces.iter().map(|ws| ws.entity_id()).collect();
        anyhow::ensure!(
            cached_ids == workspace_ids,
            "workspace_group_keys entity IDs don't match workspaces.\n\
             only in cache: {:?}\n\
             only in workspaces: {:?}",
            &cached_ids - &workspace_ids,
            &workspace_ids - &cached_ids,
        );

        for workspace in self.workspaces() {
            let live_key = workspace.read(cx).project_group_key(cx);
            let cached_key = &self.workspace_group_keys[&workspace.entity_id()];
            anyhow::ensure!(
                *cached_key == live_key,
                "workspace {:?} has live key {:?} but cached key {:?}",
                workspace.entity_id(),
                live_key,
                cached_key,
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
        self.activate(workspace.clone(), window, cx);
        workspace
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
        self.activate(new_workspace.clone(), window, cx);

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
                        db.set_session_binding(workspace_id, session_id, Some(window_id))
                            .await
                            .log_err();
                    })
                })
                .unwrap();
            task.await
        })
    }

    /// Removes one or more workspaces from this multi-workspace.
    ///
    /// If the active workspace is among those being removed,
    /// `fallback_workspace` is called **synchronously before the removal
    /// begins** to produce a `Task` that resolves to the workspace that
    /// should become active. The fallback must not be one of the
    /// workspaces being removed.
    ///
    /// Returns `true` if any workspaces were actually removed.
    pub fn remove(
        &mut self,
        workspaces: impl IntoIterator<Item = Entity<Workspace>>,
        fallback_workspace: impl FnOnce(
            &mut Self,
            &mut Window,
            &mut Context<Self>,
        ) -> Task<Result<Entity<Workspace>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<bool>> {
        let workspaces: Vec<_> = workspaces.into_iter().collect();

        if workspaces.is_empty() {
            return Task::ready(Ok(false));
        }

        let removing_active = workspaces.iter().any(|ws| ws == self.workspace());
        let original_active = self.workspace().clone();

        let fallback_task = removing_active.then(|| fallback_workspace(self, window, cx));

        cx.spawn_in(window, async move |this, cx| {
            // Prompt each workspace for unsaved changes. If any workspace
            // has dirty buffers, save_all_internal will emit Activate to
            // bring it into view before showing the save dialog.
            for workspace in &workspaces {
                let should_continue = workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace.save_all_internal(crate::SaveIntent::Close, window, cx)
                    })?
                    .await?;

                if !should_continue {
                    return Ok(false);
                }
            }

            // If we're removing the active workspace, await the
            // fallback and switch to it before tearing anything down.
            // Otherwise restore the original active workspace in case
            // prompting switched away from it.
            if let Some(fallback_task) = fallback_task {
                let new_active = fallback_task.await?;

                this.update_in(cx, |this, window, cx| {
                    assert!(
                        !workspaces.contains(&new_active),
                        "fallback workspace must not be one of the workspaces being removed"
                    );
                    this.activate(new_active, window, cx);
                })?;
            } else {
                this.update_in(cx, |this, window, cx| {
                    if *this.workspace() != original_active {
                        this.activate(original_active, window, cx);
                    }
                })?;
            }

            // Actually remove the workspaces.
            this.update_in(cx, |this, _, cx| {
                // Save a handle to the active workspace so we can restore
                // its index after the removals shift the vec around.
                let active_workspace = this.workspace().clone();

                let mut removed_workspaces: Vec<Entity<Workspace>> = Vec::new();

                this.workspaces.retain(|ws| {
                    if workspaces.contains(ws) {
                        removed_workspaces.push(ws.clone());
                        false
                    } else {
                        true
                    }
                });

                for workspace in &removed_workspaces {
                    this.detach_workspace(workspace, cx);
                }

                let removed_any = !removed_workspaces.is_empty();

                if removed_any {
                    // Restore the active workspace index after removals.
                    if let Some(new_index) = this
                        .workspaces
                        .iter()
                        .position(|ws| ws == &active_workspace)
                    {
                        this.active_workspace = ActiveWorkspace::Persistent(new_index);
                    }

                    this.serialize(cx);
                    cx.notify();
                }

                Ok(removed_any)
            })?
        })
    }

    pub fn open_project(
        &mut self,
        paths: Vec<PathBuf>,
        open_mode: OpenMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Workspace>>> {
        if self.multi_workspace_enabled(cx) {
            self.find_or_create_local_workspace(PathList::new(&paths), window, cx)
        } else {
            let workspace = self.workspace().clone();
            cx.spawn_in(window, async move |_this, cx| {
                let should_continue = workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace.prepare_to_close(crate::CloseIntent::ReplaceWindow, window, cx)
                    })?
                    .await?;
                if should_continue {
                    workspace
                        .update_in(cx, |workspace, window, cx| {
                            workspace.open_workspace_for_paths(open_mode, paths, window, cx)
                        })?
                        .await
                } else {
                    Ok(workspace)
                }
            })
        }
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
                    .on_action(cx.listener(|this: &mut Self, _: &NextThread, window, cx| {
                        if let Some(sidebar) = &this.sidebar {
                            sidebar.cycle_thread(true, window, cx);
                        }
                    }))
                    .on_action(cx.listener(
                        |this: &mut Self, _: &PreviousThread, window, cx| {
                            if let Some(sidebar) = &this.sidebar {
                                sidebar.cycle_thread(false, window, cx);
                            }
                        },
                    ))
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
            Tiling {
                left: !sidebar_on_right && multi_workspace_enabled && self.sidebar_open(),
                right: sidebar_on_right && multi_workspace_enabled && self.sidebar_open(),
                ..Tiling::default()
            },
        )
    }
}
