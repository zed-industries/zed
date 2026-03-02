use acp_thread::ThreadStatus;
use agent::ThreadStore;
use agent_client_protocol as acp;
use agent_ui::{AgentPanel, AgentPanelEvent};
use chrono::{DateTime, Utc};
use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, FocusHandle, Focusable, ListState, Pixels,
    Render, SharedString, Subscription, Window, list, prelude::*, px,
};
use project::Event as ProjectEvent;
use std::collections::{HashMap, HashSet};
use theme::ActiveTheme;
use ui::utils::TRAFFIC_LIGHT_PADDING;
use ui::{AgentThreadStatus, KeyBinding, Tooltip, prelude::*};
use util::path_list::PathList;
use workspace::{
    FocusWorkspaceSidebar, MultiWorkspace, NewWorkspaceInWindow, Sidebar as WorkspaceSidebar,
    SidebarEvent, ToggleWorkspaceSidebar, Workspace,
};

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
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum ListEntry {
    ProjectHeader {
        path_list: PathList,
        label: SharedString,
    },
    Thread {
        session_id: acp::SessionId,
        title: SharedString,
        icon: IconName,
        status: AgentThreadStatus,
        updated_at: DateTime<Utc>,
        diff_stats: Option<(usize, usize)>,
        workspace_index: Option<usize>,
    },
    ViewMore {
        path_list: PathList,
        remaining_count: usize,
    },
}

pub struct Sidebar {
    // Reference cycle with the Workspace?
    multi_workspace: Entity<MultiWorkspace>,
    width: Pixels,
    focus_handle: FocusHandle,
    list_state: ListState,
    entries: Vec<ListEntry>,
    selection: Option<usize>,
    collapsed_groups: HashSet<PathList>,
    expanded_groups: HashSet<PathList>,
    notified_workspaces: HashSet<usize>,
    _subscription: Subscription,
    _project_subscriptions: Vec<Subscription>,
    _agent_panel_subscriptions: Vec<Subscription>,
    _thread_subscriptions: Vec<Subscription>,
    _thread_store_subscription: Option<Subscription>,
}

impl EventEmitter<SidebarEvent> for Sidebar {}

impl Sidebar {
    pub fn new(
        multi_workspace: Entity<MultiWorkspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscription = cx.observe_in(
            &multi_workspace,
            window,
            |this, _multi_workspace, window, cx| {
                this.update_entries(window, cx);
            },
        );

        let mut this = Self {
            multi_workspace,
            width: DEFAULT_WIDTH,
            focus_handle: cx.focus_handle(),
            list_state: ListState::new(0, gpui::ListAlignment::Top, px(1000.)),
            entries: Vec::new(),
            selection: None,
            collapsed_groups: HashSet::new(),
            expanded_groups: HashSet::new(),
            notified_workspaces: HashSet::new(),
            _subscription: subscription,
            _project_subscriptions: Vec::new(),
            _agent_panel_subscriptions: Vec::new(),
            _thread_subscriptions: Vec::new(),
            _thread_store_subscription: None,
        };
        this.update_entries(window, cx);
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
                            this.update_entries(window, cx);
                        }
                        _ => {}
                    },
                )
            })
            .collect()
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

    fn subscribe_to_thread_store(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self._thread_store_subscription.is_some() {
            return;
        }
        if let Some(thread_store) = ThreadStore::try_global(cx) {
            self._thread_store_subscription =
                Some(cx.observe_in(&thread_store, window, |this, _, window, cx| {
                    this.update_entries(window, cx);
                }));
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

    fn active_thread_info_for_workspace(
        workspace: &Entity<Workspace>,
        cx: &App,
    ) -> Option<ActiveThreadInfo> {
        let agent_panel = workspace.read(cx).panel::<AgentPanel>(cx)?;
        let agent_panel_ref = agent_panel.read(cx);
        let thread_view = agent_panel_ref.as_active_thread_view(cx)?;
        let thread_view_ref = thread_view.read(cx);
        let thread = thread_view_ref.thread.read(cx);

        let icon = thread_view_ref.agent_icon;
        let title = thread.title();
        let session_id = thread.session_id().clone();

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

        Some(ActiveThreadInfo {
            session_id,
            title,
            status,
            icon,
        })
    }

    fn update_entries(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let multi_workspace = self.multi_workspace.clone();
        cx.defer_in(window, move |this, window, cx| {
            if !this.multi_workspace.read(cx).multi_workspace_enabled(cx) {
                return;
            }

            this._project_subscriptions = this.subscribe_to_projects(window, cx);
            this._agent_panel_subscriptions = this.subscribe_to_agent_panels(window, cx);
            this._thread_subscriptions = this.subscribe_to_threads(window, cx);
            this.subscribe_to_thread_store(window, cx);

            let (workspaces, active_workspace_index) = {
                let mw = multi_workspace.read(cx);
                (mw.workspaces().to_vec(), mw.active_workspace_index())
            };

            let thread_store = ThreadStore::try_global(cx);

            let had_notifications = !this.notified_workspaces.is_empty();

            let old_statuses: HashMap<usize, AgentThreadStatus> = this
                .entries
                .iter()
                .filter_map(|entry| match entry {
                    ListEntry::Thread {
                        workspace_index: Some(index),
                        status,
                        ..
                    } => Some((*index, *status)),
                    _ => None,
                })
                .collect();

            this.entries.clear();

            for (index, workspace) in workspaces.iter().enumerate() {
                let (path_list, label) =
                    Self::workspace_path_list_and_label(workspace, cx);

                this.entries.push(ListEntry::ProjectHeader {
                    path_list: path_list.clone(),
                    label,
                });

                if this.collapsed_groups.contains(&path_list) {
                    continue;
                }

                let mut threads: Vec<ListEntry> = Vec::new();

                if let Some(ref thread_store) = thread_store {
                    for meta in thread_store.read(cx).threads_for_paths(&path_list) {
                        threads.push(ListEntry::Thread {
                            session_id: meta.id.clone(),
                            title: meta.title.clone(),
                            icon: IconName::ZedAgent,
                            status: AgentThreadStatus::default(),
                            updated_at: meta.updated_at,
                            diff_stats: None,
                            workspace_index: None,
                        });
                    }
                }

                let active_info = Self::active_thread_info_for_workspace(workspace, cx);

                if let Some(info) = &active_info {
                    let existing = threads.iter_mut().find(|t| {
                        matches!(t, ListEntry::Thread { session_id, .. } if session_id == &info.session_id)
                    });

                    if let Some(existing) = existing {
                        if let ListEntry::Thread {
                            status,
                            icon,
                            workspace_index,
                            title,
                            ..
                        } = existing
                        {
                            *status = info.status;
                            *icon = info.icon;
                            *workspace_index = Some(index);
                            *title = info.title.clone();
                        }
                    } else {
                        threads.push(ListEntry::Thread {
                            session_id: info.session_id.clone(),
                            title: info.title.clone(),
                            icon: info.icon,
                            status: info.status,
                            updated_at: Utc::now(),
                            diff_stats: None,
                            workspace_index: Some(index),
                        });
                    }
                }

                // Detect Running → Completed transitions on background workspaces.
                for thread in &threads {
                    if let ListEntry::Thread {
                        workspace_index: Some(workspace_idx),
                        status,
                        ..
                    } = thread
                    {
                        if *status == AgentThreadStatus::Completed
                            && *workspace_idx != active_workspace_index
                            && old_statuses.get(workspace_idx) == Some(&AgentThreadStatus::Running)
                        {
                            this.notified_workspaces.insert(*workspace_idx);
                        }
                    }
                }

                threads.sort_by(|a, b| {
                    let a_time = match a {
                        ListEntry::Thread { updated_at, .. } => updated_at,
                        _ => unreachable!(),
                    };
                    let b_time = match b {
                        ListEntry::Thread { updated_at, .. } => updated_at,
                        _ => unreachable!(),
                    };
                    b_time.cmp(a_time)
                });

                let total = threads.len();
                let show_view_more =
                    total > DEFAULT_THREADS_SHOWN && !this.expanded_groups.contains(&path_list);

                let count = if show_view_more {
                    DEFAULT_THREADS_SHOWN
                } else {
                    total
                };

                this.entries.extend(threads.into_iter().take(count));

                if show_view_more {
                    this.entries.push(ListEntry::ViewMore {
                        path_list: path_list.clone(),
                        remaining_count: total - DEFAULT_THREADS_SHOWN,
                    });
                }
            }

            this.notified_workspaces.remove(&active_workspace_index);

            this.list_state.reset(this.entries.len());

            if let Some(selection) = this.selection {
                if selection >= this.entries.len() {
                    this.selection = this.entries.len().checked_sub(1);
                }
            }

            let has_notifications = !this.notified_workspaces.is_empty();
            if had_notifications != has_notifications {
                multi_workspace.update(cx, |_, cx| cx.notify());
            }

            cx.notify();
        });
    }

    fn render_list_entry(
        &mut self,
        ix: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(entry) = self.entries.get(ix) else {
            return div().into_any_element();
        };
        let is_selected = self.selection == Some(ix);

        match entry {
            ListEntry::ProjectHeader { path_list, label } => {
                self.render_project_header(path_list, label, cx)
            }
            ListEntry::Thread {
                title,
                icon,
                status,
                workspace_index,
                ..
            } => self.render_thread(ix, title, *icon, *status, *workspace_index, is_selected, cx),
            ListEntry::ViewMore {
                path_list,
                remaining_count,
            } => self.render_view_more(ix, path_list, *remaining_count, cx),
        }
    }

    fn render_project_header(
        &self,
        path_list: &PathList,
        label: &SharedString,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let is_collapsed = self.collapsed_groups.contains(path_list);
        let disclosure_icon = if is_collapsed {
            IconName::ChevronRight
        } else {
            IconName::ChevronDown
        };
        let path_list = path_list.clone();

        h_flex()
            .id(SharedString::from(format!("project-header-{}", label)))
            .w_full()
            .px_2()
            .py_1()
            .gap_1()
            .child(
                Icon::new(disclosure_icon)
                    .size(IconSize::Small)
                    .color(Color::Muted),
            )
            .child(
                Label::new(label.clone())
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, window, cx| {
                this.toggle_collapse(&path_list, window, cx);
            }))
            .into_any_element()
    }

    fn toggle_collapse(
        &mut self,
        path_list: &PathList,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.collapsed_groups.contains(path_list) {
            self.collapsed_groups.remove(path_list);
        } else {
            self.collapsed_groups.insert(path_list.clone());
        }
        self.update_entries(window, cx);
    }

    fn render_thread(
        &self,
        ix: usize,
        title: &SharedString,
        icon: IconName,
        status: AgentThreadStatus,
        workspace_index: Option<usize>,
        is_selected: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let running = matches!(
            status,
            AgentThreadStatus::Running | AgentThreadStatus::WaitingForConfirmation
        );

        let has_notification = workspace_index
            .map(|idx| self.notified_workspaces.contains(&idx))
            .unwrap_or(false);

        let is_active = workspace_index.is_some();

        let multi_workspace = self.multi_workspace.clone();

        h_flex()
            .id(SharedString::from(format!("thread-entry-{}", ix)))
            .w_full()
            .px_2()
            .py_1()
            .gap_2()
            .when(is_selected, |this| {
                this.bg(cx.theme().colors().ghost_element_selected)
            })
            .rounded_md()
            .cursor_pointer()
            .child(Icon::new(icon).size(IconSize::Small).color(if running {
                Color::Accent
            } else {
                Color::Muted
            }))
            .child(
                div().flex_1().overflow_hidden().child(
                    Label::new(title.clone())
                        .size(LabelSize::Small)
                        .single_line()
                        .color(if is_active {
                            Color::Default
                        } else {
                            Color::Muted
                        }),
                ),
            )
            .when(running, |this| {
                this.child(
                    Label::new("Running")
                        .size(LabelSize::XSmall)
                        .color(Color::Accent),
                )
            })
            .when(has_notification, |this| {
                this.child(div().size_2().rounded_full().bg(cx.theme().status().info))
            })
            .on_click(cx.listener(move |_this, _, window, cx| {
                if let Some(target_index) = workspace_index {
                    multi_workspace.update(cx, |multi_workspace, cx| {
                        multi_workspace.activate_index(target_index, window, cx);
                    });
                }
            }))
            .into_any_element()
    }

    fn render_view_more(
        &self,
        ix: usize,
        path_list: &PathList,
        remaining_count: usize,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let path_list = path_list.clone();

        h_flex()
            .id(SharedString::from(format!("view-more-{}", ix)))
            .w_full()
            .px_2()
            .py_1()
            .cursor_pointer()
            .child(
                Label::new(format!("+ View More ({})", remaining_count))
                    .size(LabelSize::Small)
                    .color(Color::Accent),
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                this.expanded_groups.insert(path_list.clone());
                this.update_entries(window, cx);
            }))
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
        !self.notified_workspaces.is_empty()
    }
}

impl Focusable for Sidebar {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar_height = ui::utils::platform_title_bar_height(window);
        let ui_font = theme::setup_ui_font(window, cx);
        let is_focused = self.focus_handle.is_focused(window);

        let focus_tooltip_label = if is_focused {
            "Focus Workspace"
        } else {
            "Focus Sidebar"
        };

        v_flex()
            .id("workspace-sidebar")
            .key_context("WorkspaceSidebar")
            .track_focus(&self.focus_handle)
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
            .child(
                div().flex_1().overflow_hidden().child(
                    list(
                        self.list_state.clone(),
                        cx.processor(Self::render_list_entry),
                    )
                    .size_full(),
                ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent::ThreadStore;
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
        let sidebar = multi_workspace.update_in(cx, |_mw, window, cx| {
            let mw_handle = cx.entity();
            cx.new(|cx| Sidebar::new(mw_handle, window, cx))
        });
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.register_sidebar(sidebar.clone(), window, cx);
        });
        cx.run_until_parked();
        sidebar
    }

    fn visible_entries_as_strings(
        sidebar: &Entity<Sidebar>,
        cx: &mut gpui::VisualTestContext,
    ) -> Vec<String> {
        sidebar.read_with(cx, |sidebar, _cx| {
            sidebar
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
                            label, path_list, ..
                        } => {
                            let icon = if sidebar.collapsed_groups.contains(path_list) {
                                ">"
                            } else {
                                "v"
                            };
                            format!("{} [{}]{}", icon, label, selected)
                        }
                        ListEntry::Thread {
                            title,
                            status,
                            workspace_index,
                            ..
                        } => {
                            let active = if workspace_index.is_some() { " *" } else { "" };
                            let status_str = match status {
                                AgentThreadStatus::Running => " (running)",
                                AgentThreadStatus::Error => " (error)",
                                AgentThreadStatus::WaitingForConfirmation => " (waiting)",
                                _ => "",
                            };
                            format!("  {}{}{}{}", title, active, status_str, selected)
                        }
                        ListEntry::ViewMore {
                            remaining_count, ..
                        } => {
                            format!("  + View More ({}){}", remaining_count, selected)
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
            vec!["v [my-project]"]
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
            vec!["v [project-a]", "  Thread A1", "v [Empty Workspace]"]
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
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));

        for i in 0..12 {
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
    async fn test_collapse_and_expand_group(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let path_list = PathList::new(&[std::path::PathBuf::from("/my-project")]);
        let thread_store = cx.update(|_window, cx| ThreadStore::global(cx));

        let save_task = thread_store.update(cx, |store, cx| {
            store.save_thread(
                acp::SessionId::new(Arc::from("test-thread")),
                make_test_thread(
                    "Test Thread",
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
            vec!["v [my-project]", "  Test Thread"]
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
            vec!["v [my-project]", "  Test Thread"]
        );
    }

    #[gpui::test]
    async fn test_visible_entries_as_strings(cx: &mut TestAppContext) {
        let project = init_test_project("/my-project", cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let sidebar = setup_sidebar(&multi_workspace, cx);

        let expanded_path = PathList::new(&[std::path::PathBuf::from("/expanded")]);
        let collapsed_path = PathList::new(&[std::path::PathBuf::from("/collapsed")]);

        sidebar.update_in(cx, |s, _window, _cx| {
            s.collapsed_groups.insert(collapsed_path.clone());
            s.entries = vec![
                // Expanded project header
                ListEntry::ProjectHeader {
                    path_list: expanded_path.clone(),
                    label: "expanded-project".into(),
                },
                // Thread with default (Completed) status, not active
                ListEntry::Thread {
                    session_id: acp::SessionId::new(Arc::from("t-1")),
                    title: "Completed thread".into(),
                    icon: IconName::ZedAgent,
                    status: AgentThreadStatus::Completed,
                    updated_at: Utc::now(),
                    diff_stats: None,
                    workspace_index: None,
                },
                // Active thread with Running status
                ListEntry::Thread {
                    session_id: acp::SessionId::new(Arc::from("t-2")),
                    title: "Running thread".into(),
                    icon: IconName::ZedAgent,
                    status: AgentThreadStatus::Running,
                    updated_at: Utc::now(),
                    diff_stats: None,
                    workspace_index: Some(0),
                },
                // Active thread with Error status
                ListEntry::Thread {
                    session_id: acp::SessionId::new(Arc::from("t-3")),
                    title: "Error thread".into(),
                    icon: IconName::ZedAgent,
                    status: AgentThreadStatus::Error,
                    updated_at: Utc::now(),
                    diff_stats: None,
                    workspace_index: Some(1),
                },
                // Thread with WaitingForConfirmation status, not active
                ListEntry::Thread {
                    session_id: acp::SessionId::new(Arc::from("t-4")),
                    title: "Waiting thread".into(),
                    icon: IconName::ZedAgent,
                    status: AgentThreadStatus::WaitingForConfirmation,
                    updated_at: Utc::now(),
                    diff_stats: None,
                    workspace_index: None,
                },
                // View More entry
                ListEntry::ViewMore {
                    path_list: expanded_path.clone(),
                    remaining_count: 42,
                },
                // Collapsed project header
                ListEntry::ProjectHeader {
                    path_list: collapsed_path.clone(),
                    label: "collapsed-project".into(),
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
                "  + View More (42)",
                "> [collapsed-project]",
            ]
        );

        // Move selection to the collapsed header
        sidebar.update_in(cx, |s, _window, _cx| {
            s.selection = Some(6);
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
}
