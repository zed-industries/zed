use acp_thread::ThreadStatus;
use agent::ThreadStore;
use agent_client_protocol as acp;
use agent_ui::{AgentPanel, AgentPanelEvent, NewThread};
use chrono::Utc;
use editor::{Editor, EditorElement, EditorStyle};
use feature_flags::{AgentV2FeatureFlag, FeatureFlagViewExt as _};
use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, FocusHandle, Focusable, FontStyle, ListState,
    Pixels, Render, SharedString, TextStyle, WeakEntity, Window, actions, list, prelude::*, px,
    relative, rems,
};
use menu::{Cancel, Confirm, SelectFirst, SelectLast, SelectNext, SelectPrevious};
use project::Event as ProjectEvent;
use recent_projects::RecentProjects;
use settings::Settings;
use std::collections::{HashMap, HashSet};
use std::mem;
use theme::{ActiveTheme, ThemeSettings};
use ui::utils::TRAFFIC_LIGHT_PADDING;
use ui::{
    AgentThreadStatus, ButtonStyle, HighlightedLabel, IconButtonShape, KeyBinding, ListItem,
    PopoverMenu, PopoverMenuHandle, Tab, ThreadItem, TintColor, Tooltip, WithScrollbar, prelude::*,
};
use util::path_list::PathList;
use workspace::{
    FocusWorkspaceSidebar, MultiWorkspace, MultiWorkspaceEvent, Sidebar as WorkspaceSidebar,
    SidebarEvent, ToggleWorkspaceSidebar, Workspace,
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
    ]
);

const DEFAULT_WIDTH: Pixels = px(320.0);
const MIN_WIDTH: Pixels = px(200.0);
const MAX_WIDTH: Pixels = px(800.0);
const DEFAULT_THREADS_SHOWN: usize = 5;

#[derive(Clone, Debug)]
struct ActiveThreadInfo {
    session_id: acp::SessionId,
    title: SharedString,
    status: AgentThreadStatus,
    icon: IconName,
    icon_from_external_svg: Option<SharedString>,
    is_background: bool,
}

impl From<&ActiveThreadInfo> for acp_thread::AgentSessionInfo {
    fn from(info: &ActiveThreadInfo) -> Self {
        Self {
            session_id: info.session_id.clone(),
            cwd: None,
            title: Some(info.title.clone()),
            updated_at: Some(Utc::now()),
            created_at: Some(Utc::now()),
            meta: None,
        }
    }
}

#[derive(Clone)]
struct ThreadEntry {
    session_info: acp_thread::AgentSessionInfo,
    icon: IconName,
    icon_from_external_svg: Option<SharedString>,
    status: AgentThreadStatus,
    workspace: Entity<Workspace>,
    is_live: bool,
    is_background: bool,
    highlight_positions: Vec<usize>,
}

#[derive(Clone)]
enum ListEntry {
    ProjectHeader {
        path_list: PathList,
        label: SharedString,
        workspace: Entity<Workspace>,
        highlight_positions: Vec<usize>,
        has_threads: bool,
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

fn workspace_path_list_and_label(
    workspace: &Entity<Workspace>,
    cx: &App,
) -> (PathList, SharedString) {
    let workspace_ref = workspace.read(cx);
    let mut paths = Vec::new();
    let mut names = Vec::new();

    for worktree in workspace_ref.worktrees(cx) {
        let worktree_ref = worktree.read(cx);
        if !worktree_ref.is_visible() {
            continue;
        }
        let abs_path = worktree_ref.abs_path();
        paths.push(abs_path.to_path_buf());
        if let Some(name) = abs_path.file_name() {
            names.push(name.to_string_lossy().to_string());
        }
    }

    let label: SharedString = if names.is_empty() {
        // TODO: Can we do something better in this case?
        "Empty Workspace".into()
    } else {
        names.join(", ").into()
    };

    (PathList::new(&paths), label)
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
    active_entry_index: Option<usize>,
    collapsed_groups: HashSet<PathList>,
    expanded_groups: HashMap<PathList, usize>,
    recent_projects_popover_handle: PopoverMenuHandle<RecentProjects>,
}

impl EventEmitter<SidebarEvent> for Sidebar {}

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
                    this.focused_thread = None;
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
            }
        })
        .detach();

        let thread_store = ThreadStore::global(cx);
        cx.observe_in(&thread_store, window, |this, _, _window, cx| {
            this.update_entries(cx);
        })
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
            active_entry_index: None,
            collapsed_groups: HashSet::new(),
            expanded_groups: HashMap::new(),
            recent_projects_popover_handle: PopoverMenuHandle::default(),
        }
    }

    fn subscribe_to_workspace(
        &self,
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
        }
    }

    fn subscribe_to_agent_panel(
        &self,
        agent_panel: &Entity<AgentPanel>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.subscribe_in(
            agent_panel,
            window,
            |this, agent_panel, event: &AgentPanelEvent, _window, cx| match event {
                AgentPanelEvent::ActiveViewChanged => {
                    match agent_panel.read(cx).active_connection_view() {
                        Some(thread) => {
                            if let Some(session_id) = thread.read(cx).parent_id(cx) {
                                this.focused_thread = Some(session_id);
                            }
                        }
                        None => {
                            this.focused_thread = None;
                        }
                    }
                    this.update_entries(cx);
                }
                AgentPanelEvent::ThreadFocused => {
                    let new_focused = agent_panel
                        .read(cx)
                        .active_connection_view()
                        .and_then(|thread| thread.read(cx).parent_id(cx));
                    if new_focused.is_some() && new_focused != this.focused_thread {
                        this.focused_thread = new_focused;
                        this.update_entries(cx);
                    }
                }
                AgentPanelEvent::BackgroundThreadChanged => {
                    this.update_entries(cx);
                }
            },
        )
        .detach();
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

                ActiveThreadInfo {
                    session_id,
                    title,
                    status,
                    icon,
                    icon_from_external_svg,
                    is_background,
                }
            })
            .collect()
    }

    fn rebuild_contents(&mut self, cx: &App) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };
        let mw = multi_workspace.read(cx);
        let workspaces = mw.workspaces().to_vec();
        let active_workspace = mw.workspaces().get(mw.active_workspace_index()).cloned();

        let thread_store = ThreadStore::try_global(cx);
        let query = self.filter_editor.read(cx).text(cx);

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
        // Track all session IDs we add to entries so we can prune stale
        // notifications without a separate pass at the end.
        let mut current_session_ids: HashSet<acp::SessionId> = HashSet::new();
        // Compute active_entry_index inline during the build pass.
        let mut active_entry_index: Option<usize> = None;

        for workspace in workspaces.iter() {
            let (path_list, label) = workspace_path_list_and_label(workspace, cx);

            let is_collapsed = self.collapsed_groups.contains(&path_list);
            let should_load_threads = !is_collapsed || !query.is_empty();

            let mut threads: Vec<ThreadEntry> = Vec::new();

            if should_load_threads {
                if let Some(ref thread_store) = thread_store {
                    for meta in thread_store.read(cx).threads_for_paths(&path_list) {
                        threads.push(ThreadEntry {
                            session_info: meta.into(),
                            icon: IconName::ZedAgent,
                            icon_from_external_svg: None,
                            status: AgentThreadStatus::default(),
                            workspace: workspace.clone(),
                            is_live: false,
                            is_background: false,
                            highlight_positions: Vec::new(),
                        });
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
                let has_threads = !threads.is_empty();

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
                    if workspace_matched || !thread.highlight_positions.is_empty() {
                        matched_threads.push(thread);
                    }
                }

                if matched_threads.is_empty() && !workspace_matched {
                    continue;
                }

                if active_entry_index.is_none()
                    && self.focused_thread.is_none()
                    && active_workspace
                        .as_ref()
                        .is_some_and(|active| active == workspace)
                {
                    active_entry_index = Some(entries.len());
                }

                entries.push(ListEntry::ProjectHeader {
                    path_list: path_list.clone(),
                    label,
                    workspace: workspace.clone(),
                    highlight_positions: workspace_highlight_positions,
                    has_threads,
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
                let has_threads = !threads.is_empty();

                // Check if this header is the active entry before pushing it.
                if active_entry_index.is_none()
                    && self.focused_thread.is_none()
                    && active_workspace
                        .as_ref()
                        .is_some_and(|active| active == workspace)
                {
                    active_entry_index = Some(entries.len());
                }

                entries.push(ListEntry::ProjectHeader {
                    path_list: path_list.clone(),
                    label,
                    workspace: workspace.clone(),
                    highlight_positions: Vec::new(),
                    has_threads,
                });

                if is_collapsed {
                    continue;
                }

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

                if total == 0 {
                    entries.push(ListEntry::NewThread {
                        path_list: path_list.clone(),
                        workspace: workspace.clone(),
                    });
                }
            }
        }

        // Prune stale notifications using the session IDs we collected during
        // the build pass (no extra scan needed).
        notified_threads.retain(|id| current_session_ids.contains(id));

        self.active_entry_index = active_entry_index;
        self.contents = SidebarContents {
            entries,
            notified_threads,
        };
    }

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

    fn render_list_entry(
        &mut self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(entry) = self.contents.entries.get(ix) else {
            return div().into_any_element();
        };
        let is_focused = self.focus_handle.is_focused(window)
            || self.filter_editor.focus_handle(cx).is_focused(window);
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
                has_threads,
            } => self.render_project_header(
                ix,
                path_list,
                label,
                workspace,
                highlight_positions,
                *has_threads,
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
                .border_color(cx.theme().colors().border_variant)
                .child(rendered)
                .into_any_element()
        } else {
            rendered
        }
    }

    fn render_project_header(
        &self,
        ix: usize,
        path_list: &PathList,
        label: &SharedString,
        workspace: &Entity<Workspace>,
        highlight_positions: &[usize],
        has_threads: bool,
        is_selected: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let id = SharedString::from(format!("project-header-{}", ix));
        let group_name = SharedString::from(format!("header-group-{}", ix));
        let ib_id = SharedString::from(format!("project-header-new-thread-{}", ix));

        let is_collapsed = self.collapsed_groups.contains(path_list);
        let disclosure_icon = if is_collapsed {
            IconName::ChevronRight
        } else {
            IconName::ChevronDown
        };
        let workspace_for_new_thread = workspace.clone();
        let workspace_for_remove = workspace.clone();
        // let workspace_for_activate = workspace.clone();

        let path_list_for_toggle = path_list.clone();
        let path_list_for_collapse = path_list.clone();
        let view_more_expanded = self.expanded_groups.contains_key(path_list);

        let multi_workspace = self.multi_workspace.upgrade();
        let workspace_count = multi_workspace
            .as_ref()
            .map_or(0, |mw| mw.read(cx).workspaces().len());
        let is_active_workspace = self.focused_thread.is_none()
            && multi_workspace
                .as_ref()
                .is_some_and(|mw| mw.read(cx).workspace() == workspace);

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
            .toggle_state(is_active_workspace)
            .focused(is_selected)
            .child(
                h_flex()
                    .relative()
                    .min_w_0()
                    .w_full()
                    .p_1()
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
                    .when(workspace_count > 1, |this| {
                        this.child(
                            IconButton::new(
                                SharedString::from(format!("project-header-remove-{}", ix)),
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
                                SharedString::from(format!("project-header-collapse-{}", ix)),
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
                    .when(has_threads, |this| {
                        this.child(
                            IconButton::new(ib_id, IconName::NewThread)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted)
                                .tooltip(Tooltip::text("New Thread"))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.selection = None;
                                    this.create_new_thread(&workspace_for_new_thread, window, cx);
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

    fn activate_workspace(
        &mut self,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            return;
        };

        self.focused_thread = None;

        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.activate(workspace.clone(), cx);
        });

        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.focus_active_workspace(window, cx);
        });
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

    fn focus_in(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn cancel(&mut self, _: &Cancel, window: &mut Window, cx: &mut Context<Self>) {
        if self.reset_filter_editor_text(window, cx) {
            self.update_entries(cx);
        } else {
            self.focus_handle.focus(window, cx);
        }
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
        self.filter_editor.read(cx).buffer().read(cx).is_empty()
    }

    fn editor_move_down(&mut self, _: &MoveDown, window: &mut Window, cx: &mut Context<Self>) {
        self.select_next(&SelectNext, window, cx);
    }

    fn editor_move_up(&mut self, _: &MoveUp, window: &mut Window, cx: &mut Context<Self>) {
        self.select_previous(&SelectPrevious, window, cx);
    }

    fn select_next(&mut self, _: &SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        let next = match self.selection {
            Some(ix) if ix + 1 < self.contents.entries.len() => ix + 1,
            None if !self.contents.entries.is_empty() => 0,
            _ => return,
        };
        self.selection = Some(next);
        self.list_state.scroll_to_reveal_item(next);
        cx.notify();
    }

    fn select_previous(
        &mut self,
        _: &SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let prev = match self.selection {
            Some(ix) if ix > 0 => ix - 1,
            None if !self.contents.entries.is_empty() => self.contents.entries.len() - 1,
            _ => return,
        };
        self.selection = Some(prev);
        self.list_state.scroll_to_reveal_item(prev);
        cx.notify();
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
            ListEntry::ProjectHeader { workspace, .. } => {
                let workspace = workspace.clone();
                self.activate_workspace(&workspace, window, cx);
            }
            ListEntry::Thread(thread) => {
                let session_info = thread.session_info.clone();
                let workspace = thread.workspace.clone();
                self.activate_thread(session_info, &workspace, window, cx);
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

    fn activate_thread(
        &mut self,
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
                    session_info.session_id,
                    session_info.cwd,
                    session_info.title,
                    window,
                    cx,
                );
            });
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

    fn render_thread(
        &self,
        ix: usize,
        thread: &ThreadEntry,
        is_selected: bool,
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
        let workspace = thread.workspace.clone();

        let id = SharedString::from(format!("thread-entry-{}", ix));
        ThreadItem::new(id, title)
            .icon(thread.icon)
            .when_some(thread.icon_from_external_svg.clone(), |this, svg| {
                this.custom_icon_from_external_svg(svg)
            })
            .highlight_positions(thread.highlight_positions.to_vec())
            .status(thread.status)
            .notified(has_notification)
            .selected(self.focused_thread.as_ref() == Some(&session_info.session_id))
            .focused(is_selected)
            .on_click(cx.listener(move |this, _, window, cx| {
                this.selection = None;
                this.activate_thread(session_info.clone(), &workspace, window, cx);
            }))
            .into_any_element()
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
            .offset(gpui::Point {
                x: px(0.0),
                y: px(2.0),
            })
    }

    fn render_filter_input(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: cx.theme().colors().text,
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_fallbacks: settings.ui_font.fallbacks.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.ui_font.weight,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            ..Default::default()
        };

        EditorElement::new(
            &self.filter_editor,
            EditorStyle {
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
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

        let (icon, label) = if is_fully_expanded {
            (IconName::ListCollapse, "Collapse List")
        } else {
            (IconName::Plus, "View More")
        };

        ListItem::new(id)
            .focused(is_selected)
            .child(
                h_flex()
                    .p_1()
                    .gap_1p5()
                    .child(Icon::new(icon).size(IconSize::Small).color(Color::Muted))
                    .child(Label::new(label).color(Color::Muted))
                    .when(!is_fully_expanded, |this| {
                        this.child(
                            Label::new(format!("({})", remaining_count))
                                .color(Color::Custom(cx.theme().colors().text_muted.opacity(0.5))),
                        )
                    }),
            )
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
        let workspace = workspace.clone();

        div()
            .w_full()
            .p_2()
            .child(
                Button::new(
                    SharedString::from(format!("new-thread-btn-{}", ix)),
                    "New Thread",
                )
                .full_width()
                .style(ButtonStyle::Outlined)
                .icon(IconName::Plus)
                .icon_color(Color::Muted)
                .icon_size(IconSize::Small)
                .icon_position(IconPosition::Start)
                .toggle_state(is_selected)
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.selection = None;
                    this.create_new_thread(&workspace, window, cx);
                })),
            )
            .into_any_element()
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
}

impl Focusable for Sidebar {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.filter_editor.focus_handle(cx)
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar_height = ui::utils::platform_title_bar_height(window);
        let ui_font = theme::setup_ui_font(window, cx);
        let is_focused = self.focus_handle.is_focused(window)
            || self.filter_editor.focus_handle(cx).is_focused(window);
        let has_query = self.has_filter_query(cx);

        let focus_tooltip_label = if is_focused {
            "Focus Workspace"
        } else {
            "Focus Sidebar"
        };

        v_flex()
            .id("workspace-sidebar")
            .key_context("WorkspaceSidebar")
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
                        let focus_handle_toggle = self.focus_handle.clone();
                        let focus_handle_focus = self.focus_handle.clone();
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
                                                &focus_handle_toggle,
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
                                                &focus_handle_focus,
                                                cx,
                                            )),
                                    )
                                    .into_any_element()
                            }))
                            .on_click(cx.listener(|_this, _, _window, cx| {
                                cx.emit(SidebarEvent::Close);
                            }))
                    })
                    .child(self.render_recent_projects_button(cx)),
            )
            .child(
                h_flex()
                    .flex_none()
                    .px_2p5()
                    .h(Tab::container_height(cx))
                    .gap_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        Icon::new(IconName::MagnifyingGlass)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(self.render_filter_input(cx))
                    .when(has_query, |this| {
                        this.pr_1().child(
                            IconButton::new("clear_filter", IconName::Close)
                                .shape(IconButtonShape::Square)
                                .tooltip(Tooltip::text("Clear Search"))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.reset_filter_editor_text(window, cx);
                                    this.update_entries(cx);
                                })),
                        )
                    }),
            )
            .child(
                v_flex()
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
                    .vertical_scrollbar_for(&self.list_state, window, cx),
            )
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
    use settings::SettingsStore;
    use std::sync::Arc;
    use util::path_list::PathList;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            cx.update_flags(false, vec!["agent-v2".into()]);
            ThreadStore::init_global(cx);
        });
    }

    fn make_test_thread(title: &str, updated_at: DateTime<Utc>) -> agent::DbThread {
        agent::DbThread {
            title: title.to_string().into(),
            messages: Vec::new(),
            updated_at,
            detailed_summary: None,
            initial_project_snapshot: None,
            cumulative_token_usage: Default::default(),
            request_token_usage: Default::default(),
            model: None,
            profile: None,
            imported: false,
            subagent_context: None,
            speed: None,
            thinking_enabled: false,
            thinking_effort: None,
            draft_prompt: None,
            ui_scroll_position: None,
        }
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
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.register_sidebar(sidebar.clone(), window, cx);
        });
        cx.run_until_parked();
        sidebar
    }

    async fn save_n_test_threads(
        count: u32,
        path_list: &PathList,
        cx: &mut gpui::VisualTestContext,
    ) {
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));
        for i in 0..count {
            let save_task = thread_store.update(cx, |store, cx| {
                store.save_thread(
                    acp::SessionId::new(Arc::from(format!("thread-{}", i))),
                    make_test_thread(
                        &format!("Thread {}", i + 1),
                        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, i).unwrap(),
                    ),
                    path_list.clone(),
                    cx,
                )
            });
            save_task.await.unwrap();
        }
        cx.run_until_parked();
    }

    async fn save_thread_to_store(
        session_id: &acp::SessionId,
        path_list: &PathList,
        cx: &mut gpui::VisualTestContext,
    ) {
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));
        let save_task = thread_store.update(cx, |store, cx| {
            store.save_thread(
                session_id.clone(),
                make_test_thread(
                    "Test",
                    chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
                ),
                path_list.clone(),
                cx,
            )
        });
        save_task.await.unwrap();
        cx.run_until_parked();
    }

    fn open_and_focus_sidebar(
        sidebar: &Entity<Sidebar>,
        multi_workspace: &Entity<MultiWorkspace>,
        cx: &mut gpui::VisualTestContext,
    ) {
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.toggle_sidebar(window, cx);
        });
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
                            format!(
                                "  {}{}{}{}{}",
                                title, active, status_str, notified, selected
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
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));

        let save_task = thread_store.update(cx, |store, cx| {
            store.save_thread(
                acp::SessionId::new(Arc::from("thread-1")),
                make_test_thread(
                    "Fix crash in project panel",
                    chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 3, 0, 0, 0).unwrap(),
                ),
                path_list.clone(),
                cx,
            )
        });
        save_task.await.unwrap();

        let save_task = thread_store.update(cx, |store, cx| {
            store.save_thread(
                acp::SessionId::new(Arc::from("thread-2")),
                make_test_thread(
                    "Add inline diff view",
                    chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
                ),
                path_list.clone(),
                cx,
            )
        });
        save_task.await.unwrap();
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
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));

        let save_task = thread_store.update(cx, |store, cx| {
            store.save_thread(
                acp::SessionId::new(Arc::from("thread-a1")),
                make_test_thread(
                    "Thread A1",
                    chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
                ),
                path_list.clone(),
                cx,
            )
        });
        save_task.await.unwrap();
        cx.run_until_parked();

        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [project-a]", "  Thread A1"]
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

        // Initially shows 5 threads + View More (12 remaining)
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 7); // header + 5 threads + View More
        assert!(entries.iter().any(|e| e.contains("View More (12)")));

        // Focus and navigate to View More, then confirm to expand by one batch
        open_and_focus_sidebar(&sidebar, &multi_workspace, cx);
        for _ in 0..7 {
            cx.dispatch_action(SelectNext);
        }
        cx.dispatch_action(Confirm);
        cx.run_until_parked();

        // Now shows 10 threads + View More (7 remaining)
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 12); // header + 10 threads + View More
        assert!(entries.iter().any(|e| e.contains("View More (7)")));

        // Expand again by one batch
        sidebar.update_in(cx, |s, _window, cx| {
            let current = s.expanded_groups.get(&path_list).copied().unwrap_or(0);
            s.expanded_groups.insert(path_list.clone(), current + 1);
            s.update_entries(cx);
        });
        cx.run_until_parked();

        // Now shows 15 threads + View More (2 remaining)
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 17); // header + 15 threads + View More
        assert!(entries.iter().any(|e| e.contains("View More (2)")));

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

        // Back to initial state: 5 threads + View More (12 remaining)
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 7); // header + 5 threads + View More
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
                    has_threads: true,
                },
                // Thread with default (Completed) status, not active
                ListEntry::Thread(ThreadEntry {
                    session_info: acp_thread::AgentSessionInfo {
                        session_id: acp::SessionId::new(Arc::from("t-1")),
                        cwd: None,
                        title: Some("Completed thread".into()),
                        updated_at: Some(Utc::now()),
                        created_at: Some(Utc::now()),
                        meta: None,
                    },
                    icon: IconName::ZedAgent,
                    icon_from_external_svg: None,
                    status: AgentThreadStatus::Completed,
                    workspace: workspace.clone(),
                    is_live: false,
                    is_background: false,
                    highlight_positions: Vec::new(),
                }),
                // Active thread with Running status
                ListEntry::Thread(ThreadEntry {
                    session_info: acp_thread::AgentSessionInfo {
                        session_id: acp::SessionId::new(Arc::from("t-2")),
                        cwd: None,
                        title: Some("Running thread".into()),
                        updated_at: Some(Utc::now()),
                        created_at: Some(Utc::now()),
                        meta: None,
                    },
                    icon: IconName::ZedAgent,
                    icon_from_external_svg: None,
                    status: AgentThreadStatus::Running,
                    workspace: workspace.clone(),
                    is_live: true,
                    is_background: false,
                    highlight_positions: Vec::new(),
                }),
                // Active thread with Error status
                ListEntry::Thread(ThreadEntry {
                    session_info: acp_thread::AgentSessionInfo {
                        session_id: acp::SessionId::new(Arc::from("t-3")),
                        cwd: None,
                        title: Some("Error thread".into()),
                        updated_at: Some(Utc::now()),
                        created_at: Some(Utc::now()),
                        meta: None,
                    },
                    icon: IconName::ZedAgent,
                    icon_from_external_svg: None,
                    status: AgentThreadStatus::Error,
                    workspace: workspace.clone(),
                    is_live: true,
                    is_background: false,
                    highlight_positions: Vec::new(),
                }),
                // Thread with WaitingForConfirmation status, not active
                ListEntry::Thread(ThreadEntry {
                    session_info: acp_thread::AgentSessionInfo {
                        session_id: acp::SessionId::new(Arc::from("t-4")),
                        cwd: None,
                        title: Some("Waiting thread".into()),
                        updated_at: Some(Utc::now()),
                        created_at: Some(Utc::now()),
                        meta: None,
                    },
                    icon: IconName::ZedAgent,
                    icon_from_external_svg: None,
                    status: AgentThreadStatus::WaitingForConfirmation,
                    workspace: workspace.clone(),
                    is_live: false,
                    is_background: false,
                    highlight_positions: Vec::new(),
                }),
                // Background thread that completed (should show notification)
                ListEntry::Thread(ThreadEntry {
                    session_info: acp_thread::AgentSessionInfo {
                        session_id: acp::SessionId::new(Arc::from("t-5")),
                        cwd: None,
                        title: Some("Notified thread".into()),
                        updated_at: Some(Utc::now()),
                        created_at: Some(Utc::now()),
                        meta: None,
                    },
                    icon: IconName::ZedAgent,
                    icon_from_external_svg: None,
                    status: AgentThreadStatus::Completed,
                    workspace: workspace.clone(),
                    is_live: true,
                    is_background: true,
                    highlight_positions: Vec::new(),
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
                    has_threads: true,
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

        // Entries: [header, thread3, thread2, thread1]
        // Focusing the sidebar does not set a selection; select_next/select_previous
        // handle None gracefully by starting from the first or last entry.
        open_and_focus_sidebar(&sidebar, &multi_workspace, cx);
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

        // At the end, selection stays on the last entry
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(3));

        // Move back up

        cx.dispatch_action(SelectPrevious);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(2));

        cx.dispatch_action(SelectPrevious);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

        cx.dispatch_action(SelectPrevious);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));

        // At the top, selection stays on the first entry
        cx.dispatch_action(SelectPrevious);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));
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

        open_and_focus_sidebar(&sidebar, &multi_workspace, cx);

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
        open_and_focus_sidebar(&sidebar, &multi_workspace, cx);
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
    async fn test_keyboard_confirm_on_project_header_activates_workspace(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.create_workspace(window, cx);
        });
        cx.run_until_parked();

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
        save_n_test_threads(1, &path_list, cx).await;
        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
                "  Thread 1",
                "v [Empty Workspace]",
                "  [+ New Thread]",
            ]
        );

        // Switch to workspace 1 so we can verify confirm switches back.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate_index(1, window, cx);
        });
        cx.run_until_parked();
        assert_eq!(
            multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
            1
        );

        // Focus the sidebar and manually select the header (index 0)
        open_and_focus_sidebar(&sidebar, &multi_workspace, cx);
        sidebar.update_in(cx, |sidebar, _window, _cx| {
            sidebar.selection = Some(0);
        });

        // Press confirm on project header (workspace 0) to activate it.
        cx.dispatch_action(Confirm);
        cx.run_until_parked();

        assert_eq!(
            multi_workspace.read_with(cx, |mw, _| mw.active_workspace_index()),
            0
        );

        // Focus should have moved out of the sidebar to the workspace center.
        let workspace_0 = multi_workspace.read_with(cx, |mw, _cx| mw.workspaces()[0].clone());
        workspace_0.update_in(cx, |workspace, window, cx| {
            let pane_focus = workspace.active_pane().read(cx).focus_handle(cx);
            assert!(
                pane_focus.contains_focused(window, cx),
                "Confirming a project header should focus the workspace center pane"
            );
        });
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

        // Should show header + 5 threads + "View More (3)"
        let entries = visible_entries_as_strings(&sidebar, cx);
        assert_eq!(entries.len(), 7);
        assert!(entries.iter().any(|e| e.contains("View More (3)")));

        // Focus sidebar (selection starts at None), then navigate down to the "View More" entry (index 6)
        open_and_focus_sidebar(&sidebar, &multi_workspace, cx);
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
        open_and_focus_sidebar(&sidebar, &multi_workspace, cx);
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
            vec!["v [my-project]  <== selected", "  Thread 1",]
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
        open_and_focus_sidebar(&sidebar, &multi_workspace, cx);
        cx.dispatch_action(SelectNext);
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]", "  Thread 1  <== selected",]
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
        open_and_focus_sidebar(&sidebar, &multi_workspace, cx);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), None);

        // First SelectNext from None starts at index 0 (header)
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));

        // SelectNext moves to the new thread button
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

        // At the end, selection stays on the last entry
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

        // SelectPrevious goes back to the header
        cx.dispatch_action(SelectPrevious);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(0));
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
        open_and_focus_sidebar(&sidebar, &multi_workspace, cx);
        cx.dispatch_action(SelectNext);
        cx.dispatch_action(SelectNext);
        assert_eq!(sidebar.read_with(cx, |s, _| s.selection), Some(1));

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
            language_model::LanguageModelRegistry::test(cx);
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
        save_thread_to_store(&session_id_a, &path_list, cx).await;

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
        save_thread_to_store(&session_id_b, &path_list, cx).await;

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
        save_thread_to_store(&session_id_a, &path_list_a, cx).await;

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
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));

        for (id, title, hour) in [
            ("t-1", "Fix crash in project panel", 3),
            ("t-2", "Add inline diff view", 2),
            ("t-3", "Refactor settings module", 1),
        ] {
            let save_task = thread_store.update(cx, |store, cx| {
                store.save_thread(
                    acp::SessionId::new(Arc::from(id)),
                    make_test_thread(
                        title,
                        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
                    ),
                    path_list.clone(),
                    cx,
                )
            });
            save_task.await.unwrap();
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
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));

        let save_task = thread_store.update(cx, |store, cx| {
            store.save_thread(
                acp::SessionId::new(Arc::from("thread-1")),
                make_test_thread(
                    "Fix Crash In Project Panel",
                    chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
                ),
                path_list.clone(),
                cx,
            )
        });
        save_task.await.unwrap();
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
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));

        for (id, title, hour) in [("t-1", "Alpha thread", 2), ("t-2", "Beta thread", 1)] {
            let save_task = thread_store.update(cx, |store, cx| {
                store.save_thread(
                    acp::SessionId::new(Arc::from(id)),
                    make_test_thread(
                        title,
                        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
                    ),
                    path_list.clone(),
                    cx,
                )
            });
            save_task.await.unwrap();
        }
        cx.run_until_parked();

        // Confirm the full list is showing.
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]", "  Alpha thread", "  Beta thread",]
        );

        // User types a search query to filter down.
        open_and_focus_sidebar(&sidebar, &multi_workspace, cx);
        type_in_search(&sidebar, "alpha", cx);
        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec!["v [my-project]", "  Alpha thread  <== selected",]
        );

        // User presses Escape — filter clears, full list is restored.
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
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));

        for (id, title, hour) in [
            ("a1", "Fix bug in sidebar", 2),
            ("a2", "Add tests for editor", 1),
        ] {
            let save_task = thread_store.update(cx, |store, cx| {
                store.save_thread(
                    acp::SessionId::new(Arc::from(id)),
                    make_test_thread(
                        title,
                        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
                    ),
                    path_list_a.clone(),
                    cx,
                )
            });
            save_task.await.unwrap();
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
            let save_task = thread_store.update(cx, |store, cx| {
                store.save_thread(
                    acp::SessionId::new(Arc::from(id)),
                    make_test_thread(
                        title,
                        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
                    ),
                    path_list_b.clone(),
                    cx,
                )
            });
            save_task.await.unwrap();
        }
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [project-a]",
                "  Fix bug in sidebar",
                "  Add tests for editor",
                "v [Empty Workspace]",
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
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));

        for (id, title, hour) in [
            ("a1", "Fix bug in sidebar", 2),
            ("a2", "Add tests for editor", 1),
        ] {
            let save_task = thread_store.update(cx, |store, cx| {
                store.save_thread(
                    acp::SessionId::new(Arc::from(id)),
                    make_test_thread(
                        title,
                        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
                    ),
                    path_list_a.clone(),
                    cx,
                )
            });
            save_task.await.unwrap();
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
            let save_task = thread_store.update(cx, |store, cx| {
                store.save_thread(
                    acp::SessionId::new(Arc::from(id)),
                    make_test_thread(
                        title,
                        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
                    ),
                    path_list_b.clone(),
                    cx,
                )
            });
            save_task.await.unwrap();
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
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));

        // Create 8 threads. The oldest one has a unique name and will be
        // behind View More (only 5 shown by default).
        for i in 0..8u32 {
            let title = if i == 0 {
                "Hidden gem thread".to_string()
            } else {
                format!("Thread {}", i + 1)
            };
            let save_task = thread_store.update(cx, |store, cx| {
                store.save_thread(
                    acp::SessionId::new(Arc::from(format!("thread-{}", i))),
                    make_test_thread(
                        &title,
                        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, i).unwrap(),
                    ),
                    path_list.clone(),
                    cx,
                )
            });
            save_task.await.unwrap();
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
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));

        let save_task = thread_store.update(cx, |store, cx| {
            store.save_thread(
                acp::SessionId::new(Arc::from("thread-1")),
                make_test_thread(
                    "Important thread",
                    chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
                ),
                path_list.clone(),
                cx,
            )
        });
        save_task.await.unwrap();
        cx.run_until_parked();

        // User focuses the sidebar and collapses the group using keyboard:
        // manually select the header, then press CollapseSelectedEntry to collapse.
        open_and_focus_sidebar(&sidebar, &multi_workspace, cx);
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
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));

        for (id, title, hour) in [
            ("t-1", "Fix crash in panel", 3),
            ("t-2", "Fix lint warnings", 2),
            ("t-3", "Add new feature", 1),
        ] {
            let save_task = thread_store.update(cx, |store, cx| {
                store.save_thread(
                    acp::SessionId::new(Arc::from(id)),
                    make_test_thread(
                        title,
                        chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, hour, 0, 0).unwrap(),
                    ),
                    path_list.clone(),
                    cx,
                )
            });
            save_task.await.unwrap();
        }
        cx.run_until_parked();

        open_and_focus_sidebar(&sidebar, &multi_workspace, cx);

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
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));

        let save_task = thread_store.update(cx, |store, cx| {
            store.save_thread(
                acp::SessionId::new(Arc::from("hist-1")),
                make_test_thread(
                    "Historical Thread",
                    chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 6, 1, 0, 0, 0).unwrap(),
                ),
                path_list.clone(),
                cx,
            )
        });
        save_task.await.unwrap();
        cx.run_until_parked();
        multi_workspace.update_in(cx, |_, _window, cx| cx.notify());
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&sidebar, cx),
            vec![
                "v [my-project]",
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
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));

        let save_task = thread_store.update(cx, |store, cx| {
            store.save_thread(
                acp::SessionId::new(Arc::from("t-1")),
                make_test_thread(
                    "Thread A",
                    chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 2, 0, 0, 0).unwrap(),
                ),
                path_list.clone(),
                cx,
            )
        });
        save_task.await.unwrap();
        let save_task = thread_store.update(cx, |store, cx| {
            store.save_thread(
                acp::SessionId::new(Arc::from("t-2")),
                make_test_thread(
                    "Thread B",
                    chrono::TimeZone::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap(),
                ),
                path_list.clone(),
                cx,
            )
        });
        save_task.await.unwrap();
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
        save_thread_to_store(&session_id, &path_list, cx).await;
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
        save_thread_to_store(&session_id_a, &path_list_a, cx).await;

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
        // Workspace B is active (just added), so its header is the active entry.
        sidebar.read_with(cx, |sidebar, _cx| {
            assert_eq!(
                sidebar.focused_thread, None,
                "Initially no thread should be focused"
            );
            let active_entry = sidebar
                .active_entry_index
                .and_then(|ix| sidebar.contents.entries.get(ix));
            assert!(
                matches!(active_entry, Some(ListEntry::ProjectHeader { .. })),
                "Active entry should be the active workspace header"
            );
        });

        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.activate_thread(
                acp_thread::AgentSessionInfo {
                    session_id: session_id_a.clone(),
                    cwd: None,
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
        save_thread_to_store(&session_id_b, &path_list_b, cx).await;
        cx.run_until_parked();

        // Workspace A is currently active. Click a thread in workspace B,
        // which also triggers a workspace switch.
        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.activate_thread(
                acp_thread::AgentSessionInfo {
                    session_id: session_id_b.clone(),
                    cwd: None,
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
            let active_entry = sidebar
                .active_entry_index
                .and_then(|ix| sidebar.contents.entries.get(ix));
            assert!(
                matches!(active_entry, Some(ListEntry::ProjectHeader { .. })),
                "Active entry should be the workspace header after external switch"
            );
        });

        let connection_b2 = StubAgentConnection::new();
        connection_b2.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("New thread".into()),
        )]);
        open_thread_with_connection(&panel_b, connection_b2, cx);
        send_message(&panel_b, cx);
        let session_id_b2 = active_session_id(&panel_b, cx);
        save_thread_to_store(&session_id_b2, &path_list_b, cx).await;
        cx.run_until_parked();

        sidebar.read_with(cx, |sidebar, _cx| {
            assert_eq!(
                sidebar.focused_thread.as_ref(),
                Some(&session_id_b2),
                "Opening a thread externally should set focused_thread"
            );
        });

        workspace_b.update_in(cx, |workspace, window, cx| {
            workspace.focus_handle(cx).focus(window, cx);
        });
        cx.run_until_parked();

        sidebar.read_with(cx, |sidebar, _cx| {
            assert_eq!(
                sidebar.focused_thread.as_ref(),
                Some(&session_id_b2),
                "Defocusing the sidebar should not clear focused_thread"
            );
        });

        sidebar.update_in(cx, |sidebar, window, cx| {
            sidebar.activate_workspace(&workspace_b, window, cx);
        });
        cx.run_until_parked();

        sidebar.read_with(cx, |sidebar, _cx| {
            assert_eq!(
                sidebar.focused_thread, None,
                "Clicking a workspace header should clear focused_thread"
            );
            let active_entry = sidebar
                .active_entry_index
                .and_then(|ix| sidebar.contents.entries.get(ix));
            assert!(
                matches!(active_entry, Some(ListEntry::ProjectHeader { .. })),
                "Active entry should be the workspace header"
            );
        });

        // ── 8. Focusing the agent panel thread restores focused_thread ────
        // Workspace B still has session_id_b2 loaded in the agent panel.
        // Clicking into the thread (simulated by focusing its view) should
        // set focused_thread via the ThreadFocused event.
        panel_b.update_in(cx, |panel, window, cx| {
            if let Some(thread_view) = panel.active_connection_view() {
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
}
