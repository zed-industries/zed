use std::fmt::Display;
use std::sync::Arc;

use gpui::{
    Action, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, KeyContext, ModifiersChangedEvent, MouseButton, ParentElement, Render,
    Styled, Subscription, WeakEntity, Window, actions, rems,
};
use project::git_store::Repository;
use ui::{
    ContextMenu, FluentBuilder, IconButton, IconName, IconSize, PopoverMenu, Tooltip, prelude::*,
};
use gpui::Corner;
use workspace::{ModalView, Workspace, pane};

use crate::branch_picker::{
    self, BranchList, CreateBranch, DeleteBranch, FilterRemotes, LoadMoreBranches,
};
use crate::pull_request_picker::{self, CreatePullRequestInline, PullRequestList};
use crate::stash_picker::{self, DropStashItem, ShowStashItem, StashList};
use crate::worktree_picker::{
    self, WorktreeFromDefault, WorktreeFromDefaultOnWindow, WorktreeList,
};

actions!(
    git_picker,
    [
        ActivateBranchesTab,
        ActivateWorktreesTab,
        ActivateStashTab,
        ActivatePullRequestsTab,
        ActivateGraphTab,
    ]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GitPickerTab {
    Graph,
    Branches,
    Worktrees,
    Stash,
    PullRequests,
}

impl GitPickerTab {
    pub const ALL: &'static [GitPickerTab] = &[
        GitPickerTab::Graph,
        GitPickerTab::Branches,
        GitPickerTab::Worktrees,
        GitPickerTab::Stash,
        GitPickerTab::PullRequests,
    ];
}

impl Display for GitPickerTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            GitPickerTab::Graph => "Graph",
            GitPickerTab::Branches => "Branches",
            GitPickerTab::Worktrees => "Worktrees",
            GitPickerTab::Stash => "Stash",
            GitPickerTab::PullRequests => "PRs",
        };
        write!(f, "{}", label)
    }
}

pub struct GitPicker {
    pub tab: GitPickerTab,
    workspace: WeakEntity<Workspace>,
    repository: Option<Entity<Repository>>,
    width: Rems,
    embedded: bool,
    panel_width: Option<Pixels>,
    branch_list: Option<Entity<BranchList>>,
    worktree_list: Option<Entity<WorktreeList>>,
    stash_list: Option<Entity<StashList>>,
    pull_request_list: Option<Entity<PullRequestList>>,
    _subscriptions: Vec<Subscription>,
}

impl GitPicker {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        repository: Option<Entity<Repository>>,
        initial_tab: GitPickerTab,
        width: Rems,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self {
            tab: initial_tab,
            workspace,
            repository,
            width,
            embedded: false,
            panel_width: None,
            branch_list: None,
            worktree_list: None,
            stash_list: None,
            pull_request_list: None,
            _subscriptions: Vec::new(),
        };

        this.ensure_active_picker(window, cx);
        this
    }

    pub fn set_embedded(&mut self) {
        self.embedded = true;
    }

    pub fn set_panel_width(&mut self, width: Pixels) {
        self.panel_width = Some(width);
    }

    fn ensure_active_picker(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.tab {
            GitPickerTab::Graph => {}
            GitPickerTab::Branches => {
                self.ensure_branch_list(window, cx);
            }
            GitPickerTab::Worktrees => {
                self.ensure_worktree_list(window, cx);
            }
            GitPickerTab::Stash => {
                self.ensure_stash_list(window, cx);
            }
            GitPickerTab::PullRequests => {
                self.ensure_pull_request_list(window, cx);
            }
        }
    }

    fn ensure_branch_list(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<BranchList> {
        if self.branch_list.is_none() {
            let branch_list = cx.new(|cx| {
                branch_picker::create_embedded(
                    self.workspace.clone(),
                    self.repository.clone(),
                    self.width,
                    window,
                    cx,
                )
            });

            let subscription = cx.subscribe(&branch_list, |this, _, _: &DismissEvent, cx| {
                if this.tab == GitPickerTab::Branches {
                    cx.emit(DismissEvent);
                }
            });

            self._subscriptions.push(subscription);
            self.branch_list = Some(branch_list);
        }
        self.branch_list.clone().unwrap()
    }

    fn ensure_worktree_list(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<WorktreeList> {
        if self.worktree_list.is_none() {
            let worktree_list = cx.new(|cx| {
                worktree_picker::create_embedded(
                    self.repository.clone(),
                    self.workspace.clone(),
                    self.width,
                    window,
                    cx,
                )
            });

            let subscription = cx.subscribe(&worktree_list, |this, _, _: &DismissEvent, cx| {
                if this.tab == GitPickerTab::Worktrees {
                    cx.emit(DismissEvent);
                }
            });

            self._subscriptions.push(subscription);
            self.worktree_list = Some(worktree_list);
        }
        self.worktree_list.clone().unwrap()
    }

    fn ensure_stash_list(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<StashList> {
        if self.stash_list.is_none() {
            let stash_list = cx.new(|cx| {
                stash_picker::create_embedded(
                    self.repository.clone(),
                    self.workspace.clone(),
                    self.width,
                    window,
                    cx,
                )
            });

            let subscription = cx.subscribe(&stash_list, |this, _, _: &DismissEvent, cx| {
                if this.tab == GitPickerTab::Stash {
                    cx.emit(DismissEvent);
                }
            });

            self._subscriptions.push(subscription);
            self.stash_list = Some(stash_list);
        }
        self.stash_list.clone().unwrap()
    }

    fn ensure_pull_request_list(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<PullRequestList> {
        if self.pull_request_list.is_none() {
            let pull_request_list = cx.new(|cx| {
                pull_request_picker::create_embedded(
                    self.repository.clone(),
                    self.workspace.clone(),
                    self.width,
                    window,
                    cx,
                )
            });

            let subscription =
                cx.subscribe(&pull_request_list, |this, _, _: &DismissEvent, cx| {
                    if this.tab == GitPickerTab::PullRequests {
                        cx.emit(DismissEvent);
                    }
                });

            self._subscriptions.push(subscription);
            self.pull_request_list = Some(pull_request_list);
        }
        self.pull_request_list.clone().unwrap()
    }

    fn activate_next_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.tab = match self.tab {
            GitPickerTab::Graph => GitPickerTab::Branches,
            GitPickerTab::Branches => GitPickerTab::Worktrees,
            GitPickerTab::Worktrees => GitPickerTab::Stash,
            GitPickerTab::Stash => GitPickerTab::PullRequests,
            GitPickerTab::PullRequests => GitPickerTab::Graph,
        };
        self.ensure_active_picker(window, cx);
        self.focus_active_picker(window, cx);
        cx.notify();
    }

    fn activate_previous_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.tab = match self.tab {
            GitPickerTab::Graph => GitPickerTab::PullRequests,
            GitPickerTab::Branches => GitPickerTab::Graph,
            GitPickerTab::Worktrees => GitPickerTab::Branches,
            GitPickerTab::Stash => GitPickerTab::Worktrees,
            GitPickerTab::PullRequests => GitPickerTab::Stash,
        };
        self.ensure_active_picker(window, cx);
        self.focus_active_picker(window, cx);
        cx.notify();
    }

    fn focus_active_picker(&self, window: &mut Window, cx: &mut App) {
        match self.tab {
            GitPickerTab::Graph => {}
            GitPickerTab::Branches => {
                if let Some(branch_list) = &self.branch_list {
                    branch_list.focus_handle(cx).focus(window, cx);
                }
            }
            GitPickerTab::Worktrees => {
                if let Some(worktree_list) = &self.worktree_list {
                    worktree_list.focus_handle(cx).focus(window, cx);
                }
            }
            GitPickerTab::Stash => {
                if let Some(stash_list) = &self.stash_list {
                    stash_list.focus_handle(cx).focus(window, cx);
                }
            }
            GitPickerTab::PullRequests => {
                if let Some(pull_request_list) = &self.pull_request_list {
                    pull_request_list.focus_handle(cx).focus(window, cx);
                }
            }
        }
    }

    fn render_tab_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_main_branch = self
            .repository
            .as_ref()
            .and_then(|repo| {
                let repo = repo.read(cx);
                let branch = repo.branch.as_ref()?;
                let name = branch.name();
                Some(name == "main" || name == "master")
            })
            .unwrap_or(false);

        let tab = self.tab;

        let make_tab_button =
            |picker_tab: GitPickerTab, selected: bool, cx: &mut Context<Self>| {
                let style = if selected {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Subtle
                };
                Button::new(
                    SharedString::from(format!("tab-{}", picker_tab)),
                    picker_tab.to_string(),
                )
                .size(ButtonSize::Compact)
                .style(style)
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.tab = picker_tab;
                    this.ensure_active_picker(window, cx);
                    this.focus_active_picker(window, cx);
                    cx.notify();
                }))
            };

        // Determine how many tabs fit based on panel width.
        // Each tab is ~70px. Reserve space for action button + padding when present.
        let all_tabs = GitPickerTab::ALL;
        let has_action_button = tab == GitPickerTab::Branches
            || (tab == GitPickerTab::PullRequests && !is_main_branch);
        let max_visible = if let Some(panel_w) = self.panel_width {
            let action_button_width = if has_action_button { px(110.) } else { px(0.) };
            let padding = px(20.);
            let overflow_btn = px(32.);
            let available = panel_w - padding - action_button_width - overflow_btn;
            let per_tab = px(75.);
            let count = (available / per_tab) as usize;
            count.min(all_tabs.len()).max(1)
        } else {
            all_tabs.len()
        };

        // Build the visible tabs list: always include the active tab,
        // then fill remaining slots in order.
        let mut visible_tabs: Vec<GitPickerTab> = Vec::with_capacity(max_visible);
        let mut overflow_tabs: Vec<GitPickerTab> = Vec::new();

        if max_visible >= all_tabs.len() {
            visible_tabs.extend_from_slice(all_tabs);
        } else {
            for &t in all_tabs {
                if visible_tabs.len() < max_visible {
                    visible_tabs.push(t);
                } else {
                    overflow_tabs.push(t);
                }
            }
            // If active tab got pushed to overflow, swap it in
            if !visible_tabs.contains(&tab) {
                if let Some(overflow_pos) = overflow_tabs.iter().position(|&t| t == tab) {
                    let last_visible = visible_tabs.len() - 1;
                    let evicted = visible_tabs[last_visible];
                    visible_tabs[last_visible] = tab;
                    overflow_tabs[overflow_pos] = evicted;
                    // Re-sort overflow to maintain order
                    overflow_tabs.sort_by_key(|t| {
                        all_tabs.iter().position(|at| at == t).unwrap_or(0)
                    });
                }
            }
        };

        let has_overflow = !overflow_tabs.is_empty();
        let overflow_tabs: Arc<[GitPickerTab]> = Arc::from(overflow_tabs.into_boxed_slice());

        let overflow_menu = PopoverMenu::new("git-picker-overflow")
            .trigger(
                IconButton::new("git-picker-overflow-trigger", IconName::Ellipsis)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted)
                    .tooltip(Tooltip::text("More tabs")),
            )
            .menu(move |window, cx| {
                let overflow_tabs = overflow_tabs.clone();
                Some(ContextMenu::build(window, cx, move |menu, _, _| {
                    let mut menu = menu;
                    for &picker_tab in overflow_tabs.iter() {
                        let label = if picker_tab == tab {
                            format!("• {}", picker_tab)
                        } else {
                            picker_tab.to_string()
                        };
                        let action: Box<dyn Action> = match picker_tab {
                            GitPickerTab::Graph => ActivateGraphTab.boxed_clone(),
                            GitPickerTab::Branches => ActivateBranchesTab.boxed_clone(),
                            GitPickerTab::Worktrees => ActivateWorktreesTab.boxed_clone(),
                            GitPickerTab::Stash => ActivateStashTab.boxed_clone(),
                            GitPickerTab::PullRequests => {
                                ActivatePullRequestsTab.boxed_clone()
                            }
                        };
                        menu = menu.action(label, action);
                    }
                    menu
                }))
            })
            .anchor(Corner::TopRight);

        h_flex()
            .px_2()
            .py_1()
            .w_full()
            .justify_between()
            .gap_1()
            .child(
                h_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_0p5()
                    .children(visible_tabs.iter().map(|&picker_tab| {
                        make_tab_button(picker_tab, picker_tab == tab, cx)
                    }))
                    .when(has_overflow, |this| this.child(overflow_menu)),
            )
            .when(tab == GitPickerTab::Branches, |this| {
                this.child(
                    Button::new("tab-action-create-branch", "New Branch")
                        .size(ButtonSize::Compact)
                        .style(ButtonStyle::Filled)
                        .on_click(|_, window, cx| {
                            window
                                .dispatch_action(branch_picker::CreateBranch.boxed_clone(), cx);
                        }),
                )
            })
            .when(
                tab == GitPickerTab::PullRequests && !is_main_branch,
                |this| {
                    this.child(
                        Button::new("tab-action-create-pr", "Create PR")
                            .size(ButtonSize::Compact)
                            .style(ButtonStyle::Filled)
                            .on_click(|_, window, cx| {
                                window.dispatch_action(
                                    CreatePullRequestInline.boxed_clone(),
                                    cx,
                                );
                            }),
                    )
                },
            )
    }

    fn render_active_picker(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        match self.tab {
            GitPickerTab::Graph => gpui::Empty.into_any_element(),
            GitPickerTab::Branches => {
                let branch_list = self.ensure_branch_list(window, cx);
                branch_list.into_any_element()
            }
            GitPickerTab::Worktrees => {
                let worktree_list = self.ensure_worktree_list(window, cx);
                worktree_list.into_any_element()
            }
            GitPickerTab::Stash => {
                let stash_list = self.ensure_stash_list(window, cx);
                stash_list.into_any_element()
            }
            GitPickerTab::PullRequests => {
                let pull_request_list = self.ensure_pull_request_list(window, cx);
                pull_request_list.into_any_element()
            }
        }
    }

    fn handle_modifiers_changed(
        &mut self,
        ev: &ModifiersChangedEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.tab {
            GitPickerTab::Graph => {}
            GitPickerTab::Branches => {
                if let Some(branch_list) = &self.branch_list {
                    branch_list.update(cx, |list, cx| {
                        list.handle_modifiers_changed(ev, window, cx);
                    });
                }
            }
            GitPickerTab::Worktrees => {
                if let Some(worktree_list) = &self.worktree_list {
                    worktree_list.update(cx, |list, cx| {
                        list.handle_modifiers_changed(ev, window, cx);
                    });
                }
            }
            GitPickerTab::Stash => {
                if let Some(stash_list) = &self.stash_list {
                    stash_list.update(cx, |list, cx| {
                        list.handle_modifiers_changed(ev, window, cx);
                    });
                }
            }
            GitPickerTab::PullRequests => {
                if let Some(pull_request_list) = &self.pull_request_list {
                    pull_request_list.update(cx, |list, cx| {
                        list.handle_modifiers_changed(ev, window, cx);
                    });
                }
            }
        }
    }

    fn handle_delete_branch(
        &mut self,
        _: &DeleteBranch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(branch_list) = &self.branch_list {
            branch_list.update(cx, |list, cx| {
                list.handle_delete(&DeleteBranch, window, cx);
            });
        }
    }

    fn handle_create_branch(
        &mut self,
        _: &CreateBranch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(branch_list) = &self.branch_list {
            branch_list.update(cx, |list, cx| {
                list.handle_create_branch(&CreateBranch, window, cx);
            });
        }
    }

    fn handle_load_more_branches(
        &mut self,
        _: &LoadMoreBranches,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(branch_list) = &self.branch_list {
            branch_list.update(cx, |list, cx| {
                list.handle_load_more(&LoadMoreBranches, window, cx);
            });
        }
    }

    fn handle_filter_remotes(
        &mut self,
        _: &FilterRemotes,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(branch_list) = &self.branch_list {
            branch_list.update(cx, |list, cx| {
                list.handle_filter(&FilterRemotes, window, cx);
            });
        }
    }

    fn handle_worktree_from_default(
        &mut self,
        _: &WorktreeFromDefault,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(worktree_list) = &self.worktree_list {
            worktree_list.update(cx, |list, cx| {
                list.handle_new_worktree(false, window, cx);
            });
        }
    }

    fn handle_worktree_from_default_on_window(
        &mut self,
        _: &WorktreeFromDefaultOnWindow,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(worktree_list) = &self.worktree_list {
            worktree_list.update(cx, |list, cx| {
                list.handle_new_worktree(true, window, cx);
            });
        }
    }

    fn handle_create_pr_inline(
        &mut self,
        _: &CreatePullRequestInline,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(pr_list) = &self.pull_request_list {
            pr_list.update(cx, |list, cx| {
                list.handle_create_pr(&CreatePullRequestInline, window, cx);
            });
        }
    }

    fn handle_drop_stash(
        &mut self,
        _: &DropStashItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(stash_list) = &self.stash_list {
            stash_list.update(cx, |list, cx| {
                list.handle_drop_stash(&DropStashItem, window, cx);
            });
        }
    }

    fn handle_show_stash(
        &mut self,
        _: &ShowStashItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(stash_list) = &self.stash_list {
            stash_list.update(cx, |list, cx| {
                list.handle_show_stash(&ShowStashItem, window, cx);
            });
        }
    }
}

impl ModalView for GitPicker {}
impl EventEmitter<DismissEvent> for GitPicker {}

impl Focusable for GitPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match self.tab {
            GitPickerTab::Graph => {}
            GitPickerTab::Branches => {
                if let Some(branch_list) = &self.branch_list {
                    return branch_list.focus_handle(cx);
                }
            }
            GitPickerTab::Worktrees => {
                if let Some(worktree_list) = &self.worktree_list {
                    return worktree_list.focus_handle(cx);
                }
            }
            GitPickerTab::Stash => {
                if let Some(stash_list) = &self.stash_list {
                    return stash_list.focus_handle(cx);
                }
            }
            GitPickerTab::PullRequests => {
                if let Some(pull_request_list) = &self.pull_request_list {
                    return pull_request_list.focus_handle(cx);
                }
            }
        }
        cx.focus_handle()
    }
}

impl Render for GitPicker {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .occlude()
            .when(!self.embedded, |this| this.w(self.width).elevation_3(cx))
            .when(
                self.embedded && self.tab == GitPickerTab::Graph,
                |this| this.w_full().flex_shrink_0(),
            )
            .when(
                self.embedded && self.tab != GitPickerTab::Graph,
                |this| this.size_full(),
            )
            .overflow_hidden()
            .key_context({
                let mut key_context = KeyContext::new_with_defaults();
                key_context.add("Pane");
                key_context.add("GitPicker");
                match self.tab {
                    GitPickerTab::Graph => key_context.add("GitGraph"),
                    GitPickerTab::Branches => key_context.add("GitBranchSelector"),
                    GitPickerTab::Worktrees => key_context.add("GitWorktreeSelector"),
                    GitPickerTab::Stash => key_context.add("StashList"),
                    GitPickerTab::PullRequests => key_context.add("PullRequestList"),
                }
                key_context
            })
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| {
                cx.emit(DismissEvent);
            }))
            .on_action(cx.listener(|this, _: &pane::ActivateNextItem, window, cx| {
                this.activate_next_tab(window, cx);
            }))
            .on_action(
                cx.listener(|this, _: &pane::ActivatePreviousItem, window, cx| {
                    this.activate_previous_tab(window, cx);
                }),
            )
            .on_action(cx.listener(|this, _: &ActivateBranchesTab, window, cx| {
                this.tab = GitPickerTab::Branches;
                this.ensure_active_picker(window, cx);
                this.focus_active_picker(window, cx);
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ActivateWorktreesTab, window, cx| {
                this.tab = GitPickerTab::Worktrees;
                this.ensure_active_picker(window, cx);
                this.focus_active_picker(window, cx);
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ActivateStashTab, window, cx| {
                this.tab = GitPickerTab::Stash;
                this.ensure_active_picker(window, cx);
                this.focus_active_picker(window, cx);
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ActivatePullRequestsTab, window, cx| {
                this.tab = GitPickerTab::PullRequests;
                this.ensure_active_picker(window, cx);
                this.focus_active_picker(window, cx);
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ActivateGraphTab, window, cx| {
                this.tab = GitPickerTab::Graph;
                this.ensure_active_picker(window, cx);
                this.focus_active_picker(window, cx);
                cx.notify();
            }))
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .when(self.tab == GitPickerTab::Branches, |el| {
                el.on_action(cx.listener(Self::handle_delete_branch))
                    .on_action(cx.listener(Self::handle_filter_remotes))
                    .on_action(cx.listener(Self::handle_load_more_branches))
                    .on_action(cx.listener(Self::handle_create_branch))
            })
            .when(self.tab == GitPickerTab::Worktrees, |el| {
                el.on_action(cx.listener(Self::handle_worktree_from_default))
                    .on_action(cx.listener(Self::handle_worktree_from_default_on_window))
            })
            .when(self.tab == GitPickerTab::Stash, |el| {
                el.on_action(cx.listener(Self::handle_drop_stash))
                    .on_action(cx.listener(Self::handle_show_stash))
            })
            .when(self.tab == GitPickerTab::PullRequests, |el| {
                el.on_action(cx.listener(Self::handle_create_pr_inline))
            })
            .child(self.render_tab_bar(cx))
            .child(self.render_active_picker(window, cx))
    }
}

pub fn open_branches(
    workspace: &mut Workspace,
    _: &zed_actions::git::Branch,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    open_with_tab(workspace, GitPickerTab::Branches, window, cx);
}

pub fn open_worktrees(
    workspace: &mut Workspace,
    _: &zed_actions::git::Worktree,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    open_with_tab(workspace, GitPickerTab::Worktrees, window, cx);
}

pub fn open_stash(
    workspace: &mut Workspace,
    _: &zed_actions::git::ViewStash,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    open_with_tab(workspace, GitPickerTab::Stash, window, cx);
}

pub fn open_pull_requests(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    open_with_tab(workspace, GitPickerTab::PullRequests, window, cx);
}

fn open_with_tab(
    workspace: &mut Workspace,
    tab: GitPickerTab,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let workspace_handle = workspace.weak_handle();
    let repository = workspace.project().read(cx).active_repository(cx);

    workspace.toggle_modal(window, cx, |window, cx| {
        GitPicker::new(workspace_handle, repository, tab, rems(34.), window, cx)
    })
}

/// Register all git picker actions with the workspace.
pub fn register(workspace: &mut Workspace) {
    workspace.register_action(|workspace, _: &zed_actions::git::Branch, window, cx| {
        open_with_tab(workspace, GitPickerTab::Branches, window, cx);
    });
    workspace.register_action(|workspace, _: &zed_actions::git::Switch, window, cx| {
        open_with_tab(workspace, GitPickerTab::Branches, window, cx);
    });
    workspace.register_action(
        |workspace, _: &zed_actions::git::CheckoutBranch, window, cx| {
            open_with_tab(workspace, GitPickerTab::Branches, window, cx);
        },
    );
    workspace.register_action(|workspace, _: &zed_actions::git::Worktree, window, cx| {
        open_with_tab(workspace, GitPickerTab::Worktrees, window, cx);
    });
    workspace.register_action(|workspace, _: &zed_actions::git::ViewStash, window, cx| {
        open_with_tab(workspace, GitPickerTab::Stash, window, cx);
    });
    workspace.register_action(
        |workspace, _: &zed_actions::git::ViewPullRequests, window, cx| {
            open_with_tab(workspace, GitPickerTab::PullRequests, window, cx);
        },
    );
}
