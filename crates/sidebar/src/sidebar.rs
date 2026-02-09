use acp_thread::ThreadStatus;
use agent_ui::{AgentPanel, AgentPanelEvent};
use db::kvp::KEY_VALUE_STORE;
use fs::Fs;
use fuzzy::StringMatchCandidate;
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, Pixels, Render, SharedString,
    Subscription, Task, Window, px,
};
use picker::{Picker, PickerDelegate};
use project::Event as ProjectEvent;
use recent_projects::{RecentProjectEntry, get_recent_projects};

use std::collections::{HashMap, HashSet};

use std::path::{Path, PathBuf};
use std::sync::Arc;
use theme::ActiveTheme;
use ui::utils::TRAFFIC_LIGHT_PADDING;
use ui::{CommonAnimationExt, Divider, HighlightedLabel, ListItem, Tab, Tooltip, prelude::*};
use ui_input::ErasedEditor;
use util::ResultExt as _;
use workspace::{
    MultiWorkspace, NewWorkspaceInWindow, Sidebar as WorkspaceSidebar, SidebarEvent,
    ToggleWorkspaceSidebar, Workspace,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AgentThreadStatus {
    Running,
    Completed,
}

#[derive(Clone, Debug)]
struct AgentThreadInfo {
    title: SharedString,
    status: AgentThreadStatus,
}

const LAST_THREAD_TITLES_KEY: &str = "sidebar-last-thread-titles";

const DEFAULT_WIDTH: Pixels = px(320.0);
const MIN_WIDTH: Pixels = px(200.0);
const MAX_WIDTH: Pixels = px(800.0);
const MAX_MATCHES: usize = 100;

#[derive(Clone)]
struct WorkspaceThreadEntry {
    index: usize,
    worktree_label: SharedString,
    full_path: SharedString,
    thread_info: Option<AgentThreadInfo>,
}

impl WorkspaceThreadEntry {
    fn new(
        index: usize,
        workspace: &Entity<Workspace>,
        persisted_titles: &HashMap<String, String>,
        cx: &App,
    ) -> Self {
        let workspace_ref = workspace.read(cx);

        let worktrees: Vec<_> = workspace_ref
            .worktrees(cx)
            .map(|worktree| worktree.read(cx).abs_path())
            .collect();

        let worktree_names: Vec<String> = worktrees
            .iter()
            .filter_map(|path| {
                path.file_name()
                    .map(|name| name.to_string_lossy().to_string())
            })
            .collect();

        let worktree_label: SharedString = if worktree_names.is_empty() {
            format!("Workspace {}", index + 1).into()
        } else {
            worktree_names.join(", ").into()
        };

        let full_path: SharedString = worktrees
            .iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("\n")
            .into();

        let thread_info = Self::thread_info(workspace, cx).or_else(|| {
            if worktrees.is_empty() {
                return None;
            }
            let path_key = sorted_paths_key(&worktrees);
            let title = persisted_titles.get(&path_key)?;
            Some(AgentThreadInfo {
                title: SharedString::from(title.clone()),
                status: AgentThreadStatus::Completed,
            })
        });

        Self {
            index,
            worktree_label,
            full_path,
            thread_info,
        }
    }

    fn thread_info(workspace: &Entity<Workspace>, cx: &App) -> Option<AgentThreadInfo> {
        let agent_panel = workspace.read(cx).panel::<AgentPanel>(cx)?;
        let thread = agent_panel.read(cx).active_agent_thread(cx)?;
        let thread_ref = thread.read(cx);
        let title = thread_ref.title();
        let status = match thread_ref.status() {
            ThreadStatus::Generating => AgentThreadStatus::Running,
            ThreadStatus::Idle => AgentThreadStatus::Completed,
        };
        Some(AgentThreadInfo { title, status })
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
            notified_workspaces: HashSet::new(),
        }
    }

    fn set_entries(
        &mut self,
        workspace_threads: Vec<WorkspaceThreadEntry>,
        active_workspace_index: usize,
        cx: &App,
    ) {
        let old_statuses: HashMap<usize, AgentThreadStatus> = self
            .entries
            .iter()
            .filter_map(|entry| match entry {
                SidebarEntry::WorkspaceThread(thread) => thread
                    .thread_info
                    .as_ref()
                    .map(|info| (thread.index, info.status.clone())),
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
        if let Some(map) = read_thread_title_map() {
            for entry in &recent_projects {
                let path_key = sorted_paths_key(&entry.paths);
                if let Some(title) = map.get(&path_key) {
                    self.recent_project_thread_titles
                        .insert(entry.full_path.clone(), title.clone().into());
                }
            }
        }

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
            for thread in workspace_threads {
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
            self.entries
                .push(SidebarEntry::Separator("Recent Projects".into()));
            for project in recent {
                self.entries.push(SidebarEntry::RecentProject(project));
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
        self.query = query.clone();
        let entries = self.entries.clone();

        if query.is_empty() {
            self.matches = entries
                .into_iter()
                .map(|entry| SidebarMatch {
                    entry,
                    positions: Vec::new(),
                })
                .collect();

            let separator_offset = if self.workspace_thread_count > 0 {
                1
            } else {
                0
            };
            self.selected_index = (self.active_workspace_index + separator_offset)
                .min(self.matches.len().saturating_sub(1));
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
                Self::open_recent_project(paths, window, cx);
            }
        }
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        index: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let match_entry = self.matches.get(index)?;
        let SidebarMatch { entry, positions } = match_entry;

        fn render_title(text: SharedString, positions: &[usize]) -> AnyElement {
            if positions.is_empty() {
                div()
                    .p_0p5()
                    .child(Label::new(text).truncate())
                    .into_any_element()
            } else {
                div()
                    .p_0p5()
                    .child(HighlightedLabel::new(text, positions.to_vec()).truncate())
                    .into_any_element()
            }
        }

        fn render_thread_status_icon(
            workspace_index: usize,
            status: &AgentThreadStatus,
            has_notification: bool,
        ) -> AnyElement {
            match status {
                AgentThreadStatus::Running => Icon::new(IconName::LoadCircle)
                    .size(IconSize::XSmall)
                    .color(Color::Muted)
                    .with_keyed_rotate_animation(
                        SharedString::from(format!("workspace-{}-spinner", workspace_index)),
                        3,
                    )
                    .into_any_element(),
                AgentThreadStatus::Completed => {
                    let color = if has_notification {
                        Color::Accent
                    } else {
                        Color::Muted
                    };
                    Icon::new(IconName::Check)
                        .size(IconSize::XSmall)
                        .color(color)
                        .into_any_element()
                }
            }
        }

        fn render_project_row(
            title: AnyElement,
            thread_subtitle: Option<SharedString>,
            status_icon: Option<AnyElement>,
            cx: &App,
        ) -> Div {
            h_flex()
                .items_start()
                .gap(DynamicSpacing::Base06.rems(cx))
                .child(
                    div().pt(px(4.0)).child(
                        Icon::new(IconName::Folder)
                            .color(Color::Muted)
                            .size(IconSize::XSmall),
                    ),
                )
                .child(v_flex().overflow_hidden().child(title).when_some(
                    thread_subtitle,
                    |this, subtitle| {
                        this.child(
                            h_flex()
                                .gap_1()
                                .items_center()
                                .px_0p5()
                                .when_some(status_icon, |this, icon| this.child(icon))
                                .child(
                                    Label::new(subtitle)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .truncate(),
                                ),
                        )
                    },
                ))
        }

        match entry {
            SidebarEntry::Separator(title) => Some(
                div()
                    .px_0p5()
                    .when(index > 0, |this| this.mt_1().child(Divider::horizontal()))
                    .child(
                        ListItem::new("section_header").selectable(false).child(
                            Label::new(title.clone())
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                                .when(index > 0, |this| this.mt_1p5())
                                .mb_1(),
                        ),
                    )
                    .into_any_element(),
            ),
            SidebarEntry::WorkspaceThread(thread_entry) => {
                let worktree_label = thread_entry.worktree_label.clone();
                let full_path = thread_entry.full_path.clone();
                let title = render_title(worktree_label.clone(), positions);
                let thread_info = thread_entry.thread_info.clone();
                let workspace_index = thread_entry.index;
                let multi_workspace = self.multi_workspace.clone();
                let workspace_count = self.multi_workspace.read(_cx).workspaces().len();

                let close_button = if workspace_count > 1 {
                    Some(
                        IconButton::new(
                            SharedString::from(format!("close-workspace-{}", workspace_index)),
                            IconName::Close,
                        )
                        .icon_size(IconSize::XSmall)
                        .icon_color(Color::Muted)
                        .tooltip(Tooltip::text("Close Workspace"))
                        .on_click({
                            let multi_workspace = multi_workspace;
                            move |_, window, cx| {
                                multi_workspace.update(cx, |mw, cx| {
                                    mw.remove_workspace(workspace_index, window, cx);
                                });
                            }
                        }),
                    )
                } else {
                    None
                };

                let has_notification = self.notified_workspaces.contains(&workspace_index);
                let (thread_subtitle, status_icon) = match thread_info {
                    Some(info) => (
                        Some(info.title),
                        Some(render_thread_status_icon(
                            workspace_index,
                            &info.status,
                            has_notification,
                        )),
                    ),
                    None => (None, None),
                };

                Some(
                    ListItem::new(("workspace-item", thread_entry.index))
                        .toggle_state(selected)
                        .when_some(close_button, |item, button| item.end_hover_slot(button))
                        .child(render_project_row(title, thread_subtitle, status_icon, _cx))
                        .when(!full_path.is_empty(), |item| {
                            item.tooltip(move |_, cx| {
                                Tooltip::with_meta(
                                    worktree_label.clone(),
                                    None,
                                    full_path.clone(),
                                    cx,
                                )
                            })
                        })
                        .into_any_element(),
                )
            }
            SidebarEntry::RecentProject(project_entry) => {
                let name = project_entry.name.clone();
                let full_path = project_entry.full_path.clone();
                let title = render_title(name.clone(), positions);
                let item_id: SharedString =
                    format!("recent-project-{:?}", project_entry.workspace_id).into();
                let thread_title = self
                    .recent_project_thread_titles
                    .get(&project_entry.full_path)
                    .cloned();

                Some(
                    ListItem::new(item_id)
                        .toggle_state(selected)
                        .child(render_project_row(title, thread_title, None, _cx))
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
            |this, multi_workspace, window, cx| {
                this.queue_refresh(multi_workspace, window, cx);
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
        this.queue_refresh(this.multi_workspace.clone(), window, cx);
        this
    }

    fn subscribe_to_projects(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Subscription> {
        let projects: Vec<_> = self
            .multi_workspace
            .read(cx)
            .workspaces()
            .iter()
            .map(|w| w.read(cx).project().clone())
            .collect();

        projects
            .iter()
            .map(|project| {
                cx.subscribe_in(
                    project,
                    window,
                    |this, _project, event, window, cx| match event {
                        ProjectEvent::WorktreeAdded(_)
                        | ProjectEvent::WorktreeRemoved(_)
                        | ProjectEvent::WorktreeOrderChanged => {
                            this.queue_refresh(this.multi_workspace.clone(), window, cx);
                        }
                        _ => {}
                    },
                )
            })
            .collect()
    }

    fn build_workspace_thread_entries(
        &self,
        multi_workspace: &MultiWorkspace,
        cx: &App,
    ) -> (Vec<WorkspaceThreadEntry>, usize) {
        let persisted_titles = read_thread_title_map().unwrap_or_default();

        #[allow(unused_mut)]
        let mut entries: Vec<WorkspaceThreadEntry> = multi_workspace
            .workspaces()
            .iter()
            .enumerate()
            .map(|(index, workspace)| {
                WorkspaceThreadEntry::new(index, workspace, &persisted_titles, cx)
            })
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
        self.test_thread_infos
            .insert(index, AgentThreadInfo { title, status });
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
                            this.queue_refresh(this.multi_workspace.clone(), window, cx);
                        },
                    )
                } else {
                    // Panel hasn't loaded yet — observe the workspace so we
                    // re-subscribe once the panel appears on its dock.
                    cx.observe_in(workspace, window, |this, _, window, cx| {
                        this.queue_refresh(this.multi_workspace.clone(), window, cx);
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
                    this.queue_refresh(this.multi_workspace.clone(), window, cx);
                }))
            })
            .collect()
    }

    fn persist_thread_titles(
        &self,
        entries: &[WorkspaceThreadEntry],
        multi_workspace: &Entity<MultiWorkspace>,
        cx: &mut Context<Self>,
    ) {
        let mut map = read_thread_title_map().unwrap_or_default();
        let workspaces = multi_workspace.read(cx).workspaces().to_vec();
        let mut changed = false;

        for (workspace, entry) in workspaces.iter().zip(entries.iter()) {
            if let Some(ref info) = entry.thread_info {
                let paths: Vec<_> = workspace
                    .read(cx)
                    .worktrees(cx)
                    .map(|wt| wt.read(cx).abs_path())
                    .collect();
                if paths.is_empty() {
                    continue;
                }
                let path_key = sorted_paths_key(&paths);
                let title = info.title.to_string();
                if map.get(&path_key) != Some(&title) {
                    map.insert(path_key, title);
                    changed = true;
                }
            }
        }

        if changed {
            if let Some(json) = serde_json::to_string(&map).log_err() {
                cx.background_spawn(async move {
                    KEY_VALUE_STORE
                        .write_kvp(LAST_THREAD_TITLES_KEY.into(), json)
                        .await
                        .log_err();
                })
                .detach();
            }
        }
    }

    fn queue_refresh(
        &mut self,
        multi_workspace: Entity<MultiWorkspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.defer_in(window, move |this, window, cx| {
            this._project_subscriptions = this.subscribe_to_projects(window, cx);
            this._agent_panel_subscriptions = this.subscribe_to_agent_panels(window, cx);
            this._thread_subscriptions = this.subscribe_to_threads(window, cx);
            let (entries, active_index) = multi_workspace.read_with(cx, |multi_workspace, cx| {
                this.build_workspace_thread_entries(multi_workspace, cx)
            });

            this.persist_thread_titles(&entries, &multi_workspace, cx);

            let had_notifications = !this.picker.read(cx).delegate.notified_workspaces.is_empty();
            this.picker.update(cx, |picker, cx| {
                picker.delegate.set_entries(entries, active_index, cx);
                let query = picker.query(cx);
                picker.update_matches(query, window, cx);
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

fn sorted_paths_key<P: AsRef<Path>>(paths: &[P]) -> String {
    let mut sorted: Vec<String> = paths
        .iter()
        .map(|p| p.as_ref().to_string_lossy().to_string())
        .collect();
    sorted.sort();
    sorted.join("\n")
}

fn read_thread_title_map() -> Option<HashMap<String, String>> {
    let json = KEY_VALUE_STORE
        .read_kvp(LAST_THREAD_TITLES_KEY)
        .log_err()
        .flatten()?;
    serde_json::from_str(&json).log_err()
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar_height = ui::utils::platform_title_bar_height(window);
        let ui_font = theme::setup_ui_font(window, cx);

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
                    .pr_2()
                    .when(cfg!(target_os = "macos"), |this| {
                        this.pl(px(TRAFFIC_LIGHT_PADDING))
                    })
                    .justify_between()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        IconButton::new("close-sidebar", IconName::WorkspaceNavOpen)
                            .icon_size(IconSize::Small)
                            .tooltip(|_window, cx| {
                                Tooltip::for_action("Close Sidebar", &ToggleWorkspaceSidebar, cx)
                            })
                            .on_click(cx.listener(|_this, _, _window, cx| {
                                cx.emit(SidebarEvent::Close);
                            })),
                    )
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
            s.set_test_thread_info(index, SharedString::from(title.to_string()), status.clone());
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
}
