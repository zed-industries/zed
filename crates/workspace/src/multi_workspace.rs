use feature_flags::{AgentV2FeatureFlag, FeatureFlagAppExt};
use gpui::{Action, App, Context, Entity, ManagedView, Render, Window, actions, px};
use project::Project;
use theme::ActiveTheme;
use ui::{IconButton, IconName, IconSize, Label, LabelSize, ListItem, SharedString, prelude::*};

use crate::{
    DockPosition, Item, ModalView, Panel, Workspace, WorkspaceId, client_side_decorations,
};

actions!(
    multi_workspace,
    [
        /// Creates a new workspace within the current window.
        NewWorkspaceInWindow,
        /// Switches to the next workspace within the current window.
        NextWorkspaceInWindow,
        /// Switches to the previous workspace within the current window.
        PreviousWorkspaceInWindow,
        /// Toggles the workspace switcher sidebar.
        ToggleWorkspaceSidebar,
    ]
);

pub struct MultiWorkspace {
    workspaces: Vec<Entity<Workspace>>,
    active_workspace_index: usize,
    sidebar_open: bool,
}

impl MultiWorkspace {
    pub fn new(workspace: Entity<Workspace>, _cx: &mut Context<Self>) -> Self {
        Self {
            workspaces: vec![workspace],
            active_workspace_index: 0,
            sidebar_open: false,
        }
    }

    fn multi_workspace_enabled(&self, cx: &App) -> bool {
        cx.has_flag::<AgentV2FeatureFlag>()
    }

    pub fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        if !self.multi_workspace_enabled(cx) {
            return;
        }
        self.sidebar_open = !self.sidebar_open;
        cx.notify();
    }

    pub fn workspace(&self) -> &Entity<Workspace> {
        &self.workspaces[self.active_workspace_index]
    }

    pub fn workspaces(&self) -> &[Entity<Workspace>] {
        &self.workspaces
    }

    pub fn active_workspace_index(&self) -> usize {
        self.active_workspace_index
    }

    pub fn activate(&mut self, workspace: Entity<Workspace>, cx: &mut Context<Self>) {
        if !self.multi_workspace_enabled(cx) {
            // In single workspace mode, replace the current workspace
            self.workspaces[0] = workspace;
            self.active_workspace_index = 0;
            cx.notify();
            return;
        }

        // Multi-workspace mode: insert if not present, then activate
        let index = self
            .workspaces
            .iter()
            .position(|w| *w == workspace)
            .unwrap_or_else(|| {
                self.workspaces.push(workspace);
                self.workspaces.len() - 1
            });
        if self.active_workspace_index != index {
            self.active_workspace_index = index;
            cx.notify();
        }
    }

    fn activate_index(&mut self, index: usize, cx: &mut Context<Self>) {
        debug_assert!(
            index < self.workspaces.len(),
            "workspace index out of bounds"
        );
        self.active_workspace_index = index;
        cx.notify();
    }

    pub fn activate_next_workspace(&mut self, cx: &mut Context<Self>) {
        if self.workspaces.len() > 1 {
            let next_index = (self.active_workspace_index + 1) % self.workspaces.len();
            self.activate_index(next_index, cx);
        }
    }

    pub fn activate_previous_workspace(&mut self, cx: &mut Context<Self>) {
        if self.workspaces.len() > 1 {
            let prev_index = if self.active_workspace_index == 0 {
                self.workspaces.len() - 1
            } else {
                self.active_workspace_index - 1
            };
            self.activate_index(prev_index, cx);
        }
    }

    pub fn panel<T: Panel>(&self, cx: &App) -> Option<Entity<T>> {
        self.workspace().read(cx).panel::<T>(cx)
    }

    pub fn active_modal<V: ManagedView + 'static>(&self, cx: &App) -> Option<Entity<V>> {
        self.workspace().read(cx).active_modal::<V>(cx)
    }

    pub fn add_panel<T: Panel>(
        &mut self,
        panel: Entity<T>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.workspace().update(cx, |workspace, cx| {
            workspace.add_panel(panel, window, cx);
        });
    }

    pub fn focus_panel<T: Panel>(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<T>> {
        self.workspace()
            .update(cx, |workspace, cx| workspace.focus_panel::<T>(window, cx))
    }

    pub fn toggle_modal<V: ModalView, B>(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        build: B,
    ) where
        B: FnOnce(&mut Window, &mut gpui::Context<V>) -> V,
    {
        self.workspace().update(cx, |workspace, cx| {
            workspace.toggle_modal(window, cx, build);
        });
    }

    pub fn toggle_dock(
        &mut self,
        dock_side: DockPosition,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.workspace().update(cx, |workspace, cx| {
            workspace.toggle_dock(dock_side, window, cx);
        });
    }

    pub fn active_item_as<I: 'static>(&self, cx: &App) -> Option<Entity<I>> {
        self.workspace().read(cx).active_item_as::<I>(cx)
    }

    pub fn items_of_type<'a, T: Item>(
        &'a self,
        cx: &'a App,
    ) -> impl 'a + Iterator<Item = Entity<T>> {
        self.workspace().read(cx).items_of_type::<T>(cx)
    }

    pub fn database_id(&self, cx: &App) -> Option<WorkspaceId> {
        self.workspace().read(cx).database_id()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_random_database_id(&mut self, cx: &mut Context<Self>) {
        self.workspace().update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
        Self::new(workspace, cx)
    }
}

impl Render for MultiWorkspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar_height = (1.75 * window.rem_size()).max(px(34.));
        let multi_workspace_enabled = self.multi_workspace_enabled(cx);

        let sidebar = if multi_workspace_enabled && self.sidebar_open {
            let items: Vec<_> = self
                .workspaces
                .iter()
                .enumerate()
                .map(|(index, workspace)| {
                    let is_active = index == self.active_workspace_index;
                    let worktree_names: Vec<String> = workspace
                        .read(cx)
                        .worktrees(cx)
                        .filter_map(|wt| {
                            wt.read(cx)
                                .abs_path()
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                        })
                        .collect();
                    let label: SharedString = if worktree_names.is_empty() {
                        format!("Workspace {}", index + 1).into()
                    } else {
                        worktree_names.join(", ").into()
                    };

                    ListItem::new(("workspace-item", index))
                        .inset(true)
                        .toggle_state(is_active)
                        .on_click(cx.listener(move |this, _, _window, cx| {
                            this.activate_index(index, cx);
                            this.sidebar_open = false;
                            cx.notify();
                        }))
                        .child(Label::new(label))
                })
                .collect();

            Some(
                div()
                    .id("workspace-sidebar")
                    .h_full()
                    .w_64()
                    .flex()
                    .flex_col()
                    .bg(cx.theme().colors().surface_background)
                    .border_r_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        div()
                            .h(titlebar_height)
                            .w_full()
                            .pl(px(80.))
                            .flex()
                            .items_center()
                            .justify_between()
                            .pr_2()
                            .child(Label::new("Workspaces").size(LabelSize::Small))
                            .child(
                                IconButton::new("new-workspace", IconName::Plus)
                                    .icon_size(IconSize::Small)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        window.dispatch_action(
                                            NewWorkspaceInWindow.boxed_clone(),
                                            cx,
                                        );
                                        this.sidebar_open = false;
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(ui::ListSeparator)
                    .child(
                        div()
                            .id("workspace-sidebar-content")
                            .flex_1()
                            .overflow_y_scroll()
                            .p_1()
                            .children(items),
                    ),
            )
        } else {
            None
        };

        client_side_decorations(
            div()
                .size_full()
                .flex()
                .flex_row()
                .on_action(
                    cx.listener(|this: &mut Self, _: &NewWorkspaceInWindow, window, cx| {
                        if !this.multi_workspace_enabled(cx) {
                            return;
                        }
                        let app_state = this.workspace().read(cx).app_state().clone();
                        let project = Project::local(
                            app_state.client.clone(),
                            app_state.node_runtime.clone(),
                            app_state.user_store.clone(),
                            app_state.languages.clone(),
                            app_state.fs.clone(),
                            None,
                            project::LocalProjectFlags::default(),
                            cx,
                        );
                        let new_workspace =
                            cx.new(|cx| Workspace::new(None, project, app_state, window, cx));
                        this.activate(new_workspace, cx);
                    }),
                )
                .on_action(cx.listener(
                    |this: &mut Self, _: &NextWorkspaceInWindow, _window, cx| {
                        this.activate_next_workspace(cx);
                    },
                ))
                .on_action(cx.listener(
                    |this: &mut Self, _: &PreviousWorkspaceInWindow, _window, cx| {
                        this.activate_previous_workspace(cx);
                    },
                ))
                .on_action(cx.listener(
                    |this: &mut Self, _: &ToggleWorkspaceSidebar, _window, cx| {
                        this.toggle_sidebar(cx);
                    },
                ))
                .children(sidebar)
                .child(div().flex_1().size_full().child(self.workspace().clone())),
            window,
            cx,
        )
    }
}
