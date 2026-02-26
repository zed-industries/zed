use acp_thread::ThreadStatus;
use agent::ThreadStore;
use agent_client_protocol as acp;
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
    Sidebar as WorkspaceSidebar, SidebarEvent, ToggleWorkspaceSidebar, Workspace, WorkspaceStore,
};

/*
 *
 * Active Projects (serialized, managed by the user):
 * - zed
 * - zed,ex
 * - ex
 * - zed.dev
 * - zed.dev,cloud
 * - cloud
 *
 * Windows (totally seperate, but can overlap with active projects):
 * Window 1: cloud
 * Window 2: zed,ex
 * Window 3: alacritty <----????? How do you navigate back? Does it show up at all? Where's it going???
 *
 * Threads (Annotates the final set of projects with the associated thread data):
 * Thread1 - cloud-olivetti
 * Thread2 - zed.dev
 * Thread3 - ex
 * Thread4 - cloud-olivetti
 * Thread5 - zed.dev
 * Thread6 - ex
 * Thread7 - cloud-olivetti
 * Thread8 - zed.dev
 * Thread9 - ex
 *
 *
 * What the sidebar shows, is the union of Active Projects and Windows
 * And the threads, which intersect with that overall set.
 *
 * Do this everytime the data chagnes to _derive_ the project groups from the underlying data:
 *
 * let project_groups = Vec::new();
 * for each window {
 *   project_groups.push(window.projects);
 * }
 *
 * for each active_project {
 *   if project_groups.does_not_contain(project) {
 *      project_groups.push(window.projects);
 *   }
 * }
 *
 * for each thread {
 *    project_groups.for_project(thread.project).push(thread.data)
 * }
 *
 * Sidebar contents:
 * - cloud 🪟
 *    - Thread1 - olivetti   [x] <- When you click this x, delete the thread
 *    - Thread4 - olivetti
 *    - Thread7 - olivetti
 * - alacritty 🪟            [x] <- When you click this x, close the window
 * - zed                     [x] <- When you click this x, remove it from the active projects list
 * - zed,ex 🪟               [x] <- When you click this x, remove it from the active projects list & close the window
 * - ex
 *   - Thread3
 *   - Thread6
 *   - Thread9
 * - zed.dev
 *  - Thread2
 *  - Thread5
 *  - Thread8
 * - zed.dev,cloud
 *
 *  ^ What happens when I click [x]????
 *
 *
 *
 *
 */

// 2 datasets:
// - The list of threads (We want to derive from and watch this data)
//  - Threads contain worktree and project data in them.
// - The list of active projects (App-global, cross-window), a super set, of the collection of projects open in windows
//  - If you have 3 windows, with zed, zed+cloud, zed.dev open in each, your active projects _inherently has all 3 of those_
//  - + Every project you've opened before and started a thread in
//  - Interesting digression: You can have projects open with no threads, we need to render that,
//    but we also want to automatically remove those from the active projects list.
//  - If you click "x" on a project in this set, what happens?
//    - It's removed from active projects.
//    - It's threads stay in the database but aren't rendered
//    - It's window (if any) is closed.
// - This sidebar is the RIGHT JOIN of these two sets

const DEFAULT_WIDTH: Pixels = px(320.0);
const MIN_WIDTH: Pixels = px(200.0);
const MAX_WIDTH: Pixels = px(800.0);
const MAX_MATCHES: usize = 100;

/// Returns the session ID of the workspace's active agent thread, if any.
fn workspace_session_id(workspace: &Entity<Workspace>, cx: &App) -> Option<acp::SessionId> {
    let agent_panel = workspace.read(cx).panel::<AgentPanel>(cx)?;
    let thread = agent_panel.read(cx).active_agent_thread(cx)?;
    Some(thread.read(cx).session_id().clone())
}

// We're going to have a pile of active projects
// Those are going to have workspaces, and worktrees, and all this stuff,
//  - Key point: They're all going to be live representations.

/// A single workspace entry within a project group.
///
/// Thread metadata (title, timestamp, etc.) is read live from [`ThreadStore`]
/// via the `session_id`.
struct ProjectEntry {
    workspace: Entity<Workspace>,
    session_id: Option<acp::SessionId>,
}

impl ProjectEntry {
    /// Derives the group name from the workspace's root paths.
    fn group_name(&self, cx: &App) -> SharedString {
        let paths = self.workspace.read(cx).root_paths(cx);
        if paths.is_empty() {
            return "Untitled Project".into();
        }
        paths
            .iter()
            .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .collect::<Vec<_>>()
            .join(", ")
            .into()
    }

    /// Returns the thread title from the database via [`ThreadStore`].
    /// Falls back to "New Thread" if no thread is associated yet.
    fn thread_title(&self, cx: &App) -> SharedString {
        let Some(session_id) = self.session_id.as_ref() else {
            return "New Thread".into();
        };
        let thread_store = ThreadStore::global(cx);
        thread_store
            .read(cx)
            .thread_from_session_id(session_id)
            .map(|m| m.title.clone())
            .unwrap_or_else(|| "New Thread".into())
    }
}

/// A ProjectGroup is a group of workspaces, each one associated with a specific
/// git worktree or the main worktree.
///
///
/// This maintains the invariant, that every ProjectEntry, is for the same
/// "Project Group", same means "main path list"
#[derive(Default)]
struct ProjectGroup {
    path_list: PathList, // FIXME: let's pull this out of ProjectEntry in case the user changes the path name, etc.
    entries: Vec<Entity<ProjectEntry>>,
}

impl ProjectGroup {
    fn from_workspace(workspace: Entity<Workspace>, cx: &mut App) -> Self {
        let paths = workspace.read(cx).root_paths(cx);
        let path_list = PathList::new(&paths);
        let session_id = workspace_session_id(&workspace, cx);

        let entry = cx.new(|_| ProjectEntry {
            workspace,
            session_id,
        });

        Self {
            path_list,
            entries: vec![entry],
        }
    }

    fn add_project(&mut self, workspace: Entity<Workspace>, cx: &mut App) {
        assert_eq!(
            PathList::new(&workspace.read(cx).root_paths(cx)),
            self.path_list,
            "project must share root path"
        );

        if !self.contains(&workspace, cx) {
            let session_id = workspace_session_id(&workspace, cx);
            let entry = cx.new(|_| ProjectEntry {
                workspace,
                session_id,
            });
            self.entries.push(entry);
        }
    }

    fn contains(&self, workspace: &Entity<Workspace>, cx: &App) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.read(cx).workspace == *workspace)
    }

    const fn len(&self) -> usize {
        self.entries.len()
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
    groups: Vec<ProjectGroup>,
}

impl ActiveProjects {
    fn empty() -> Self {
        Self { groups: Vec::new() }
    }

    /// Create a new [`ActiveProjects`] populated from a slice of workspaces.
    fn from_workspaces(workspaces: &[Entity<Workspace>], cx: &mut App) -> Self {
        let mut active_projects = Self::empty();

        for workspace in workspaces {
            active_projects.add_project(workspace.clone(), cx);
        }

        active_projects
    }

    fn add_project(&mut self, workspace: Entity<Workspace>, cx: &mut App) {
        let paths = workspace.read(cx).root_paths(cx);
        let key = PathList::new(&paths);

        match self.groups.iter_mut().find(|g| g.path_list == key) {
            Some(group) => {
                group.add_project(workspace, cx);
            }
            None => {
                self.groups
                    .push(ProjectGroup::from_workspace(workspace, cx));
            }
        }
    }

    /// Returns the total number of projects across all groups.
    fn num_projects(&self) -> usize {
        self.groups.iter().map(|group| group.len()).sum()
    }

    /// Iterate over all project entries across all groups, in group order.
    fn iter(&self) -> impl Iterator<Item = &Entity<ProjectEntry>> {
        self.groups.iter().flat_map(|group| group.entries.iter())
    }

    /// Preserve session IDs from a previous snapshot for workspaces that
    /// temporarily have no active thread (e.g. mid-switch). The actual
    /// thread metadata is always read live from [`ThreadStore`].
    fn preserve_session_ids_from(&mut self, old: &ActiveProjects, cx: &mut App) {
        for group in &mut self.groups {
            for entry in &mut group.entries {
                let entry_ref = entry.read(cx);
                if entry_ref.session_id.is_none() {
                    // Find the same workspace in the old data and carry forward its session ID.
                    let old_session_id = old
                        .iter()
                        .find(|old| old.read(cx).workspace == entry_ref.workspace)
                        .and_then(|old| old.read(cx).session_id.clone());
                    if let Some(id) = old_session_id {
                        entry.update(cx, |e, _| e.session_id = Some(id));
                    }
                }
            }
        }
    }
}

struct ActiveProjectsDelegate {
    /// Handle to the [`MultiWorkspace`] that owns this window. Used for
    /// window-local operations like activating a workspace in *this* window.
    /// The global workspace list comes from `workspace_store` instead.
    multi_workspace: Entity<MultiWorkspace>,
    workspace_store: Entity<WorkspaceStore>,
    /// The primary list of things shown in the sidebar.
    active_projects: ActiveProjects,
    /// Flat view of all project entries in group order, for Picker indexing.
    ///
    /// Note that `active_projects` is the source of truth and this is simply
    /// built on top of that for displaying in a list.
    flat_entries: Vec<Entity<ProjectEntry>>,
    selected_index: usize,
}

impl ActiveProjectsDelegate {
    fn new(
        multi_workspace: Entity<MultiWorkspace>,
        workspace_store: Entity<WorkspaceStore>,
    ) -> Self {
        Self {
            multi_workspace,
            workspace_store,
            active_projects: ActiveProjects::empty(),
            flat_entries: Vec::new(),
            selected_index: 0,
        }
    }
}

impl PickerDelegate for ActiveProjectsDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.flat_entries.len()
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
        _ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> bool {
        true
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

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry) = self.flat_entries.get(self.selected_index) else {
            return;
        };
        let workspace = entry.read(cx).workspace.clone();
        self.multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.activate(workspace, cx);
        });
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        index: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = self.flat_entries.get(index)?;
        let entry_ref = entry.read(cx);

        let show_header = index == 0 || {
            let prev = self.flat_entries[index - 1].read(cx);
            prev.group_name(cx) != entry_ref.group_name(cx)
        };

        let thread_title = entry_ref.thread_title(cx);

        Some(
            v_flex()
                .when(show_header, |el| {
                    el.child(ListSubHeader::new(entry_ref.group_name(cx)).inset(true))
                })
                .child(Label::new(thread_title).color(Color::Muted))
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
    /// Handle to the [`MultiWorkspace`] that owns this window. Used for
    /// window-local operations. The global workspace list
    /// comes from `workspace_store` instead.
    multi_workspace: Entity<MultiWorkspace>,
    workspace_store: Entity<WorkspaceStore>,
    width: Pixels,
    picker: Entity<Picker<ActiveProjectsDelegate>>,
    _subscription: Subscription,
    _workspace_store_subscription: Subscription,
    _thread_store_subscription: Option<Subscription>,
    _project_subscriptions: Vec<Subscription>,
    _agent_panel_subscriptions: Vec<Subscription>,
    _thread_subscriptions: Vec<Subscription>,
    #[cfg(any(test, feature = "test-support"))]
    test_statuses: HashMap<usize, AgentThreadStatus>,
    #[cfg(any(test, feature = "test-support"))]
    test_recent_project_thread_titles: HashMap<SharedString, SharedString>,
}

impl EventEmitter<SidebarEvent> for Sidebar {}

impl Sidebar {
    /// Creates a new sidebar for the given window.
    ///
    /// `workspace_store` must be passed explicitly (rather than derived from
    /// `multi_workspace`) so that this constructor can be called inside an
    /// `observe_new` callback where the `MultiWorkspace` entity is already
    /// mutably borrowed — reading through its entity handle would panic.
    pub fn new(
        multi_workspace: Entity<MultiWorkspace>,
        workspace_store: Entity<WorkspaceStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate =
            ActiveProjectsDelegate::new(multi_workspace.clone(), workspace_store.clone());
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

        let workspace_store_subscription =
            cx.observe_in(&workspace_store, window, |this, _, window, cx| {
                this.update_entries(window, cx);
            });

        // Observe ThreadStore so the sidebar refreshes when thread metadata
        // changes (e.g. title updated after summarization, thread deleted).
        let thread_store_subscription = ThreadStore::try_global(cx).map(|thread_store| {
            cx.observe_in(&thread_store, window, |this, _, window, cx| {
                this.update_entries(window, cx);
            })
        });

        let mut this = Self {
            multi_workspace,
            workspace_store,
            width: DEFAULT_WIDTH,
            picker,
            _subscription: subscription,
            _workspace_store_subscription: workspace_store_subscription,
            _thread_store_subscription: thread_store_subscription,
            _project_subscriptions: Vec::new(),
            _agent_panel_subscriptions: Vec::new(),
            _thread_subscriptions: Vec::new(),
            #[cfg(any(test, feature = "test-support"))]
            test_statuses: HashMap::new(),
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
        _title: SharedString,
        status: AgentThreadStatus,
    ) {
        // Title now comes from DbThreadMetadata via ThreadStore.
        // We only track status here for future notification logic.
        self.test_statuses.insert(index, status);
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_test_recent_project_thread_title(
        &mut self,
        full_path: SharedString,
        title: SharedString,
        cx: &mut Context<Self>,
    ) {
    }

    /// Collects the full set of workspaces the sidebar should display: the
    /// union of windowed workspaces (ephemeral) and active projects (durable).
    fn collect_all_workspaces(
        workspace_store: &WorkspaceStore,
        cx: &App,
    ) -> Vec<Entity<Workspace>> {
        let mut seen = HashSet::new();
        let mut workspaces = Vec::new();

        // Windowed workspaces (ephemeral — visible because they have a window).
        for (_, weak_workspace) in workspace_store.workspaces_with_windows() {
            if let Some(workspace) = weak_workspace.upgrade() {
                if seen.insert(workspace.entity_id()) {
                    workspaces.push(workspace);
                }
            }
        }

        // Active projects (durable — persisted because they've had threads).
        for workspace in workspace_store.active_projects() {
            if seen.insert(workspace.entity_id()) {
                workspaces.push(workspace.clone());
            }
        }

        workspaces
    }

    /// Reconciles the sidebar's displayed entries with the current state of all
    /// workspaces and their agent threads.
    fn update_entries(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let multi_workspace = self.multi_workspace.clone();
        let workspace_store = self.workspace_store.clone();
        cx.defer_in(window, move |this, window, cx| {
            let active_workspace = multi_workspace.read(cx).workspace().clone();
            let workspaces = Self::collect_all_workspaces(workspace_store.read(cx), cx);

            // Rebuild the active projects from scratch, preserving session IDs
            // for workspaces that temporarily have no active thread.
            let mut active_projects = ActiveProjects::from_workspaces(&workspaces, cx);
            this.picker.update(cx, |picker, cx| {
                active_projects.preserve_session_ids_from(&picker.delegate.active_projects, cx);
                let flat_entries: Vec<_> = active_projects.iter().cloned().collect();
                let selected_index = flat_entries
                    .iter()
                    .position(|e| e.read(cx).workspace == active_workspace)
                    .unwrap_or(0);
                picker.delegate.active_projects = active_projects;
                picker.delegate.flat_entries = flat_entries;
                picker.delegate.selected_index = selected_index;
                let query = picker.query(cx);
                picker.update_matches(query, window, cx);
            });

            // Re-subscribe to project worktree changes (add/remove/reorder).
            this._project_subscriptions = workspaces
                .iter()
                .map(|workspace| {
                    let project = workspace.read(cx).project().clone();
                    cx.subscribe_in(&project, window, |this, _project, event, window, cx| {
                        // FIXME: we should be able to handle these more cheaply
                        // than rebuilding everything at once.
                        match event {
                            ProjectEvent::WorktreeAdded(_)
                            | ProjectEvent::WorktreeRemoved(_)
                            | ProjectEvent::WorktreeOrderChanged => {
                                this.update_entries(window, cx);
                            }
                            _ => {}
                        }
                    })
                })
                .collect();

            // Re-subscribe to agent panel events (thread switched, etc.).
            this._agent_panel_subscriptions = workspaces
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
                        // re-subscribe once the panel appears.
                        cx.observe_in(workspace, window, |this, _, window, cx| {
                            this.update_entries(window, cx);
                        })
                    }
                })
                .collect();

            // Re-subscribe to active thread changes (title, status).
            this._thread_subscriptions = workspaces
                .iter()
                .filter_map(|workspace| {
                    let agent_panel = workspace.read(cx).panel::<AgentPanel>(cx)?;
                    let thread = agent_panel.read(cx).active_agent_thread(cx)?;
                    Some(cx.observe_in(&thread, window, |this, _, window, cx| {
                        this.update_entries(window, cx);
                    }))
                })
                .collect();
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
            let workspace_store = mw.workspace().read(cx).app_state().workspace_store.clone();
            cx.new(|cx| Sidebar::new(mw_handle, workspace_store, window, cx))
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
            let workspace_store = mw.workspace().read(cx).app_state().workspace_store.clone();
            cx.new(|cx| Sidebar::new(mw_handle, workspace_store, window, cx))
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
            let workspace_store = mw.workspace().read(cx).app_state().workspace_store.clone();
            cx.new(|cx| Sidebar::new(mw_handle, workspace_store, window, cx))
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
