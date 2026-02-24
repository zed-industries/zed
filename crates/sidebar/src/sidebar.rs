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
    FocusWorkspaceSidebar, MultiWorkspace, NewWorkspaceInWindow, PathList,
    Sidebar as WorkspaceSidebar, SidebarEvent, ToggleWorkspaceSidebar, Workspace,
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
    thread_info: Option<AgentThreadInfo>,
}

impl WorkspaceThreadEntry {
    fn new(index: usize, workspace: &Entity<Workspace>, cx: &App) -> Self {
        let workspace_ref = workspace.read(cx);

        let worktrees: Vec<_> = workspace_ref
            .worktrees(cx)
            .filter(|worktree| worktree.read(cx).is_visible())
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

        let thread_info = Self::thread_info(workspace, cx);

        Self {
            index,
            worktree_label,
            full_path,
            thread_info,
        }
    }

    fn thread_info(workspace: &Entity<Workspace>, cx: &App) -> Option<AgentThreadInfo> {
        workspace_thread_info(workspace, cx)
    }
}

fn workspace_thread_info(workspace: &Entity<Workspace>, cx: &App) -> Option<AgentThreadInfo> {
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

/// A ProjectGroup is a group of workspaces, each one associated with a specific
/// git worktree or the main worktree.
#[derive(Default)]
struct ProjectGroup {
    workspaces: Vec<Entity<Workspace>>,
}

impl ProjectGroup {
    const fn len(&self) -> usize {
        self.workspaces.len()
    }
}

/// Manages a group of [`ProjectGroup`]s
///
/// This is responsible for grouping projects by their worktrees, keeping track
/// of group names, and any sorting we might apply.
///
/// Although this is backed by a vec, we should never expose direct vec indices
/// as part of the [`ActiveProjects`] API.
struct ActiveProjects {
    /// An association list mapping keys to projects
    ///
    /// We use an associate list because the set of project groups should be
    /// small enough that iterating a vec is likely to be faster than a more
    /// complex structure like a BTreeMap. It also let's us sort the groups by
    /// sorting the vec in different ways.
    groups: Vec<(PathList, ProjectGroup)>,
}

impl ActiveProjects {
    fn empty() -> Self {
        Self { groups: Vec::new() }
    }

    /// Create a new [`ActiveProjects`] populated from a slice of workspaces.
    fn from_workspaces(workspaces: &[Entity<Workspace>], cx: &App) -> Self {
        let mut active_projects = Self::empty();

        for workspace in workspaces {
            active_projects.add_workspace(workspace.clone(), cx);
        }

        active_projects
    }

    fn add_workspace(&mut self, workspace: Entity<Workspace>, cx: &App) {
        let paths = workspace.read(cx).root_paths(cx);
        let key = PathList::new(&paths);

        match self.groups.iter_mut().find(|(k, _)| *k == key) {
            Some((_key, group)) => {
                group.workspaces.push(workspace);
            }
            None => {
                self.groups.push((
                    key,
                    ProjectGroup {
                        workspaces: vec![workspace],
                    },
                ));
            }
        }
    }

    /// Returns the total number of projects across all groups.
    fn num_projects(&self) -> usize {
        self.groups.iter().map(|(_, group)| group.len()).sum()
    }

    /// Returns the project, its path list, and whether this is the first item
    /// in the group (so we can draw a header).
    fn project_by_ix(&self, mut ix: usize) -> Option<(Entity<Workspace>, Option<&PathList>)> {
        for (path_list, group) in self.groups.iter() {
            if ix < group.len() {
                return Some((group.workspaces[ix].clone(), Some(path_list)));
            }
            ix -= group.len();
        }
        None
    }
}

struct ActiveProjectsDelegate {
    multi_workspace: Entity<MultiWorkspace>,
    /// The primary list of things shown in the sidebar.
    active_projects: ActiveProjects,
}

impl ActiveProjectsDelegate {
    fn new(
        multi_workspace: Entity<MultiWorkspace>,
        workspaces: &[Entity<Workspace>],
        cx: &App,
    ) -> Self {
        let active_projects = ActiveProjects::from_workspaces(workspaces, cx);

        Self {
            multi_workspace,
            active_projects,
        }
    }
}

impl PickerDelegate for ActiveProjectsDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.active_projects.num_projects()
    }

    fn selected_index(&self) -> usize {
        0
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
    }

    fn can_select(
        &mut self,
        _ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> bool {
        false
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search…".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        None
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        cx.spawn_in(window, async move |_picker, cx| {})
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {}

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        index: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let (workspace, paths_if_first) = self.active_projects.project_by_ix(index)?;

        Some(
            v_flex()
                .when(paths_if_first.is_some(), |el| {
                    let header_label: SharedString = paths_if_first
                        .unwrap()
                        .ordered_paths()
                        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
                        .collect::<Vec<_>>()
                        .join(", ")
                        .into();
                    el.child(ListSubHeader::new(header_label).inset(true))
                })
                .child(Label::new("todo: workspace thread row").color(Color::Muted))
                .into_any_element(),
        )
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
    picker: Entity<Picker<ActiveProjectsDelegate>>,
    _subscription: Subscription,
    _project_subscriptions: Vec<Subscription>,
    _agent_panel_subscriptions: Vec<Subscription>,
    _thread_subscriptions: Vec<Subscription>,
    #[cfg(any(test, feature = "test-support"))]
    test_thread_infos: HashMap<usize, AgentThreadInfo>,
    #[cfg(any(test, feature = "test-support"))]
    test_recent_project_thread_titles: HashMap<SharedString, SharedString>,
}

impl EventEmitter<SidebarEvent> for Sidebar {}

impl Sidebar {
    pub fn new(
        multi_workspace: Entity<MultiWorkspace>,
        workspaces: &[Entity<Workspace>],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = ActiveProjectsDelegate::new(multi_workspace.clone(), workspaces, cx);
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
        };
        this.update_entries(window, cx);
        this
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_test_recent_projects(
        &self,
        projects: Vec<RecentProjectEntry>,
        cx: &mut Context<Self>,
    ) {
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
    }

    /// Reconciles the sidebar's displayed entries with the current state of all
    /// workspaces and their agent threads.
    fn update_entries(&mut self, window: &mut Window, cx: &mut Context<Self>) {}
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
        false
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
            .child(sidebar_header(
                window,
                cx,
                titlebar_height,
                focus_tooltip_label,
            ))
            .child(self.picker.clone())
    }
}

/// Renders the sidebar header, including the expand/collapse button and the new
/// thread button.
fn sidebar_header(
    window: &mut Window,
    cx: &mut Context<'_, Sidebar>,
    titlebar_height: Pixels,
    focus_tooltip_label: impl Into<SharedString>,
) -> impl IntoElement {
    let focus_tooltip_label = focus_tooltip_label.into();

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
                                .child(Label::new(focus_tooltip_label.clone()))
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
        )
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

        let sidebar = multi_workspace.update_in(cx, |mw, window, cx| {
            let mw_handle = cx.entity();
            let workspaces = mw.workspaces().to_vec();
            cx.new(|cx| Sidebar::new(mw_handle, &workspaces, window, cx))
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

        let sidebar = multi_workspace.update_in(cx, |mw, window, cx| {
            let mw_handle = cx.entity();
            let workspaces = mw.workspaces().to_vec();
            cx.new(|cx| Sidebar::new(mw_handle, &workspaces, window, cx))
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

        let sidebar = multi_workspace.update_in(cx, |mw, window, cx| {
            let mw_handle = cx.entity();
            let workspaces = mw.workspaces().to_vec();
            cx.new(|cx| Sidebar::new(mw_handle, &workspaces, window, cx))
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
