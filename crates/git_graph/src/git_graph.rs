use anyhow::Result;
use gpui::{
    Action, App, Bounds, ClickEvent, ClipboardItem, Context, Corner, DismissEvent, ElementId,
    Entity, EventEmitter, FocusHandle, Focusable, Hsla, InteractiveElement, ListSizingBehavior,
    MouseButton, MouseDownEvent, ParentElement, Pixels, Point, Render, SharedString, Styled,
    Subscription, Task, UniformListScrollHandle, WeakEntity, Window, actions, anchored, canvas,
    deferred, uniform_list,
};
use project::Project;
use project::git_store::GitStoreEvent;
use settings::Settings;
use std::ops::Range;
use std::path::PathBuf;
use theme::ThemeSettings;
use time::{OffsetDateTime, UtcOffset};
use ui::prelude::*;
use ui::{ContextMenu, ScrollAxes, Scrollbars, Tooltip, WithScrollbar};
use util::command::new_smol_command;
use workspace::item::{Item, ItemEvent};

actions!(
    git_graph,
    [
        OpenGitGraph,
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
            let git_graph = cx.new(|cx| GitGraph::new(project, window, cx));
            workspace.add_item_to_active_pane(Box::new(git_graph), None, true, window, cx);
        });
    })
    .detach();
}

const BRANCH_COLORS: &[Hsla] = &[
    Hsla {
        h: 200.0 / 360.0,
        s: 0.9,
        l: 0.55,
        a: 1.0,
    }, // Cyan
    Hsla {
        h: 320.0 / 360.0,
        s: 0.9,
        l: 0.55,
        a: 1.0,
    }, // Magenta/Pink
    Hsla {
        h: 45.0 / 360.0,
        s: 0.95,
        l: 0.50,
        a: 1.0,
    }, // Orange
    Hsla {
        h: 120.0 / 360.0,
        s: 0.8,
        l: 0.45,
        a: 1.0,
    }, // Green
    Hsla {
        h: 270.0 / 360.0,
        s: 0.8,
        l: 0.60,
        a: 1.0,
    }, // Purple
    Hsla {
        h: 0.0 / 360.0,
        s: 0.85,
        l: 0.55,
        a: 1.0,
    }, // Red
    Hsla {
        h: 180.0 / 360.0,
        s: 0.8,
        l: 0.45,
        a: 1.0,
    }, // Teal
    Hsla {
        h: 60.0 / 360.0,
        s: 0.9,
        l: 0.50,
        a: 1.0,
    }, // Yellow
    Hsla {
        h: 210.0 / 360.0,
        s: 0.85,
        l: 0.55,
        a: 1.0,
    }, // Blue
    Hsla {
        h: 340.0 / 360.0,
        s: 0.85,
        l: 0.55,
        a: 1.0,
    }, // Rose
    Hsla {
        h: 90.0 / 360.0,
        s: 0.75,
        l: 0.50,
        a: 1.0,
    }, // Lime
    Hsla {
        h: 240.0 / 360.0,
        s: 0.75,
        l: 0.60,
        a: 1.0,
    }, // Indigo
    Hsla {
        h: 30.0 / 360.0,
        s: 0.90,
        l: 0.50,
        a: 1.0,
    }, // Orange-Red
    Hsla {
        h: 160.0 / 360.0,
        s: 0.75,
        l: 0.45,
        a: 1.0,
    }, // Sea Green
    Hsla {
        h: 290.0 / 360.0,
        s: 0.70,
        l: 0.55,
        a: 1.0,
    }, // Violet
    Hsla {
        h: 15.0 / 360.0,
        s: 0.85,
        l: 0.55,
        a: 1.0,
    }, // Coral
    Hsla {
        h: 175.0 / 360.0,
        s: 0.70,
        l: 0.50,
        a: 1.0,
    }, // Aqua
    Hsla {
        h: 300.0 / 360.0,
        s: 0.65,
        l: 0.55,
        a: 1.0,
    }, // Orchid
    Hsla {
        h: 75.0 / 360.0,
        s: 0.80,
        l: 0.45,
        a: 1.0,
    }, // Yellow-Green
    Hsla {
        h: 225.0 / 360.0,
        s: 0.75,
        l: 0.55,
        a: 1.0,
    }, // Slate Blue
    Hsla {
        h: 350.0 / 360.0,
        s: 0.80,
        l: 0.50,
        a: 1.0,
    }, // Crimson
    Hsla {
        h: 140.0 / 360.0,
        s: 0.70,
        l: 0.50,
        a: 1.0,
    }, // Spring Green
    Hsla {
        h: 255.0 / 360.0,
        s: 0.65,
        l: 0.60,
        a: 1.0,
    }, // Periwinkle
    Hsla {
        h: 20.0 / 360.0,
        s: 0.85,
        l: 0.50,
        a: 1.0,
    }, // Burnt Orange
    Hsla {
        h: 190.0 / 360.0,
        s: 0.75,
        l: 0.50,
        a: 1.0,
    }, // Steel Blue
    Hsla {
        h: 330.0 / 360.0,
        s: 0.75,
        l: 0.55,
        a: 1.0,
    }, // Hot Pink
    Hsla {
        h: 100.0 / 360.0,
        s: 0.65,
        l: 0.50,
        a: 1.0,
    }, // Olive Green
    Hsla {
        h: 265.0 / 360.0,
        s: 0.60,
        l: 0.55,
        a: 1.0,
    }, // Lavender
    Hsla {
        h: 5.0 / 360.0,
        s: 0.80,
        l: 0.55,
        a: 1.0,
    }, // Tomato
    Hsla {
        h: 150.0 / 360.0,
        s: 0.65,
        l: 0.50,
        a: 1.0,
    }, // Medium Sea Green
    Hsla {
        h: 280.0 / 360.0,
        s: 0.55,
        l: 0.55,
        a: 1.0,
    }, // Medium Purple
    Hsla {
        h: 35.0 / 360.0,
        s: 0.85,
        l: 0.55,
        a: 1.0,
    }, // Gold
    Hsla {
        h: 195.0 / 360.0,
        s: 0.70,
        l: 0.55,
        a: 1.0,
    }, // Light Blue
    Hsla {
        h: 310.0 / 360.0,
        s: 0.70,
        l: 0.55,
        a: 1.0,
    }, // Medium Violet
];

#[derive(Clone, Debug)]
pub struct GraphLine {
    pub from_lane: usize,
    pub to_lane: usize,
    pub line_type: LineType,
    pub color_idx: usize,
    pub rows_to_parent: usize,
    pub parent_sha: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LineType {
    Straight,
    MergeDown,
    BranchOut,
}

#[derive(Clone, Debug)]
pub struct CommitEntry {
    pub sha: String,
    pub short_sha: String,
    pub subject: String,
    pub author_name: String,
    pub timestamp: i64,
    pub parents: Vec<String>,
    pub refs: Vec<String>,
    pub lane: usize,
    pub color_idx: usize,
    pub lines: Vec<GraphLine>,
    pub is_first_on_lane: bool,
}

pub struct GitGraph {
    focus_handle: FocusHandle,
    project: Entity<Project>,
    commits: Vec<CommitEntry>,
    max_lanes: usize,
    loading: bool,
    error: Option<SharedString>,
    _load_task: Option<Task<()>>,
    scroll_handle: UniformListScrollHandle,
    selected_commit: Option<usize>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    work_dir: Option<PathBuf>,
    _subscriptions: Vec<Subscription>,
}

impl GitGraph {
    pub fn new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
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

        let mut this = GitGraph {
            focus_handle,
            project,
            commits: Vec::new(),
            max_lanes: 0,
            loading: true,
            error: None,
            _load_task: None,
            scroll_handle: UniformListScrollHandle::new(),
            selected_commit: None,
            context_menu: None,
            work_dir: None,
            _subscriptions: vec![git_store_subscription],
        };

        this.load_data(cx);
        this
    }

    fn get_selected_commit(&self) -> Option<&CommitEntry> {
        self.selected_commit.and_then(|idx| self.commits.get(idx))
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
                        this.commits = commits;
                        this.max_lanes = max_lanes;
                        this.work_dir = Some(work_dir);
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
}

async fn run_git_command(work_dir: &PathBuf, args: &[&str]) -> Result<String> {
    let output = new_smol_command("git")
        .current_dir(work_dir)
        .args(args)
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("{}", stderr);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

async fn load_commits(
    project: Entity<Project>,
    cx: &mut gpui::AsyncApp,
) -> Result<(Vec<CommitEntry>, usize, PathBuf)> {
    let work_dir = cx
        .update(|cx| {
            let project = project.read(cx);
            project
                .worktrees(cx)
                .next()
                .map(|wt| wt.read(cx).abs_path().to_path_buf())
        })?
        .ok_or_else(|| anyhow::anyhow!("No worktree found"))?;

    let (commits, max_lanes) = fetch_git_log(&work_dir).await?;
    Ok((commits, max_lanes, work_dir))
}

async fn fetch_git_log(work_dir: &PathBuf) -> Result<(Vec<CommitEntry>, usize)> {
    let output = new_smol_command("git")
        .current_dir(work_dir)
        .args([
            "log",
            "--all",
            "--format=%H|%h|%s|%an|%at|%P|%D",
            "--date-order",
        ])
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git log failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut raw_commits = Vec::new();

    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 6 {
            let sha = parts[0].to_string();
            let short_sha = parts[1].to_string();
            let subject = parts[2].to_string();
            let author_name = parts[3].to_string();
            let timestamp = parts[4].parse().unwrap_or(0);
            let parents: Vec<String> = parts[5].split_whitespace().map(|s| s.to_string()).collect();
            let refs: Vec<String> = if parts.len() > 6 && !parts[6].is_empty() {
                parts[6].split(", ").map(|s| s.to_string()).collect()
            } else {
                Vec::new()
            };

            raw_commits.push((
                sha,
                short_sha,
                subject,
                author_name,
                timestamp,
                parents,
                refs,
            ));
        }
    }

    let (commits, max_lanes) = build_graph(raw_commits);
    Ok((commits, max_lanes))
}

fn build_graph(
    raw_commits: Vec<(
        String,
        String,
        String,
        String,
        i64,
        Vec<String>,
        Vec<String>,
    )>,
) -> (Vec<CommitEntry>, usize) {
    use std::collections::HashMap;

    let mut commits = Vec::new();
    let mut active_lanes: Vec<Option<(String, usize)>> = Vec::new();
    let mut lane_colors: HashMap<usize, usize> = HashMap::new();
    let mut next_color = 0;
    let mut max_lanes = 0;

    for (sha, short_sha, subject, author_name, timestamp, parents, refs) in raw_commits {
        let mut lines = Vec::new();

        let was_expected = active_lanes
            .iter()
            .any(|s| s.as_ref().map(|(h, _)| h) == Some(&sha));

        let commit_lane = active_lanes
            .iter()
            .position(|s| s.as_ref().map(|(h, _)| h) == Some(&sha))
            .unwrap_or_else(|| {
                let lane = active_lanes
                    .iter()
                    .position(|s| s.is_none())
                    .unwrap_or_else(|| {
                        active_lanes.push(None);
                        active_lanes.len() - 1
                    });
                lane
            });

        let is_first_on_lane = !was_expected;

        let color_idx = *lane_colors.entry(commit_lane).or_insert_with(|| {
            let color = next_color;
            next_color = (next_color + 1) % BRANCH_COLORS.len();
            color
        });

        for (lane_idx, lane_data) in active_lanes.iter().enumerate() {
            if let Some((hash, lane_color)) = lane_data {
                if hash != &sha {
                    lines.push(GraphLine {
                        from_lane: lane_idx,
                        to_lane: lane_idx,
                        line_type: LineType::Straight,
                        color_idx: *lane_color,
                        rows_to_parent: 1,
                        parent_sha: None,
                    });
                }
            }
        }

        if commit_lane < active_lanes.len() {
            active_lanes[commit_lane] = None;
        }

        for (i, parent) in parents.iter().enumerate() {
            let existing_lane = active_lanes
                .iter()
                .position(|s| s.as_ref().map(|(h, _)| h) == Some(parent));

            if let Some(target_lane) = existing_lane {
                let target_color = active_lanes[target_lane]
                    .as_ref()
                    .map(|(_, c)| *c)
                    .unwrap_or(color_idx);
                if target_lane != commit_lane {
                    lines.push(GraphLine {
                        from_lane: commit_lane,
                        to_lane: target_lane,
                        line_type: LineType::MergeDown,
                        color_idx: target_color,
                        rows_to_parent: 1,
                        parent_sha: Some(parent.clone()),
                    });
                }
            } else if i == 0 {
                if commit_lane < active_lanes.len() {
                    active_lanes[commit_lane] = Some((parent.clone(), color_idx));
                } else {
                    active_lanes.push(Some((parent.clone(), color_idx)));
                }
                lines.push(GraphLine {
                    from_lane: commit_lane,
                    to_lane: commit_lane,
                    line_type: LineType::Straight,
                    color_idx,
                    rows_to_parent: 1,
                    parent_sha: Some(parent.clone()),
                });
            } else {
                let target_lane = active_lanes
                    .iter()
                    .position(|s| s.is_none())
                    .unwrap_or_else(|| {
                        active_lanes.push(None);
                        active_lanes.len() - 1
                    });

                let branch_color = *lane_colors.entry(target_lane).or_insert_with(|| {
                    let color = next_color;
                    next_color = (next_color + 1) % BRANCH_COLORS.len();
                    color
                });

                active_lanes[target_lane] = Some((parent.clone(), branch_color));
                lines.push(GraphLine {
                    from_lane: commit_lane,
                    to_lane: target_lane,
                    line_type: LineType::BranchOut,
                    color_idx: branch_color,
                    rows_to_parent: 1,
                    parent_sha: Some(parent.clone()),
                });
            }
        }

        max_lanes = max_lanes.max(active_lanes.len());

        commits.push(CommitEntry {
            sha,
            short_sha,
            subject,
            author_name,
            timestamp,
            parents,
            refs,
            lane: commit_lane,
            color_idx,
            lines,
            is_first_on_lane,
        });
    }

    let sha_to_row: HashMap<String, usize> = commits
        .iter()
        .enumerate()
        .map(|(i, c)| (c.sha.clone(), i))
        .collect();

    for (row_idx, commit) in commits.iter_mut().enumerate() {
        for line in &mut commit.lines {
            if let Some(ref parent_sha) = line.parent_sha {
                if let Some(&parent_row) = sha_to_row.get(parent_sha) {
                    if parent_row > row_idx {
                        line.rows_to_parent = parent_row - row_idx;
                    }
                }
            }
        }
    }

    (commits, max_lanes.max(1))
}

fn format_timestamp(timestamp: i64) -> String {
    let Ok(datetime) = OffsetDateTime::from_unix_timestamp(timestamp) else {
        return "Unknown".to_string();
    };

    let local_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    let local_datetime = datetime.to_offset(local_offset);

    let format = time::format_description::parse("[day] [month repr:short] [year] [hour]:[minute]")
        .unwrap_or_default();
    local_datetime.format(&format).unwrap_or_default()
}

fn render_graph_cell(
    commit: &CommitEntry,
    row_height: Pixels,
    graph_width: Pixels,
) -> impl IntoElement {
    let lane = commit.lane;
    let lines = commit.lines.clone();
    let commit_color_idx = commit.color_idx;
    let is_first_on_lane = commit.is_first_on_lane;

    canvas(
        move |_bounds, _window, _cx| {},
        move |bounds: Bounds<Pixels>, _: (), window: &mut Window, _cx: &mut App| {
            let lane_width = px(16.0);
            let left_padding = px(12.0);
            let y_top = bounds.origin.y;
            let y_center = bounds.origin.y + row_height / 2.0;
            let y_bottom = bounds.origin.y + row_height;
            let line_width = px(2.0);

            for line in &lines {
                let color = BRANCH_COLORS[line.color_idx % BRANCH_COLORS.len()];
                let from_x = bounds.origin.x
                    + left_padding
                    + lane_width * line.from_lane as f32
                    + lane_width / 2.0;
                let to_x = bounds.origin.x
                    + left_padding
                    + lane_width * line.to_lane as f32
                    + lane_width / 2.0;

                match line.line_type {
                    LineType::Straight => {
                        let is_commit_lane = line.from_lane == lane;
                        let start_y = if is_commit_lane && is_first_on_lane {
                            y_center
                        } else {
                            y_top
                        };

                        window.paint_quad(gpui::fill(
                            Bounds::new(
                                Point::new(from_x - line_width / 2.0, start_y),
                                gpui::size(line_width, y_bottom - start_y),
                            ),
                            color,
                        ));
                    }
                    LineType::MergeDown | LineType::BranchOut => {
                        draw_s_curve(window, from_x, y_center, to_x, y_bottom, line_width, color);
                    }
                }
            }

            let commit_x =
                bounds.origin.x + left_padding + lane_width * lane as f32 + lane_width / 2.0;
            let commit_color = BRANCH_COLORS[commit_color_idx % BRANCH_COLORS.len()];
            let dot_radius = px(4.0);

            window.paint_quad(
                gpui::fill(
                    Bounds::centered_at(
                        Point::new(commit_x, y_center),
                        gpui::size(dot_radius * 2.0, dot_radius * 2.0),
                    ),
                    commit_color,
                )
                .corner_radii(dot_radius),
            );
        },
    )
    .w(graph_width)
    .h(row_height)
}

fn draw_s_curve(
    window: &mut Window,
    from_x: Pixels,
    from_y: Pixels,
    to_x: Pixels,
    to_y: Pixels,
    line_width: Pixels,
    color: Hsla,
) {
    let segments = 20;
    let half_width = f32::from(line_width / 2.0);

    let mid_y = (from_y + to_y) / 2.0;

    for i in 0..segments {
        let t0 = i as f32 / segments as f32;
        let t1 = (i + 1) as f32 / segments as f32;

        let (x0, y0) = cubic_bezier(from_x, from_y, from_x, mid_y, to_x, mid_y, to_x, to_y, t0);
        let (x1, y1) = cubic_bezier(from_x, from_y, from_x, mid_y, to_x, mid_y, to_x, to_y, t1);

        let dx = f32::from(x1 - x0);
        let dy = f32::from(y1 - y0);
        let len = (dx * dx + dy * dy).sqrt();

        if len > 0.01 {
            let nx = -dy / len * half_width;
            let ny = dx / len * half_width;

            let mut path = gpui::Path::new(Point::new(x0 - px(nx), y0 - px(ny)));
            path.line_to(Point::new(x0 + px(nx), y0 + px(ny)));
            path.line_to(Point::new(x1 + px(nx), y1 + px(ny)));
            path.line_to(Point::new(x1 - px(nx), y1 - px(ny)));
            window.paint_path(path, color);
        }
    }
}

fn cubic_bezier(
    p0x: Pixels,
    p0y: Pixels,
    p1x: Pixels,
    p1y: Pixels,
    p2x: Pixels,
    p2y: Pixels,
    p3x: Pixels,
    p3y: Pixels,
    t: f32,
) -> (Pixels, Pixels) {
    let inv_t = 1.0 - t;
    let inv_t2 = inv_t * inv_t;
    let inv_t3 = inv_t2 * inv_t;
    let t2 = t * t;
    let t3 = t2 * t;

    let x = inv_t3 * p0x + 3.0 * inv_t2 * t * p1x + 3.0 * inv_t * t2 * p2x + t3 * p3x;
    let y = inv_t3 * p0y + 3.0 * inv_t2 * t * p1y + 3.0 * inv_t * t2 * p2y + t3 * p3y;
    (x, y)
}

fn render_ref_badges(refs: &[String], cx: &Context<GitGraph>) -> impl IntoElement {
    let badges: Vec<_> = refs
        .iter()
        .take(3)
        .map(|ref_name| {
            let (bg_color, text_color, border_color, icon) = if ref_name.starts_with("HEAD") {
                (
                    cx.theme().colors().element_selected,
                    Color::Default,
                    Some(cx.theme().colors().border_focused),
                    IconName::GitBranch,
                )
            } else if ref_name.starts_with("origin/") {
                (
                    cx.theme().colors().element_background,
                    Color::Muted,
                    None,
                    IconName::GitBranch,
                )
            } else if ref_name.starts_with("tag:") {
                (
                    cx.theme().colors().element_background,
                    Color::Muted,
                    None,
                    IconName::GitBranch,
                )
            } else {
                (
                    cx.theme().colors().element_selected,
                    Color::Accent,
                    None,
                    IconName::GitBranch,
                )
            };

            let display_name: SharedString = if let Some(stripped) = ref_name.strip_prefix("tag: ")
            {
                stripped.to_string().into()
            } else if let Some(stripped) = ref_name.strip_prefix("HEAD -> ") {
                stripped.to_string().into()
            } else {
                ref_name.clone().into()
            };

            (display_name, bg_color, text_color, border_color, icon)
        })
        .collect();

    h_flex().gap_1().children(badges.into_iter().map(
        |(display_name, bg_color, text_color, border_color, icon)| {
            h_flex()
                .gap_1()
                .px_1()
                .py_px()
                .rounded_sm()
                .bg(bg_color)
                .when_some(border_color, |this, color| {
                    this.border_1().border_color(color)
                })
                .child(Icon::new(icon).size(IconSize::Small).color(text_color))
                .child(
                    Label::new(display_name)
                        .size(LabelSize::Small)
                        .color(text_color),
                )
        },
    ))
}

impl Render for GitGraph {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let font_size = settings.buffer_font_size(cx);
        let _row_height = font_size + px(10.0);
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
            let commit_count = self.commits.len();

            div()
                .size_full()
                .flex()
                .flex_col()
                .child(
                    h_flex()
                        .w_full()
                        .px_2()
                        .py_1()
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
                    div()
                        .id("git-graph-list")
                        .flex_1()
                        .size_full()
                        .child(
                            uniform_list(
                                "git-graph-commits",
                                commit_count,
                                cx.processor(move |this: &mut Self, range: Range<usize>, _window, cx| {
                                    let settings = ThemeSettings::get_global(cx);
                                    let font_size = settings.buffer_font_size(cx);
                                    let row_height = font_size + px(10.0);
                                    let graph_width = px(16.0) * (this.max_lanes.max(2) as f32) + px(24.0);
                                    let date_width = px(140.0);
                                    let author_width = px(120.0);
                                    let commit_width = px(80.0);

                                    range
                                        .filter_map(|idx| {
                                            let commit = this.commits.get(idx)?;
                                            let bg = if idx % 2 == 0 {
                                                cx.theme().colors().surface_background
                                            } else {
                                                cx.theme().colors().background
                                            };

                                            Some(
                                                h_flex()
                                                    .id(ElementId::NamedInteger("commit-row".into(), idx as u64))
                                                    .w_full()
                                                    .px_2()
                                                    .h(row_height)
                                                    .min_h(row_height)
                                                    .max_h(row_height)
                                                    .flex_shrink_0()
                                                    .bg(bg)
                                                    .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                                                    .on_click(cx.listener(move |this, event: &ClickEvent, _window, cx| {
                                                        if event.click_count() == 2 {
                                                            this.selected_commit = Some(idx);
                                                            this.checkout_commit(cx);
                                                        }
                                                    }))
                                                    .child(
                                                        div()
                                                            .w(graph_width)
                                                            .h_full()
                                                            .flex_shrink_0()
                                                            .child(render_graph_cell(commit, row_height, graph_width))
                                                    )
                                                    .child(
                                                        h_flex()
                                                            .flex_1()
                                                            .min_w(px(0.0))
                                                            .gap_2()
                                                            .overflow_hidden()
                                                            .items_center()
                                                            .when(!commit.refs.is_empty(), |this| {
                                                                this.child(
                                                                    div()
                                                                        .id(ElementId::NamedInteger("ref-badges".into(), idx as u64))
                                                                        .cursor_pointer()
                                                                        .on_mouse_down(MouseButton::Right, cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                                                                            this.deploy_context_menu(event.position, idx, window, cx);
                                                                        }))
                                                                        .child(render_ref_badges(&commit.refs, cx))
                                                                )
                                                            })
                                                            .child({
                                                                let subject = commit.subject.clone();
                                                                div()
                                                                    .id(ElementId::NamedInteger("commit-subject".into(), idx as u64))
                                                                    .flex_1()
                                                                    .min_w(px(0.0))
                                                                    .overflow_hidden()
                                                                    .tooltip(Tooltip::text(subject.clone()))
                                                                    .child(
                                                                        Label::new(subject)
                                                                            .single_line()
                                                                    )
                                                            }),
                                                    )
                                                    .child(
                                                        div()
                                                            .w(date_width)
                                                            .flex_shrink_0()
                                                            .overflow_hidden()
                                                            .child(Label::new(format_timestamp(commit.timestamp)).color(Color::Muted).single_line()),
                                                    )
                                                    .child(
                                                        div()
                                                            .w(author_width)
                                                            .flex_shrink_0()
                                                            .overflow_hidden()
                                                            .child(Label::new(commit.author_name.clone()).color(Color::Muted).single_line()),
                                                    )
                                                    .child(
                                                        div()
                                                            .w(commit_width)
                                                            .flex_shrink_0()
                                                            .child(Label::new(commit.short_sha.clone()).color(Color::Accent).single_line()),
                                                    )
                                            )
                                        })
                                        .collect()
                                }),
                            )
                            .size_full()
                            .with_sizing_behavior(ListSizingBehavior::Infer)
                            .track_scroll(&self.scroll_handle),
                        )
                        .custom_scrollbars(
                            Scrollbars::new(ScrollAxes::Vertical)
                                .tracked_scroll_handle(&self.scroll_handle),
                            window,
                            cx,
                        ),
                )
        };

        div()
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .key_context("GitGraph")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::handle_refresh_graph))
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
