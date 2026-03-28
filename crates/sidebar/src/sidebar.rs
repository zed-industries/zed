use acp_thread::ThreadStatus;
use action_log::DiffStats;
use agent_client_protocol::{self as acp};
use agent_settings::AgentSettings;
use agent_ui::thread_metadata_store::{SidebarThreadMetadataStore, ThreadMetadata};
use agent_ui::threads_archive_view::{
    ThreadsArchiveView, ThreadsArchiveViewEvent, format_history_entry_timestamp,
};
use agent_ui::{
    Agent, AgentPanel, AgentPanelEvent, DEFAULT_THREAD_TITLE, NewThread, RemoveSelectedThread,
};
use chrono::Utc;
use editor::Editor;
use feature_flags::{AgentV2FeatureFlag, FeatureFlagViewExt as _};
use gpui::{
    Action as _, AnyElement, App, Context, Entity, FocusHandle, Focusable, KeyContext, ListState,
    Pixels, Render, SharedString, WeakEntity, Window, WindowHandle, list, prelude::*, px,
};
use menu::{
    Cancel, Confirm, SelectChild, SelectFirst, SelectLast, SelectNext, SelectParent, SelectPrevious,
};
use project::{Event as ProjectEvent, linked_worktree_short_name};
use recent_projects::sidebar_recent_projects::SidebarRecentProjects;
use ui::utils::platform_title_bar_height;

use settings::Settings as _;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::rc::Rc;
use theme::ActiveTheme;
use ui::{
    AgentThreadStatus, CommonAnimationExt, ContextMenu, Divider, HighlightedLabel, KeyBinding,
    PopoverMenu, PopoverMenuHandle, Tab, ThreadItem, ThreadItemWorktreeInfo, TintColor, Tooltip,
    WithScrollbar, prelude::*,
};
use util::ResultExt as _;
use util::path_list::PathList;
use workspace::{
    AddFolderToProject, FocusWorkspaceSidebar, MultiWorkspace, MultiWorkspaceEvent, Open,
    Sidebar as WorkspaceSidebar, SidebarSide, ToggleWorkspaceSidebar, Workspace, WorkspaceId,
    sidebar_side_context_menu,
};

use zed_actions::OpenRecent;
use zed_actions::editor::{MoveDown, MoveUp};

use zed_actions::agents_sidebar::FocusSidebarFilter;

use crate::project_group_builder::ProjectGroupBuilder;

mod project_group_builder;

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

const DEFAULT_WIDTH: Pixels = px(300.0);
const MIN_WIDTH: Pixels = px(200.0);
const MAX_WIDTH: Pixels = px(800.0);
const DEFAULT_THREADS_SHOWN: usize = 5;

#[derive(Debug, Default)]
enum SidebarView {
    #[default]
    ThreadList,
    Archive(Entity<ThreadsArchiveView>),
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

impl From<&ActiveThreadInfo> for acp_thread::AgentSessionInfo {
    fn from(info: &ActiveThreadInfo) -> Self {
        Self {
            session_id: info.session_id.clone(),
            work_dirs: None,
            title: Some(info.title.clone()),
            updated_at: Some(Utc::now()),
            created_at: Some(Utc::now()),
            meta: None,
        }
    }
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
}

#[derive(Clone)]
struct ThreadEntry {
    agent: Agent,
    session_info: acp_thread::AgentSessionInfo,
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
        self.session_info.title = Some(info.title.clone());
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
        path_list: PathList,
        label: SharedString,
        workspace: Entity<Workspace>,
        highlight_positions: Vec<usize>,
        has_running_threads: bool,
        waiting_thread_count: usize,
        is_active: bool,
    },
    Thread(ThreadEntry),
    ViewMore {
        path_list: PathList,
        is_fully_expanded: bool,
    },
    NewThread {
        path_list: PathList,
        workspace: Entity<Workspace>,
        is_active_draft: bool,
    },
}

#[cfg(test)]
impl ListEntry {
    fn workspace(&self) -> Option<Entity<Workspace>> {
        match self {
            ListEntry::ProjectHeader { workspace, .. } => Some(workspace.clone()),
            ListEntry::Thread(thread_entry) => match &thread_entry.workspace {
                ThreadEntryWorkspace::Open(workspace) => Some(workspace.clone()),
                ThreadEntryWorkspace::Closed(_) => None,
            },
            ListEntry::ViewMore { .. } => None,
            ListEntry::NewThread { workspace, .. } => Some(workspace.clone()),
        }
    }

    fn session_id(&self) -> Option<&acp::SessionId> {
        match self {
            ListEntry::Thread(thread_entry) => Some(&thread_entry.session_info.session_id),
            _ => None,
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
/// For each path in the thread's `folder_paths` that canonicalizes to a
/// different path (i.e. it's a git worktree), produces a [`WorktreeInfo`]
/// with the short worktree name and full path.
fn worktree_info_from_thread_paths(
    folder_paths: &PathList,
    project_groups: &ProjectGroupBuilder,
) -> Vec<WorktreeInfo> {
    folder_paths
        .paths()
        .iter()
        .filter_map(|path| {
            let canonical = project_groups.canonicalize_path(path);
            if canonical != path.as_path() {
                Some(WorktreeInfo {
                    name: linked_worktree_short_name(canonical, path).unwrap_or_default(),
                    full_path: SharedString::from(path.display().to_string()),
                    highlight_positions: Vec::new(),
                })
            } else {
                None
            }
        })
        .collect()
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
    /// Derived from the active panel's thread in `rebuild_contents`.
    /// Only updated when the panel returns `Some` — never cleared by
    /// derivation, since the panel may transiently return `None` while
    /// loading. User actions may write directly for immediate feedback.
    focused_thread: Option<acp::SessionId>,
    agent_panel_visible: bool,
    active_thread_is_draft: bool,
    hovered_thread_index: Option<usize>,
    collapsed_groups: HashSet<PathList>,
    expanded_groups: HashMap<PathList, usize>,
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

        cx.observe(
            &SidebarThreadMetadataStore::global(cx),
            |this, _store, cx| {
                this.update_entries(cx);
            },
        )
        .detach();

        cx.observe_flag::<AgentV2FeatureFlag, _>(window, |_is_enabled, this, _window, cx| {
            this.update_entries(cx);
        })
        .detach();

        let workspaces = multi_workspace.read(cx).workspaces().to_vec();
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
            focused_thread: None,
            agent_panel_visible: false,
            active_thread_is_draft: false,
            hovered_thread_index: None,
            collapsed_groups: HashSet::new(),
            expanded_groups: HashMap::new(),
            view: SidebarView::default(),
            recent_projects_popover_handle: PopoverMenuHandle::default(),
            project_header_menu_ix: None,
            _subscriptions: Vec::new(),
            _draft_observation: None,
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
            if self.is_active_workspace(workspace, cx) {
                self.agent_panel_visible = AgentPanel::is_visible(workspace, cx);
            }
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
                        this.focused_thread = None;
                    }
                    this.observe_draft_editor(cx);
                    this.update_entries(cx);
                }
                AgentPanelEvent::ThreadFocused | AgentPanelEvent::BackgroundThreadChanged => {
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

                let is_visible = AgentPanel::is_visible(&workspace, cx);

                if this.agent_panel_visible != is_visible {
                    this.agent_panel_visible = is_visible;
                    cx.notify();
                }
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

    /// Rebuilds the sidebar contents from current workspace and thread state.
    ///
    /// Uses [`ProjectGroupBuilder`] to group workspaces by their main git
    /// repository, then populates thread entries from the metadata store and
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
        let workspaces = mw.workspaces().to_vec();
        let active_workspace = mw.workspaces().get(mw.active_workspace_index()).cloned();

        let agent_server_store = workspaces
            .first()
            .map(|ws| ws.read(cx).project().read(cx).agent_server_store().clone());

        let query = self.filter_editor.read(cx).text(cx);

        // Re-derive agent_panel_visible from the active workspace so it stays
        // correct after workspace switches.
        self.agent_panel_visible = active_workspace
            .as_ref()
            .map_or(false, |ws| AgentPanel::is_visible(ws, cx));

        // Derive active_thread_is_draft BEFORE focused_thread so we can
        // use it as a guard below.
        self.active_thread_is_draft = active_workspace
            .as_ref()
            .and_then(|ws| ws.read(cx).panel::<AgentPanel>(cx))
            .map_or(false, |panel| panel.read(cx).active_thread_is_draft(cx));

        // Derive focused_thread from the active workspace's agent panel.
        // Only update when the panel gives us a positive signal — if the
        // panel returns None (e.g. still loading after a thread activation),
        // keep the previous value so eager writes from user actions survive.
        let panel_focused = active_workspace
            .as_ref()
            .and_then(|ws| ws.read(cx).panel::<AgentPanel>(cx))
            .and_then(|panel| {
                panel
                    .read(cx)
                    .active_conversation_view()
                    .and_then(|cv| cv.read(cx).parent_id(cx))
            });
        if panel_focused.is_some() && !self.active_thread_is_draft {
            self.focused_thread = panel_focused;
        }

        let previous = mem::take(&mut self.contents);

        let old_statuses: HashMap<acp::SessionId, AgentThreadStatus> = previous
            .entries
            .iter()
            .filter_map(|entry| match entry {
                ListEntry::Thread(thread) if thread.is_live => {
                    Some((thread.session_info.session_id.clone(), thread.status))
                }
                _ => None,
            })
            .collect();

        let mut entries = Vec::new();
        let mut notified_threads = previous.notified_threads;
        let mut current_session_ids: HashSet<acp::SessionId> = HashSet::new();
        let mut project_header_indices: Vec<usize> = Vec::new();

        // Use ProjectGroupBuilder to canonically group workspaces by their
        // main git repository. This replaces the manual absorbed-workspace
        // detection that was here before.
        let project_groups = ProjectGroupBuilder::from_multiworkspace(mw, cx);

        let has_open_projects = workspaces
            .iter()
            .any(|ws| !workspace_path_list(ws, cx).paths().is_empty());

        let resolve_agent = |row: &ThreadMetadata| -> (Agent, IconName, Option<SharedString>) {
            match &row.agent_id {
                None => (Agent::NativeAgent, IconName::ZedAgent, None),
                Some(id) => {
                    let custom_icon = agent_server_store
                        .as_ref()
                        .and_then(|store| store.read(cx).agent_icon(id));
                    (
                        Agent::Custom { id: id.clone() },
                        IconName::Terminal,
                        custom_icon,
                    )
                }
            }
        };

        for (group_name, group) in project_groups.groups() {
            let path_list = group_name.path_list().clone();
            if path_list.paths().is_empty() {
                continue;
            }

            let label = group_name.display_name();

            let is_collapsed = self.collapsed_groups.contains(&path_list);
            let should_load_threads = !is_collapsed || !query.is_empty();

            let is_active = active_workspace
                .as_ref()
                .is_some_and(|active| group.workspaces.contains(active));

            // Pick a representative workspace for the group: prefer the active
            // workspace if it belongs to this group, otherwise use the main
            // repo workspace (not a linked worktree).
            let representative_workspace = active_workspace
                .as_ref()
                .filter(|_| is_active)
                .unwrap_or_else(|| group.main_workspace(cx));

            // Collect live thread infos from all workspaces in this group.
            let live_infos: Vec<_> = group
                .workspaces
                .iter()
                .flat_map(|ws| all_thread_infos_for_workspace(ws, cx))
                .collect();

            let mut threads: Vec<ThreadEntry> = Vec::new();
            let mut has_running_threads = false;
            let mut waiting_thread_count: usize = 0;

            if should_load_threads {
                let mut seen_session_ids: HashSet<acp::SessionId> = HashSet::new();
                let thread_store = SidebarThreadMetadataStore::global(cx);

                // Load threads from each workspace in the group.
                for workspace in &group.workspaces {
                    let ws_path_list = workspace_path_list(workspace, cx);

                    for row in thread_store.read(cx).entries_for_path(&ws_path_list) {
                        if !seen_session_ids.insert(row.session_id.clone()) {
                            continue;
                        }
                        let (agent, icon, icon_from_external_svg) = resolve_agent(&row);
                        let worktrees =
                            worktree_info_from_thread_paths(&row.folder_paths, &project_groups);
                        threads.push(ThreadEntry {
                            agent,
                            session_info: acp_thread::AgentSessionInfo {
                                session_id: row.session_id.clone(),
                                work_dirs: None,
                                title: Some(row.title.clone()),
                                updated_at: Some(row.updated_at),
                                created_at: row.created_at,
                                meta: None,
                            },
                            icon,
                            icon_from_external_svg,
                            status: AgentThreadStatus::default(),
                            workspace: ThreadEntryWorkspace::Open(workspace.clone()),
                            is_live: false,
                            is_background: false,
                            is_title_generating: false,
                            highlight_positions: Vec::new(),
                            worktrees,
                            diff_stats: DiffStats::default(),
                        });
                    }
                }

                // Load threads from linked git worktrees whose
                // canonical paths belong to this group.
                let linked_worktree_queries = group
                    .workspaces
                    .iter()
                    .flat_map(|ws| root_repository_snapshots(ws, cx))
                    .filter(|snapshot| !snapshot.is_linked_worktree())
                    .flat_map(|snapshot| {
                        snapshot
                            .linked_worktrees()
                            .iter()
                            .filter(|wt| {
                                project_groups.group_owns_worktree(group, &path_list, &wt.path)
                            })
                            .map(|wt| PathList::new(std::slice::from_ref(&wt.path)))
                            .collect::<Vec<_>>()
                    });

                for worktree_path_list in linked_worktree_queries {
                    for row in thread_store.read(cx).entries_for_path(&worktree_path_list) {
                        if !seen_session_ids.insert(row.session_id.clone()) {
                            continue;
                        }
                        let (agent, icon, icon_from_external_svg) = resolve_agent(&row);
                        let worktrees =
                            worktree_info_from_thread_paths(&row.folder_paths, &project_groups);
                        threads.push(ThreadEntry {
                            agent,
                            session_info: acp_thread::AgentSessionInfo {
                                session_id: row.session_id.clone(),
                                work_dirs: None,
                                title: Some(row.title.clone()),
                                updated_at: Some(row.updated_at),
                                created_at: row.created_at,
                                meta: None,
                            },
                            icon,
                            icon_from_external_svg,
                            status: AgentThreadStatus::default(),
                            workspace: ThreadEntryWorkspace::Closed(worktree_path_list.clone()),
                            is_live: false,
                            is_background: false,
                            is_title_generating: false,
                            highlight_positions: Vec::new(),
                            worktrees,
                            diff_stats: DiffStats::default(),
                        });
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
                    if let Some(info) = live_info_by_session.get(&thread.session_info.session_id) {
                        thread.apply_active_info(info);
                    }

                    let session_id = &thread.session_info.session_id;

                    let is_thread_workspace_active = match &thread.workspace {
                        ThreadEntryWorkspace::Open(thread_workspace) => active_workspace
                            .as_ref()
                            .is_some_and(|active| active == thread_workspace),
                        ThreadEntryWorkspace::Closed(_) => false,
                    };

                    if thread.status == AgentThreadStatus::Completed
                        && !is_thread_workspace_active
                        && old_statuses.get(session_id) == Some(&AgentThreadStatus::Running)
                    {
                        notified_threads.insert(session_id.clone());
                    }

                    if is_thread_workspace_active && !thread.is_background {
                        notified_threads.remove(session_id);
                    }
                }

                threads.sort_by(|a, b| {
                    let a_time = a.session_info.created_at.or(a.session_info.updated_at);
                    let b_time = b.session_info.created_at.or(b.session_info.updated_at);
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
                    let title = thread
                        .session_info
                        .title
                        .as_ref()
                        .map(|s| s.as_ref())
                        .unwrap_or("");
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
                    path_list: path_list.clone(),
                    label,
                    workspace: representative_workspace.clone(),
                    highlight_positions: workspace_highlight_positions,
                    has_running_threads,
                    waiting_thread_count,
                    is_active,
                });

                for thread in matched_threads {
                    current_session_ids.insert(thread.session_info.session_id.clone());
                    entries.push(thread.into());
                }
            } else {
                let thread_count = threads.len();
                let is_draft_for_workspace = self.agent_panel_visible
                    && self.active_thread_is_draft
                    && self.focused_thread.is_none()
                    && is_active;

                let show_new_thread_entry = thread_count == 0 || is_draft_for_workspace;

                project_header_indices.push(entries.len());
                entries.push(ListEntry::ProjectHeader {
                    path_list: path_list.clone(),
                    label,
                    workspace: representative_workspace.clone(),
                    highlight_positions: Vec::new(),
                    has_running_threads,
                    waiting_thread_count,
                    is_active,
                });

                if is_collapsed {
                    continue;
                }

                if show_new_thread_entry {
                    entries.push(ListEntry::NewThread {
                        path_list: path_list.clone(),
                        workspace: representative_workspace.clone(),
                        is_active_draft: is_draft_for_workspace,
                    });
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

                    let session_id = &thread.session_info.session_id;
                    if is_hidden {
                        let is_promoted = thread.status == AgentThreadStatus::Running
                            || thread.status == AgentThreadStatus::WaitingForConfirmation
                            || notified_threads.contains(session_id)
                            || self
                                .focused_thread
                                .as_ref()
                                .is_some_and(|id| id == session_id);
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
                        path_list: path_list.clone(),
                        is_fully_expanded,
                    });
                }
            }
        }

        // Prune stale notifications using the session IDs we collected during
        // the build pass (no extra scan needed).
        notified_threads.retain(|id| current_session_ids.contains(id));

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

        let rendered = match entry {
            ListEntry::ProjectHeader {
                path_list,
                label,
                workspace,
                highlight_positions,
                has_running_threads,
                waiting_thread_count,
                is_active,
            } => self.render_project_header(
                ix,
                false,
                path_list,
                label,
                workspace,
                highlight_positions,
                *has_running_threads,
                *waiting_thread_count,
                *is_active,
                is_selected,
                cx,
            ),
            ListEntry::Thread(thread) => self.render_thread(ix, thread, is_selected, cx),
            ListEntry::ViewMore {
                path_list,
                is_fully_expanded,
            } => self.render_view_more(ix, path_list, *is_fully_expanded, is_selected, cx),
            ListEntry::NewThread {
                path_list,
                workspace,
                is_active_draft,
            } => {
                self.render_new_thread(ix, path_list, workspace, *is_active_draft, is_selected, cx)
            }
        };

        if is_group_header_after_first {
            v_flex()
                .w_full()
                .border_t_1()
                .border_color(cx.theme().colors().border.opacity(0.5))
                .child(rendered)
                .into_any_element()
        } else {
            rendered
        }
    }

    fn render_project_header(
        &self,
        ix: usize,
        is_sticky: bool,
        path_list: &PathList,
        label: &SharedString,
        workspace: &Entity<Workspace>,
        highlight_positions: &[usize],
        has_running_threads: bool,
        waiting_thread_count: usize,
        is_active: bool,
        is_selected: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let id_prefix = if is_sticky { "sticky-" } else { "" };
        let id = SharedString::from(format!("{id_prefix}project-header-{ix}"));
        let group_name = SharedString::from(format!("{id_prefix}header-group-{ix}"));

        let is_collapsed = self.collapsed_groups.contains(path_list);
        let disclosure_icon = if is_collapsed {
            IconName::ChevronRight
        } else {
            IconName::ChevronDown
        };

        let has_new_thread_entry = self
            .contents
            .entries
            .get(ix + 1)
            .is_some_and(|entry| matches!(entry, ListEntry::NewThread { .. }));
        let show_new_thread_button = !has_new_thread_entry && !self.has_filter_query(cx);

        let workspace_for_remove = workspace.clone();
        let workspace_for_menu = workspace.clone();
        let workspace_for_open = workspace.clone();

        let path_list_for_toggle = path_list.clone();
        let path_list_for_collapse = path_list.clone();
        let view_more_expanded = self.expanded_groups.contains_key(path_list);

        let label = if highlight_positions.is_empty() {
            Label::new(label.clone())
                .color(Color::Muted)
                .into_any_element()
        } else {
            HighlightedLabel::new(label.clone(), highlight_positions.to_vec())
                .color(Color::Muted)
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
            .pl_1p5()
            .pr_1()
            .border_1()
            .map(|this| {
                if is_selected {
                    this.border_color(color.border_focused)
                } else {
                    this.border_color(gpui::transparent_black())
                }
            })
            .justify_between()
            .hover(|s| s.bg(hover_color))
            .child(
                h_flex()
                    .relative()
                    .min_w_0()
                    .w_full()
                    .gap_1p5()
                    .child(
                        h_flex().size_4().flex_none().justify_center().child(
                            Icon::new(disclosure_icon)
                                .size(IconSize::Small)
                                .color(Color::Custom(cx.theme().colors().icon_muted.opacity(0.5))),
                        ),
                    )
                    .child(label)
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
            .child({
                let workspace_for_new_thread = workspace.clone();
                let path_list_for_new_thread = path_list.clone();

                h_flex()
                    .when(self.project_header_menu_ix != Some(ix), |this| {
                        this.visible_on_hover(group_name)
                    })
                    .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    })
                    .child(self.render_project_header_menu(
                        ix,
                        id_prefix,
                        &workspace_for_menu,
                        &workspace_for_remove,
                        cx,
                    ))
                    .when(view_more_expanded && !is_collapsed, |this| {
                        this.child(
                            IconButton::new(
                                SharedString::from(format!(
                                    "{id_prefix}project-header-collapse-{ix}",
                                )),
                                IconName::ListCollapse,
                            )
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .tooltip(Tooltip::text("Collapse Displayed Threads"))
                            .on_click(cx.listener({
                                let path_list_for_collapse = path_list_for_collapse.clone();
                                move |this, _, _window, cx| {
                                    this.selection = None;
                                    this.expanded_groups.remove(&path_list_for_collapse);
                                    this.update_entries(cx);
                                }
                            })),
                        )
                    })
                    .when(!is_active, |this| {
                        this.child(
                            IconButton::new(
                                SharedString::from(format!(
                                    "{id_prefix}project-header-open-workspace-{ix}",
                                )),
                                IconName::Focus,
                            )
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .tooltip(Tooltip::text("Activate Workspace"))
                            .on_click(cx.listener({
                                move |this, _, window, cx| {
                                    this.focused_thread = None;
                                    if let Some(multi_workspace) = this.multi_workspace.upgrade() {
                                        multi_workspace.update(cx, |multi_workspace, cx| {
                                            multi_workspace
                                                .activate(workspace_for_open.clone(), cx);
                                        });
                                    }
                                    if AgentPanel::is_visible(&workspace_for_open, cx) {
                                        workspace_for_open.update(cx, |workspace, cx| {
                                            workspace.focus_panel::<AgentPanel>(window, cx);
                                        });
                                    }
                                }
                            })),
                        )
                    })
                    .when(show_new_thread_button, |this| {
                        this.child(
                            IconButton::new(
                                SharedString::from(format!(
                                    "{id_prefix}project-header-new-thread-{ix}",
                                )),
                                IconName::Plus,
                            )
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .tooltip(Tooltip::text("New Thread"))
                            .on_click(cx.listener({
                                let workspace_for_new_thread = workspace_for_new_thread.clone();
                                let path_list_for_new_thread = path_list_for_new_thread.clone();
                                move |this, _, window, cx| {
                                    // Uncollapse the group if collapsed so
                                    // the new-thread entry becomes visible.
                                    this.collapsed_groups.remove(&path_list_for_new_thread);
                                    this.selection = None;
                                    this.create_new_thread(&workspace_for_new_thread, window, cx);
                                }
                            })),
                        )
                    })
            })
            .on_click(cx.listener(move |this, _, window, cx| {
                this.selection = None;
                this.toggle_collapse(&path_list_for_toggle, window, cx);
            }))
            .into_any_element()
    }

    fn render_project_header_menu(
        &self,
        ix: usize,
        id_prefix: &str,
        workspace: &Entity<Workspace>,
        workspace_for_remove: &Entity<Workspace>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let workspace_for_menu = workspace.clone();
        let workspace_for_remove = workspace_for_remove.clone();
        let multi_workspace = self.multi_workspace.clone();
        let this = cx.weak_entity();

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
                let workspace = workspace_for_menu.clone();
                let workspace_for_remove = workspace_for_remove.clone();
                let multi_workspace = multi_workspace.clone();

                let menu = ContextMenu::build_persistent(window, cx, move |menu, _window, cx| {
                    let worktrees: Vec<_> = workspace
                        .read(cx)
                        .visible_worktrees(cx)
                        .map(|worktree| {
                            let worktree_read = worktree.read(cx);
                            let id = worktree_read.id();
                            let name: SharedString =
                                worktree_read.root_name().as_unix_str().to_string().into();
                            (id, name)
                        })
                        .collect();

                    let worktree_count = worktrees.len();

                    let mut menu = menu
                        .header("Project Folders")
                        .end_slot_action(Box::new(menu::EndSlot));

                    for (worktree_id, name) in &worktrees {
                        let worktree_id = *worktree_id;
                        let workspace_for_worktree = workspace.clone();
                        let workspace_for_remove_worktree = workspace_for_remove.clone();
                        let multi_workspace_for_worktree = multi_workspace.clone();

                        let remove_handler = move |window: &mut Window, cx: &mut App| {
                            if worktree_count <= 1 {
                                if let Some(mw) = multi_workspace_for_worktree.upgrade() {
                                    let ws = workspace_for_remove_worktree.clone();
                                    mw.update(cx, |multi_workspace, cx| {
                                        if let Some(index) = multi_workspace
                                            .workspaces()
                                            .iter()
                                            .position(|w| *w == ws)
                                        {
                                            multi_workspace.remove_workspace(index, window, cx);
                                        }
                                    });
                                }
                            } else {
                                workspace_for_worktree.update(cx, |workspace, cx| {
                                    workspace.project().update(cx, |project, cx| {
                                        project.remove_worktree(worktree_id, cx);
                                    });
                                });
                            }
                        };

                        menu = menu.entry_with_end_slot_on_hover(
                            name.clone(),
                            None,
                            |_, _| {},
                            IconName::Close,
                            "Remove Folder".into(),
                            remove_handler,
                        );
                    }

                    let workspace_for_add = workspace.clone();
                    let multi_workspace_for_add = multi_workspace.clone();
                    let menu = menu.separator().entry(
                        "Add Folder to Project",
                        Some(Box::new(AddFolderToProject)),
                        move |window, cx| {
                            if let Some(mw) = multi_workspace_for_add.upgrade() {
                                mw.update(cx, |mw, cx| {
                                    mw.activate(workspace_for_add.clone(), cx);
                                });
                            }
                            workspace_for_add.update(cx, |workspace, cx| {
                                workspace.add_folder_to_project(&AddFolderToProject, window, cx);
                            });
                        },
                    );

                    let workspace_count = multi_workspace
                        .upgrade()
                        .map_or(0, |mw| mw.read(cx).workspaces().len());
                    let menu = if workspace_count > 1 {
                        let workspace_for_move = workspace.clone();
                        let multi_workspace_for_move = multi_workspace.clone();
                        menu.entry(
                            "Move to New Window",
                            Some(Box::new(
                                zed_actions::agents_sidebar::MoveWorkspaceToNewWindow,
                            )),
                            move |window, cx| {
                                if let Some(mw) = multi_workspace_for_move.upgrade() {
                                    mw.update(cx, |multi_workspace, cx| {
                                        if let Some(index) = multi_workspace
                                            .workspaces()
                                            .iter()
                                            .position(|w| *w == workspace_for_move)
                                        {
                                            multi_workspace
                                                .move_workspace_to_new_window(index, window, cx);
                                        }
                                    });
                                }
                            },
                        )
                    } else {
                        menu
                    };

                    let workspace_for_remove = workspace_for_remove.clone();
                    let multi_workspace_for_remove = multi_workspace.clone();
                    menu.separator()
                        .entry("Remove Project", None, move |window, cx| {
                            if let Some(mw) = multi_workspace_for_remove.upgrade() {
                                let ws = workspace_for_remove.clone();
                                mw.update(cx, |multi_workspace, cx| {
                                    if let Some(index) =
                                        multi_workspace.workspaces().iter().position(|w| *w == ws)
                                    {
                                        multi_workspace.remove_workspace(index, window, cx);
                                    }
                                });
                            }
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
                .icon_size(IconSize::Small)
                .icon_color(Color::Muted),
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
            path_list,
            label,
            workspace,
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
            &path_list,
            &label,
            workspace,
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
        self.update_entries(cx);
    }

    fn dispatch_context(&self, window: &Window, cx: &Context<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("ThreadsSidebar");
        dispatch_context.add("menu");

        let identifier = if self.filter_editor.focus_handle(cx).is_focused(window) {
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
            ListEntry::ProjectHeader { path_list, .. } => {
                let path_list = path_list.clone();
                self.toggle_collapse(&path_list, window, cx);
            }
            ListEntry::Thread(thread) => {
                let session_info = thread.session_info.clone();
                match &thread.workspace {
                    ThreadEntryWorkspace::Open(workspace) => {
                        let workspace = workspace.clone();
                        self.activate_thread(
                            thread.agent.clone(),
                            session_info,
                            &workspace,
                            window,
                            cx,
                        );
                    }
                    ThreadEntryWorkspace::Closed(path_list) => {
                        self.open_workspace_and_activate_thread(
                            thread.agent.clone(),
                            session_info,
                            path_list.clone(),
                            window,
                            cx,
                        );
                    }
                }
            }
            ListEntry::ViewMore {
                path_list,
                is_fully_expanded,
                ..
            } => {
                let path_list = path_list.clone();
                if *is_fully_expanded {
                    self.expanded_groups.remove(&path_list);
                } else {
                    let current = self.expanded_groups.get(&path_list).copied().unwrap_or(0);
                    self.expanded_groups.insert(path_list, current + 1);
                }
                self.update_entries(cx);
            }
            ListEntry::NewThread { workspace, .. } => {
                let workspace = workspace.clone();
                self.create_new_thread(&workspace, window, cx);
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
                        .iter()
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
                .iter()
                .find(|workspace| predicate(workspace, cx))
                .cloned()
        })
    }

    fn load_agent_thread_in_workspace(
        workspace: &Entity<Workspace>,
        agent: Agent,
        session_info: acp_thread::AgentSessionInfo,
        window: &mut Window,
        cx: &mut App,
    ) {
        workspace.update(cx, |workspace, cx| {
            workspace.open_panel::<AgentPanel>(window, cx);
        });

        if let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
            agent_panel.update(cx, |panel, cx| {
                panel.load_agent_thread(
                    agent,
                    session_info.session_id,
                    session_info.work_dirs,
                    session_info.title,
                    true,
                    window,
                    cx,
                );
            });
        }
    }

    fn activate_thread_locally(
        &mut self,
        agent: Agent,
        session_info: acp_thread::AgentSessionInfo,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        // Set focused_thread eagerly so the sidebar highlight updates
        // immediately, rather than waiting for a deferred AgentPanel
        // event which can race with ActiveWorkspaceChanged clearing it.
        self.focused_thread = Some(session_info.session_id.clone());

        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.activate(workspace.clone(), cx);
        });

        Self::load_agent_thread_in_workspace(workspace, agent, session_info, window, cx);

        self.update_entries(cx);
    }

    fn activate_thread_in_other_window(
        &self,
        agent: Agent,
        session_info: acp_thread::AgentSessionInfo,
        workspace: Entity<Workspace>,
        target_window: WindowHandle<MultiWorkspace>,
        cx: &mut Context<Self>,
    ) {
        let target_session_id = session_info.session_id.clone();

        let activated = target_window
            .update(cx, |multi_workspace, window, cx| {
                window.activate_window();
                multi_workspace.activate(workspace.clone(), cx);
                Self::load_agent_thread_in_workspace(&workspace, agent, session_info, window, cx);
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
                    sidebar.focused_thread = Some(target_session_id);
                    sidebar.update_entries(cx);
                });
            }
        }
    }

    fn activate_thread(
        &mut self,
        agent: Agent,
        session_info: acp_thread::AgentSessionInfo,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self
            .find_workspace_in_current_window(cx, |candidate, _| candidate == workspace)
            .is_some()
        {
            self.activate_thread_locally(agent, session_info, &workspace, window, cx);
            return;
        }

        let Some((target_window, workspace)) =
            self.find_workspace_across_windows(cx, |candidate, _| candidate == workspace)
        else {
            return;
        };

        self.activate_thread_in_other_window(agent, session_info, workspace, target_window, cx);
    }

    fn open_workspace_and_activate_thread(
        &mut self,
        agent: Agent,
        session_info: acp_thread::AgentSessionInfo,
        path_list: PathList,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        let paths: Vec<std::path::PathBuf> =
            path_list.paths().iter().map(|p| p.to_path_buf()).collect();

        let open_task = multi_workspace.update(cx, |mw, cx| mw.open_project(paths, window, cx));

        cx.spawn_in(window, async move |this, cx| {
            let workspace = open_task.await?;

            this.update_in(cx, |this, window, cx| {
                this.activate_thread(agent, session_info, &workspace, window, cx);
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
        agent: Agent,
        session_info: acp_thread::AgentSessionInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Eagerly save thread metadata so that the sidebar is updated immediately
        SidebarThreadMetadataStore::global(cx).update(cx, |store, cx| {
            store.save(
                ThreadMetadata::from_session_info(agent.id(), &session_info),
                cx,
            )
        });

        if let Some(path_list) = &session_info.work_dirs {
            if let Some(workspace) = self.find_current_workspace_for_path_list(path_list, cx) {
                self.activate_thread_locally(agent, session_info, &workspace, window, cx);
            } else if let Some((target_window, workspace)) =
                self.find_open_workspace_for_path_list(path_list, cx)
            {
                self.activate_thread_in_other_window(
                    agent,
                    session_info,
                    workspace,
                    target_window,
                    cx,
                );
            } else {
                let path_list = path_list.clone();
                self.open_workspace_and_activate_thread(agent, session_info, path_list, window, cx);
            }
            return;
        }

        let active_workspace = self.multi_workspace.upgrade().and_then(|w| {
            w.read(cx)
                .workspaces()
                .get(w.read(cx).active_workspace_index())
                .cloned()
        });

        if let Some(workspace) = active_workspace {
            self.activate_thread_locally(agent, session_info, &workspace, window, cx);
        }
    }

    fn expand_selected_entry(
        &mut self,
        _: &SelectChild,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else { return };

        match self.contents.entries.get(ix) {
            Some(ListEntry::ProjectHeader { path_list, .. }) => {
                if self.collapsed_groups.contains(path_list) {
                    let path_list = path_list.clone();
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
            Some(ListEntry::ProjectHeader { path_list, .. }) => {
                if !self.collapsed_groups.contains(path_list) {
                    let path_list = path_list.clone();
                    self.collapsed_groups.insert(path_list);
                    self.update_entries(cx);
                }
            }
            Some(
                ListEntry::Thread(_) | ListEntry::ViewMore { .. } | ListEntry::NewThread { .. },
            ) => {
                for i in (0..ix).rev() {
                    if let Some(ListEntry::ProjectHeader { path_list, .. }) =
                        self.contents.entries.get(i)
                    {
                        let path_list = path_list.clone();
                        self.selection = Some(i);
                        self.collapsed_groups.insert(path_list);
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
                ListEntry::Thread(_) | ListEntry::ViewMore { .. } | ListEntry::NewThread { .. },
            ) => (0..ix).rev().find(|&i| {
                matches!(
                    self.contents.entries.get(i),
                    Some(ListEntry::ProjectHeader { .. })
                )
            }),
            None => None,
        };

        if let Some(header_ix) = header_ix {
            if let Some(ListEntry::ProjectHeader { path_list, .. }) =
                self.contents.entries.get(header_ix)
            {
                let path_list = path_list.clone();
                if self.collapsed_groups.contains(&path_list) {
                    self.collapsed_groups.remove(&path_list);
                } else {
                    self.selection = Some(header_ix);
                    self.collapsed_groups.insert(path_list);
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
            if let ListEntry::ProjectHeader { path_list, .. } = entry {
                self.collapsed_groups.insert(path_list.clone());
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

        let workspaces = multi_workspace.read(cx).workspaces().to_vec();
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
        // If we're archiving the currently focused thread, move focus to the
        // nearest thread within the same project group. We never cross group
        // boundaries — if the group has no other threads, clear focus and open
        // a blank new thread in the panel instead.
        if self.focused_thread.as_ref() == Some(session_id) {
            let current_pos = self.contents.entries.iter().position(|entry| {
                matches!(entry, ListEntry::Thread(t) if &t.session_info.session_id == session_id)
            });

            // Find the workspace that owns this thread's project group by
            // walking backwards to the nearest ProjectHeader. We must use
            // *this* workspace (not the active workspace) because the user
            // might be archiving a thread in a non-active group.
            let group_workspace = current_pos.and_then(|pos| {
                self.contents.entries[..pos]
                    .iter()
                    .rev()
                    .find_map(|e| match e {
                        ListEntry::ProjectHeader { workspace, .. } => Some(workspace.clone()),
                        _ => None,
                    })
            });

            let next_thread = current_pos.and_then(|pos| {
                let group_start = self.contents.entries[..pos]
                    .iter()
                    .rposition(|e| matches!(e, ListEntry::ProjectHeader { .. }))
                    .map_or(0, |i| i + 1);
                let group_end = self.contents.entries[pos + 1..]
                    .iter()
                    .position(|e| matches!(e, ListEntry::ProjectHeader { .. }))
                    .map_or(self.contents.entries.len(), |i| pos + 1 + i);

                let above = self.contents.entries[group_start..pos]
                    .iter()
                    .rev()
                    .find_map(|entry| {
                        if let ListEntry::Thread(t) = entry {
                            Some(t)
                        } else {
                            None
                        }
                    });

                above.or_else(|| {
                    self.contents.entries[pos + 1..group_end]
                        .iter()
                        .find_map(|entry| {
                            if let ListEntry::Thread(t) = entry {
                                Some(t)
                            } else {
                                None
                            }
                        })
                })
            });

            if let Some(next) = next_thread {
                self.focused_thread = Some(next.session_info.session_id.clone());

                // Use the thread's own workspace when it has one open (e.g. an absorbed
                // linked worktree thread that appears under the main workspace's header
                // but belongs to its own workspace). Loading into the wrong panel binds
                // the thread to the wrong project, which corrupts its stored folder_paths
                // when metadata is saved via ThreadMetadata::from_thread.
                let target_workspace = match &next.workspace {
                    ThreadEntryWorkspace::Open(ws) => Some(ws.clone()),
                    ThreadEntryWorkspace::Closed(_) => group_workspace,
                };

                if let Some(workspace) = target_workspace {
                    if let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                        agent_panel.update(cx, |panel, cx| {
                            panel.load_agent_thread(
                                next.agent.clone(),
                                next.session_info.session_id.clone(),
                                next.session_info.work_dirs.clone(),
                                next.session_info.title.clone(),
                                true,
                                window,
                                cx,
                            );
                        });
                    }
                }
            } else {
                self.focused_thread = None;
                if let Some(workspace) = &group_workspace {
                    if let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                        agent_panel.update(cx, |panel, cx| {
                            panel.new_thread(&NewThread, window, cx);
                        });
                    }
                }
            }
        }

        SidebarThreadMetadataStore::global(cx)
            .update(cx, |store, cx| store.delete(session_id.clone(), cx));
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
        let Some(ListEntry::Thread(thread)) = self.contents.entries.get(ix) else {
            return;
        };
        if thread.agent != Agent::NativeAgent {
            return;
        }
        let session_id = thread.session_info.session_id.clone();
        self.archive_thread(&session_id, window, cx);
    }

    fn render_thread(
        &self,
        ix: usize,
        thread: &ThreadEntry,
        is_focused: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let has_notification = self
            .contents
            .is_thread_notified(&thread.session_info.session_id);

        let title: SharedString = thread
            .session_info
            .title
            .clone()
            .unwrap_or_else(|| "Untitled".into());
        let session_info = thread.session_info.clone();
        let thread_workspace = thread.workspace.clone();

        let is_hovered = self.hovered_thread_index == Some(ix);
        let is_selected = self.agent_panel_visible
            && self.focused_thread.as_ref() == Some(&session_info.session_id);
        let is_running = matches!(
            thread.status,
            AgentThreadStatus::Running | AgentThreadStatus::WaitingForConfirmation
        );

        let session_id_for_delete = thread.session_info.session_id.clone();
        let focus_handle = self.focus_handle.clone();

        let id = SharedString::from(format!("thread-entry-{}", ix));

        let timestamp = thread
            .session_info
            .created_at
            .or(thread.session_info.updated_at)
            .map(format_history_entry_timestamp);

        ThreadItem::new(id, title)
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
                    })
                    .collect(),
            )
            .when_some(timestamp, |this, ts| this.timestamp(ts))
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
                let agent = thread.agent.clone();
                cx.listener(move |this, _, window, cx| {
                    this.selection = None;
                    match &thread_workspace {
                        ThreadEntryWorkspace::Open(workspace) => {
                            this.activate_thread(
                                agent.clone(),
                                session_info.clone(),
                                workspace,
                                window,
                                cx,
                            );
                        }
                        ThreadEntryWorkspace::Closed(path_list) => {
                            this.open_workspace_and_activate_thread(
                                agent.clone(),
                                session_info.clone(),
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

        let sibling_workspace_ids: HashSet<WorkspaceId> = multi_workspace
            .as_ref()
            .map(|mw| {
                mw.read(cx)
                    .workspaces()
                    .iter()
                    .filter_map(|ws| ws.read(cx).database_id())
                    .collect()
            })
            .unwrap_or_default();

        let popover_handle = self.recent_projects_popover_handle.clone();

        PopoverMenu::new("sidebar-recent-projects-menu")
            .with_handle(popover_handle)
            .menu(move |window, cx| {
                workspace.as_ref().map(|ws| {
                    SidebarRecentProjects::popover(
                        ws.clone(),
                        sibling_workspace_ids.clone(),
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
                    this.expanded_groups.remove(&path_list);
                } else {
                    let current = this.expanded_groups.get(&path_list).copied().unwrap_or(0);
                    this.expanded_groups.insert(path_list.clone(), current + 1);
                }
                this.update_entries(cx);
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
                    ListEntry::ProjectHeader { workspace, .. } => Some(workspace.clone()),
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

        // Clear focused_thread immediately so no existing thread stays
        // highlighted while the new blank thread is being shown. Without this,
        // if the target workspace is already active (so ActiveWorkspaceChanged
        // never fires), the previous thread's highlight would linger.
        self.focused_thread = None;

        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.activate(workspace.clone(), cx);
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

    fn render_new_thread(
        &self,
        ix: usize,
        _path_list: &PathList,
        workspace: &Entity<Workspace>,
        is_active_draft: bool,
        is_selected: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let is_active = is_active_draft && self.agent_panel_visible && self.active_thread_is_draft;

        let label: SharedString = if is_active {
            self.active_draft_text(cx)
                .unwrap_or_else(|| DEFAULT_THREAD_TITLE.into())
        } else {
            DEFAULT_THREAD_TITLE.into()
        };

        let workspace = workspace.clone();
        let id = SharedString::from(format!("new-thread-btn-{}", ix));

        let thread_item = ThreadItem::new(id, label)
            .icon(IconName::Plus)
            .icon_color(Color::Custom(cx.theme().colors().icon_muted.opacity(0.8)))
            .selected(is_active)
            .focused(is_selected)
            .when(!is_active, |this| {
                this.on_click(cx.listener(move |this, _, window, cx| {
                    this.selection = None;
                    this.create_new_thread(&workspace, window, cx);
                }))
            });

        if is_active {
            div()
                .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| {
                    cx.stop_propagation();
                })
                .child(thread_item)
                .into_any_element()
        } else {
            thread_item.into_any_element()
        }
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
                    .child(Divider::horizontal())
                    .child(Label::new("or").size(LabelSize::XSmall).color(Color::Muted))
                    .child(Divider::horizontal()),
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
        let traffic_lights =
            cfg!(target_os = "macos") && !window.is_fullscreen() && sidebar_on_left;
        let header_height = platform_title_bar_height(window);

        h_flex()
            .h(header_height)
            .mt_px()
            .pb_px()
            .map(|this| {
                if traffic_lights {
                    this.pl(px(ui::utils::TRAFFIC_LIGHT_PADDING))
                } else {
                    this.pl_1p5()
                }
            })
            .pr_1p5()
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
        let on_right = self.side(cx) == SidebarSide::Right;
        let is_archive = matches!(self.view, SidebarView::Archive(..));
        let action_buttons = h_flex()
            .gap_1()
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
        let border_color = cx.theme().colors().border;
        let toggle_button = self.render_sidebar_toggle_button(cx);

        let bar = h_flex()
            .p_1()
            .gap_1()
            .justify_between()
            .border_t_1()
            .border_color(border_color);

        if on_right {
            bar.child(action_buttons).child(toggle_button)
        } else {
            bar.child(toggle_button).child(action_buttons)
        }
    }

    fn toggle_archive(&mut self, _: &ToggleArchive, window: &mut Window, cx: &mut Context<Self>) {
        match &self.view {
            SidebarView::ThreadList => self.show_archive(window, cx),
            SidebarView::Archive(_) => self.show_thread_list(window, cx),
        }
    }

    fn show_archive(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_workspace) = self.multi_workspace.upgrade().and_then(|w| {
            w.read(cx)
                .workspaces()
                .get(w.read(cx).active_workspace_index())
                .cloned()
        }) else {
            return;
        };

        let Some(agent_panel) = active_workspace.read(cx).panel::<AgentPanel>(cx) else {
            return;
        };

        let thread_store = agent_panel.read(cx).thread_store().clone();
        let fs = active_workspace.read(cx).project().read(cx).fs().clone();
        let agent_connection_store = agent_panel.read(cx).connection_store().clone();
        let agent_server_store = active_workspace
            .read(cx)
            .project()
            .read(cx)
            .agent_server_store()
            .clone();

        let archive_view = cx.new(|cx| {
            ThreadsArchiveView::new(
                agent_connection_store,
                agent_server_store,
                thread_store,
                fs,
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
                ThreadsArchiveViewEvent::Unarchive {
                    agent,
                    session_info,
                } => {
                    this.show_thread_list(window, cx);
                    this.activate_archived_thread(agent.clone(), session_info.clone(), window, cx);
                }
            },
        );

        self._subscriptions.push(subscription);
        self.view = SidebarView::Archive(archive_view.clone());
        archive_view.update(cx, |view, cx| view.focus_filter_editor(window, cx));
        cx.notify();
    }

    fn show_thread_list(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.view = SidebarView::ThreadList;
        self._subscriptions.clear();
        let handle = self.filter_editor.read(cx).focus_handle(cx);
        handle.focus(window, cx);
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
}

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
            .blend(color.panel_background.opacity(0.32));

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
        .parent_threads(cx)
        .into_iter()
        .map(|thread_view| {
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

            let status = if thread.is_waiting_for_confirmation() {
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

            ActiveThreadInfo {
                session_id,
                title,
                status,
                icon,
                icon_from_external_svg,
                is_background,
                is_title_generating,
                diff_stats,
            }
        });

    Some(threads).into_iter().flatten()
}
