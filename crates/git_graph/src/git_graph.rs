mod commit_data;
mod graph;
mod graph_rendering;

use anyhow::Context as _;
use commit_data::{CommitEntry, load_commits, run_git_command};
use git_ui::commit_view::CommitView;
use gpui::{
    Action, App, ClickEvent, ClipboardItem, Context, Corner, DismissEvent, ElementId, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement, ListAlignment,
    ListState, MouseButton, MouseDownEvent, ParentElement, Pixels, Point, Render, SharedString,
    Styled, Subscription, Task, WeakEntity, Window, actions, anchored, deferred, list,
};
use graph_rendering::{
    BRANCH_COLORS, BadgeType, parse_refs_to_badges, render_graph_cell, render_graph_continuation,
};
use project::Project;
use project::git_store::{GitStoreEvent, Repository};
use settings::Settings;
use std::path::PathBuf;
use theme::ThemeSettings;
use ui::prelude::*;
use ui::{ContextMenu, Tooltip};
use ui_input::InputField;
use util::ResultExt;
use workspace::ModalView;
use workspace::Workspace;
use workspace::item::{Item, ItemEvent};

use crate::graph::GraphCommit;

actions!(
    git_graph,
    [
        OpenGitGraph,
        OpenCommitView,
        RefreshGraph,
        CheckoutCommit,
        CopySha,
        CopyBranchName,
        CreateBranch,
        CreateTag,
        RenameBranch,
        DeleteBranch,
        DeleteRemoteBranch,
        RevertCommit,
        CherryPickCommit,
        MergeIntoCurrent,
        PullIntoCurrent,
        RebaseOnto,
        ResetSoft,
        ResetMixed,
        ResetHard,
        PushBranch,
        PullBranch,
        FetchAll,
        StashChanges,
        StashPop,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &OpenGitGraph, window, cx| {
            let project = workspace.project().clone();
            let workspace_handle = workspace.weak_handle();
            let git_graph = cx.new(|cx| GitGraph::new(project, workspace_handle, window, cx));
            workspace.add_item_to_active_pane(Box::new(git_graph), None, true, window, cx);
        });
    })
    .detach();
}

#[derive(Clone)]
pub enum InputModalKind {
    CreateBranch { sha: String },
    CreateTag { sha: String },
    RenameBranch { old_name: String },
    CheckoutRemoteBranch { remote_branch: String },
}

pub struct InputModal {
    focus_handle: FocusHandle,
    input: Entity<InputField>,
    kind: InputModalKind,
    work_dir: PathBuf,
    git_graph: WeakEntity<GitGraph>,
    push_to_remote: bool,
}

impl InputModal {
    pub fn new(
        kind: InputModalKind,
        placeholder: impl Into<SharedString>,
        default_value: &str,
        work_dir: PathBuf,
        git_graph: WeakEntity<GitGraph>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let placeholder_text: SharedString = placeholder.into();
        let default_text = default_value.to_string();
        let input = cx.new(|cx| {
            let field = InputField::new(window, cx, placeholder_text);
            field.set_text(default_text, window, cx);
            field
        });

        Self {
            focus_handle: cx.focus_handle(),
            input,
            kind,
            work_dir,
            git_graph,
            push_to_remote: false,
        }
    }

    fn supports_push(&self) -> bool {
        matches!(
            self.kind,
            InputModalKind::CreateBranch { .. } | InputModalKind::CreateTag { .. }
        )
    }

    fn is_checkout_remote(&self) -> bool {
        matches!(self.kind, InputModalKind::CheckoutRemoteBranch { .. })
    }

    fn toggle_push_to_remote(&mut self, cx: &mut Context<Self>) {
        self.push_to_remote = !self.push_to_remote;
        cx.notify();
    }

    fn title(&self) -> &'static str {
        match &self.kind {
            InputModalKind::CreateBranch { .. } => "Create Branch",
            InputModalKind::CreateTag { .. } => "Create Tag",
            InputModalKind::RenameBranch { .. } => "Rename Branch",
            InputModalKind::CheckoutRemoteBranch { .. } => "Checkout Remote Branch",
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        let input_text = self.input.read(cx).text(cx);
        if input_text.is_empty() {
            return;
        }

        let work_dir = self.work_dir.clone();
        let kind = self.kind.clone();
        let git_graph = self.git_graph.clone();
        let push_to_remote = self.push_to_remote;

        cx.spawn(async move |_, cx| {
            let result = match &kind {
                InputModalKind::CreateBranch { sha } => {
                    run_git_command(&work_dir, &["branch", &input_text, sha]).await
                }
                InputModalKind::CreateTag { sha } => {
                    run_git_command(&work_dir, &["tag", &input_text, sha]).await
                }
                InputModalKind::RenameBranch { old_name } => {
                    run_git_command(&work_dir, &["branch", "-m", old_name, &input_text]).await
                }
                InputModalKind::CheckoutRemoteBranch { remote_branch } => {
                    run_git_command(&work_dir, &["checkout", "-b", &input_text, remote_branch])
                        .await
                }
            };

            let post_result = if result.is_ok() && push_to_remote {
                match &kind {
                    InputModalKind::CreateBranch { .. } => {
                        run_git_command(&work_dir, &["push", "-u", "origin", &input_text]).await
                    }
                    InputModalKind::CreateTag { .. } => {
                        run_git_command(&work_dir, &["push", "origin", &input_text]).await
                    }
                    InputModalKind::CheckoutRemoteBranch { .. } => {
                        run_git_command(&work_dir, &["pull"]).await
                    }
                    InputModalKind::RenameBranch { .. } => Ok(String::new()),
                }
            } else {
                Ok(String::new())
            };

            git_graph
                .update(cx, |this, cx| {
                    match (&result, &post_result) {
                        (Ok(_), Ok(_)) => {
                            this.error = None;
                            this.load_data(cx);
                        }
                        (Err(e), _) => {
                            this.error = Some(format!("Failed: {}", e).into());
                        }
                        (Ok(_), Err(e)) => {
                            this.error =
                                Some(format!("Checkout succeeded but pull failed: {}", e).into());
                            this.load_data(cx);
                        }
                    }
                    cx.notify();
                })
                .log_err();
        })
        .detach();

        cx.emit(DismissEvent);
    }

    fn cancel(&mut self, _: &menu::Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl Focusable for InputModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for InputModal {}

impl ModalView for InputModal {}

impl Render for InputModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.title();
        let supports_push = self.supports_push();
        let is_checkout_remote = self.is_checkout_remote();
        let checkbox_state = if self.push_to_remote {
            ui::ToggleState::Selected
        } else {
            ui::ToggleState::Unselected
        };
        let checkbox_label = if is_checkout_remote {
            "Pull after checkout"
        } else {
            "Push to remote"
        };
        let confirm_label = if is_checkout_remote {
            "Checkout"
        } else {
            "Confirm"
        };

        v_flex()
            .key_context("InputModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .elevation_2(cx)
            .w(px(400.0))
            .overflow_hidden()
            .rounded_lg()
            .border_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().elevated_surface_background)
            .child(
                h_flex()
                    .w_full()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(Label::new(title).size(LabelSize::Small).color(Color::Muted)),
            )
            .child(
                v_flex()
                    .w_full()
                    .p_3()
                    .gap_2()
                    .child(self.input.clone())
                    .when(supports_push || is_checkout_remote, |el| {
                        el.child(
                            h_flex()
                                .gap_2()
                                .items_center()
                                .child(
                                    ui::Checkbox::new("checkbox-option", checkbox_state).on_click(
                                        cx.listener(|this, _, _, cx| {
                                            this.toggle_push_to_remote(cx);
                                        }),
                                    ),
                                )
                                .child(
                                    Label::new(checkbox_label)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                ),
                        )
                    }),
            )
            .child(
                h_flex()
                    .w_full()
                    .px_3()
                    .py_2()
                    .gap_2()
                    .justify_end()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        Button::new("cancel", "Cancel")
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.cancel(&menu::Cancel, window, cx);
                            })),
                    )
                    .child(
                        Button::new("confirm", confirm_label)
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.confirm(&menu::Confirm, window, cx);
                            })),
                    ),
            )
    }
}

pub struct GitGraph {
    focus_handle: FocusHandle,
    graph: crate::graph::GitGraph,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    commits: Vec<CommitEntry>,
    max_lanes: usize,
    loading: bool,
    error: Option<SharedString>,
    _load_task: Option<Task<()>>,
    selected_commit: Option<usize>,
    expanded_commit: Option<usize>,
    expanded_files: Vec<ChangedFile>,
    loading_files: bool,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    work_dir: Option<PathBuf>,
    row_height: Pixels,
    list_state: ListState,
    _subscriptions: Vec<Subscription>,
}

#[derive(Clone, Debug)]
pub struct ChangedFile {
    pub path: String,
    pub status: FileStatus,
}

#[derive(Clone, Debug)]
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
    Unknown,
}

impl GitGraph {
    pub fn new(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, window, |_, _, cx| cx.notify())
            .detach();

        let git_store = project.read(cx).git_store().clone();
        let git_store_subscription = cx.subscribe(&git_store, |this, _, event, cx| match event {
            GitStoreEvent::RepositoryUpdated(_, _, _)
            | GitStoreEvent::RepositoryAdded
            | GitStoreEvent::RepositoryRemoved(_) => {
                this.load_data(cx);
            }
            _ => {}
        });

        let settings = ThemeSettings::get_global(cx);
        let font_size = settings.buffer_font_size(cx);
        let row_height = font_size + px(10.0);

        let list_state = ListState::new(0, ListAlignment::Top, px(500.0));

        let mut this = GitGraph {
            focus_handle,
            project,
            workspace,
            graph: crate::graph::GitGraph::new(),
            commits: Vec::new(),
            max_lanes: 0,
            loading: true,
            error: None,
            _load_task: None,
            selected_commit: None,
            expanded_commit: None,
            expanded_files: Vec::new(),
            loading_files: false,
            context_menu: None,
            work_dir: None,
            row_height,
            list_state,
            _subscriptions: vec![git_store_subscription],
        };

        this.load_data(cx);
        this
    }

    fn get_selected_commit(&self) -> Option<&CommitEntry> {
        self.selected_commit.and_then(|idx| self.commits.get(idx))
    }

    fn get_repository(&self, cx: &App) -> Option<Entity<Repository>> {
        let git_store = self.project.read(cx).git_store();
        git_store.read(cx).repositories().values().next().cloned()
    }

    fn open_commit_view(
        &mut self,
        file_path: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let sha = commit.sha.clone();

        let Some(repository) = self.get_repository(cx) else {
            self.error = Some("No repository found".into());
            cx.notify();
            return;
        };

        let file_filter = file_path.and_then(|p| git::repository::RepoPath::new(&p).ok());

        CommitView::open(
            sha,
            repository.downgrade(),
            self.workspace.clone(),
            None,
            file_filter,
            window,
            cx,
        );
    }

    fn toggle_commit_expansion(&mut self, commit_idx: usize, cx: &mut Context<Self>) {
        let scroll_pos = self.list_state.logical_scroll_top();

        if self.expanded_commit == Some(commit_idx) {
            self.expanded_commit = None;
            self.expanded_files.clear();
            self.list_state.reset(self.commits.len());
            self.list_state.scroll_to(scroll_pos);
            cx.notify();
            return;
        }

        let Some(commit) = self.commits.get(commit_idx) else {
            return;
        };
        let sha = commit.sha.clone();

        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        self.selected_commit = Some(commit_idx);
        self.expanded_commit = Some(commit_idx);
        self.expanded_files.clear();
        self.loading_files = true;
        self.error = None;
        self.list_state.reset(self.commits.len());
        self.list_state.scroll_to(scroll_pos);
        cx.notify();

        cx.spawn(async move |this, cx| {
            let result = run_git_command(
                &work_dir,
                &[
                    "diff-tree",
                    "--root",
                    "--no-commit-id",
                    "--name-status",
                    "-r",
                    &sha,
                ],
            )
            .await;

            this.update(cx, |this, cx| {
                this.loading_files = false;
                match result {
                    Ok(output) => {
                        this.expanded_files = output
                            .lines()
                            .filter_map(|line| {
                                let parts: Vec<&str> = line.split('\t').collect();
                                if parts.len() >= 2 {
                                    let status = match parts[0].chars().next() {
                                        Some('A') => FileStatus::Added,
                                        Some('M') => FileStatus::Modified,
                                        Some('D') => FileStatus::Deleted,
                                        Some('R') => FileStatus::Renamed,
                                        Some('C') => FileStatus::Copied,
                                        _ => FileStatus::Unknown,
                                    };
                                    Some(ChangedFile {
                                        path: parts[1].to_string(),
                                        status,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                    }
                    Err(e) => {
                        this.error = Some(format!("Failed to load files: {}", e).into());
                    }
                }
                cx.notify();
            })
            .log_err();
        })
        .detach();
    }

    fn deploy_context_menu(
        &mut self,
        position: Point<Pixels>,
        commit_idx: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selected_commit = Some(commit_idx);

        if self.commits.get(commit_idx).is_none() {
            return;
        };

        let commit = &self.commits[commit_idx];
        let refs = &commit.refs;

        let is_head = refs.iter().any(|r| r.contains("HEAD"));
        let has_local_branch = refs
            .iter()
            .any(|r| !r.starts_with("origin/") && !r.starts_with("tag:") && !r.contains("HEAD"));
        let has_remote_branch = refs.iter().any(|r| r.starts_with("origin/"));
        let has_any_branch = has_local_branch || has_remote_branch || is_head;

        let focus_handle = self.focus_handle.clone();
        let context_menu = ContextMenu::build(window, cx, |menu, _, _| {
            let mut menu = menu.context(focus_handle);

            menu = menu.action("Refresh", RefreshGraph.boxed_clone());

            if !is_head {
                menu = menu
                    .separator()
                    .action("Checkout", CheckoutCommit.boxed_clone())
                    .action(
                        "Merge into current branch...",
                        MergeIntoCurrent.boxed_clone(),
                    )
                    .action("Rebase onto", RebaseOnto.boxed_clone());

                if has_remote_branch {
                    menu =
                        menu.action("Pull into current branch...", PullIntoCurrent.boxed_clone());
                }

                menu = menu
                    .separator()
                    .action("Cherry-pick", CherryPickCommit.boxed_clone())
                    .action("Revert", RevertCommit.boxed_clone());
            }

            menu = menu
                .separator()
                .action("Create Branch...", CreateBranch.boxed_clone())
                .action("Create Tag...", CreateTag.boxed_clone());

            if has_local_branch || is_head {
                menu = menu.action("Rename Branch...", RenameBranch.boxed_clone());
            }

            if has_local_branch && !is_head {
                menu = menu.action("Delete Local Branch", DeleteBranch.boxed_clone());
            }

            if has_remote_branch {
                menu = menu.action("Delete Remote Branch...", DeleteRemoteBranch.boxed_clone());
            }

            if !is_head {
                menu = menu
                    .separator()
                    .action("Reset (soft)", ResetSoft.boxed_clone())
                    .action("Reset (mixed)", ResetMixed.boxed_clone())
                    .action("Reset (hard)", ResetHard.boxed_clone());
            }

            menu = menu
                .separator()
                .action("Push", PushBranch.boxed_clone())
                .action("Pull", PullBranch.boxed_clone())
                .action("Fetch All", FetchAll.boxed_clone())
                .separator()
                .action("Stash", StashChanges.boxed_clone())
                .action("Stash Pop", StashPop.boxed_clone())
                .separator()
                .action("Copy SHA", CopySha.boxed_clone());

            if has_any_branch {
                menu = menu.action("Copy Branch Name", CopyBranchName.boxed_clone());
            }

            menu
        });

        self.set_context_menu(context_menu, position, window, cx);
    }

    fn set_context_menu(
        &mut self,
        context_menu: Entity<ContextMenu>,
        position: Point<Pixels>,
        window: &Window,
        cx: &mut Context<Self>,
    ) {
        let subscription = cx.subscribe_in(
            &context_menu,
            window,
            |this, _, _: &DismissEvent, window, cx| {
                if this
                    .context_menu
                    .as_ref()
                    .is_some_and(|cm| cm.0.focus_handle(cx).contains_focused(window, cx))
                {
                    cx.focus_self(window);
                }
                this.context_menu.take();
                cx.notify();
            },
        );
        self.context_menu = Some((context_menu, position, subscription));
        cx.notify();
    }

    fn checkout_commit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let sha = commit.sha.clone();
        let refs = commit.refs.clone();
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        let has_local_branch = refs
            .iter()
            .any(|r| !r.starts_with("origin/") && !r.starts_with("tag:") && !r.contains("HEAD"))
            || refs.iter().any(|r| r.starts_with("HEAD -> "));

        let remote_only_branch = if !has_local_branch {
            refs.iter()
                .find(|r| r.starts_with("origin/") && !r.ends_with("/HEAD"))
                .cloned()
        } else {
            None
        };

        if let Some(remote_branch) = remote_only_branch {
            let local_name = remote_branch
                .strip_prefix("origin/")
                .unwrap_or(&remote_branch)
                .to_string();
            let Some(workspace) = self.workspace.upgrade() else {
                return;
            };
            let git_graph = cx.entity().downgrade();

            workspace.update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    InputModal::new(
                        InputModalKind::CheckoutRemoteBranch { remote_branch },
                        "Enter local branch name",
                        &local_name,
                        work_dir,
                        git_graph,
                        window,
                        cx,
                    )
                });
            });
            return;
        }

        let target = refs
            .iter()
            .find(|r| r.starts_with("HEAD -> "))
            .map(|r| r.strip_prefix("HEAD -> ").unwrap_or(r).to_string())
            .or_else(|| {
                refs.iter()
                    .find(|r| {
                        !r.starts_with("origin/") && !r.starts_with("tag:") && !r.contains("HEAD")
                    })
                    .cloned()
            })
            .unwrap_or(sha);

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["checkout", &target]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Checkout failed: {}", e).into());
                    cx.notify();
                }
            })
            .log_err();
        })
        .detach();
    }

    fn checkout_branch(&mut self, branch: String, window: &mut Window, cx: &mut Context<Self>) {
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        let is_remote = branch.starts_with("origin/");

        if is_remote {
            let local_name = branch
                .strip_prefix("origin/")
                .unwrap_or(&branch)
                .to_string();
            let Some(workspace) = self.workspace.upgrade() else {
                return;
            };
            let git_graph = cx.entity().downgrade();

            workspace.update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    InputModal::new(
                        InputModalKind::CheckoutRemoteBranch {
                            remote_branch: branch,
                        },
                        "Enter local branch name",
                        &local_name,
                        work_dir,
                        git_graph,
                        window,
                        cx,
                    )
                });
            });
            return;
        }

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["checkout", &branch]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Checkout failed: {}", e).into());
                    cx.notify();
                }
            })
            .log_err();
        })
        .detach();
    }

    fn copy_sha(&mut self, cx: &mut Context<Self>) {
        let sha = match self.get_selected_commit() {
            Some(commit) => commit.sha.clone(),
            None => return,
        };
        self.error = None;
        cx.write_to_clipboard(ClipboardItem::new_string(sha));
        cx.notify();
    }

    fn create_branch(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let sha = commit.sha.clone();
        let short_sha = commit.short_sha.clone();
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let git_graph = cx.entity().downgrade();
        let default_name = format!("branch-from-{}", short_sha);

        workspace.update(cx, |workspace, cx| {
            workspace.toggle_modal(window, cx, |window, cx| {
                InputModal::new(
                    InputModalKind::CreateBranch { sha },
                    "Enter branch name",
                    &default_name,
                    work_dir,
                    git_graph,
                    window,
                    cx,
                )
            });
        });
    }

    fn cherry_pick_commit(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let sha = commit.sha.clone();
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["cherry-pick", &sha]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Cherry-pick failed: {}", e).into());
                    cx.notify();
                }
            })
            .log_err();
        })
        .detach();
    }

    fn revert_commit(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let sha = commit.sha.clone();
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["revert", "--no-edit", &sha]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Revert failed: {}", e).into());
                    cx.notify();
                }
            })
            .log_err();
        })
        .detach();
    }

    fn reset_soft(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let sha = commit.sha.clone();
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["reset", "--soft", &sha]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Reset failed: {}", e).into());
                    cx.notify();
                }
            })
            .log_err();
        })
        .detach();
    }

    fn reset_hard(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let sha = commit.sha.clone();
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["reset", "--hard", &sha]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Reset failed: {}", e).into());
                    cx.notify();
                }
            })
            .log_err();
        })
        .detach();
    }

    fn reset_mixed(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let sha = commit.sha.clone();
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["reset", "--mixed", &sha]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Reset failed: {}", e).into());
                    cx.notify();
                }
            })
            .log_err();
        })
        .detach();
    }

    fn load_data(&mut self, cx: &mut Context<Self>) {
        let project = self.project.clone();
        // todo!: Is this the best worktree to use?
        let first_visible_worktree = project.read_with(cx, |project, cx| {
            project
                .visible_worktrees(cx)
                .next()
                .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
        });

        self.loading = true;
        self.error = None;

        self._load_task = Some(cx.spawn(async move |this: WeakEntity<Self>, cx| {
            let Some(worktree_path) = first_visible_worktree
                .context("Can't open git graph in Project without visible worktrees")
                .ok()
            else {
                // todo! handle error
                return;
            };

            let result = crate::graph::load_commits(worktree_path.clone()).await;

            this.update(cx, |this, cx| {
                this.loading = false;
                match result {
                    Ok(commits) => {
                        this.graph.add_commits(commits);

                        let commit_count = this.graph.commits.len();
                        this.commits = this.graph.commits.clone();
                        this.max_lanes = this.graph.max_lanes;
                        this.work_dir = Some(worktree_path);
                        this.list_state.reset(commit_count);
                    }
                    Err(e) => {
                        this.error = Some(format!("{:?}", e).into());
                    }
                }
                cx.notify();
            })
            .log_err();
        }));
    }

    fn render_list_item(
        &mut self,
        idx: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::AnyElement {
        let row_height = self.row_height;
        let graph_width = px(16.0) * (self.max_lanes.max(2) as f32) + px(24.0);

        self.render_commit_row(idx, row_height, graph_width, cx)
    }

    fn render_commit_row(
        &self,
        idx: usize,
        row_height: Pixels,
        graph_width: Pixels,
        cx: &Context<Self>,
    ) -> gpui::AnyElement {
        let Some(commit) = self.commits.get(idx) else {
            return div().into_any_element();
        };

        let subject: SharedString = commit.subject.clone().into();
        let author_name: SharedString = commit.author_name.clone().into();
        let short_sha: SharedString = commit.short_sha.clone().into();
        let formatted_time: SharedString = commit.formatted_time.clone().into();
        let refs = commit.refs.clone();
        let lane = commit.lane;
        let lines = commit.lines.clone();
        let color_idx = commit.color_idx;

        let is_selected = self.expanded_commit == Some(idx);
        let bg = if is_selected {
            cx.theme().colors().ghost_element_selected
        } else {
            cx.theme().colors().editor_background
        };
        let hover_bg = cx.theme().colors().ghost_element_hover;

        let date_width = px(140.0);
        let author_width = px(120.0);
        let commit_width = px(80.0);

        h_flex()
            .id(ElementId::NamedInteger("commit-row".into(), idx as u64))
            .w_full()
            .px_2()
            .gap_4()
            .h(row_height)
            .min_h(row_height)
            .flex_shrink_0()
            .bg(bg)
            .hover(move |style| style.bg(hover_bg))
            .on_click(cx.listener(move |this, _event: &ClickEvent, _window, cx| {
                this.selected_commit = Some(idx);
                this.toggle_commit_expansion(idx, cx);
            }))
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                    this.deploy_context_menu(event.position, idx, window, cx);
                }),
            )
            .child(
                div()
                    .w(graph_width)
                    .h_full()
                    .flex_shrink_0()
                    .child(render_graph_cell(
                        lane,
                        lines,
                        color_idx,
                        row_height,
                        graph_width,
                    )),
            )
            .child(
                h_flex()
                    .flex_1()
                    .min_w(px(0.0))
                    .gap_2()
                    .overflow_hidden()
                    .items_center()
                    .when(!refs.is_empty(), |el| {
                        el.child(self.render_badges(&refs, color_idx, idx, cx))
                    })
                    .child(
                        div()
                            .id(ElementId::NamedInteger("commit-subject".into(), idx as u64))
                            .flex_1()
                            .min_w(px(0.0))
                            .overflow_hidden()
                            .tooltip(Tooltip::text(subject.clone()))
                            .child(Label::new(subject).single_line()),
                    ),
            )
            .child(
                div()
                    .w(date_width)
                    .flex_shrink_0()
                    .overflow_hidden()
                    .child(Label::new(formatted_time).color(Color::Muted).single_line()),
            )
            .child(
                div()
                    .w(author_width)
                    .flex_shrink_0()
                    .overflow_hidden()
                    .child(Label::new(author_name).color(Color::Muted).single_line()),
            )
            .child(
                div()
                    .w(commit_width)
                    .flex_shrink_0()
                    .child(Label::new(short_sha).color(Color::Accent).single_line()),
            )
            .into_any_element()
    }

    fn render_badges(
        &self,
        refs: &[String],
        color_idx: usize,
        commit_idx: usize,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let badges = parse_refs_to_badges(refs);
        let branch_color = BRANCH_COLORS[color_idx % BRANCH_COLORS.len()];
        let tag_color = gpui::hsla(140.0 / 360.0, 0.55, 0.45, 1.0);
        let hover_bg = cx.theme().colors().ghost_element_hover;
        let accent_color = cx.theme().colors().border_focused;

        h_flex()
            .gap_1()
            .flex_shrink_0()
            .children(
                badges
                    .into_iter()
                    .take(5)
                    .enumerate()
                    .map(|(badge_idx, badge)| match badge {
                        BadgeType::Tag(name) => h_flex()
                            .gap_0p5()
                            .px_1()
                            .rounded_sm()
                            .child(
                                Icon::new(IconName::Hash)
                                    .size(IconSize::Small)
                                    .color(Color::Custom(tag_color)),
                            )
                            .child(
                                Label::new(name)
                                    .size(LabelSize::Default)
                                    .color(Color::Default),
                            )
                            .into_any_element(),
                        BadgeType::CurrentBranch(name, has_origin) => {
                            let branch_name = name.clone();
                            h_flex()
                                .id(ElementId::NamedInteger(
                                    SharedString::from(format!(
                                        "badge-current-{}-{}",
                                        commit_idx, badge_idx
                                    )),
                                    commit_idx as u64,
                                ))
                                .gap_0p5()
                                .px_1()
                                .rounded_sm()
                                .border_1()
                                .border_color(accent_color)
                                .cursor_pointer()
                                .hover(move |style| style.bg(hover_bg))
                                .on_click(cx.listener(
                                    move |this, event: &ClickEvent, window, cx| {
                                        cx.stop_propagation();
                                        if event.click_count() == 2 {
                                            this.checkout_branch(branch_name.clone(), window, cx);
                                        }
                                    },
                                ))
                                .child(
                                    Icon::new(IconName::GitBranch)
                                        .size(IconSize::Small)
                                        .color(Color::Custom(branch_color)),
                                )
                                .child(
                                    Label::new(name)
                                        .size(LabelSize::Default)
                                        .color(Color::Default),
                                )
                                .when(has_origin, |el| {
                                    el.child(
                                        Label::new("origin")
                                            .size(LabelSize::Default)
                                            .color(Color::Muted),
                                    )
                                })
                                .into_any_element()
                        }
                        BadgeType::LocalBranch(name, has_origin) => {
                            let branch_name = name.clone();
                            h_flex()
                                .id(ElementId::NamedInteger(
                                    SharedString::from(format!(
                                        "badge-local-{}-{}",
                                        commit_idx, badge_idx
                                    )),
                                    commit_idx as u64,
                                ))
                                .gap_0p5()
                                .px_1()
                                .rounded_sm()
                                .cursor_pointer()
                                .hover(move |style| style.bg(hover_bg))
                                .on_click(cx.listener(
                                    move |this, event: &ClickEvent, window, cx| {
                                        cx.stop_propagation();
                                        if event.click_count() == 2 {
                                            this.checkout_branch(branch_name.clone(), window, cx);
                                        }
                                    },
                                ))
                                .child(
                                    Icon::new(IconName::GitBranch)
                                        .size(IconSize::Small)
                                        .color(Color::Custom(branch_color)),
                                )
                                .child(
                                    Label::new(name)
                                        .size(LabelSize::Default)
                                        .color(Color::Default),
                                )
                                .when(has_origin, |el| {
                                    el.child(
                                        Label::new("origin")
                                            .size(LabelSize::Default)
                                            .color(Color::Muted),
                                    )
                                })
                                .into_any_element()
                        }
                        BadgeType::RemoteBranch(name) => {
                            let branch_name = name.clone();
                            h_flex()
                                .id(ElementId::NamedInteger(
                                    SharedString::from(format!(
                                        "badge-remote-{}-{}",
                                        commit_idx, badge_idx
                                    )),
                                    commit_idx as u64,
                                ))
                                .gap_0p5()
                                .px_1()
                                .rounded_sm()
                                .cursor_pointer()
                                .hover(move |style| style.bg(hover_bg))
                                .on_click(cx.listener(
                                    move |this, event: &ClickEvent, window, cx| {
                                        cx.stop_propagation();
                                        if event.click_count() == 2 {
                                            this.checkout_branch(branch_name.clone(), window, cx);
                                        }
                                    },
                                ))
                                .child(
                                    Icon::new(IconName::GitBranch)
                                        .size(IconSize::Small)
                                        .color(Color::Custom(branch_color)),
                                )
                                .child(
                                    Label::new(name)
                                        .size(LabelSize::Default)
                                        .color(Color::Muted),
                                )
                                .into_any_element()
                        }
                    }),
            )
    }
}

impl Render for GitGraph {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let graph_width = px(16.0) * (self.max_lanes.max(2) as f32) + px(24.0);
        let date_width = px(140.0);
        let author_width = px(120.0);
        let commit_width = px(80.0);

        let error_banner = self.error.as_ref().map(|error| {
            h_flex()
                .id("error-banner")
                .w_full()
                .px_2()
                .py_1()
                .bg(cx.theme().colors().surface_background)
                .border_b_1()
                .border_color(cx.theme().colors().border)
                .justify_between()
                .items_center()
                .child(
                    h_flex()
                        .gap_2()
                        .overflow_hidden()
                        .child(Icon::new(IconName::Warning).color(Color::Error))
                        .child(Label::new(error.clone()).color(Color::Error).single_line()),
                )
                .child(
                    IconButton::new("dismiss-error", IconName::Close)
                        .icon_size(IconSize::Small)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.error = None;
                            cx.notify();
                        })),
                )
        });

        let content = if self.loading {
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(Label::new("Loading commits...").color(Color::Muted))
        } else if self.commits.is_empty() {
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(Label::new("No commits found").color(Color::Muted))
        } else {
            div()
                .size_full()
                .flex()
                .flex_col()
                .child(
                    h_flex()
                        .w_full()
                        .px_2()
                        .py_1()
                        .gap_4()
                        .border_b_1()
                        .border_color(cx.theme().colors().border)
                        .flex_shrink_0()
                        .child(
                            div()
                                .w(graph_width)
                                .child(Label::new("Graph").color(Color::Muted)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .child(Label::new("Description").color(Color::Muted)),
                        )
                        .child(
                            div()
                                .w(date_width)
                                .child(Label::new("Date").color(Color::Muted)),
                        )
                        .child(
                            div()
                                .w(author_width)
                                .child(Label::new("Author").color(Color::Muted)),
                        )
                        .child(
                            div()
                                .w(commit_width)
                                .child(Label::new("Commit").color(Color::Muted)),
                        ),
                )
                .child(
                    list(
                        self.list_state.clone(),
                        cx.processor(Self::render_list_item),
                    )
                    .flex_1()
                    .w_full(),
                )
        };

        div()
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .key_context("GitGraph")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &RefreshGraph, _, cx| this.load_data(cx)))
            .on_action(cx.listener(|this, _: &CopySha, _, cx| this.copy_sha(cx)))
            .on_action(
                cx.listener(|this, _: &CheckoutCommit, window, cx| {
                    this.checkout_commit(window, cx)
                }),
            )
            .on_action(
                cx.listener(|this, _: &CreateBranch, window, cx| this.create_branch(window, cx)),
            )
            .on_action(cx.listener(|this, _: &CherryPickCommit, _, cx| this.cherry_pick_commit(cx)))
            .on_action(cx.listener(|this, _: &RevertCommit, _, cx| this.revert_commit(cx)))
            .on_action(cx.listener(|this, _: &ResetSoft, _, cx| this.reset_soft(cx)))
            .on_action(cx.listener(|this, _: &ResetMixed, _, cx| this.reset_mixed(cx)))
            .on_action(cx.listener(|this, _: &ResetHard, _, cx| this.reset_hard(cx)))
            .child(v_flex().size_full().children(error_banner).child(content))
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(Corner::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
    }
}

impl EventEmitter<ItemEvent> for GitGraph {}

impl Focusable for GitGraph {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for GitGraph {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Git Graph".into()
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        f(*event)
    }
}
