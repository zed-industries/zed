mod thread_switcher;

use acp_thread::ThreadStatus;
use action_log::DiffStats;
use agent_client_protocol::{self as acp};
use agent_settings::AgentSettings;
use agent_ui::thread_metadata_store::{ThreadMetadata, ThreadMetadataStore};
use agent_ui::thread_worktree_archive;
use agent_ui::threads_archive_view::{
    ThreadsArchiveView, ThreadsArchiveViewEvent, format_history_entry_timestamp,
};
use agent_ui::{AcpThreadImportOnboarding, ThreadImportModal};
use agent_ui::{
    Agent, AgentPanel, AgentPanelEvent, DEFAULT_THREAD_TITLE, NewThread, RemoveSelectedThread,
};
use chrono::{DateTime, Utc};
use editor::Editor;
use gpui::{
    Action as _, AnyElement, App, Context, Entity, FocusHandle, Focusable, KeyContext, ListState,
    Pixels, Render, SharedString, Task, WeakEntity, Window, WindowHandle, linear_color_stop,
    linear_gradient, list, prelude::*, px,
};
use menu::{
    Cancel, Confirm, SelectChild, SelectFirst, SelectLast, SelectNext, SelectParent, SelectPrevious,
};
use project::{
    AgentId, AgentRegistryStore, Event as ProjectEvent, ProjectGroupKey, linked_worktree_short_name,
};
use recent_projects::sidebar_recent_projects::SidebarRecentProjects;
use remote::RemoteConnectionOptions;
use ui::utils::platform_title_bar_height;

use serde::{Deserialize, Serialize};
use settings::Settings as _;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::path::PathBuf;
use std::rc::Rc;
use theme::ActiveTheme;
use ui::{
    AgentThreadStatus, CommonAnimationExt, ContextMenu, Divider, HighlightedLabel, KeyBinding,
    PopoverMenu, PopoverMenuHandle, Tab, ThreadItem, ThreadItemWorktreeInfo, TintColor, Tooltip,
    WithScrollbar, prelude::*,
};
use util::ResultExt as _;
use util::path_list::{PathList, SerializedPathList};
use workspace::{
    AddFolderToProject, CloseWindow, FocusWorkspaceSidebar, MultiWorkspace, MultiWorkspaceEvent,
    NextProject, NextThread, Open, PreviousProject, PreviousThread, ShowFewerThreads,
    ShowMoreThreads, Sidebar as WorkspaceSidebar, SidebarSide, Toast, ToggleWorkspaceSidebar,
    Workspace, notifications::NotificationId, sidebar_side_context_menu,
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
    collapsed_groups: Vec<SerializedPathList>,
    #[serde(default)]
    expanded_groups: Vec<(SerializedPathList, usize)>,
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
enum ActiveEntry {
    Thread {
        session_id: acp::SessionId,
        workspace: Entity<Workspace>,
    },
    Draft(Entity<Workspace>),
}

impl ActiveEntry {
    fn workspace(&self) -> &Entity<Workspace> {
        match self {
            ActiveEntry::Thread { workspace, .. } => workspace,
            ActiveEntry::Draft(workspace) => workspace,
        }
    }

    fn is_active_thread(&self, session_id: &acp::SessionId) -> bool {
        matches!(self, ActiveEntry::Thread { session_id: id, .. } if id == session_id)
    }

    fn matches_entry(&self, entry: &ListEntry) -> bool {
        match (self, entry) {
            (ActiveEntry::Thread { session_id, .. }, ListEntry::Thread(thread)) => {
                thread.metadata.session_id == *session_id
            }
            (ActiveEntry::Draft(_workspace), ListEntry::DraftThread { .. }) => true,
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
    Closed(PathList),
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
        self.metadata.title = info.title.clone();
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
    },
    Thread(ThreadEntry),
    ViewMore {
        key: ProjectGroupKey,
        is_fully_expanded: bool,
    },
    /// The user's active draft thread. Shows a prefix of the currently-typed
    /// prompt, or "Untitled Thread" if the prompt is empty.
    DraftThread {
        worktrees: Vec<WorktreeInfo>,
    },
    /// A convenience row for starting a new thread. Shown when a project group
    /// has no threads, or when an open linked worktree workspace has no threads.
    /// When `workspace` is `Some`, this entry is for a specific linked worktree
    /// workspace and can be dismissed (removing that workspace).
    NewThread {
        key: project::ProjectGroupKey,
        worktrees: Vec<WorktreeInfo>,
        workspace: Option<Entity<Workspace>>,
    },
}

#[cfg(test)]
impl ListEntry {
    fn session_id(&self) -> Option<&acp::SessionId> {
        match self {
            ListEntry::Thread(thread_entry) => Some(&thread_entry.metadata.session_id),
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
                ThreadEntryWorkspace::Closed(_) => Vec::new(),
            },
            ListEntry::DraftThread { .. } => {
                vec![multi_workspace.workspace().clone()]
            }
            ListEntry::ProjectHeader { key, .. } => {
                // The header only activates the main worktree workspace
                // (the one whose root paths match the group key's path list).
                multi_workspace
                    .workspaces()
                    .find(|ws| PathList::new(&ws.read(cx).root_paths(cx)) == *key.path_list())
                    .cloned()
                    .into_iter()
                    .collect()
            }
            ListEntry::NewThread { key, workspace, .. } => {
                // When the NewThread entry is for a specific linked worktree
                // workspace, that workspace is reachable. Otherwise fall back
                // to the main worktree workspace.
                if let Some(ws) = workspace {
                    vec![ws.clone()]
                } else {
                    multi_workspace
                        .workspaces()
                        .find(|ws| PathList::new(&ws.read(cx).root_paths(cx)) == *key.path_list())
                        .cloned()
                        .into_iter()
                        .collect()
                }
            }
            _ => Vec::new(),
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
    notified_threads: HashSet<acp::SessionId>,
    project_header_indices: Vec<usize>,
    has_open_projects: bool,
}

impl SidebarContents {
    fn is_thread_notified(&self, session_id: &acp::SessionId) -> bool {
        self.notified_threads.contains(session_id)
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
/// the worktree is the main checkout or a linked git worktree.
fn worktree_info_from_thread_paths(
    folder_paths: &PathList,
    group_key: &project::ProjectGroupKey,
) -> impl Iterator<Item = WorktreeInfo> {
    let main_paths = group_key.path_list().paths();
    folder_paths.paths().iter().filter_map(|path| {
        let is_main = main_paths.iter().any(|mp| mp.as_path() == path.as_path());
        if is_main {
            let name = path.file_name()?.to_string_lossy().to_string();
            Some(WorktreeInfo {
                name: SharedString::from(name),
                full_path: SharedString::from(path.display().to_string()),
                highlight_positions: Vec::new(),
                kind: ui::WorktreeKind::Main,
            })
        } else {
            let main_path = main_paths
                .iter()
                .find(|mp| mp.file_name() == path.file_name())
                .or(main_paths.first())?;
            Some(WorktreeInfo {
                name: linked_worktree_short_name(main_path, path).unwrap_or_default(),
                full_path: SharedString::from(path.display().to_string()),
                highlight_positions: Vec::new(),
                kind: ui::WorktreeKind::Linked,
            })
        }
    })
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
    collapsed_groups: HashSet<PathList>,
    expanded_groups: HashMap<PathList, usize>,
    /// Updated only in response to explicit user actions (clicking a
    /// thread, confirming in the thread switcher, etc.) — never from
    /// background data changes. Used to sort the thread switcher popup.
    thread_last_accessed: HashMap<acp::SessionId, DateTime<Utc>>,
    /// Updated when the user presses a key to send or queue a message.
    /// Used for sorting threads in the sidebar and as a secondary sort
    /// key in the thread switcher.
    thread_last_message_sent_or_queued: HashMap<acp::SessionId, DateTime<Utc>>,
    thread_switcher: Option<Entity<ThreadSwitcher>>,
    _thread_switcher_subscriptions: Vec<gpui::Subscription>,
    view: SidebarView,
    recent_projects_popover_handle: PopoverMenuHandle<SidebarRecentProjects>,
    project_header_menu_ix: Option<usize>,
    _subscriptions: Vec<gpui::Subscription>,
    _draft_observation: Option<gpui::Subscription>,
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
                    this.observe_draft_editor(cx);
                    this.update_entries(cx);
                }
                MultiWorkspaceEvent::WorkspaceAdded(workspace) => {
                    this.subscribe_to_workspace(workspace, window, cx);
                    this.update_entries(cx);
                }
                MultiWorkspaceEvent::WorkspaceRemoved(_) => {
                    this.update_entries(cx);
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
            collapsed_groups: HashSet::new(),
            expanded_groups: HashMap::new(),
            thread_last_accessed: HashMap::new(),
            thread_last_message_sent_or_queued: HashMap::new(),
            thread_switcher: None,
            _thread_switcher_subscriptions: Vec::new(),
            view: SidebarView::default(),
            recent_projects_popover_handle: PopoverMenuHandle::default(),
            project_header_menu_ix: None,
            _subscriptions: Vec::new(),
            _draft_observation: None,
        }
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        cx.emit(workspace::SidebarEvent::SerializeNeeded);
    }

    fn active_entry_workspace(&self) -> Option<&Entity<Workspace>> {
        self.active_entry.as_ref().map(|entry| entry.workspace())
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
            |this, _project, event, _window, cx| match event {
                ProjectEvent::WorktreeAdded(_)
                | ProjectEvent::WorktreeRemoved(_)
                | ProjectEvent::WorktreeOrderChanged => {
                    this.update_entries(cx);
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
                    }
                }
            },
        )
        .detach();

        self.observe_docks(workspace, cx);

        if let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
            self.subscribe_to_agent_panel(&agent_panel, window, cx);
            self.observe_draft_editor(cx);
        }
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
            |this, agent_panel, event: &AgentPanelEvent, _window, cx| match event {
                AgentPanelEvent::ActiveViewChanged => {
                    let is_new_draft = agent_panel
                        .read(cx)
                        .active_conversation_view()
                        .is_some_and(|cv| cv.read(cx).parent_id(cx).is_none());
                    if is_new_draft {
                        if let Some(active_workspace) = this
                            .multi_workspace
                            .upgrade()
                            .map(|mw| mw.read(cx).workspace().clone())
                        {
                            this.active_entry = Some(ActiveEntry::Draft(active_workspace));
                        }
                    }
                    this.observe_draft_editor(cx);
                    this.update_entries(cx);
                }
                AgentPanelEvent::ThreadFocused | AgentPanelEvent::BackgroundThreadChanged => {
                    this.update_entries(cx);
                }
                AgentPanelEvent::MessageSentOrQueued { session_id } => {
                    this.record_thread_message_sent(session_id);
                    this.update_entries(cx);
                }
            },
        )
        .detach();
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

    fn observe_draft_editor(&mut self, cx: &mut Context<Self>) {
        self._draft_observation = self
            .multi_workspace
            .upgrade()
            .and_then(|mw| {
                let ws = mw.read(cx).workspace();
                ws.read(cx).panel::<AgentPanel>(cx)
            })
            .and_then(|panel| {
                let cv = panel.read(cx).active_conversation_view()?;
                let tv = cv.read(cx).active_thread()?;
                Some(tv.read(cx).message_editor.clone())
            })
            .map(|editor| {
                cx.observe(&editor, |_this, _editor, cx| {
                    cx.notify();
                })
            });
    }

    fn active_draft_text(&self, cx: &App) -> Option<SharedString> {
        let mw = self.multi_workspace.upgrade()?;
        let workspace = mw.read(cx).workspace();
        let panel = workspace.read(cx).panel::<AgentPanel>(cx)?;
        let conversation_view = panel.read(cx).active_conversation_view()?;
        let thread_view = conversation_view.read(cx).active_thread()?;
        let raw = thread_view.read(cx).message_editor.read(cx).text(cx);
        let cleaned = Self::clean_mention_links(&raw);
        let mut text: String = cleaned.split_whitespace().collect::<Vec<_>>().join(" ");
        if text.is_empty() {
            None
        } else {
            const MAX_CHARS: usize = 250;
            if let Some((truncate_at, _)) = text.char_indices().nth(MAX_CHARS) {
                text.truncate(truncate_at);
            }
            Some(text.into())
        }
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

    /// Finds the main worktree workspace for a project group.
    fn workspace_for_group(&self, path_list: &PathList, cx: &App) -> Option<Entity<Workspace>> {
        let mw = self.multi_workspace.upgrade()?;
        mw.read(cx).workspace_for_paths(path_list, cx)
    }

    /// Opens a new workspace for a group that has no open workspaces.
    fn open_workspace_for_group(
        &mut self,
        path_list: &PathList,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        multi_workspace
            .update(cx, |this, cx| {
                this.find_or_create_local_workspace(path_list.clone(), window, cx)
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

        // Derive active_entry from the active workspace's agent panel.
        // Draft is checked first because a conversation can have a session_id
        // before any messages are sent. However, a thread that's still loading
        // also appears as a "draft" (no messages yet).
        if let Some(active_ws) = &active_workspace {
            if let Some(panel) = active_ws.read(cx).panel::<AgentPanel>(cx) {
                if panel.read(cx).active_thread_is_draft(cx)
                    || panel.read(cx).active_conversation_view().is_none()
                {
                    let conversation_parent_id = panel
                        .read(cx)
                        .active_conversation_view()
                        .and_then(|cv| cv.read(cx).parent_id(cx));
                    let preserving_thread =
                        if let Some(ActiveEntry::Thread { session_id, .. }) = &self.active_entry {
                            self.active_entry_workspace() == Some(active_ws)
                                && conversation_parent_id
                                    .as_ref()
                                    .is_some_and(|id| id == session_id)
                        } else {
                            false
                        };
                    if !preserving_thread {
                        self.active_entry = Some(ActiveEntry::Draft(active_ws.clone()));
                    }
                } else if let Some(session_id) = panel
                    .read(cx)
                    .active_conversation_view()
                    .and_then(|cv| cv.read(cx).parent_id(cx))
                {
                    self.active_entry = Some(ActiveEntry::Thread {
                        session_id,
                        workspace: active_ws.clone(),
                    });
                }
                // else: conversation exists, not a draft, but no session_id
                // yet — thread is mid-load. Keep previous value.
            }
        }

        let previous = mem::take(&mut self.contents);

        let old_statuses: HashMap<acp::SessionId, AgentThreadStatus> = previous
            .entries
            .iter()
            .filter_map(|entry| match entry {
                ListEntry::Thread(thread) if thread.is_live => {
                    Some((thread.metadata.session_id.clone(), thread.status))
                }
                _ => None,
            })
            .collect();

        let mut entries = Vec::new();
        let mut notified_threads = previous.notified_threads;
        let mut current_session_ids: HashSet<acp::SessionId> = HashSet::new();
        let mut project_header_indices: Vec<usize> = Vec::new();

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

        for (group_key, group_workspaces) in mw.project_groups(cx) {
            let path_list = group_key.path_list().clone();
            if path_list.paths().is_empty() {
                continue;
            }

            let label = group_key.display_name();

            let is_collapsed = self.collapsed_groups.contains(&path_list);
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
                let mut seen_session_ids: HashSet<acp::SessionId> = HashSet::new();
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
                        .get(&row.folder_paths)
                        .map(|ws| ThreadEntryWorkspace::Open((*ws).clone()))
                        .unwrap_or_else(|| ThreadEntryWorkspace::Closed(row.folder_paths.clone()))
                };

                // Build a ThreadEntry from a metadata row.
                let make_thread_entry = |row: ThreadMetadata,
                                         workspace: ThreadEntryWorkspace|
                 -> ThreadEntry {
                    let (icon, icon_from_external_svg) = resolve_agent_icon(&row.agent_id);
                    let worktrees: Vec<WorktreeInfo> =
                        worktree_info_from_thread_paths(&row.folder_paths, &group_key).collect();
                    ThreadEntry {
                        metadata: row,
                        icon,
                        icon_from_external_svg,
                        status: AgentThreadStatus::default(),
                        workspace,
                        is_live: false,
                        is_background: false,
                        is_title_generating: false,
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
                    .entries_for_main_worktree_path(&path_list)
                    .cloned()
                {
                    if !seen_session_ids.insert(row.session_id.clone()) {
                        continue;
                    }
                    let workspace = resolve_workspace(&row);
                    threads.push(make_thread_entry(row, workspace));
                }

                // Legacy threads did not have `main_worktree_paths` populated, so they
                // must be queried by their `folder_paths`.

                // Load any legacy threads for the main worktrees of this project group.
                for row in thread_store.read(cx).entries_for_path(&path_list).cloned() {
                    if !seen_session_ids.insert(row.session_id.clone()) {
                        continue;
                    }
                    let workspace = resolve_workspace(&row);
                    threads.push(make_thread_entry(row, workspace));
                }

                // Load any legacy threads for any single linked wortree of this project group.
                let mut linked_worktree_paths = HashSet::new();
                for workspace in &group_workspaces {
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
                        if !seen_session_ids.insert(row.session_id.clone()) {
                            continue;
                        }
                        threads.push(make_thread_entry(
                            row,
                            ThreadEntryWorkspace::Closed(worktree_path_list.clone()),
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
                    if let Some(info) = live_info_by_session.get(&thread.metadata.session_id) {
                        thread.apply_active_info(info);
                    }

                    let session_id = &thread.metadata.session_id;

                    let is_active_thread = self.active_entry.as_ref().is_some_and(|entry| {
                        entry.is_active_thread(session_id)
                            && active_workspace
                                .as_ref()
                                .is_some_and(|active| active == entry.workspace())
                    });

                    if thread.status == AgentThreadStatus::Completed
                        && !is_active_thread
                        && old_statuses.get(session_id) == Some(&AgentThreadStatus::Running)
                    {
                        notified_threads.insert(session_id.clone());
                    }

                    if is_active_thread && !thread.is_background {
                        notified_threads.remove(session_id);
                    }
                }

                threads.sort_by(|a, b| {
                    let a_time = self
                        .thread_last_message_sent_or_queued
                        .get(&a.metadata.session_id)
                        .copied()
                        .or(a.metadata.created_at)
                        .or(Some(a.metadata.updated_at));
                    let b_time = self
                        .thread_last_message_sent_or_queued
                        .get(&b.metadata.session_id)
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

            if !query.is_empty() {
                let workspace_highlight_positions =
                    fuzzy_match_positions(&query, &label).unwrap_or_default();
                let workspace_matched = !workspace_highlight_positions.is_empty();

                let mut matched_threads: Vec<ThreadEntry> = Vec::new();
                for mut thread in threads {
                    let title: &str = &thread.metadata.title;
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
                });

                for thread in matched_threads {
                    current_session_ids.insert(thread.metadata.session_id.clone());
                    entries.push(thread.into());
                }
            } else {
                let is_draft_for_group = is_active
                    && matches!(&self.active_entry, Some(ActiveEntry::Draft(ws)) if group_workspaces.contains(ws));

                project_header_indices.push(entries.len());
                entries.push(ListEntry::ProjectHeader {
                    key: group_key.clone(),
                    label,
                    highlight_positions: Vec::new(),
                    has_running_threads,
                    waiting_thread_count,
                    is_active,
                });

                if is_collapsed {
                    continue;
                }

                // Emit a DraftThread entry when the active draft belongs to this group.
                if is_draft_for_group {
                    if let Some(ActiveEntry::Draft(draft_ws)) = &self.active_entry {
                        let ws_path_list = workspace_path_list(draft_ws, cx);
                        let worktrees = worktree_info_from_thread_paths(&ws_path_list, &group_key);
                        entries.push(ListEntry::DraftThread {
                            worktrees: worktrees.collect(),
                        });
                    }
                }

                // Emit NewThread entries:
                // 1. When the group has zero threads (convenient affordance).
                // 2. For each open linked worktree workspace in this group
                //    that has no threads (makes the workspace reachable and
                //    dismissable).
                let group_has_no_threads = threads.is_empty() && !group_workspaces.is_empty();

                if !is_draft_for_group && group_has_no_threads {
                    entries.push(ListEntry::NewThread {
                        key: group_key.clone(),
                        worktrees: Vec::new(),
                        workspace: None,
                    });
                }

                // Emit a NewThread for each open linked worktree workspace
                // that has no threads. Skip the workspace if it's showing
                // the active draft (it already has a DraftThread entry).
                if !is_draft_for_group {
                    let thread_store = ThreadMetadataStore::global(cx);
                    for ws in &group_workspaces {
                        let ws_path_list = workspace_path_list(ws, cx);
                        let has_linked_worktrees =
                            worktree_info_from_thread_paths(&ws_path_list, &group_key)
                                .any(|wt| wt.kind == ui::WorktreeKind::Linked);
                        if !has_linked_worktrees {
                            continue;
                        }
                        let store = thread_store.read(cx);
                        let has_threads = store.entries_for_path(&ws_path_list).next().is_some()
                            || store
                                .entries_for_main_worktree_path(&ws_path_list)
                                .next()
                                .is_some();
                        if has_threads {
                            continue;
                        }
                        let worktrees: Vec<WorktreeInfo> =
                            worktree_info_from_thread_paths(&ws_path_list, &group_key).collect();
                        entries.push(ListEntry::NewThread {
                            key: group_key.clone(),
                            worktrees,
                            workspace: Some(ws.clone()),
                        });
                    }
                }

                let total = threads.len();

                let extra_batches = self.expanded_groups.get(&path_list).copied().unwrap_or(0);
                let threads_to_show =
                    DEFAULT_THREADS_SHOWN + (extra_batches * DEFAULT_THREADS_SHOWN);
                let count = threads_to_show.min(total);

                let mut promoted_threads: HashSet<acp::SessionId> = HashSet::new();

                // Build visible entries in a single pass. Threads within
                // the cutoff are always shown. Threads beyond it are shown
                // only if they should be promoted (running, waiting, or
                // focused)
                for (index, thread) in threads.into_iter().enumerate() {
                    let is_hidden = index >= count;

                    let session_id = &thread.metadata.session_id;
                    if is_hidden {
                        let is_promoted = thread.status == AgentThreadStatus::Running
                            || thread.status == AgentThreadStatus::WaitingForConfirmation
                            || notified_threads.contains(session_id)
                            || self.active_entry.as_ref().is_some_and(|active| {
                                active.matches_entry(&ListEntry::Thread(thread.clone()))
                            });
                        if is_promoted {
                            promoted_threads.insert(session_id.clone());
                        }
                        if !promoted_threads.contains(session_id) {
                            continue;
                        }
                    }

                    current_session_ids.insert(session_id.clone());
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

        // Prune stale notifications using the session IDs we collected during
        // the build pass (no extra scan needed).
        notified_threads.retain(|id| current_session_ids.contains(id));

        self.thread_last_accessed
            .retain(|id, _| current_session_ids.contains(id));
        self.thread_last_message_sent_or_queued
            .retain(|id, _| current_session_ids.contains(id));

        self.contents = SidebarContents {
            entries,
            notified_threads,
            project_header_indices,
            has_open_projects,
        };
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
                cx,
            ),
            ListEntry::Thread(thread) => self.render_thread(ix, thread, is_active, is_selected, cx),
            ListEntry::ViewMore {
                key,
                is_fully_expanded,
            } => self.render_view_more(ix, key.path_list(), *is_fully_expanded, is_selected, cx),
            ListEntry::DraftThread { worktrees, .. } => {
                self.render_draft_thread(ix, is_active, worktrees, is_selected, cx)
            }
            ListEntry::NewThread {
                key,
                worktrees,
                workspace,
            } => self.render_new_thread(ix, key, worktrees, workspace.as_ref(), is_selected, cx),
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
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let path_list = key.path_list();
        let host = key.host();

        let id_prefix = if is_sticky { "sticky-" } else { "" };
        let id = SharedString::from(format!("{id_prefix}project-header-{ix}"));
        let disclosure_id = SharedString::from(format!("disclosure-{ix}"));
        let group_name = SharedString::from(format!("{id_prefix}header-group-{ix}"));

        let is_collapsed = self.collapsed_groups.contains(path_list);
        let (disclosure_icon, disclosure_tooltip) = if is_collapsed {
            (IconName::ChevronRight, "Expand Project")
        } else {
            (IconName::ChevronDown, "Collapse Project")
        };

        let has_new_thread_entry = self.contents.entries.get(ix + 1).is_some_and(|entry| {
            matches!(
                entry,
                ListEntry::NewThread { .. } | ListEntry::DraftThread { .. }
            )
        });
        let show_new_thread_button = !has_new_thread_entry && !self.has_filter_query(cx);

        let workspace = self.workspace_for_group(path_list, cx);

        let path_list_for_toggle = path_list.clone();
        let path_list_for_collapse = path_list.clone();
        let view_more_expanded = self.expanded_groups.contains_key(path_list);

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
        let hover_color = color
            .element_active
            .blend(color.element_background.opacity(0.2));

        h_flex()
            .id(id)
            .group(&group_name)
            .h(Tab::content_height(cx))
            .w_full()
            .pl(px(5.))
            .pr_1p5()
            .border_1()
            .map(|this| {
                if is_focused {
                    this.border_color(color.border_focused)
                } else {
                    this.border_color(gpui::transparent_black())
                }
            })
            .justify_between()
            .child(
                h_flex()
                    .cursor_pointer()
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
                                this.toggle_collapse(&path_list_for_toggle, window, cx);
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
            .child(
                h_flex()
                    .when(self.project_header_menu_ix != Some(ix), |this| {
                        this.visible_on_hover(group_name)
                    })
                    .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    })
                    .child(self.render_project_header_menu(ix, id_prefix, key, cx))
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
                                let path_list_for_collapse = path_list_for_collapse.clone();
                                move |this, _, _window, cx| {
                                    this.selection = None;
                                    this.expanded_groups.remove(&path_list_for_collapse);
                                    this.serialize(cx);
                                    this.update_entries(cx);
                                }
                            })),
                        )
                    })
                    .when_some(
                        workspace.filter(|_| show_new_thread_button),
                        |this, workspace| {
                            let path_list = path_list.clone();
                            this.child(
                                IconButton::new(
                                    SharedString::from(format!(
                                        "{id_prefix}project-header-new-thread-{ix}",
                                    )),
                                    IconName::Plus,
                                )
                                .icon_size(IconSize::Small)
                                .tooltip(Tooltip::text("New Thread"))
                                .on_click(cx.listener(
                                    move |this, _, window, cx| {
                                        this.collapsed_groups.remove(&path_list);
                                        this.selection = None;
                                        this.create_new_thread(&workspace, window, cx);
                                    },
                                )),
                            )
                        },
                    ),
            )
            .map(|this| {
                let path_list = path_list.clone();
                this.cursor_pointer()
                    .when(!is_active, |this| this.hover(|s| s.bg(hover_color)))
                    .tooltip(Tooltip::text("Open Workspace"))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        if let Some(workspace) = this.workspace_for_group(&path_list, cx) {
                            this.active_entry = Some(ActiveEntry::Draft(workspace.clone()));
                            if let Some(multi_workspace) = this.multi_workspace.upgrade() {
                                multi_workspace.update(cx, |multi_workspace, cx| {
                                    multi_workspace.activate(workspace.clone(), window, cx);
                                });
                            }
                            if AgentPanel::is_visible(&workspace, cx) {
                                workspace.update(cx, |workspace, cx| {
                                    workspace.focus_panel::<AgentPanel>(window, cx);
                                });
                            }
                        } else {
                            this.open_workspace_for_group(&path_list, window, cx);
                        }
                    }))
            })
            .into_any_element()
    }

    fn render_project_header_menu(
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

                let menu = ContextMenu::build_persistent(window, cx, move |menu, _window, _cx| {
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
                            },
                        );
                    }

                    let menu = menu.separator().entry(
                        "Add Folder to Project",
                        Some(Box::new(AddFolderToProject)),
                        {
                            let project_group_key = project_group_key.clone();
                            let multi_workspace = multi_workspace.clone();
                            move |window, cx| {
                                multi_workspace
                                    .update(cx, |multi_workspace, cx| {
                                        multi_workspace.prompt_to_add_folders_to_project_group(
                                            &project_group_key,
                                            window,
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                        },
                    );

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
        path_list: &PathList,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.collapsed_groups.contains(path_list) {
            self.collapsed_groups.remove(path_list);
        } else {
            self.collapsed_groups.insert(path_list.clone());
        }
        self.serialize(cx);
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
                let path_list = key.path_list().clone();
                self.toggle_collapse(&path_list, window, cx);
            }
            ListEntry::Thread(thread) => {
                let metadata = thread.metadata.clone();
                match &thread.workspace {
                    ThreadEntryWorkspace::Open(workspace) => {
                        let workspace = workspace.clone();
                        self.activate_thread(metadata, &workspace, false, window, cx);
                    }
                    ThreadEntryWorkspace::Closed(path_list) => {
                        self.open_workspace_and_activate_thread(
                            metadata,
                            path_list.clone(),
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
                let path_list = key.path_list().clone();
                if *is_fully_expanded {
                    self.reset_thread_group_expansion(&path_list, cx);
                } else {
                    self.expand_thread_group(&path_list, cx);
                }
            }
            ListEntry::DraftThread { .. } => {
                // Already active — nothing to do.
            }
            ListEntry::NewThread { key, workspace, .. } => {
                let path_list = key.path_list().clone();
                if let Some(workspace) = workspace
                    .clone()
                    .or_else(|| self.workspace_for_group(&path_list, cx))
                {
                    self.create_new_thread(&workspace, window, cx);
                } else {
                    self.open_workspace_for_group(&path_list, window, cx);
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
        workspace.update(cx, |workspace, cx| {
            workspace.reveal_panel::<AgentPanel>(window, cx);
        });

        if let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
            agent_panel.update(cx, |panel, cx| {
                panel.load_agent_thread(
                    Agent::from(metadata.agent_id.clone()),
                    metadata.session_id.clone(),
                    Some(metadata.folder_paths.clone()),
                    Some(metadata.title.clone()),
                    focus,
                    window,
                    cx,
                );
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
        self.active_entry = Some(ActiveEntry::Thread {
            session_id: metadata.session_id.clone(),
            workspace: workspace.clone(),
        });
        self.record_thread_access(&metadata.session_id);

        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.activate(workspace.clone(), window, cx);
            if retain {
                multi_workspace.retain_active_workspace(cx);
            }
        });

        Self::load_agent_thread_in_workspace(workspace, metadata, true, window, cx);

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
                    sidebar.active_entry = Some(ActiveEntry::Thread {
                        session_id: target_session_id.clone(),
                        workspace: workspace_for_entry.clone(),
                    });
                    sidebar.record_thread_access(&target_session_id);
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
        path_list: PathList,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let open_task = multi_workspace.update(cx, |this, cx| {
            this.find_or_create_local_workspace(path_list, window, cx)
        });

        cx.spawn_in(window, async move |this, cx| {
            let workspace = open_task.await?;
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
        let session_id = metadata.session_id.clone();

        ThreadMetadataStore::global(cx).update(cx, |store, cx| store.unarchive(&session_id, cx));

        if metadata.folder_paths.paths().is_empty() {
            let active_workspace = self
                .multi_workspace
                .upgrade()
                .map(|w| w.read(cx).workspace().clone());

            if let Some(workspace) = active_workspace {
                self.activate_thread_locally(&metadata, &workspace, false, window, cx);
            }
            return;
        }

        let store = ThreadMetadataStore::global(cx);
        let task = store
            .read(cx)
            .get_archived_worktrees_for_thread(session_id.0.to_string(), cx);
        let path_list = metadata.folder_paths.clone();

        cx.spawn_in(window, async move |this, cx| {
            let archived_worktrees = task.await?;

            // No archived worktrees means the thread wasn't associated with a
            // linked worktree that got deleted, so we just need to find (or
            // open) a workspace that matches the thread's folder paths.
            if archived_worktrees.is_empty() {
                this.update_in(cx, |this, window, cx| {
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
                        this.open_workspace_and_activate_thread(metadata, path_list, window, cx);
                    }
                })?;
                return anyhow::Ok(());
            }

            // Restore each archived worktree back to disk via git. If the
            // worktree already exists (e.g. a previous unarchive of a different
            // thread on the same worktree already restored it), it's reused
            // as-is. We track (old_path, restored_path) pairs so we can update
            // the thread's folder_paths afterward.
            let mut path_replacements: Vec<(PathBuf, PathBuf)> = Vec::new();
            for row in &archived_worktrees {
                match thread_worktree_archive::restore_worktree_via_git(row, &mut *cx).await {
                    Ok(restored_path) => {
                        // The worktree is on disk now; clean up the DB record
                        // and git ref we created during archival.
                        thread_worktree_archive::cleanup_archived_worktree_record(row, &mut *cx)
                            .await;
                        path_replacements.push((row.worktree_path.clone(), restored_path));
                    }
                    Err(error) => {
                        log::error!("Failed to restore worktree: {error:#}");
                        this.update_in(cx, |this, _window, cx| {
                            if let Some(multi_workspace) = this.multi_workspace.upgrade() {
                                let workspace = multi_workspace.read(cx).workspace().clone();
                                workspace.update(cx, |workspace, cx| {
                                    struct RestoreWorktreeErrorToast;
                                    workspace.show_toast(
                                        Toast::new(
                                            NotificationId::unique::<RestoreWorktreeErrorToast>(),
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
                // Update the thread's stored folder_paths: swap each old
                // worktree path for the restored path (which may differ if
                // the worktree was restored to a new location).
                cx.update(|_window, cx| {
                    store.update(cx, |store, cx| {
                        store.update_restored_worktree_paths(&session_id, &path_replacements, cx);
                    });
                })?;

                // Re-read the metadata (now with updated paths) and open
                // the workspace so the user lands in the restored worktree.
                let updated_metadata =
                    cx.update(|_window, cx| store.read(cx).entry(&session_id).cloned())?;

                if let Some(updated_metadata) = updated_metadata {
                    let new_paths = updated_metadata.folder_paths.clone();
                    this.update_in(cx, |this, window, cx| {
                        this.open_workspace_and_activate_thread(
                            updated_metadata,
                            new_paths,
                            window,
                            cx,
                        );
                    })?;
                }
            }

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
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
                if self.collapsed_groups.contains(key.path_list()) {
                    let path_list = key.path_list().clone();
                    self.collapsed_groups.remove(&path_list);
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
                if !self.collapsed_groups.contains(key.path_list()) {
                    self.collapsed_groups.insert(key.path_list().clone());
                    self.update_entries(cx);
                }
            }
            Some(
                ListEntry::Thread(_)
                | ListEntry::ViewMore { .. }
                | ListEntry::NewThread { .. }
                | ListEntry::DraftThread { .. },
            ) => {
                for i in (0..ix).rev() {
                    if let Some(ListEntry::ProjectHeader { key, .. }) = self.contents.entries.get(i)
                    {
                        self.selection = Some(i);
                        self.collapsed_groups.insert(key.path_list().clone());
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
            Some(
                ListEntry::Thread(_)
                | ListEntry::ViewMore { .. }
                | ListEntry::NewThread { .. }
                | ListEntry::DraftThread { .. },
            ) => (0..ix).rev().find(|&i| {
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
                let path_list = key.path_list();
                if self.collapsed_groups.contains(path_list) {
                    self.collapsed_groups.remove(path_list);
                } else {
                    self.selection = Some(header_ix);
                    self.collapsed_groups.insert(path_list.clone());
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
        for entry in &self.contents.entries {
            if let ListEntry::ProjectHeader { key, .. } = entry {
                self.collapsed_groups.insert(key.path_list().clone());
            }
        }
        self.update_entries(cx);
    }

    fn unfold_all(
        &mut self,
        _: &editor::actions::UnfoldAll,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.collapsed_groups.clear();
        self.update_entries(cx);
    }

    fn stop_thread(&mut self, session_id: &acp::SessionId, cx: &mut Context<Self>) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let workspaces: Vec<_> = multi_workspace.read(cx).workspaces().cloned().collect();
        for workspace in workspaces {
            if let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                let cancelled =
                    agent_panel.update(cx, |panel, cx| panel.cancel_thread(session_id, cx));
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
        let metadata = ThreadMetadataStore::global(cx)
            .read(cx)
            .entry(session_id)
            .cloned();
        let thread_folder_paths = metadata.as_ref().map(|m| m.folder_paths.clone());

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
                    .folder_paths
                    .ordered_paths()
                    .filter_map(|path| {
                        thread_worktree_archive::build_root_plan(path, &workspaces, cx)
                    })
                    .filter(|plan| {
                        !thread_worktree_archive::path_is_referenced_by_other_unarchived_threads(
                            session_id,
                            &plan.root_path,
                            cx,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        // Find the neighbor thread in the sidebar (by display position).
        // Look below first, then above, for the nearest thread that isn't
        // the one being archived. We capture both the neighbor's metadata
        // (for activation) and its workspace paths (for the workspace
        // removal fallback).
        let current_pos = self.contents.entries.iter().position(
            |entry| matches!(entry, ListEntry::Thread(t) if &t.metadata.session_id == session_id),
        );
        let neighbor = current_pos.and_then(|pos| {
            self.contents.entries[pos + 1..]
                .iter()
                .chain(self.contents.entries[..pos].iter().rev())
                .find_map(|entry| match entry {
                    ListEntry::Thread(t) if t.metadata.session_id != *session_id => {
                        let workspace_paths = match &t.workspace {
                            ThreadEntryWorkspace::Open(ws) => {
                                PathList::new(&ws.read(cx).root_paths(cx))
                            }
                            ThreadEntryWorkspace::Closed(paths) => paths.clone(),
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
                .filter(|t| t.session_id != *session_id)
                .count();
            if remaining > 0 {
                return None;
            }

            let multi_workspace = self.multi_workspace.upgrade()?;
            let workspace = multi_workspace
                .read(cx)
                .workspace_for_paths(folder_paths, cx)?;

            // Don't remove the main worktree workspace — the project
            // header always provides access to it.
            let group_key = workspace.read(cx).project_group_key(cx);
            (group_key.path_list() != folder_paths).then_some(workspace)
        });

        if let Some(workspace_to_remove) = workspace_to_remove {
            let multi_workspace = self.multi_workspace.upgrade().unwrap();
            let session_id = session_id.clone();

            // For the workspace-removal fallback, use the neighbor's workspace
            // paths if available, otherwise fall back to the project group key.
            let fallback_paths = neighbor
                .as_ref()
                .map(|(_, paths)| paths.clone())
                .unwrap_or_else(|| {
                    workspace_to_remove
                        .read(cx)
                        .project_group_key(cx)
                        .path_list()
                        .clone()
                });

            let remove_task = multi_workspace.update(cx, |mw, cx| {
                mw.remove(
                    [workspace_to_remove],
                    move |this, window, cx| {
                        this.find_or_create_local_workspace(fallback_paths, window, cx)
                    },
                    window,
                    cx,
                )
            });

            let neighbor_metadata = neighbor.map(|(metadata, _)| metadata);
            let thread_folder_paths = thread_folder_paths.clone();
            cx.spawn_in(window, async move |this, cx| {
                let removed = remove_task.await?;
                if removed {
                    this.update_in(cx, |this, window, cx| {
                        let in_flight =
                            this.start_archive_worktree_task(&session_id, roots_to_archive, cx);
                        this.archive_and_activate(
                            &session_id,
                            neighbor_metadata.as_ref(),
                            thread_folder_paths.as_ref(),
                            in_flight,
                            window,
                            cx,
                        );
                    })?;
                }
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        } else {
            // Simple case: no workspace removal needed.
            let neighbor_metadata = neighbor.map(|(metadata, _)| metadata);
            let in_flight = self.start_archive_worktree_task(session_id, roots_to_archive, cx);
            self.archive_and_activate(
                session_id,
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
        session_id: &acp::SessionId,
        neighbor: Option<&ThreadMetadata>,
        thread_folder_paths: Option<&PathList>,
        in_flight_archive: Option<(Task<()>, smol::channel::Sender<()>)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        ThreadMetadataStore::global(cx).update(cx, |store, cx| {
            store.archive(session_id, in_flight_archive, cx);
        });

        let is_active = self
            .active_entry
            .as_ref()
            .is_some_and(|e| e.is_active_thread(session_id));

        if !is_active {
            // The user is looking at a different thread/draft. Clear the
            // archived thread from its workspace's panel so that switching
            // to that workspace later doesn't show a stale thread.
            if let Some(folder_paths) = thread_folder_paths {
                if let Some(workspace) = self
                    .multi_workspace
                    .upgrade()
                    .and_then(|mw| mw.read(cx).workspace_for_paths(folder_paths, cx))
                {
                    if let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                        let panel_shows_archived = panel
                            .read(cx)
                            .active_conversation_view()
                            .and_then(|cv| cv.read(cx).parent_id(cx))
                            .is_some_and(|id| id == *session_id);
                        if panel_shows_archived {
                            panel.update(cx, |panel, cx| {
                                panel.clear_active_thread(window, cx);
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
            if let Some(workspace) = self
                .multi_workspace
                .upgrade()
                .and_then(|mw| mw.read(cx).workspace_for_paths(&metadata.folder_paths, cx))
            {
                self.activate_workspace(&workspace, window, cx);
                Self::load_agent_thread_in_workspace(&workspace, metadata, true, window, cx);
                return;
            }
        }

        // No neighbor or its workspace isn't open — fall back to a new
        // draft. Use the group workspace (main project) rather than the
        // active entry workspace, which may be a linked worktree that is
        // about to be cleaned up.
        let fallback_workspace = thread_folder_paths
            .and_then(|folder_paths| {
                let mw = self.multi_workspace.upgrade()?;
                let mw = mw.read(cx);
                // Find the group's main workspace (whose root paths match
                // the project group key, not the thread's folder paths).
                let thread_workspace = mw.workspace_for_paths(folder_paths, cx)?;
                let group_key = thread_workspace.read(cx).project_group_key(cx);
                mw.workspace_for_paths(group_key.path_list(), cx)
            })
            .or_else(|| self.active_entry_workspace().cloned());

        if let Some(workspace) = fallback_workspace {
            self.activate_workspace(&workspace, window, cx);
            if let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                panel.update(cx, |panel, cx| {
                    panel.new_thread(&NewThread, window, cx);
                });
            }
        }
    }

    fn start_archive_worktree_task(
        &self,
        session_id: &acp::SessionId,
        roots: Vec<thread_worktree_archive::RootPlan>,
        cx: &mut Context<Self>,
    ) -> Option<(Task<()>, smol::channel::Sender<()>)> {
        if roots.is_empty() {
            return None;
        }

        let (cancel_tx, cancel_rx) = smol::channel::bounded::<()>(1);
        let session_id = session_id.clone();
        let task = cx.spawn(async move |_this, cx| {
            match Self::archive_worktree_roots(roots, cancel_rx, cx).await {
                Ok(ArchiveWorktreeOutcome::Success) => {
                    cx.update(|cx| {
                        ThreadMetadataStore::global(cx).update(cx, |store, _cx| {
                            store.cleanup_completed_archive(&session_id);
                        });
                    });
                }
                Ok(ArchiveWorktreeOutcome::Cancelled) => {}
                Err(error) => {
                    log::error!("Failed to archive worktree: {error:#}");
                    cx.update(|cx| {
                        ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                            store.unarchive(&session_id, cx);
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
        let mut completed_persists: Vec<(
            thread_worktree_archive::PersistOutcome,
            thread_worktree_archive::RootPlan,
        )> = Vec::new();

        for root in &roots {
            if cancel_rx.is_closed() {
                for (outcome, completed_root) in completed_persists.iter().rev() {
                    thread_worktree_archive::rollback_persist(outcome, completed_root, cx).await;
                }
                return Ok(ArchiveWorktreeOutcome::Cancelled);
            }

            if root.worktree_repo.is_some() {
                match thread_worktree_archive::persist_worktree_state(root, cx).await {
                    Ok(outcome) => {
                        completed_persists.push((outcome, root.clone()));
                    }
                    Err(error) => {
                        for (outcome, completed_root) in completed_persists.iter().rev() {
                            thread_worktree_archive::rollback_persist(outcome, completed_root, cx)
                                .await;
                        }
                        return Err(error);
                    }
                }
            }

            if cancel_rx.is_closed() {
                for (outcome, completed_root) in completed_persists.iter().rev() {
                    thread_worktree_archive::rollback_persist(outcome, completed_root, cx).await;
                }
                return Ok(ArchiveWorktreeOutcome::Cancelled);
            }

            if let Err(error) = thread_worktree_archive::remove_root(root.clone(), cx).await {
                if let Some((outcome, completed_root)) = completed_persists.last() {
                    if completed_root.root_path == root.root_path {
                        thread_worktree_archive::rollback_persist(outcome, completed_root, cx)
                            .await;
                        completed_persists.pop();
                    }
                }
                for (outcome, completed_root) in completed_persists.iter().rev() {
                    thread_worktree_archive::rollback_persist(outcome, completed_root, cx).await;
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
            Some(ListEntry::Thread(thread)) => {
                match thread.status {
                    AgentThreadStatus::Running | AgentThreadStatus::WaitingForConfirmation => {
                        return;
                    }
                    AgentThreadStatus::Completed | AgentThreadStatus::Error => {}
                }
                let session_id = thread.metadata.session_id.clone();
                self.archive_thread(&session_id, window, cx);
            }
            Some(ListEntry::NewThread {
                workspace: Some(workspace),
                ..
            }) => {
                self.remove_worktree_workspace(workspace.clone(), window, cx);
            }
            _ => {}
        }
    }

    fn remove_worktree_workspace(
        &mut self,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(multi_workspace) = self.multi_workspace.upgrade() {
            multi_workspace
                .update(cx, |mw, cx| {
                    mw.remove(
                        [workspace],
                        |this, _window, _cx| gpui::Task::ready(Ok(this.workspace().clone())),
                        window,
                        cx,
                    )
                })
                .detach_and_log_err(cx);
        }
    }

    fn record_thread_access(&mut self, session_id: &acp::SessionId) {
        self.thread_last_accessed
            .insert(session_id.clone(), Utc::now());
    }

    fn record_thread_message_sent(&mut self, session_id: &acp::SessionId) {
        self.thread_last_message_sent_or_queued
            .insert(session_id.clone(), Utc::now());
    }

    fn mru_threads_for_switcher(&self, cx: &App) -> Vec<ThreadSwitcherEntry> {
        let mut current_header_label: Option<SharedString> = None;
        let mut current_header_path_list: Option<PathList> = None;
        let mut entries: Vec<ThreadSwitcherEntry> = self
            .contents
            .entries
            .iter()
            .filter_map(|entry| match entry {
                ListEntry::ProjectHeader { label, key, .. } => {
                    current_header_label = Some(label.clone());
                    current_header_path_list = Some(key.path_list().clone());
                    None
                }
                ListEntry::Thread(thread) => {
                    let workspace = match &thread.workspace {
                        ThreadEntryWorkspace::Open(workspace) => Some(workspace.clone()),
                        ThreadEntryWorkspace::Closed(_) => current_header_path_list
                            .as_ref()
                            .and_then(|pl| self.workspace_for_group(pl, cx)),
                    }?;
                    let notified = self
                        .contents
                        .is_thread_notified(&thread.metadata.session_id);
                    let timestamp: SharedString = format_history_entry_timestamp(
                        self.thread_last_message_sent_or_queued
                            .get(&thread.metadata.session_id)
                            .copied()
                            .or(thread.metadata.created_at)
                            .unwrap_or(thread.metadata.updated_at),
                    )
                    .into();
                    Some(ThreadSwitcherEntry {
                        session_id: thread.metadata.session_id.clone(),
                        title: thread.metadata.title.clone(),
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
                    let a_sent = self.thread_last_message_sent_or_queued.get(&a.session_id);
                    let b_sent = self.thread_last_message_sent_or_queued.get(&b.session_id);

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
            Some(ActiveEntry::Thread { session_id, .. }) => entries
                .iter()
                .find(|e| &e.session_id == session_id)
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
                    this.active_entry = Some(ActiveEntry::Thread {
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
                    this.active_entry = Some(ActiveEntry::Thread {
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
                            this.active_entry = Some(ActiveEntry::Thread {
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
            self.active_entry = Some(ActiveEntry::Thread {
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
        let has_notification = self
            .contents
            .is_thread_notified(&thread.metadata.session_id);

        let title: SharedString = thread.metadata.title.clone();
        let metadata = thread.metadata.clone();
        let thread_workspace = thread.workspace.clone();

        let is_hovered = self.hovered_thread_index == Some(ix);
        let is_selected = is_active;
        let is_running = matches!(
            thread.status,
            AgentThreadStatus::Running | AgentThreadStatus::WaitingForConfirmation
        );

        let session_id_for_delete = thread.metadata.session_id.clone();
        let focus_handle = self.focus_handle.clone();

        let id = SharedString::from(format!("thread-entry-{}", ix));

        let color = cx.theme().colors();
        let sidebar_bg = color
            .title_bar_background
            .blend(color.panel_background.opacity(0.25));

        let timestamp = format_history_entry_timestamp(
            self.thread_last_message_sent_or_queued
                .get(&thread.metadata.session_id)
                .copied()
                .or(thread.metadata.created_at)
                .unwrap_or(thread.metadata.updated_at),
        );

        ThreadItem::new(id, title)
            .base_bg(sidebar_bg)
            .icon(thread.icon)
            .status(thread.status)
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
                            let session_id = session_id_for_delete.clone();
                            cx.listener(move |this, _, _window, cx| {
                                this.stop_thread(&session_id, cx);
                            })
                        }),
                )
            })
            .when(is_hovered && !is_running, |this| {
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
                                this.archive_thread(&session_id, window, cx);
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
                        ThreadEntryWorkspace::Closed(path_list) => {
                            this.open_workspace_and_activate_thread(
                                metadata.clone(),
                                path_list.clone(),
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
            .map(|mw| mw.read(cx).project_group_keys().cloned().collect())
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
        path_list: &PathList,
        is_fully_expanded: bool,
        is_selected: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let path_list = path_list.clone();
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
                    this.reset_thread_group_expansion(&path_list, cx);
                } else {
                    this.expand_thread_group(&path_list, cx);
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
        let workspace = if let Some(selected_ix) = self.selection {
            self.contents
                .project_header_indices
                .iter()
                .rev()
                .find(|&&header_ix| header_ix <= selected_ix)
                .and_then(|&header_ix| match &self.contents.entries[header_ix] {
                    ListEntry::ProjectHeader { key, .. } => {
                        self.workspace_for_group(key.path_list(), cx)
                    }
                    _ => None,
                })
        } else {
            // Use the currently active workspace.
            self.multi_workspace
                .upgrade()
                .map(|mw| mw.read(cx).workspace().clone())
        };

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

        self.active_entry = Some(ActiveEntry::Draft(workspace.clone()));

        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.activate(workspace.clone(), window, cx);
        });

        workspace.update(cx, |workspace, cx| {
            if let Some(agent_panel) = workspace.panel::<AgentPanel>(cx) {
                agent_panel.update(cx, |panel, cx| {
                    panel.new_thread(&NewThread, window, cx);
                });
            }
            workspace.focus_panel::<AgentPanel>(window, cx);
        });
    }

    fn active_project_group_key(&self, cx: &App) -> Option<ProjectGroupKey> {
        let multi_workspace = self.multi_workspace.upgrade()?;
        let mw = multi_workspace.read(cx);
        Some(mw.workspace().read(cx).project_group_key(cx))
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
        let path_list = key.path_list().clone();

        // Uncollapse the target group so that threads become visible.
        self.collapsed_groups.remove(&path_list);

        if let Some(workspace) = self.workspace_for_group(&path_list, cx) {
            multi_workspace.update(cx, |multi_workspace, cx| {
                multi_workspace.activate(workspace, window, cx);
                multi_workspace.retain_active_workspace(cx);
            });
        } else {
            self.open_workspace_for_group(&path_list, window, cx);
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
            ThreadEntryWorkspace::Closed(path_list) => {
                self.open_workspace_and_activate_thread(metadata, path_list.clone(), window, cx);
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

    fn expand_thread_group(&mut self, path_list: &PathList, cx: &mut Context<Self>) {
        let current = self.expanded_groups.get(path_list).copied().unwrap_or(0);
        self.expanded_groups.insert(path_list.clone(), current + 1);
        self.serialize(cx);
        self.update_entries(cx);
    }

    fn reset_thread_group_expansion(&mut self, path_list: &PathList, cx: &mut Context<Self>) {
        self.expanded_groups.remove(path_list);
        self.serialize(cx);
        self.update_entries(cx);
    }

    fn collapse_thread_group(&mut self, path_list: &PathList, cx: &mut Context<Self>) {
        match self.expanded_groups.get(path_list).copied() {
            Some(batches) if batches > 1 => {
                self.expanded_groups.insert(path_list.clone(), batches - 1);
            }
            Some(_) => {
                self.expanded_groups.remove(path_list);
            }
            None => return,
        }
        self.serialize(cx);
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
        self.expand_thread_group(active_key.path_list(), cx);
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
        self.collapse_thread_group(active_key.path_list(), cx);
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

    fn render_draft_thread(
        &self,
        ix: usize,
        is_active: bool,
        worktrees: &[WorktreeInfo],
        is_selected: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let label: SharedString = if is_active {
            self.active_draft_text(cx)
                .unwrap_or_else(|| "Untitled Thread".into())
        } else {
            "Untitled Thread".into()
        };

        let id = SharedString::from(format!("draft-thread-btn-{}", ix));

        let thread_item = ThreadItem::new(id, label)
            .icon(IconName::Plus)
            .icon_color(Color::Custom(cx.theme().colors().icon_muted.opacity(0.8)))
            .worktrees(
                worktrees
                    .iter()
                    .map(|wt| ThreadItemWorktreeInfo {
                        name: wt.name.clone(),
                        full_path: wt.full_path.clone(),
                        highlight_positions: wt.highlight_positions.clone(),
                        kind: wt.kind,
                    })
                    .collect(),
            )
            .selected(true)
            .focused(is_selected);

        div()
            .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
            .child(thread_item)
            .into_any_element()
    }

    fn render_new_thread(
        &self,
        ix: usize,
        key: &ProjectGroupKey,
        worktrees: &[WorktreeInfo],
        workspace: Option<&Entity<Workspace>>,
        is_selected: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let label: SharedString = DEFAULT_THREAD_TITLE.into();
        let path_list = key.path_list().clone();

        let id = SharedString::from(format!("new-thread-btn-{}", ix));

        let mut thread_item = ThreadItem::new(id, label)
            .icon(IconName::Plus)
            .icon_color(Color::Custom(cx.theme().colors().icon_muted.opacity(0.8)))
            .worktrees(
                worktrees
                    .iter()
                    .map(|wt| ThreadItemWorktreeInfo {
                        name: wt.name.clone(),
                        full_path: wt.full_path.clone(),
                        highlight_positions: wt.highlight_positions.clone(),
                        kind: wt.kind,
                    })
                    .collect(),
            )
            .selected(false)
            .focused(is_selected)
            .on_click(cx.listener(move |this, _, window, cx| {
                this.selection = None;
                if let Some(workspace) = this.workspace_for_group(&path_list, cx) {
                    this.create_new_thread(&workspace, window, cx);
                } else {
                    this.open_workspace_for_group(&path_list, window, cx);
                }
            }));

        // Linked worktree NewThread entries can be dismissed, which removes
        // the workspace from the multi-workspace.
        if let Some(workspace) = workspace.cloned() {
            thread_item = thread_item.action_slot(
                IconButton::new("close-worktree-workspace", IconName::Close)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted)
                    .tooltip(Tooltip::text("Close Workspace"))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.remove_worktree_workspace(workspace.clone(), window, cx);
                    })),
            );
        }

        thread_item.into_any_element()
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
        let description =
            "Import threads from your ACP agents — whether started in Zed or another client.";

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
                    .child(Label::new("Looking for ACP threads?"))
                    .child(
                        IconButton::new("close-onboarding", IconName::Close)
                            .icon_size(IconSize::Small)
                            .on_click(|_, _window, cx| AcpThreadImportOnboarding::dismiss(cx)),
                    ),
            )
            .child(Label::new(description).color(Color::Muted).mb_2())
            .child(
                Button::new("import-acp", "Import ACP Threads")
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
                    this.show_thread_list(window, cx);
                    this.activate_archived_thread(thread.clone(), window, cx);
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
            collapsed_groups: self
                .collapsed_groups
                .iter()
                .map(|pl| pl.serialize())
                .collect(),
            expanded_groups: self
                .expanded_groups
                .iter()
                .map(|(pl, count)| (pl.serialize(), *count))
                .collect(),
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
            self.collapsed_groups = serialized
                .collapsed_groups
                .into_iter()
                .map(|s| PathList::deserialize(&s))
                .collect();
            self.expanded_groups = serialized
                .expanded_groups
                .into_iter()
                .map(|(s, count)| (PathList::deserialize(&s), count))
                .collect();
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
            let is_background = agent_panel.is_background_thread(&session_id);

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
        let keys: Vec<_> = mw.read(cx).project_group_keys().cloned().collect();
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

        let background_threads = panel.background_threads();
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
                    write!(output, "  - {title} (session: {session_id})").ok();
                    write!(output, " [{status}, {entry_count} entries").ok();
                    if conversation_view
                        .read(cx)
                        .root_thread_has_pending_tool_call(cx)
                    {
                        write!(output, ", awaiting confirmation").ok();
                    }
                    writeln!(output, "]").ok();
                } else {
                    writeln!(output, "  - (not connected) (session: {session_id})").ok();
                }
            }
        }
    } else {
        writeln!(output, "Agent panel: not loaded").ok();
    }

    writeln!(output).ok();
}
