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
use project::{AgentId, Event as ProjectEvent, linked_worktree_short_name};
use recent_projects::sidebar_recent_projects::SidebarRecentProjects;
use ui::utils::platform_title_bar_height;

use settings::Settings as _;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use theme::ActiveTheme;
use ui::{
    AgentThreadStatus, CommonAnimationExt, ContextMenu, Divider, HighlightedLabel, KeyBinding,
    PopoverMenu, PopoverMenuHandle, Tab, ThreadItem, TintColor, Tooltip, WithScrollbar, prelude::*,
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
    worktree_full_path: Option<SharedString>,
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

    /// When modifying this thread, aim for a single forward pass over workspaces
    /// and threads plus an O(T log T) sort. Avoid adding extra scans over the data.
    fn rebuild_contents(&mut self, cx: &App) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };
        let mw = multi_workspace.read(cx);
        let workspaces = mw.workspaces().to_vec();
        let active_workspace = mw.workspaces().get(mw.active_workspace_index()).cloned();

        // Build a lookup for agent icons from the first workspace's AgentServerStore.
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

        // Identify absorbed workspaces in a single pass. A workspace is
        // "absorbed" when it points at a git worktree checkout whose main
        // repo is open as another workspace — its threads appear under the
        // main repo's header instead of getting their own.
        let mut main_repo_workspace: HashMap<Arc<Path>, usize> = HashMap::new();
        let mut absorbed: HashMap<usize, (usize, SharedString)> = HashMap::new();
        let mut pending: HashMap<Arc<Path>, Vec<(usize, SharedString, Arc<Path>)>> = HashMap::new();
        let mut absorbed_workspace_by_path: HashMap<Arc<Path>, usize> = HashMap::new();
        let workspace_indices_by_path: HashMap<Arc<Path>, Vec<usize>> = workspaces
            .iter()
            .enumerate()
            .flat_map(|(index, workspace)| {
                let paths = workspace_path_list(workspace, cx).paths().to_vec();
                paths
                    .into_iter()
                    .map(move |path| (Arc::from(path.as_path()), index))
            })
            .fold(HashMap::new(), |mut map, (path, index)| {
                map.entry(path).or_default().push(index);
                map
            });

        for (i, workspace) in workspaces.iter().enumerate() {
            for snapshot in root_repository_snapshots(workspace, cx) {
                if snapshot.is_main_worktree() {
                    main_repo_workspace
                        .entry(snapshot.work_directory_abs_path.clone())
                        .or_insert(i);

                    for git_worktree in snapshot.linked_worktrees() {
                        let worktree_path: Arc<Path> = Arc::from(git_worktree.path.as_path());
                        if let Some(worktree_indices) =
                            workspace_indices_by_path.get(worktree_path.as_ref())
                        {
                            for &worktree_idx in worktree_indices {
                                if worktree_idx == i {
                                    continue;
                                }

                                let worktree_name = linked_worktree_short_name(
                                    &snapshot.original_repo_abs_path,
                                    &git_worktree.path,
                                )
                                .unwrap_or_default();
                                absorbed.insert(worktree_idx, (i, worktree_name.clone()));
                                absorbed_workspace_by_path
                                    .insert(worktree_path.clone(), worktree_idx);
                            }
                        }
                    }

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

        let has_open_projects = workspaces
            .iter()
            .any(|ws| !workspace_path_list(ws, cx).paths().is_empty());

        let active_ws_index = active_workspace
            .as_ref()
            .and_then(|active| workspaces.iter().position(|ws| ws == active));

        for (ws_index, workspace) in workspaces.iter().enumerate() {
            if absorbed.contains_key(&ws_index) {
                continue;
            }

            let path_list = workspace_path_list(workspace, cx);
            if path_list.paths().is_empty() {
                continue;
            }

            let label = workspace_label_from_path_list(&path_list);

            let is_collapsed = self.collapsed_groups.contains(&path_list);
            let should_load_threads = !is_collapsed || !query.is_empty();

            let is_active = active_ws_index.is_some_and(|active_idx| {
                active_idx == ws_index
                    || absorbed
                        .get(&active_idx)
                        .is_some_and(|(main_idx, _)| *main_idx == ws_index)
            });

            let mut live_infos: Vec<_> = all_thread_infos_for_workspace(workspace, cx).collect();

            let mut threads: Vec<ThreadEntry> = Vec::new();
            let mut has_running_threads = false;
            let mut waiting_thread_count: usize = 0;

            if should_load_threads {
                let mut seen_session_ids: HashSet<acp::SessionId> = HashSet::new();

                // Read threads from the store cache for this workspace's path list.
                let thread_store = SidebarThreadMetadataStore::global(cx);
                let workspace_rows: Vec<_> =
                    thread_store.read(cx).entries_for_path(&path_list).collect();
                for row in workspace_rows {
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
                        worktree_full_path: None,
                        worktree_highlight_positions: Vec::new(),
                        diff_stats: DiffStats::default(),
                    });
                }

                // Load threads from linked git worktrees of this workspace's repos.
                {
                    let mut linked_worktree_queries: Vec<(PathList, SharedString, Arc<Path>)> =
                        Vec::new();
                    for snapshot in root_repository_snapshots(workspace, cx) {
                        if snapshot.is_linked_worktree() {
                            continue;
                        }

                        let main_worktree_path = snapshot.original_repo_abs_path.clone();

                        for git_worktree in snapshot.linked_worktrees() {
                            let worktree_name =
                                linked_worktree_short_name(&main_worktree_path, &git_worktree.path)
                                    .unwrap_or_default();
                            linked_worktree_queries.push((
                                PathList::new(std::slice::from_ref(&git_worktree.path)),
                                worktree_name,
                                Arc::from(git_worktree.path.as_path()),
                            ));
                        }
                    }

                    for (worktree_path_list, worktree_name, worktree_path) in
                        &linked_worktree_queries
                    {
                        let target_workspace = match absorbed_workspace_by_path
                            .get(worktree_path.as_ref())
                        {
                            Some(&idx) => {
                                live_infos
                                    .extend(all_thread_infos_for_workspace(&workspaces[idx], cx));
                                ThreadEntryWorkspace::Open(workspaces[idx].clone())
                            }
                            None => ThreadEntryWorkspace::Closed(worktree_path_list.clone()),
                        };

                        let worktree_rows: Vec<_> = thread_store
                            .read(cx)
                            .entries_for_path(worktree_path_list)
                            .collect();
                        for row in worktree_rows {
                            if !seen_session_ids.insert(row.session_id.clone()) {
                                continue;
                            }
                            let (agent, icon, icon_from_external_svg) = match &row.agent_id {
                                None => (Agent::NativeAgent, IconName::ZedAgent, None),
                                Some(name) => {
                                    let custom_icon =
                                        agent_server_store.as_ref().and_then(|store| {
                                            store.read(cx).agent_icon(&AgentId(name.clone().into()))
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
                                worktree_full_path: Some(
                                    worktree_path.display().to_string().into(),
                                ),
                                worktree_highlight_positions: Vec::new(),
                                diff_stats: DiffStats::default(),
                            });
                        }
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
                    let session_id = &thread.session_info.session_id;

                    if let Some(info) = live_info_by_session.get(session_id) {
                        thread.session_info.title = Some(info.title.clone());
                        thread.status = info.status;
                        thread.icon = info.icon;
                        thread.icon_from_external_svg = info.icon_from_external_svg.clone();
                        thread.is_live = true;
                        thread.is_background = info.is_background;
                        thread.is_title_generating = info.is_title_generating;
                        thread.diff_stats = info.diff_stats;
                    }

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
                for info in &live_infos {
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

                project_header_indices.push(entries.len());
                entries.push(ListEntry::ProjectHeader {
                    path_list: path_list.clone(),
                    label,
                    workspace: workspace.clone(),
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
                    workspace: workspace.clone(),
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
                        workspace: workspace.clone(),
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
            &workspace,
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
                if snapshot.is_linked_worktree() {
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
            let should_prune = root_repository_snapshots(workspace, cx).any(|snapshot| {
                snapshot.is_linked_worktree()
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
            .when_some(thread.worktree_name.clone(), |this, name| {
                let this = this.worktree(name);
                match thread.worktree_full_path.clone() {
                    Some(path) => this.worktree_full_path(path),
                    None => this,
                }
            })
            .worktree_highlight_positions(thread.worktree_highlight_positions.clone())
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
}

impl Sidebar {
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
        let ui_font = theme::setup_ui_font(window, cx);
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
    enum ThreadInfoIterator<T: Iterator<Item = ActiveThreadInfo>> {
        Empty,
        Threads(T),
    }

    impl<T: Iterator<Item = ActiveThreadInfo>> Iterator for ThreadInfoIterator<T> {
        type Item = ActiveThreadInfo;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                ThreadInfoIterator::Empty => None,
                ThreadInfoIterator::Threads(threads) => threads.next(),
            }
        }
    }

    let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) else {
        return ThreadInfoIterator::Empty;
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

    ThreadInfoIterator::Threads(threads)
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
            SidebarThreadMetadataStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            prompt_store::init(cx);
        });
    }

    fn has_thread_entry(sidebar: &Sidebar, session_id: &acp::SessionId) -> bool {
        sidebar.contents.entries.iter().any(|entry| {
            matches!(entry, ListEntry::Thread(t) if &t.session_info.session_id == session_id)
        })
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
        multi_workspace.update(cx, |mw, cx| {
            mw.register_sidebar(sidebar.clone(), cx);
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
        cx.update(|cx| {
            SidebarThreadMetadataStore::global(cx).update(cx, |store, cx| store.save(metadata, cx))
        });
        cx.run_until_parked();
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
                            is_fully_expanded, ..
                        } => {
                            if *is_fully_expanded {
                                format!("  - Collapse{}", selected)
                            } else {
                                format!("  + View More{}", selected)
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
    async fn test_entities_released_on_window_close(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let weak_workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().downgrade());
        let weak_sidebar = sidebar.downgrade();
        let weak_multi_workspace = multi_workspace.downgrade();

        drop(sidebar);
        drop(multi_workspace);
        cx.update(|window, _cx| window.remove_window());
        cx.run_until_parked();

        weak_multi_workspace.assert_released();
        weak_sidebar.assert_released();
        weak_workspace.assert_released();
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
            vec!["v [project-a]", "  Thread A1"]
        );

        // Add a second workspace
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.create_test_workspace(window, cx).detach();
        });
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [project-a]", "  Thread A1",]
        );

        // Remove the second workspace
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.remove_workspace(1, window, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [project-a]", "  Thread A1"]
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
                "  Thread 12",
                "  Thread 11",
                "  Thread 10",
                "  Thread 9",
                "  Thread 8",
                "  + View More",
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

        // Initially shows 5 threads + View More
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 7); // header + 5 threads + View More
        assert!(entries.iter().any(|e| e.contains("View More")));

        // Focus and navigate to View More, then confirm to expand by one batch
        open_and_focus_sidebar(&sidebar, cx);
        for _ in 0..7 {
            cx.dispatch_action(SelectNext);
        }
        cx.dispatch_action(Confirm);
        cx.run_until_parked();

        // Now shows 10 threads + View More
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 12); // header + 10 threads + View More
        assert!(entries.iter().any(|e| e.contains("View More")));

        // Expand again by one batch
        sidebar.update_in(cx, |s, _window, cx| {
            let current = s.expanded_groups.get(&path_list).copied().unwrap_or(0);
            s.expanded_groups.insert(path_list.clone(), current + 1);
            s.update_entries(cx);
        });
        cx.run_until_parked();

        // Now shows 15 threads + View More
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 17); // header + 15 threads + View More
        assert!(entries.iter().any(|e| e.contains("View More")));

        // Expand one more time - should show all 17 threads with Collapse button
        sidebar.update_in(cx, |s, _window, cx| {
            let current = s.expanded_groups.get(&path_list).copied().unwrap_or(0);
            s.expanded_groups.insert(path_list.clone(), current + 1);
            s.update_entries(cx);
        });
        cx.run_until_parked();

        // All 17 threads shown with Collapse button
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 19); // header + 17 threads + Collapse
        assert!(!entries.iter().any(|e| e.contains("View More")));
        assert!(entries.iter().any(|e| e.contains("Collapse")));

        // Click collapse - should go back to showing 5 threads
        sidebar.update_in(cx, |s, _window, cx| {
            s.expanded_groups.remove(&path_list);
            s.update_entries(cx);
        });
        cx.run_until_parked();

        // Back to initial state: 5 threads + View More
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 7); // header + 5 threads + View More
        assert!(entries.iter().any(|e| e.contains("View More")));
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
            vec!["v [my-project]", "  Thread 1"]
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
            vec!["v [my-project]", "  Thread 1"]
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
                    has_running_threads: false,
                    waiting_thread_count: 0,
                    is_active: true,
                },
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
                    worktree_full_path: None,
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
                    worktree_full_path: None,
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
                    worktree_full_path: None,
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
                    worktree_full_path: None,
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
                    worktree_full_path: None,
                    worktree_highlight_positions: Vec::new(),
                    diff_stats: DiffStats::default(),
                }),
                // View More entry
                ListEntry::ViewMore {
                    path_list: expanded_path.clone(),
                    is_fully_expanded: false,
                },
                // Collapsed project header
                ListEntry::ProjectHeader {
                    path_list: collapsed_path.clone(),
                    label: "collapsed-project".into(),
                    workspace: workspace.clone(),
                    highlight_positions: Vec::new(),
                    has_running_threads: false,
                    waiting_thread_count: 0,
                    is_active: false,
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
                "  + View More",
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

        // Entries: [header, thread3, thread2, thread1]
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

        // Move back up
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
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(3));

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
            vec!["v [my-project]", "  Thread 1"]
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
            vec!["v [my-project]  <== selected", "  Thread 1",]
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

        // Should show header + 5 threads + "View More"
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 7);
        assert!(entries.iter().any(|e| e.contains("View More")));

        // Focus sidebar (selection starts at None), then navigate down to the "View More" entry (index 6)
        open_and_focus_sidebar(&sidebar, cx);
        for _ in 0..7 {
            cx.dispatch_action(SelectNext);
        }
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(6));

        // Confirm on "View More" to expand
        cx.dispatch_action(Confirm);
        cx.run_until_parked();

        // All 8 threads should now be visible with a "Collapse" button
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 10); // header + 8 threads + Collapse button
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
            vec!["v [my-project]", "  Thread 1"]
        );

        // Focus sidebar and manually select the header (index 0). Press left to collapse.
        open_and_focus_sidebar(&sidebar, cx);
        sidebar.update_in(cx, |sidebar, _window, _cx| {
            sidebar.selection = Some(0);
        });

        cx.dispatch_action(SelectParent);
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["> [my-project]  <== selected"]
        );

        // Press right to expand
        cx.dispatch_action(SelectChild);
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]  <== selected", "  Thread 1",]
        );

        // Press right again on already-expanded header moves selection down
        cx.dispatch_action(SelectChild);
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
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]", "  Thread 1  <== selected",]
        );

        // Pressing left on a child collapses the parent group and selects it
        cx.dispatch_action(SelectParent);
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

        // An empty project has the header and a new thread button.
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

        // Focus sidebar (selection starts at None), navigate down to the thread (index 1)
        open_and_focus_sidebar(&sidebar, cx);
        cx.dispatch_action(SelectNext);
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

        // Collapse the group, which removes the thread from the list
        cx.dispatch_action(SelectParent);
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
            SidebarThreadMetadataStore::init_global(cx);
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
        entries[1..].sort();
        assert_eq!(
            entries,
            vec!["v [my-project]", "  Hello *", "  Hello * (running)",]
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
            vec!["v [project-a]", "  Hello * (running)",]
        );

        // Complete thread A's turn (transition Running → Completed).
        connection_a.end_turn(session_id_a.clone(), acp::StopReason::EndTurn);
        cx.run_until_parked();

        // The completed background thread shows a notification indicator.
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [project-a]", "  Hello * (!)",]
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
            vec!["v [my-project]", "  Alpha thread", "  Beta thread",]
        );

        // User types a search query to filter down.
        open_and_focus_sidebar(&sidebar, cx);
        type_in_search(&sidebar, "alpha", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]", "  Alpha thread  <== selected",]
        );

        // User presses Escape — filter clears, full list is restored.
        // The selection index (1) now points at the first thread entry.
        cx.dispatch_action(Cancel);
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  Alpha thread  <== selected",
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
            mw.create_test_workspace(window, cx).detach();
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
                "  Fix bug in sidebar",
                "  Add tests for editor",
            ]
        );

        // "sidebar" matches a thread in each workspace — both headers stay visible.
        type_in_search(&sidebar, "sidebar", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [project-a]", "  Fix bug in sidebar  <== selected",]
        );

        // "typo" only matches in the second workspace — the first header disappears.
        type_in_search(&sidebar, "typo", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            Vec::<String>::new()
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
            mw.create_test_workspace(window, cx).detach();
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
            vec!["v [alpha-project]", "  Fix bug in sidebar  <== selected",]
        );

        // "alpha sidebar" matches the workspace name "alpha-project" (fuzzy: a-l-p-h-a-s-i-d-e-b-a-r
        // doesn't match) — but does not match either workspace name or any thread.
        // Actually let's test something simpler: a query that matches both a workspace
        // name AND some threads in that workspace. Matching threads should still appear.
        type_in_search(&sidebar, "fix", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [alpha-project]", "  Fix bug in sidebar  <== selected",]
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
        // manually select the header, then press SelectParent to collapse.
        open_and_focus_sidebar(&sidebar, cx);
        sidebar.update_in(cx, |sidebar, _window, _cx| {
            sidebar.selection = Some(0);
        });
        cx.dispatch_action(SelectParent);
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
            mw.create_test_workspace(window, cx).detach();
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
            vec!["v [my-project]", "  Historical Thread",]
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
            vec!["v [my-project]", "  Thread A", "  Thread B",]
        );

        // Keyboard confirm preserves selection.
        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.selection = Some(1);
            sidebar.confirm(&Confirm, window, cx);
        });
        assert_eq!(
            sidebar.read_with(cx, |sidebar, _| sidebar.selection),
            Some(1)
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
            vec!["v [my-project]", "  Hello *"]
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
            vec!["v [my-project]", "  Friendly Greeting with AI *"]
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

        // ── 1. Initial state: focused thread derived from active panel ─────
        sidebar.read_with(cx, |sidebar, _cx| {
            assert_eq!(
                sidebar.focused_thread.as_ref(),
                Some(&session_id_a),
                "The active panel's thread should be focused on startup"
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
            assert!(
                has_thread_entry(sidebar, &session_id_a),
                "The clicked thread should be present in the entries"
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
            assert!(
                has_thread_entry(sidebar, &session_id_b),
                "The cross-workspace thread should be present in the entries"
            );
        });

        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(0, window, cx);
        });
        cx.run_until_parked();

        sidebar.read_with(cx, |sidebar, _cx| {
            assert_eq!(
                sidebar.focused_thread.as_ref(),
                Some(&session_id_a),
                "Switching workspace should seed focused_thread from the new active panel"
            );
            assert!(
                has_thread_entry(sidebar, &session_id_a),
                "The seeded thread should be present in the entries"
            );
        });

        let connection_b2 = StubAgentConnection::new();
        connection_b2.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new(DEFAULT_THREAD_TITLE.into()),
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
                sidebar.focused_thread.as_ref(),
                Some(&session_id_a),
                "Opening a thread in a non-active panel should not change focused_thread"
            );
        });

        workspace_b.update_in(cx, |workspace, window, cx| {
            workspace.focus_handle(cx).focus(window, cx);
        });
        cx.run_until_parked();

        sidebar.read_with(cx, |sidebar, _cx| {
            assert_eq!(
                sidebar.focused_thread.as_ref(),
                Some(&session_id_a),
                "Defocusing the sidebar should not change focused_thread"
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
                sidebar.focused_thread.as_ref(),
                Some(&session_id_b2),
                "Switching workspace should seed focused_thread from the new active panel"
            );
            assert!(
                has_thread_entry(sidebar, &session_id_b2),
                "The seeded thread should be present in the entries"
            );
        });

        // ── 8. Focusing the agent panel thread keeps focused_thread ────
        // Workspace B still has session_id_b2 loaded in the agent panel.
        // Clicking into the thread (simulated by focusing its view) should
        // keep focused_thread since it was already seeded on workspace switch.
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
            assert!(
                has_thread_entry(sidebar, &session_id_b2),
                "The focused thread should be present in the entries"
            );
        });
    }

    #[gpui::test]
    async fn test_new_thread_button_works_after_adding_folder(cx: &mut TestAppContext) {
        let project = init_test_project_with_agent_panel("/project-a", cx).await;
        let fs = cx.update(|cx| <dyn fs::Fs>::global(cx));
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, &project, cx);

        let path_list_a = PathList::new(&[std::path::PathBuf::from("/project-a")]);

        // Start a thread and send a message so it has history.
        let connection = StubAgentConnection::new();
        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Done".into()),
        )]);
        open_thread_with_connection(&panel, connection, cx);
        send_message(&panel, cx);
        let session_id = active_session_id(&panel, cx);
        save_test_thread_metadata(&session_id, path_list_a.clone(), cx).await;
        cx.run_until_parked();

        // Verify the thread appears in the sidebar.
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [project-a]", "  Hello *",]
        );

        // The "New Thread" button should NOT be in "active/draft" state
        // because the panel has a thread with messages.
        sidebar.read_with(cx, |sidebar, _cx| {
            assert!(
                !sidebar.active_thread_is_draft,
                "Panel has a thread with messages, so it should not be a draft"
            );
        });

        // Now add a second folder to the workspace, changing the path_list.
        fs.as_fake()
            .insert_tree("/project-b", serde_json::json!({ "src": {} }))
            .await;
        project
            .update(cx, |project, cx| {
                project.find_or_create_worktree("/project-b", true, cx)
            })
            .await
            .expect("should add worktree");
        cx.run_until_parked();

        // The workspace path_list is now [project-a, project-b]. The old
        // thread was stored under [project-a], so it no longer appears in
        // the sidebar list for this workspace.
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert!(
            !entries.iter().any(|e| e.contains("Hello")),
            "Thread stored under the old path_list should not appear: {:?}",
            entries
        );

        // The "New Thread" button must still be clickable (not stuck in
        // "active/draft" state). Verify that `active_thread_is_draft` is
        // false — the panel still has the old thread with messages.
        sidebar.read_with(cx, |sidebar, _cx| {
            assert!(
                !sidebar.active_thread_is_draft,
                "After adding a folder the panel still has a thread with messages, \
                 so active_thread_is_draft should be false"
            );
        });

        // Actually click "New Thread" by calling create_new_thread and
        // verify a new draft is created.
        let workspace = multi_workspace.read_with(cx, |mw, _cx| mw.workspace().clone());
        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.create_new_thread(&workspace, window, cx);
        });
        cx.run_until_parked();

        // After creating a new thread, the panel should now be in draft
        // state (no messages on the new thread).
        sidebar.read_with(cx, |sidebar, _cx| {
            assert!(
                sidebar.active_thread_is_draft,
                "After creating a new thread the panel should be in draft state"
            );
        });
    }

    #[gpui::test]
    async fn test_cmd_n_shows_new_thread_entry(cx: &mut TestAppContext) {
        // When the user presses Cmd-N (NewThread action) while viewing a
        // non-empty thread, the sidebar should show the "New Thread" entry.
        // This exercises the same code path as the workspace action handler
        // (which bypasses the sidebar's create_new_thread method).
        let project = init_test_project_with_agent_panel("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let (sidebar, panel) = setup_sidebar_with_agent_panel(&multi_workspace, &project, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);

        // Create a non-empty thread (has messages).
        let connection = StubAgentConnection::new();
        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Done".into()),
        )]);
        open_thread_with_connection(&panel, connection, cx);
        send_message(&panel, cx);

        let session_id = active_session_id(&panel, cx);
        save_test_thread_metadata(&session_id, path_list.clone(), cx).await;
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]", "  Hello *"]
        );

        // Simulate cmd-n
        let workspace = multi_workspace.read_with(cx, |mw, _cx| mw.workspace().clone());
        panel.update_in(cx, |panel, window, cx| {
            panel.new_thread(&NewThread, window, cx);
        });
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.focus_panel::<AgentPanel>(window, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]", "  [+ New Thread]", "  Hello *"],
            "After Cmd-N the sidebar should show a highlighted New Thread entry"
        );

        sidebar.read_with(cx, |sidebar, _cx| {
            assert!(
                sidebar.focused_thread.is_none(),
                "focused_thread should be cleared after Cmd-N"
            );
            assert!(
                sidebar.active_thread_is_draft,
                "the new blank thread should be a draft"
            );
        });
    }

    #[gpui::test]
    async fn test_cmd_n_shows_new_thread_entry_in_absorbed_worktree(cx: &mut TestAppContext) {
        // When the active workspace is an absorbed git worktree, cmd-n
        // should still show the "New Thread" entry under the main repo's
        // header and highlight it as active.
        agent_ui::test_support::init_test(cx);
        cx.update(|cx| {
            cx.update_flags(false, vec!["agent-v2".into()]);
            ThreadStore::init_global(cx);
            SidebarThreadMetadataStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            prompt_store::init(cx);
        });

        let fs = FakeFs::new(cx.executor());

        // Main repo with a linked worktree.
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

        // Worktree checkout pointing back to the main repo.
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
                ref_name: Some("refs/heads/feature-a".into()),
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

        let worktree_panel = add_agent_panel(&worktree_workspace, &worktree_project, cx);

        // Switch to the worktree workspace.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(1, window, cx);
        });

        let sidebar = setup_sidebar(&multi_workspace, cx);

        // Create a non-empty thread in the worktree workspace.
        let connection = StubAgentConnection::new();
        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Done".into()),
        )]);
        open_thread_with_connection(&worktree_panel, connection, cx);
        send_message(&worktree_panel, cx);

        let session_id = active_session_id(&worktree_panel, cx);
        let wt_path_list = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
        save_test_thread_metadata(&session_id, wt_path_list, cx).await;
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [project]", "  Hello {wt-feature-a} *"]
        );

        // Simulate Cmd-N in the worktree workspace.
        worktree_panel.update_in(cx, |panel, window, cx| {
            panel.new_thread(&NewThread, window, cx);
        });
        worktree_workspace.update_in(cx, |workspace, window, cx| {
            workspace.focus_panel::<AgentPanel>(window, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [project]",
                "  [+ New Thread]",
                "  Hello {wt-feature-a} *"
            ],
            "After Cmd-N in an absorbed worktree, the sidebar should show \
             a highlighted New Thread entry under the main repo header"
        );

        sidebar.read_with(cx, |sidebar, _cx| {
            assert!(
                sidebar.focused_thread.is_none(),
                "focused_thread should be cleared after Cmd-N"
            );
            assert!(
                sidebar.active_thread_is_draft,
                "the new blank thread should be a draft"
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
                    ref_name: Some("refs/heads/rosewood".into()),
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
                    ref_name: Some("refs/heads/rosewood".into()),
                    sha: "abc".into(),
                });
            })
            .unwrap();

        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [project]", "  Worktree Thread {rosewood}",]
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
                "  Thread A",
                "v [wt-feature-b]",
                "  Thread B",
            ]
        );

        // Configure the main repo to list both worktrees before opening
        // it so the initial git scan picks them up.
        fs.with_git_state(std::path::Path::new("/project/.git"), false, |state| {
            state.worktrees.push(git::repository::Worktree {
                path: std::path::PathBuf::from("/wt-feature-a"),
                ref_name: Some("refs/heads/feature-a".into()),
                sha: "aaa".into(),
            });
            state.worktrees.push(git::repository::Worktree {
                path: std::path::PathBuf::from("/wt-feature-b"),
                ref_name: Some("refs/heads/feature-b".into()),
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
            vec!["v [project]", "  Thread A {wt-feature-a}",]
        );
    }

    #[gpui::test]
    async fn test_absorbed_worktree_running_thread_shows_live_status(cx: &mut TestAppContext) {
        // When a worktree workspace is absorbed under the main repo, a
        // running thread in the worktree's agent panel should still show
        // live status (spinner + "(running)") in the sidebar.
        agent_ui::test_support::init_test(cx);
        cx.update(|cx| {
            cx.update_flags(false, vec!["agent-v2".into()]);
            ThreadStore::init_global(cx);
            SidebarThreadMetadataStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            prompt_store::init(cx);
        });

        let fs = FakeFs::new(cx.executor());

        // Main repo with a linked worktree.
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

        // Worktree checkout pointing back to the main repo.
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
                ref_name: Some("refs/heads/feature-a".into()),
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

        // Create the MultiWorkspace with both projects.
        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            MultiWorkspace::test_new(main_project.clone(), window, cx)
        });

        let worktree_workspace = multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(worktree_project.clone(), window, cx)
        });

        // Add an agent panel to the worktree workspace so we can run a
        // thread inside it.
        let worktree_panel = add_agent_panel(&worktree_workspace, &worktree_project, cx);

        // Switch back to the main workspace before setting up the sidebar.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(0, window, cx);
        });

        let sidebar = setup_sidebar(&multi_workspace, cx);

        // Start a thread in the worktree workspace's panel and keep it
        // generating (don't resolve it).
        let connection = StubAgentConnection::new();
        open_thread_with_connection(&worktree_panel, connection.clone(), cx);
        send_message(&worktree_panel, cx);

        let session_id = active_session_id(&worktree_panel, cx);

        // Save metadata so the sidebar knows about this thread.
        let wt_paths = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
        save_test_thread_metadata(&session_id, wt_paths, cx).await;

        // Keep the thread generating by sending a chunk without ending
        // the turn.
        cx.update(|_, cx| {
            connection.send_update(
                session_id.clone(),
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("working...".into())),
                cx,
            );
        });
        cx.run_until_parked();

        // The worktree thread should be absorbed under the main project
        // and show live running status.
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(
            entries,
            vec!["v [project]", "  Hello {wt-feature-a} * (running)",]
        );
    }

    #[gpui::test]
    async fn test_absorbed_worktree_completion_triggers_notification(cx: &mut TestAppContext) {
        agent_ui::test_support::init_test(cx);
        cx.update(|cx| {
            cx.update_flags(false, vec!["agent-v2".into()]);
            ThreadStore::init_global(cx);
            SidebarThreadMetadataStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            prompt_store::init(cx);
        });

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
                ref_name: Some("refs/heads/feature-a".into()),
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

        let worktree_panel = add_agent_panel(&worktree_workspace, &worktree_project, cx);

        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(0, window, cx);
        });

        let sidebar = setup_sidebar(&multi_workspace, cx);

        let connection = StubAgentConnection::new();
        open_thread_with_connection(&worktree_panel, connection.clone(), cx);
        send_message(&worktree_panel, cx);

        let session_id = active_session_id(&worktree_panel, cx);
        let wt_paths = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
        save_test_thread_metadata(&session_id, wt_paths, cx).await;

        cx.update(|_, cx| {
            connection.send_update(
                session_id.clone(),
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("working...".into())),
                cx,
            );
        });
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [project]", "  Hello {wt-feature-a} * (running)",]
        );

        connection.end_turn(session_id, acp::StopReason::EndTurn);
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [project]", "  Hello {wt-feature-a} * (!)",]
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
                ref_name: Some("refs/heads/feature-a".into()),
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
            vec!["v [project]", "  WT Thread {wt-feature-a}"],
        );

        // Only 1 workspace should exist.
        assert_eq!(
            multi_workspace.read_with(cx, |mw, _| mw.workspaces().len()),
            1,
        );

        // Focus the sidebar and select the worktree thread.
        open_and_focus_sidebar(&sidebar, cx);
        sidebar.update_in(cx, |sidebar, _window, _cx| {
            sidebar.selection = Some(1); // index 0 is header, 1 is the thread
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
    async fn test_clicking_worktree_thread_does_not_briefly_render_as_separate_project(
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
                ref_name: Some("refs/heads/feature-a".into()),
                sha: "aaa".into(),
            });
        })
        .unwrap();

        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let main_project = project::Project::test(fs.clone(), ["/project".as_ref()], cx).await;
        main_project
            .update(cx, |p, cx| p.git_scans_complete(cx))
            .await;

        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            MultiWorkspace::test_new(main_project.clone(), window, cx)
        });
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let paths_wt = PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]);
        save_named_thread_metadata("thread-wt", "WT Thread", &paths_wt, cx).await;

        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [project]", "  WT Thread {wt-feature-a}"],
        );

        open_and_focus_sidebar(&sidebar, cx);
        sidebar.update_in(cx, |sidebar, _window, _cx| {
            sidebar.selection = Some(1);
        });

        let assert_sidebar_state = |sidebar: &mut Sidebar, _cx: &mut Context<Sidebar>| {
            let mut project_headers = sidebar.contents.entries.iter().filter_map(|entry| {
                if let ListEntry::ProjectHeader { label, .. } = entry {
                    Some(label.as_ref())
                } else {
                    None
                }
            });

            let Some(project_header) = project_headers.next() else {
                panic!("expected exactly one sidebar project header named `project`, found none");
            };
            assert_eq!(
                project_header, "project",
                "expected the only sidebar project header to be `project`"
            );
            if let Some(unexpected_header) = project_headers.next() {
                panic!(
                    "expected exactly one sidebar project header named `project`, found extra header `{unexpected_header}`"
                );
            }

            let mut saw_expected_thread = false;
            for entry in &sidebar.contents.entries {
                match entry {
                    ListEntry::ProjectHeader { label, .. } => {
                        assert_eq!(
                            label.as_ref(),
                            "project",
                            "expected the only sidebar project header to be `project`"
                        );
                    }
                    ListEntry::Thread(thread)
                        if thread
                            .session_info
                            .title
                            .as_ref()
                            .map(|title| title.as_ref())
                            == Some("WT Thread")
                            && thread.worktree_name.as_ref().map(|name| name.as_ref())
                                == Some("wt-feature-a") =>
                    {
                        saw_expected_thread = true;
                    }
                    ListEntry::Thread(thread) => {
                        let title = thread
                            .session_info
                            .title
                            .as_ref()
                            .map(|title| title.as_ref())
                            .unwrap_or("Untitled");
                        let worktree_name = thread
                            .worktree_name
                            .as_ref()
                            .map(|name| name.as_ref())
                            .unwrap_or("<none>");
                        panic!(
                            "unexpected sidebar thread while opening linked worktree thread: title=`{title}`, worktree=`{worktree_name}`"
                        );
                    }
                    ListEntry::ViewMore { .. } => {
                        panic!("unexpected `View More` entry while opening linked worktree thread");
                    }
                    ListEntry::NewThread { .. } => {
                        panic!(
                            "unexpected `New Thread` entry while opening linked worktree thread"
                        );
                    }
                }
            }

            assert!(
                saw_expected_thread,
                "expected the sidebar to keep showing `WT Thread {{wt-feature-a}}` under `project`"
            );
        };

        sidebar
            .update(cx, |_, cx| cx.observe_self(assert_sidebar_state))
            .detach();

        let window = cx.windows()[0];
        cx.update_window(window, |_, window, cx| {
            window.dispatch_action(Confirm.boxed_clone(), cx);
        })
        .unwrap();

        cx.run_until_parked();

        sidebar.update(cx, assert_sidebar_state);
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
                ref_name: Some("refs/heads/feature-a".into()),
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
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0], "v [project]");
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

    #[gpui::test]
    async fn test_activate_archived_thread_reuses_workspace_in_another_window(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
            .await;
        fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
            .await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
        let project_b = project::Project::test(fs.clone(), ["/project-b".as_ref()], cx).await;

        let multi_workspace_a =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project_a, window, cx));
        let multi_workspace_b =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project_b, window, cx));

        let multi_workspace_a_entity = multi_workspace_a.root(cx).unwrap();

        let cx_a = &mut gpui::VisualTestContext::from_window(multi_workspace_a.into(), cx);
        let sidebar = setup_sidebar(&multi_workspace_a_entity, cx_a);

        let session_id = acp::SessionId::new(Arc::from("archived-cross-window"));

        sidebar.update_in(cx_a, |sidebar, window, cx| {
            sidebar.activate_archived_thread(
                Agent::NativeAgent,
                acp_thread::AgentSessionInfo {
                    session_id: session_id.clone(),
                    work_dirs: Some(PathList::new(&[PathBuf::from("/project-b")])),
                    title: Some("Cross Window Thread".into()),
                    updated_at: None,
                    created_at: None,
                    meta: None,
                },
                window,
                cx,
            );
        });
        cx_a.run_until_parked();

        assert_eq!(
            multi_workspace_a
                .read_with(cx_a, |mw, _| mw.workspaces().len())
                .unwrap(),
            1,
            "should not add the other window's workspace into the current window"
        );
        assert_eq!(
            multi_workspace_b
                .read_with(cx_a, |mw, _| mw.workspaces().len())
                .unwrap(),
            1,
            "should reuse the existing workspace in the other window"
        );
        assert!(
            cx_a.read(|cx| cx.active_window().unwrap()) == *multi_workspace_b,
            "should activate the window that already owns the matching workspace"
        );
        sidebar.read_with(cx_a, |sidebar, _| {
            assert_eq!(
                sidebar.focused_thread, None,
                "source window's sidebar should not eagerly claim focus for a thread opened in another window"
            );
        });
    }

    #[gpui::test]
    async fn test_activate_archived_thread_reuses_workspace_in_another_window_with_target_sidebar(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
            .await;
        fs.insert_tree("/project-b", serde_json::json!({ "src": {} }))
            .await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
        let project_b = project::Project::test(fs.clone(), ["/project-b".as_ref()], cx).await;

        let multi_workspace_a =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project_a, window, cx));
        let multi_workspace_b =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project_b.clone(), window, cx));

        let multi_workspace_a_entity = multi_workspace_a.root(cx).unwrap();
        let multi_workspace_b_entity = multi_workspace_b.root(cx).unwrap();

        let cx_a = &mut gpui::VisualTestContext::from_window(multi_workspace_a.into(), cx);
        let sidebar_a = setup_sidebar(&multi_workspace_a_entity, cx_a);

        let cx_b = &mut gpui::VisualTestContext::from_window(multi_workspace_b.into(), cx);
        let sidebar_b = setup_sidebar(&multi_workspace_b_entity, cx_b);
        let workspace_b = multi_workspace_b_entity.read_with(cx_b, |mw, _| mw.workspace().clone());
        let _panel_b = add_agent_panel(&workspace_b, &project_b, cx_b);

        let session_id = acp::SessionId::new(Arc::from("archived-cross-window-with-sidebar"));

        sidebar_a.update_in(cx_a, |sidebar, window, cx| {
            sidebar.activate_archived_thread(
                Agent::NativeAgent,
                acp_thread::AgentSessionInfo {
                    session_id: session_id.clone(),
                    work_dirs: Some(PathList::new(&[PathBuf::from("/project-b")])),
                    title: Some("Cross Window Thread".into()),
                    updated_at: None,
                    created_at: None,
                    meta: None,
                },
                window,
                cx,
            );
        });
        cx_a.run_until_parked();

        assert_eq!(
            multi_workspace_a
                .read_with(cx_a, |mw, _| mw.workspaces().len())
                .unwrap(),
            1,
            "should not add the other window's workspace into the current window"
        );
        assert_eq!(
            multi_workspace_b
                .read_with(cx_a, |mw, _| mw.workspaces().len())
                .unwrap(),
            1,
            "should reuse the existing workspace in the other window"
        );
        assert!(
            cx_a.read(|cx| cx.active_window().unwrap()) == *multi_workspace_b,
            "should activate the window that already owns the matching workspace"
        );
        sidebar_a.read_with(cx_a, |sidebar, _| {
            assert_eq!(
                sidebar.focused_thread, None,
                "source window's sidebar should not eagerly claim focus for a thread opened in another window"
            );
        });
        sidebar_b.read_with(cx_b, |sidebar, _| {
            assert_eq!(
                sidebar.focused_thread.as_ref(),
                Some(&session_id),
                "target window's sidebar should eagerly focus the activated archived thread"
            );
        });
    }

    #[gpui::test]
    async fn test_activate_archived_thread_prefers_current_window_for_matching_paths(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project-a", serde_json::json!({ "src": {} }))
            .await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let project_b = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;
        let project_a = project::Project::test(fs.clone(), ["/project-a".as_ref()], cx).await;

        let multi_workspace_b =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project_b, window, cx));
        let multi_workspace_a =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project_a, window, cx));

        let multi_workspace_a_entity = multi_workspace_a.root(cx).unwrap();

        let cx_a = &mut gpui::VisualTestContext::from_window(multi_workspace_a.into(), cx);
        let sidebar_a = setup_sidebar(&multi_workspace_a_entity, cx_a);

        let session_id = acp::SessionId::new(Arc::from("archived-current-window"));

        sidebar_a.update_in(cx_a, |sidebar, window, cx| {
            sidebar.activate_archived_thread(
                Agent::NativeAgent,
                acp_thread::AgentSessionInfo {
                    session_id: session_id.clone(),
                    work_dirs: Some(PathList::new(&[PathBuf::from("/project-a")])),
                    title: Some("Current Window Thread".into()),
                    updated_at: None,
                    created_at: None,
                    meta: None,
                },
                window,
                cx,
            );
        });
        cx_a.run_until_parked();

        assert!(
            cx_a.read(|cx| cx.active_window().unwrap()) == *multi_workspace_a,
            "should keep activation in the current window when it already has a matching workspace"
        );
        sidebar_a.read_with(cx_a, |sidebar, _| {
            assert_eq!(
                sidebar.focused_thread.as_ref(),
                Some(&session_id),
                "current window's sidebar should eagerly focus the activated archived thread"
            );
        });
        assert_eq!(
            multi_workspace_a
                .read_with(cx_a, |mw, _| mw.workspaces().len())
                .unwrap(),
            1,
            "current window should continue reusing its existing workspace"
        );
        assert_eq!(
            multi_workspace_b
                .read_with(cx_a, |mw, _| mw.workspaces().len())
                .unwrap(),
            1,
            "other windows should not be activated just because they also match the saved paths"
        );
    }

    #[gpui::test]
    async fn test_archive_thread_uses_next_threads_own_workspace(cx: &mut TestAppContext) {
        // Regression test: archive_thread previously always loaded the next thread
        // through group_workspace (the main workspace's ProjectHeader), even when
        // the next thread belonged to an absorbed linked-worktree workspace. That
        // caused the worktree thread to be loaded in the main panel, which bound it
        // to the main project and corrupted its stored folder_paths.
        //
        // The fix: use next.workspace (ThreadEntryWorkspace::Open) when available,
        // falling back to group_workspace only for Closed workspaces.
        agent_ui::test_support::init_test(cx);
        cx.update(|cx| {
            cx.update_flags(false, vec!["agent-v2".into()]);
            ThreadStore::init_global(cx);
            SidebarThreadMetadataStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            prompt_store::init(cx);
        });

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
                ref_name: Some("refs/heads/feature-a".into()),
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

        // Activate main workspace so the sidebar tracks the main panel.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(0, window, cx);
        });

        let sidebar = setup_sidebar(&multi_workspace, cx);

        let main_workspace = multi_workspace.read_with(cx, |mw, _| mw.workspaces()[0].clone());
        let main_panel = add_agent_panel(&main_workspace, &main_project, cx);
        let _worktree_panel = add_agent_panel(&worktree_workspace, &worktree_project, cx);

        // Open Thread 2 in the main panel and keep it running.
        let connection = StubAgentConnection::new();
        open_thread_with_connection(&main_panel, connection.clone(), cx);
        send_message(&main_panel, cx);

        let thread2_session_id = active_session_id(&main_panel, cx);

        cx.update(|_, cx| {
            connection.send_update(
                thread2_session_id.clone(),
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("working...".into())),
                cx,
            );
        });

        // Save thread 2's metadata with a newer timestamp so it sorts above thread 1.
        save_thread_metadata(
            thread2_session_id.clone(),
            "Thread 2".into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
            PathList::new(&[std::path::PathBuf::from("/project")]),
            cx,
        )
        .await;

        // Save thread 1's metadata with the worktree path and an older timestamp so
        // it sorts below thread 2. archive_thread will find it as the "next" candidate.
        let thread1_session_id = acp::SessionId::new(Arc::from("thread1-worktree-session"));
        save_thread_metadata(
            thread1_session_id.clone(),
            "Thread 1".into(),
            chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
            PathList::new(&[std::path::PathBuf::from("/wt-feature-a")]),
            cx,
        )
        .await;

        cx.run_until_parked();

        // Verify the sidebar absorbed thread 1 under [project] with the worktree chip.
        let entries_before = visible_entries_as_strings(&sidebar, cx);
        assert!(
            entries_before.iter().any(|s| s.contains("{wt-feature-a}")),
            "Thread 1 should appear with the linked-worktree chip before archiving: {:?}",
            entries_before
        );

        // The sidebar should track T2 as the focused thread (derived from the
        // main panel's active view).
        let focused = sidebar.read_with(cx, |s, _| s.focused_thread.clone());
        assert_eq!(
            focused,
            Some(thread2_session_id.clone()),
            "focused thread should be Thread 2 before archiving: {:?}",
            focused
        );

        // Archive thread 2.
        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.archive_thread(&thread2_session_id, window, cx);
        });

        cx.run_until_parked();

        // The main panel's active thread must still be thread 2.
        let main_active = main_panel.read_with(cx, |panel, cx| {
            panel
                .active_agent_thread(cx)
                .map(|t| t.read(cx).session_id().clone())
        });
        assert_eq!(
            main_active,
            Some(thread2_session_id.clone()),
            "main panel should not have been taken over by loading the linked-worktree thread T1; \
             before the fix, archive_thread used group_workspace instead of next.workspace, \
             causing T1 to be loaded in the wrong panel"
        );

        // Thread 1 should still appear in the sidebar with its worktree chip
        // (Thread 2 was archived so it is gone from the list).
        let entries_after = visible_entries_as_strings(&sidebar, cx);
        assert!(
            entries_after.iter().any(|s| s.contains("{wt-feature-a}")),
            "T1 should still carry its linked-worktree chip after archiving T2: {:?}",
            entries_after
        );
    }
}
