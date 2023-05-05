use context_menu::{ContextMenu, ContextMenuItem};
use gpui::{
    elements::*,
    platform::{CursorStyle, MouseButton},
    AnyElement, Element, Entity, View, ViewContext, ViewHandle, WeakViewHandle,
};
use settings::Settings;
use workspace::Workspace;

///! TODO: This file will hold the branch switching UI once we build it.

pub struct BranchesButton {
    workspace: WeakViewHandle<Workspace>,
    popup_menu: ViewHandle<ContextMenu>,
}

impl Entity for BranchesButton {
    type Event = ();
}

impl View for BranchesButton {
    fn ui_name() -> &'static str {
        "BranchesButton"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let Some(workspace) = self.workspace.upgrade(cx) else {
            return Empty::new().into_any();
        };

        let project = workspace.read(cx).project().read(cx);
        let only_one_worktree = project.visible_worktrees(cx).count() == 1;
        let branches_count: usize = project
            .visible_worktrees(cx)
            .map(|worktree_handle| worktree_handle.read(cx).snapshot().git_entries().count())
            .sum();
        let branch_caption: String = if only_one_worktree {
            project
                .visible_worktrees(cx)
                .next()
                .unwrap()
                .read(cx)
                .snapshot()
                .root_git_entry()
                .and_then(|entry| entry.branch())
                .map(|branch| branch.to_string())
                .unwrap_or_else(|| "".to_owned())
        } else {
            branches_count.to_string()
        };
        let is_popup_menu_visible = self.popup_menu.read(cx).visible();

        let theme = cx.global::<Settings>().theme.clone();

        Stack::new()
            .with_child(
                MouseEventHandler::<Self, _>::new(0, cx, {
                    let theme = theme.clone();
                    move |state, _cx| {
                        let style = theme
                            .workspace
                            .titlebar
                            .toggle_contacts_button
                            .style_for(state, is_popup_menu_visible);

                        Flex::row()
                            .with_child(
                                Svg::new("icons/version_control_branch_12.svg")
                                    .with_color(style.color)
                                    .constrained()
                                    .with_width(style.icon_width)
                                    .aligned()
                                    // .constrained()
                                    // .with_width(style.button_width)
                                    // .with_height(style.button_width)
                                    // .contained()
                                    // .with_style(style.container)
                                    .into_any_named("version-control-branch-icon"),
                            )
                            .with_child(
                                Label::new(branch_caption, theme.workspace.titlebar.title.clone())
                                    .contained()
                                    .with_style(style.container)
                                    .aligned(),
                            )
                            .constrained()
                            .with_height(style.button_width)
                            .contained()
                            .with_style(style.container)
                    }
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, this, cx| {
                    this.deploy_branches_menu(cx);
                })
                .with_tooltip::<Self>(
                    0,
                    "Branches".into(),
                    None,
                    theme.tooltip.clone(),
                    cx,
                ),
            )
            .with_child(
                ChildView::new(&self.popup_menu, cx)
                    .aligned()
                    .bottom()
                    .left(),
            )
            .into_any_named("branches-button")
    }
}

impl BranchesButton {
    pub fn new(workspace: ViewHandle<Workspace>, cx: &mut ViewContext<Self>) -> Self {
        let parent_id = cx.view_id();
        cx.observe(&workspace, |_, _, cx| cx.notify()).detach();
        Self {
            workspace: workspace.downgrade(),
            popup_menu: cx.add_view(|cx| {
                let mut menu = ContextMenu::new(parent_id, cx);
                menu.set_position_mode(OverlayPositionMode::Local);
                menu
            }),
        }
    }

    pub fn deploy_branches_menu(&mut self, cx: &mut ViewContext<Self>) {
        let mut menu_options = vec![];

        if let Some(workspace) = self.workspace.upgrade(cx) {
            let project = workspace.read(cx).project().read(cx);

            let worktrees_with_branches = project
                .visible_worktrees(cx)
                .map(|worktree_handle| {
                    worktree_handle
                        .read(cx)
                        .snapshot()
                        .git_entries()
                        .filter_map(|entry| {
                            entry.branch().map(|branch| {
                                let repo_name = entry.work_directory();
                                if let Some(name) = repo_name.file_name() {
                                    (name.to_string_lossy().to_string(), branch)
                                } else {
                                    ("WORKTREE ROOT".into(), branch)
                                }
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .flatten();

            let context_menu_items = worktrees_with_branches.map(|(repo_name, branch_name)| {
                let caption = format!("{} / {}", repo_name, branch_name);
                ContextMenuItem::handler(caption.to_owned(), move |_| {
                    println!("{}", caption);
                })
            });
            menu_options.extend(context_menu_items);
        }

        self.popup_menu.update(cx, |menu, cx| {
            menu.show(Default::default(), AnchorCorner::TopLeft, menu_options, cx);
        });
    }
}
