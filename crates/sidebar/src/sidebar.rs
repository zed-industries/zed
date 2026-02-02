use gpui::{Action, App, Context, Entity, EventEmitter, Pixels, Render, Subscription, Window, px};
use theme::ActiveTheme;
use ui::{
    IconButton, IconName, IconSize, InteractiveElement, IntoElement, Label, LabelSize, ListItem,
    ParentElement, SharedString, Styled, div, prelude::*,
};
use workspace::{
    MultiWorkspace, NewWorkspaceInWindow, Sidebar as WorkspaceSidebar, SidebarEvent, Workspace,
};

const DEFAULT_WIDTH: Pixels = px(256.0);
const MIN_WIDTH: Pixels = px(150.0);
const MAX_WIDTH: Pixels = px(600.0);

pub struct Sidebar {
    multi_workspace: Entity<MultiWorkspace>,
    width: Pixels,
    _subscription: Subscription,
}

impl EventEmitter<SidebarEvent> for Sidebar {}

impl Sidebar {
    pub fn new(multi_workspace: Entity<MultiWorkspace>, cx: &mut Context<Self>) -> Self {
        let subscription = cx.observe(&multi_workspace, |_, _, cx| cx.notify());

        Self {
            multi_workspace,
            width: DEFAULT_WIDTH,
            _subscription: subscription,
        }
    }

    fn render_workspace_item(
        &self,
        index: usize,
        workspace: &Entity<Workspace>,
        is_active: bool,
        cx: &App,
    ) -> ListItem {
        let worktree_names: Vec<String> = workspace
            .read(cx)
            .worktrees(cx)
            .filter_map(|worktree| {
                worktree
                    .read(cx)
                    .abs_path()
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
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
            .child(Label::new(label))
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
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar_height = (1.75 * window.rem_size()).max(px(34.));
        let multi_workspace = self.multi_workspace.clone();

        let workspaces = self.multi_workspace.read(cx).workspaces().to_vec();
        let active_index = self.multi_workspace.read(cx).active_workspace_index();

        let items: Vec<_> = workspaces
            .iter()
            .enumerate()
            .map(|(index, workspace)| {
                let is_active = index == active_index;
                let multi_workspace = multi_workspace.clone();

                self.render_workspace_item(index, workspace, is_active, cx)
                    .on_click(cx.listener(move |_, _, _window, cx| {
                        multi_workspace.update(cx, |mw, cx| {
                            mw.activate_index(index, cx);
                        });
                        cx.emit(SidebarEvent::Close);
                    }))
            })
            .collect();

        div()
            .id("workspace-sidebar")
            .h_full()
            .w(self.width)
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
                            .on_click(cx.listener(|_this, _, window, cx| {
                                window.dispatch_action(NewWorkspaceInWindow.boxed_clone(), cx);
                                cx.emit(SidebarEvent::Close);
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
            )
    }
}
