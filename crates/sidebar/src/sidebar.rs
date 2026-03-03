use acp_thread::ThreadStatus;
use agent_ui::{AgentPanel, AgentPanelEvent};
use chrono::{Datelike, Local, NaiveDate, TimeDelta};

use fs::Fs;
use fuzzy::StringMatchCandidate;
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, Pixels, Render, SharedString,
    Subscription, Task, Window, px,
};
use picker::{Picker, PickerDelegate};
use project::git_store::{GitStoreEvent, RepositoryEvent};
use project::Event as ProjectEvent;
use recent_projects::{RecentProjectEntry, get_recent_projects};
use std::fmt::Display;

use std::collections::{HashMap, HashSet};

use std::path::{Path, PathBuf};
use std::sync::Arc;
use theme::ActiveTheme;
use ui::utils::TRAFFIC_LIGHT_PADDING;
use ui::{
    AgentThreadStatus, Divider, DividerColor, KeyBinding, ListSubHeader, Tab, ThreadItem, Tooltip,
    prelude::*,
};
use ui_input::ErasedEditor;
use util::ResultExt as _;
use workspace::{
    FocusWorkspaceSidebar, MultiWorkspace, NewWorkspaceInWindow, Sidebar as WorkspaceSidebar,
    SidebarEvent, ToggleWorkspaceSidebar, Workspace,
};

#[derive(Clone, Debug)]
struct AgentThreadInfo {
    title: SharedString,
    status: AgentThreadStatus,
    icon: IconName,
}

const DEFAULT_WIDTH: Pixels = px(320.0);
const MIN_WIDTH: Pixels = px(200.0);
const MAX_WIDTH: Pixels = px(800.0);
const MAX_MATCHES: usize = 100;

#[derive(Clone)]
struct WorkspaceThreadEntry {
    index: usize,
    worktree_label: SharedString,
    full_path: SharedString,
    common_dir: Option<Arc<Path>>,
    is_prime: bool,
    thread_info: Option<AgentThreadInfo>,
}

impl WorkspaceThreadEntry {
    fn new(index: usize, workspace: &Entity<Workspace>, cx: &App) -> Self {
        let workspace_ref = workspace.read(cx);
        let project = workspace_ref.project().read(cx);
        let git_store = project.git_store().read(cx);

        let worktrees: Vec<_> = workspace_ref
            .worktrees(cx)
            .filter(|worktree| worktree.read(cx).is_visible())
            .map(|worktree| worktree.read(cx).abs_path())
            .collect();

        let best_repo = Self::best_matching_repo(&worktrees, git_store, cx);
        let common_dir = best_repo.as_ref().and_then(|s| s.common_dir_abs_path.clone());
        let is_prime = match (&common_dir, worktrees.first()) {
            (Some(cd), Some(worktree_path)) => {
                cd.parent().map_or(false, |repo_root| repo_root == worktree_path.as_ref())
            }
            _ => true,
        };

        Self {
            index,
            worktree_label: Self::compute_label(index, &worktrees, best_repo.as_ref()),
            full_path: worktrees
                .iter()
                .map(|path| path.to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join("\n")
                .into(),
            common_dir,
            is_prime,
            thread_info: Self::thread_info(workspace, cx),
        }
    }

    fn best_matching_repo(
        worktrees: &[Arc<Path>],
        git_store: &project::git_store::GitStore,
        cx: &App,
    ) -> Option<project::git_store::RepositorySnapshot> {
        worktrees.first().and_then(|path| {
            git_store
                .repositories()
                .values()
                .filter_map(|repo| {
                    let repo = repo.read(cx);
                    let repo_path = &*repo.work_directory_abs_path;
                    if path.as_ref() == repo_path || path.starts_with(repo_path) {
                        Some((repo_path.as_os_str().len(), repo.snapshot()))
                    } else {
                        None
                    }
                })
                .max_by_key(|(len, _)| *len)
                .map(|(_, snapshot)| snapshot)
        })
    }

    fn compute_label(
        index: usize,
        worktrees: &[Arc<Path>],
        best_repo: Option<&project::git_store::RepositorySnapshot>,
    ) -> SharedString {
        let names: Vec<String> = worktrees
            .iter()
            .filter_map(|path| {
                let dir_name = path.file_name()?.to_string_lossy();
                if let Some(snapshot) = best_repo {
                    let repo_name = snapshot
                        .common_dir_abs_path
                        .as_ref()
                        .and_then(|p| p.parent())
                        .and_then(|p| p.file_name())
                        .map(|n| n.to_string_lossy());
                    let branch = snapshot.branch.as_ref().map(|b| b.name().to_string());
                    match (repo_name, branch) {
                        (Some(repo), Some(branch)) => Some(format!("{repo} / {branch}")),
                        (Some(repo), None) => Some(repo.to_string()),
                        _ => Some(dir_name.to_string()),
                    }
                } else {
                    Some(dir_name.to_string())
                }
            })
            .collect();

        if names.is_empty() {
            format!("Workspace {}", index + 1).into()
        } else {
            names.join(", ").into()
        }
    }

    fn thread_info(workspace: &Entity<Workspace>, cx: &App) -> Option<AgentThreadInfo> {
        let agent_panel = workspace.read(cx).panel::<AgentPanel>(cx)?;
        let agent_panel_ref = agent_panel.read(cx);

        let thread_view = agent_panel_ref.as_active_thread_view(cx)?.read(cx);
        let thread = thread_view.thread.read(cx);

        let icon = thread_view.agent_icon;
        let title = thread.title();

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
        Some(AgentThreadInfo {
            title,
            status,
            icon,
        })
    }
}

#[derive(Clone)]
enum SidebarEntry {
    Separator(SharedString),
    WorkspaceThread(WorkspaceThreadEntry),
    RecentProject(RecentProjectEntry),
}

impl SidebarEntry {
    fn searchable_text(&self) -> &str {
        match self {
            SidebarEntry::Separator(_) => "",
            SidebarEntry::WorkspaceThread(entry) => entry.worktree_label.as_ref(),
            SidebarEntry::RecentProject(entry) => entry.name.as_ref(),
        }
    }
}

#[derive(Clone)]
struct SidebarMatch {
    entry: SidebarEntry,
    positions: Vec<usize>,
}

struct WorkspacePickerDelegate {
    multi_workspace: Entity<MultiWorkspace>,
    entries: Vec<SidebarEntry>,
    active_workspace_index: usize,
    workspace_thread_count: usize,
    /// All recent projects including what's filtered out of entries
    /// used to add unopened projects to entries on rebuild
    recent_projects: Vec<RecentProjectEntry>,
    recent_project_thread_titles: HashMap<SharedString, SharedString>,
    matches: Vec<SidebarMatch>,
    selected_index: usize,
    query: String,
    hovered_thread_item: Option<usize>,
    notified_workspaces: HashSet<usize>,
}

impl WorkspacePickerDelegate {
    fn new(multi_workspace: Entity<MultiWorkspace>) -> Self {
        Self {
            multi_workspace,
            entries: Vec::new(),
            active_workspace_index: 0,
            workspace_thread_count: 0,
            recent_projects: Vec::new(),
            recent_project_thread_titles: HashMap::new(),
            matches: Vec::new(),
            selected_index: 0,
            query: String::new(),
            hovered_thread_item: None,
            notified_workspaces: HashSet::new(),
        }
    }

    fn set_entries(
        &mut self,
        workspace_threads: Vec<WorkspaceThreadEntry>,
        active_workspace_index: usize,
        cx: &App,
    ) {
        if let Some(hovered_index) = self.hovered_thread_item {
            let still_exists = workspace_threads
                .iter()
                .any(|thread| thread.index == hovered_index);
            if !still_exists {
                self.hovered_thread_item = None;
            }
        }

        let old_statuses: HashMap<usize, AgentThreadStatus> = self
            .entries
            .iter()
            .filter_map(|entry| match entry {
                SidebarEntry::WorkspaceThread(thread) => thread
                    .thread_info
                    .as_ref()
                    .map(|info| (thread.index, info.status)),
                _ => None,
            })
            .collect();

        for thread in &workspace_threads {
            if let Some(info) = &thread.thread_info {
                if info.status == AgentThreadStatus::Completed
                    && thread.index != active_workspace_index
                {
                    if old_statuses.get(&thread.index) == Some(&AgentThreadStatus::Running) {
                        self.notified_workspaces.insert(thread.index);
                    }
                }
            }
        }

        if self.active_workspace_index != active_workspace_index {
            self.notified_workspaces.remove(&active_workspace_index);
        }
        self.active_workspace_index = active_workspace_index;
        self.workspace_thread_count = workspace_threads.len();
        self.rebuild_entries(workspace_threads, cx);
    }

    fn set_recent_projects(&mut self, recent_projects: Vec<RecentProjectEntry>, cx: &App) {
        self.recent_project_thread_titles.clear();

        self.recent_projects = recent_projects;

        let workspace_threads: Vec<WorkspaceThreadEntry> = self
            .entries
            .iter()
            .filter_map(|entry| match entry {
                SidebarEntry::WorkspaceThread(thread) => Some(thread.clone()),
                _ => None,
            })
            .collect();
        self.rebuild_entries(workspace_threads, cx);
    }

    fn open_workspace_path_sets(&self, cx: &App) -> Vec<Vec<Arc<Path>>> {
        self.multi_workspace
            .read(cx)
            .workspaces()
            .iter()
            .map(|workspace| {
                let mut paths = workspace.read(cx).root_paths(cx);
                paths.sort();
                paths
            })
            .collect()
    }

    fn rebuild_entries(&mut self, workspace_threads: Vec<WorkspaceThreadEntry>, cx: &App) {
        let open_path_sets = self.open_workspace_path_sets(cx);

        self.entries.clear();

        if !workspace_threads.is_empty() {
            self.entries
                .push(SidebarEntry::Separator("Active Workspaces".into()));

            // Sort by common_dir so related worktrees are adjacent, prime first.
            let mut sorted = workspace_threads;
            sorted.sort_by(|a, b| {
                a.common_dir.cmp(&b.common_dir)
                    .then(b.is_prime.cmp(&a.is_prime))
                    .then(a.worktree_label.cmp(&b.worktree_label))
            });

            for thread in sorted {
                self.entries.push(SidebarEntry::WorkspaceThread(thread));
            }
        }

        let recent: Vec<_> = self
            .recent_projects
            .iter()
            .filter(|project| {
                let mut project_paths: Vec<&Path> =
                    project.paths.iter().map(|p| p.as_path()).collect();
                project_paths.sort();
                !open_path_sets.iter().any(|open_paths| {
                    open_paths.len() == project_paths.len()
                        && open_paths
                            .iter()
                            .zip(&project_paths)
                            .all(|(a, b)| a.as_ref() == *b)
                })
            })
            .cloned()
            .collect();

        if !recent.is_empty() {
            let today = Local::now().naive_local().date();
            let mut current_bucket: Option<TimeBucket> = None;

            for project in recent {
                let entry_date = project.timestamp.with_timezone(&Local).naive_local().date();
                let bucket = TimeBucket::from_dates(today, entry_date);

                if current_bucket != Some(bucket) {
                    current_bucket = Some(bucket);
                    self.entries
                        .push(SidebarEntry::Separator(bucket.to_string().into()));
                }

                self.entries.push(SidebarEntry::RecentProject(project));
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TimeBucket {
    Today,
    Yesterday,
    ThisWeek,
    PastWeek,
    All,
}

impl TimeBucket {
    fn from_dates(reference: NaiveDate, date: NaiveDate) -> Self {
        if date == reference {
            return TimeBucket::Today;
        }

        if date == reference - TimeDelta::days(1) {
            return TimeBucket::Yesterday;
        }

        let week = date.iso_week();

        if reference.iso_week() == week {
            return TimeBucket::ThisWeek;
        }

        let last_week = (reference - TimeDelta::days(7)).iso_week();

        if week == last_week {
            return TimeBucket::PastWeek;
        }

        TimeBucket::All
    }
}

impl Display for TimeBucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeBucket::Today => write!(f, "Today"),
            TimeBucket::Yesterday => write!(f, "Yesterday"),
            TimeBucket::ThisWeek => write!(f, "This Week"),
            TimeBucket::PastWeek => write!(f, "Past Week"),
            TimeBucket::All => write!(f, "All"),
        }
    }
}

fn open_recent_project(paths: Vec<PathBuf>, window: &mut Window, cx: &mut App) {
    let Some(handle) = window.window_handle().downcast::<MultiWorkspace>() else {
        return;
    };

    cx.defer(move |cx| {
        if let Some(task) = handle
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.open_project(paths, window, cx)
            })
            .log_err()
        {
            task.detach_and_log_err(cx);
        }
    });
}

impl PickerDelegate for WorkspacePickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn can_select(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> bool {
        match self.matches.get(ix) {
            Some(SidebarMatch {
                entry: SidebarEntry::Separator(_),
                ..
            }) => false,
            _ => true,
        }
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search…".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        if self.query.is_empty() {
            None
        } else {
            Some("No threads match your search.".into())
        }
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let query_changed = self.query != query;
        self.query = query.clone();
        if query_changed {
            self.hovered_thread_item = None;
        }
        let entries = self.entries.clone();

        if query.is_empty() {
            self.matches = entries
                .into_iter()
                .map(|entry| SidebarMatch {
                    entry,
                    positions: Vec::new(),
                })
                .collect();

            self.selected_index = self
                .matches
                .iter()
                .position(|m| match &m.entry {
                    SidebarEntry::WorkspaceThread(t) => t.index == self.active_workspace_index,
                    _ => false,
                })
                .unwrap_or(0);
            return Task::ready(());
        }

        let executor = cx.background_executor().clone();
        cx.spawn_in(window, async move |picker, cx| {
            let matches = cx
                .background_spawn(async move {
                    let data_entries: Vec<(usize, &SidebarEntry)> = entries
                        .iter()
                        .enumerate()
                        .filter(|(_, entry)| !matches!(entry, SidebarEntry::Separator(_)))
                        .collect();

                    let candidates: Vec<StringMatchCandidate> = data_entries
                        .iter()
                        .enumerate()
                        .map(|(candidate_index, (_, entry))| {
                            StringMatchCandidate::new(candidate_index, entry.searchable_text())
                        })
                        .collect();

                    let search_matches = fuzzy::match_strings(
                        &candidates,
                        &query,
                        false,
                        true,
                        MAX_MATCHES,
                        &Default::default(),
                        executor,
                    )
                    .await;

                    let mut workspace_matches = Vec::new();
                    let mut project_matches = Vec::new();

                    for search_match in search_matches {
                        let (original_index, _) = data_entries[search_match.candidate_id];
                        let entry = entries[original_index].clone();
                        let sidebar_match = SidebarMatch {
                            positions: search_match.positions,
                            entry: entry.clone(),
                        };
                        match entry {
                            SidebarEntry::WorkspaceThread(_) => {
                                workspace_matches.push(sidebar_match)
                            }
                            SidebarEntry::RecentProject(_) => project_matches.push(sidebar_match),
                            SidebarEntry::Separator(_) => {}
                        }
                    }

                    let mut result = Vec::new();
                    if !workspace_matches.is_empty() {
                        result.push(SidebarMatch {
                            entry: SidebarEntry::Separator("Active Workspaces".into()),
                            positions: Vec::new(),
                        });
                        result.extend(workspace_matches);
                    }
                    if !project_matches.is_empty() {
                        result.push(SidebarMatch {
                            entry: SidebarEntry::Separator("Recent Projects".into()),
                            positions: Vec::new(),
                        });
                        result.extend(project_matches);
                    }
                    result
                })
                .await;

            picker
                .update_in(cx, |picker, _window, _cx| {
                    picker.delegate.matches = matches;
                    if picker.delegate.matches.is_empty() {
                        picker.delegate.selected_index = 0;
                    } else {
                        let first_selectable = picker
                            .delegate
                            .matches
                            .iter()
                            .position(|m| !matches!(m.entry, SidebarEntry::Separator(_)))
                            .unwrap_or(0);
                        picker.delegate.selected_index = first_selectable;
                    }
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(selected_match) = self.matches.get(self.selected_index) else {
            return;
        };

        match &selected_match.entry {
            SidebarEntry::Separator(_) => {}
            SidebarEntry::WorkspaceThread(thread_entry) => {
                let target_index = thread_entry.index;
                self.multi_workspace.update(cx, |multi_workspace, cx| {
                    multi_workspace.activate_index(target_index, window, cx);
                });
            }
            SidebarEntry::RecentProject(project_entry) => {
                let paths = project_entry.paths.clone();
                open_recent_project(paths, window, cx);
            }
        }
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        index: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let match_entry = self.matches.get(index)?;
        let SidebarMatch { entry, positions } = match_entry;

        match entry {
            SidebarEntry::Separator(title) => Some(
                v_flex()
                    .when(index > 0, |this| {
                        this.mt_1()
                            .gap_2()
                            .child(Divider::horizontal().color(DividerColor::BorderFaded))
                    })
                    .child(ListSubHeader::new(title.clone()).inset(true))
                    .into_any_element(),
            ),
            SidebarEntry::WorkspaceThread(thread_entry) => {
                let worktree_label = thread_entry.worktree_label.clone();
                let full_path = thread_entry.full_path.clone();
                let thread_info = thread_entry.thread_info.clone();
                let workspace_index = thread_entry.index;
                let multi_workspace = self.multi_workspace.clone();
                let workspace_count = self.multi_workspace.read(cx).workspaces().len();
                let is_hovered = self.hovered_thread_item == Some(workspace_index);

                let remove_btn = IconButton::new(
                    format!("remove-workspace-{}", workspace_index),
                    IconName::Close,
                )
                .icon_size(IconSize::Small)
                .icon_color(Color::Muted)
                .tooltip(Tooltip::text("Remove Workspace"))
                .on_click({
                    let multi_workspace = multi_workspace;
                    move |_, window, cx| {
                        multi_workspace.update(cx, |mw, cx| {
                            mw.remove_workspace(workspace_index, window, cx);
                        });
                    }
                });

                let has_notification = self.notified_workspaces.contains(&workspace_index);
                let thread_subtitle = thread_info.as_ref().map(|info| info.title.clone());
                let status = thread_info
                    .as_ref()
                    .map_or(AgentThreadStatus::default(), |info| info.status);
                let running = matches!(
                    status,
                    AgentThreadStatus::Running | AgentThreadStatus::WaitingForConfirmation
                );

                let thread_item = ThreadItem::new(
                        ("workspace-item", thread_entry.index),
                        thread_subtitle.unwrap_or("New Thread".into()),
                    )
                    .icon(
                        thread_info
                            .as_ref()
                            .map_or(IconName::ZedAgent, |info| info.icon),
                    )
                    .running(running)
                    .generation_done(has_notification)
                    .status(status)
                    .selected(selected)
                    .worktree(worktree_label.clone())
                    .worktree_highlight_positions(positions.clone())
                    .when(workspace_count > 1, |item| item.action_slot(remove_btn))
                    .hovered(is_hovered)
                    .on_hover(cx.listener(move |picker, is_hovered, _window, cx| {
                        let mut changed = false;
                        if *is_hovered {
                            if picker.delegate.hovered_thread_item != Some(workspace_index) {
                                picker.delegate.hovered_thread_item = Some(workspace_index);
                                changed = true;
                            }
                        } else if picker.delegate.hovered_thread_item == Some(workspace_index) {
                            picker.delegate.hovered_thread_item = None;
                            changed = true;
                        }
                        if changed {
                            cx.notify();
                        }
                    }))
                    .when(!full_path.is_empty(), |this| {
                        this.tooltip(move |_, cx| {
                            Tooltip::with_meta(worktree_label.clone(), None, full_path.clone(), cx)
                        })
                    });

                Some(thread_item.into_any_element())
            }
            SidebarEntry::RecentProject(project_entry) => {
                let name = project_entry.name.clone();
                let full_path = project_entry.full_path.clone();
                let item_id: SharedString =
                    format!("recent-project-{:?}", project_entry.workspace_id).into();

                Some(
                    ThreadItem::new(item_id, name.clone())
                        .icon(IconName::Folder)
                        .selected(selected)
                        .highlight_positions(positions.clone())
                        .tooltip(move |_, cx| {
                            Tooltip::with_meta(name.clone(), None, full_path.clone(), cx)
                        })
                        .into_any_element(),
                )
            }
        }
    }

    fn render_editor(
        &self,
        editor: &Arc<dyn ErasedEditor>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        h_flex()
            .h(Tab::container_height(cx))
            .w_full()
            .px_2()
            .gap_2()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                Icon::new(IconName::MagnifyingGlass)
                    .color(Color::Muted)
                    .size(IconSize::Small),
            )
            .child(editor.render(window, cx))
    }
}

pub struct Sidebar {
    multi_workspace: Entity<MultiWorkspace>,
    width: Pixels,
    picker: Entity<Picker<WorkspacePickerDelegate>>,
    _subscription: Subscription,
    _project_subscriptions: Vec<Subscription>,
    _agent_panel_subscriptions: Vec<Subscription>,
    _thread_subscriptions: Vec<Subscription>,
    #[cfg(any(test, feature = "test-support"))]
    test_thread_infos: HashMap<usize, AgentThreadInfo>,
    #[cfg(any(test, feature = "test-support"))]
    test_recent_project_thread_titles: HashMap<SharedString, SharedString>,
    _fetch_recent_projects: Task<()>,
}

impl EventEmitter<SidebarEvent> for Sidebar {}

impl Sidebar {
    pub fn new(
        multi_workspace: Entity<MultiWorkspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = WorkspacePickerDelegate::new(multi_workspace.clone());
        let picker = cx.new(|cx| {
            Picker::list(delegate, window, cx)
                .max_height(None)
                .show_scrollbar(true)
                .modal(false)
        });

        let subscription = cx.observe_in(
            &multi_workspace,
            window,
            |this, _multi_workspace, window, cx| {
                this.update_entries(window, cx);
            },
        );

        let fetch_recent_projects = {
            let picker = picker.downgrade();
            let fs = <dyn Fs>::global(cx);
            cx.spawn_in(window, async move |_this, cx| {
                let projects = get_recent_projects(None, None, fs).await;

                cx.update(|window, cx| {
                    if let Some(picker) = picker.upgrade() {
                        picker.update(cx, |picker, cx| {
                            picker.delegate.set_recent_projects(projects, cx);
                            let query = picker.query(cx);
                            picker.update_matches(query, window, cx);
                        });
                    }
                })
                .log_err();
            })
        };

        let mut this = Self {
            multi_workspace,
            width: DEFAULT_WIDTH,
            picker,
            _subscription: subscription,
            _project_subscriptions: Vec::new(),
            _agent_panel_subscriptions: Vec::new(),
            _thread_subscriptions: Vec::new(),
            #[cfg(any(test, feature = "test-support"))]
            test_thread_infos: HashMap::new(),
            #[cfg(any(test, feature = "test-support"))]
            test_recent_project_thread_titles: HashMap::new(),
            _fetch_recent_projects: fetch_recent_projects,
        };
        this.update_entries(window, cx);
        this
    }

    fn subscribe_to_projects(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Subscription> {
        let workspaces: Vec<_> = self
            .multi_workspace
            .read(cx)
            .workspaces()
            .to_vec();

        let mut subscriptions = Vec::new();
        for workspace in &workspaces {
            let project = workspace.read(cx).project().clone();

            subscriptions.push(cx.subscribe_in(
                &project,
                window,
                |this, _project, event, window, cx| match event {
                    ProjectEvent::WorktreeAdded(_)
                    | ProjectEvent::WorktreeRemoved(_)
                    | ProjectEvent::WorktreeOrderChanged => {
                        this.update_entries(window, cx);
                    }
                    _ => {}
                },
            ));

            let git_store = project.read(cx).git_store().clone();
            subscriptions.push(cx.subscribe_in(
                &git_store,
                window,
                |this, _, event, window, cx| match event {
                    GitStoreEvent::RepositoryAdded
                    | GitStoreEvent::RepositoryRemoved(_)
                    | GitStoreEvent::RepositoryUpdated(
                        _,
                        RepositoryEvent::BranchChanged,
                        _,
                    ) => {
                        this.update_entries(window, cx);
                    }
                    _ => {}
                },
            ));
        }
        subscriptions
    }

    fn build_workspace_thread_entries(
        &self,
        multi_workspace: &MultiWorkspace,
        cx: &App,
    ) -> (Vec<WorkspaceThreadEntry>, usize) {
        #[allow(unused_mut)]
        let mut entries: Vec<WorkspaceThreadEntry> = multi_workspace
            .workspaces()
            .iter()
            .enumerate()
            .map(|(index, workspace)| WorkspaceThreadEntry::new(index, workspace, cx))
            .collect();

        #[cfg(any(test, feature = "test-support"))]
        for (index, info) in &self.test_thread_infos {
            if let Some(entry) = entries.get_mut(*index) {
                entry.thread_info = Some(info.clone());
            }
        }

        (entries, multi_workspace.active_workspace_index())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_test_recent_projects(
        &self,
        projects: Vec<RecentProjectEntry>,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, _cx| {
            picker.delegate.recent_projects = projects;
        });
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_test_thread_info(
        &mut self,
        index: usize,
        title: SharedString,
        status: AgentThreadStatus,
    ) {
        self.test_thread_infos.insert(
            index,
            AgentThreadInfo {
                title,
                status,
                icon: IconName::ZedAgent,
            },
        );
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_test_recent_project_thread_title(
        &mut self,
        full_path: SharedString,
        title: SharedString,
        cx: &mut Context<Self>,
    ) {
        self.test_recent_project_thread_titles
            .insert(full_path.clone(), title.clone());
        self.picker.update(cx, |picker, _cx| {
            picker
                .delegate
                .recent_project_thread_titles
                .insert(full_path, title);
        });
    }

    fn subscribe_to_agent_panels(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Subscription> {
        let workspaces: Vec<_> = self.multi_workspace.read(cx).workspaces().to_vec();

        workspaces
            .iter()
            .map(|workspace| {
                if let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                    cx.subscribe_in(
                        &agent_panel,
                        window,
                        |this, _, _event: &AgentPanelEvent, window, cx| {
                            this.update_entries(window, cx);
                        },
                    )
                } else {
                    // Panel hasn't loaded yet — observe the workspace so we
                    // re-subscribe once the panel appears on its dock.
                    cx.observe_in(workspace, window, |this, _, window, cx| {
                        this.update_entries(window, cx);
                    })
                }
            })
            .collect()
    }

    fn subscribe_to_threads(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Subscription> {
        let workspaces: Vec<_> = self.multi_workspace.read(cx).workspaces().to_vec();

        workspaces
            .iter()
            .filter_map(|workspace| {
                let agent_panel = workspace.read(cx).panel::<AgentPanel>(cx)?;
                let thread = agent_panel.read(cx).active_agent_thread(cx)?;
                Some(cx.observe_in(&thread, window, |this, _, window, cx| {
                    this.update_entries(window, cx);
                }))
            })
            .collect()
    }

    /// Reconciles the sidebar's displayed entries with the current state of all
    /// workspaces and their agent threads.
    fn update_entries(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let multi_workspace = self.multi_workspace.clone();
        cx.defer_in(window, move |this, window, cx| {
            if !this.multi_workspace.read(cx).multi_workspace_enabled(cx) {
                return;
            }

            this._project_subscriptions = this.subscribe_to_projects(window, cx);
            this._agent_panel_subscriptions = this.subscribe_to_agent_panels(window, cx);
            this._thread_subscriptions = this.subscribe_to_threads(window, cx);
            let (entries, active_index) = multi_workspace.read_with(cx, |multi_workspace, cx| {
                this.build_workspace_thread_entries(multi_workspace, cx)
            });

            let had_notifications = !this.picker.read(cx).delegate.notified_workspaces.is_empty();
            this.picker.update(cx, |picker, cx| {
                picker.delegate.set_entries(entries, active_index, cx);
                let query = picker.query(cx);
                picker.update_matches(query, window, cx);
            });

            // Push sidebar sort order to MultiWorkspace for next/previous cycling.
            let display_order: Vec<usize> = this.picker.read(cx).delegate.entries
                .iter()
                .filter_map(|entry| match entry {
                    SidebarEntry::WorkspaceThread(t) => Some(t.index),
                    _ => None,
                })
                .collect();
            multi_workspace.update(cx, |mw, _| {
                mw.set_display_order(display_order);
            });

            let has_notifications = !this.picker.read(cx).delegate.notified_workspaces.is_empty();
            if had_notifications != has_notifications {
                multi_workspace.update(cx, |_, cx| cx.notify());
            }
        });
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

    fn has_notifications(&self, cx: &App) -> bool {
        !self.picker.read(cx).delegate.notified_workspaces.is_empty()
    }
}

impl Focusable for Sidebar {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.read(cx).focus_handle(cx)
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar_height = ui::utils::platform_title_bar_height(window);
        let ui_font = theme::setup_ui_font(window, cx);
        let is_focused = self.focus_handle(cx).is_focused(window);

        let focus_tooltip_label = if is_focused {
            "Focus Workspace"
        } else {
            "Focus Sidebar"
        };

        v_flex()
            .id("workspace-sidebar")
            .key_context("WorkspaceSidebar")
            .font(ui_font)
            .h_full()
            .w(self.width)
            .bg(cx.theme().colors().surface_background)
            .border_r_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .flex_none()
                    .h(titlebar_height)
                    .w_full()
                    .mt_px()
                    .pb_px()
                    .pr_1()
                    .when_else(
                        cfg!(target_os = "macos") && !window.is_fullscreen(),
                        |this| this.pl(px(TRAFFIC_LIGHT_PADDING)),
                        |this| this.pl_2(),
                    )
                    .justify_between()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child({
                        let focus_handle = cx.focus_handle();
                        IconButton::new("close-sidebar", IconName::WorkspaceNavOpen)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::element(move |_, cx| {
                                v_flex()
                                    .gap_1()
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .justify_between()
                                            .child(Label::new("Close Sidebar"))
                                            .child(KeyBinding::for_action_in(
                                                &ToggleWorkspaceSidebar,
                                                &focus_handle,
                                                cx,
                                            )),
                                    )
                                    .child(
                                        h_flex()
                                            .pt_1()
                                            .gap_2()
                                            .border_t_1()
                                            .border_color(cx.theme().colors().border_variant)
                                            .justify_between()
                                            .child(Label::new(focus_tooltip_label))
                                            .child(KeyBinding::for_action_in(
                                                &FocusWorkspaceSidebar,
                                                &focus_handle,
                                                cx,
                                            )),
                                    )
                                    .into_any_element()
                            }))
                            .on_click(cx.listener(|_this, _, _window, cx| {
                                cx.emit(SidebarEvent::Close);
                            }))
                    })
                    .child(
                        IconButton::new("new-workspace", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .tooltip(|_window, cx| {
                                Tooltip::for_action("New Workspace", &NewWorkspaceInWindow, cx)
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.multi_workspace.update(cx, |multi_workspace, cx| {
                                    multi_workspace.create_workspace(window, cx);
                                });
                            })),
                    ),
            )
            .child(self.picker.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use feature_flags::FeatureFlagAppExt as _;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use settings::SettingsStore;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            cx.update_flags(false, vec!["agent-v2".into()]);
        });
    }

    fn set_thread_info_and_refresh(
        sidebar: &Entity<Sidebar>,
        multi_workspace: &Entity<MultiWorkspace>,
        index: usize,
        title: &str,
        status: AgentThreadStatus,
        cx: &mut gpui::VisualTestContext,
    ) {
        sidebar.update_in(cx, |s, _window, _cx| {
            s.set_test_thread_info(index, SharedString::from(title.to_string()), status);
        });
        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();
    }

    fn has_notifications(sidebar: &Entity<Sidebar>, cx: &mut gpui::VisualTestContext) -> bool {
        sidebar.read_with(cx, |s, cx| s.has_notifications(cx))
    }

    #[gpui::test]
    async fn test_notification_on_running_to_completed_transition(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| <dyn Fs>::set_global(fs.clone(), cx));
        let project = project::Project::test(fs, [], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));

        let sidebar = multi_workspace.update_in(cx, |_mw, window, cx| {
            let mw_handle = cx.entity();
            cx.new(|cx| Sidebar::new(mw_handle, window, cx))
        });
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.register_sidebar(sidebar.clone(), window, cx);
        });
        cx.run_until_parked();

        // Create a second workspace and switch to it so workspace 0 is background.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.create_workspace(window, cx);
        });
        cx.run_until_parked();
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(1, window, cx);
        });
        cx.run_until_parked();

        assert!(
            !has_notifications(&sidebar, cx),
            "should have no notifications initially"
        );

        set_thread_info_and_refresh(
            &sidebar,
            &multi_workspace,
            0,
            "Test Thread",
            AgentThreadStatus::Running,
            cx,
        );

        assert!(
            !has_notifications(&sidebar, cx),
            "Running status alone should not create a notification"
        );

        set_thread_info_and_refresh(
            &sidebar,
            &multi_workspace,
            0,
            "Test Thread",
            AgentThreadStatus::Completed,
            cx,
        );

        assert!(
            has_notifications(&sidebar, cx),
            "Running → Completed transition should create a notification"
        );
    }

    #[gpui::test]
    async fn test_no_notification_for_active_workspace(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| <dyn Fs>::set_global(fs.clone(), cx));
        let project = project::Project::test(fs, [], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));

        let sidebar = multi_workspace.update_in(cx, |_mw, window, cx| {
            let mw_handle = cx.entity();
            cx.new(|cx| Sidebar::new(mw_handle, window, cx))
        });
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.register_sidebar(sidebar.clone(), window, cx);
        });
        cx.run_until_parked();

        // Workspace 0 is the active workspace — thread completes while
        // the user is already looking at it.
        set_thread_info_and_refresh(
            &sidebar,
            &multi_workspace,
            0,
            "Test Thread",
            AgentThreadStatus::Running,
            cx,
        );
        set_thread_info_and_refresh(
            &sidebar,
            &multi_workspace,
            0,
            "Test Thread",
            AgentThreadStatus::Completed,
            cx,
        );

        assert!(
            !has_notifications(&sidebar, cx),
            "should not notify for the workspace the user is already looking at"
        );
    }

    #[gpui::test]
    async fn test_notification_cleared_on_workspace_activation(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| <dyn Fs>::set_global(fs.clone(), cx));
        let project = project::Project::test(fs, [], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));

        let sidebar = multi_workspace.update_in(cx, |_mw, window, cx| {
            let mw_handle = cx.entity();
            cx.new(|cx| Sidebar::new(mw_handle, window, cx))
        });
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.register_sidebar(sidebar.clone(), window, cx);
        });
        cx.run_until_parked();

        // Create a second workspace so we can switch away and back.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.create_workspace(window, cx);
        });
        cx.run_until_parked();

        // Switch to workspace 1 so workspace 0 becomes a background workspace.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(1, window, cx);
        });
        cx.run_until_parked();

        // Thread on workspace 0 transitions Running → Completed while
        // the user is looking at workspace 1.
        set_thread_info_and_refresh(
            &sidebar,
            &multi_workspace,
            0,
            "Test Thread",
            AgentThreadStatus::Running,
            cx,
        );
        set_thread_info_and_refresh(
            &sidebar,
            &multi_workspace,
            0,
            "Test Thread",
            AgentThreadStatus::Completed,
            cx,
        );

        assert!(
            has_notifications(&sidebar, cx),
            "background workspace completion should create a notification"
        );

        // Switching back to workspace 0 should clear the notification.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(0, window, cx);
        });
        cx.run_until_parked();

        assert!(
            !has_notifications(&sidebar, cx),
            "notification should be cleared when workspace becomes active"
        );
    }

    #[gpui::test]
    async fn test_worktree_label_shows_repo_name_and_branch(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            serde_json::json!({
                ".git": {},
                "src": { "main.rs": "" }
            }),
        )
        .await;
        fs.set_branch_name(Path::new("/project/.git"), Some("feature-branch"));
        cx.update(|cx| <dyn Fs>::set_global(fs.clone(), cx));

        let project = project::Project::test(fs, [Path::new("/project")], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        cx.run_until_parked();

        let label = multi_workspace.read_with(cx, |mw, cx| {
            let workspace = mw.workspace();
            WorkspaceThreadEntry::new(0, workspace, cx).worktree_label
        });
        assert_eq!(label.as_ref(), "project / feature-branch");
    }

    #[gpui::test]
    async fn test_worktree_label_for_git_worktree(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/myrepo",
            serde_json::json!({
                ".git": {
                    "worktrees": {
                        "feature-x": {
                            "commondir": "../..\n",
                            "HEAD": "",
                            "config": ""
                        }
                    }
                },
                "sub": {
                    "feature-x": {
                        ".git": "gitdir: ../../.git/worktrees/feature-x\n",
                        "src": { "main.rs": "" }
                    }
                }
            }),
        )
        .await;
        fs.set_branch_name(
            Path::new("/myrepo/.git/worktrees/feature-x"),
            Some("feature-x"),
        );
        cx.update(|cx| <dyn Fs>::set_global(fs.clone(), cx));

        let project = project::Project::test(
            fs,
            [Path::new("/myrepo/sub/feature-x")],
            cx,
        )
        .await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        cx.run_until_parked();

        let label = multi_workspace.read_with(cx, |mw, cx| {
            let workspace = mw.workspace();
            WorkspaceThreadEntry::new(0, workspace, cx).worktree_label
        });
        assert_eq!(label.as_ref(), "myrepo / feature-x");
    }

    #[gpui::test]
    async fn test_worktree_label_without_git(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/plain-dir",
            serde_json::json!({
                "src": { "main.rs": "" }
            }),
        )
        .await;
        cx.update(|cx| <dyn Fs>::set_global(fs.clone(), cx));

        let project = project::Project::test(fs, [Path::new("/plain-dir")], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        cx.run_until_parked();

        let label = multi_workspace.read_with(cx, |mw, cx| {
            let workspace = mw.workspace();
            WorkspaceThreadEntry::new(0, workspace, cx).worktree_label
        });
        assert_eq!(label.as_ref(), "plain-dir");
    }

    #[gpui::test]
    async fn test_display_order_follows_sidebar_sort(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        // Four repos: alpha, bravo (two worktrees sharing common_dir), charlie.
        // Opened in order: bravo/main(0), alpha(1), bravo/feature(2), charlie(3)
        // Sidebar sort by common_dir: alpha(1), bravo/main(0), bravo/feature(2), charlie(3)
        fs.insert_tree(
            "/bravo-main",
            serde_json::json!({ ".git": {}, "src": {} }),
        )
        .await;
        fs.set_branch_name(Path::new("/bravo-main/.git"), Some("main"));

        fs.insert_tree(
            "/alpha",
            serde_json::json!({ ".git": {}, "src": {} }),
        )
        .await;
        fs.set_branch_name(Path::new("/alpha/.git"), Some("main"));

        fs.insert_tree(
            "/bravo-feature",
            serde_json::json!({
                ".git": "gitdir: /bravo-main/.git/worktrees/feature\n",
                "src": {}
            }),
        )
        .await;
        fs.insert_tree(
            "/bravo-main/.git/worktrees/feature",
            serde_json::json!({
                "commondir": "../..\n",
                "HEAD": "",
                "config": ""
            }),
        )
        .await;
        fs.set_branch_name(
            Path::new("/bravo-main/.git/worktrees/feature"),
            Some("feature"),
        );

        fs.insert_tree(
            "/charlie",
            serde_json::json!({ ".git": {}, "src": {} }),
        )
        .await;
        fs.set_branch_name(Path::new("/charlie/.git"), Some("main"));

        cx.update(|cx| <dyn Fs>::set_global(fs.clone(), cx));

        // Workspace 0: bravo/main
        let project0 =
            project::Project::test(fs.clone(), [Path::new("/bravo-main")], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project0, window, cx));

        let sidebar = multi_workspace.update_in(cx, |_mw, window, cx| {
            let mw_handle = cx.entity();
            cx.new(|cx| Sidebar::new(mw_handle, window, cx))
        });
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.register_sidebar(sidebar.clone(), window, cx);
        });

        // Workspace 1: alpha
        let project1 = project::Project::test(fs.clone(), [Path::new("/alpha")], cx).await;
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(project1, window, cx);
        });

        // Workspace 2: bravo/feature
        let project2 =
            project::Project::test(fs.clone(), [Path::new("/bravo-feature")], cx).await;
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(project2, window, cx);
        });

        // Workspace 3: charlie
        let project3 =
            project::Project::test(fs.clone(), [Path::new("/charlie")], cx).await;
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(project3, window, cx);
        });

        cx.run_until_parked();

        // Trigger sidebar update
        sidebar.update_in(cx, |s, window, cx| {
            s.update_entries(window, cx);
        });
        cx.run_until_parked();

        // Verify display_order matches sidebar sort (by common_dir, prime first)
        let display_order = multi_workspace.read_with(cx, |mw, _cx| {
            mw.display_order().to_vec()
        });

        // alpha(1) sorts before bravo(0,2) before charlie(3)
        // Within bravo group: main(0) is prime, feature(2) is not
        assert_eq!(
            display_order,
            vec![1, 0, 2, 3],
            "display_order should be sorted: alpha, bravo/main, bravo/feature, charlie"
        );

        // Verify cycling from workspace 0 (bravo/main)
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(0, window, cx);
        });
        cx.run_until_parked();

        // Next from bravo/main(0) should go to bravo/feature(2)
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_next_workspace(window, cx);
        });
        let active = multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index());
        assert_eq!(active, 2, "next from bravo/main should be bravo/feature");

        // Next from bravo/feature(2) should go to charlie(3)
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_next_workspace(window, cx);
        });
        let active = multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index());
        assert_eq!(active, 3, "next from bravo/feature should be charlie");

        // Previous from charlie(3) should go back to bravo/feature(2)
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_previous_workspace(window, cx);
        });
        let active = multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index());
        assert_eq!(active, 2, "previous from charlie should be bravo/feature");

        // Wrap around: previous from alpha(1) should go to charlie(3)
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(1, window, cx);
        });
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_previous_workspace(window, cx);
        });
        let active = multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index());
        assert_eq!(active, 3, "previous from alpha should wrap to charlie");
    }
}
