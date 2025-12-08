mod commit_data;
mod graph_rendering;

use git;
use git_ui::commit_view::CommitView;
use gpui::{
    Action, App, ClickEvent, ClipboardItem, Context, Corner, DismissEvent, ElementId, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, ListAlignment, ListState,
    MouseButton, MouseDownEvent, ParentElement, Pixels, Point, Render, SharedString, Styled,
    Subscription, Task, WeakEntity, Window, actions, anchored, deferred, list, px,
};
use project::Project;
use project::git_store::{GitStoreEvent, Repository};
use settings::Settings;
use std::path::PathBuf;
use theme::ThemeSettings;
use ui::prelude::*;
use ui::{ContextMenu, Tooltip};
use workspace::Workspace;
use workspace::item::{Item, ItemEvent};

use commit_data::{CommitEntry, load_commits, run_git_command};
use graph_rendering::{format_timestamp, render_graph_cell, render_graph_continuation, render_ref_badges};

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
    cx.observe_new(|workspace: &mut workspace::Workspace, _, _| {
        workspace.register_action(|workspace, _: &OpenGitGraph, window, cx| {
            let project = workspace.project().clone();
            let workspace_handle = workspace.weak_handle();
            let git_graph = cx.new(|cx| GitGraph::new(project, workspace_handle, window, cx));
            workspace.add_item_to_active_pane(Box::new(git_graph), None, true, window, cx);
        });
    })
    .detach();
}

pub struct GitGraph {
    focus_handle: FocusHandle,
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

    fn open_commit_view(&mut self, file_path: Option<String>, window: &mut Window, cx: &mut Context<Self>) {
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
        if self.expanded_commit == Some(commit_idx) {
            self.expanded_commit = None;
            self.expanded_files.clear();
            self.list_state.reset(self.commits.len());
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
        cx.notify();

        cx.spawn(async move |this, cx| {
            let result = run_git_command(
                &work_dir,
                &["diff-tree", "--root", "--no-commit-id", "--name-status", "-r", &sha],
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
            .ok();
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

    fn checkout_commit(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let sha = commit.sha.clone();
        let refs = commit.refs.clone();
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

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
            .ok();
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

    fn create_branch(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let sha = commit.sha.clone();
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        let branch_name = format!("branch-from-{}", &commit.short_sha);

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["checkout", "-b", &branch_name, &sha]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Create branch failed: {}", e).into());
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
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
            .ok();
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
            .ok();
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
            .ok();
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
            .ok();
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
            .ok();
        })
        .detach();
    }

    fn create_tag(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let sha = commit.sha.clone();
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        let tag_name = format!("tag-{}", &commit.short_sha);

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["tag", &tag_name, &sha]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Create tag failed: {}", e).into());
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    fn merge_into_current(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let sha = commit.sha.clone();
        let refs = commit.refs.clone();
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        let target = refs
            .iter()
            .find(|r| !r.starts_with("tag:") && !r.contains("HEAD"))
            .cloned()
            .unwrap_or(sha);

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["merge", &target]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Merge failed: {}", e).into());
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    fn pull_into_current(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let refs = commit.refs.clone();
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        let remote_branch = refs.iter().find(|r| r.starts_with("origin/")).cloned();

        let Some(remote_ref) = remote_branch else {
            self.error = Some("No remote branch found for this commit".into());
            cx.notify();
            return;
        };

        cx.spawn(async move |this, cx| {
            let result = run_git_command(
                &work_dir,
                &["pull", "origin", remote_ref.trim_start_matches("origin/")],
            )
            .await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Pull failed: {}", e).into());
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    fn copy_branch_name(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };

        let branch_name = commit
            .refs
            .iter()
            .find(|r| r.starts_with("HEAD -> "))
            .map(|r| r.strip_prefix("HEAD -> ").unwrap_or(r).to_string())
            .or_else(|| {
                commit
                    .refs
                    .iter()
                    .find(|r| {
                        !r.starts_with("origin/") && !r.starts_with("tag:") && !r.contains("HEAD")
                    })
                    .cloned()
            })
            .or_else(|| {
                commit
                    .refs
                    .iter()
                    .find(|r| r.starts_with("origin/"))
                    .cloned()
            });

        match branch_name {
            Some(name) => {
                self.error = None;
                cx.write_to_clipboard(ClipboardItem::new_string(name));
            }
            None => {
                self.error = Some("No branch found for this commit".into());
            }
        }
        cx.notify();
    }

    fn rename_branch(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        let branch_name = commit
            .refs
            .iter()
            .find(|r| r.starts_with("HEAD -> "))
            .map(|r| r.strip_prefix("HEAD -> ").unwrap_or(r).to_string())
            .or_else(|| {
                commit
                    .refs
                    .iter()
                    .find(|r| {
                        !r.starts_with("origin/") && !r.starts_with("tag:") && !r.contains("HEAD")
                    })
                    .cloned()
            });

        let Some(old_name) = branch_name else {
            self.error = Some("No local branch found to rename".into());
            cx.notify();
            return;
        };

        let new_name = format!("{}-renamed", old_name);

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["branch", "-m", &old_name, &new_name]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Rename failed: {}", e).into());
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    fn delete_branch(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        let branch_name = commit
            .refs
            .iter()
            .find(|r| !r.starts_with("origin/") && !r.starts_with("tag:") && !r.contains("HEAD"))
            .cloned();

        let Some(branch) = branch_name else {
            self.error = Some("No local branch found to delete".into());
            cx.notify();
            return;
        };

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["branch", "-d", &branch]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Delete branch failed: {}", e).into());
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    fn delete_remote_branch(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        let remote_branch = commit
            .refs
            .iter()
            .find(|r| r.starts_with("origin/"))
            .cloned();

        let Some(remote_ref) = remote_branch else {
            self.error = Some("No remote branch found to delete".into());
            cx.notify();
            return;
        };

        let branch = remote_ref.trim_start_matches("origin/").to_string();

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["push", "origin", "--delete", &branch]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Delete remote branch failed: {}", e).into());
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    fn rebase_onto(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let sha = commit.sha.clone();
        let refs = commit.refs.clone();
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        let target = refs
            .iter()
            .find(|r| !r.starts_with("origin/") && !r.starts_with("tag:") && !r.contains("HEAD"))
            .cloned()
            .unwrap_or(sha);

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["rebase", &target]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Rebase failed: {}", e).into());
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    fn push_branch(&mut self, cx: &mut Context<Self>) {
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["push"]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Push failed: {}", e).into());
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    fn pull_branch(&mut self, cx: &mut Context<Self>) {
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["pull"]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Pull failed: {}", e).into());
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    fn fetch_all(&mut self, cx: &mut Context<Self>) {
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["fetch", "--all"]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Fetch failed: {}", e).into());
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    fn stash_changes(&mut self, cx: &mut Context<Self>) {
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        cx.spawn(async move |this, cx| {
            let result = run_git_command(
                &work_dir,
                &["stash", "push", "-m", "Stashed from Git Graph"],
            )
            .await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Stash failed: {}", e).into());
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    fn stash_pop(&mut self, cx: &mut Context<Self>) {
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };

        cx.spawn(async move |this, cx| {
            let result = run_git_command(&work_dir, &["stash", "pop"]).await;

            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    this.error = None;
                    this.load_data(cx);
                }
                Err(e) => {
                    this.error = Some(format!("Stash pop failed: {}", e).into());
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    fn load_data(&mut self, cx: &mut Context<Self>) {
        let project = self.project.clone();
        self.loading = true;
        self.error = None;

        self._load_task = Some(cx.spawn(async move |this: WeakEntity<Self>, mut cx| {
            let result = load_commits(project, &mut cx).await;

            this.update(cx, |this, cx| {
                this.loading = false;
                match result {
                    Ok((commits, max_lanes, work_dir)) => {
                        let commit_count = commits.len();
                        this.commits = commits;
                        this.max_lanes = max_lanes;
                        this.work_dir = Some(work_dir);
                        this.list_state.reset(commit_count);
                    }
                    Err(e) => {
                        this.error = Some(format!("{:?}", e).into());
                    }
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn handle_checkout_commit(
        &mut self,
        _: &CheckoutCommit,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.checkout_commit(cx);
    }

    fn handle_open_commit_view(
        &mut self,
        _: &OpenCommitView,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_commit_view(None, window, cx);
    }

    fn handle_copy_sha(&mut self, _: &CopySha, _: &mut Window, cx: &mut Context<Self>) {
        self.copy_sha(cx);
    }

    fn handle_create_branch(&mut self, _: &CreateBranch, _: &mut Window, cx: &mut Context<Self>) {
        self.create_branch(cx);
    }

    fn handle_cherry_pick_commit(
        &mut self,
        _: &CherryPickCommit,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.cherry_pick_commit(cx);
    }

    fn handle_revert_commit(&mut self, _: &RevertCommit, _: &mut Window, cx: &mut Context<Self>) {
        self.revert_commit(cx);
    }

    fn handle_reset_soft(&mut self, _: &ResetSoft, _: &mut Window, cx: &mut Context<Self>) {
        self.reset_soft(cx);
    }

    fn handle_reset_hard(&mut self, _: &ResetHard, _: &mut Window, cx: &mut Context<Self>) {
        self.reset_hard(cx);
    }

    fn handle_refresh_graph(&mut self, _: &RefreshGraph, _: &mut Window, cx: &mut Context<Self>) {
        self.load_data(cx);
    }

    fn handle_reset_mixed(&mut self, _: &ResetMixed, _: &mut Window, cx: &mut Context<Self>) {
        self.reset_mixed(cx);
    }

    fn handle_create_tag(&mut self, _: &CreateTag, _: &mut Window, cx: &mut Context<Self>) {
        self.create_tag(cx);
    }

    fn handle_merge_into_current(
        &mut self,
        _: &MergeIntoCurrent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.merge_into_current(cx);
    }

    fn handle_pull_into_current(
        &mut self,
        _: &PullIntoCurrent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.pull_into_current(cx);
    }

    fn handle_copy_branch_name(
        &mut self,
        _: &CopyBranchName,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.copy_branch_name(cx);
    }

    fn handle_rename_branch(&mut self, _: &RenameBranch, _: &mut Window, cx: &mut Context<Self>) {
        self.rename_branch(cx);
    }

    fn handle_delete_branch(&mut self, _: &DeleteBranch, _: &mut Window, cx: &mut Context<Self>) {
        self.delete_branch(cx);
    }

    fn handle_delete_remote_branch(
        &mut self,
        _: &DeleteRemoteBranch,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.delete_remote_branch(cx);
    }

    fn handle_rebase_onto(&mut self, _: &RebaseOnto, _: &mut Window, cx: &mut Context<Self>) {
        self.rebase_onto(cx);
    }

    fn handle_push_branch(&mut self, _: &PushBranch, _: &mut Window, cx: &mut Context<Self>) {
        self.push_branch(cx);
    }

    fn handle_pull_branch(&mut self, _: &PullBranch, _: &mut Window, cx: &mut Context<Self>) {
        self.pull_branch(cx);
    }

    fn handle_fetch_all(&mut self, _: &FetchAll, _: &mut Window, cx: &mut Context<Self>) {
        self.fetch_all(cx);
    }

    fn handle_stash_changes(&mut self, _: &StashChanges, _: &mut Window, cx: &mut Context<Self>) {
        self.stash_changes(cx);
    }

    fn handle_stash_pop(&mut self, _: &StashPop, _: &mut Window, cx: &mut Context<Self>) {
        self.stash_pop(cx);
    }

    fn render_list_item(
        &mut self,
        idx: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::AnyElement {
        let row_height = self.row_height;
        let graph_width = px(16.0) * (self.max_lanes.max(2) as f32) + px(24.0);
        let date_width = px(140.0);
        let author_width = px(120.0);
        let commit_width = px(80.0);

        self.render_commit_row_inline(idx, row_height, graph_width, date_width, author_width, commit_width, cx)
    }

    fn render_commit_row_inline(
        &self,
        idx: usize,
        row_height: Pixels,
        graph_width: Pixels,
        date_width: Pixels,
        author_width: Pixels,
        commit_width: Pixels,
        cx: &Context<Self>,
    ) -> gpui::AnyElement {
        let is_expanded = self.expanded_commit == Some(idx);
        let row = self.render_commit_row(idx, row_height, graph_width, date_width, author_width, commit_width, cx);

        if is_expanded {
            v_flex()
                .w_full()
                .child(row)
                .child(self.render_inline_expansion(idx, graph_width, cx))
                .into_any_element()
        } else {
            row
        }
    }

    fn render_commit_row(
        &self,
        idx: usize,
        row_height: Pixels,
        graph_width: Pixels,
        date_width: Pixels,
        author_width: Pixels,
        commit_width: Pixels,
        cx: &Context<Self>,
    ) -> gpui::AnyElement {
        let Some(commit) = self.commits.get(idx) else {
            return div().into_any_element();
        };

        let subject: SharedString = commit.subject.clone().into();
        let author_name: SharedString = commit.author_name.clone().into();
        let short_sha: SharedString = commit.short_sha.clone().into();
        let timestamp = commit.timestamp;
        let refs = commit.refs.clone();
        let lane = commit.lane;
        let lines = commit.lines.clone();
        let color_idx = commit.color_idx;

        let is_selected = self.expanded_commit == Some(idx);
        let bg = if is_selected {
            cx.theme().colors().ghost_element_selected
        } else if idx % 2 == 0 {
            cx.theme().colors().surface_background
        } else {
            cx.theme().colors().background
        };
        let hover_bg = cx.theme().colors().ghost_element_hover;

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
            .on_click(cx.listener(move |this, event: &ClickEvent, _window, cx| {
                this.selected_commit = Some(idx);
                if event.click_count() == 2 {
                    this.checkout_commit(cx);
                } else {
                    this.toggle_commit_expansion(idx, cx);
                }
            }))
            .on_mouse_down(MouseButton::Right, cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                this.deploy_context_menu(event.position, idx, window, cx);
            }))
            .child(
                div()
                    .w(graph_width)
                    .h_full()
                    .flex_shrink_0()
                    .child(render_graph_cell(lane, lines, color_idx, row_height, graph_width))
            )
            .child(
                h_flex()
                    .flex_1()
                    .min_w(px(0.0))
                    .gap_2()
                    .overflow_hidden()
                    .items_center()
                    .when(!refs.is_empty(), |el| {
                        el.child(render_ref_badges(&refs))
                    })
                    .child(
                        div()
                            .id(ElementId::NamedInteger("commit-subject".into(), idx as u64))
                            .flex_1()
                            .min_w(px(0.0))
                            .overflow_hidden()
                            .tooltip(Tooltip::text(subject.clone()))
                            .child(Label::new(subject).single_line())
                    ),
            )
            .child(
                div()
                    .w(date_width)
                    .flex_shrink_0()
                    .overflow_hidden()
                    .child(Label::new(format_timestamp(timestamp)).color(Color::Muted).single_line()),
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

    fn render_inline_expansion(
        &self,
        idx: usize,
        graph_width: Pixels,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let Some(commit) = self.commits.get(idx) else {
            return div().into_any_element();
        };

        let commit_sha = commit.sha.clone();
        let parents = commit.parents.clone();
        let author = commit.author_name.clone();
        let subject = commit.subject.clone();
        let timestamp = commit.timestamp;
        let lines = commit.lines.clone();
        let loading_files = self.loading_files;
        let expanded_files = &self.expanded_files;

        h_flex()
            .id(ElementId::NamedInteger("expanded-details".into(), idx as u64))
            .w_full()
            .min_h(px(120.0))
            .px_2()
            .gap_4()
            .bg(cx.theme().colors().background)
            .flex_shrink_0()
            .child(
                div()
                    .w(graph_width)
                    .h_full()
                    .flex_shrink_0()
                    .child(render_graph_continuation(lines, graph_width))
            )
            .child(
                h_flex()
                    .flex_1()
                    .h_full()
                    .child(
                        v_flex()
                            .w(px(400.0))
                            .p_2()
                            .gap_1()
                            .border_r_1()
                            .border_color(cx.theme().colors().border)
                            .child(
                                h_flex()
                                    .w_full()
                                    .pb_1()
                                    .mb_1()
                                    .border_b_1()
                                    .border_color(cx.theme().colors().border)
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(Icon::new(IconName::Info).color(Color::Muted).size(IconSize::Small))
                                            .child(Label::new("Info").size(LabelSize::Small).color(Color::Muted))
                                    )
                            )
                            .child(
                                h_flex().gap_2()
                                    .child(Label::new("Commit:").size(LabelSize::Small).color(Color::Muted))
                                    .child(Label::new(commit_sha).size(LabelSize::Small).color(Color::Accent))
                            )
                            .when(!parents.is_empty(), |el| {
                                let parent_str = parents.iter()
                                    .map(|p| if p.len() >= 7 { &p[..7] } else { p.as_str() })
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                el.child(
                                    h_flex().gap_2()
                                        .child(Label::new("Parents:").size(LabelSize::Small).color(Color::Muted))
                                        .child(Label::new(parent_str).size(LabelSize::Small).color(Color::Accent))
                                )
                            })
                            .child(
                                h_flex().gap_2()
                                    .child(Label::new("Author:").size(LabelSize::Small).color(Color::Muted))
                                    .child(Label::new(author).size(LabelSize::Small))
                            )
                            .child(
                                h_flex().gap_2()
                                    .child(Label::new("Date:").size(LabelSize::Small).color(Color::Muted))
                                    .child(Label::new(format_timestamp(timestamp)).size(LabelSize::Small))
                            )
                            .child(div().h_2())
                            .child(Label::new(subject).size(LabelSize::Small))
                    )
                    .child(
                        v_flex()
                            .id("file-list-scroll")
                            .flex_1()
                            .h_full()
                            .p_2()
                            .gap_0p5()
                            .overflow_y_scroll()
                            .child(
                                h_flex()
                                    .w_full()
                                    .pb_1()
                                    .mb_1()
                                    .border_b_1()
                                    .border_color(cx.theme().colors().border)
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(Icon::new(IconName::FileTree).color(Color::Muted).size(IconSize::Small))
                                            .child(Label::new(format!("{} files", expanded_files.len())).size(LabelSize::Small).color(Color::Muted))
                                    )
                                    .child(
                                        Button::new("view-diff", "View Diff")
                                            .style(ButtonStyle::Filled)
                                            .label_size(LabelSize::Small)
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.open_commit_view(None, window, cx);
                                            }))
                                    )
                            )
                            .when(loading_files, |el| {
                                el.child(
                                    div()
                                        .w_full()
                                        .py_2()
                                        .child(Label::new("Loading...").size(LabelSize::Small).color(Color::Muted))
                                )
                            })
                            .when(!loading_files && expanded_files.is_empty(), |el| {
                                el.child(
                                    div()
                                        .w_full()
                                        .py_2()
                                        .child(Label::new("No files changed").size(LabelSize::Small).color(Color::Muted))
                                )
                            })
                            .when(!loading_files && !expanded_files.is_empty(), |el| {
                                el.children(expanded_files.iter().enumerate().map(|(file_idx, file)| {
                                    let file_path = file.path.clone();
                                    let status_icon = match file.status {
                                        FileStatus::Added => IconName::Plus,
                                        FileStatus::Modified => IconName::Pencil,
                                        FileStatus::Deleted => IconName::Trash,
                                        FileStatus::Renamed => IconName::Replace,
                                        FileStatus::Copied => IconName::Copy,
                                        FileStatus::Unknown => IconName::File,
                                    };
                                    let status_color = match file.status {
                                        FileStatus::Added => Color::Created,
                                        FileStatus::Modified => Color::Modified,
                                        FileStatus::Deleted => Color::Deleted,
                                        _ => Color::Muted,
                                    };

                                    h_flex()
                                        .id(ElementId::NamedInteger("file-row".into(), file_idx as u64))
                                        .w_full()
                                        .px_1()
                                        .py_0p5()
                                        .gap_2()
                                        .items_center()
                                        .cursor_pointer()
                                        .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                                        .rounded_sm()
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            this.open_commit_view(Some(file_path.clone()), window, cx);
                                        }))
                                        .child(Icon::new(status_icon).color(status_color).size(IconSize::Small))
                                        .child(Label::new(file.path.clone()).size(LabelSize::Small).single_line())
                                }))
                            })
                    )
            )
            .child(
                div()
                    .w(px(24.0))
                    .h_full()
                    .flex_shrink_0()
                    .flex()
                    .items_start()
                    .justify_center()
                    .pt_1()
                    .child(
                        IconButton::new("close-expanded", IconName::Close)
                            .icon_size(IconSize::Small)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.expanded_commit = None;
                                this.expanded_files.clear();
                                cx.notify();
                            }))
                    )
            )
            .into_any_element()
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
                        .child(div().w(graph_width).child(Label::new("Graph").color(Color::Muted)))
                        .child(div().flex_1().child(Label::new("Description").color(Color::Muted)))
                        .child(div().w(date_width).child(Label::new("Date").color(Color::Muted)))
                        .child(div().w(author_width).child(Label::new("Author").color(Color::Muted)))
                        .child(div().w(commit_width).child(Label::new("Commit").color(Color::Muted))),
                )
                .child(
                    list(
                        self.list_state.clone(),
                        cx.processor(Self::render_list_item),
                    )
                    .flex_1()
                    .w_full()
                )
        };

        div()
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .key_context("GitGraph")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::handle_refresh_graph))
            .on_action(cx.listener(Self::handle_open_commit_view))
            .on_action(cx.listener(Self::handle_checkout_commit))
            .on_action(cx.listener(Self::handle_copy_sha))
            .on_action(cx.listener(Self::handle_create_branch))
            .on_action(cx.listener(Self::handle_cherry_pick_commit))
            .on_action(cx.listener(Self::handle_revert_commit))
            .on_action(cx.listener(Self::handle_reset_soft))
            .on_action(cx.listener(Self::handle_reset_hard))
            .on_action(cx.listener(Self::handle_reset_mixed))
            .on_action(cx.listener(Self::handle_create_tag))
            .on_action(cx.listener(Self::handle_merge_into_current))
            .on_action(cx.listener(Self::handle_pull_into_current))
            .on_action(cx.listener(Self::handle_copy_branch_name))
            .on_action(cx.listener(Self::handle_rename_branch))
            .on_action(cx.listener(Self::handle_delete_branch))
            .on_action(cx.listener(Self::handle_delete_remote_branch))
            .on_action(cx.listener(Self::handle_rebase_onto))
            .on_action(cx.listener(Self::handle_push_branch))
            .on_action(cx.listener(Self::handle_pull_branch))
            .on_action(cx.listener(Self::handle_fetch_all))
            .on_action(cx.listener(Self::handle_stash_changes))
            .on_action(cx.listener(Self::handle_stash_pop))
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
