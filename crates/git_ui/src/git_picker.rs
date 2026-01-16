use std::fmt::Display;

use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement,
    KeyContext, ModifiersChangedEvent, MouseButton, ParentElement, Render, Styled, Subscription,
    WeakEntity, Window, actions, rems,
};
use project::git_store::Repository;
use ui::{
    FluentBuilder, ToggleButtonGroup, ToggleButtonGroupStyle, ToggleButtonSimple, Tooltip,
    prelude::*,
};
use workspace::{ModalView, Workspace, pane};

use crate::branch_picker::{self, BranchList, DeleteBranch, FilterRemotes};
use crate::stash_picker::{self, DropStashItem, ShowStashItem, StashList};
use crate::worktree_picker::{
    self, WorktreeFromDefault, WorktreeFromDefaultOnWindow, WorktreeList,
};

actions!(
    git_picker,
    [ActivateBranchesTab, ActivateWorktreesTab, ActivateStashTab,]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GitPickerTab {
    Branches,
    Worktrees,
    Stash,
}

impl Display for GitPickerTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            GitPickerTab::Branches => "Branches",
            GitPickerTab::Worktrees => "Worktrees",
            GitPickerTab::Stash => "Stash",
        };
        write!(f, "{}", label)
    }
}

pub struct GitPicker {
    tab: GitPickerTab,
    workspace: WeakEntity<Workspace>,
    repository: Option<Entity<Repository>>,
    width: Rems,
    branch_list: Option<Entity<BranchList>>,
    worktree_list: Option<Entity<WorktreeList>>,
    stash_list: Option<Entity<StashList>>,
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
            branch_list: None,
            worktree_list: None,
            stash_list: None,
            _subscriptions: Vec::new(),
        };

        this.ensure_active_picker(window, cx);
        this
    }

    fn ensure_active_picker(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.tab {
            GitPickerTab::Branches => {
                self.ensure_branch_list(window, cx);
            }
            GitPickerTab::Worktrees => {
                self.ensure_worktree_list(window, cx);
            }
            GitPickerTab::Stash => {
                self.ensure_stash_list(window, cx);
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

    fn activate_next_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.tab = match self.tab {
            GitPickerTab::Branches => GitPickerTab::Worktrees,
            GitPickerTab::Worktrees => GitPickerTab::Stash,
            GitPickerTab::Stash => GitPickerTab::Branches,
        };
        self.ensure_active_picker(window, cx);
        self.focus_active_picker(window, cx);
        cx.notify();
    }

    fn activate_previous_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.tab = match self.tab {
            GitPickerTab::Branches => GitPickerTab::Stash,
            GitPickerTab::Worktrees => GitPickerTab::Branches,
            GitPickerTab::Stash => GitPickerTab::Worktrees,
        };
        self.ensure_active_picker(window, cx);
        self.focus_active_picker(window, cx);
        cx.notify();
    }

    fn focus_active_picker(&self, window: &mut Window, cx: &mut App) {
        match self.tab {
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
        }
    }

    fn render_tab_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);
        let branches_focus_handle = focus_handle.clone();
        let worktrees_focus_handle = focus_handle.clone();
        let stash_focus_handle = focus_handle;

        h_flex().p_2().pb_0p5().w_full().child(
            ToggleButtonGroup::single_row(
                "git-picker-tabs",
                [
                    ToggleButtonSimple::new(
                        GitPickerTab::Branches.to_string(),
                        cx.listener(|this, _, window, cx| {
                            this.tab = GitPickerTab::Branches;
                            this.ensure_active_picker(window, cx);
                            this.focus_active_picker(window, cx);
                            cx.notify();
                        }),
                    )
                    .tooltip(move |_, cx| {
                        Tooltip::for_action_in(
                            "Toggle Branch Picker",
                            &ActivateBranchesTab,
                            &branches_focus_handle,
                            cx,
                        )
                    }),
                    ToggleButtonSimple::new(
                        GitPickerTab::Worktrees.to_string(),
                        cx.listener(|this, _, window, cx| {
                            this.tab = GitPickerTab::Worktrees;
                            this.ensure_active_picker(window, cx);
                            this.focus_active_picker(window, cx);
                            cx.notify();
                        }),
                    )
                    .tooltip(move |_, cx| {
                        Tooltip::for_action_in(
                            "Toggle Worktree Picker",
                            &ActivateWorktreesTab,
                            &worktrees_focus_handle,
                            cx,
                        )
                    }),
                    ToggleButtonSimple::new(
                        GitPickerTab::Stash.to_string(),
                        cx.listener(|this, _, window, cx| {
                            this.tab = GitPickerTab::Stash;
                            this.ensure_active_picker(window, cx);
                            this.focus_active_picker(window, cx);
                            cx.notify();
                        }),
                    )
                    .tooltip(move |_, cx| {
                        Tooltip::for_action_in(
                            "Toggle Stash Picker",
                            &ActivateStashTab,
                            &stash_focus_handle,
                            cx,
                        )
                    }),
                ],
            )
            .label_size(LabelSize::Default)
            .style(ToggleButtonGroupStyle::Outlined)
            .auto_width()
            .selected_index(match self.tab {
                GitPickerTab::Branches => 0,
                GitPickerTab::Worktrees => 1,
                GitPickerTab::Stash => 2,
            }),
        )
    }

    fn render_active_picker(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        match self.tab {
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
        }
    }

    fn handle_modifiers_changed(
        &mut self,
        ev: &ModifiersChangedEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.tab {
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
        }
        cx.focus_handle()
    }
}

impl Render for GitPicker {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .occlude()
            .w(self.width)
            .elevation_3(cx)
            .overflow_hidden()
            .key_context({
                let mut key_context = KeyContext::new_with_defaults();
                key_context.add("Pane");
                key_context.add("GitPicker");
                match self.tab {
                    GitPickerTab::Branches => key_context.add("GitBranchSelector"),
                    GitPickerTab::Worktrees => key_context.add("GitWorktreeSelector"),
                    GitPickerTab::Stash => key_context.add("StashList"),
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
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .when(self.tab == GitPickerTab::Branches, |el| {
                el.on_action(cx.listener(Self::handle_delete_branch))
                    .on_action(cx.listener(Self::handle_filter_remotes))
            })
            .when(self.tab == GitPickerTab::Worktrees, |el| {
                el.on_action(cx.listener(Self::handle_worktree_from_default))
                    .on_action(cx.listener(Self::handle_worktree_from_default_on_window))
            })
            .when(self.tab == GitPickerTab::Stash, |el| {
                el.on_action(cx.listener(Self::handle_drop_stash))
                    .on_action(cx.listener(Self::handle_show_stash))
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

fn open_with_tab(
    workspace: &mut Workspace,
    tab: GitPickerTab,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let workspace_handle = workspace.weak_handle();
    let project = workspace.project().clone();

    // Check if there's a worktree override from the project dropdown.
    // This ensures the git picker shows info for the project the user
    // explicitly selected in the title bar, not just the focused file's project.
    // This is only relevant if for multi-projects workspaces.
    let repository = workspace
        .active_worktree_override()
        .and_then(|override_id| {
            let project_ref = project.read(cx);
            project_ref
                .worktree_for_id(override_id, cx)
                .and_then(|worktree| {
                    let worktree_abs_path = worktree.read(cx).abs_path();
                    let git_store = project_ref.git_store().read(cx);
                    git_store
                        .repositories()
                        .values()
                        .find(|repo| {
                            let repo_path = &repo.read(cx).work_directory_abs_path;
                            *repo_path == worktree_abs_path
                                || worktree_abs_path.starts_with(repo_path.as_ref())
                        })
                        .cloned()
                })
        })
        .or_else(|| project.read(cx).active_repository(cx));

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
}
