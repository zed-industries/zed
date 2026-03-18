use acp_thread::ThreadStatus;
use action_log::DiffStats;
use agent::ThreadStore;
use agent_client_protocol::{self as acp};
use agent_ui::thread_metadata_store::{ThreadMetadata, ThreadMetadataStore};
use agent_ui::threads_archive_view::{ThreadsArchiveView, ThreadsArchiveViewEvent};
use agent_ui::{Agent, AgentPanel, AgentPanelEvent, NewThread, RemoveSelectedThread};
use chrono::Utc;
use editor::Editor;
use feature_flags::{AgentV2FeatureFlag, FeatureFlagViewExt as _};
use gpui::{
    Action as _, AnyElement, App, Context, Entity, FocusHandle, Focusable, ListState, Pixels,
    Render, SharedString, WeakEntity, Window, actions, list, prelude::*, px,
};
use menu::{Cancel, Confirm, SelectFirst, SelectLast, SelectNext, SelectPrevious};
use project::{AgentId, Event as ProjectEvent};
use recent_projects::RecentProjects;
use ui::utils::platform_title_bar_height;

use std::collections::{HashMap, HashSet};
use std::mem;
use std::path::Path;
use std::sync::Arc;
use theme::ActiveTheme;
use ui::{
    AgentThreadStatus, ButtonStyle, HighlightedLabel, KeyBinding, ListItem, PopoverMenu,
    PopoverMenuHandle, Tab, ThreadItem, TintColor, Tooltip, WithScrollbar, prelude::*,
};
use util::ResultExt as _;
use util::path_list::PathList;
use workspace::{
    FocusWorkspaceSidebar, MultiWorkspace, MultiWorkspaceEvent, Sidebar as WorkspaceSidebar,
    ToggleWorkspaceSidebar, Workspace,
};

use zed_actions::OpenRecent;
use zed_actions::editor::{MoveDown, MoveUp};

actions!(
    agents_sidebar,
    [
        /// Collapses the selected entry in the workspace sidebar.
        CollapseSelectedEntry,
        /// Expands the selected entry in the workspace sidebar.
        ExpandSelectedEntry,
        /// Moves focus to the sidebar's search/filter editor.
        FocusSidebarFilter,
        /// Creates a new thread in the currently selected or active project group.
        NewThreadInGroup,
    ]
);

const DEFAULT_WIDTH: Pixels = px(320.0);
const MIN_WIDTH: Pixels = px(200.0);
const MAX_WIDTH: Pixels = px(800.0);
const DEFAULT_THREADS_SHOWN: usize = 5;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum SidebarView {
    #[default]
    ThreadList,
    Archive,
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
    worktree_name: Option<SharedString>,
    worktree_highlight_positions: Vec<usize>,
    diff_stats: DiffStats,
}

#[derive(Clone)]
enum ListEntry {
    ProjectHeader {
        path_list: PathList,
        label: SharedString,
        workspace: Entity<Workspace>,
        highlight_positions: Vec<usize>,
    },
    Thread(ThreadEntry),
    ViewMore {
        path_list: PathList,
        remaining_count: usize,
        is_fully_expanded: bool,
    },
    NewThread {
        path_list: PathList,
        workspace: Entity<Workspace>,
    },
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
) -> Vec<project::git_store::RepositorySnapshot> {
    let path_list = workspace_path_list(workspace, cx);
    let project = workspace.read(cx).project().read(cx);
    project
        .repositories(cx)
        .values()
        .filter_map(|repo| {
            let snapshot = repo.read(cx).snapshot();
            let is_root = path_list
                .paths()
                .iter()
                .any(|p| p.as_path() == snapshot.work_directory_abs_path.as_ref());
            is_root.then_some(snapshot)
        })
        .collect()
}

fn workspace_path_list(workspace: &Entity<Workspace>, cx: &App) -> PathList {
    PathList::new(&workspace.read(cx).root_paths(cx))
}

fn workspace_label_from_path_list(path_list: &PathList) -> SharedString {
    let mut names = Vec::with_capacity(path_list.paths().len());
    for abs_path in path_list.paths() {
        if let Some(name) = abs_path.file_name() {
            names.push(name.to_string_lossy().to_string());
        }
    }
    if names.is_empty() {
        // TODO: Can we do something better in this case?
        "Empty Workspace".into()
    } else {
        names.join(", ").into()
    }
}

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
    focused_thread: Option<acp::SessionId>,
    /// Set to true when WorkspaceRemoved fires so the subsequent
    /// ActiveWorkspaceChanged event knows not to clear focused_thread.
    /// A workspace removal changes the active workspace as a side-effect, but
    /// that should not reset the user's thread focus the way an explicit
    /// workspace switch does.
    pending_workspace_removal: bool,

    active_entry_index: Option<usize>,
    hovered_thread_index: Option<usize>,
    collapsed_groups: HashSet<PathList>,
    expanded_groups: HashMap<PathList, usize>,
    view: SidebarView,
    archive_view: Option<Entity<ThreadsArchiveView>>,
    recent_projects_popover_handle: PopoverMenuHandle<RecentProjects>,
    _subscriptions: Vec<gpui::Subscription>,
    _update_entries_task: Option<gpui::Task<()>>,
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
            editor.set_placeholder_text("Search…", window, cx);
            editor
        });

        cx.subscribe_in(
            &multi_workspace,
            window,
            |this, _multi_workspace, event: &MultiWorkspaceEvent, window, cx| match event {
                MultiWorkspaceEvent::ActiveWorkspaceChanged => {
                    // Don't clear focused_thread when the active workspace
                    // changed because a workspace was removed — the focused
                    // thread may still be valid in the new active workspace.
                    // Only clear it for explicit user-initiated switches.
                    if mem::take(&mut this.pending_workspace_removal) {
                        // If the removed workspace had no focused thread, seed
                        // from the new active panel so its current thread gets
                        // highlighted — same logic as subscribe_to_workspace.
                        if this.focused_thread.is_none() {
                            if let Some(mw) = this.multi_workspace.upgrade() {
                                let ws = mw.read(cx).workspace();
                                if let Some(panel) = ws.read(cx).panel::<AgentPanel>(cx) {
                                    this.focused_thread = panel
                                        .read(cx)
                                        .active_conversation()
                                        .and_then(|cv| cv.read(cx).parent_id(cx));
                                }
                            }
                        }
                    } else {
                        this.focused_thread = None;
                    }
                    this.observe_draft_editor(cx);
                    this.update_entries(false, cx);
                }
                MultiWorkspaceEvent::WorkspaceAdded(workspace) => {
                    this.subscribe_to_workspace(workspace, window, cx);
                    this.update_entries(false, cx);
                }
                MultiWorkspaceEvent::WorkspaceRemoved(_) => {
                    // Signal that the upcoming ActiveWorkspaceChanged event is
                    // a consequence of this removal, not a user workspace switch.
                    this.pending_workspace_removal = true;
                    this.update_entries(false, cx);
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
                this.update_entries(!query.is_empty(), cx);
            }
        })
        .detach();

        cx.observe(&ThreadMetadataStore::global(cx), |this, _store, cx| {
            this.update_entries(false, cx);
        })
        .detach();

        cx.observe_flag::<AgentV2FeatureFlag, _>(window, |_is_enabled, this, _window, cx| {
            this.update_entries(false, cx);
        })
        .detach();

        let workspaces = multi_workspace.read(cx).workspaces().to_vec();
        cx.defer_in(window, move |this, window, cx| {
            for workspace in &workspaces {
                this.subscribe_to_workspace(workspace, window, cx);
            }
            this.update_entries(false, cx);
        });

        Self {
            _update_entries_task: None,
            multi_workspace: multi_workspace.downgrade(),
            width: DEFAULT_WIDTH,
            focus_handle,
            filter_editor,
            list_state: ListState::new(0, gpui::ListAlignment::Top, px(1000.)),
            contents: SidebarContents::default(),
            selection: None,
            focused_thread: None,
            pending_workspace_removal: false,
            active_entry_index: None,
            hovered_thread_index: None,
            collapsed_groups: HashSet::new(),
            expanded_groups: HashMap::new(),
            view: SidebarView::default(),
            archive_view: None,
            recent_projects_popover_handle: PopoverMenuHandle::default(),
            _subscriptions: Vec::new(),
            _draft_observation: None,
        }
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
                    this.update_entries(false, cx);
                }
                _ => {}
            },
        )
        .detach();

        let git_store = workspace.read(cx).project().read(cx).git_store().clone();
        cx.subscribe_in(
            &git_store,
            window,
            |this, _, event: &project::git_store::GitStoreEvent, window, cx| {
                if matches!(
                    event,
                    project::git_store::GitStoreEvent::RepositoryUpdated(
                        _,
                        project::git_store::RepositoryEvent::GitWorktreeListChanged,
                        _,
                    )
                ) {
                    this.prune_stale_worktree_workspaces(window, cx);
                    this.update_entries(false, cx);
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

        if let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
            self.subscribe_to_agent_panel(&agent_panel, window, cx);
            // Seed the initial focused_thread so the correct thread item is
            // highlighted right away, without waiting for the panel to emit
            // an event (which only happens on *changes*, not on first load).
            self.focused_thread = agent_panel
                .read(cx)
                .active_conversation()
                .and_then(|cv| cv.read(cx).parent_id(cx));
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
            |this, agent_panel, event: &AgentPanelEvent, _window, cx| {
                // Check whether the panel that emitted this event belongs to
                // the currently active workspace. Only the active workspace's
                // panel should drive focused_thread — otherwise running threads
                // in background workspaces would continuously overwrite it,
                // causing the selection highlight to jump around.
                let is_active_panel = this
                    .multi_workspace
                    .upgrade()
                    .and_then(|mw| mw.read(cx).workspace().read(cx).panel::<AgentPanel>(cx))
                    .is_some_and(|active_panel| active_panel == *agent_panel);

                match event {
                    AgentPanelEvent::ActiveViewChanged => {
                        if is_active_panel {
                            this.focused_thread = agent_panel
                                .read(cx)
                                .active_conversation()
                                .and_then(|cv| cv.read(cx).parent_id(cx));
                            this.observe_draft_editor(cx);
                        }
                        this.update_entries(false, cx);
                    }
                    AgentPanelEvent::ThreadFocused => {
                        if is_active_panel {
                            let new_focused = agent_panel
                                .read(cx)
                                .active_conversation()
                                .and_then(|cv| cv.read(cx).parent_id(cx));
                            if new_focused.is_some() && new_focused != this.focused_thread {
                                this.focused_thread = new_focused;
                                this.update_entries(false, cx);
                            }
                        }
                    }
                    AgentPanelEvent::BackgroundThreadChanged => {
                        this.update_entries(false, cx);
                    }
                }
            },
        )
        .detach();
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
                let cv = panel.read(cx).active_conversation()?;
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
        let conversation_view = panel.read(cx).active_conversation()?;
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

    fn all_thread_infos_for_workspace(
        workspace: &Entity<Workspace>,
        cx: &App,
    ) -> Vec<ActiveThreadInfo> {
        let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) else {
            return Vec::new();
        };
        let agent_panel_ref = agent_panel.read(cx);

        agent_panel_ref
            .parent_threads(cx)
            .into_iter()
            .map(|thread_view| {
                let thread_view_ref = thread_view.read(cx);
                let thread = thread_view_ref.thread.read(cx);

                let icon = thread_view_ref.agent_icon;
                let icon_from_external_svg = thread_view_ref.agent_icon_from_external_svg.clone();
                let title = thread.title();
                let is_native = thread_view_ref.as_native_thread(cx).is_some();
                let is_title_generating = is_native && thread.has_provisional_title();
                let session_id = thread.session_id().clone();
                let is_background = agent_panel_ref.is_background_thread(&session_id);

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
            })
            .collect()
    }

    fn rebuild_contents(&mut self, thread_entries: Vec<ThreadMetadata>, cx: &App) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };
        let mw = multi_workspace.read(cx);
        let workspaces = mw.workspaces().to_vec();
        let active_workspace = mw.workspaces().get(mw.active_workspace_index()).cloned();

        let mut threads_by_paths: HashMap<PathList, Vec<ThreadMetadata>> = HashMap::new();
        for row in thread_entries {
            threads_by_paths
                .entry(row.folder_paths.clone())
                .or_default()
                .push(row);
        }

        // Build a lookup for agent icons from the first workspace's AgentServerStore.
        let agent_server_store = workspaces
            .first()
            .map(|ws| ws.read(cx).project().read(cx).agent_server_store().clone());

        let query = self.filter_editor.read(cx).text(cx);

        let previous = mem::take(&mut self.contents);

        // Collect the session IDs that were visible before this rebuild so we
        // can distinguish a thread that was deleted/removed (was in the list,
        // now gone) from a brand-new thread that hasn't been saved to the
        // metadata store yet (never was in the list).
        let previous_session_ids: HashSet<acp::SessionId> = previous
            .entries
            .iter()
            .filter_map(|entry| match entry {
                ListEntry::Thread(t) => Some(t.session_info.session_id.clone()),
                _ => None,
            })
            .collect();

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
        // Track all session IDs we add to entries so we can prune stale
        // notifications without a separate pass at the end.
        let mut current_session_ids: HashSet<acp::SessionId> = HashSet::new();
        // Compute active_entry_index inline during the build pass.
        let mut active_entry_index: Option<usize> = None;

        // Identify absorbed workspaces in a single pass. A workspace is
        // "absorbed" when it points at a git worktree checkout whose main
        // repo is open as another workspace — its threads appear under the
        // main repo's header instead of getting their own.
        let mut main_repo_workspace: HashMap<Arc<Path>, usize> = HashMap::new();
        let mut absorbed: HashMap<usize, (usize, SharedString)> = HashMap::new();
        let mut pending: HashMap<Arc<Path>, Vec<(usize, SharedString, Arc<Path>)>> = HashMap::new();
        let mut absorbed_workspace_by_path: HashMap<Arc<Path>, usize> = HashMap::new();

        for (i, workspace) in workspaces.iter().enumerate() {
            for snapshot in root_repository_snapshots(workspace, cx) {
                if snapshot.work_directory_abs_path == snapshot.original_repo_abs_path {
                    main_repo_workspace
                        .entry(snapshot.work_directory_abs_path.clone())
                        .or_insert(i);
                    if let Some(waiting) = pending.remove(&snapshot.work_directory_abs_path) {
                        for (ws_idx, name, ws_path) in waiting {
                            absorbed.insert(ws_idx, (i, name));
                            absorbed_workspace_by_path.insert(ws_path, ws_idx);
                        }
                    }
                } else {
                    let name: SharedString = snapshot
                        .work_directory_abs_path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                        .into();
                    if let Some(&main_idx) =
                        main_repo_workspace.get(&snapshot.original_repo_abs_path)
                    {
                        absorbed.insert(i, (main_idx, name));
                        absorbed_workspace_by_path
                            .insert(snapshot.work_directory_abs_path.clone(), i);
                    } else {
                        pending
                            .entry(snapshot.original_repo_abs_path.clone())
                            .or_default()
                            .push((i, name, snapshot.work_directory_abs_path.clone()));
                    }
                }
            }
        }

        for (ws_index, workspace) in workspaces.iter().enumerate() {
            if absorbed.contains_key(&ws_index) {
                continue;
            }

            let path_list = workspace_path_list(workspace, cx);
            let label = workspace_label_from_path_list(&path_list);

            let is_collapsed = self.collapsed_groups.contains(&path_list);
            let should_load_threads = !is_collapsed || !query.is_empty();

            let mut threads: Vec<ThreadEntry> = Vec::new();

            if should_load_threads {
                let mut seen_session_ids: HashSet<acp::SessionId> = HashSet::new();

                // Read threads from SidebarDb for this workspace's path list.
                if let Some(rows) = threads_by_paths.get(&path_list) {
                    for row in rows {
                        seen_session_ids.insert(row.session_id.clone());
                        let (agent, icon, icon_from_external_svg) = match &row.agent_id {
                            None => (Agent::NativeAgent, IconName::ZedAgent, None),
                            Some(id) => {
                                let custom_icon = agent_server_store
                                    .as_ref()
                                    .and_then(|store| store.read(cx).agent_icon(&id));
                                (
                                    Agent::Custom { id: id.clone() },
                                    IconName::Terminal,
                                    custom_icon,
                                )
                            }
                        };
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
                            worktree_name: None,
                            worktree_highlight_positions: Vec::new(),
                            diff_stats: DiffStats::default(),
                        });
                    }
                }

                // Load threads from linked git worktrees of this workspace's repos.
                {
                    let mut linked_worktree_queries: Vec<(PathList, SharedString, Arc<Path>)> =
                        Vec::new();
                    for snapshot in root_repository_snapshots(workspace, cx) {
                        if snapshot.work_directory_abs_path != snapshot.original_repo_abs_path {
                            continue;
                        }
                        for git_worktree in snapshot.linked_worktrees() {
                            let name = git_worktree
                                .path
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                            linked_worktree_queries.push((
                                PathList::new(std::slice::from_ref(&git_worktree.path)),
                                name.into(),
                                Arc::from(git_worktree.path.as_path()),
                            ));
                        }
                    }

                    for (worktree_path_list, worktree_name, worktree_path) in
                        &linked_worktree_queries
                    {
                        let target_workspace =
                            match absorbed_workspace_by_path.get(worktree_path.as_ref()) {
                                Some(&idx) => ThreadEntryWorkspace::Open(workspaces[idx].clone()),
                                None => ThreadEntryWorkspace::Closed(worktree_path_list.clone()),
                            };

                        if let Some(rows) = threads_by_paths.get(worktree_path_list) {
                            for row in rows {
                                if !seen_session_ids.insert(row.session_id.clone()) {
                                    continue;
                                }
                                let (agent, icon, icon_from_external_svg) = match &row.agent_id {
                                    None => (Agent::NativeAgent, IconName::ZedAgent, None),
                                    Some(name) => {
                                        let custom_icon =
                                            agent_server_store.as_ref().and_then(|store| {
                                                store
                                                    .read(cx)
                                                    .agent_icon(&AgentId(name.clone().into()))
                                            });
                                        (
                                            Agent::Custom {
                                                id: AgentId::new(name.clone()),
                                            },
                                            IconName::Terminal,
                                            custom_icon,
                                        )
                                    }
                                };
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
                                    workspace: target_workspace.clone(),
                                    is_live: false,
                                    is_background: false,
                                    is_title_generating: false,
                                    highlight_positions: Vec::new(),
                                    worktree_name: Some(worktree_name.clone()),
                                    worktree_highlight_positions: Vec::new(),
                                    diff_stats: DiffStats::default(),
                                });
                            }
                        }
                    }
                }

                let live_infos = Self::all_thread_infos_for_workspace(workspace, cx);

                if !live_infos.is_empty() {
                    let thread_index_by_session: HashMap<acp::SessionId, usize> = threads
                        .iter()
                        .enumerate()
                        .map(|(i, t)| (t.session_info.session_id.clone(), i))
                        .collect();

                    for info in &live_infos {
                        let Some(&idx) = thread_index_by_session.get(&info.session_id) else {
                            continue;
                        };

                        let thread = &mut threads[idx];
                        thread.session_info.title = Some(info.title.clone());
                        thread.status = info.status;
                        thread.icon = info.icon;
                        thread.icon_from_external_svg = info.icon_from_external_svg.clone();
                        thread.is_live = true;
                        thread.is_background = info.is_background;
                        thread.is_title_generating = info.is_title_generating;
                        thread.diff_stats = info.diff_stats;
                    }
                }

                // Update notification state for live threads in the same pass.
                let is_active_workspace = active_workspace
                    .as_ref()
                    .is_some_and(|active| active == workspace);

                for thread in &threads {
                    let session_id = &thread.session_info.session_id;
                    if thread.is_background && thread.status == AgentThreadStatus::Completed {
                        notified_threads.insert(session_id.clone());
                    } else if thread.status == AgentThreadStatus::Completed
                        && !is_active_workspace
                        && old_statuses.get(session_id) == Some(&AgentThreadStatus::Running)
                    {
                        notified_threads.insert(session_id.clone());
                    }

                    if is_active_workspace && !thread.is_background {
                        notified_threads.remove(session_id);
                    }
                }

                // Sort by created_at (newest first), falling back to updated_at
                // for threads without a created_at (e.g., ACP sessions).
                threads.sort_by(|a, b| {
                    let a_time = a.session_info.created_at.or(a.session_info.updated_at);
                    let b_time = b.session_info.created_at.or(b.session_info.updated_at);
                    b_time.cmp(&a_time)
                });
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
                    if let Some(worktree_name) = &thread.worktree_name {
                        if let Some(positions) = fuzzy_match_positions(&query, worktree_name) {
                            thread.worktree_highlight_positions = positions;
                        }
                    }
                    let worktree_matched = !thread.worktree_highlight_positions.is_empty();
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

                entries.push(ListEntry::ProjectHeader {
                    path_list: path_list.clone(),
                    label,
                    workspace: workspace.clone(),
                    highlight_positions: workspace_highlight_positions,
                });

                // Track session IDs and compute active_entry_index as we add
                // thread entries.
                for thread in matched_threads {
                    current_session_ids.insert(thread.session_info.session_id.clone());
                    if active_entry_index.is_none() {
                        if let Some(focused) = &self.focused_thread {
                            if &thread.session_info.session_id == focused {
                                active_entry_index = Some(entries.len());
                            }
                        }
                    }
                    entries.push(thread.into());
                }
            } else {
                entries.push(ListEntry::ProjectHeader {
                    path_list: path_list.clone(),
                    label,
                    workspace: workspace.clone(),
                    highlight_positions: Vec::new(),
                });

                if is_collapsed {
                    continue;
                }

                entries.push(ListEntry::NewThread {
                    path_list: path_list.clone(),
                    workspace: workspace.clone(),
                });

                let total = threads.len();

                let extra_batches = self.expanded_groups.get(&path_list).copied().unwrap_or(0);
                let threads_to_show =
                    DEFAULT_THREADS_SHOWN + (extra_batches * DEFAULT_THREADS_SHOWN);
                let count = threads_to_show.min(total);
                let is_fully_expanded = count >= total;

                // Track session IDs and compute active_entry_index as we add
                // thread entries.
                for thread in threads.into_iter().take(count) {
                    current_session_ids.insert(thread.session_info.session_id.clone());
                    if active_entry_index.is_none() {
                        if let Some(focused) = &self.focused_thread {
                            if &thread.session_info.session_id == focused {
                                active_entry_index = Some(entries.len());
                            }
                        }
                    }
                    entries.push(thread.into());
                }

                if total > DEFAULT_THREADS_SHOWN {
                    entries.push(ListEntry::ViewMore {
                        path_list: path_list.clone(),
                        remaining_count: total.saturating_sub(count),
                        is_fully_expanded,
                    });
                }
            }
        }

        // Prune stale notifications using the session IDs we collected during
        // the build pass (no extra scan needed).
        notified_threads.retain(|id| current_session_ids.contains(id));

        let project_header_indices = entries
            .iter()
            .enumerate()
            .filter_map(|(i, e)| matches!(e, ListEntry::ProjectHeader { .. }).then_some(i))
            .collect();

        // If focused_thread points to a thread that was previously in the
        // list but is now gone (deleted, or its workspace was removed), clear
        // it. We don't try to redirect to a thread in a different project
        // group — the delete_thread method already handles within-group
        // neighbor selection. If it was never in the list it's a brand-new
        // thread that hasn't been saved to the metadata store yet — leave
        // things alone and wait for the next rebuild.
        let focused_thread_was_known = self
            .focused_thread
            .as_ref()
            .is_some_and(|id| previous_session_ids.contains(id));

        if focused_thread_was_known && active_entry_index.is_none() {
            self.focused_thread = None;
        }

        self.active_entry_index = active_entry_index;
        self.contents = SidebarContents {
            entries,
            notified_threads,
            project_header_indices,
        };
    }

    fn update_entries(&mut self, select_first_thread: bool, cx: &mut Context<Self>) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };
        if !multi_workspace.read(cx).multi_workspace_enabled(cx) {
            return;
        }

        let had_notifications = self.has_notifications(cx);

        let scroll_position = self.list_state.logical_scroll_top();

        let list_thread_entries_task = ThreadMetadataStore::global(cx).read(cx).list(cx);

        self._update_entries_task.take();
        self._update_entries_task = Some(cx.spawn(async move |this, cx| {
            let Some(thread_entries) = list_thread_entries_task.await.log_err() else {
                return;
            };
            this.update(cx, |this, cx| {
                this.rebuild_contents(thread_entries, cx);

                if select_first_thread {
                    this.selection = this
                        .contents
                        .entries
                        .iter()
                        .position(|entry| matches!(entry, ListEntry::Thread(_)))
                        .or_else(|| {
                            if this.contents.entries.is_empty() {
                                None
                            } else {
                                Some(0)
                            }
                        });
                }

                this.list_state.reset(this.contents.entries.len());
                this.list_state.scroll_to(scroll_position);

                if had_notifications != this.has_notifications(cx) {
                    multi_workspace.update(cx, |_, cx| {
                        cx.notify();
                    });
                }

                cx.notify();
            })
            .ok();
        }));
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
            } => self.render_project_header(
                ix,
                false,
                path_list,
                label,
                workspace,
                highlight_positions,
                is_selected,
                cx,
            ),
            ListEntry::Thread(thread) => self.render_thread(ix, thread, is_selected, cx),
            ListEntry::ViewMore {
                path_list,
                remaining_count,
                is_fully_expanded,
            } => self.render_view_more(
                ix,
                path_list,
                *remaining_count,
                *is_fully_expanded,
                is_selected,
                cx,
            ),
            ListEntry::NewThread {
                path_list,
                workspace,
            } => self.render_new_thread(ix, path_list, workspace, is_selected, cx),
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
        let workspace_for_remove = workspace.clone();

        let path_list_for_toggle = path_list.clone();
        let path_list_for_collapse = path_list.clone();
        let view_more_expanded = self.expanded_groups.contains_key(path_list);

        let multi_workspace = self.multi_workspace.upgrade();
        let workspace_count = multi_workspace
            .as_ref()
            .map_or(0, |mw| mw.read(cx).workspaces().len());

        let label = if highlight_positions.is_empty() {
            Label::new(label.clone())
                .size(LabelSize::Small)
                .color(Color::Muted)
                .into_any_element()
        } else {
            HighlightedLabel::new(label.clone(), highlight_positions.to_vec())
                .size(LabelSize::Small)
                .color(Color::Muted)
                .into_any_element()
        };

        ListItem::new(id)
            .group_name(group_name)
            .focused(is_selected)
            .child(
                h_flex()
                    .relative()
                    .min_w_0()
                    .w_full()
                    .py_1()
                    .gap_1p5()
                    .child(
                        Icon::new(disclosure_icon)
                            .size(IconSize::Small)
                            .color(Color::Custom(cx.theme().colors().icon_muted.opacity(0.6))),
                    )
                    .child(label),
            )
            .end_hover_gradient_overlay(true)
            .end_hover_slot(
                h_flex()
                    .gap_1()
                    .when(workspace_count > 1, |this| {
                        this.child(
                            IconButton::new(
                                SharedString::from(format!(
                                    "{id_prefix}project-header-remove-{ix}",
                                )),
                                IconName::Close,
                            )
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .tooltip(Tooltip::text("Remove Project"))
                            .on_click(cx.listener(
                                move |this, _, window, cx| {
                                    this.remove_workspace(&workspace_for_remove, window, cx);
                                },
                            )),
                        )
                    })
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
                                    this.update_entries(false, cx);
                                }
                            })),
                        )
                    }),
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                this.selection = None;
                this.toggle_collapse(&path_list_for_toggle, window, cx);
            }))
            // TODO: Decide if we really want the header to be activating different workspaces
            // .on_click(cx.listener(move |this, _, window, cx| {
            //     this.selection = None;
            //     this.activate_workspace(&workspace_for_activate, window, cx);
            // }))
            .into_any_element()
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
            &workspace,
            &highlight_positions,
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
            .blend(color.panel_background.opacity(0.8));

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

    fn prune_stale_worktree_workspaces(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };
        let workspaces = multi_workspace.read(cx).workspaces().to_vec();

        // Collect all worktree paths that are currently listed by any main
        // repo open in any workspace.
        let mut known_worktree_paths: HashSet<std::path::PathBuf> = HashSet::new();
        for workspace in &workspaces {
            for snapshot in root_repository_snapshots(workspace, cx) {
                if snapshot.work_directory_abs_path != snapshot.original_repo_abs_path {
                    continue;
                }
                for git_worktree in snapshot.linked_worktrees() {
                    known_worktree_paths.insert(git_worktree.path.to_path_buf());
                }
            }
        }

        // Find workspaces that consist of exactly one root folder which is a
        // stale worktree checkout. Multi-root workspaces are never pruned —
        // losing one worktree shouldn't destroy a workspace that also
        // contains other folders.
        let mut to_remove: Vec<Entity<Workspace>> = Vec::new();
        for workspace in &workspaces {
            let path_list = workspace_path_list(workspace, cx);
            if path_list.paths().len() != 1 {
                continue;
            }
            let should_prune = root_repository_snapshots(workspace, cx)
                .iter()
                .any(|snapshot| {
                    snapshot.work_directory_abs_path != snapshot.original_repo_abs_path
                        && !known_worktree_paths.contains(snapshot.work_directory_abs_path.as_ref())
                });
            if should_prune {
                to_remove.push(workspace.clone());
            }
        }

        for workspace in &to_remove {
            self.remove_workspace(workspace, window, cx);
        }
    }

    fn remove_workspace(
        &mut self,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        multi_workspace.update(cx, |multi_workspace, cx| {
            let Some(index) = multi_workspace
                .workspaces()
                .iter()
                .position(|w| w == workspace)
            else {
                return;
            };
            multi_workspace.remove_workspace(index, window, cx);
        });
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
        self.update_entries(false, cx);
    }

    fn focus_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.selection.is_none() {
            self.filter_editor.focus_handle(cx).focus(window, cx);
        }
    }

    fn cancel(&mut self, _: &Cancel, window: &mut Window, cx: &mut Context<Self>) {
        if self.reset_filter_editor_text(window, cx) {
            self.update_entries(false, cx);
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
        self.filter_editor.focus_handle(cx).focus(window, cx);
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
                self.update_entries(false, cx);
            }
            ListEntry::NewThread { workspace, .. } => {
                let workspace = workspace.clone();
                self.create_new_thread(&workspace, window, cx);
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
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.activate(workspace.clone(), cx);
        });

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

        self.update_entries(false, cx);
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

    fn find_open_workspace_for_path_list(
        &self,
        path_list: &PathList,
        cx: &App,
    ) -> Option<Entity<Workspace>> {
        let multi_workspace = self.multi_workspace.upgrade()?;
        multi_workspace
            .read(cx)
            .workspaces()
            .iter()
            .find(|workspace| workspace_path_list(workspace, cx).paths() == path_list.paths())
            .cloned()
    }

    fn activate_archived_thread(
        &mut self,
        agent: Agent,
        session_info: acp_thread::AgentSessionInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(path_list) = &session_info.work_dirs {
            if let Some(workspace) = self.find_open_workspace_for_path_list(&path_list, cx) {
                self.activate_thread(agent, session_info, &workspace, window, cx);
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
            self.activate_thread(agent, session_info, &workspace, window, cx);
        }
    }

    fn expand_selected_entry(
        &mut self,
        _: &ExpandSelectedEntry,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else { return };

        match self.contents.entries.get(ix) {
            Some(ListEntry::ProjectHeader { path_list, .. }) => {
                if self.collapsed_groups.contains(path_list) {
                    let path_list = path_list.clone();
                    self.collapsed_groups.remove(&path_list);
                    self.update_entries(false, cx);
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
        _: &CollapseSelectedEntry,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else { return };

        match self.contents.entries.get(ix) {
            Some(ListEntry::ProjectHeader { path_list, .. }) => {
                if !self.collapsed_groups.contains(path_list) {
                    let path_list = path_list.clone();
                    self.collapsed_groups.insert(path_list);
                    self.update_entries(false, cx);
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
                        self.update_entries(false, cx);
                        break;
                    }
                }
            }
            None => {}
        }
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

    fn delete_thread(
        &mut self,
        session_id: &acp::SessionId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // If we're deleting the currently focused thread, move focus to the
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
            // might be deleting a thread in a non-active group.
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

                if let Some(workspace) = &group_workspace {
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

        let Some(thread_store) = ThreadStore::try_global(cx) else {
            return;
        };
        thread_store.update(cx, |store, cx| {
            store
                .delete_thread(session_id.clone(), cx)
                .detach_and_log_err(cx);
        });

        ThreadMetadataStore::global(cx)
            .update(cx, |store, cx| store.delete(session_id.clone(), cx))
            .detach_and_log_err(cx);
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
        self.delete_thread(&session_id, window, cx);
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
        let is_selected = self.focused_thread.as_ref() == Some(&session_info.session_id);
        let is_running = matches!(
            thread.status,
            AgentThreadStatus::Running | AgentThreadStatus::WaitingForConfirmation
        );
        let can_delete = thread.agent == Agent::NativeAgent;
        let session_id_for_delete = thread.session_info.session_id.clone();
        let focus_handle = self.focus_handle.clone();

        let id = SharedString::from(format!("thread-entry-{}", ix));

        let timestamp = thread
            .session_info
            .created_at
            .or(thread.session_info.updated_at)
            .map(|entry_time| {
                let now = Utc::now();
                let duration = now.signed_duration_since(entry_time);

                let minutes = duration.num_minutes();
                let hours = duration.num_hours();
                let days = duration.num_days();
                let weeks = days / 7;
                let months = days / 30;

                if minutes < 60 {
                    format!("{}m", minutes.max(1))
                } else if hours < 24 {
                    format!("{}h", hours)
                } else if weeks < 4 {
                    format!("{}w", weeks.max(1))
                } else {
                    format!("{}mo", months.max(1))
                }
            });

        ThreadItem::new(id, title)
            .icon(thread.icon)
            .when_some(thread.icon_from_external_svg.clone(), |this, svg| {
                this.custom_icon_from_external_svg(svg)
            })
            .when_some(thread.worktree_name.clone(), |this, name| {
                this.worktree(name)
            })
            .worktree_highlight_positions(thread.worktree_highlight_positions.clone())
            .when_some(timestamp, |this, ts| this.timestamp(ts))
            .highlight_positions(thread.highlight_positions.to_vec())
            .status(thread.status)
            .generating_title(thread.is_title_generating)
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
            .when(is_hovered && can_delete && !is_running, |this| {
                this.action_slot(
                    IconButton::new("delete-thread", IconName::Trash)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .tooltip({
                            let focus_handle = focus_handle.clone();
                            move |_window, cx| {
                                Tooltip::for_action_in(
                                    "Delete Thread",
                                    &RemoveSelectedThread,
                                    &focus_handle,
                                    cx,
                                )
                            }
                        })
                        .on_click({
                            let session_id = session_id_for_delete.clone();
                            cx.listener(move |this, _, window, cx| {
                                this.delete_thread(&session_id, window, cx);
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
        let workspace = self
            .multi_workspace
            .upgrade()
            .map(|mw| mw.read(cx).workspace().downgrade());

        let focus_handle = workspace
            .as_ref()
            .and_then(|ws| ws.upgrade())
            .map(|w| w.read(cx).focus_handle(cx))
            .unwrap_or_else(|| cx.focus_handle());

        let popover_handle = self.recent_projects_popover_handle.clone();

        PopoverMenu::new("sidebar-recent-projects-menu")
            .with_handle(popover_handle)
            .menu(move |window, cx| {
                workspace.as_ref().map(|ws| {
                    RecentProjects::popover(ws.clone(), false, focus_handle.clone(), window, cx)
                })
            })
            .trigger_with_tooltip(
                IconButton::new("open-project", IconName::OpenFolder)
                    .icon_size(IconSize::Small)
                    .selected_style(ButtonStyle::Tinted(TintColor::Accent)),
                |_window, cx| {
                    Tooltip::for_action(
                        "Recent Projects",
                        &OpenRecent {
                            create_new_window: false,
                        },
                        cx,
                    )
                },
            )
            .anchor(gpui::Corner::TopLeft)
    }

    fn render_view_more(
        &self,
        ix: usize,
        path_list: &PathList,
        remaining_count: usize,
        is_fully_expanded: bool,
        is_selected: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let path_list = path_list.clone();
        let id = SharedString::from(format!("view-more-{}", ix));

        let icon = if is_fully_expanded {
            IconName::ListCollapse
        } else {
            IconName::Plus
        };

        let label: SharedString = if is_fully_expanded {
            "Collapse".into()
        } else if remaining_count > 0 {
            format!("View More ({})", remaining_count).into()
        } else {
            "View More".into()
        };

        ThreadItem::new(id, label)
            .icon(icon)
            .focused(is_selected)
            .title_label_color(Color::Custom(cx.theme().colors().text.opacity(0.85)))
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.selection = None;
                if is_fully_expanded {
                    this.expanded_groups.remove(&path_list);
                } else {
                    let current = this.expanded_groups.get(&path_list).copied().unwrap_or(0);
                    this.expanded_groups.insert(path_list.clone(), current + 1);
                }
                this.update_entries(false, cx);
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
        is_selected: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let is_active = self.active_entry_index.is_none()
            && self
                .multi_workspace
                .upgrade()
                .map_or(false, |mw| mw.read(cx).workspace() == workspace);

        let label: SharedString = if is_active {
            self.active_draft_text(cx)
                .unwrap_or_else(|| "New Thread".into())
        } else {
            "New Thread".into()
        };

        let workspace = workspace.clone();
        let id = SharedString::from(format!("new-thread-btn-{}", ix));

        ThreadItem::new(id, label)
            .icon(IconName::Plus)
            .selected(is_active)
            .focused(is_selected)
            .title_label_color(Color::Custom(cx.theme().colors().text.opacity(0.85)))
            .on_click(cx.listener(move |this, _, window, cx| {
                this.selection = None;
                this.create_new_thread(&workspace, window, cx);
            }))
            .into_any_element()
    }

    fn render_sidebar_header(&self, window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_query = self.has_filter_query(cx);
        let traffic_lights = cfg!(target_os = "macos") && !window.is_fullscreen();
        let header_height = platform_title_bar_height(window);

        v_flex()
            .child(
                h_flex()
                    .h(header_height)
                    .mt_px()
                    .pb_px()
                    .when(traffic_lights, |this| {
                        this.pl(px(ui::utils::TRAFFIC_LIGHT_PADDING))
                    })
                    .pr_1p5()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .justify_between()
                    .child(self.render_sidebar_toggle_button(cx))
                    .child(
                        h_flex()
                            .gap_0p5()
                            .child(
                                IconButton::new("archive", IconName::Archive)
                                    .icon_size(IconSize::Small)
                                    .tooltip(Tooltip::text("View Archived Threads"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.show_archive(window, cx);
                                    })),
                            )
                            .child(self.render_recent_projects_button(cx)),
                    ),
            )
            .child(
                h_flex()
                    .h(Tab::container_height(cx))
                    .px_1p5()
                    .gap_1p5()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        h_flex().size_4().flex_none().justify_center().child(
                            Icon::new(IconName::MagnifyingGlass)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                    .child(self.render_filter_input(cx))
                    .child(
                        h_flex()
                            .gap_1()
                            .when(self.selection.is_some(), |this| {
                                this.child(KeyBinding::for_action(&FocusSidebarFilter, cx))
                            })
                            .when(has_query, |this| {
                                this.child(
                                    IconButton::new("clear_filter", IconName::Close)
                                        .icon_size(IconSize::Small)
                                        .tooltip(Tooltip::text("Clear Search"))
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.reset_filter_editor_text(window, cx);
                                            this.update_entries(false, cx);
                                        })),
                                )
                            }),
                    ),
            )
    }

    fn render_sidebar_toggle_button(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let icon = IconName::ThreadsSidebarLeftOpen;

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
                window.dispatch_action(ToggleWorkspaceSidebar.boxed_clone(), cx);
            })
    }
}

impl Sidebar {
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
                ThreadsArchiveViewEvent::OpenThread {
                    agent,
                    session_info,
                } => {
                    this.show_thread_list(window, cx);
                    this.activate_archived_thread(agent.clone(), session_info.clone(), window, cx);
                }
            },
        );

        self._subscriptions.push(subscription);
        self.archive_view = Some(archive_view);
        self.view = SidebarView::Archive;
        cx.notify();
    }

    fn show_thread_list(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.view = SidebarView::ThreadList;
        self.archive_view = None;
        self._subscriptions.clear();
        window.focus(&self.focus_handle, cx);
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

    fn toggle_recent_projects_popover(&self, window: &mut Window, cx: &mut App) {
        self.recent_projects_popover_handle.toggle(window, cx);
    }

    fn is_recent_projects_popover_deployed(&self) -> bool {
        self.recent_projects_popover_handle.is_deployed()
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
        let ui_font = theme::setup_ui_font(window, cx);
        let sticky_header = self.render_sticky_header(window, cx);
        let bg = cx
            .theme()
            .colors()
            .title_bar_background
            .blend(cx.theme().colors().panel_background.opacity(0.8));

        v_flex()
            .id("workspace-sidebar")
            .key_context("ThreadsSidebar")
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
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::remove_selected_thread))
            .on_action(cx.listener(Self::new_thread_in_group))
            .on_action(cx.listener(Self::focus_sidebar_filter))
            .font(ui_font)
            .h_full()
            .w(self.width)
            .bg(bg)
            .border_r_1()
            .border_color(cx.theme().colors().border)
            .map(|this| match self.view {
                SidebarView::ThreadList => {
                    this.child(self.render_sidebar_header(window, cx)).child(
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
                            .when_some(sticky_header, |this, header| this.child(header))
                            .vertical_scrollbar_for(&self.list_state, window, cx),
                    )
                }
                SidebarView::Archive => {
                    if let Some(archive_view) = &self.archive_view {
                        this.child(archive_view.clone())
                    } else {
                        this
                    }
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use acp_thread::StubAgentConnection;
    use agent::ThreadStore;
    use agent_ui::test_support::{active_session_id, open_thread_with_connection, send_message};
    use assistant_text_thread::TextThreadStore;
    use chrono::DateTime;
    use feature_flags::FeatureFlagAppExt as _;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use pretty_assertions::assert_eq;
    use settings::SettingsStore;
    use std::{path::PathBuf, sync::Arc};
    use util::path_list::PathList;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            cx.update_flags(false, vec!["agent-v2".into()]);
            ThreadStore::init_global(cx);
            ThreadMetadataStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            prompt_store::init(cx);
        });
    }

    async fn init_test_project(
        worktree_path: &str,
        cx: &mut TestAppContext,
    ) -> Entity<project::Project> {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(worktree_path, serde_json::json!({ "src": {} }))
            .await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
        project::Project::test(fs, [worktree_path.as_ref()], cx).await
    }

    fn setup_sidebar(
        multi_workspace: &Entity<MultiWorkspace>,
        cx: &mut gpui::VisualTestContext,
    ) -> Entity<Sidebar> {
        let multi_workspace = multi_workspace.clone();
        let sidebar =
            cx.update(|window, cx| cx.new(|cx| Sidebar::new(multi_workspace.clone(), window, cx)));
        multi_workspace.update(cx, |mw, _cx| {
            mw.register_sidebar(sidebar.clone());
        });
        cx.run_until_parked();
        sidebar
    }

    async fn save_n_test_threads(
        count: u32,
        path_list: &PathList,
        cx: &mut gpui::VisualTestContext,
    ) {
        for i in 0..count {
            save_thread_metadata(
                acp::SessionId::new(Arc::from(format!("thread-{}", i))),
                format!("Thread {}", i + 1).into(),
                chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, i).unwrap(),
                path_list.clone(),
                cx,
            )
            .await;
        }
        cx.run_until_parked();
    }

    async fn save_test_thread_metadata(
        session_id: &acp::SessionId,
        path_list: PathList,
        cx: &mut TestAppContext,
    ) {
        save_thread_metadata(
            session_id.clone(),
            "Test".into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
            path_list,
            cx,
        )
        .await;
    }

    async fn save_named_thread_metadata(
        session_id: &str,
        title: &str,
        path_list: &PathList,
        cx: &mut gpui::VisualTestContext,
    ) {
        save_thread_metadata(
            acp::SessionId::new(Arc::from(session_id)),
            SharedString::from(title.to_string()),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
            path_list.clone(),
            cx,
        )
        .await;
        cx.run_until_parked();
    }

    async fn save_thread_metadata(
        session_id: acp::SessionId,
        title: SharedString,
        updated_at: DateTime<Utc>,
        path_list: PathList,
        cx: &mut TestAppContext,
    ) {
        let metadata = ThreadMetadata {
            session_id,
            agent_id: None,
            title,
            updated_at,
            created_at: None,
            folder_paths: path_list,
        };
        let task = cx.update(|cx| {
            ThreadMetadataStore::global(cx).update(cx, |store, cx| store.save(metadata, cx))
        });
        task.await.unwrap();
    }

    fn open_and_focus_sidebar(sidebar: &Entity<Sidebar>, cx: &mut gpui::VisualTestContext) {
        let multi_workspace = sidebar.read_with(cx, |s, _| s.multi_workspace.upgrade());
        if let Some(multi_workspace) = multi_workspace {
            multi_workspace.update_in(cx, |mw, window, cx| {
                if !mw.sidebar_open() {
                    mw.toggle_sidebar(window, cx);
                }
            });
        }
        cx.run_until_parked();
        sidebar.update_in(cx, |_, window, cx| {
            cx.focus_self(window);
        });
        cx.run_until_parked();
    }

    fn visible_entries_as_strings(
        sidebar: &Entity<Sidebar>,
        cx: &mut gpui::VisualTestContext,
    ) -> Vec<String> {
        sidebar.read_with(cx, |sidebar, _cx| {
            sidebar
                .contents
                .entries
                .iter()
                .enumerate()
                .map(|(ix, entry)| {
                    let selected = if sidebar.selection == Some(ix) {
                        "  <== selected"
                    } else {
                        ""
                    };
                    match entry {
                        ListEntry::ProjectHeader {
                            label,
                            path_list,
                            highlight_positions: _,
                            ..
                        } => {
                            let icon = if sidebar.collapsed_groups.contains(path_list) {
                                ">"
                            } else {
                                "v"
                            };
                            format!("{} [{}]{}", icon, label, selected)
                        }
                        ListEntry::Thread(thread) => {
                            let title = thread
                                .session_info
                                .title
                                .as_ref()
                                .map(|s| s.as_ref())
                                .unwrap_or("Untitled");
                            let active = if thread.is_live { " *" } else { "" };
                            let status_str = match thread.status {
                                AgentThreadStatus::Running => " (running)",
                                AgentThreadStatus::Error => " (error)",
                                AgentThreadStatus::WaitingForConfirmation => " (waiting)",
                                _ => "",
                            };
                            let notified = if sidebar
                                .contents
                                .is_thread_notified(&thread.session_info.session_id)
                            {
                                " (!)"
                            } else {
                                ""
                            };
                            let worktree = thread
                                .worktree_name
                                .as_ref()
                                .map(|name| format!(" {{{}}}", name))
                                .unwrap_or_default();
                            format!(
                                "  {}{}{}{}{}{}",
                                title, worktree, active, status_str, notified, selected
                            )
                        }
                        ListEntry::ViewMore {
                            remaining_count,
                            is_fully_expanded,
                            ..
                        } => {
                            if *is_fully_expanded {
                                format!("  - Collapse{}", selected)
                            } else {
                                format!("  + View More ({}){}", remaining_count, selected)
                            }
                        }
                        ListEntry::NewThread { .. } => {
                            format!("  [+ New Thread]{}", selected)
                        }
                    }
                })
                .collect()
        })
    }

    #[test]
    fn test_clean_mention_links() {
        // Simple mention link
        assert_eq!(
            Sidebar::clean_mention_links("check [@Button.tsx](file:///path/to/Button.tsx)"),
            "check @Button.tsx"
        );

        // Multiple mention links
        assert_eq!(
            Sidebar::clean_mention_links(
                "look at [@foo.rs](file:///foo.rs) and [@bar.rs](file:///bar.rs)"
            ),
            "look at @foo.rs and @bar.rs"
        );

        // No mention links — passthrough
        assert_eq!(
            Sidebar::clean_mention_links("plain text with no mentions"),
            "plain text with no mentions"
        );

        // Incomplete link syntax — preserved as-is
        assert_eq!(
            Sidebar::clean_mention_links("broken [@mention without closing"),
            "broken [@mention without closing"
        );

        // Regular markdown link (no @) — not touched
        assert_eq!(
            Sidebar::clean_mention_links("see [docs](https://example.com)"),
            "see [docs](https://example.com)"
        );

        // Empty input
        assert_eq!(Sidebar::clean_mention_links(""), "");
    }

    #[gpui::test]
    async fn test_single_workspace_no_threads(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]", "  [+ New Thread]"]
        );
    }

    #[gpui::test]
    async fn test_single_workspace_with_saved_threads(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

        save_thread_metadata(
            acp::SessionId::new(Arc::from("thread-1")),
            "Fix crash in project panel".into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 3, 0, 0, 0).unwrap(),
            path_list.clone(),
            cx,
        )
        .await;

        save_thread_metadata(
            acp::SessionId::new(Arc::from("thread-2")),
            "Add inline diff view".into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
            path_list.clone(),
            cx,
        )
        .await;
        cx.run_until_parked();

        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  [+ New Thread]",
                "  Fix crash in project panel",
                "  Add inline diff view",
            ]
        );
    }

    #[gpui::test]
    async fn test_workspace_lifecycle(cx: &mut TestAppContext) {
        let project = init_test_project("/project-a", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        // Single workspace with a thread
        let path_list = PathList::new(&[std::path::PathBuf::from("/project-a")]);

        save_thread_metadata(
            acp::SessionId::new(Arc::from("thread-a1")),
            "Thread A1".into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
            path_list.clone(),
            cx,
        )
        .await;
        cx.run_until_parked();

        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [project-a]", "  [+ New Thread]", "  Thread A1"]
        );

        // Add a second workspace
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.create_workspace(window, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [project-a]",
                "  [+ New Thread]",
                "  Thread A1",
                "v [Empty Workspace]",
                "  [+ New Thread]"
            ]
        );

        // Remove the second workspace
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.remove_workspace(1, window, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [project-a]", "  [+ New Thread]", "  Thread A1"]
        );
    }

    #[gpui::test]
    async fn test_view_more_pagination(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
        save_n_test_threads(12, &path_list, cx).await;

        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  [+ New Thread]",
                "  Thread 12",
                "  Thread 11",
                "  Thread 10",
                "  Thread 9",
                "  Thread 8",
                "  + View More (7)",
            ]
        );
    }

    #[gpui::test]
    async fn test_view_more_batched_expansion(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
        // Create 17 threads: initially shows 5, then 10, then 15, then all 17 with Collapse
        save_n_test_threads(17, &path_list, cx).await;

        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        // Initially shows NewThread + 5 threads + View More (12 remaining)
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 8); // header + NewThread + 5 threads + View More
        assert!(entries.iter().any(|e| e.contains("View More (12)")));

        // Focus and navigate to View More, then confirm to expand by one batch
        open_and_focus_sidebar(&sidebar, cx);
        for _ in 0..8 {
            cx.dispatch_action(SelectNext);
        }
        cx.dispatch_action(Confirm);
        cx.run_until_parked();

        // Now shows NewThread + 10 threads + View More (7 remaining)
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 13); // header + NewThread + 10 threads + View More
        assert!(entries.iter().any(|e| e.contains("View More (7)")));

        // Expand again by one batch
        sidebar.update_in(cx, |s, _window, cx| {
            let current = s.expanded_groups.get(&path_list).copied().unwrap_or(0);
            s.expanded_groups.insert(path_list.clone(), current + 1);
            s.update_entries(false, cx);
        });
        cx.run_until_parked();

        // Now shows NewThread + 15 threads + View More (2 remaining)
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 18); // header + NewThread + 15 threads + View More
        assert!(entries.iter().any(|e| e.contains("View More (2)")));

        // Expand one more time - should show all 17 threads with Collapse button
        sidebar.update_in(cx, |s, _window, cx| {
            let current = s.expanded_groups.get(&path_list).copied().unwrap_or(0);
            s.expanded_groups.insert(path_list.clone(), current + 1);
            s.update_entries(false, cx);
        });
        cx.run_until_parked();

        // All 17 threads shown with Collapse button
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 20); // header + NewThread + 17 threads + Collapse
        assert!(!entries.iter().any(|e| e.contains("View More")));
        assert!(entries.iter().any(|e| e.contains("Collapse")));

        // Click collapse - should go back to showing 5 threads
        sidebar.update_in(cx, |s, _window, cx| {
            s.expanded_groups.remove(&path_list);
            s.update_entries(false, cx);
        });
        cx.run_until_parked();

        // Back to initial state: NewThread + 5 threads + View More (12 remaining)
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 8); // header + NewThread + 5 threads + View More
        assert!(entries.iter().any(|e| e.contains("View More (12)")));
    }

    #[gpui::test]
    async fn test_collapse_and_expand_group(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
        save_n_test_threads(1, &path_list, cx).await;

        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]", "  [+ New Thread]", "  Thread 1"]
        );

        // Collapse
        sidebar.update_in(cx, |s, window, cx| {
            s.toggle_collapse(&path_list, window, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["> [my-project]"]
        );

        // Expand
        sidebar.update_in(cx, |s, window, cx| {
            s.toggle_collapse(&path_list, window, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]", "  [+ New Thread]", "  Thread 1"]
        );
    }

    #[gpui::test]
    async fn test_visible_entries_as_strings(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let expanded_path = PathList::new(&[std::path::PathBuf::from("/expanded")]);
        let collapsed_path = PathList::new(&[std::path::PathBuf::from("/collapsed")]);

        sidebar.update_in(cx, |s, _window, _cx| {
            s.collapsed_groups.insert(collapsed_path.clone());
            s.contents
                .notified_threads
                .insert(acp::SessionId::new(Arc::from("t-5")));
            s.contents.entries = vec![
                // Expanded project header
                ListEntry::ProjectHeader {
                    path_list: expanded_path.clone(),
                    label: "expanded-project".into(),
                    workspace: workspace.clone(),
                    highlight_positions: Vec::new(),
                },
                // Thread with default (Completed) status, not active
                ListEntry::Thread(ThreadEntry {
                    agent: Agent::NativeAgent,
                    session_info: acp_thread::AgentSessionInfo {
                        session_id: acp::SessionId::new(Arc::from("t-1")),
                        work_dirs: None,
                        title: Some("Completed thread".into()),
                        updated_at: Some(Utc::now()),
                        created_at: Some(Utc::now()),
                        meta: None,
                    },
                    icon: IconName::ZedAgent,
                    icon_from_external_svg: None,
                    status: AgentThreadStatus::Completed,
                    workspace: ThreadEntryWorkspace::Open(workspace.clone()),
                    is_live: false,
                    is_background: false,
                    is_title_generating: false,
                    highlight_positions: Vec::new(),
                    worktree_name: None,
                    worktree_highlight_positions: Vec::new(),
                    diff_stats: DiffStats::default(),
                }),
                // Active thread with Running status
                ListEntry::Thread(ThreadEntry {
                    agent: Agent::NativeAgent,
                    session_info: acp_thread::AgentSessionInfo {
                        session_id: acp::SessionId::new(Arc::from("t-2")),
                        work_dirs: None,
                        title: Some("Running thread".into()),
                        updated_at: Some(Utc::now()),
                        created_at: Some(Utc::now()),
                        meta: None,
                    },
                    icon: IconName::ZedAgent,
                    icon_from_external_svg: None,
                    status: AgentThreadStatus::Running,
                    workspace: ThreadEntryWorkspace::Open(workspace.clone()),
                    is_live: true,
                    is_background: false,
                    is_title_generating: false,
                    highlight_positions: Vec::new(),
                    worktree_name: None,
                    worktree_highlight_positions: Vec::new(),
                    diff_stats: DiffStats::default(),
                }),
                // Active thread with Error status
                ListEntry::Thread(ThreadEntry {
                    agent: Agent::NativeAgent,
                    session_info: acp_thread::AgentSessionInfo {
                        session_id: acp::SessionId::new(Arc::from("t-3")),
                        work_dirs: None,
                        title: Some("Error thread".into()),
                        updated_at: Some(Utc::now()),
                        created_at: Some(Utc::now()),
                        meta: None,
                    },
                    icon: IconName::ZedAgent,
                    icon_from_external_svg: None,
                    status: AgentThreadStatus::Error,
                    workspace: ThreadEntryWorkspace::Open(workspace.clone()),
                    is_live: true,
                    is_background: false,
                    is_title_generating: false,
                    highlight_positions: Vec::new(),
                    worktree_name: None,
                    worktree_highlight_positions: Vec::new(),
                    diff_stats: DiffStats::default(),
                }),
                // Thread with WaitingForConfirmation status, not active
                ListEntry::Thread(ThreadEntry {
                    agent: Agent::NativeAgent,
                    session_info: acp_thread::AgentSessionInfo {
                        session_id: acp::SessionId::new(Arc::from("t-4")),
                        work_dirs: None,
                        title: Some("Waiting thread".into()),
                        updated_at: Some(Utc::now()),
                        created_at: Some(Utc::now()),
                        meta: None,
                    },
                    icon: IconName::ZedAgent,
                    icon_from_external_svg: None,
                    status: AgentThreadStatus::WaitingForConfirmation,
                    workspace: ThreadEntryWorkspace::Open(workspace.clone()),
                    is_live: false,
                    is_background: false,
                    is_title_generating: false,
                    highlight_positions: Vec::new(),
                    worktree_name: None,
                    worktree_highlight_positions: Vec::new(),
                    diff_stats: DiffStats::default(),
                }),
                // Background thread that completed (should show notification)
                ListEntry::Thread(ThreadEntry {
                    agent: Agent::NativeAgent,
                    session_info: acp_thread::AgentSessionInfo {
                        session_id: acp::SessionId::new(Arc::from("t-5")),
                        work_dirs: None,
                        title: Some("Notified thread".into()),
                        updated_at: Some(Utc::now()),
                        created_at: Some(Utc::now()),
                        meta: None,
                    },
                    icon: IconName::ZedAgent,
                    icon_from_external_svg: None,
                    status: AgentThreadStatus::Completed,
                    workspace: ThreadEntryWorkspace::Open(workspace.clone()),
                    is_live: true,
                    is_background: true,
                    is_title_generating: false,
                    highlight_positions: Vec::new(),
                    worktree_name: None,
                    worktree_highlight_positions: Vec::new(),
                    diff_stats: DiffStats::default(),
                }),
                // View More entry
                ListEntry::ViewMore {
                    path_list: expanded_path.clone(),
                    remaining_count: 42,
                    is_fully_expanded: false,
                },
                // Collapsed project header
                ListEntry::ProjectHeader {
                    path_list: collapsed_path.clone(),
                    label: "collapsed-project".into(),
                    workspace: workspace.clone(),
                    highlight_positions: Vec::new(),
                },
            ];
            // Select the Running thread (index 2)
            s.selection = Some(2);
        });

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [expanded-project]",
                "  Completed thread",
                "  Running thread * (running)  <== selected",
                "  Error thread * (error)",
                "  Waiting thread (waiting)",
                "  Notified thread * (!)",
                "  + View More (42)",
                "> [collapsed-project]",
            ]
        );

        // Move selection to the collapsed header
        sidebar.update_in(cx, |s, _window, _cx| {
            s.selection = Some(7);
        });

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx).last().cloned(),
            Some("> [collapsed-project]  <== selected".to_string()),
        );

        // Clear selection
        sidebar.update_in(cx, |s, _window, _cx| {
            s.selection = None;
        });

        // No entry should have the selected marker
        let entries = visible_entries_as_strings(&sidebar, cx);
        for entry in &entries {
            assert!(
                !entry.contains("<== selected"),
                "unexpected selection marker in: {}",
                entry
            );
        }
    }

    #[gpui::test]
    async fn test_keyboard_select_next_and_previous(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
        save_n_test_threads(3, &path_list, cx).await;

        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        // Entries: [header, new_thread, thread3, thread2, thread1]
        // Focusing the sidebar does not set a selection; select_next/select_previous
        // handle None gracefully by starting from the first or last entry.
        open_and_focus_sidebar(&sidebar, cx);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), None);

        // First SelectNext from None starts at index 0
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));

        // Move down through remaining entries
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(2));

        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(3));

        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(4));

        // At the end, wraps back to first entry
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));

        // Navigate back to the end
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(2));
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(3));
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(4));

        // Move back up
        cx.dispatch_action(SelectPrevious);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(3));

        cx.dispatch_action(SelectPrevious);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(2));

        cx.dispatch_action(SelectPrevious);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

        cx.dispatch_action(SelectPrevious);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));

        // At the top, selection clears (focus returns to editor)
        cx.dispatch_action(SelectPrevious);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), None);
    }

    #[gpui::test]
    async fn test_keyboard_select_first_and_last(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
        save_n_test_threads(3, &path_list, cx).await;
        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        open_and_focus_sidebar(&sidebar, cx);

        // SelectLast jumps to the end
        cx.dispatch_action(SelectLast);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(4));

        // SelectFirst jumps to the beginning
        cx.dispatch_action(SelectFirst);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));
    }

    #[gpui::test]
    async fn test_keyboard_focus_in_does_not_set_selection(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        // Initially no selection
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), None);

        // Open the sidebar so it's rendered, then focus it to trigger focus_in.
        // focus_in no longer sets a default selection.
        open_and_focus_sidebar(&sidebar, cx);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), None);

        // Manually set a selection, blur, then refocus — selection should be preserved
        sidebar.update_in(cx, |sidebar, _window, _cx| {
            sidebar.selection = Some(0);
        });

        cx.update(|window, _cx| {
            window.blur();
        });
        cx.run_until_parked();

        sidebar.update_in(cx, |_, window, cx| {
            cx.focus_self(window);
        });
        cx.run_until_parked();
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));
    }

    #[gpui::test]
    async fn test_keyboard_confirm_on_project_header_toggles_collapse(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
        save_n_test_threads(1, &path_list, cx).await;
        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]", "  [+ New Thread]", "  Thread 1"]
        );

        // Focus the sidebar and select the header (index 0)
        open_and_focus_sidebar(&sidebar, cx);
        sidebar.update_in(cx, |sidebar, _window, _cx| {
            sidebar.selection = Some(0);
        });

        // Confirm on project header collapses the group
        cx.dispatch_action(Confirm);
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["> [my-project]  <== selected"]
        );

        // Confirm again expands the group
        cx.dispatch_action(Confirm);
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]  <== selected",
                "  [+ New Thread]",
                "  Thread 1",
            ]
        );
    }

    #[gpui::test]
    async fn test_keyboard_confirm_on_view_more_expands(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
        save_n_test_threads(8, &path_list, cx).await;
        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        // Should show header + NewThread + 5 threads + "View More (3)"
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 8);
        assert!(entries.iter().any(|e| e.contains("View More (3)")));

        // Focus sidebar (selection starts at None), then navigate down to the "View More" entry (index 7)
        open_and_focus_sidebar(&sidebar, cx);
        for _ in 0..8 {
            cx.dispatch_action(SelectNext);
        }
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(7));

        // Confirm on "View More" to expand
        cx.dispatch_action(Confirm);
        cx.run_until_parked();

        // All 8 threads should now be visible with a "Collapse" button
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 11); // header + NewThread + 8 threads + Collapse button
        assert!(!entries.iter().any(|e| e.contains("View More")));
        assert!(entries.iter().any(|e| e.contains("Collapse")));
    }

    #[gpui::test]
    async fn test_keyboard_expand_and_collapse_selected_entry(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
        save_n_test_threads(1, &path_list, cx).await;
        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]", "  [+ New Thread]", "  Thread 1"]
        );

        // Focus sidebar and manually select the header (index 0). Press left to collapse.
        open_and_focus_sidebar(&sidebar, cx);
        sidebar.update_in(cx, |sidebar, _window, _cx| {
            sidebar.selection = Some(0);
        });

        cx.dispatch_action(CollapseSelectedEntry);
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["> [my-project]  <== selected"]
        );

        // Press right to expand
        cx.dispatch_action(ExpandSelectedEntry);
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]  <== selected",
                "  [+ New Thread]",
                "  Thread 1",
            ]
        );

        // Press right again on already-expanded header moves selection down
        cx.dispatch_action(ExpandSelectedEntry);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));
    }

    #[gpui::test]
    async fn test_keyboard_collapse_from_child_selects_parent(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
        save_n_test_threads(1, &path_list, cx).await;
        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        // Focus sidebar (selection starts at None), then navigate down to the thread (child)
        open_and_focus_sidebar(&sidebar, cx);
        cx.dispatch_action(SelectNext);
        cx.dispatch_action(SelectNext);
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(2));

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  [+ New Thread]",
                "  Thread 1  <== selected",
            ]
        );

        // Pressing left on a child collapses the parent group and selects it
        cx.dispatch_action(CollapseSelectedEntry);
        cx.run_until_parked();

        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["> [my-project]  <== selected"]
        );
    }

    #[gpui::test]
    async fn test_keyboard_navigation_on_empty_list(cx: &mut TestAppContext) {
        let project = init_test_project("/empty-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        // Even an empty project has the header and a new thread button
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [empty-project]", "  [+ New Thread]"]
        );

        // Focus sidebar — focus_in does not set a selection
        open_and_focus_sidebar(&sidebar, cx);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), None);

        // First SelectNext from None starts at index 0 (header)
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));

        // SelectNext moves to the new thread button
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

        // At the end, wraps back to first entry
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));

        // SelectPrevious from first entry clears selection (returns to editor)
        cx.dispatch_action(SelectPrevious);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), None);
    }

    #[gpui::test]
    async fn test_selection_clamps_after_entry_removal(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
        save_n_test_threads(1, &path_list, cx).await;
        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        // Focus sidebar (selection starts at None), navigate down to the thread (index 2)
        open_and_focus_sidebar(&sidebar, cx);
        cx.dispatch_action(SelectNext);
        cx.dispatch_action(SelectNext);
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(2));

        // Collapse the group, which removes the thread from the list
        cx.dispatch_action(CollapseSelectedEntry);
        cx.run_until_parked();

        // Selection should be clamped to the last valid index (0 = header)
        let selection = sidebar.read_with(cx, |s, _| s.selection);
        let entry_count = sidebar.read_with(cx, |s, _| s.contents.entries.len());
        assert!(
            selection.unwrap_or(0) < entry_count,
            "selection {} should be within bounds (entries: {})",
            selection.unwrap_or(0),
            entry_count,
        );
    }

    async fn init_test_project_with_agent_panel(
        worktree_path: &str,
        cx: &mut TestAppContext,
    ) -> Entity<project::Project> {
        agent_ui::test_support::init_test(cx);
        cx.update(|cx| {
            cx.update_flags(false, vec!["agent-v2".into()]);
            ThreadStore::init_global(cx);
            ThreadMetadataStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            prompt_store::init(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(worktree_path, serde_json::json!({ "src": {} }))
            .await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
        project::Project::test(fs, [worktree_path.as_ref()], cx).await
    }

    fn add_agent_panel(
        workspace: &Entity<Workspace>,
        project: &Entity<project::Project>,
        cx: &mut gpui::VisualTestContext,
    ) -> Entity<AgentPanel> {
        workspace.update_in(cx, |workspace, window, cx| {
            let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
            let panel = cx.new(|cx| AgentPanel::test_new(workspace, text_thread_store, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        })
    }

    fn setup_sidebar_with_agent_panel(
        multi_workspace: &Entity<MultiWorkspace>,
        project: &Entity<project::Project>,
        cx: &mut gpui::VisualTestContext,
    ) -> (Entity<Sidebar>, Entity<AgentPanel>) {
        let sidebar = setup_sidebar(multi_workspace, cx);
        let workspace = multi_workspace.read_with(cx, |mw, _cx| mw.workspace().clone());
        let panel = add_agent_panel(&workspace, project, cx);
        (sidebar, panel)
    }

    #[gpui::test]
    async fn test_parallel_threads_shown_with_live_status(cx: &mut TestAppContext) {
        let project = init_test_project_with_agent_panel("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, &project, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

        // Open thread A and keep it generating.
        let connection = StubAgentConnection::new();
        open_thread_with_connection(&panel, connection.clone(), cx);
        send_message(&panel, cx);

        let session_id_a = active_session_id(&panel, cx);
        save_test_thread_metadata(&session_id_a, path_list.clone(), cx).await;

        cx.update(|_, cx| {
            connection.send_update(
                session_id_a.clone(),
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("working...".into())),
                cx,
            );
        });
        cx.run_until_parked();

        // Open thread B (idle, default response) — thread A goes to background.
        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Done".into()),
        )]);
        open_thread_with_connection(&panel, connection, cx);
        send_message(&panel, cx);

        let session_id_b = active_session_id(&panel, cx);
        save_test_thread_metadata(&session_id_b, path_list.clone(), cx).await;

        cx.run_until_parked();

        let mut entries = visible_entries_as_strings(&sidebar, cx);
        entries[2..].sort();
        assert_eq!(
            entries,
            vec![
                "v [my-project]",
                "  [+ New Thread]",
                "  Hello *",
                "  Hello * (running)",
            ]
        );
    }

    #[gpui::test]
    async fn test_background_thread_completion_triggers_notification(cx: &mut TestAppContext) {
        let project_a = init_test_project_with_agent_panel("/project-a", cx).await;
        let (multi_workspace, cx) = cx
            .add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
        let (sidebar, panel_a) = setup_sidebar_with_agent_panel(&multi_workspace, &project_a, cx);

        let path_list_a = PathList::new(&[std::path::PathBuf::from("/project-a")]);

        // Open thread on workspace A and keep it generating.
        let connection_a = StubAgentConnection::new();
        open_thread_with_connection(&panel_a, connection_a.clone(), cx);
        send_message(&panel_a, cx);

        let session_id_a = active_session_id(&panel_a, cx);
        save_test_thread_metadata(&session_id_a, path_list_a.clone(), cx).await;

        cx.update(|_, cx| {
            connection_a.send_update(
                session_id_a.clone(),
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("chunk".into())),
                cx,
            );
        });
        cx.run_until_parked();

        // Add a second workspace and activate it (making workspace A the background).
        let fs = cx.update(|_, cx| <dyn fs::Fs>::global(cx));
        let project_b = project::Project::test(fs, [], cx).await;
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(project_b, window, cx);
        });
        cx.run_until_parked();

        // Thread A is still running; no notification yet.
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [project-a]",
                "  [+ New Thread]",
                "  Hello * (running)",
                "v [Empty Workspace]",
                "  [+ New Thread]",
            ]
        );

        // Complete thread A's turn (transition Running → Completed).
        connection_a.end_turn(session_id_a.clone(), acp::StopReason::EndTurn);
        cx.run_until_parked();

        // The completed background thread shows a notification indicator.
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [project-a]",
                "  [+ New Thread]",
                "  Hello * (!)",
                "v [Empty Workspace]",
                "  [+ New Thread]",
            ]
        );
    }

    fn type_in_search(sidebar: &Entity<Sidebar>, query: &str, cx: &mut gpui::VisualTestContext) {
        sidebar.update_in(cx, |sidebar, window, cx| {
            window.focus(&sidebar.filter_editor.focus_handle(cx), cx);
            sidebar.filter_editor.update(cx, |editor, cx| {
                editor.set_text(query, window, cx);
            });
        });
        cx.run_until_parked();
    }

    #[gpui::test]
    async fn test_search_narrows_visible_threads_to_matches(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

        for (id, title, hour) in [
            ("t-1", "Fix crash in project panel", 3),
            ("t-2", "Add inline diff view", 2),
            ("t-3", "Refactor settings module", 1),
        ] {
            save_thread_metadata(
                acp::SessionId::new(Arc::from(id)),
                title.into(),
                chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
                path_list.clone(),
                cx,
            )
            .await;
        }
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  [+ New Thread]",
                "  Fix crash in project panel",
                "  Add inline diff view",
                "  Refactor settings module",
            ]
        );

        // User types "diff" in the search box — only the matching thread remains,
        // with its workspace header preserved for context.
        type_in_search(&sidebar, "diff", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]", "  Add inline diff view  <== selected",]
        );

        // User changes query to something with no matches — list is empty.
        type_in_search(&sidebar, "nonexistent", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            Vec::<String>::new()
        );
    }

    #[gpui::test]
    async fn test_search_matches_regardless_of_case(cx: &mut TestAppContext) {
        // Scenario: A user remembers a thread title but not the exact casing.
        // Search should match case-insensitively so they can still find it.
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

        save_thread_metadata(
            acp::SessionId::new(Arc::from("thread-1")),
            "Fix Crash In Project Panel".into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
            path_list.clone(),
            cx,
        )
        .await;
        cx.run_until_parked();

        // Lowercase query matches mixed-case title.
        type_in_search(&sidebar, "fix crash", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  Fix Crash In Project Panel  <== selected",
            ]
        );

        // Uppercase query also matches the same title.
        type_in_search(&sidebar, "FIX CRASH", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  Fix Crash In Project Panel  <== selected",
            ]
        );
    }

    #[gpui::test]
    async fn test_escape_clears_search_and_restores_full_list(cx: &mut TestAppContext) {
        // Scenario: A user searches, finds what they need, then presses Escape
        // to dismiss the filter and see the full list again.
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

        for (id, title, hour) in [("t-1", "Alpha thread", 2), ("t-2", "Beta thread", 1)] {
            save_thread_metadata(
                acp::SessionId::new(Arc::from(id)),
                title.into(),
                chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
                path_list.clone(),
                cx,
            )
            .await;
        }
        cx.run_until_parked();

        // Confirm the full list is showing.
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  [+ New Thread]",
                "  Alpha thread",
                "  Beta thread",
            ]
        );

        // User types a search query to filter down.
        open_and_focus_sidebar(&sidebar, cx);
        type_in_search(&sidebar, "alpha", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]", "  Alpha thread  <== selected",]
        );

        // User presses Escape — filter clears, full list is restored.
        // The selection index (1) now points at the NewThread entry that was
        // re-inserted when the filter was removed.
        cx.dispatch_action(Cancel);
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  [+ New Thread]  <== selected",
                "  Alpha thread",
                "  Beta thread",
            ]
        );
    }

    #[gpui::test]
    async fn test_search_only_shows_workspace_headers_with_matches(cx: &mut TestAppContext) {
        let project_a = init_test_project("/project-a", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list_a = PathList::new(&[std::path::PathBuf::from("/project-a")]);

        for (id, title, hour) in [
            ("a1", "Fix bug in sidebar", 2),
            ("a2", "Add tests for editor", 1),
        ] {
            save_thread_metadata(
                acp::SessionId::new(Arc::from(id)),
                title.into(),
                chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
                path_list_a.clone(),
                cx,
            )
            .await;
        }

        // Add a second workspace.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.create_workspace(window, cx);
        });
        cx.run_until_parked();

        let path_list_b = PathList::new::<std::path::PathBuf>(&[]);

        for (id, title, hour) in [
            ("b1", "Refactor sidebar layout", 3),
            ("b2", "Fix typo in README", 1),
        ] {
            save_thread_metadata(
                acp::SessionId::new(Arc::from(id)),
                title.into(),
                chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
                path_list_b.clone(),
                cx,
            )
            .await;
        }
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [project-a]",
                "  [+ New Thread]",
                "  Fix bug in sidebar",
                "  Add tests for editor",
                "v [Empty Workspace]",
                "  [+ New Thread]",
                "  Refactor sidebar layout",
                "  Fix typo in README",
            ]
        );

        // "sidebar" matches a thread in each workspace — both headers stay visible.
        type_in_search(&sidebar, "sidebar", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [project-a]",
                "  Fix bug in sidebar  <== selected",
                "v [Empty Workspace]",
                "  Refactor sidebar layout",
            ]
        );

        // "typo" only matches in the second workspace — the first header disappears.
        type_in_search(&sidebar, "typo", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [Empty Workspace]", "  Fix typo in README  <== selected",]
        );

        // "project-a" matches the first workspace name — the header appears
        // with all child threads included.
        type_in_search(&sidebar, "project-a", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [project-a]",
                "  Fix bug in sidebar  <== selected",
                "  Add tests for editor",
            ]
        );
    }

    #[gpui::test]
    async fn test_search_matches_workspace_name(cx: &mut TestAppContext) {
        let project_a = init_test_project("/alpha-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list_a = PathList::new(&[std::path::PathBuf::from("/alpha-project")]);

        for (id, title, hour) in [
            ("a1", "Fix bug in sidebar", 2),
            ("a2", "Add tests for editor", 1),
        ] {
            save_thread_metadata(
                acp::SessionId::new(Arc::from(id)),
                title.into(),
                chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
                path_list_a.clone(),
                cx,
            )
            .await;
        }

        // Add a second workspace.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.create_workspace(window, cx);
        });
        cx.run_until_parked();

        let path_list_b = PathList::new::<std::path::PathBuf>(&[]);

        for (id, title, hour) in [
            ("b1", "Refactor sidebar layout", 3),
            ("b2", "Fix typo in README", 1),
        ] {
            save_thread_metadata(
                acp::SessionId::new(Arc::from(id)),
                title.into(),
                chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
                path_list_b.clone(),
                cx,
            )
            .await;
        }
        cx.run_until_parked();

        // "alpha" matches the workspace name "alpha-project" but no thread titles.
        // The workspace header should appear with all child threads included.
        type_in_search(&sidebar, "alpha", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [alpha-project]",
                "  Fix bug in sidebar  <== selected",
                "  Add tests for editor",
            ]
        );

        // "sidebar" matches thread titles in both workspaces but not workspace names.
        // Both headers appear with their matching threads.
        type_in_search(&sidebar, "sidebar", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [alpha-project]",
                "  Fix bug in sidebar  <== selected",
                "v [Empty Workspace]",
                "  Refactor sidebar layout",
            ]
        );

        // "alpha sidebar" matches the workspace name "alpha-project" (fuzzy: a-l-p-h-a-s-i-d-e-b-a-r
        // doesn't match) — but does not match either workspace name or any thread.
        // Actually let's test something simpler: a query that matches both a workspace
        // name AND some threads in that workspace. Matching threads should still appear.
        type_in_search(&sidebar, "fix", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [alpha-project]",
                "  Fix bug in sidebar  <== selected",
                "v [Empty Workspace]",
                "  Fix typo in README",
            ]
        );

        // A query that matches a workspace name AND a thread in that same workspace.
        // Both the header (highlighted) and all child threads should appear.
        type_in_search(&sidebar, "alpha", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [alpha-project]",
                "  Fix bug in sidebar  <== selected",
                "  Add tests for editor",
            ]
        );

        // Now search for something that matches only a workspace name when there
        // are also threads with matching titles — the non-matching workspace's
        // threads should still appear if their titles match.
        type_in_search(&sidebar, "alp", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [alpha-project]",
                "  Fix bug in sidebar  <== selected",
                "  Add tests for editor",
            ]
        );
    }

    #[gpui::test]
    async fn test_search_finds_threads_hidden_behind_view_more(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

        // Create 8 threads. The oldest one has a unique name and will be
        // behind View More (only 5 shown by default).
        for i in 0..8u32 {
            let title = if i == 0 {
                "Hidden gem thread".to_string()
            } else {
                format!("Thread {}", i + 1)
            };
            save_thread_metadata(
                acp::SessionId::new(Arc::from(format!("thread-{}", i))),
                title.into(),
                chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, i).unwrap(),
                path_list.clone(),
                cx,
            )
            .await;
        }
        cx.run_until_parked();

        // Confirm the thread is not visible and View More is shown.
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert!(
            entries.iter().any(|e| e.contains("View More")),
            "should have View More button"
        );
        assert!(
            !entries.iter().any(|e| e.contains("Hidden gem")),
            "Hidden gem should be behind View More"
        );

        // User searches for the hidden thread — it appears, and View More is gone.
        type_in_search(&sidebar, "hidden gem", cx);
        let filtered = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(
            filtered,
            vec!["v [my-project]", "  Hidden gem thread  <== selected",]
        );
        assert!(
            !filtered.iter().any(|e| e.contains("View More")),
            "View More should not appear when filtering"
        );
    }

    #[gpui::test]
    async fn test_search_finds_threads_inside_collapsed_groups(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

        save_thread_metadata(
            acp::SessionId::new(Arc::from("thread-1")),
            "Important thread".into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
            path_list.clone(),
            cx,
        )
        .await;
        cx.run_until_parked();

        // User focuses the sidebar and collapses the group using keyboard:
        // manually select the header, then press CollapseSelectedEntry to collapse.
        open_and_focus_sidebar(&sidebar, cx);
        sidebar.update_in(cx, |sidebar, _window, _cx| {
            sidebar.selection = Some(0);
        });
        cx.dispatch_action(CollapseSelectedEntry);
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["> [my-project]  <== selected"]
        );

        // User types a search — the thread appears even though its group is collapsed.
        type_in_search(&sidebar, "important", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["> [my-project]", "  Important thread  <== selected",]
        );
    }

    #[gpui::test]
    async fn test_search_then_keyboard_navigate_and_confirm(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

        for (id, title, hour) in [
            ("t-1", "Fix crash in panel", 3),
            ("t-2", "Fix lint warnings", 2),
            ("t-3", "Add new feature", 1),
        ] {
            save_thread_metadata(
                acp::SessionId::new(Arc::from(id)),
                title.into(),
                chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
                path_list.clone(),
                cx,
            )
            .await;
        }
        cx.run_until_parked();

        open_and_focus_sidebar(&sidebar, cx);

        // User types "fix" — two threads match.
        type_in_search(&sidebar, "fix", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  Fix crash in panel  <== selected",
                "  Fix lint warnings",
            ]
        );

        // Selection starts on the first matching thread. User presses
        // SelectNext to move to the second match.
        cx.dispatch_action(SelectNext);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  Fix crash in panel",
                "  Fix lint warnings  <== selected",
            ]
        );

        // User can also jump back with SelectPrevious.
        cx.dispatch_action(SelectPrevious);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  Fix crash in panel  <== selected",
                "  Fix lint warnings",
            ]
        );
    }

    #[gpui::test]
    async fn test_confirm_on_historical_thread_activates_workspace(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.create_workspace(window, cx);
        });
        cx.run_until_parked();

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

        save_thread_metadata(
            acp::SessionId::new(Arc::from("hist-1")),
            "Historical Thread".into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 6, 1, 0, 0, 0).unwrap(),
            path_list.clone(),
            cx,
        )
        .await;
        cx.run_until_parked();
        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  [+ New Thread]",
                "  Historical Thread",
                "v [Empty Workspace]",
                "  [+ New Thread]",
            ]
        );

        // Switch to workspace 1 so we can verify the confirm switches back.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(1, window, cx);
        });
        cx.run_until_parked();
        assert_eq!(
            multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
            1
        );

        // Confirm on the historical (non-live) thread at index 1.
        // Before a previous fix, the workspace field was Option<usize> and
        // historical threads had None, so activate_thread early-returned
        // without switching the workspace.
        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.selection = Some(1);
            sidebar.confirm(&Confirm, window, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
            0
        );
    }

    #[gpui::test]
    async fn test_click_clears_selection_and_focus_in_restores_it(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

        save_thread_metadata(
            acp::SessionId::new(Arc::from("t-1")),
            "Thread A".into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
            path_list.clone(),
            cx,
        )
        .await;

        save_thread_metadata(
            acp::SessionId::new(Arc::from("t-2")),
            "Thread B".into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
            path_list.clone(),
            cx,
        )
        .await;

        cx.run_until_parked();
        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  [+ New Thread]",
                "  Thread A",
                "  Thread B",
            ]
        );

        // Keyboard confirm preserves selection.
        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.selection = Some(2);
            sidebar.confirm(&Confirm, window, cx);
        });
        assert_eq!(
            sidebar.read_with(cx, |sidebar, _| sidebar.selection),
            Some(2)
        );

        // Click handlers clear selection to None so no highlight lingers
        // after a click regardless of focus state. The hover style provides
        // visual feedback during mouse interaction instead.
        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.selection = None;
            let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
            sidebar.toggle_collapse(&path_list, window, cx);
        });
        assert_eq!(sidebar.read_with(cx, |sidebar, _| sidebar.selection), None);

        // When the user tabs back into the sidebar, focus_in no longer
        // restores selection — it stays None.
        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.focus_in(window, cx);
        });
        assert_eq!(sidebar.read_with(cx, |sidebar, _| sidebar.selection), None);
    }

    #[gpui::test]
    async fn test_thread_title_update_propagates_to_sidebar(cx: &mut TestAppContext) {
        let project = init_test_project_with_agent_panel("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, &project, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

        let connection = StubAgentConnection::new();
        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Hi there!".into()),
        )]);
        open_thread_with_connection(&panel, connection, cx);
        send_message(&panel, cx);

        let session_id = active_session_id(&panel, cx);
        save_test_thread_metadata(&session_id, path_list.clone(), cx).await;
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]", "  [+ New Thread]", "  Hello *"]
        );

        // Simulate the agent generating a title. The notification chain is:
        // AcpThread::set_title emits TitleUpdated →
        // ConnectionView::handle_thread_event calls cx.notify() →
        // AgentPanel observer fires and emits AgentPanelEvent →
        // Sidebar subscription calls update_entries / rebuild_contents.
        //
        // Before the fix, handle_thread_event did NOT call cx.notify() for
        // TitleUpdated, so the AgentPanel observer never fired and the
        // sidebar kept showing the old title.
        let thread = panel.read_with(cx, |panel, cx| panel.active_agent_thread(cx).unwrap());
        thread.update(cx, |thread, cx| {
            thread
                .set_title("Friendly Greeting with AI".into(), cx)
                .detach();
        });
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  [+ New Thread]",
                "  Friendly Greeting with AI *"
            ]
        );
    }

    #[gpui::test]
    async fn test_focused_thread_tracks_user_intent(cx: &mut TestAppContext) {
        let project_a = init_test_project_with_agent_panel("/project-a", cx).await;
        let (multi_workspace, cx) = cx
            .add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
        let (sidebar, panel_a) = setup_sidebar_with_agent_panel(&multi_workspace, &project_a, cx);

        let path_list_a = PathList::new(&[std::path::PathBuf::from("/project-a")]);

        // Save a thread so it appears in the list.
        let connection_a = StubAgentConnection::new();
        connection_a.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Done".into()),
        )]);
        open_thread_with_connection(&panel_a, connection_a, cx);
        send_message(&panel_a, cx);
        let session_id_a = active_session_id(&panel_a, cx);
        save_test_thread_metadata(&session_id_a, path_list_a.clone(), cx).await;

        // Add a second workspace with its own agent panel.
        let fs = cx.update(|_, cx| <dyn fs::Fs>::global(cx));
        fs.as_fake()
            .insert_tree("/project-b", serde_json::json!({ "src": {} }))
            .await;
        let project_b = project::Project::test(fs, ["/project-b".as_ref()], cx).await;
        let workspace_b = multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(project_b.clone(), window, cx)
        });
        let panel_b = add_agent_panel(&workspace_b, &project_b, cx);
        cx.run_until_parked();

        let workspace_a = multi_workspace.read_with(cx, |mw, _cx| mw.workspaces()[0].clone());

        // ── 1. Initial state: no focused thread ──────────────────────────────
        sidebar.read_with(cx, |sidebar, _cx| {
            assert_eq!(
                sidebar.focused_thread, None,
                "Initially no thread should be focused"
            );
            assert_eq!(
                sidebar.active_entry_index, None,
                "No active entry when no thread is focused"
            );
        });

        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.activate_thread(
                Agent::NativeAgent,
                acp_thread::AgentSessionInfo {
                    session_id: session_id_a.clone(),
                    work_dirs: None,
                    title: Some("Test".into()),
                    updated_at: None,
                    created_at: None,
                    meta: None,
                },
                &workspace_a,
                window,
                cx,
            );
        });
        cx.run_until_parked();

        sidebar.read_with(cx, |sidebar, _cx| {
            assert_eq!(
                sidebar.focused_thread.as_ref(),
                Some(&session_id_a),
                "After clicking a thread, it should be the focused thread"
            );
            let active_entry = sidebar.active_entry_index
                .and_then(|ix| sidebar.contents.entries.get(ix));
            assert!(
                matches!(active_entry, Some(ListEntry::Thread(thread)) if thread.session_info.session_id == session_id_a),
                "Active entry should be the clicked thread"
            );
        });

        workspace_a.read_with(cx, |workspace, cx| {
            assert!(
                workspace.panel::<AgentPanel>(cx).is_some(),
                "Agent panel should exist"
            );
            let dock = workspace.right_dock().read(cx);
            assert!(
                dock.is_open(),
                "Clicking a thread should open the agent panel dock"
            );
        });

        let connection_b = StubAgentConnection::new();
        connection_b.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Thread B".into()),
        )]);
        open_thread_with_connection(&panel_b, connection_b, cx);
        send_message(&panel_b, cx);
        let session_id_b = active_session_id(&panel_b, cx);
        let path_list_b = PathList::new(&[std::path::PathBuf::from("/project-b")]);
        save_test_thread_metadata(&session_id_b, path_list_b.clone(), cx).await;
        cx.run_until_parked();

        // Workspace A is currently active. Click a thread in workspace B,
        // which also triggers a workspace switch.
        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.activate_thread(
                Agent::NativeAgent,
                acp_thread::AgentSessionInfo {
                    session_id: session_id_b.clone(),
                    work_dirs: None,
                    title: Some("Thread B".into()),
                    updated_at: None,
                    created_at: None,
                    meta: None,
                },
                &workspace_b,
                window,
                cx,
            );
        });
        cx.run_until_parked();

        sidebar.read_with(cx, |sidebar, _cx| {
            assert_eq!(
                sidebar.focused_thread.as_ref(),
                Some(&session_id_b),
                "Clicking a thread in another workspace should focus that thread"
            );
            let active_entry = sidebar
                .active_entry_index
                .and_then(|ix| sidebar.contents.entries.get(ix));
            assert!(
                matches!(active_entry, Some(ListEntry::Thread(thread)) if thread.session_info.session_id == session_id_b),
                "Active entry should be the cross-workspace thread"
            );
        });

        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_next_workspace(window, cx);
        });
        cx.run_until_parked();

        sidebar.read_with(cx, |sidebar, _cx| {
            assert_eq!(
                sidebar.focused_thread, None,
                "External workspace switch should clear focused_thread"
            );
            assert_eq!(
                sidebar.active_entry_index, None,
                "No active entry when no thread is focused"
            );
        });

        let connection_b2 = StubAgentConnection::new();
        connection_b2.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("New thread".into()),
        )]);
        open_thread_with_connection(&panel_b, connection_b2, cx);
        send_message(&panel_b, cx);
        let session_id_b2 = active_session_id(&panel_b, cx);
        save_test_thread_metadata(&session_id_b2, path_list_b.clone(), cx).await;
        cx.run_until_parked();

        // Panel B is not the active workspace's panel (workspace A is
        // active), so opening a thread there should not change focused_thread.
        // This prevents running threads in background workspaces from causing
        // the selection highlight to jump around.
        sidebar.read_with(cx, |sidebar, _cx| {
            assert_eq!(
                sidebar.focused_thread, None,
                "Opening a thread in a non-active panel should not set focused_thread"
            );
        });

        workspace_b.update_in(cx, |workspace, window, cx| {
            workspace.focus_handle(cx).focus(window, cx);
        });
        cx.run_until_parked();

        sidebar.read_with(cx, |sidebar, _cx| {
            assert_eq!(
                sidebar.focused_thread, None,
                "Defocusing the sidebar should not set focused_thread"
            );
        });

        // Switching workspaces via the multi_workspace (simulates clicking
        // a workspace header) should clear focused_thread.
        multi_workspace.update_in(cx, |mw, window, cx| {
            if let Some(index) = mw.workspaces().iter().position(|w| w == &workspace_b) {
                mw.activate_index(index, window, cx);
            }
        });
        cx.run_until_parked();

        sidebar.read_with(cx, |sidebar, _cx| {
            assert_eq!(
                sidebar.focused_thread, None,
                "Switching workspace should clear focused_thread"
            );
            assert_eq!(
                sidebar.active_entry_index, None,
                "No active entry when no thread is focused"
            );
        });

        // ── 8. Focusing the agent panel thread restores focused_thread ────
        // Workspace B still has session_id_b2 loaded in the agent panel.
        // Clicking into the thread (simulated by focusing its view) should
        // set focused_thread via the ThreadFocused event.
        panel_b.update_in(cx, |panel, window, cx| {
            if let Some(thread_view) = panel.active_conversation_view() {
                thread_view.read(cx).focus_handle(cx).focus(window, cx);
            }
        });
        cx.run_until_parked();

        sidebar.read_with(cx, |sidebar, _cx| {
            assert_eq!(
                sidebar.focused_thread.as_ref(),
                Some(&session_id_b2),
                "Focusing the agent panel thread should set focused_thread"
            );
            let active_entry = sidebar
                .active_entry_index
                .and_then(|ix| sidebar.contents.entries.get(ix));
            assert!(
                matches!(active_entry, Some(ListEntry::Thread(thread)) if thread.session_info.session_id == session_id_b2),
                "Active entry should be the focused thread"
            );
        });
    }

    async fn init_test_project_with_git(
        worktree_path: &str,
        cx: &mut TestAppContext,
    ) -> (Entity<project::Project>, Arc<dyn fs::Fs>) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            worktree_path,
            serde_json::json!({
                ".git": {},
                "src": {},
            }),
        )
        .await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
        let project = project::Project::test(fs.clone(), [worktree_path.as_ref()], cx).await;
        (project, fs)
    }

    #[gpui::test]
    async fn test_search_matches_worktree_name(cx: &mut TestAppContext) {
        let (project, fs) = init_test_project_with_git("/project", cx).await;

        fs.as_fake()
            .with_git_state(std::path::Path::new("/project/.git"), false, |state| {
                state.worktrees.push(git::repository::Worktree {
                    path: std::path::PathBuf::from("/wt/rosewood"),
                    ref_name: "refs/heads/rosewood".into(),
                    sha: "abc".into(),
                });
            })
            .unwrap();

        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let main_paths = PathList::new(&[std::path::PathBuf::from("/project")]);
        let wt_paths = PathList::new(&[std::path::PathBuf::from("/wt/rosewood")]);
        save_named_thread_metadata("main-t", "Unrelated Thread", &main_paths, cx).await;
        save_named_thread_metadata("wt-t", "Fix Bug", &wt_paths, cx).await;

        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        // Search for "rosewood" — should match the worktree name, not the title.
        type_in_search(&sidebar, "rosewood", cx);

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [project]", "  Fix Bug {rosewood}  <== selected"],
        );
    }

    #[gpui::test]
    async fn test_git_worktree_added_live_updates_sidebar(cx: &mut TestAppContext) {
        let (project, fs) = init_test_project_with_git("/project", cx).await;

        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        // Save a thread against a worktree path that doesn't exist yet.
        let wt_paths = PathList::new(&[std::path::PathBuf::from("/wt/rosewood")]);
        save_named_thread_metadata("wt-thread", "Worktree Thread", &wt_paths, cx).await;

        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        // Thread is not visible yet — no worktree knows about this path.
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [project]", "  [+ New Thread]"]
        );

        // Now add the worktree to the git state and trigger a rescan.
        fs.as_fake()
            .with_git_state(std::path::Path::new("/project/.git"), true, |state| {
                state.worktrees.push(git::repository::Worktree {
                    path: std::path::PathBuf::from("/wt/rosewood"),
                    ref_name: "refs/heads/rosewood".into(),
                    sha: "abc".into(),
                });
            })
            .unwrap();

        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [project]",
                "  [+ New Thread]",
                "  Worktree Thread {rosewood}",
            ]
        );
    }

    #[gpui::test]
    async fn test_two_worktree_workspaces_absorbed_when_main_added(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        // Create the main repo directory (not opened as a workspace yet).
        fs.insert_tree(
            "/project",
            serde_json::json!({
                ".git": {
                    "worktrees": {
                        "feature-a": {
                            "commondir": "../../",
                            "HEAD": "ref: refs/heads/feature-a",
                        },
                        "feature-b": {
                            "commondir": "../../",
                            "HEAD": "ref: refs/heads/feature-b",
                        },
                    },
                },
                "src": {},
            }),
        )
        .await;

        // Two worktree checkouts whose .git files point back to the main repo.
        fs.insert_tree(
            "/wt-feature-a",
            serde_json::json!({
                ".git": "gitdir: /project/.git/worktrees/feature-a",
                "src": {},
            }),
        )
        .await;
        fs.insert_tree(
            "/wt-feature-b",
            serde_json::json!({
                ".git": "gitdir: /project/.git/worktrees/feature-b",
                "src": {},
            }),
        )
        .await;

        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let project_a = project::Project::test(fs.clone(), ["/wt-feature-a".as_ref()], cx).await;
        let project_b = project::Project::test(fs.clone(), ["/wt-feature-b".as_ref()], cx).await;

        project_a.update(cx, |p, cx| p.git_scans_complete(cx)).await;
        project_b.update(cx, |p, cx| p.git_scans_complete(cx)).await;

        // Open both worktrees as workspaces — no main repo yet.
        let (multi_workspace, cx) = cx
            .add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(project_b.clone(), window, cx);
        });
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let paths_a = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
        let paths_b = PathList::new(&[std::path::PathBuf::from("/wt-feature-b")]);
        save_named_thread_metadata("thread-a", "Thread A", &paths_a, cx).await;
        save_named_thread_metadata("thread-b", "Thread B", &paths_b, cx).await;

        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        // Without the main repo, each worktree has its own header.
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [wt-feature-a]",
                "  [+ New Thread]",
                "  Thread A",
                "v [wt-feature-b]",
                "  [+ New Thread]",
                "  Thread B",
            ]
        );

        // Configure the main repo to list both worktrees before opening
        // it so the initial git scan picks them up.
        fs.with_git_state(std::path::Path::new("/project/.git"), false, |state| {
            state.worktrees.push(git::repository::Worktree {
                path: std::path::PathBuf::from("/wt-feature-a"),
                ref_name: "refs/heads/feature-a".into(),
                sha: "aaa".into(),
            });
            state.worktrees.push(git::repository::Worktree {
                path: std::path::PathBuf::from("/wt-feature-b"),
                ref_name: "refs/heads/feature-b".into(),
                sha: "bbb".into(),
            });
        })
        .unwrap();

        let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
        main_project
            .update(cx, |p, cx| p.git_scans_complete(cx))
            .await;

        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(main_project.clone(), window, cx);
        });
        cx.run_until_parked();

        // Both worktree workspaces should now be absorbed under the main
        // repo header, with worktree chips.
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [project]",
                "  [+ New Thread]",
                "  Thread A {wt-feature-a}",
                "  Thread B {wt-feature-b}",
            ]
        );

        // Remove feature-b from the main repo's linked worktrees.
        // The feature-b workspace should be pruned automatically.
        fs.with_git_state(std::path::Path::new("/project/.git"), true, |state| {
            state
                .worktrees
                .retain(|wt| wt.path != std::path::Path::new("/wt-feature-b"));
        })
        .unwrap();

        cx.run_until_parked();

        // feature-b's workspace is pruned; feature-a remains absorbed
        // under the main repo.
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [project]",
                "  [+ New Thread]",
                "  Thread A {wt-feature-a}",
            ]
        );
    }

    #[gpui::test]
    async fn test_clicking_worktree_thread_opens_workspace_when_none_exists(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        fs.insert_tree(
            "/project",
            serde_json::json!({
                ".git": {
                    "worktrees": {
                        "feature-a": {
                            "commondir": "../../",
                            "HEAD": "ref: refs/heads/feature-a",
                        },
                    },
                },
                "src": {},
            }),
        )
        .await;

        fs.insert_tree(
            "/wt-feature-a",
            serde_json::json!({
                ".git": "gitdir: /project/.git/worktrees/feature-a",
                "src": {},
            }),
        )
        .await;

        fs.with_git_state(std::path::Path::new("/project/.git"), false, |state| {
            state.worktrees.push(git::repository::Worktree {
                path: std::path::PathBuf::from("/wt-feature-a"),
                ref_name: "refs/heads/feature-a".into(),
                sha: "aaa".into(),
            });
        })
        .unwrap();

        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        // Only open the main repo — no workspace for the worktree.
        let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
        main_project
            .update(cx, |p, cx| p.git_scans_complete(cx))
            .await;

        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            MultiWorkspace::test_new(main_project.clone(), window, cx)
        });
        let sidebar = setup_sidebar(&multi_workspace, cx);

        // Save a thread for the worktree path (no workspace for it).
        let paths_wt = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
        save_named_thread_metadata("thread-wt", "WT Thread", &paths_wt, cx).await;

        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        // Thread should appear under the main repo with a worktree chip.
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [project]",
                "  [+ New Thread]",
                "  WT Thread {wt-feature-a}"
            ],
        );

        // Only 1 workspace should exist.
        assert_eq!(
            multi_workspace.read_with(cx, |mw, _| mw.workspaces().len()),
            1,
        );

        // Focus the sidebar and select the worktree thread.
        open_and_focus_sidebar(&sidebar, cx);
        sidebar.update_in(cx, |sidebar, _window, _cx| {
            sidebar.selection = Some(2); // index 0 is header, 1 is NewThread, 2 is the thread
        });

        // Confirm to open the worktree thread.
        cx.dispatch_action(Confirm);
        cx.run_until_parked();

        // A new workspace should have been created for the worktree path.
        let new_workspace = multi_workspace.read_with(cx, |mw, _| {
            assert_eq!(
                mw.workspaces().len(),
                2,
                "confirming a worktree thread without a workspace should open one",
            );
            mw.workspaces()[1].clone()
        });

        let new_path_list =
            new_workspace.read_with(cx, |_, cx| workspace_path_list(&new_workspace, cx));
        assert_eq!(
            new_path_list,
            PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]),
            "the new workspace should have been opened for the worktree path",
        );
    }

    #[gpui::test]
    async fn test_clicking_absorbed_worktree_thread_activates_worktree_workspace(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        fs.insert_tree(
            "/project",
            serde_json::json!({
                ".git": {
                    "worktrees": {
                        "feature-a": {
                            "commondir": "../../",
                            "HEAD": "ref: refs/heads/feature-a",
                        },
                    },
                },
                "src": {},
            }),
        )
        .await;

        fs.insert_tree(
            "/wt-feature-a",
            serde_json::json!({
                ".git": "gitdir: /project/.git/worktrees/feature-a",
                "src": {},
            }),
        )
        .await;

        fs.with_git_state(std::path::Path::new("/project/.git"), false, |state| {
            state.worktrees.push(git::repository::Worktree {
                path: std::path::PathBuf::from("/wt-feature-a"),
                ref_name: "refs/heads/feature-a".into(),
                sha: "aaa".into(),
            });
        })
        .unwrap();

        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
        let worktree_project =
            project::Project::test(fs.clone(), ["/wt-feature-a".as_ref()], cx).await;

        main_project
            .update(cx, |p, cx| p.git_scans_complete(cx))
            .await;
        worktree_project
            .update(cx, |p, cx| p.git_scans_complete(cx))
            .await;

        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            MultiWorkspace::test_new(main_project.clone(), window, cx)
        });

        let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(worktree_project.clone(), window, cx)
        });

        // Activate the main workspace before setting up the sidebar.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(0, window, cx);
        });

        let sidebar = setup_sidebar(&multi_workspace, cx);

        let paths_main = PathList::new(&[std::path::PathBuf::from("/project")]);
        let paths_wt = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
        save_named_thread_metadata("thread-main", "Main Thread", &paths_main, cx).await;
        save_named_thread_metadata("thread-wt", "WT Thread", &paths_wt, cx).await;

        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        // The worktree workspace should be absorbed under the main repo.
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0], "v [project]");
        assert_eq!(entries[1], "  [+ New Thread]");
        assert!(entries.contains(&"  Main Thread".to_string()));
        assert!(entries.contains(&"  WT Thread {wt-feature-a}".to_string()));

        let wt_thread_index = entries
            .iter()
            .position(|e| e.contains("WT Thread"))
            .expect("should find the worktree thread entry");

        assert_eq!(
            multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
            0,
            "main workspace should be active initially"
        );

        // Focus the sidebar and select the absorbed worktree thread.
        open_and_focus_sidebar(&sidebar, cx);
        sidebar.update_in(cx, |sidebar, _window, _cx| {
            sidebar.selection = Some(wt_thread_index);
        });

        // Confirm to activate the worktree thread.
        cx.dispatch_action(Confirm);
        cx.run_until_parked();

        // The worktree workspace should now be active, not the main one.
        let active_workspace = multi_workspace.read_with(cx, |mw, _| {
            mw.workspaces()[mw.active_workspace_index()].clone()
        });
        assert_eq!(
            active_workspace, worktree_workspace,
            "clicking an absorbed worktree thread should activate the worktree workspace"
        );
    }

    #[gpui::test]
    async fn test_activate_archived_thread_with_saved_paths_activates_matching_workspace(
        cx: &mut TestAppContext,
    ) {
        // Thread has saved metadata in ThreadStore. A matching workspace is
        // already open. Expected: activates the matching workspace.
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
            .await;
        fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
            .await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
        let project_b = project::Project::test(fs.clone(), ["/project-b".as_ref()], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a, window, cx));

        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(project_b, window, cx);
        });

        let sidebar = setup_sidebar(&multi_workspace, cx);

        // Save a thread with path_list pointing to project-b.
        let path_list_b = PathList::new(&[std::path::PathBuf::from("/project-b")]);
        let session_id = acp::SessionId::new(Arc::from("archived-1"));
        save_test_thread_metadata(&session_id, path_list_b.clone(), cx).await;

        // Ensure workspace A is active.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(0, window, cx);
        });
        cx.run_until_parked();
        assert_eq!(
            multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
            0
        );

        // Call activate_archived_thread – should resolve saved paths and
        // switch to the workspace for project-b.
        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.activate_archived_thread(
                Agent::NativeAgent,
                acp_thread::AgentSessionInfo {
                    session_id: session_id.clone(),
                    work_dirs: Some(PathList::new(&[PathBuf::from("/project-b")])),
                    title: Some("Archived Thread".into()),
                    updated_at: None,
                    created_at: None,
                    meta: None,
                },
                window,
                cx,
            );
        });
        cx.run_until_parked();

        assert_eq!(
            multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
            1,
            "should have activated the workspace matching the saved path_list"
        );
    }

    #[gpui::test]
    async fn test_activate_archived_thread_cwd_fallback_with_matching_workspace(
        cx: &mut TestAppContext,
    ) {
        // Thread has no saved metadata but session_info has cwd. A matching
        // workspace is open. Expected: uses cwd to find and activate it.
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
            .await;
        fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
            .await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
        let project_b = project::Project::test(fs.clone(), ["/project-b".as_ref()], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a, window, cx));

        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(project_b, window, cx);
        });

        let sidebar = setup_sidebar(&multi_workspace, cx);

        // Start with workspace A active.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(0, window, cx);
        });
        cx.run_until_parked();
        assert_eq!(
            multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
            0
        );

        // No thread saved to the store – cwd is the only path hint.
        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.activate_archived_thread(
                Agent::NativeAgent,
                acp_thread::AgentSessionInfo {
                    session_id: acp::SessionId::new(Arc::from("unknown-session")),
                    work_dirs: Some(PathList::new(&[std::path::PathBuf::from("/project-b")])),
                    title: Some("CWD Thread".into()),
                    updated_at: None,
                    created_at: None,
                    meta: None,
                },
                window,
                cx,
            );
        });
        cx.run_until_parked();

        assert_eq!(
            multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
            1,
            "should have activated the workspace matching the cwd"
        );
    }

    #[gpui::test]
    async fn test_activate_archived_thread_no_paths_no_cwd_uses_active_workspace(
        cx: &mut TestAppContext,
    ) {
        // Thread has no saved metadata and no cwd. Expected: falls back to
        // the currently active workspace.
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
            .await;
        fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
            .await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
        let project_b = project::Project::test(fs.clone(), ["/project-b".as_ref()], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a, window, cx));

        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(project_b, window, cx);
        });

        let sidebar = setup_sidebar(&multi_workspace, cx);

        // Activate workspace B (index 1) to make it the active one.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(1, window, cx);
        });
        cx.run_until_parked();
        assert_eq!(
            multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
            1
        );

        // No saved thread, no cwd – should fall back to the active workspace.
        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.activate_archived_thread(
                Agent::NativeAgent,
                acp_thread::AgentSessionInfo {
                    session_id: acp::SessionId::new(Arc::from("no-context-session")),
                    work_dirs: None,
                    title: Some("Contextless Thread".into()),
                    updated_at: None,
                    created_at: None,
                    meta: None,
                },
                window,
                cx,
            );
        });
        cx.run_until_parked();

        assert_eq!(
            multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
            1,
            "should have stayed on the active workspace when no path info is available"
        );
    }

    #[gpui::test]
    async fn test_activate_archived_thread_saved_paths_opens_new_workspace(
        cx: &mut TestAppContext,
    ) {
        // Thread has saved metadata pointing to a path with no open workspace.
        // Expected: opens a new workspace for that path.
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
            .await;
        fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
            .await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a, window, cx));

        let sidebar = setup_sidebar(&multi_workspace, cx);

        // Save a thread with path_list pointing to project-b – which has no
        // open workspace.
        let path_list_b = PathList::new(&[std::path::PathBuf::from("/project-b")]);
        let session_id = acp::SessionId::new(Arc::from("archived-new-ws"));

        assert_eq!(
            multi_workspace.read_with(cx, |mw, _| mw.workspaces().len()),
            1,
            "should start with one workspace"
        );

        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.activate_archived_thread(
                Agent::NativeAgent,
                acp_thread::AgentSessionInfo {
                    session_id: session_id.clone(),
                    work_dirs: Some(path_list_b),
                    title: Some("New WS Thread".into()),
                    updated_at: None,
                    created_at: None,
                    meta: None,
                },
                window,
                cx,
            );
        });
        cx.run_until_parked();

        assert_eq!(
            multi_workspace.read_with(cx, |mw, _| mw.workspaces().len()),
            2,
            "should have opened a second workspace for the archived thread's saved paths"
        );
    }
}
