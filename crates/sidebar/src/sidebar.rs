mod thread_switcher;

use acp_thread::ThreadStatus;
use action_log::DiffStats;
use agent_client_protocol::{self as acp};
use agent_settings::AgentSettings;
use agent_ui::thread_metadata_store::{ThreadMetadata, ThreadMetadataStore, WorktreePaths};
use agent_ui::thread_worktree_archive;
use agent_ui::threads_archive_view::{
    ThreadsArchiveView, ThreadsArchiveViewEvent, format_history_entry_timestamp,
};
use agent_ui::{
    AcpThreadImportOnboarding, Agent, AgentPanel, AgentPanelEvent, DEFAULT_THREAD_TITLE, NewThread,
    RemoveSelectedThread, ThreadId, ThreadImportModal,
};
use chrono::{DateTime, Utc};
use editor::Editor;
use gpui::{
    Action as _, AnyElement, App, Context, DismissEvent, Entity, EntityId, FocusHandle, Focusable,
    KeyContext, ListState, Pixels, Render, SharedString, Task, WeakEntity, Window, WindowHandle,
    linear_color_stop, linear_gradient, list, prelude::*, px,
};
use menu::{
    Cancel, Confirm, SelectChild, SelectFirst, SelectLast, SelectNext, SelectParent, SelectPrevious,
};
use project::{
    AgentId, AgentRegistryStore, Event as ProjectEvent, WorktreeId, linked_worktree_short_name,
};
use recent_projects::sidebar_recent_projects::SidebarRecentProjects;
use remote::RemoteConnectionOptions;
use ui::utils::platform_title_bar_height;

use serde::{Deserialize, Serialize};
use settings::Settings as _;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use theme::ActiveTheme;
use ui::{
    AgentThreadStatus, CommonAnimationExt, ContextMenu, Divider, GradientFade, HighlightedLabel,
    KeyBinding, PopoverMenu, PopoverMenuHandle, Tab, ThreadItem, ThreadItemWorktreeInfo, TintColor,
    Tooltip, WithScrollbar, prelude::*,
};
use util::ResultExt as _;
use util::path_list::PathList;
use workspace::{
    AddFolderToProject, CloseWindow, FocusWorkspaceSidebar, MultiWorkspace, MultiWorkspaceEvent,
    NextProject, NextThread, Open, PreviousProject, PreviousThread, ProjectGroupKey, SaveIntent,
    ShowFewerThreads, ShowMoreThreads, Sidebar as WorkspaceSidebar, SidebarSide, Toast,
    ToggleWorkspaceSidebar, Workspace, notifications::NotificationId, sidebar_side_context_menu,
};

use zed_actions::OpenRecent;
use zed_actions::editor::{MoveDown, MoveUp};

use zed_actions::agents_sidebar::{FocusSidebarFilter, ToggleThreadSwitcher};

use crate::thread_switcher::{ThreadSwitcher, ThreadSwitcherEntry, ThreadSwitcherEvent};

#[cfg(test)]
mod sidebar_tests;

gpui::actions!(
    agents_sidebar,
    [
        /// Creates a new thread in the currently selected or active project group.
        NewThreadInGroup,
        /// Toggles between the thread list and the archive view.
        ToggleArchive,
    ]
);

gpui::actions!(
    dev,
    [
        /// Dumps multi-workspace state (projects, worktrees, active threads) into a new buffer.
        DumpWorkspaceInfo,
    ]
);

const DEFAULT_WIDTH: Pixels = px(300.0);
const MIN_WIDTH: Pixels = px(200.0);
const MAX_WIDTH: Pixels = px(800.0);
const DEFAULT_THREADS_SHOWN: usize = 5;

#[derive(Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum SerializedSidebarView {
    #[default]
    ThreadList,
    Archive,
}

#[derive(Default, Serialize, Deserialize)]
struct SerializedSidebar {
    #[serde(default)]
    width: Option<f32>,
    #[serde(default)]
    active_view: SerializedSidebarView,
}

#[derive(Debug, Default)]
enum SidebarView {
    #[default]
    ThreadList,
    Archive(Entity<ThreadsArchiveView>),
}

enum ArchiveWorktreeOutcome {
    Success,
    Cancelled,
}

#[derive(Clone, Debug)]
struct ActiveEntry {
    thread_id: agent_ui::ThreadId,
    /// Stable remote identifier, used for matching when thread_id
    /// differs (e.g. after cross-window activation creates a new
    /// local ThreadId).
    session_id: Option<acp::SessionId>,
    workspace: Entity<Workspace>,
}

impl ActiveEntry {
    fn workspace(&self) -> &Entity<Workspace> {
        &self.workspace
    }

    fn is_active_thread(&self, thread_id: &agent_ui::ThreadId) -> bool {
        self.thread_id == *thread_id
    }

    fn matches_entry(&self, entry: &ListEntry) -> bool {
        match entry {
            ListEntry::Thread(thread) => {
                self.thread_id == thread.metadata.thread_id
                    || self
                        .session_id
                        .as_ref()
                        .zip(thread.metadata.session_id.as_ref())
                        .is_some_and(|(a, b)| a == b)
            }
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
struct ActiveThreadInfo {
    session_id: acp::SessionId,
    title: SharedString,
    status: AgentThreadStatus,
    icon: IconName,
    icon_from_external_svg: Option<SharedString>,
    is_background: bool,
    is_title_generating: bool,
    diff_stats: DiffStats,
}

#[derive(Clone)]
enum ThreadEntryWorkspace {
    Open(Entity<Workspace>),
    Closed {
        /// The paths this thread uses (may point to linked worktrees).
        folder_paths: PathList,
        /// The project group this thread belongs to.
        project_group_key: ProjectGroupKey,
    },
}

impl ThreadEntryWorkspace {
    fn is_remote(&self, cx: &App) -> bool {
        match self {
            ThreadEntryWorkspace::Open(workspace) => {
                !workspace.read(cx).project().read(cx).is_local()
            }
            ThreadEntryWorkspace::Closed {
                project_group_key, ..
            } => project_group_key.host().is_some(),
        }
    }
}

#[derive(Clone)]
struct WorktreeInfo {
    name: SharedString,
    full_path: SharedString,
    highlight_positions: Vec<usize>,
    kind: ui::WorktreeKind,
}

#[derive(Clone)]
struct ThreadEntry {
    metadata: ThreadMetadata,
    icon: IconName,
    icon_from_external_svg: Option<SharedString>,
    status: AgentThreadStatus,
    workspace: ThreadEntryWorkspace,
    is_live: bool,
    is_background: bool,
    is_title_generating: bool,
    is_draft: bool,
    highlight_positions: Vec<usize>,
    worktrees: Vec<WorktreeInfo>,
    diff_stats: DiffStats,
}

impl ThreadEntry {
    /// Updates this thread entry with active thread information.
    ///
    /// The existing [`ThreadEntry`] was likely deserialized from the database
    /// but if we have a correspond thread already loaded we want to apply the
    /// live information.
    fn apply_active_info(&mut self, info: &ActiveThreadInfo) {
        self.metadata.title = Some(info.title.clone());
        self.status = info.status;
        self.icon = info.icon;
        self.icon_from_external_svg = info.icon_from_external_svg.clone();
        self.is_live = true;
        self.is_background = info.is_background;
        self.is_title_generating = info.is_title_generating;
        self.diff_stats = info.diff_stats;
    }
}

#[derive(Clone)]
enum ListEntry {
    ProjectHeader {
        key: ProjectGroupKey,
        label: SharedString,
        highlight_positions: Vec<usize>,
        has_running_threads: bool,
        waiting_thread_count: usize,
        is_active: bool,
        has_threads: bool,
    },
    Thread(ThreadEntry),
    ViewMore {
        key: ProjectGroupKey,
        is_fully_expanded: bool,
    },
}

#[cfg(test)]
impl ListEntry {
    fn session_id(&self) -> Option<&acp::SessionId> {
        match self {
            ListEntry::Thread(thread_entry) => thread_entry.metadata.session_id.as_ref(),
            _ => None,
        }
    }

    fn reachable_workspaces<'a>(
        &'a self,
        multi_workspace: &'a workspace::MultiWorkspace,
        cx: &'a App,
    ) -> Vec<Entity<Workspace>> {
        match self {
            ListEntry::Thread(thread) => match &thread.workspace {
                ThreadEntryWorkspace::Open(ws) => vec![ws.clone()],
                ThreadEntryWorkspace::Closed { .. } => Vec::new(),
            },
            ListEntry::ProjectHeader { key, .. } => multi_workspace
                .workspaces_for_project_group(key, cx)
                .unwrap_or_default(),
            ListEntry::ViewMore { .. } => Vec::new(),
        }
    }
}

impl From<ThreadEntry> for ListEntry {
    fn from(thread: ThreadEntry) -> Self {
        ListEntry::Thread(thread)
    }
}

#[derive(Default)]
struct SidebarContents {
    entries: Vec<ListEntry>,
    notified_threads: HashSet<agent_ui::ThreadId>,
    project_header_indices: Vec<usize>,
    has_open_projects: bool,
}

impl SidebarContents {
    fn is_thread_notified(&self, thread_id: &agent_ui::ThreadId) -> bool {
        self.notified_threads.contains(thread_id)
    }
}

fn fuzzy_match_positions(query: &str, candidate: &str) -> Option<Vec<usize>> {
    let mut positions = Vec::new();
    let mut query_chars = query.chars().peekable();

    for (byte_idx, candidate_char) in candidate.char_indices() {
        if let Some(&query_char) = query_chars.peek() {
            if candidate_char.eq_ignore_ascii_case(&query_char) {
                positions.push(byte_idx);
                query_chars.next();
            }
        } else {
            break;
        }
    }

    if query_chars.peek().is_none() {
        Some(positions)
    } else {
        None
    }
}

// TODO: The mapping from workspace root paths to git repositories needs a
// unified approach across the codebase: this function, `AgentPanel::classify_worktrees`,
// thread persistence (which PathList is saved to the database), and thread
// querying (which PathList is used to read threads back). All of these need
// to agree on how repos are resolved for a given workspace, especially in
// multi-root and nested-repo configurations.
fn root_repository_snapshots(
    workspace: &Entity<Workspace>,
    cx: &App,
) -> impl Iterator<Item = project::git_store::RepositorySnapshot> {
    let path_list = workspace_path_list(workspace, cx);
    let project = workspace.read(cx).project().read(cx);
    project.repositories(cx).values().filter_map(move |repo| {
        let snapshot = repo.read(cx).snapshot();
        let is_root = path_list
            .paths()
            .iter()
            .any(|p| p.as_path() == snapshot.work_directory_abs_path.as_ref());
        is_root.then_some(snapshot)
    })
}

fn workspace_path_list(workspace: &Entity<Workspace>, cx: &App) -> PathList {
    PathList::new(&workspace.read(cx).root_paths(cx))
}

/// Derives worktree display info from a thread's stored path list.
///
/// For each path in the thread's `folder_paths`, produces a
/// [`WorktreeInfo`] with a short display name, full path, and whether
/// the worktree is the main checkout or a linked git worktree. When
/// multiple main paths exist and a linked worktree's short name alone
/// wouldn't identify which main project it belongs to, the main project
/// name is prefixed for disambiguation (e.g. `project:feature`).
///
fn worktree_info_from_thread_paths(worktree_paths: &WorktreePaths) -> Vec<WorktreeInfo> {
    let mut infos: Vec<WorktreeInfo> = Vec::new();
    let mut linked_short_names: Vec<(SharedString, SharedString)> = Vec::new();
    let mut unique_main_count = HashSet::new();

    for (main_path, folder_path) in worktree_paths.ordered_pairs() {
        unique_main_count.insert(main_path.clone());
        let is_linked = main_path != folder_path;

        if is_linked {
            let short_name = linked_worktree_short_name(main_path, folder_path).unwrap_or_default();
            let project_name = main_path
                .file_name()
                .map(|n| SharedString::from(n.to_string_lossy().to_string()))
                .unwrap_or_default();
            linked_short_names.push((short_name.clone(), project_name));
            infos.push(WorktreeInfo {
                name: short_name,
                full_path: SharedString::from(folder_path.display().to_string()),
                highlight_positions: Vec::new(),
                kind: ui::WorktreeKind::Linked,
            });
        } else {
            let Some(name) = folder_path.file_name() else {
                continue;
            };
            infos.push(WorktreeInfo {
                name: SharedString::from(name.to_string_lossy().to_string()),
                full_path: SharedString::from(folder_path.display().to_string()),
                highlight_positions: Vec::new(),
                kind: ui::WorktreeKind::Main,
            });
        }
    }

    // When the group has multiple main worktree paths and the thread's
    // folder paths don't all share the same short name, prefix each
    // linked worktree chip with its main project name so the user knows
    // which project it belongs to.
    let all_same_name = infos.len() > 1 && infos.iter().all(|i| i.name == infos[0].name);

    if unique_main_count.len() > 1 && !all_same_name {
        for (info, (_short_name, project_name)) in infos
            .iter_mut()
            .filter(|i| i.kind == ui::WorktreeKind::Linked)
            .zip(linked_short_names.iter())
        {
            info.name = SharedString::from(format!("{}:{}", project_name, info.name));
        }
    }

    infos
}

/// Shows a [`RemoteConnectionModal`] on the given workspace and establishes
/// an SSH connection. Suitable for passing to
/// [`MultiWorkspace::find_or_create_workspace`] as the `connect_remote`
/// argument.
fn connect_remote(
    modal_workspace: Entity<Workspace>,
    connection_options: RemoteConnectionOptions,
    window: &mut Window,
    cx: &mut Context<MultiWorkspace>,
) -> gpui::Task<anyhow::Result<Option<Entity<remote::RemoteClient>>>> {
    remote_connection::connect_with_modal(&modal_workspace, connection_options, window, cx)
}

/// The sidebar re-derives its entire entry list from scratch on every
/// change via `update_entries` → `rebuild_contents`. Avoid adding
/// incremental or inter-event coordination state — if something can
/// be computed from the current world state, compute it in the rebuild.
pub struct Sidebar {
    multi_workspace: WeakEntity<MultiWorkspace>,
    width: Pixels,
    focus_handle: FocusHandle,
    filter_editor: Entity<Editor>,
    list_state: ListState,
    contents: SidebarContents,
    /// The index of the list item that currently has the keyboard focus
    ///
    /// Note: This is NOT the same as the active item.
    selection: Option<usize>,
    /// Tracks which sidebar entry is currently active (highlighted).
    active_entry: Option<ActiveEntry>,
    hovered_thread_index: Option<usize>,

    /// Updated only in response to explicit user actions (clicking a
    /// thread, confirming in the thread switcher, etc.) — never from
    /// background data changes. Used to sort the thread switcher popup.
    thread_last_accessed: HashMap<acp::SessionId, DateTime<Utc>>,
    /// Updated when the user presses a key to send or queue a message.
    /// Used for sorting threads in the sidebar and as a secondary sort
    /// key in the thread switcher.
    thread_last_message_sent_or_queued: HashMap<agent_ui::ThreadId, DateTime<Utc>>,
    thread_switcher: Option<Entity<ThreadSwitcher>>,
    _thread_switcher_subscriptions: Vec<gpui::Subscription>,
    pending_thread_activation: Option<agent_ui::ThreadId>,
    view: SidebarView,
    restoring_tasks: HashMap<agent_ui::ThreadId, Task<()>>,
    recent_projects_popover_handle: PopoverMenuHandle<SidebarRecentProjects>,
    project_header_menu_ix: Option<usize>,
    _subscriptions: Vec<gpui::Subscription>,
    _draft_observations: Vec<gpui::Subscription>,
    reconciling: bool,
}

impl Sidebar {
    pub fn new(
        multi_workspace: Entity<MultiWorkspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        cx.on_focus_in(&focus_handle, window, Self::focus_in)
            .detach();

        let filter_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_use_modal_editing(true);
            editor.set_placeholder_text("Search…", window, cx);
            editor
        });

        cx.subscribe_in(
            &multi_workspace,
            window,
            |this, _multi_workspace, event: &MultiWorkspaceEvent, window, cx| match event {
                MultiWorkspaceEvent::ActiveWorkspaceChanged => {
                    this.sync_active_entry_from_active_workspace(cx);
                    this.observe_draft_editors(cx);
                    this.update_entries(cx);
                    this.reconcile_groups(window, cx);
                }
                MultiWorkspaceEvent::WorkspaceAdded(workspace) => {
                    this.subscribe_to_workspace(workspace, window, cx);
                    this.update_entries(cx);
                    this.reconcile_groups(window, cx);
                }
                MultiWorkspaceEvent::WorkspaceRemoved(_) => {
                    this.update_entries(cx);
                    this.reconcile_groups(window, cx);
                }
                MultiWorkspaceEvent::ProjectGroupKeyUpdated { old_key, new_key } => {
                    this.move_threads_for_key_change(old_key, new_key, cx);
                    this.update_entries(cx);
                    this.reconcile_groups(window, cx);
                }
            },
        )
        .detach();

        cx.subscribe(&filter_editor, |this: &mut Self, _, event, cx| {
            if let editor::EditorEvent::BufferEdited = event {
                let query = this.filter_editor.read(cx).text(cx);
                if !query.is_empty() {
                    this.selection.take();
                }
                this.update_entries(cx);
                if !query.is_empty() {
                    this.select_first_entry();
                }
            }
        })
        .detach();

        cx.observe(&ThreadMetadataStore::global(cx), |this, _store, cx| {
            this.update_entries(cx);
        })
        .detach();

        let workspaces: Vec<_> = multi_workspace.read(cx).workspaces().cloned().collect();
        cx.defer_in(window, move |this, window, cx| {
            for workspace in &workspaces {
                this.subscribe_to_workspace(workspace, window, cx);
            }
            this.update_entries(cx);
            this.reconcile_groups(window, cx);
        });

        Self {
            multi_workspace: multi_workspace.downgrade(),
            width: DEFAULT_WIDTH,
            focus_handle,
            filter_editor,
            list_state: ListState::new(0, gpui::ListAlignment::Top, px(1000.)),
            contents: SidebarContents::default(),
            selection: None,
            active_entry: None,
            hovered_thread_index: None,

            thread_last_accessed: HashMap::new(),
            thread_last_message_sent_or_queued: HashMap::new(),
            thread_switcher: None,
            _thread_switcher_subscriptions: Vec::new(),
            pending_thread_activation: None,
            view: SidebarView::default(),
            restoring_tasks: HashMap::new(),
            recent_projects_popover_handle: PopoverMenuHandle::default(),
            project_header_menu_ix: None,
            _subscriptions: Vec::new(),
            _draft_observations: Vec::new(),
            reconciling: false,
        }
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        cx.emit(workspace::SidebarEvent::SerializeNeeded);
    }

    fn is_group_collapsed(&self, key: &ProjectGroupKey, cx: &App) -> bool {
        self.multi_workspace
            .upgrade()
            .and_then(|mw| {
                mw.read(cx)
                    .group_state_by_key(key)
                    .map(|state| !state.expanded)
            })
            .unwrap_or(false)
    }

    fn group_extra_batches(&self, key: &ProjectGroupKey, cx: &App) -> usize {
        self.multi_workspace
            .upgrade()
            .and_then(|mw| {
                mw.read(cx)
                    .group_state_by_key(key)
                    .and_then(|state| state.visible_thread_count)
            })
            .unwrap_or(0)
    }

    fn set_group_expanded(&self, key: &ProjectGroupKey, expanded: bool, cx: &mut Context<Self>) {
        if let Some(mw) = self.multi_workspace.upgrade() {
            mw.update(cx, |mw, cx| {
                if let Some(state) = mw.group_state_by_key_mut(key) {
                    state.expanded = expanded;
                }
                mw.serialize(cx);
            });
        }
    }

    fn set_group_visible_thread_count(
        &self,
        key: &ProjectGroupKey,
        count: Option<usize>,
        cx: &mut Context<Self>,
    ) {
        if let Some(mw) = self.multi_workspace.upgrade() {
            mw.update(cx, |mw, cx| {
                if let Some(state) = mw.group_state_by_key_mut(key) {
                    state.visible_thread_count = count;
                }
                mw.serialize(cx);
            });
        }
    }

    fn is_active_workspace(&self, workspace: &Entity<Workspace>, cx: &App) -> bool {
        self.multi_workspace
            .upgrade()
            .map_or(false, |mw| mw.read(cx).workspace() == workspace)
    }

    fn subscribe_to_workspace(
        &mut self,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let project = workspace.read(cx).project().clone();

        cx.subscribe_in(
            &project,
            window,
            |this, project, event, window, cx| match event {
                ProjectEvent::WorktreeAdded(_)
                | ProjectEvent::WorktreeRemoved(_)
                | ProjectEvent::WorktreeOrderChanged => {
                    this.observe_draft_editors(cx);
                    this.update_entries(cx);
                    this.reconcile_groups(window, cx);
                }
                ProjectEvent::WorktreePathsChanged { old_worktree_paths } => {
                    this.move_thread_paths(project, old_worktree_paths, cx);
                    this.observe_draft_editors(cx);
                    this.update_entries(cx);
                    this.reconcile_groups(window, cx);
                }
                _ => {}
            },
        )
        .detach();

        let git_store = workspace.read(cx).project().read(cx).git_store().clone();
        cx.subscribe_in(
            &git_store,
            window,
            |this, _, event: &project::git_store::GitStoreEvent, _window, cx| {
                if matches!(
                    event,
                    project::git_store::GitStoreEvent::RepositoryUpdated(
                        _,
                        project::git_store::RepositoryEvent::GitWorktreeListChanged,
                        _,
                    )
                ) {
                    this.update_entries(cx);
                }
            },
        )
        .detach();

        cx.subscribe_in(
            workspace,
            window,
            |this, _workspace, event: &workspace::Event, window, cx| {
                if let workspace::Event::PanelAdded(view) = event {
                    if let Ok(agent_panel) = view.clone().downcast::<AgentPanel>() {
                        this.subscribe_to_agent_panel(&agent_panel, window, cx);
                        this.update_entries(cx);
                        this.reconcile_groups(window, cx);
                    }
                }
            },
        )
        .detach();

        self.observe_docks(workspace, cx);

        if let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
            self.subscribe_to_agent_panel(&agent_panel, window, cx);
            self.observe_draft_editors(cx);
        }
    }

    fn move_threads_for_key_change(
        &mut self,
        old_key: &ProjectGroupKey,
        new_key: &ProjectGroupKey,
        cx: &mut Context<Self>,
    ) {
        let old_main_paths = old_key.path_list();
        let new_main_paths = new_key.path_list();

        let added_paths: Vec<PathBuf> = new_main_paths
            .paths()
            .iter()
            .filter(|p| !old_main_paths.paths().contains(p))
            .cloned()
            .collect();

        let removed_paths: Vec<PathBuf> = old_main_paths
            .paths()
            .iter()
            .filter(|p| !new_main_paths.paths().contains(p))
            .cloned()
            .collect();

        if added_paths.is_empty() && removed_paths.is_empty() {
            return;
        }

        let remote_connection = old_key.host();
        ThreadMetadataStore::global(cx).update(cx, |store, store_cx| {
            store.change_worktree_paths_by_main(
                old_main_paths,
                remote_connection.as_ref(),
                |paths| {
                    for path in &added_paths {
                        paths.add_path(path, path);
                    }
                    for path in &removed_paths {
                        paths.remove_main_path(path);
                    }
                },
                store_cx,
            );
        });
    }

    fn move_thread_paths(
        &mut self,
        project: &Entity<project::Project>,
        old_paths: &WorktreePaths,
        cx: &mut Context<Self>,
    ) {
        let new_paths = project.read(cx).worktree_paths(cx);
        let old_folder_paths = old_paths.folder_path_list().clone();

        let added_pairs: Vec<_> = new_paths
            .ordered_pairs()
            .filter(|(main, folder)| {
                !old_paths
                    .ordered_pairs()
                    .any(|(old_main, old_folder)| old_main == *main && old_folder == *folder)
            })
            .map(|(m, f)| (m.clone(), f.clone()))
            .collect();

        let new_folder_paths = new_paths.folder_path_list();
        let removed_folder_paths: Vec<PathBuf> = old_folder_paths
            .paths()
            .iter()
            .filter(|p| !new_folder_paths.paths().contains(p))
            .cloned()
            .collect();

        if added_pairs.is_empty() && removed_folder_paths.is_empty() {
            return;
        }

        let remote_connection = project.read(cx).remote_connection_options(cx);
        ThreadMetadataStore::global(cx).update(cx, |store, store_cx| {
            store.change_worktree_paths(
                &old_folder_paths,
                remote_connection.as_ref(),
                |paths| {
                    for (main_path, folder_path) in &added_pairs {
                        paths.add_path(main_path, folder_path);
                    }
                    for path in &removed_folder_paths {
                        paths.remove_folder_path(path);
                    }
                },
                store_cx,
            );
        });
    }

    fn subscribe_to_agent_panel(
        &mut self,
        agent_panel: &Entity<AgentPanel>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.subscribe_in(
            agent_panel,
            window,
            |this, _agent_panel, event: &AgentPanelEvent, window, cx| match event {
                AgentPanelEvent::ActiveViewChanged => {
                    let resolved_pending_activation =
                        this.sync_active_entry_from_panel(_agent_panel, cx);
                    if resolved_pending_activation {
                        let active_workspace = this.active_workspace(cx);
                        if let Some(active_workspace) = active_workspace {
                            this.clear_empty_group_drafts(&active_workspace, cx);
                        }
                    }
                    this.observe_draft_editors(cx);
                    this.update_entries(cx);
                    this.reconcile_groups(window, cx);
                }
                AgentPanelEvent::ThreadFocused | AgentPanelEvent::RetainedThreadChanged => {
                    this.sync_active_entry_from_panel(_agent_panel, cx);
                    this.update_entries(cx);
                    this.reconcile_groups(window, cx);
                }
                AgentPanelEvent::MessageSentOrQueued { thread_id } => {
                    this.record_thread_message_sent(thread_id);
                    this.update_entries(cx);
                    this.reconcile_groups(window, cx);
                }
            },
        )
        .detach();
    }

    fn sync_active_entry_from_active_workspace(&mut self, cx: &App) {
        let panel = self
            .active_workspace(cx)
            .and_then(|ws| ws.read(cx).panel::<AgentPanel>(cx));
        if let Some(panel) = panel {
            self.sync_active_entry_from_panel(&panel, cx);
        }
    }

    /// Syncs `active_entry` from the agent panel's current state.
    /// Called from `ActiveViewChanged` — the panel has settled into its
    /// new view, so we can safely read it without race conditions.
    ///
    /// Also resolves `pending_thread_activation` when the panel's
    /// active thread matches the pending activation.
    fn sync_active_entry_from_panel(&mut self, agent_panel: &Entity<AgentPanel>, cx: &App) -> bool {
        let Some(active_workspace) = self.active_workspace(cx) else {
            return false;
        };

        // Only sync when the event comes from the active workspace's panel.
        let is_active_panel = active_workspace
            .read(cx)
            .panel::<AgentPanel>(cx)
            .is_some_and(|p| p == *agent_panel);
        if !is_active_panel {
            return false;
        }

        let panel = agent_panel.read(cx);

        if let Some(pending_thread_id) = self.pending_thread_activation {
            let panel_thread_id = panel
                .active_conversation_view()
                .map(|cv| cv.read(cx).parent_id());

            if panel_thread_id == Some(pending_thread_id) {
                let session_id = panel
                    .active_agent_thread(cx)
                    .map(|thread| thread.read(cx).session_id().clone());
                self.active_entry = Some(ActiveEntry {
                    thread_id: pending_thread_id,
                    session_id,
                    workspace: active_workspace,
                });
                self.pending_thread_activation = None;
                return true;
            }
            // Pending activation not yet resolved — keep current active_entry.
            return false;
        }

        if let Some(thread_id) = panel.active_thread_id(cx) {
            let session_id = panel
                .active_agent_thread(cx)
                .map(|thread| thread.read(cx).session_id().clone());
            self.active_entry = Some(ActiveEntry {
                thread_id,
                session_id,
                workspace: active_workspace,
            });
        }

        false
    }

    fn observe_docks(&mut self, workspace: &Entity<Workspace>, cx: &mut Context<Self>) {
        let docks: Vec<_> = workspace
            .read(cx)
            .all_docks()
            .into_iter()
            .cloned()
            .collect();
        let workspace = workspace.downgrade();
        for dock in docks {
            let workspace = workspace.clone();
            cx.observe(&dock, move |this, _dock, cx| {
                let Some(workspace) = workspace.upgrade() else {
                    return;
                };
                if !this.is_active_workspace(&workspace, cx) {
                    return;
                }

                cx.notify();
            })
            .detach();
        }
    }

    fn observe_draft_editors(&mut self, cx: &mut Context<Self>) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            self._draft_observations.clear();
            return;
        };

        // Collect conversation views up front to avoid holding a
        // borrow on `cx` across `cx.observe` calls.
        let conversation_views: Vec<_> = multi_workspace
            .read(cx)
            .workspaces()
            .filter_map(|ws| ws.read(cx).panel::<AgentPanel>(cx))
            .flat_map(|panel| panel.read(cx).conversation_views())
            .collect();

        let mut subscriptions = Vec::with_capacity(conversation_views.len());
        for cv in conversation_views {
            if let Some(thread_view) = cv.read(cx).active_thread() {
                let editor = thread_view.read(cx).message_editor.clone();
                subscriptions.push(cx.observe(&editor, |this, _editor, cx| {
                    this.update_entries(cx);
                }));
            } else {
                subscriptions.push(cx.observe(&cv, |this, _cv, cx| {
                    this.observe_draft_editors(cx);
                    this.update_entries(cx);
                }));
            }
        }

        self._draft_observations = subscriptions;
    }

    fn clean_mention_links(input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        let mut remaining = input;

        while let Some(start) = remaining.find("[@") {
            result.push_str(&remaining[..start]);
            let after_bracket = &remaining[start + 1..]; // skip '['
            if let Some(close_bracket) = after_bracket.find("](") {
                let mention = &after_bracket[..close_bracket]; // "@something"
                let after_link_start = &after_bracket[close_bracket + 2..]; // after "]("
                if let Some(close_paren) = after_link_start.find(')') {
                    result.push_str(mention);
                    remaining = &after_link_start[close_paren + 1..];
                    continue;
                }
            }
            // Couldn't parse full link syntax — emit the literal "[@" and move on.
            result.push_str("[@");
            remaining = &remaining[start + 2..];
        }
        result.push_str(remaining);
        result
    }

    /// Opens a new workspace for a group that has no open workspaces.
    fn open_workspace_for_group(
        &mut self,
        project_group_key: &ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };
        let path_list = project_group_key.path_list().clone();
        let host = project_group_key.host();
        let provisional_key = Some(project_group_key.clone());
        let active_workspace = multi_workspace.read(cx).workspace().clone();
        let modal_workspace = active_workspace.clone();

        let task = multi_workspace.update(cx, |this, cx| {
            this.find_or_create_workspace(
                path_list,
                host,
                provisional_key,
                |options, window, cx| connect_remote(active_workspace, options, window, cx),
                window,
                cx,
            )
        });

        cx.spawn_in(window, async move |_this, cx| {
            let result = task.await;
            remote_connection::dismiss_connection_modal(&modal_workspace, cx);
            result?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn open_workspace_and_create_draft(
        &mut self,
        project_group_key: &ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let path_list = project_group_key.path_list().clone();
        let host = project_group_key.host();
        let provisional_key = Some(project_group_key.clone());
        let active_workspace = multi_workspace.read(cx).workspace().clone();

        let task = multi_workspace.update(cx, |this, cx| {
            this.find_or_create_workspace(
                path_list,
                host,
                provisional_key,
                |options, window, cx| connect_remote(active_workspace, options, window, cx),
                window,
                cx,
            )
        });

        cx.spawn_in(window, async move |this, cx| {
            let workspace = task.await?;
            this.update_in(cx, |this, window, cx| {
                this.create_new_thread(&workspace, window, cx);
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    /// Rebuilds the sidebar contents from current workspace and thread state.
    ///
    /// Iterates [`MultiWorkspace::project_group_keys`] to determine project
    /// groups, then populates thread entries from the metadata store and
    /// merges live thread info from active agent panels.
    ///
    /// Aim for a single forward pass over workspaces and threads plus an
    /// O(T log T) sort. Avoid adding extra scans over the data.
    ///
    /// Properties:
    ///
    /// - Should always show every workspace in the multiworkspace
    ///     - If you have no threads, and two workspaces for the worktree and the main workspace, make sure at least one is shown
    /// - Should always show every thread, associated with each workspace in the multiworkspace
    /// - After every build_contents, our "active" state should exactly match the current workspace's, current agent panel's current thread.
    fn rebuild_contents(&mut self, cx: &App) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };
        let mw = multi_workspace.read(cx);
        let workspaces: Vec<_> = mw.workspaces().cloned().collect();
        let active_workspace = Some(mw.workspace().clone());

        let agent_server_store = workspaces
            .first()
            .map(|ws| ws.read(cx).project().read(cx).agent_server_store().clone());

        let query = self.filter_editor.read(cx).text(cx);

        let previous = mem::take(&mut self.contents);

        let old_statuses: HashMap<acp::SessionId, AgentThreadStatus> = previous
            .entries
            .iter()
            .filter_map(|entry| match entry {
                ListEntry::Thread(thread) if thread.is_live => {
                    let sid = thread.metadata.session_id.clone()?;
                    Some((sid, thread.status))
                }
                _ => None,
            })
            .collect();

        let mut entries = Vec::new();
        let mut notified_threads = previous.notified_threads;
        let mut current_session_ids: HashSet<acp::SessionId> = HashSet::new();
        let mut current_thread_ids: HashSet<agent_ui::ThreadId> = HashSet::new();
        let mut project_header_indices: Vec<usize> = Vec::new();
        let mut seen_thread_ids: HashSet<agent_ui::ThreadId> = HashSet::new();

        let has_open_projects = workspaces
            .iter()
            .any(|ws| !workspace_path_list(ws, cx).paths().is_empty());

        let resolve_agent_icon = |agent_id: &AgentId| -> (IconName, Option<SharedString>) {
            let agent = Agent::from(agent_id.clone());
            let icon = match agent {
                Agent::NativeAgent => IconName::ZedAgent,
                Agent::Custom { .. } => IconName::Terminal,
            };
            let icon_from_external_svg = agent_server_store
                .as_ref()
                .and_then(|store| store.read(cx).agent_icon(&agent_id));
            (icon, icon_from_external_svg)
        };

        let groups = mw.project_groups(cx);

        let mut all_paths: Vec<PathBuf> = groups
            .iter()
            .flat_map(|group| group.key.path_list().paths().iter().cloned())
            .collect();
        all_paths.sort();
        all_paths.dedup();
        let path_details =
            util::disambiguate::compute_disambiguation_details(&all_paths, |path, detail| {
                project::path_suffix(path, detail)
            });
        let path_detail_map: HashMap<PathBuf, usize> =
            all_paths.into_iter().zip(path_details).collect();

        for group in &groups {
            let group_key = &group.key;
            let group_workspaces = &group.workspaces;
            if group_key.path_list().paths().is_empty() {
                continue;
            }

            let label = group_key.display_name(&path_detail_map);

            let is_collapsed = self.is_group_collapsed(group_key, cx);
            let should_load_threads = !is_collapsed || !query.is_empty();

            let is_active = active_workspace
                .as_ref()
                .is_some_and(|active| group_workspaces.contains(active));

            // Collect live thread infos from all workspaces in this group.
            let live_infos: Vec<_> = group_workspaces
                .iter()
                .flat_map(|ws| all_thread_infos_for_workspace(ws, cx))
                .collect();

            let mut threads: Vec<ThreadEntry> = Vec::new();
            let mut has_running_threads = false;
            let mut waiting_thread_count: usize = 0;

            if should_load_threads {
                let thread_store = ThreadMetadataStore::global(cx);

                // Build a lookup from workspace root paths to their workspace
                // entity, used to assign ThreadEntryWorkspace::Open for threads
                // whose folder_paths match an open workspace.
                let workspace_by_path_list: HashMap<PathList, &Entity<Workspace>> =
                    group_workspaces
                        .iter()
                        .map(|ws| (workspace_path_list(ws, cx), ws))
                        .collect();

                // Resolve a ThreadEntryWorkspace for a thread row. If any open
                // workspace's root paths match the thread's folder_paths, use
                // Open; otherwise use Closed.
                let resolve_workspace = |row: &ThreadMetadata| -> ThreadEntryWorkspace {
                    workspace_by_path_list
                        .get(row.folder_paths())
                        .map(|ws| ThreadEntryWorkspace::Open((*ws).clone()))
                        .unwrap_or_else(|| ThreadEntryWorkspace::Closed {
                            folder_paths: row.folder_paths().clone(),
                            project_group_key: group_key.clone(),
                        })
                };

                // Build a ThreadEntry from a metadata row.
                let make_thread_entry =
                    |row: ThreadMetadata, workspace: ThreadEntryWorkspace| -> ThreadEntry {
                        let (icon, icon_from_external_svg) = resolve_agent_icon(&row.agent_id);
                        let worktrees = worktree_info_from_thread_paths(&row.worktree_paths);
                        let is_draft = row.is_draft();
                        ThreadEntry {
                            metadata: row,
                            icon,
                            icon_from_external_svg,
                            status: AgentThreadStatus::default(),
                            workspace,
                            is_live: false,
                            is_background: false,
                            is_title_generating: false,
                            is_draft,
                            highlight_positions: Vec::new(),
                            worktrees,
                            diff_stats: DiffStats::default(),
                        }
                    };

                // Main code path: one query per group via main_worktree_paths.
                // The main_worktree_paths column is set on all new threads and
                // points to the group's canonical paths regardless of which
                // linked worktree the thread was opened in.
                for row in thread_store
                    .read(cx)
                    .entries_for_main_worktree_path(group_key.path_list())
                    .cloned()
                {
                    if !seen_thread_ids.insert(row.thread_id) {
                        continue;
                    }
                    let workspace = resolve_workspace(&row);
                    threads.push(make_thread_entry(row, workspace));
                }

                // Legacy threads did not have `main_worktree_paths` populated, so they
                // must be queried by their `folder_paths`.

                // Load any legacy threads for the main worktrees of this project group.
                for row in thread_store
                    .read(cx)
                    .entries_for_path(group_key.path_list())
                    .cloned()
                {
                    if !seen_thread_ids.insert(row.thread_id) {
                        continue;
                    }
                    let workspace = resolve_workspace(&row);
                    threads.push(make_thread_entry(row, workspace));
                }

                // Load any legacy threads for any single linked wortree of this project group.
                let mut linked_worktree_paths = HashSet::new();
                for workspace in group_workspaces {
                    if workspace.read(cx).visible_worktrees(cx).count() != 1 {
                        continue;
                    }
                    for snapshot in root_repository_snapshots(workspace, cx) {
                        for linked_worktree in snapshot.linked_worktrees() {
                            linked_worktree_paths.insert(linked_worktree.path.clone());
                        }
                    }
                }
                for path in linked_worktree_paths {
                    let worktree_path_list = PathList::new(std::slice::from_ref(&path));
                    for row in thread_store
                        .read(cx)
                        .entries_for_path(&worktree_path_list)
                        .cloned()
                    {
                        if !seen_thread_ids.insert(row.thread_id) {
                            continue;
                        }
                        threads.push(make_thread_entry(
                            row,
                            ThreadEntryWorkspace::Closed {
                                folder_paths: worktree_path_list.clone(),
                                project_group_key: group_key.clone(),
                            },
                        ));
                    }
                }

                // Build a lookup from live_infos and compute running/waiting
                // counts in a single pass.
                let mut live_info_by_session: HashMap<&acp::SessionId, &ActiveThreadInfo> =
                    HashMap::new();
                for info in &live_infos {
                    live_info_by_session.insert(&info.session_id, info);
                    if info.status == AgentThreadStatus::Running {
                        has_running_threads = true;
                    }
                    if info.status == AgentThreadStatus::WaitingForConfirmation {
                        waiting_thread_count += 1;
                    }
                }

                // Merge live info into threads and update notification state
                // in a single pass.
                for thread in &mut threads {
                    if let Some(session_id) = &thread.metadata.session_id {
                        if let Some(info) = live_info_by_session.get(session_id) {
                            thread.apply_active_info(info);
                        }
                    }

                    let session_id = &thread.metadata.session_id;
                    let is_active_thread = self.active_entry.as_ref().is_some_and(|entry| {
                        entry.is_active_thread(&thread.metadata.thread_id)
                            && active_workspace
                                .as_ref()
                                .is_some_and(|active| active == entry.workspace())
                    });

                    if thread.status == AgentThreadStatus::Completed
                        && !is_active_thread
                        && session_id.as_ref().and_then(|sid| old_statuses.get(sid))
                            == Some(&AgentThreadStatus::Running)
                    {
                        notified_threads.insert(thread.metadata.thread_id);
                    }

                    if is_active_thread && !thread.is_background {
                        notified_threads.remove(&thread.metadata.thread_id);
                    }
                }

                threads.sort_by(|a, b| {
                    let a_time = a
                        .metadata
                        .session_id
                        .as_ref()
                        .and_then(|_sid| {
                            self.thread_last_message_sent_or_queued
                                .get(&a.metadata.thread_id)
                        })
                        .copied()
                        .or(a.metadata.created_at)
                        .or(Some(a.metadata.updated_at));
                    let b_time = b
                        .metadata
                        .session_id
                        .as_ref()
                        .and_then(|_sid| {
                            self.thread_last_message_sent_or_queued
                                .get(&b.metadata.thread_id)
                        })
                        .copied()
                        .or(b.metadata.created_at)
                        .or(Some(b.metadata.updated_at));
                    b_time.cmp(&a_time)
                });
            } else {
                for info in live_infos {
                    if info.status == AgentThreadStatus::Running {
                        has_running_threads = true;
                    }
                    if info.status == AgentThreadStatus::WaitingForConfirmation {
                        waiting_thread_count += 1;
                    }
                }
            }

            let has_threads = if !threads.is_empty() {
                true
            } else {
                let store = ThreadMetadataStore::global(cx).read(cx);
                store
                    .entries_for_main_worktree_path(group_key.path_list())
                    .next()
                    .is_some()
                    || store
                        .entries_for_path(group_key.path_list())
                        .next()
                        .is_some()
            };

            if !query.is_empty() {
                let workspace_highlight_positions =
                    fuzzy_match_positions(&query, &label).unwrap_or_default();
                let workspace_matched = !workspace_highlight_positions.is_empty();

                let mut matched_threads: Vec<ThreadEntry> = Vec::new();
                for mut thread in threads {
                    let title: &str = thread
                        .metadata
                        .title
                        .as_ref()
                        .map_or(DEFAULT_THREAD_TITLE, |t| t.as_ref());
                    if let Some(positions) = fuzzy_match_positions(&query, title) {
                        thread.highlight_positions = positions;
                    }
                    let mut worktree_matched = false;
                    for worktree in &mut thread.worktrees {
                        if let Some(positions) = fuzzy_match_positions(&query, &worktree.name) {
                            worktree.highlight_positions = positions;
                            worktree_matched = true;
                        }
                    }
                    if workspace_matched
                        || !thread.highlight_positions.is_empty()
                        || worktree_matched
                    {
                        matched_threads.push(thread);
                    }
                }

                if matched_threads.is_empty() && !workspace_matched {
                    continue;
                }

                project_header_indices.push(entries.len());
                entries.push(ListEntry::ProjectHeader {
                    key: group_key.clone(),
                    label,
                    highlight_positions: workspace_highlight_positions,
                    has_running_threads,
                    waiting_thread_count,
                    is_active,
                    has_threads,
                });

                for thread in matched_threads {
                    if let Some(sid) = thread.metadata.session_id.clone() {
                        current_session_ids.insert(sid);
                    }
                    current_thread_ids.insert(thread.metadata.thread_id);
                    entries.push(thread.into());
                }
            } else {
                project_header_indices.push(entries.len());
                entries.push(ListEntry::ProjectHeader {
                    key: group_key.clone(),
                    label,
                    highlight_positions: Vec::new(),
                    has_running_threads,
                    waiting_thread_count,
                    is_active,
                    has_threads,
                });

                if is_collapsed {
                    continue;
                }

                {
                    // Override titles with editor text for drafts and
                    // threads that still have the default placeholder
                    // title (panel considers them drafts even if they
                    // have a session_id).
                    for thread in &mut threads {
                        let needs_title_override =
                            thread.is_draft || thread.metadata.title.is_none();
                        if needs_title_override {
                            if let ThreadEntryWorkspace::Open(workspace) = &thread.workspace {
                                if let Some(text) =
                                    self.read_draft_text(thread.metadata.thread_id, workspace, cx)
                                {
                                    thread.metadata.title = Some(text);
                                }
                            }
                        }
                    }
                }

                let total = threads.len();

                let extra_batches = self.group_extra_batches(&group_key, cx);
                let threads_to_show =
                    DEFAULT_THREADS_SHOWN + (extra_batches * DEFAULT_THREADS_SHOWN);
                let count = threads_to_show.min(total);

                let mut promoted_threads: HashSet<agent_ui::ThreadId> = HashSet::new();

                // Build visible entries in a single pass. Threads within
                // the cutoff are always shown. Threads beyond it are shown
                // only if they should be promoted (running, waiting, or
                // focused)
                for (index, thread) in threads.into_iter().enumerate() {
                    let is_hidden = index >= count;

                    if is_hidden {
                        let is_notified = notified_threads.contains(&thread.metadata.thread_id);
                        let is_promoted = thread.status == AgentThreadStatus::Running
                            || thread.status == AgentThreadStatus::WaitingForConfirmation
                            || is_notified
                            || self.active_entry.as_ref().is_some_and(|active| {
                                active.matches_entry(&ListEntry::Thread(thread.clone()))
                            });
                        if is_promoted {
                            promoted_threads.insert(thread.metadata.thread_id);
                        }
                        let is_in_promoted = promoted_threads.contains(&thread.metadata.thread_id);
                        if !is_in_promoted {
                            continue;
                        }
                    }

                    if let Some(sid) = &thread.metadata.session_id {
                        current_session_ids.insert(sid.clone());
                    }
                    current_thread_ids.insert(thread.metadata.thread_id);
                    entries.push(thread.into());
                }

                let visible = count + promoted_threads.len();
                let is_fully_expanded = visible >= total;

                if total > DEFAULT_THREADS_SHOWN {
                    entries.push(ListEntry::ViewMore {
                        key: group_key.clone(),
                        is_fully_expanded,
                    });
                }
            }
        }

        notified_threads.retain(|id| current_thread_ids.contains(id));

        self.thread_last_accessed
            .retain(|id, _| current_session_ids.contains(id));
        self.thread_last_message_sent_or_queued
            .retain(|id, _| current_thread_ids.contains(id));

        self.contents = SidebarContents {
            entries,
            notified_threads,
            project_header_indices,
            has_open_projects,
        };
    }

    /// Gives each workspace's sidebar delegate a chance to reconcile its
    /// project group (e.g. create a draft when the group is empty).
    ///
    /// Called at the top of `render` so we have `Window` + `Context`
    /// available for panel mutations.
    fn reconcile_groups(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.reconciling
            || self.pending_thread_activation.is_some()
            || !self.restoring_tasks.is_empty()
        {
            return;
        }
        self.reconciling = true;

        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            self.reconciling = false;
            return;
        };

        let empty_group_keys: Vec<ProjectGroupKey> = self
            .contents
            .entries
            .iter()
            .filter_map(|entry| match entry {
                ListEntry::ProjectHeader {
                    key,
                    has_threads: false,
                    ..
                } => Some(key.clone()),
                _ => None,
            })
            .collect();

        if empty_group_keys.is_empty() {
            self.reconciling = false;
            return;
        }

        let mut did_reconcile = false;

        for key in &empty_group_keys {
            let workspace = {
                let mw = multi_workspace.read(cx);
                let active = mw.workspace().clone();
                if active.read(cx).project_group_key(cx) == *key {
                    Some(active)
                } else {
                    mw.workspace_for_paths(key.path_list(), key.host().as_ref(), cx)
                }
            };

            let Some(workspace) = workspace else {
                continue;
            };

            let Some(delegate) = workspace.read(cx).sidebar_delegate() else {
                continue;
            };

            let changed = workspace.update(cx, |workspace, cx| {
                delegate.reconcile_group(workspace, key, window, cx)
            });

            did_reconcile |= changed;
        }

        if did_reconcile {
            self.update_entries(cx);
        }

        self.reconciling = false;
    }

    /// Rebuilds the sidebar's visible entries from already-cached state.
    fn update_entries(&mut self, cx: &mut Context<Self>) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };
        if !multi_workspace.read(cx).multi_workspace_enabled(cx) {
            return;
        }

        let had_notifications = self.has_notifications(cx);
        let scroll_position = self.list_state.logical_scroll_top();

        self.rebuild_contents(cx);

        self.list_state.reset(self.contents.entries.len());
        self.list_state.scroll_to(scroll_position);

        if had_notifications != self.has_notifications(cx) {
            multi_workspace.update(cx, |_, cx| {
                cx.notify();
            });
        }

        cx.notify();
    }

    fn select_first_entry(&mut self) {
        self.selection = self
            .contents
            .entries
            .iter()
            .position(|entry| matches!(entry, ListEntry::Thread(_)))
            .or_else(|| {
                if self.contents.entries.is_empty() {
                    None
                } else {
                    Some(0)
                }
            });
    }

    fn render_list_entry(
        &mut self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(entry) = self.contents.entries.get(ix) else {
            return div().into_any_element();
        };
        let is_focused = self.focus_handle.is_focused(window);
        // is_selected means the keyboard selector is here.
        let is_selected = is_focused && self.selection == Some(ix);

        let is_group_header_after_first =
            ix > 0 && matches!(entry, ListEntry::ProjectHeader { .. });

        let is_active = self
            .active_entry
            .as_ref()
            .is_some_and(|active| active.matches_entry(entry));

        let rendered = match entry {
            ListEntry::ProjectHeader {
                key,
                label,
                highlight_positions,
                has_running_threads,
                waiting_thread_count,
                is_active: is_active_group,
                has_threads,
            } => self.render_project_header(
                ix,
                false,
                key,
                label,
                highlight_positions,
                *has_running_threads,
                *waiting_thread_count,
                *is_active_group,
                is_selected,
                *has_threads,
                cx,
            ),
            ListEntry::Thread(thread) => self.render_thread(ix, thread, is_active, is_selected, cx),
            ListEntry::ViewMore {
                key,
                is_fully_expanded,
            } => self.render_view_more(ix, key, *is_fully_expanded, is_selected, cx),
        };

        if is_group_header_after_first {
            v_flex()
                .w_full()
                .border_t_1()
                .border_color(cx.theme().colors().border)
                .child(rendered)
                .into_any_element()
        } else {
            rendered
        }
    }

    fn render_remote_project_icon(
        &self,
        ix: usize,
        host: Option<&RemoteConnectionOptions>,
    ) -> Option<AnyElement> {
        let remote_icon_per_type = match host? {
            RemoteConnectionOptions::Wsl(_) => IconName::Linux,
            RemoteConnectionOptions::Docker(_) => IconName::Box,
            _ => IconName::Server,
        };

        Some(
            div()
                .id(format!("remote-project-icon-{}", ix))
                .child(
                    Icon::new(remote_icon_per_type)
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                )
                .tooltip(Tooltip::text("Remote Project"))
                .into_any_element(),
        )
    }

    fn render_project_header(
        &self,
        ix: usize,
        is_sticky: bool,
        key: &ProjectGroupKey,
        label: &SharedString,
        highlight_positions: &[usize],
        has_running_threads: bool,
        waiting_thread_count: usize,
        is_active: bool,
        is_focused: bool,
        has_threads: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let host = key.host();

        let id_prefix = if is_sticky { "sticky-" } else { "" };
        let id = SharedString::from(format!("{id_prefix}project-header-{ix}"));
        let disclosure_id = SharedString::from(format!("disclosure-{ix}"));
        let group_name = SharedString::from(format!("{id_prefix}header-group-{ix}"));

        let is_collapsed = self.is_group_collapsed(key, cx);
        let (disclosure_icon, disclosure_tooltip) = if is_collapsed {
            (IconName::ChevronRight, "Expand Project")
        } else {
            (IconName::ChevronDown, "Collapse Project")
        };

        let key_for_toggle = key.clone();
        let key_for_collapse = key.clone();
        let view_more_expanded = self.group_extra_batches(key, cx) > 0;

        let label = if highlight_positions.is_empty() {
            Label::new(label.clone())
                .when(!is_active, |this| this.color(Color::Muted))
                .into_any_element()
        } else {
            HighlightedLabel::new(label.clone(), highlight_positions.to_vec())
                .when(!is_active, |this| this.color(Color::Muted))
                .into_any_element()
        };

        let color = cx.theme().colors();
        let sidebar_base_bg = color
            .title_bar_background
            .blend(color.panel_background.opacity(0.25));

        let base_bg = color.background.blend(sidebar_base_bg);

        let hover_base = color
            .element_active
            .blend(color.element_background.opacity(0.2));
        let hover_solid = base_bg.blend(hover_base);
        let real_hover_color = if is_active { base_bg } else { hover_solid };

        let group_name_for_gradient = group_name.clone();
        let gradient_overlay = move || {
            GradientFade::new(base_bg, real_hover_color, real_hover_color)
                .width(px(64.0))
                .right(px(-2.0))
                .gradient_stop(0.75)
                .group_name(group_name_for_gradient.clone())
        };

        let is_ellipsis_menu_open = self.project_header_menu_ix == Some(ix);

        h_flex()
            .id(id)
            .group(&group_name)
            .h(Tab::content_height(cx))
            .relative()
            .w_full()
            .pl(px(5.))
            .pr_1p5()
            .justify_between()
            .border_1()
            .map(|this| {
                if is_focused {
                    this.border_color(color.border_focused)
                } else {
                    this.border_color(gpui::transparent_black())
                }
            })
            .child(
                h_flex()
                    .relative()
                    .min_w_0()
                    .w_full()
                    .gap(px(5.))
                    .child(
                        IconButton::new(disclosure_id, disclosure_icon)
                            .shape(ui::IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Custom(cx.theme().colors().icon_muted.opacity(0.5)))
                            .tooltip(Tooltip::text(disclosure_tooltip))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.selection = None;
                                this.toggle_collapse(&key_for_toggle, window, cx);
                            })),
                    )
                    .child(label)
                    .when_some(
                        self.render_remote_project_icon(ix, host.as_ref()),
                        |this, icon| this.child(icon),
                    )
                    .when(is_collapsed, |this| {
                        this.when(has_running_threads, |this| {
                            this.child(
                                Icon::new(IconName::LoadCircle)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted)
                                    .with_rotate_animation(2),
                            )
                        })
                        .when(waiting_thread_count > 0, |this| {
                            let tooltip_text = if waiting_thread_count == 1 {
                                "1 thread is waiting for confirmation".to_string()
                            } else {
                                format!(
                                    "{waiting_thread_count} threads are waiting for confirmation",
                                )
                            };
                            this.child(
                                div()
                                    .id(format!("{id_prefix}waiting-indicator-{ix}"))
                                    .child(
                                        Icon::new(IconName::Warning)
                                            .size(IconSize::XSmall)
                                            .color(Color::Warning),
                                    )
                                    .tooltip(Tooltip::text(tooltip_text)),
                            )
                        })
                    }),
            )
            .child(gradient_overlay())
            .child(
                h_flex()
                    .when(!is_ellipsis_menu_open, |this| {
                        this.visible_on_hover(&group_name)
                    })
                    .child(gradient_overlay())
                    .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    })
                    .child(self.render_project_header_ellipsis_menu(ix, id_prefix, key, cx))
                    .when(view_more_expanded && !is_collapsed, |this| {
                        this.child(
                            IconButton::new(
                                SharedString::from(format!(
                                    "{id_prefix}project-header-collapse-{ix}",
                                )),
                                IconName::ListCollapse,
                            )
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Collapse Displayed Threads"))
                            .on_click(cx.listener({
                                let key_for_collapse = key_for_collapse.clone();
                                move |this, _, _window, cx| {
                                    this.selection = None;
                                    this.set_group_visible_thread_count(
                                        &key_for_collapse,
                                        None,
                                        cx,
                                    );
                                    this.update_entries(cx);
                                }
                            })),
                        )
                    })
                    .child({
                        let key = key.clone();
                        let focus_handle = self.focus_handle.clone();

                        IconButton::new(
                            SharedString::from(format!(
                                "{id_prefix}project-header-new-thread-{ix}",
                            )),
                            IconName::Plus,
                        )
                        .icon_size(IconSize::Small)
                        .tooltip(move |_, cx| {
                            Tooltip::for_action_in(
                                "Start New Agent Thread",
                                &NewThread,
                                &focus_handle,
                                cx,
                            )
                        })
                        .on_click(cx.listener(
                            move |this, _, window, cx| {
                                this.set_group_expanded(&key, true, cx);
                                this.selection = None;
                                // If the active workspace belongs to this
                                // group, use it (preserves linked worktree
                                // context). Otherwise resolve from the key.
                                let workspace = this.multi_workspace.upgrade().and_then(|mw| {
                                    let mw = mw.read(cx);
                                    let active = mw.workspace().clone();
                                    let active_key = active.read(cx).project_group_key(cx);
                                    if active_key == key {
                                        Some(active)
                                    } else {
                                        mw.workspace_for_paths(
                                            key.path_list(),
                                            key.host().as_ref(),
                                            cx,
                                        )
                                    }
                                });
                                if let Some(workspace) = workspace {
                                    this.create_new_thread(&workspace, window, cx);
                                } else {
                                    this.open_workspace_and_create_draft(&key, window, cx);
                                }
                            },
                        ))
                    }),
            )
            .map(|this| {
                if !has_threads && is_active {
                    this
                } else {
                    let key = key.clone();
                    this.cursor_pointer()
                        .when(!is_active, |this| this.hover(|s| s.bg(hover_solid)))
                        .tooltip(Tooltip::text("Open Workspace"))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            if let Some(workspace) = this.multi_workspace.upgrade().and_then(|mw| {
                                mw.read(cx).workspace_for_paths(
                                    key.path_list(),
                                    key.host().as_ref(),
                                    cx,
                                )
                            }) {
                                // Just activate the workspace. The
                                // AgentPanel remembers what was last
                                // shown, so the user returns to whatever
                                // thread/draft they were looking at.
                                this.activate_workspace(&workspace, window, cx);
                            } else {
                                this.open_workspace_for_group(&key, window, cx);
                            }
                        }))
                }
            })
            .into_any_element()
    }

    fn render_project_header_ellipsis_menu(
        &self,
        ix: usize,
        id_prefix: &str,
        project_group_key: &ProjectGroupKey,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let multi_workspace = self.multi_workspace.clone();
        let this = cx.weak_entity();
        let project_group_key = project_group_key.clone();

        PopoverMenu::new(format!("{id_prefix}project-header-menu-{ix}"))
            .on_open(Rc::new({
                let this = this.clone();
                move |_window, cx| {
                    this.update(cx, |sidebar, cx| {
                        sidebar.project_header_menu_ix = Some(ix);
                        cx.notify();
                    })
                    .ok();
                }
            }))
            .menu(move |window, cx| {
                let multi_workspace = multi_workspace.clone();
                let project_group_key = project_group_key.clone();

                let has_multiple_projects = multi_workspace
                    .read_with(cx, |mw, _| mw.project_group_keys().len() >= 2)
                    .unwrap_or(false);

                let menu =
                    ContextMenu::build_persistent(window, cx, move |menu, _window, menu_cx| {
                        let weak_menu = menu_cx.weak_entity();
                        let mut menu = menu
                            .header("Project Folders")
                            .end_slot_action(Box::new(menu::EndSlot));

                        for path in project_group_key.path_list().paths() {
                            let Some(name) = path.file_name() else {
                                continue;
                            };
                            let name: SharedString = name.to_string_lossy().into_owned().into();
                            let path = path.clone();
                            let project_group_key = project_group_key.clone();
                            let multi_workspace = multi_workspace.clone();
                            let weak_menu = weak_menu.clone();
                            menu = menu.entry_with_end_slot_on_hover(
                                name.clone(),
                                None,
                                |_, _| {},
                                IconName::Close,
                                "Remove Folder".into(),
                                move |_window, cx| {
                                    multi_workspace
                                        .update(cx, |multi_workspace, cx| {
                                            multi_workspace.remove_folder_from_project_group(
                                                &project_group_key,
                                                &path,
                                                cx,
                                            );
                                        })
                                        .ok();
                                    weak_menu.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
                                },
                            );
                        }

                        let menu = menu.separator().entry(
                            "Add Folder to Project",
                            Some(Box::new(AddFolderToProject)),
                            {
                                let project_group_key = project_group_key.clone();
                                let multi_workspace = multi_workspace.clone();
                                let weak_menu = weak_menu.clone();
                                move |window, cx| {
                                    multi_workspace
                                        .update(cx, |multi_workspace, cx| {
                                            multi_workspace.prompt_to_add_folders_to_project_group(
                                                project_group_key.clone(),
                                                window,
                                                cx,
                                            );
                                        })
                                        .ok();
                                    weak_menu.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
                                }
                            },
                        );

                        let menu = if project_group_key.host().is_none() && has_multiple_projects {
                            menu.entry(
                                "Open Project in New Window",
                                Some(Box::new(workspace::MoveProjectToNewWindow)),
                                {
                                    let project_group_key = project_group_key.clone();
                                    let multi_workspace = multi_workspace.clone();
                                    move |window, cx| {
                                        multi_workspace
                                            .update(cx, |multi_workspace, cx| {
                                                multi_workspace
                                                    .open_project_group_in_new_window(
                                                        &project_group_key,
                                                        window,
                                                        cx,
                                                    )
                                                    .detach_and_log_err(cx);
                                            })
                                            .ok();
                                    }
                                },
                            )
                        } else {
                            menu
                        };

                        let project_group_key = project_group_key.clone();
                        let multi_workspace = multi_workspace.clone();
                        menu.separator()
                            .entry("Remove Project", None, move |window, cx| {
                                multi_workspace
                                    .update(cx, |multi_workspace, cx| {
                                        multi_workspace
                                            .remove_project_group(&project_group_key, window, cx)
                                            .detach_and_log_err(cx);
                                    })
                                    .ok();
                                weak_menu.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
                            })
                    });

                let this = this.clone();
                window
                    .subscribe(&menu, cx, move |_, _: &gpui::DismissEvent, _window, cx| {
                        this.update(cx, |sidebar, cx| {
                            sidebar.project_header_menu_ix = None;
                            cx.notify();
                        })
                        .ok();
                    })
                    .detach();

                Some(menu)
            })
            .trigger(
                IconButton::new(
                    SharedString::from(format!("{id_prefix}-ellipsis-menu-{ix}")),
                    IconName::Ellipsis,
                )
                .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                .icon_size(IconSize::Small),
            )
            .anchor(gpui::Corner::TopRight)
            .offset(gpui::Point {
                x: px(0.),
                y: px(1.),
            })
    }

    fn render_sticky_header(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let scroll_top = self.list_state.logical_scroll_top();

        let &header_idx = self
            .contents
            .project_header_indices
            .iter()
            .rev()
            .find(|&&idx| idx <= scroll_top.item_ix)?;

        let needs_sticky = header_idx < scroll_top.item_ix
            || (header_idx == scroll_top.item_ix && scroll_top.offset_in_item > px(0.));

        if !needs_sticky {
            return None;
        }

        let ListEntry::ProjectHeader {
            key,
            label,
            highlight_positions,
            has_running_threads,
            waiting_thread_count,
            is_active,
            has_threads,
        } = self.contents.entries.get(header_idx)?
        else {
            return None;
        };

        let is_focused = self.focus_handle.is_focused(window);
        let is_selected = is_focused && self.selection == Some(header_idx);

        let header_element = self.render_project_header(
            header_idx,
            true,
            key,
            &label,
            &highlight_positions,
            *has_running_threads,
            *waiting_thread_count,
            *is_active,
            *has_threads,
            is_selected,
            cx,
        );

        let top_offset = self
            .contents
            .project_header_indices
            .iter()
            .find(|&&idx| idx > header_idx)
            .and_then(|&next_idx| {
                let bounds = self.list_state.bounds_for_item(next_idx)?;
                let viewport = self.list_state.viewport_bounds();
                let y_in_viewport = bounds.origin.y - viewport.origin.y;
                let header_height = bounds.size.height;
                (y_in_viewport < header_height).then_some(y_in_viewport - header_height)
            })
            .unwrap_or(px(0.));

        let color = cx.theme().colors();
        let background = color
            .title_bar_background
            .blend(color.panel_background.opacity(0.2));

        let element = v_flex()
            .absolute()
            .top(top_offset)
            .left_0()
            .w_full()
            .bg(background)
            .border_b_1()
            .border_color(color.border.opacity(0.5))
            .child(header_element)
            .shadow_xs()
            .into_any_element();

        Some(element)
    }

    fn toggle_collapse(
        &mut self,
        project_group_key: &ProjectGroupKey,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let is_collapsed = self.is_group_collapsed(project_group_key, cx);
        self.set_group_expanded(project_group_key, is_collapsed, cx);
        self.update_entries(cx);
    }

    fn dispatch_context(&self, window: &Window, cx: &Context<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("ThreadsSidebar");
        dispatch_context.add("menu");

        let is_archived_search_focused = matches!(&self.view, SidebarView::Archive(archive) if archive.read(cx).is_filter_editor_focused(window, cx));

        let identifier = if self.filter_editor.focus_handle(cx).is_focused(window)
            || is_archived_search_focused
        {
            "searching"
        } else {
            "not_searching"
        };

        dispatch_context.add(identifier);
        dispatch_context
    }

    fn focus_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.focus_handle.is_focused(window) {
            return;
        }

        if let SidebarView::Archive(archive) = &self.view {
            let has_selection = archive.read(cx).has_selection();
            if !has_selection {
                archive.update(cx, |view, cx| view.focus_filter_editor(window, cx));
            }
        } else if self.selection.is_none() {
            self.filter_editor.focus_handle(cx).focus(window, cx);
        }
    }

    fn cancel(&mut self, _: &Cancel, window: &mut Window, cx: &mut Context<Self>) {
        if self.reset_filter_editor_text(window, cx) {
            self.update_entries(cx);
        } else {
            self.selection = None;
            self.filter_editor.focus_handle(cx).focus(window, cx);
            cx.notify();
        }
    }

    fn focus_sidebar_filter(
        &mut self,
        _: &FocusSidebarFilter,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selection = None;
        if let SidebarView::Archive(archive) = &self.view {
            archive.update(cx, |view, cx| {
                view.clear_selection();
                view.focus_filter_editor(window, cx);
            });
        } else {
            self.filter_editor.focus_handle(cx).focus(window, cx);
        }

        // When vim mode is active, the editor defaults to normal mode which
        // blocks text input. Switch to insert mode so the user can type
        // immediately.
        if vim_mode_setting::VimModeSetting::get_global(cx).0 {
            if let Ok(action) = cx.build_action("vim::SwitchToInsertMode", None) {
                window.dispatch_action(action, cx);
            }
        }

        cx.notify();
    }

    fn reset_filter_editor_text(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        self.filter_editor.update(cx, |editor, cx| {
            if editor.buffer().read(cx).len(cx).0 > 0 {
                editor.set_text("", window, cx);
                true
            } else {
                false
            }
        })
    }

    fn has_filter_query(&self, cx: &App) -> bool {
        !self.filter_editor.read(cx).text(cx).is_empty()
    }

    fn editor_move_down(&mut self, _: &MoveDown, window: &mut Window, cx: &mut Context<Self>) {
        self.select_next(&SelectNext, window, cx);
        if self.selection.is_some() {
            self.focus_handle.focus(window, cx);
        }
    }

    fn editor_move_up(&mut self, _: &MoveUp, window: &mut Window, cx: &mut Context<Self>) {
        self.select_previous(&SelectPrevious, window, cx);
        if self.selection.is_some() {
            self.focus_handle.focus(window, cx);
        }
    }

    fn editor_confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.selection.is_none() {
            self.select_next(&SelectNext, window, cx);
        }
        if self.selection.is_some() {
            self.focus_handle.focus(window, cx);
        }
    }

    fn select_next(&mut self, _: &SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        let next = match self.selection {
            Some(ix) if ix + 1 < self.contents.entries.len() => ix + 1,
            Some(_) if !self.contents.entries.is_empty() => 0,
            None if !self.contents.entries.is_empty() => 0,
            _ => return,
        };
        self.selection = Some(next);
        self.list_state.scroll_to_reveal_item(next);
        cx.notify();
    }

    fn select_previous(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        match self.selection {
            Some(0) => {
                self.selection = None;
                self.filter_editor.focus_handle(cx).focus(window, cx);
                cx.notify();
            }
            Some(ix) => {
                self.selection = Some(ix - 1);
                self.list_state.scroll_to_reveal_item(ix - 1);
                cx.notify();
            }
            None if !self.contents.entries.is_empty() => {
                let last = self.contents.entries.len() - 1;
                self.selection = Some(last);
                self.list_state.scroll_to_reveal_item(last);
                cx.notify();
            }
            None => {}
        }
    }

    fn select_first(&mut self, _: &SelectFirst, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.contents.entries.is_empty() {
            self.selection = Some(0);
            self.list_state.scroll_to_reveal_item(0);
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(last) = self.contents.entries.len().checked_sub(1) {
            self.selection = Some(last);
            self.list_state.scroll_to_reveal_item(last);
            cx.notify();
        }
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let Some(ix) = self.selection else { return };
        let Some(entry) = self.contents.entries.get(ix) else {
            return;
        };

        match entry {
            ListEntry::ProjectHeader { key, .. } => {
                let key = key.clone();
                self.toggle_collapse(&key, window, cx);
            }
            ListEntry::Thread(thread) => {
                let metadata = thread.metadata.clone();
                match &thread.workspace {
                    ThreadEntryWorkspace::Open(workspace) => {
                        let workspace = workspace.clone();
                        self.activate_thread(metadata, &workspace, false, window, cx);
                    }
                    ThreadEntryWorkspace::Closed {
                        folder_paths,
                        project_group_key,
                    } => {
                        let folder_paths = folder_paths.clone();
                        let project_group_key = project_group_key.clone();
                        self.open_workspace_and_activate_thread(
                            metadata,
                            folder_paths,
                            &project_group_key,
                            window,
                            cx,
                        );
                    }
                }
            }
            ListEntry::ViewMore {
                key,
                is_fully_expanded,
                ..
            } => {
                let key = key.clone();
                if *is_fully_expanded {
                    self.reset_thread_group_expansion(&key, cx);
                } else {
                    self.expand_thread_group(&key, cx);
                }
            }
        }
    }

    fn find_workspace_across_windows(
        &self,
        cx: &App,
        predicate: impl Fn(&Entity<Workspace>, &App) -> bool,
    ) -> Option<(WindowHandle<MultiWorkspace>, Entity<Workspace>)> {
        cx.windows()
            .into_iter()
            .filter_map(|window| window.downcast::<MultiWorkspace>())
            .find_map(|window| {
                let workspace = window.read(cx).ok().and_then(|multi_workspace| {
                    multi_workspace
                        .workspaces()
                        .find(|workspace| predicate(workspace, cx))
                        .cloned()
                })?;
                Some((window, workspace))
            })
    }

    fn find_workspace_in_current_window(
        &self,
        cx: &App,
        predicate: impl Fn(&Entity<Workspace>, &App) -> bool,
    ) -> Option<Entity<Workspace>> {
        self.multi_workspace.upgrade().and_then(|multi_workspace| {
            multi_workspace
                .read(cx)
                .workspaces()
                .find(|workspace| predicate(workspace, cx))
                .cloned()
        })
    }

    fn load_agent_thread_in_workspace(
        workspace: &Entity<Workspace>,
        metadata: &ThreadMetadata,
        focus: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        let load_thread = |agent_panel: Entity<AgentPanel>,
                           metadata: &ThreadMetadata,
                           focus: bool,
                           window: &mut Window,
                           cx: &mut App| {
            let Some(session_id) = metadata.session_id.clone() else {
                return;
            };
            agent_panel.update(cx, |panel, cx| {
                panel.load_agent_thread(
                    Agent::from(metadata.agent_id.clone()),
                    session_id,
                    Some(metadata.folder_paths().clone()),
                    metadata.title.clone(),
                    focus,
                    window,
                    cx,
                );
            });
        };

        let mut existing_panel = None;
        workspace.update(cx, |workspace, cx| {
            if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                panel.update(cx, |panel, _cx| panel.begin_loading_thread());
                existing_panel = Some(panel);
            }
        });

        if let Some(agent_panel) = existing_panel {
            load_thread(agent_panel, metadata, focus, window, cx);
            workspace.update(cx, |workspace, cx| {
                workspace.reveal_panel::<AgentPanel>(window, cx);
            });
            return;
        }

        let workspace = workspace.downgrade();
        let metadata = metadata.clone();
        let mut async_window_cx = window.to_async(cx);
        cx.spawn(async move |_cx| {
            let panel = AgentPanel::load(workspace.clone(), async_window_cx.clone()).await?;

            workspace.update_in(&mut async_window_cx, |workspace, window, cx| {
                let panel = workspace.panel::<AgentPanel>(cx).unwrap_or_else(|| {
                    workspace.add_panel(panel.clone(), window, cx);
                    panel.clone()
                });
                panel.update(cx, |panel, _cx| panel.begin_loading_thread());
                load_thread(panel, &metadata, focus, window, cx);
                workspace.reveal_panel::<AgentPanel>(window, cx);
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn clear_empty_group_drafts(&mut self, workspace: &Entity<Workspace>, cx: &mut Context<Self>) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let group_key = workspace.read(cx).project_group_key(cx);
        let group_workspaces: Vec<_> = multi_workspace
            .read(cx)
            .workspaces()
            .filter(|candidate| candidate.read(cx).project_group_key(cx) == group_key)
            .cloned()
            .collect();

        for group_workspace in group_workspaces {
            group_workspace.update(cx, |workspace, cx| {
                let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
                    return;
                };

                panel.update(cx, |panel, cx| {
                    let empty_draft_ids: Vec<ThreadId> = panel
                        .draft_thread_ids(cx)
                        .into_iter()
                        .filter(|id| panel.editor_text(*id, cx).is_none())
                        .collect();

                    for id in empty_draft_ids {
                        panel.remove_thread(id, cx);
                    }
                });
            });
        }
    }

    fn activate_thread_locally(
        &mut self,
        metadata: &ThreadMetadata,
        workspace: &Entity<Workspace>,
        retain: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        // Set active_entry eagerly so the sidebar highlight updates
        // immediately, rather than waiting for a deferred AgentPanel
        // event which can race with ActiveWorkspaceChanged clearing it.
        self.active_entry = Some(ActiveEntry {
            thread_id: metadata.thread_id,
            session_id: metadata.session_id.clone(),
            workspace: workspace.clone(),
        });
        self.record_thread_access(&metadata.session_id);

        if metadata.session_id.is_some() {
            self.pending_thread_activation = Some(metadata.thread_id);
        }

        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.activate(workspace.clone(), window, cx);
            if retain {
                multi_workspace.retain_active_workspace(cx);
            }
        });

        // Drafts (and other retained threads without a session_id) are
        // already in memory — activate them directly instead of loading.
        let thread_id = metadata.thread_id;
        if metadata.session_id.is_none() {
            workspace.update(cx, |ws, cx| {
                if let Some(panel) = ws.panel::<AgentPanel>(cx) {
                    panel.update(cx, |panel, cx| {
                        panel.activate_retained_thread(thread_id, true, window, cx);
                    });
                }
                ws.focus_panel::<AgentPanel>(window, cx);
            });
            self.pending_thread_activation = None;
            self.observe_draft_editors(cx);
        } else {
            Self::load_agent_thread_in_workspace(workspace, metadata, true, window, cx);
            self.clear_empty_group_drafts(workspace, cx);
        }

        self.update_entries(cx);
    }

    fn activate_thread_in_other_window(
        &self,
        metadata: ThreadMetadata,
        workspace: Entity<Workspace>,
        target_window: WindowHandle<MultiWorkspace>,
        cx: &mut Context<Self>,
    ) {
        let target_session_id = metadata.session_id.clone();
        let metadata_thread_id = metadata.thread_id;
        let workspace_for_entry = workspace.clone();

        let activated = target_window
            .update(cx, |multi_workspace, window, cx| {
                window.activate_window();
                multi_workspace.activate(workspace.clone(), window, cx);
                Self::load_agent_thread_in_workspace(&workspace, &metadata, true, window, cx);
            })
            .log_err()
            .is_some();

        if activated {
            if let Some(target_sidebar) = target_window
                .read(cx)
                .ok()
                .and_then(|multi_workspace| {
                    multi_workspace.sidebar().map(|sidebar| sidebar.to_any())
                })
                .and_then(|sidebar| sidebar.downcast::<Self>().ok())
            {
                target_sidebar.update(cx, |sidebar, cx| {
                    sidebar.pending_thread_activation = Some(metadata_thread_id);
                    sidebar.active_entry = Some(ActiveEntry {
                        thread_id: metadata_thread_id,
                        session_id: target_session_id.clone(),
                        workspace: workspace_for_entry.clone(),
                    });
                    sidebar.record_thread_access(&target_session_id);
                    sidebar.clear_empty_group_drafts(&workspace_for_entry, cx);
                    sidebar.update_entries(cx);
                });
            }
        }
    }

    fn activate_thread(
        &mut self,
        metadata: ThreadMetadata,
        workspace: &Entity<Workspace>,
        retain: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self
            .find_workspace_in_current_window(cx, |candidate, _| candidate == workspace)
            .is_some()
        {
            self.activate_thread_locally(&metadata, &workspace, retain, window, cx);
            return;
        }

        let Some((target_window, workspace)) =
            self.find_workspace_across_windows(cx, |candidate, _| candidate == workspace)
        else {
            return;
        };

        self.activate_thread_in_other_window(metadata, workspace, target_window, cx);
    }

    fn open_workspace_and_activate_thread(
        &mut self,
        metadata: ThreadMetadata,
        folder_paths: PathList,
        project_group_key: &ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let pending_thread_id = metadata.thread_id;
        // Mark the pending thread activation so rebuild_contents
        // preserves the Thread active_entry during loading and
        // reconciliation cannot synthesize an empty fallback draft.
        self.pending_thread_activation = Some(pending_thread_id);

        let host = project_group_key.host();
        let provisional_key = Some(project_group_key.clone());
        let active_workspace = multi_workspace.read(cx).workspace().clone();
        let modal_workspace = active_workspace.clone();

        let open_task = multi_workspace.update(cx, |this, cx| {
            this.find_or_create_workspace(
                folder_paths,
                host,
                provisional_key,
                |options, window, cx| connect_remote(active_workspace, options, window, cx),
                window,
                cx,
            )
        });

        cx.spawn_in(window, async move |this, cx| {
            let result = open_task.await;
            // Dismiss the modal as soon as the open attempt completes so
            // failures or cancellations do not leave a stale connection modal behind.
            remote_connection::dismiss_connection_modal(&modal_workspace, cx);

            if result.is_err() {
                this.update(cx, |this, _cx| {
                    if this.pending_thread_activation == Some(pending_thread_id) {
                        this.pending_thread_activation = None;
                    }
                })
                .ok();
            }

            let workspace = result?;
            this.update_in(cx, |this, window, cx| {
                this.activate_thread(metadata, &workspace, false, window, cx);
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn find_current_workspace_for_path_list(
        &self,
        path_list: &PathList,
        cx: &App,
    ) -> Option<Entity<Workspace>> {
        self.find_workspace_in_current_window(cx, |workspace, cx| {
            workspace_path_list(workspace, cx).paths() == path_list.paths()
        })
    }

    fn find_open_workspace_for_path_list(
        &self,
        path_list: &PathList,
        cx: &App,
    ) -> Option<(WindowHandle<MultiWorkspace>, Entity<Workspace>)> {
        self.find_workspace_across_windows(cx, |workspace, cx| {
            workspace_path_list(workspace, cx).paths() == path_list.paths()
        })
    }

    fn activate_archived_thread(
        &mut self,
        metadata: ThreadMetadata,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let thread_id = metadata.thread_id;
        let weak_archive_view = match &self.view {
            SidebarView::Archive(view) => Some(view.downgrade()),
            _ => None,
        };

        if metadata.folder_paths().paths().is_empty() {
            ThreadMetadataStore::global(cx).update(cx, |store, cx| store.unarchive(thread_id, cx));

            let active_workspace = self
                .multi_workspace
                .upgrade()
                .map(|w| w.read(cx).workspace().clone());

            if let Some(workspace) = active_workspace {
                self.activate_thread_locally(&metadata, &workspace, false, window, cx);
            } else {
                let path_list = metadata.folder_paths().clone();
                if let Some((target_window, workspace)) =
                    self.find_open_workspace_for_path_list(&path_list, cx)
                {
                    self.activate_thread_in_other_window(metadata, workspace, target_window, cx);
                } else {
                    let key = ProjectGroupKey::new(None, path_list.clone());
                    self.open_workspace_and_activate_thread(metadata, path_list, &key, window, cx);
                }
            }
            self.show_thread_list(window, cx);
            return;
        }

        let store = ThreadMetadataStore::global(cx);
        let task = store
            .read(cx)
            .get_archived_worktrees_for_thread(thread_id, cx);
        let path_list = metadata.folder_paths().clone();

        let restore_task = cx.spawn_in(window, async move |this, cx| {
            let result: anyhow::Result<()> = async {
                let archived_worktrees = task.await?;

                if archived_worktrees.is_empty() {
                    this.update_in(cx, |this, window, cx| {
                        this.restoring_tasks.remove(&thread_id);
                        ThreadMetadataStore::global(cx)
                            .update(cx, |store, cx| store.unarchive(thread_id, cx));

                        if let Some(workspace) =
                            this.find_current_workspace_for_path_list(&path_list, cx)
                        {
                            this.activate_thread_locally(&metadata, &workspace, false, window, cx);
                        } else if let Some((target_window, workspace)) =
                            this.find_open_workspace_for_path_list(&path_list, cx)
                        {
                            this.activate_thread_in_other_window(
                                metadata,
                                workspace,
                                target_window,
                                cx,
                            );
                        } else {
                            let key = ProjectGroupKey::new(None, path_list.clone());
                            this.open_workspace_and_activate_thread(
                                metadata, path_list, &key, window, cx,
                            );
                        }
                        this.show_thread_list(window, cx);
                    })?;
                    return anyhow::Ok(());
                }

                let mut path_replacements: Vec<(PathBuf, PathBuf)> = Vec::new();
                for row in &archived_worktrees {
                    match thread_worktree_archive::restore_worktree_via_git(row, &mut *cx).await {
                        Ok(restored_path) => {
                            thread_worktree_archive::cleanup_archived_worktree_record(
                                row, &mut *cx,
                            )
                            .await;
                            path_replacements.push((row.worktree_path.clone(), restored_path));
                        }
                        Err(error) => {
                            log::error!("Failed to restore worktree: {error:#}");
                            this.update_in(cx, |this, _window, cx| {
                                this.restoring_tasks.remove(&thread_id);
                                if let Some(weak_archive_view) = &weak_archive_view {
                                    weak_archive_view
                                        .update(cx, |view, cx| {
                                            view.clear_restoring(&thread_id, cx);
                                        })
                                        .ok();
                                }

                                if let Some(multi_workspace) = this.multi_workspace.upgrade() {
                                    let workspace = multi_workspace.read(cx).workspace().clone();
                                    workspace.update(cx, |workspace, cx| {
                                        struct RestoreWorktreeErrorToast;
                                        workspace.show_toast(
                                            Toast::new(
                                                NotificationId::unique::<RestoreWorktreeErrorToast>(
                                                ),
                                                format!("Failed to restore worktree: {error:#}"),
                                            )
                                            .autohide(),
                                            cx,
                                        );
                                    });
                                }
                            })
                            .ok();
                            return anyhow::Ok(());
                        }
                    }
                }

                if !path_replacements.is_empty() {
                    cx.update(|_window, cx| {
                        store.update(cx, |store, cx| {
                            store.update_restored_worktree_paths(thread_id, &path_replacements, cx);
                        });
                    })?;

                    let updated_metadata =
                        cx.update(|_window, cx| store.read(cx).entry(thread_id).cloned())?;

                    if let Some(updated_metadata) = updated_metadata {
                        let new_paths = updated_metadata.folder_paths().clone();

                        cx.update(|_window, cx| {
                            store.update(cx, |store, cx| {
                                store.unarchive(updated_metadata.thread_id, cx);
                            });
                        })?;

                        this.update_in(cx, |this, window, cx| {
                            this.restoring_tasks.remove(&thread_id);
                            let key = ProjectGroupKey::new(None, new_paths.clone());
                            this.open_workspace_and_activate_thread(
                                updated_metadata,
                                new_paths,
                                &key,
                                window,
                                cx,
                            );
                            this.show_thread_list(window, cx);
                        })?;
                    }
                }

                anyhow::Ok(())
            }
            .await;
            if let Err(error) = result {
                log::error!("{error:#}");
            }
        });
        self.restoring_tasks.insert(thread_id, restore_task);
    }

    fn expand_selected_entry(
        &mut self,
        _: &SelectChild,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else { return };

        match self.contents.entries.get(ix) {
            Some(ListEntry::ProjectHeader { key, .. }) => {
                let key = key.clone();
                if self.is_group_collapsed(&key, cx) {
                    self.set_group_expanded(&key, true, cx);
                    self.update_entries(cx);
                } else if ix + 1 < self.contents.entries.len() {
                    self.selection = Some(ix + 1);
                    self.list_state.scroll_to_reveal_item(ix + 1);
                    cx.notify();
                }
            }
            _ => {}
        }
    }

    fn collapse_selected_entry(
        &mut self,
        _: &SelectParent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else { return };

        match self.contents.entries.get(ix) {
            Some(ListEntry::ProjectHeader { key, .. }) => {
                let key = key.clone();
                if !self.is_group_collapsed(&key, cx) {
                    self.set_group_expanded(&key, false, cx);
                    self.update_entries(cx);
                }
            }
            Some(ListEntry::Thread(_) | ListEntry::ViewMore { .. }) => {
                for i in (0..ix).rev() {
                    if let Some(ListEntry::ProjectHeader { key, .. }) = self.contents.entries.get(i)
                    {
                        let key = key.clone();
                        self.selection = Some(i);
                        self.set_group_expanded(&key, false, cx);
                        self.update_entries(cx);
                        break;
                    }
                }
            }
            None => {}
        }
    }

    fn toggle_selected_fold(
        &mut self,
        _: &editor::actions::ToggleFold,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else { return };

        // Find the group header for the current selection.
        let header_ix = match self.contents.entries.get(ix) {
            Some(ListEntry::ProjectHeader { .. }) => Some(ix),
            Some(ListEntry::Thread(_) | ListEntry::ViewMore { .. }) => (0..ix).rev().find(|&i| {
                matches!(
                    self.contents.entries.get(i),
                    Some(ListEntry::ProjectHeader { .. })
                )
            }),
            None => None,
        };

        if let Some(header_ix) = header_ix {
            if let Some(ListEntry::ProjectHeader { key, .. }) = self.contents.entries.get(header_ix)
            {
                let key = key.clone();
                if self.is_group_collapsed(&key, cx) {
                    self.set_group_expanded(&key, true, cx);
                } else {
                    self.selection = Some(header_ix);
                    self.set_group_expanded(&key, false, cx);
                }
                self.update_entries(cx);
            }
        }
    }

    fn fold_all(
        &mut self,
        _: &editor::actions::FoldAll,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(mw) = self.multi_workspace.upgrade() {
            mw.update(cx, |mw, _cx| {
                mw.set_all_groups_expanded(false);
            });
        }
        self.update_entries(cx);
    }

    fn unfold_all(
        &mut self,
        _: &editor::actions::UnfoldAll,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(mw) = self.multi_workspace.upgrade() {
            mw.update(cx, |mw, _cx| {
                mw.set_all_groups_expanded(true);
            });
        }
        self.update_entries(cx);
    }

    fn stop_thread(&mut self, thread_id: &agent_ui::ThreadId, cx: &mut Context<Self>) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let workspaces: Vec<_> = multi_workspace.read(cx).workspaces().cloned().collect();
        for workspace in workspaces {
            if let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                let cancelled =
                    agent_panel.update(cx, |panel, cx| panel.cancel_thread(thread_id, cx));
                if cancelled {
                    return;
                }
            }
        }
    }

    fn archive_thread(
        &mut self,
        session_id: &acp::SessionId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let store = ThreadMetadataStore::global(cx);
        let metadata = store.read(cx).entry_by_session(session_id).cloned();
        let active_workspace = metadata.as_ref().and_then(|metadata| {
            self.active_entry.as_ref().and_then(|entry| {
                if entry.is_active_thread(&metadata.thread_id) {
                    Some(entry.workspace.clone())
                } else {
                    None
                }
            })
        });
        let thread_id = metadata.as_ref().map(|metadata| metadata.thread_id);
        let thread_folder_paths = metadata
            .as_ref()
            .map(|metadata| metadata.folder_paths().clone())
            .or_else(|| {
                active_workspace
                    .as_ref()
                    .map(|workspace| PathList::new(&workspace.read(cx).root_paths(cx)))
            });

        // Compute which linked worktree roots should be archived from disk if
        // this thread is archived. This must happen before we remove any
        // workspace from the MultiWorkspace, because `build_root_plan` needs
        // the currently open workspaces in order to find the affected projects
        // and repository handles for each linked worktree.
        let roots_to_archive = metadata
            .as_ref()
            .map(|metadata| {
                let mut workspaces = self
                    .multi_workspace
                    .upgrade()
                    .map(|multi_workspace| {
                        multi_workspace
                            .read(cx)
                            .workspaces()
                            .cloned()
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                for workspace in thread_worktree_archive::all_open_workspaces(cx) {
                    if !workspaces.contains(&workspace) {
                        workspaces.push(workspace);
                    }
                }
                metadata
                    .folder_paths()
                    .ordered_paths()
                    .filter_map(|path| {
                        thread_worktree_archive::build_root_plan(path, &workspaces, cx)
                    })
                    .filter(|plan| {
                        thread_id.map_or(true, |tid| {
                            !thread_worktree_archive::path_is_referenced_by_other_unarchived_threads(
                                tid,
                                &plan.root_path,
                                cx,
                            )
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        // Find the neighbor thread in the sidebar (by display position).
        // Look below first, then above, for the nearest thread that isn't
        // the one being archived. We capture both the neighbor's metadata
        // (for activation) and its workspace paths (for the workspace
        // removal fallback).
        let current_pos = self.contents.entries.iter().position(|entry| match entry {
            ListEntry::Thread(thread) => thread_id.map_or_else(
                || thread.metadata.session_id.as_ref() == Some(session_id),
                |tid| thread.metadata.thread_id == tid,
            ),
            _ => false,
        });
        let neighbor = current_pos.and_then(|pos| {
            self.contents.entries[pos + 1..]
                .iter()
                .chain(self.contents.entries[..pos].iter().rev())
                .find_map(|entry| match entry {
                    ListEntry::Thread(t)
                        if !t.is_draft && t.metadata.session_id.as_ref() != Some(session_id) =>
                    {
                        let workspace_paths = match &t.workspace {
                            ThreadEntryWorkspace::Open(ws) => {
                                PathList::new(&ws.read(cx).root_paths(cx))
                            }
                            ThreadEntryWorkspace::Closed { folder_paths, .. } => {
                                folder_paths.clone()
                            }
                        };
                        Some((t.metadata.clone(), workspace_paths))
                    }
                    _ => None,
                })
        });

        // Check if archiving this thread would leave its worktree workspace
        // with no threads, requiring workspace removal.
        let workspace_to_remove = thread_folder_paths.as_ref().and_then(|folder_paths| {
            if folder_paths.is_empty() {
                return None;
            }

            let remaining = ThreadMetadataStore::global(cx)
                .read(cx)
                .entries_for_path(folder_paths)
                .filter(|t| t.session_id.as_ref() != Some(session_id))
                .count();

            if remaining > 0 {
                return None;
            }

            let multi_workspace = self.multi_workspace.upgrade()?;
            let workspace = multi_workspace
                .read(cx)
                .workspace_for_paths(folder_paths, None, cx)?;

            let group_key = workspace.read(cx).project_group_key(cx);
            let is_linked_worktree = group_key.path_list() != folder_paths;

            is_linked_worktree.then_some(workspace)
        });

        // Also find workspaces for root plans that aren't covered by
        // workspace_to_remove. For workspaces that exclusively contain
        // worktrees being archived, remove the whole workspace. For
        // "mixed" workspaces (containing both archived and non-archived
        // worktrees), close only the editor items referencing the
        // archived worktrees so their Entity<Worktree> handles are
        // dropped without destroying the user's workspace layout.
        let mut workspaces_to_remove: Vec<Entity<Workspace>> =
            workspace_to_remove.into_iter().collect();
        let mut close_item_tasks: Vec<Task<anyhow::Result<()>>> = Vec::new();

        let archive_paths: HashSet<&Path> = roots_to_archive
            .iter()
            .map(|root| root.root_path.as_path())
            .collect();

        // Classify workspaces into "exclusive" (all worktrees archived)
        // and "mixed" (some worktrees archived, some not).
        let mut mixed_workspaces: Vec<(Entity<Workspace>, Vec<WorktreeId>)> = Vec::new();

        if let Some(multi_workspace) = self.multi_workspace.upgrade() {
            let all_workspaces: Vec<_> = multi_workspace.read(cx).workspaces().cloned().collect();

            for workspace in all_workspaces {
                if workspaces_to_remove.contains(&workspace) {
                    continue;
                }

                let project = workspace.read(cx).project().read(cx);
                let visible_worktrees: Vec<_> = project
                    .visible_worktrees(cx)
                    .map(|wt| (wt.read(cx).id(), wt.read(cx).abs_path()))
                    .collect();

                let archived_worktree_ids: Vec<WorktreeId> = visible_worktrees
                    .iter()
                    .filter(|(_, path)| archive_paths.contains(path.as_ref()))
                    .map(|(id, _)| *id)
                    .collect();

                if archived_worktree_ids.is_empty() {
                    continue;
                }

                if visible_worktrees.len() == archived_worktree_ids.len() {
                    workspaces_to_remove.push(workspace);
                } else {
                    mixed_workspaces.push((workspace, archived_worktree_ids));
                }
            }
        }

        // For mixed workspaces, close only items belonging to the
        // worktrees being archived.
        for (workspace, archived_worktree_ids) in &mixed_workspaces {
            let panes: Vec<_> = workspace.read(cx).panes().to_vec();
            for pane in panes {
                let items_to_close: Vec<EntityId> = pane
                    .read(cx)
                    .items()
                    .filter(|item| {
                        item.project_path(cx)
                            .is_some_and(|pp| archived_worktree_ids.contains(&pp.worktree_id))
                    })
                    .map(|item| item.item_id())
                    .collect();

                if !items_to_close.is_empty() {
                    let task = pane.update(cx, |pane, cx| {
                        pane.close_items(window, cx, SaveIntent::Close, &|item_id| {
                            items_to_close.contains(&item_id)
                        })
                    });
                    close_item_tasks.push(task);
                }
            }
        }

        if !workspaces_to_remove.is_empty() {
            let multi_workspace = self.multi_workspace.upgrade().unwrap();
            let session_id = session_id.clone();

            let fallback_paths = neighbor
                .as_ref()
                .map(|(_, paths)| paths.clone())
                .unwrap_or_else(|| {
                    workspaces_to_remove
                        .first()
                        .map(|ws| ws.read(cx).project_group_key(cx).path_list().clone())
                        .unwrap_or_default()
                });

            let excluded = workspaces_to_remove.clone();
            let remove_task = multi_workspace.update(cx, |mw, cx| {
                mw.remove(
                    workspaces_to_remove,
                    move |this, window, cx| {
                        this.find_or_create_local_workspace(fallback_paths, &excluded, window, cx)
                    },
                    window,
                    cx,
                )
            });

            let neighbor_metadata = neighbor.map(|(metadata, _)| metadata);
            let thread_folder_paths = thread_folder_paths.clone();
            cx.spawn_in(window, async move |this, cx| {
                if !remove_task.await? {
                    return anyhow::Ok(());
                }

                for task in close_item_tasks {
                    let result: anyhow::Result<()> = task.await;
                    result.log_err();
                }

                this.update_in(cx, |this, window, cx| {
                    let in_flight = thread_id.and_then(|tid| {
                        this.start_archive_worktree_task(tid, roots_to_archive, cx)
                    });
                    this.archive_and_activate(
                        &session_id,
                        thread_id,
                        neighbor_metadata.as_ref(),
                        thread_folder_paths.as_ref(),
                        in_flight,
                        window,
                        cx,
                    );
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        } else if !close_item_tasks.is_empty() {
            let session_id = session_id.clone();
            let neighbor_metadata = neighbor.map(|(metadata, _)| metadata);
            let thread_folder_paths = thread_folder_paths.clone();
            cx.spawn_in(window, async move |this, cx| {
                for task in close_item_tasks {
                    let result: anyhow::Result<()> = task.await;
                    result.log_err();
                }

                this.update_in(cx, |this, window, cx| {
                    let in_flight = thread_id.and_then(|tid| {
                        this.start_archive_worktree_task(tid, roots_to_archive, cx)
                    });
                    this.archive_and_activate(
                        &session_id,
                        thread_id,
                        neighbor_metadata.as_ref(),
                        thread_folder_paths.as_ref(),
                        in_flight,
                        window,
                        cx,
                    );
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        } else {
            let neighbor_metadata = neighbor.map(|(metadata, _)| metadata);
            let in_flight = thread_id
                .and_then(|tid| self.start_archive_worktree_task(tid, roots_to_archive, cx));
            self.archive_and_activate(
                session_id,
                thread_id,
                neighbor_metadata.as_ref(),
                thread_folder_paths.as_ref(),
                in_flight,
                window,
                cx,
            );
        }
    }

    /// Archive a thread and activate the nearest neighbor or a draft.
    ///
    /// IMPORTANT: when activating a neighbor or creating a fallback draft,
    /// this method also activates the target workspace in the MultiWorkspace.
    /// This is critical because `rebuild_contents` derives the active
    /// workspace from `mw.workspace()`. If the linked worktree workspace is
    /// still active after archiving its last thread, `rebuild_contents` sees
    /// the threadless linked worktree as active and emits a spurious
    /// "+ New Thread" entry with the worktree chip — keeping the worktree
    /// alive and preventing disk cleanup.
    ///
    /// When `in_flight_archive` is present, it is the background task that
    /// persists the linked worktree's git state and deletes it from disk.
    /// We attach it to the metadata store at the same time we mark the thread
    /// archived so failures can automatically unarchive the thread and user-
    /// initiated unarchive can cancel the task.
    fn archive_and_activate(
        &mut self,
        _session_id: &acp::SessionId,
        thread_id: Option<agent_ui::ThreadId>,
        neighbor: Option<&ThreadMetadata>,
        thread_folder_paths: Option<&PathList>,
        in_flight_archive: Option<(Task<()>, smol::channel::Sender<()>)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(thread_id) = thread_id {
            ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                store.archive(thread_id, in_flight_archive, cx);
            });
        }

        let is_active = self
            .active_entry
            .as_ref()
            .is_some_and(|entry| thread_id.is_some_and(|tid| entry.is_active_thread(&tid)));

        if is_active {
            self.active_entry = None;
        }

        if !is_active {
            // The user is looking at a different thread/draft. Clear the
            // archived thread from its workspace's panel so that switching
            // to that workspace later doesn't show a stale thread.
            if let Some(folder_paths) = thread_folder_paths {
                if let Some(workspace) = self
                    .multi_workspace
                    .upgrade()
                    .and_then(|mw| mw.read(cx).workspace_for_paths(folder_paths, None, cx))
                {
                    if let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                        let panel_shows_archived = panel
                            .read(cx)
                            .active_conversation_view()
                            .map(|cv| cv.read(cx).parent_id())
                            .is_some_and(|live_thread_id| {
                                thread_id.is_some_and(|id| id == live_thread_id)
                            });
                        if panel_shows_archived {
                            panel.update(cx, |panel, cx| {
                                // Replace the archived thread with a
                                // tracked draft so the panel isn't left
                                // in Uninitialized state.
                                let id = panel.create_thread(window, cx);
                                panel.activate_retained_thread(id, false, window, cx);
                            });
                        }
                    }
                }
            }
            return;
        }

        // Try to activate the neighbor thread. If its workspace is open,
        // tell the panel to load it and activate that workspace.
        // `rebuild_contents` will reconcile `active_entry` once the thread
        // finishes loading.

        if let Some(metadata) = neighbor {
            if let Some(workspace) = self.multi_workspace.upgrade().and_then(|mw| {
                mw.read(cx)
                    .workspace_for_paths(metadata.folder_paths(), None, cx)
            }) {
                self.active_entry = Some(ActiveEntry {
                    thread_id: metadata.thread_id,
                    session_id: metadata.session_id.clone(),
                    workspace: workspace.clone(),
                });
                self.activate_workspace(&workspace, window, cx);
                Self::load_agent_thread_in_workspace(&workspace, metadata, true, window, cx);
                return;
            }
        }

        // No neighbor or its workspace isn't open — fall back to a new
        // draft. Use the group workspace (main project) rather than the
        // active entry workspace, which may be a linked worktree that is
        // about to be cleaned up or already removed.
        let fallback_workspace = thread_folder_paths
            .and_then(|folder_paths| {
                let mw = self.multi_workspace.upgrade()?;
                let mw = mw.read(cx);
                let thread_workspace = mw.workspace_for_paths(folder_paths, None, cx)?;
                let group_key = thread_workspace.read(cx).project_group_key(cx);
                mw.workspace_for_paths(group_key.path_list(), None, cx)
            })
            .or_else(|| {
                self.multi_workspace
                    .upgrade()
                    .map(|mw| mw.read(cx).workspace().clone())
            });

        if let Some(workspace) = fallback_workspace {
            self.activate_workspace(&workspace, window, cx);
            self.create_new_thread(&workspace, window, cx);
        }
    }

    fn start_archive_worktree_task(
        &self,
        thread_id: ThreadId,
        roots: Vec<thread_worktree_archive::RootPlan>,
        cx: &mut Context<Self>,
    ) -> Option<(Task<()>, smol::channel::Sender<()>)> {
        if roots.is_empty() {
            return None;
        }

        let (cancel_tx, cancel_rx) = smol::channel::bounded::<()>(1);
        let task = cx.spawn(async move |_this, cx| {
            match Self::archive_worktree_roots(roots, cancel_rx, cx).await {
                Ok(ArchiveWorktreeOutcome::Success) => {
                    cx.update(|cx| {
                        ThreadMetadataStore::global(cx).update(cx, |store, _cx| {
                            store.cleanup_completed_archive(thread_id);
                        });
                    });
                }
                Ok(ArchiveWorktreeOutcome::Cancelled) => {}
                Err(error) => {
                    log::error!("Failed to archive worktree: {error:#}");
                    cx.update(|cx| {
                        ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                            store.unarchive(thread_id, cx);
                        });
                    });
                }
            }
        });

        Some((task, cancel_tx))
    }

    async fn archive_worktree_roots(
        roots: Vec<thread_worktree_archive::RootPlan>,
        cancel_rx: smol::channel::Receiver<()>,
        cx: &mut gpui::AsyncApp,
    ) -> anyhow::Result<ArchiveWorktreeOutcome> {
        let mut completed_persists: Vec<(i64, thread_worktree_archive::RootPlan)> = Vec::new();

        for root in &roots {
            if cancel_rx.is_closed() {
                for &(id, ref completed_root) in completed_persists.iter().rev() {
                    thread_worktree_archive::rollback_persist(id, completed_root, cx).await;
                }
                return Ok(ArchiveWorktreeOutcome::Cancelled);
            }

            match thread_worktree_archive::persist_worktree_state(root, cx).await {
                Ok(id) => {
                    completed_persists.push((id, root.clone()));
                }
                Err(error) => {
                    for &(id, ref completed_root) in completed_persists.iter().rev() {
                        thread_worktree_archive::rollback_persist(id, completed_root, cx).await;
                    }
                    return Err(error);
                }
            }

            if cancel_rx.is_closed() {
                for &(id, ref completed_root) in completed_persists.iter().rev() {
                    thread_worktree_archive::rollback_persist(id, completed_root, cx).await;
                }
                return Ok(ArchiveWorktreeOutcome::Cancelled);
            }

            if let Err(error) = thread_worktree_archive::remove_root(root.clone(), cx).await {
                if let Some(&(id, ref completed_root)) = completed_persists.last() {
                    if completed_root.root_path == root.root_path {
                        thread_worktree_archive::rollback_persist(id, completed_root, cx).await;
                        completed_persists.pop();
                    }
                }
                for &(id, ref completed_root) in completed_persists.iter().rev() {
                    thread_worktree_archive::rollback_persist(id, completed_root, cx).await;
                }
                return Err(error);
            }
        }

        Ok(ArchiveWorktreeOutcome::Success)
    }

    fn activate_workspace(
        &self,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(multi_workspace) = self.multi_workspace.upgrade() {
            multi_workspace.update(cx, |mw, cx| {
                mw.activate(workspace.clone(), window, cx);
            });
        }
    }

    fn remove_selected_thread(
        &mut self,
        _: &RemoveSelectedThread,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else {
            return;
        };
        match self.contents.entries.get(ix) {
            Some(ListEntry::Thread(thread)) if thread.is_draft => {
                let draft_id = thread.metadata.thread_id;
                if let ThreadEntryWorkspace::Open(workspace) = &thread.workspace {
                    let workspace = workspace.clone();
                    self.remove_draft(draft_id, &workspace, window, cx);
                }
            }
            Some(ListEntry::Thread(thread)) => {
                match thread.status {
                    AgentThreadStatus::Running | AgentThreadStatus::WaitingForConfirmation => {
                        return;
                    }
                    AgentThreadStatus::Completed | AgentThreadStatus::Error => {}
                }
                if let Some(session_id) = thread.metadata.session_id.clone() {
                    self.archive_thread(&session_id, window, cx);
                }
            }
            _ => {}
        }
    }

    fn record_thread_access(&mut self, session_id: &Option<acp::SessionId>) {
        if let Some(sid) = session_id {
            self.thread_last_accessed.insert(sid.clone(), Utc::now());
        }
    }

    fn record_thread_message_sent(&mut self, thread_id: &agent_ui::ThreadId) {
        self.thread_last_message_sent_or_queued
            .insert(*thread_id, Utc::now());
    }

    fn mru_threads_for_switcher(&self, cx: &App) -> Vec<ThreadSwitcherEntry> {
        let mut current_header_label: Option<SharedString> = None;
        let mut current_header_key: Option<ProjectGroupKey> = None;
        let mut entries: Vec<ThreadSwitcherEntry> = self
            .contents
            .entries
            .iter()
            .filter_map(|entry| match entry {
                ListEntry::ProjectHeader { label, key, .. } => {
                    current_header_label = Some(label.clone());
                    current_header_key = Some(key.clone());
                    None
                }
                ListEntry::Thread(thread) => {
                    let session_id = thread.metadata.session_id.clone()?;
                    let workspace = match &thread.workspace {
                        ThreadEntryWorkspace::Open(workspace) => Some(workspace.clone()),
                        ThreadEntryWorkspace::Closed { .. } => {
                            current_header_key.as_ref().and_then(|key| {
                                self.multi_workspace.upgrade().and_then(|mw| {
                                    mw.read(cx).workspace_for_paths(
                                        key.path_list(),
                                        key.host().as_ref(),
                                        cx,
                                    )
                                })
                            })
                        }
                    }?;
                    let notified = self.contents.is_thread_notified(&thread.metadata.thread_id);
                    let timestamp: SharedString = format_history_entry_timestamp(
                        self.thread_last_message_sent_or_queued
                            .get(&thread.metadata.thread_id)
                            .copied()
                            .or(thread.metadata.created_at)
                            .unwrap_or(thread.metadata.updated_at),
                    )
                    .into();
                    Some(ThreadSwitcherEntry {
                        session_id,
                        title: thread.metadata.display_title(),
                        icon: thread.icon,
                        icon_from_external_svg: thread.icon_from_external_svg.clone(),
                        status: thread.status,
                        metadata: thread.metadata.clone(),
                        workspace,
                        project_name: current_header_label.clone(),
                        worktrees: thread
                            .worktrees
                            .iter()
                            .map(|wt| ThreadItemWorktreeInfo {
                                name: wt.name.clone(),
                                full_path: wt.full_path.clone(),
                                highlight_positions: Vec::new(),
                                kind: wt.kind,
                            })
                            .collect(),
                        diff_stats: thread.diff_stats,
                        is_title_generating: thread.is_title_generating,
                        notified,
                        timestamp,
                    })
                }
                _ => None,
            })
            .collect();

        entries.sort_by(|a, b| {
            let a_accessed = self.thread_last_accessed.get(&a.session_id);
            let b_accessed = self.thread_last_accessed.get(&b.session_id);

            match (a_accessed, b_accessed) {
                (Some(a_time), Some(b_time)) => b_time.cmp(a_time),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => {
                    let a_sent = self
                        .thread_last_message_sent_or_queued
                        .get(&a.metadata.thread_id);
                    let b_sent = self
                        .thread_last_message_sent_or_queued
                        .get(&b.metadata.thread_id);

                    match (a_sent, b_sent) {
                        (Some(a_time), Some(b_time)) => b_time.cmp(a_time),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => {
                            let a_time = a.metadata.created_at.or(Some(a.metadata.updated_at));
                            let b_time = b.metadata.created_at.or(Some(b.metadata.updated_at));
                            b_time.cmp(&a_time)
                        }
                    }
                }
            }
        });

        entries
    }

    fn dismiss_thread_switcher(&mut self, cx: &mut Context<Self>) {
        self.thread_switcher = None;
        self._thread_switcher_subscriptions.clear();
        if let Some(mw) = self.multi_workspace.upgrade() {
            mw.update(cx, |mw, cx| {
                mw.set_sidebar_overlay(None, cx);
            });
        }
    }

    fn on_toggle_thread_switcher(
        &mut self,
        action: &ToggleThreadSwitcher,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.toggle_thread_switcher_impl(action.select_last, window, cx);
    }

    fn toggle_thread_switcher_impl(
        &mut self,
        select_last: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(thread_switcher) = &self.thread_switcher {
            thread_switcher.update(cx, |switcher, cx| {
                if select_last {
                    switcher.select_last(cx);
                } else {
                    switcher.cycle_selection(cx);
                }
            });
            return;
        }

        let entries = self.mru_threads_for_switcher(cx);
        if entries.len() < 2 {
            return;
        }

        let weak_multi_workspace = self.multi_workspace.clone();

        let original_metadata = match &self.active_entry {
            Some(ActiveEntry { thread_id, .. }) => entries
                .iter()
                .find(|e| *thread_id == e.metadata.thread_id)
                .map(|e| e.metadata.clone()),
            _ => None,
        };
        let original_workspace = self
            .multi_workspace
            .upgrade()
            .map(|mw| mw.read(cx).workspace().clone());

        let thread_switcher = cx.new(|cx| ThreadSwitcher::new(entries, select_last, window, cx));

        let mut subscriptions = Vec::new();

        subscriptions.push(cx.subscribe_in(&thread_switcher, window, {
            let thread_switcher = thread_switcher.clone();
            move |this, _emitter, event: &ThreadSwitcherEvent, window, cx| match event {
                ThreadSwitcherEvent::Preview {
                    metadata,
                    workspace,
                } => {
                    if let Some(mw) = weak_multi_workspace.upgrade() {
                        mw.update(cx, |mw, cx| {
                            mw.activate(workspace.clone(), window, cx);
                        });
                    }
                    this.active_entry = Some(ActiveEntry {
                        thread_id: metadata.thread_id,
                        session_id: metadata.session_id.clone(),
                        workspace: workspace.clone(),
                    });
                    this.update_entries(cx);
                    Self::load_agent_thread_in_workspace(workspace, metadata, false, window, cx);
                    let focus = thread_switcher.focus_handle(cx);
                    window.focus(&focus, cx);
                }
                ThreadSwitcherEvent::Confirmed {
                    metadata,
                    workspace,
                } => {
                    if let Some(mw) = weak_multi_workspace.upgrade() {
                        mw.update(cx, |mw, cx| {
                            mw.activate(workspace.clone(), window, cx);
                            mw.retain_active_workspace(cx);
                        });
                    }
                    this.record_thread_access(&metadata.session_id);
                    this.active_entry = Some(ActiveEntry {
                        thread_id: metadata.thread_id,
                        session_id: metadata.session_id.clone(),
                        workspace: workspace.clone(),
                    });
                    this.update_entries(cx);
                    Self::load_agent_thread_in_workspace(workspace, metadata, false, window, cx);
                    this.dismiss_thread_switcher(cx);
                    workspace.update(cx, |workspace, cx| {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                    });
                }
                ThreadSwitcherEvent::Dismissed => {
                    if let Some(mw) = weak_multi_workspace.upgrade() {
                        if let Some(original_ws) = &original_workspace {
                            mw.update(cx, |mw, cx| {
                                mw.activate(original_ws.clone(), window, cx);
                            });
                        }
                    }
                    if let Some(metadata) = &original_metadata {
                        if let Some(original_ws) = &original_workspace {
                            this.active_entry = Some(ActiveEntry {
                                thread_id: metadata.thread_id,
                                session_id: metadata.session_id.clone(),
                                workspace: original_ws.clone(),
                            });
                        }
                        this.update_entries(cx);
                        if let Some(original_ws) = &original_workspace {
                            Self::load_agent_thread_in_workspace(
                                original_ws,
                                metadata,
                                false,
                                window,
                                cx,
                            );
                        }
                    }
                    this.dismiss_thread_switcher(cx);
                }
            }
        }));

        subscriptions.push(cx.subscribe_in(
            &thread_switcher,
            window,
            |this, _emitter, _event: &gpui::DismissEvent, _window, cx| {
                this.dismiss_thread_switcher(cx);
            },
        ));

        let focus = thread_switcher.focus_handle(cx);
        let overlay_view = gpui::AnyView::from(thread_switcher.clone());

        // Replay the initial preview that was emitted during construction
        // before subscriptions were wired up.
        let initial_preview = thread_switcher
            .read(cx)
            .selected_entry()
            .map(|entry| (entry.metadata.clone(), entry.workspace.clone()));

        self.thread_switcher = Some(thread_switcher);
        self._thread_switcher_subscriptions = subscriptions;
        if let Some(mw) = self.multi_workspace.upgrade() {
            mw.update(cx, |mw, cx| {
                mw.set_sidebar_overlay(Some(overlay_view), cx);
            });
        }

        if let Some((metadata, workspace)) = initial_preview {
            if let Some(mw) = self.multi_workspace.upgrade() {
                mw.update(cx, |mw, cx| {
                    mw.activate(workspace.clone(), window, cx);
                });
            }
            self.active_entry = Some(ActiveEntry {
                thread_id: metadata.thread_id,
                session_id: metadata.session_id.clone(),
                workspace: workspace.clone(),
            });
            self.update_entries(cx);
            Self::load_agent_thread_in_workspace(&workspace, &metadata, false, window, cx);
        }

        window.focus(&focus, cx);
    }

    fn render_thread(
        &self,
        ix: usize,
        thread: &ThreadEntry,
        is_active: bool,
        is_focused: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let has_notification = self.contents.is_thread_notified(&thread.metadata.thread_id);

        let title: SharedString = thread.metadata.display_title();
        let metadata = thread.metadata.clone();
        let thread_workspace = thread.workspace.clone();
        let is_draft = thread.is_draft;

        let is_hovered = self.hovered_thread_index == Some(ix);
        let is_selected = is_active;
        let is_running = matches!(
            thread.status,
            AgentThreadStatus::Running | AgentThreadStatus::WaitingForConfirmation
        );

        let thread_id_for_actions = thread.metadata.thread_id;
        let session_id_for_delete = thread.metadata.session_id.clone();
        let thread_workspace_for_dismiss = thread.workspace.clone();
        let focus_handle = self.focus_handle.clone();

        let id = SharedString::from(format!("thread-entry-{}", ix));

        let color = cx.theme().colors();
        let sidebar_bg = color
            .title_bar_background
            .blend(color.panel_background.opacity(0.25));

        let timestamp = format_history_entry_timestamp(
            self.thread_last_message_sent_or_queued
                .get(&thread.metadata.thread_id)
                .copied()
                .or(thread.metadata.created_at)
                .unwrap_or(thread.metadata.updated_at),
        );

        let is_remote = thread.workspace.is_remote(cx);

        ThreadItem::new(id, title)
            .base_bg(sidebar_bg)
            .icon(thread.icon)
            .status(thread.status)
            .is_remote(is_remote)
            .when_some(thread.icon_from_external_svg.clone(), |this, svg| {
                this.custom_icon_from_external_svg(svg)
            })
            .worktrees(
                thread
                    .worktrees
                    .iter()
                    .map(|wt| ThreadItemWorktreeInfo {
                        name: wt.name.clone(),
                        full_path: wt.full_path.clone(),
                        highlight_positions: wt.highlight_positions.clone(),
                        kind: wt.kind,
                    })
                    .collect(),
            )
            .timestamp(timestamp)
            .highlight_positions(thread.highlight_positions.to_vec())
            .title_generating(thread.is_title_generating)
            .notified(has_notification)
            .when(thread.diff_stats.lines_added > 0, |this| {
                this.added(thread.diff_stats.lines_added as usize)
            })
            .when(thread.diff_stats.lines_removed > 0, |this| {
                this.removed(thread.diff_stats.lines_removed as usize)
            })
            .selected(is_selected)
            .focused(is_focused)
            .hovered(is_hovered)
            .on_hover(cx.listener(move |this, is_hovered: &bool, _window, cx| {
                if *is_hovered {
                    this.hovered_thread_index = Some(ix);
                } else if this.hovered_thread_index == Some(ix) {
                    this.hovered_thread_index = None;
                }
                cx.notify();
            }))
            .when(is_hovered && is_running, |this| {
                this.action_slot(
                    IconButton::new("stop-thread", IconName::Stop)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Error)
                        .style(ButtonStyle::Tinted(TintColor::Error))
                        .tooltip(Tooltip::text("Stop Generation"))
                        .on_click({
                            cx.listener(move |this, _, _window, cx| {
                                this.stop_thread(&thread_id_for_actions, cx);
                            })
                        }),
                )
            })
            .when(is_hovered && !is_running && is_draft, |this| {
                this.action_slot(
                    div()
                        .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| {
                            cx.stop_propagation();
                        })
                        .child(
                            IconButton::new("close-draft", IconName::Close)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted)
                                .tooltip(Tooltip::text("Remove Draft"))
                                .on_click({
                                    let thread_workspace = thread_workspace_for_dismiss.clone();
                                    cx.listener(move |this, _, window, cx| {
                                        if let ThreadEntryWorkspace::Open(workspace) =
                                            &thread_workspace
                                        {
                                            this.remove_draft(
                                                thread_id_for_actions,
                                                workspace,
                                                window,
                                                cx,
                                            );
                                        }
                                    })
                                }),
                        ),
                )
            })
            .when(is_hovered && !is_running && !is_draft, |this| {
                this.action_slot(
                    IconButton::new("archive-thread", IconName::Archive)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .tooltip({
                            let focus_handle = focus_handle.clone();
                            move |_window, cx| {
                                Tooltip::for_action_in(
                                    "Archive Thread",
                                    &RemoveSelectedThread,
                                    &focus_handle,
                                    cx,
                                )
                            }
                        })
                        .on_click({
                            let session_id = session_id_for_delete.clone();
                            cx.listener(move |this, _, window, cx| {
                                if let Some(ref session_id) = session_id {
                                    this.archive_thread(session_id, window, cx);
                                }
                            })
                        }),
                )
            })
            .on_click({
                cx.listener(move |this, _, window, cx| {
                    this.selection = None;
                    match &thread_workspace {
                        ThreadEntryWorkspace::Open(workspace) => {
                            this.activate_thread(metadata.clone(), workspace, false, window, cx);
                        }
                        ThreadEntryWorkspace::Closed {
                            folder_paths,
                            project_group_key,
                        } => {
                            this.open_workspace_and_activate_thread(
                                metadata.clone(),
                                folder_paths.clone(),
                                project_group_key,
                                window,
                                cx,
                            );
                        }
                    }
                })
            })
            .into_any_element()
    }

    fn render_filter_input(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .min_w_0()
            .flex_1()
            .capture_action(
                cx.listener(|this, _: &editor::actions::Newline, window, cx| {
                    this.editor_confirm(window, cx);
                }),
            )
            .child(self.filter_editor.clone())
    }

    fn render_recent_projects_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let multi_workspace = self.multi_workspace.upgrade();

        let workspace = multi_workspace
            .as_ref()
            .map(|mw| mw.read(cx).workspace().downgrade());

        let focus_handle = workspace
            .as_ref()
            .and_then(|ws| ws.upgrade())
            .map(|w| w.read(cx).focus_handle(cx))
            .unwrap_or_else(|| cx.focus_handle());

        let window_project_groups: Vec<ProjectGroupKey> = multi_workspace
            .as_ref()
            .map(|mw| mw.read(cx).project_group_keys())
            .unwrap_or_default();

        let popover_handle = self.recent_projects_popover_handle.clone();

        PopoverMenu::new("sidebar-recent-projects-menu")
            .with_handle(popover_handle)
            .menu(move |window, cx| {
                workspace.as_ref().map(|ws| {
                    SidebarRecentProjects::popover(
                        ws.clone(),
                        window_project_groups.clone(),
                        focus_handle.clone(),
                        window,
                        cx,
                    )
                })
            })
            .trigger_with_tooltip(
                IconButton::new("open-project", IconName::OpenFolder)
                    .icon_size(IconSize::Small)
                    .selected_style(ButtonStyle::Tinted(TintColor::Accent)),
                |_window, cx| {
                    Tooltip::for_action(
                        "Add Project",
                        &OpenRecent {
                            create_new_window: false,
                        },
                        cx,
                    )
                },
            )
            .offset(gpui::Point {
                x: px(-2.0),
                y: px(-2.0),
            })
            .anchor(gpui::Corner::BottomRight)
    }

    fn render_view_more(
        &self,
        ix: usize,
        key: &ProjectGroupKey,
        is_fully_expanded: bool,
        is_selected: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let key = key.clone();
        let id = SharedString::from(format!("view-more-{}", ix));

        let label: SharedString = if is_fully_expanded {
            "Collapse".into()
        } else {
            "View More".into()
        };

        ThreadItem::new(id, label)
            .focused(is_selected)
            .icon_visible(false)
            .title_label_color(Color::Muted)
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.selection = None;
                if is_fully_expanded {
                    this.reset_thread_group_expansion(&key, cx);
                } else {
                    this.expand_thread_group(&key, cx);
                }
            }))
            .into_any_element()
    }

    fn new_thread_in_group(
        &mut self,
        _: &NewThreadInGroup,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // If there is a keyboard selection, walk backwards through
        // `project_header_indices` to find the header that owns the selected
        // row. Otherwise fall back to the active workspace.
        // Always use the currently active workspace so that drafts
        // are created in the linked worktree the user is focused on,
        // not the main worktree resolved from the project header.
        let workspace = self
            .multi_workspace
            .upgrade()
            .map(|mw| mw.read(cx).workspace().clone());

        let Some(workspace) = workspace else {
            return;
        };

        self.create_new_thread(&workspace, window, cx);
    }

    fn create_new_thread(
        &mut self,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.activate(workspace.clone(), window, cx);
        });

        let draft_id = workspace.update(cx, |workspace, cx| {
            let panel = workspace.panel::<AgentPanel>(cx)?;
            let draft_id = panel.update(cx, |panel, cx| {
                if let Some(id) = panel.draft_thread_ids(cx).first().copied() {
                    if panel.active_thread_id(cx) != Some(id) {
                        panel.activate_retained_thread(id, true, window, cx);
                    }
                    id
                } else {
                    let id = panel.create_thread(window, cx);
                    panel.activate_retained_thread(id, true, window, cx);
                    id
                }
            });
            workspace.focus_panel::<AgentPanel>(window, cx);
            Some(draft_id)
        });

        if let Some(draft_id) = draft_id {
            self.active_entry = Some(ActiveEntry {
                thread_id: draft_id,
                session_id: None,
                workspace: workspace.clone(),
            });
        }
    }

    fn remove_draft(
        &mut self,
        draft_id: ThreadId,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        workspace.update(cx, |ws, cx| {
            if let Some(panel) = ws.panel::<AgentPanel>(cx) {
                panel.update(cx, |panel, cx| {
                    panel.remove_thread(draft_id, cx);
                });
            }
        });

        let was_active = self
            .active_entry
            .as_ref()
            .is_some_and(|e| e.is_active_thread(&draft_id));

        if was_active {
            let group_key = workspace.read(cx).project_group_key(cx);

            // Find any remaining thread in the same group.
            let next = self.contents.entries.iter().find_map(|entry| {
                if let ListEntry::Thread(thread) = entry {
                    if thread.metadata.thread_id != draft_id {
                        if let ThreadEntryWorkspace::Open(ws) = &thread.workspace {
                            if ws.read(cx).project_group_key(cx) == group_key {
                                return Some((thread.metadata.clone(), ws.clone()));
                            }
                        }
                    }
                }
                None
            });
            if let Some((metadata, ws)) = next {
                self.activate_thread(metadata, &ws, false, window, cx);
            } else {
                self.active_entry = None;
            }
        }

        self.update_entries(cx);
        self.reconcile_groups(window, cx);
    }

    /// Cleans, collapses whitespace, and truncates raw editor text
    /// for display as a draft label in the sidebar.
    fn truncate_draft_label(raw: &str) -> Option<SharedString> {
        let first_line = raw.lines().next().unwrap_or("");
        let cleaned = Self::clean_mention_links(first_line);
        let mut text: String = cleaned.split_whitespace().collect::<Vec<_>>().join(" ");
        if text.is_empty() {
            return None;
        }
        const MAX_CHARS: usize = 250;
        if let Some((truncate_at, _)) = text.char_indices().nth(MAX_CHARS) {
            text.truncate(truncate_at);
        }
        Some(text.into())
    }

    /// Reads a draft's prompt text from its ConversationView in the AgentPanel.
    fn read_draft_text(
        &self,
        draft_id: ThreadId,
        workspace: &Entity<Workspace>,
        cx: &App,
    ) -> Option<SharedString> {
        let panel = workspace.read(cx).panel::<AgentPanel>(cx)?;
        let raw = panel.read(cx).editor_text(draft_id, cx)?;
        Self::truncate_draft_label(&raw)
    }

    fn active_project_group_key(&self, cx: &App) -> Option<ProjectGroupKey> {
        let multi_workspace = self.multi_workspace.upgrade()?;
        let multi_workspace = multi_workspace.read(cx);
        Some(multi_workspace.project_group_key_for_workspace(multi_workspace.workspace(), cx))
    }

    fn active_project_header_position(&self, cx: &App) -> Option<usize> {
        let active_key = self.active_project_group_key(cx)?;
        self.contents
            .project_header_indices
            .iter()
            .position(|&entry_ix| {
                matches!(
                    &self.contents.entries[entry_ix],
                    ListEntry::ProjectHeader { key, .. } if *key == active_key
                )
            })
    }

    fn cycle_project_impl(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let header_count = self.contents.project_header_indices.len();
        if header_count == 0 {
            return;
        }

        let current_pos = self.active_project_header_position(cx);

        let next_pos = match current_pos {
            Some(pos) => {
                if forward {
                    (pos + 1) % header_count
                } else {
                    (pos + header_count - 1) % header_count
                }
            }
            None => 0,
        };

        let header_entry_ix = self.contents.project_header_indices[next_pos];
        let Some(ListEntry::ProjectHeader { key, .. }) = self.contents.entries.get(header_entry_ix)
        else {
            return;
        };
        let key = key.clone();

        // Uncollapse the target group so that threads become visible.
        self.set_group_expanded(&key, true, cx);

        if let Some(workspace) = self.multi_workspace.upgrade().and_then(|mw| {
            mw.read(cx)
                .workspace_for_paths(key.path_list(), key.host().as_ref(), cx)
        }) {
            multi_workspace.update(cx, |multi_workspace, cx| {
                multi_workspace.activate(workspace, window, cx);
                multi_workspace.retain_active_workspace(cx);
            });
        } else {
            self.open_workspace_for_group(&key, window, cx);
        }
    }

    fn on_next_project(&mut self, _: &NextProject, window: &mut Window, cx: &mut Context<Self>) {
        self.cycle_project_impl(true, window, cx);
    }

    fn on_previous_project(
        &mut self,
        _: &PreviousProject,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.cycle_project_impl(false, window, cx);
    }

    fn cycle_thread_impl(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        let thread_indices: Vec<usize> = self
            .contents
            .entries
            .iter()
            .enumerate()
            .filter_map(|(ix, entry)| match entry {
                ListEntry::Thread(_) => Some(ix),
                _ => None,
            })
            .collect();

        if thread_indices.is_empty() {
            return;
        }

        let current_thread_pos = self.active_entry.as_ref().and_then(|active| {
            thread_indices
                .iter()
                .position(|&ix| active.matches_entry(&self.contents.entries[ix]))
        });

        let next_pos = match current_thread_pos {
            Some(pos) => {
                let count = thread_indices.len();
                if forward {
                    (pos + 1) % count
                } else {
                    (pos + count - 1) % count
                }
            }
            None => 0,
        };

        let entry_ix = thread_indices[next_pos];
        let ListEntry::Thread(thread) = &self.contents.entries[entry_ix] else {
            return;
        };

        let metadata = thread.metadata.clone();
        match &thread.workspace {
            ThreadEntryWorkspace::Open(workspace) => {
                let workspace = workspace.clone();
                self.activate_thread(metadata, &workspace, true, window, cx);
            }
            ThreadEntryWorkspace::Closed {
                folder_paths,
                project_group_key,
            } => {
                let folder_paths = folder_paths.clone();
                let project_group_key = project_group_key.clone();
                self.open_workspace_and_activate_thread(
                    metadata,
                    folder_paths,
                    &project_group_key,
                    window,
                    cx,
                );
            }
        }
    }

    fn on_next_thread(&mut self, _: &NextThread, window: &mut Window, cx: &mut Context<Self>) {
        self.cycle_thread_impl(true, window, cx);
    }

    fn on_previous_thread(
        &mut self,
        _: &PreviousThread,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.cycle_thread_impl(false, window, cx);
    }

    fn expand_thread_group(&mut self, project_group_key: &ProjectGroupKey, cx: &mut Context<Self>) {
        let current = self.group_extra_batches(project_group_key, cx);
        self.set_group_visible_thread_count(project_group_key, Some(current + 1), cx);
        self.update_entries(cx);
    }

    fn reset_thread_group_expansion(
        &mut self,
        project_group_key: &ProjectGroupKey,
        cx: &mut Context<Self>,
    ) {
        self.set_group_visible_thread_count(project_group_key, None, cx);
        self.update_entries(cx);
    }

    fn collapse_thread_group(
        &mut self,
        project_group_key: &ProjectGroupKey,
        cx: &mut Context<Self>,
    ) {
        let batches = self.group_extra_batches(project_group_key, cx);
        match batches {
            0 => return,
            1 => self.set_group_visible_thread_count(project_group_key, None, cx),
            _ => self.set_group_visible_thread_count(project_group_key, Some(batches - 1), cx),
        }
        self.update_entries(cx);
    }

    fn on_show_more_threads(
        &mut self,
        _: &ShowMoreThreads,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_key) = self.active_project_group_key(cx) else {
            return;
        };
        self.expand_thread_group(&active_key, cx);
    }

    fn on_show_fewer_threads(
        &mut self,
        _: &ShowFewerThreads,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_key) = self.active_project_group_key(cx) else {
            return;
        };
        self.collapse_thread_group(&active_key, cx);
    }

    fn on_new_thread(
        &mut self,
        _: &workspace::NewThread,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.active_workspace(cx) else {
            return;
        };
        self.create_new_thread(&workspace, window, cx);
    }

    fn render_no_results(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let has_query = self.has_filter_query(cx);
        let message = if has_query {
            "No threads match your search."
        } else {
            "No threads yet"
        };

        v_flex()
            .id("sidebar-no-results")
            .p_4()
            .size_full()
            .items_center()
            .justify_center()
            .child(
                Label::new(message)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
    }

    fn render_empty_state(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("sidebar-empty-state")
            .p_4()
            .size_full()
            .items_center()
            .justify_center()
            .gap_1()
            .track_focus(&self.focus_handle(cx))
            .child(
                Button::new("open_project", "Open Project")
                    .full_width()
                    .key_binding(KeyBinding::for_action(&workspace::Open::default(), cx))
                    .on_click(|_, window, cx| {
                        window.dispatch_action(
                            Open {
                                create_new_window: false,
                            }
                            .boxed_clone(),
                            cx,
                        );
                    }),
            )
            .child(
                h_flex()
                    .w_1_2()
                    .gap_2()
                    .child(Divider::horizontal().color(ui::DividerColor::Border))
                    .child(Label::new("or").size(LabelSize::XSmall).color(Color::Muted))
                    .child(Divider::horizontal().color(ui::DividerColor::Border)),
            )
            .child(
                Button::new("clone_repo", "Clone Repository")
                    .full_width()
                    .on_click(|_, window, cx| {
                        window.dispatch_action(git::Clone.boxed_clone(), cx);
                    }),
            )
    }

    fn render_sidebar_header(
        &self,
        no_open_projects: bool,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let has_query = self.has_filter_query(cx);
        let sidebar_on_left = self.side(cx) == SidebarSide::Left;
        let sidebar_on_right = self.side(cx) == SidebarSide::Right;
        let not_fullscreen = !window.is_fullscreen();
        let traffic_lights = cfg!(target_os = "macos") && not_fullscreen && sidebar_on_left;
        let left_window_controls = !cfg!(target_os = "macos") && not_fullscreen && sidebar_on_left;
        let right_window_controls =
            !cfg!(target_os = "macos") && not_fullscreen && sidebar_on_right;
        let header_height = platform_title_bar_height(window);

        h_flex()
            .h(header_height)
            .mt_px()
            .pb_px()
            .when(left_window_controls, |this| {
                this.children(Self::render_left_window_controls(window, cx))
            })
            .map(|this| {
                if traffic_lights {
                    this.pl(px(ui::utils::TRAFFIC_LIGHT_PADDING))
                } else if !left_window_controls {
                    this.pl_1p5()
                } else {
                    this
                }
            })
            .when(!right_window_controls, |this| this.pr_1p5())
            .gap_1()
            .when(!no_open_projects, |this| {
                this.border_b_1()
                    .border_color(cx.theme().colors().border)
                    .when(traffic_lights, |this| {
                        this.child(Divider::vertical().color(ui::DividerColor::Border))
                    })
                    .child(
                        div().ml_1().child(
                            Icon::new(IconName::MagnifyingGlass)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                    .child(self.render_filter_input(cx))
                    .child(
                        h_flex()
                            .gap_1()
                            .when(
                                self.selection.is_some()
                                    && !self.filter_editor.focus_handle(cx).is_focused(window),
                                |this| this.child(KeyBinding::for_action(&FocusSidebarFilter, cx)),
                            )
                            .when(has_query, |this| {
                                this.child(
                                    IconButton::new("clear_filter", IconName::Close)
                                        .icon_size(IconSize::Small)
                                        .tooltip(Tooltip::text("Clear Search"))
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.reset_filter_editor_text(window, cx);
                                            this.update_entries(cx);
                                        })),
                                )
                            }),
                    )
            })
            .when(right_window_controls, |this| {
                this.children(Self::render_right_window_controls(window, cx))
            })
    }

    fn render_left_window_controls(window: &Window, cx: &mut App) -> Option<AnyElement> {
        platform_title_bar::render_left_window_controls(
            cx.button_layout(),
            Box::new(CloseWindow),
            window,
        )
    }

    fn render_right_window_controls(window: &Window, cx: &mut App) -> Option<AnyElement> {
        platform_title_bar::render_right_window_controls(
            cx.button_layout(),
            Box::new(CloseWindow),
            window,
        )
    }

    fn render_sidebar_toggle_button(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let on_right = AgentSettings::get_global(_cx).sidebar_side() == SidebarSide::Right;

        sidebar_side_context_menu("sidebar-toggle-menu", _cx)
            .anchor(if on_right {
                gpui::Corner::BottomRight
            } else {
                gpui::Corner::BottomLeft
            })
            .attach(if on_right {
                gpui::Corner::TopRight
            } else {
                gpui::Corner::TopLeft
            })
            .trigger(move |_is_active, _window, _cx| {
                let icon = if on_right {
                    IconName::ThreadsSidebarRightOpen
                } else {
                    IconName::ThreadsSidebarLeftOpen
                };
                IconButton::new("sidebar-close-toggle", icon)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::element(move |_window, cx| {
                        v_flex()
                            .gap_1()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .justify_between()
                                    .child(Label::new("Toggle Sidebar"))
                                    .child(KeyBinding::for_action(&ToggleWorkspaceSidebar, cx)),
                            )
                            .child(
                                h_flex()
                                    .pt_1()
                                    .gap_2()
                                    .border_t_1()
                                    .border_color(cx.theme().colors().border_variant)
                                    .justify_between()
                                    .child(Label::new("Focus Sidebar"))
                                    .child(KeyBinding::for_action(&FocusWorkspaceSidebar, cx)),
                            )
                            .into_any_element()
                    }))
                    .on_click(|_, window, cx| {
                        if let Some(multi_workspace) = window.root::<MultiWorkspace>().flatten() {
                            multi_workspace.update(cx, |multi_workspace, cx| {
                                multi_workspace.close_sidebar(window, cx);
                            });
                        }
                    })
            })
    }

    fn render_sidebar_bottom_bar(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_archive = matches!(self.view, SidebarView::Archive(..));
        let show_import_button = is_archive && !self.should_render_acp_import_onboarding(cx);
        let on_right = self.side(cx) == SidebarSide::Right;

        let action_buttons = h_flex()
            .gap_1()
            .when(on_right, |this| this.flex_row_reverse())
            .when(show_import_button, |this| {
                this.child(
                    IconButton::new("thread-import", IconName::ThreadImport)
                        .icon_size(IconSize::Small)
                        .tooltip(Tooltip::text("Import ACP Threads"))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.show_archive(window, cx);
                            this.show_thread_import_modal(window, cx);
                        })),
                )
            })
            .child(
                IconButton::new("archive", IconName::Archive)
                    .icon_size(IconSize::Small)
                    .toggle_state(is_archive)
                    .tooltip(move |_, cx| {
                        Tooltip::for_action("Toggle Archived Threads", &ToggleArchive, cx)
                    })
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.toggle_archive(&ToggleArchive, window, cx);
                    })),
            )
            .child(self.render_recent_projects_button(cx));

        h_flex()
            .p_1()
            .gap_1()
            .when(on_right, |this| this.flex_row_reverse())
            .justify_between()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(self.render_sidebar_toggle_button(cx))
            .child(action_buttons)
    }

    fn active_workspace(&self, cx: &App) -> Option<Entity<Workspace>> {
        self.multi_workspace
            .upgrade()
            .map(|w| w.read(cx).workspace().clone())
    }

    fn show_thread_import_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_workspace) = self.active_workspace(cx) else {
            return;
        };

        let Some(agent_registry_store) = AgentRegistryStore::try_global(cx) else {
            return;
        };

        let agent_server_store = active_workspace
            .read(cx)
            .project()
            .read(cx)
            .agent_server_store()
            .clone();

        let workspace_handle = active_workspace.downgrade();
        let multi_workspace = self.multi_workspace.clone();

        active_workspace.update(cx, |workspace, cx| {
            workspace.toggle_modal(window, cx, |window, cx| {
                ThreadImportModal::new(
                    agent_server_store,
                    agent_registry_store,
                    workspace_handle.clone(),
                    multi_workspace.clone(),
                    window,
                    cx,
                )
            });
        });
    }

    fn should_render_acp_import_onboarding(&self, cx: &App) -> bool {
        let has_external_agents = self
            .active_workspace(cx)
            .map(|ws| {
                ws.read(cx)
                    .project()
                    .read(cx)
                    .agent_server_store()
                    .read(cx)
                    .has_external_agents()
            })
            .unwrap_or(false);

        has_external_agents && !AcpThreadImportOnboarding::dismissed(cx)
    }

    fn render_acp_import_onboarding(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let description = "Import threads from agents like Claude Agent, Codex, and more, whether started in Zed or another client.";

        let bg = cx.theme().colors().text_accent;

        v_flex()
            .min_w_0()
            .w_full()
            .p_2()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .bg(linear_gradient(
                360.,
                linear_color_stop(bg.opacity(0.06), 1.),
                linear_color_stop(bg.opacity(0.), 0.),
            ))
            .child(
                h_flex()
                    .min_w_0()
                    .w_full()
                    .gap_1()
                    .justify_between()
                    .child(Label::new("Looking for threads from external agents?"))
                    .child(
                        IconButton::new("close-onboarding", IconName::Close)
                            .icon_size(IconSize::Small)
                            .on_click(|_, _window, cx| AcpThreadImportOnboarding::dismiss(cx)),
                    ),
            )
            .child(Label::new(description).color(Color::Muted).mb_2())
            .child(
                Button::new("import-acp", "Import Threads")
                    .full_width()
                    .style(ButtonStyle::OutlinedCustom(cx.theme().colors().border))
                    .label_size(LabelSize::Small)
                    .start_icon(
                        Icon::new(IconName::ThreadImport)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.show_archive(window, cx);
                        this.show_thread_import_modal(window, cx);
                    })),
            )
    }

    fn toggle_archive(&mut self, _: &ToggleArchive, window: &mut Window, cx: &mut Context<Self>) {
        match &self.view {
            SidebarView::ThreadList => self.show_archive(window, cx),
            SidebarView::Archive(_) => self.show_thread_list(window, cx),
        }
    }

    fn show_archive(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_workspace) = self
            .multi_workspace
            .upgrade()
            .map(|w| w.read(cx).workspace().clone())
        else {
            return;
        };
        let Some(agent_panel) = active_workspace.read(cx).panel::<AgentPanel>(cx) else {
            return;
        };

        let agent_server_store = active_workspace
            .read(cx)
            .project()
            .read(cx)
            .agent_server_store()
            .downgrade();

        let agent_connection_store = agent_panel.read(cx).connection_store().downgrade();

        let archive_view = cx.new(|cx| {
            ThreadsArchiveView::new(
                active_workspace.downgrade(),
                agent_connection_store.clone(),
                agent_server_store.clone(),
                window,
                cx,
            )
        });

        let subscription = cx.subscribe_in(
            &archive_view,
            window,
            |this, _, event: &ThreadsArchiveViewEvent, window, cx| match event {
                ThreadsArchiveViewEvent::Close => {
                    this.show_thread_list(window, cx);
                }
                ThreadsArchiveViewEvent::Unarchive { thread } => {
                    this.activate_archived_thread(thread.clone(), window, cx);
                }
                ThreadsArchiveViewEvent::CancelRestore { thread_id } => {
                    this.restoring_tasks.remove(thread_id);
                }
            },
        );

        self._subscriptions.push(subscription);
        self.view = SidebarView::Archive(archive_view.clone());
        archive_view.update(cx, |view, cx| view.focus_filter_editor(window, cx));
        self.serialize(cx);
        cx.notify();
    }

    fn show_thread_list(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.view = SidebarView::ThreadList;
        self._subscriptions.clear();
        let handle = self.filter_editor.read(cx).focus_handle(cx);
        handle.focus(window, cx);
        self.serialize(cx);
        cx.notify();
    }
}

impl WorkspaceSidebar for Sidebar {
    fn width(&self, _cx: &App) -> Pixels {
        self.width
    }

    fn set_width(&mut self, width: Option<Pixels>, cx: &mut Context<Self>) {
        self.width = width.unwrap_or(DEFAULT_WIDTH).clamp(MIN_WIDTH, MAX_WIDTH);
        cx.notify();
    }

    fn has_notifications(&self, _cx: &App) -> bool {
        !self.contents.notified_threads.is_empty()
    }

    fn is_threads_list_view_active(&self) -> bool {
        matches!(self.view, SidebarView::ThreadList)
    }

    fn side(&self, cx: &App) -> SidebarSide {
        AgentSettings::get_global(cx).sidebar_side()
    }

    fn prepare_for_focus(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.selection = None;
        cx.notify();
    }

    fn toggle_thread_switcher(
        &mut self,
        select_last: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.toggle_thread_switcher_impl(select_last, window, cx);
    }

    fn cycle_project(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.cycle_project_impl(forward, window, cx);
    }

    fn cycle_thread(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.cycle_thread_impl(forward, window, cx);
    }

    fn serialized_state(&self, _cx: &App) -> Option<String> {
        let serialized = SerializedSidebar {
            width: Some(f32::from(self.width)),
            active_view: match self.view {
                SidebarView::ThreadList => SerializedSidebarView::ThreadList,
                SidebarView::Archive(_) => SerializedSidebarView::Archive,
            },
        };
        serde_json::to_string(&serialized).ok()
    }

    fn restore_serialized_state(
        &mut self,
        state: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(serialized) = serde_json::from_str::<SerializedSidebar>(state).log_err() {
            if let Some(width) = serialized.width {
                self.width = px(width).clamp(MIN_WIDTH, MAX_WIDTH);
            }
            if serialized.active_view == SerializedSidebarView::Archive {
                cx.defer_in(window, |this, window, cx| {
                    this.show_archive(window, cx);
                });
            }
        }
        cx.notify();
    }
}

impl gpui::EventEmitter<workspace::SidebarEvent> for Sidebar {}

impl Focusable for Sidebar {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let _titlebar_height = ui::utils::platform_title_bar_height(window);
        let ui_font = theme_settings::setup_ui_font(window, cx);
        let sticky_header = self.render_sticky_header(window, cx);

        let color = cx.theme().colors();
        let bg = color
            .title_bar_background
            .blend(color.panel_background.opacity(0.25));

        let no_open_projects = !self.contents.has_open_projects;
        let no_search_results = self.contents.entries.is_empty();

        v_flex()
            .id("workspace-sidebar")
            .key_context(self.dispatch_context(window, cx))
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::editor_move_down))
            .on_action(cx.listener(Self::editor_move_up))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::expand_selected_entry))
            .on_action(cx.listener(Self::collapse_selected_entry))
            .on_action(cx.listener(Self::toggle_selected_fold))
            .on_action(cx.listener(Self::fold_all))
            .on_action(cx.listener(Self::unfold_all))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::remove_selected_thread))
            .on_action(cx.listener(Self::new_thread_in_group))
            .on_action(cx.listener(Self::toggle_archive))
            .on_action(cx.listener(Self::focus_sidebar_filter))
            .on_action(cx.listener(Self::on_toggle_thread_switcher))
            .on_action(cx.listener(Self::on_next_project))
            .on_action(cx.listener(Self::on_previous_project))
            .on_action(cx.listener(Self::on_next_thread))
            .on_action(cx.listener(Self::on_previous_thread))
            .on_action(cx.listener(Self::on_show_more_threads))
            .on_action(cx.listener(Self::on_show_fewer_threads))
            .on_action(cx.listener(Self::on_new_thread))
            .on_action(cx.listener(|this, _: &OpenRecent, window, cx| {
                this.recent_projects_popover_handle.toggle(window, cx);
            }))
            .font(ui_font)
            .h_full()
            .w(self.width)
            .bg(bg)
            .when(self.side(cx) == SidebarSide::Left, |el| el.border_r_1())
            .when(self.side(cx) == SidebarSide::Right, |el| el.border_l_1())
            .border_color(color.border)
            .map(|this| match &self.view {
                SidebarView::ThreadList => this
                    .child(self.render_sidebar_header(no_open_projects, window, cx))
                    .map(|this| {
                        if no_open_projects {
                            this.child(self.render_empty_state(cx))
                        } else {
                            this.child(
                                v_flex()
                                    .relative()
                                    .flex_1()
                                    .overflow_hidden()
                                    .child(
                                        list(
                                            self.list_state.clone(),
                                            cx.processor(Self::render_list_entry),
                                        )
                                        .flex_1()
                                        .size_full(),
                                    )
                                    .when(no_search_results, |this| {
                                        this.child(self.render_no_results(cx))
                                    })
                                    .when_some(sticky_header, |this, header| this.child(header))
                                    .vertical_scrollbar_for(&self.list_state, window, cx),
                            )
                        }
                    }),
                SidebarView::Archive(archive_view) => this.child(archive_view.clone()),
            })
            .when(self.should_render_acp_import_onboarding(cx), |this| {
                this.child(self.render_acp_import_onboarding(cx))
            })
            .child(self.render_sidebar_bottom_bar(cx))
    }
}

fn all_thread_infos_for_workspace(
    workspace: &Entity<Workspace>,
    cx: &App,
) -> impl Iterator<Item = ActiveThreadInfo> {
    let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) else {
        return None.into_iter().flatten();
    };
    let agent_panel = agent_panel.read(cx);
    let threads = agent_panel
        .conversation_views()
        .into_iter()
        .filter_map(|conversation_view| {
            let has_pending_tool_call = conversation_view
                .read(cx)
                .root_thread_has_pending_tool_call(cx);
            let conversation_thread_id = conversation_view.read(cx).parent_id();
            let thread_view = conversation_view.read(cx).root_thread(cx)?;
            let thread_view_ref = thread_view.read(cx);
            let thread = thread_view_ref.thread.read(cx);

            let icon = thread_view_ref.agent_icon;
            let icon_from_external_svg = thread_view_ref.agent_icon_from_external_svg.clone();
            let title = thread
                .title()
                .unwrap_or_else(|| DEFAULT_THREAD_TITLE.into());
            let is_native = thread_view_ref.as_native_thread(cx).is_some();
            let is_title_generating = is_native && thread.has_provisional_title();
            let session_id = thread.session_id().clone();
            let is_background = agent_panel.is_retained_thread(&conversation_thread_id);

            let status = if has_pending_tool_call {
                AgentThreadStatus::WaitingForConfirmation
            } else if thread.had_error() {
                AgentThreadStatus::Error
            } else {
                match thread.status() {
                    ThreadStatus::Generating => AgentThreadStatus::Running,
                    ThreadStatus::Idle => AgentThreadStatus::Completed,
                }
            };

            let diff_stats = thread.action_log().read(cx).diff_stats(cx);

            Some(ActiveThreadInfo {
                session_id,
                title,
                status,
                icon,
                icon_from_external_svg,
                is_background,
                is_title_generating,
                diff_stats,
            })
        });

    Some(threads).into_iter().flatten()
}

pub fn dump_workspace_info(
    workspace: &mut Workspace,
    _: &DumpWorkspaceInfo,
    window: &mut gpui::Window,
    cx: &mut gpui::Context<Workspace>,
) {
    use std::fmt::Write;

    let mut output = String::new();
    let this_entity = cx.entity();

    let multi_workspace = workspace.multi_workspace().and_then(|weak| weak.upgrade());
    let workspaces: Vec<gpui::Entity<Workspace>> = match &multi_workspace {
        Some(mw) => mw.read(cx).workspaces().cloned().collect(),
        None => vec![this_entity.clone()],
    };
    let active_workspace = multi_workspace
        .as_ref()
        .map(|mw| mw.read(cx).workspace().clone());

    writeln!(output, "MultiWorkspace: {} workspace(s)", workspaces.len()).ok();

    if let Some(mw) = &multi_workspace {
        let keys: Vec<_> = mw.read(cx).project_group_keys();
        writeln!(output, "Project group keys ({}):", keys.len()).ok();
        for key in keys {
            writeln!(output, "  - {key:?}").ok();
        }
    }

    writeln!(output).ok();

    for (index, ws) in workspaces.iter().enumerate() {
        let is_active = active_workspace.as_ref() == Some(ws);
        writeln!(
            output,
            "--- Workspace {index}{} ---",
            if is_active { " (active)" } else { "" }
        )
        .ok();

        // project_group_key_for_workspace internally reads the workspace,
        // so we can only call it for workspaces other than this_entity
        // (which is already being updated).
        if let Some(mw) = &multi_workspace {
            if *ws == this_entity {
                let workspace_key = workspace.project_group_key(cx);
                writeln!(output, "ProjectGroupKey: {workspace_key:?}").ok();
            } else {
                let effective_key = mw.read(cx).project_group_key_for_workspace(ws, cx);
                let workspace_key = ws.read(cx).project_group_key(cx);
                if effective_key != workspace_key {
                    writeln!(
                        output,
                        "ProjectGroupKey (multi_workspace): {effective_key:?}"
                    )
                    .ok();
                    writeln!(
                        output,
                        "ProjectGroupKey (workspace, DISAGREES): {workspace_key:?}"
                    )
                    .ok();
                } else {
                    writeln!(output, "ProjectGroupKey: {effective_key:?}").ok();
                }
            }
        } else {
            let workspace_key = workspace.project_group_key(cx);
            writeln!(output, "ProjectGroupKey: {workspace_key:?}").ok();
        }

        // The action handler is already inside an update on `this_entity`,
        // so we must avoid a nested read/update on that same entity.
        if *ws == this_entity {
            dump_single_workspace(workspace, &mut output, cx);
        } else {
            ws.read_with(cx, |ws, cx| {
                dump_single_workspace(ws, &mut output, cx);
            });
        }
    }

    let project = workspace.project().clone();
    cx.spawn_in(window, async move |_this, cx| {
        let buffer = project
            .update(cx, |project, cx| project.create_buffer(None, false, cx))
            .await?;

        buffer.update(cx, |buffer, cx| {
            buffer.set_text(output, cx);
        });

        let buffer = cx.new(|cx| {
            editor::MultiBuffer::singleton(buffer, cx).with_title("Workspace Info".into())
        });

        _this.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(
                Box::new(cx.new(|cx| {
                    let mut editor =
                        editor::Editor::for_multibuffer(buffer, Some(project.clone()), window, cx);
                    editor.set_read_only(true);
                    editor.set_should_serialize(false, cx);
                    editor.set_breadcrumb_header("Workspace Info".into());
                    editor
                })),
                None,
                true,
                window,
                cx,
            );
        })
    })
    .detach_and_log_err(cx);
}

fn dump_single_workspace(workspace: &Workspace, output: &mut String, cx: &gpui::App) {
    use std::fmt::Write;

    let workspace_db_id = workspace.database_id();
    match workspace_db_id {
        Some(id) => writeln!(output, "Workspace DB ID: {id:?}").ok(),
        None => writeln!(output, "Workspace DB ID: (none)").ok(),
    };

    let project = workspace.project().read(cx);

    let repos: Vec<_> = project
        .repositories(cx)
        .values()
        .map(|repo| repo.read(cx).snapshot())
        .collect();

    writeln!(output, "Worktrees:").ok();
    for worktree in project.worktrees(cx) {
        let worktree = worktree.read(cx);
        let abs_path = worktree.abs_path();
        let visible = worktree.is_visible();

        let repo_info = repos
            .iter()
            .find(|snapshot| abs_path.starts_with(&*snapshot.work_directory_abs_path));

        let is_linked = repo_info.map(|s| s.is_linked_worktree()).unwrap_or(false);
        let original_repo_path = repo_info.map(|s| &s.original_repo_abs_path);
        let branch = repo_info.and_then(|s| s.branch.as_ref().map(|b| b.ref_name.clone()));

        write!(output, "  - {}", abs_path.display()).ok();
        if !visible {
            write!(output, " (hidden)").ok();
        }
        if let Some(branch) = &branch {
            write!(output, " [branch: {branch}]").ok();
        }
        if is_linked {
            if let Some(original) = original_repo_path {
                write!(output, " [linked worktree -> {}]", original.display()).ok();
            } else {
                write!(output, " [linked worktree]").ok();
            }
        }
        writeln!(output).ok();
    }

    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
        let panel = panel.read(cx);

        let panel_workspace_id = panel.workspace_id();
        if panel_workspace_id != workspace_db_id {
            writeln!(
                output,
                "  \u{26a0} workspace ID mismatch! panel has {panel_workspace_id:?}, workspace has {workspace_db_id:?}"
            )
            .ok();
        }

        if let Some(thread) = panel.active_agent_thread(cx) {
            let thread = thread.read(cx);
            let title = thread.title().unwrap_or_else(|| "(untitled)".into());
            let session_id = thread.session_id();
            let status = match thread.status() {
                ThreadStatus::Idle => "idle",
                ThreadStatus::Generating => "generating",
            };
            let entry_count = thread.entries().len();
            write!(output, "Active thread: {title} (session: {session_id})").ok();
            write!(output, " [{status}, {entry_count} entries").ok();
            if panel
                .active_conversation_view()
                .is_some_and(|conversation_view| {
                    conversation_view
                        .read(cx)
                        .root_thread_has_pending_tool_call(cx)
                })
            {
                write!(output, ", awaiting confirmation").ok();
            }
            writeln!(output, "]").ok();
        } else {
            writeln!(output, "Active thread: (none)").ok();
        }

        let background_threads = panel.retained_threads();
        if !background_threads.is_empty() {
            writeln!(
                output,
                "Background threads ({}): ",
                background_threads.len()
            )
            .ok();
            for (session_id, conversation_view) in background_threads {
                if let Some(thread_view) = conversation_view.read(cx).root_thread(cx) {
                    let thread = thread_view.read(cx).thread.read(cx);
                    let title = thread.title().unwrap_or_else(|| "(untitled)".into());
                    let status = match thread.status() {
                        ThreadStatus::Idle => "idle",
                        ThreadStatus::Generating => "generating",
                    };
                    let entry_count = thread.entries().len();
                    write!(output, "  - {title} (thread: {session_id:?})").ok();
                    write!(output, " [{status}, {entry_count} entries").ok();
                    if conversation_view
                        .read(cx)
                        .root_thread_has_pending_tool_call(cx)
                    {
                        write!(output, ", awaiting confirmation").ok();
                    }
                    writeln!(output, "]").ok();
                } else {
                    writeln!(output, "  - (not connected) (thread: {session_id:?})").ok();
                }
            }
        }
    } else {
        writeln!(output, "Agent panel: not loaded").ok();
    }

    writeln!(output).ok();
}
