use crate::{
    branch_picker,
    conflict_view::ConflictAddon,
    git_panel::{GitPanel, GitPanelAddon, GitStatusEntry, Section},
    git_panel_settings::GitPanelSettings,
};
use agent_settings::AgentSettings;
use anyhow::{Context as _, Result, anyhow};
use buffer_diff::{BufferDiff, DiffHunkSecondaryStatus};
use collections::HashMap;
use editor::{
    Addon, Editor, EditorEvent, EditorSettings, SelectionEffects, SplittableEditor,
    actions::{GoToHunk, GoToPreviousHunk, SendReviewToAgent},
    multibuffer_context_lines,
    scroll::Autoscroll,
};
use futures_lite::future::yield_now;
use git::repository::DiffType;

use git::{
    Commit, StageAll, StageAndNext, ToggleStaged, UnstageAll, UnstageAndNext, repository::RepoPath,
    status::FileStatus,
};
use gpui::{
    Action, AnyElement, App, AppContext as _, AsyncWindowContext, Entity, EventEmitter,
    FocusHandle, Focusable, Render, Subscription, Task, WeakEntity, actions,
};
use language::{Anchor, Buffer, BufferId, Capability, DiskState, OffsetRangeExt};
use multi_buffer::{MultiBuffer, PathKey};
use project::{
    Project, ProjectPath,
    git_store::{
        Repository,
        branch_diff::{self, BranchDiffEvent, DiffBase},
    },
};
use settings::{Settings, SettingsStore};
use std::any::{Any, TypeId};
use std::sync::Arc;
use theme::ActiveTheme;
use ui::{
    CommonAnimationExt as _, ContextMenu, DiffStat, Divider, DropdownMenu, DropdownStyle,
    KeyBinding, PopoverMenu, Tooltip, prelude::*, vertical_divider,
};
use util::{ResultExt as _, rel_path::RelPath};
use workspace::{
    CloseActiveItem, ItemNavHistory, SerializableItem, ToolbarItemEvent, ToolbarItemLocation,
    ToolbarItemView, Workspace,
    item::{Item, ItemEvent, ItemHandle, SaveOptions, TabContentParams},
    notifications::NotifyTaskExt,
    searchable::SearchableItemHandle,
};
use zed_actions::agent::ReviewBranchDiff;
use ztracing::instrument;

actions!(
    git,
    [
        /// Shows the diff between the working directory and the index.
        Diff,
        /// Adds files to the git staging area.
        Add,
        /// Shows the diff between the working directory and your default
        /// branch (typically main or master).
        BranchDiff,
        /// Opens a new agent thread with the branch diff for review.
        ReviewDiff,
        LeaderAndFollower,
    ]
);

pub struct ProjectDiff {
    project: Entity<Project>,
    multibuffer: Entity<MultiBuffer>,
    branch_diff: Entity<branch_diff::BranchDiff>,
    editor: Entity<SplittableEditor>,
    buffer_diff_subscriptions: HashMap<Arc<RelPath>, (Entity<BufferDiff>, Subscription)>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    pending_scroll: Option<PathKey>,
    branch_filter_generation: u64,
    review_comment_count: usize,
    _task: Task<Result<()>>,
    _subscription: Subscription,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefreshReason {
    DiffChanged,
    StatusesChanged,
    EditorSaved,
}

const CONFLICT_SORT_PREFIX: u64 = 1;
const TRACKED_SORT_PREFIX: u64 = 2;
const NEW_SORT_PREFIX: u64 = 3;

impl ProjectDiff {
    pub(crate) fn register(workspace: &mut Workspace, cx: &mut Context<Workspace>) {
        workspace.register_action(Self::deploy);
        workspace.register_action(Self::deploy_branch_diff);
        workspace.register_action(|workspace, _: &Add, window, cx| {
            Self::deploy(workspace, &Diff, window, cx);
        });
        workspace::register_serializable_item::<ProjectDiff>(cx);
    }

    fn deploy(
        workspace: &mut Workspace,
        _: &Diff,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        Self::deploy_at(workspace, None, DiffBase::Head, window, cx)
    }

    fn deploy_branch_diff(
        workspace: &mut Workspace,
        _: &BranchDiff,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        telemetry::event!("Git Branch Diff Opened");
        let project = workspace.project().clone();
        let intended_repo = project.read(cx).active_repository(cx);

        let existing = workspace
            .items_of_type::<Self>(cx)
            .find(|item| matches!(item.read(cx).diff_base(cx), DiffBase::Merge { .. }));
        if let Some(existing) = existing {
            workspace.activate_item(&existing, true, true, window, cx);

            if let Some(intended_repo) = intended_repo {
                let needs_switch = existing
                    .read(cx)
                    .branch_diff
                    .read(cx)
                    .repo()
                    .map_or(true, |current| {
                        current.read(cx).id != intended_repo.read(cx).id
                    });

                if needs_switch {
                    let default_branch =
                        intended_repo.update(cx, |repo, _| repo.default_branch(true));
                    let existing = existing.downgrade();
                    let workspace = cx.entity().downgrade();
                    window
                        .spawn(cx, async move |cx| {
                            let default_branch = default_branch
                                .await??
                                .context("Could not determine default branch")?;

                            existing.update(cx, |project_diff, cx| {
                                project_diff.branch_diff.update(cx, |branch_diff, cx| {
                                    branch_diff.set_repo(Some(intended_repo), cx);
                                    branch_diff.set_diff_base(
                                        DiffBase::Merge {
                                            base_ref: default_branch,
                                        },
                                        cx,
                                    );
                                });
                            })?;
                            anyhow::Ok(())
                        })
                        .detach_and_notify_err(workspace, window, cx);
                }
            }

            return;
        }
        let workspace = cx.entity();
        let workspace_weak = workspace.downgrade();
        window
            .spawn(cx, async move |cx| {
                let this = cx
                    .update(|window, cx| {
                        Self::new_with_default_branch(project, workspace.clone(), window, cx)
                    })?
                    .await?;
                workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace.add_item_to_active_pane(Box::new(this), None, true, window, cx);
                    })
                    .ok();
                anyhow::Ok(())
            })
            .detach_and_notify_err(workspace_weak, window, cx);
    }

    fn review_diff(&mut self, _: &ReviewDiff, window: &mut Window, cx: &mut Context<Self>) {
        let diff_base = self.diff_base(cx).clone();
        let DiffBase::Merge { base_ref } = diff_base else {
            return;
        };

        let Some(repo) = self.branch_diff.read(cx).repo().cloned() else {
            return;
        };

        let diff_receiver = repo.update(cx, |repo, cx| {
            repo.diff(
                DiffType::MergeBase {
                    base_ref: base_ref.clone(),
                },
                cx,
            )
        });

        let workspace = self.workspace.clone();

        window
            .spawn(cx, {
                let workspace = workspace.clone();
                async move |cx| {
                    let diff_text = diff_receiver.await??;

                    if let Some(workspace) = workspace.upgrade() {
                        workspace.update_in(cx, |_workspace, window, cx| {
                            window.dispatch_action(
                                ReviewBranchDiff {
                                    diff_text: diff_text.into(),
                                    base_ref: base_ref.to_string().into(),
                                }
                                .boxed_clone(),
                                cx,
                            );
                        })?;
                    }

                    anyhow::Ok(())
                }
            })
            .detach_and_notify_err(workspace, window, cx);
    }

    pub fn deploy_at(
        workspace: &mut Workspace,
        entry: Option<GitStatusEntry>,
        target_base: DiffBase,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        telemetry::event!(
            "Git Diff Opened",
            source = if entry.is_some() {
                "Git Panel"
            } else {
                "Action"
            }
        );
        let intended_repo = workspace.project().read(cx).active_repository(cx);

        let existing = workspace
            .items_of_type::<Self>(cx)
            .find(|item| item.read(cx).diff_base(cx) == &target_base);
        let project_diff = if let Some(existing) = existing {
            existing.update(cx, |project_diff, cx| {
                project_diff.move_to_beginning(window, cx);
            });

            workspace.activate_item(&existing, true, true, window, cx);
            existing
        } else {
            let workspace_handle = cx.entity();
            let project_diff = cx.new(|cx| {
                Self::new(
                    workspace.project().clone(),
                    workspace_handle,
                    target_base,
                    window,
                    cx,
                )
            });
            workspace.add_item_to_active_pane(
                Box::new(project_diff.clone()),
                None,
                true,
                window,
                cx,
            );
            project_diff
        };

        if let Some(intended) = &intended_repo {
            let needs_switch = project_diff
                .read(cx)
                .branch_diff
                .read(cx)
                .repo()
                .map_or(true, |current| current.read(cx).id != intended.read(cx).id);
            if needs_switch {
                project_diff.update(cx, |project_diff, cx| {
                    project_diff.branch_diff.update(cx, |branch_diff, cx| {
                        branch_diff.set_repo(Some(intended.clone()), cx);
                    });
                });
            }
        }

        if let Some(entry) = entry {
            project_diff.update(cx, |project_diff, cx| {
                project_diff.move_to_entry(entry, window, cx);
            })
        }
    }

    pub fn deploy_at_project_path(
        workspace: &mut Workspace,
        project_path: ProjectPath,
        target_base: DiffBase,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        telemetry::event!("Git Diff Opened", source = "Agent Panel");
        let existing = workspace
            .items_of_type::<Self>(cx)
            .find(|item| item.read(cx).diff_base(cx) == &target_base);
        let project_diff = if let Some(existing) = existing {
            workspace.activate_item(&existing, true, true, window, cx);
            existing
        } else {
            let workspace_handle = cx.entity();
            let project_diff = cx.new(|cx| {
                Self::new(
                    workspace.project().clone(),
                    workspace_handle,
                    target_base,
                    window,
                    cx,
                )
            });
            workspace.add_item_to_active_pane(
                Box::new(project_diff.clone()),
                None,
                true,
                window,
                cx,
            );
            project_diff
        };
        project_diff.update(cx, |project_diff, cx| {
            project_diff.move_to_project_path(&project_path, window, cx);
        });
    }

    pub fn autoscroll(&self, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |editor, cx| {
                editor.request_autoscroll(Autoscroll::fit(), cx);
            })
        })
    }

    fn new_with_default_branch(
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let Some(repo) = project.read(cx).git_store().read(cx).active_repository() else {
            return Task::ready(Err(anyhow!("No active repository")));
        };
        let main_branch = repo.update(cx, |repo, _| repo.default_branch(true));
        window.spawn(cx, async move |cx| {
            let main_branch = main_branch
                .await??
                .context("Could not determine default branch")?;

            let branch_diff = cx.new_window_entity(|window, cx| {
                branch_diff::BranchDiff::new(
                    DiffBase::Merge {
                        base_ref: main_branch,
                    },
                    project.clone(),
                    window,
                    cx,
                )
            })?;
            cx.new_window_entity(|window, cx| {
                Self::new_impl(branch_diff, project, workspace, window, cx)
            })
        })
    }

    fn new(
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        initial_diff_base: DiffBase,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let branch_diff = cx.new(|cx| {
            branch_diff::BranchDiff::new(initial_diff_base, project.clone(), window, cx)
        });
        Self::new_impl(branch_diff, project, workspace, window, cx)
    }

    fn new_impl(
        branch_diff: Entity<branch_diff::BranchDiff>,
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });

        let editor = cx.new(|cx| {
            let diff_display_editor = SplittableEditor::new(
                EditorSettings::get_global(cx).diff_view_style,
                multibuffer.clone(),
                project.clone(),
                workspace.clone(),
                window,
                cx,
            );
            diff_display_editor.rhs_editor().update(cx, |editor, cx| {
                editor.set_show_diff_review_button(true, cx);
            });
            diff_display_editor
        });
        let editor_subscription = cx.subscribe_in(&editor, window, Self::handle_editor_event);

        let primary_editor = editor.read(cx).rhs_editor().clone();
        let review_comment_subscription =
            cx.subscribe(&primary_editor, |this, _editor, event: &EditorEvent, cx| {
                if let EditorEvent::ReviewCommentsChanged { total_count } = event {
                    this.review_comment_count = *total_count;
                    cx.notify();
                }
            });

        let branch_diff_subscription = cx.subscribe_in(
            &branch_diff,
            window,
            move |this, _git_store, event, window, cx| match event {
                BranchDiffEvent::FileListChanged => {
                    this._task = window.spawn(cx, {
                        let this = cx.weak_entity();
                        async |cx| Self::refresh(this, RefreshReason::StatusesChanged, cx).await
                    })
                }
                BranchDiffEvent::DiffBaseChanged => {
                    this.pending_scroll.take();
                    this.configure_editor_for_diff_base(cx);
                    this._task = window.spawn(cx, {
                        let this = cx.weak_entity();
                        async |cx| Self::refresh(this, RefreshReason::StatusesChanged, cx).await
                    })
                }
            },
        );

        let mut was_sort_by_path = GitPanelSettings::get_global(cx).sort_by_path;
        let mut was_collapse_untracked_diff =
            GitPanelSettings::get_global(cx).collapse_untracked_diff;
        cx.observe_global_in::<SettingsStore>(window, move |this, window, cx| {
            let is_sort_by_path = GitPanelSettings::get_global(cx).sort_by_path;
            let is_collapse_untracked_diff =
                GitPanelSettings::get_global(cx).collapse_untracked_diff;
            if is_sort_by_path != was_sort_by_path
                || is_collapse_untracked_diff != was_collapse_untracked_diff
            {
                this._task = {
                    window.spawn(cx, {
                        let this = cx.weak_entity();
                        async |cx| Self::refresh(this, RefreshReason::StatusesChanged, cx).await
                    })
                }
            }
            was_sort_by_path = is_sort_by_path;
            was_collapse_untracked_diff = is_collapse_untracked_diff;
        })
        .detach();

        let task = window.spawn(cx, {
            let this = cx.weak_entity();
            async |cx| Self::refresh(this, RefreshReason::StatusesChanged, cx).await
        });

        let mut this = Self {
            project,
            workspace: workspace.downgrade(),
            branch_diff,
            focus_handle,
            editor,
            multibuffer,
            buffer_diff_subscriptions: Default::default(),
            pending_scroll: None,
            branch_filter_generation: 0,
            review_comment_count: 0,
            _task: task,
            _subscription: Subscription::join(
                branch_diff_subscription,
                Subscription::join(editor_subscription, review_comment_subscription),
            ),
        };
        this.configure_editor_for_diff_base(cx);
        this
    }

    pub fn diff_base<'a>(&'a self, cx: &'a App) -> &'a DiffBase {
        self.branch_diff.read(cx).diff_base()
    }

    pub fn active_diff_base_in(workspace: &Workspace, cx: &App) -> DiffBase {
        workspace
            .items_of_type::<Self>(cx)
            .next()
            .map(|diff| diff.read(cx).diff_base(cx).clone())
            .unwrap_or(DiffBase::Head)
    }

    fn empty_state_text(&self, cx: &App) -> SharedString {
        match self.diff_base(cx) {
            DiffBase::Head => "No uncommitted changes".into(),
            DiffBase::Staged => "No staged changes".into(),
            DiffBase::Unstaged => "No unstaged changes".into(),
            DiffBase::Merge { base_ref } => format!("No changes since {base_ref}").into(),
        }
    }

    fn select_diff_filter(
        &mut self,
        diff_filter: DiffFilter,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.branch_filter_generation = self.branch_filter_generation.wrapping_add(1);
        let generation = self.branch_filter_generation;

        let diff_base = match diff_filter {
            DiffFilter::Uncommitted => DiffBase::Head,
            DiffFilter::Staged => DiffBase::Staged,
            DiffFilter::Unstaged => DiffBase::Unstaged,
            DiffFilter::Branch => {
                self.select_branch_diff_filter(generation, window, cx);
                return;
            }
        };

        self.branch_diff.update(cx, |branch_diff, cx| {
            branch_diff.set_diff_base(diff_base, cx);
        });
    }

    fn select_branch_diff_filter(
        &mut self,
        generation: u64,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let repo = self.branch_diff.read(cx).repo().cloned().or_else(|| {
            self.project
                .read(cx)
                .git_store()
                .read(cx)
                .active_repository()
        });
        let Some(repo) = repo else {
            let error = anyhow!("No active repository");
            log::error!("{error:?}");
            self.workspace
                .update(cx, |workspace, cx| workspace.show_error(&error, cx))
                .log_err();
            return;
        };

        let default_branch = repo.update(cx, |repo, _| repo.default_branch(true));
        let this = cx.weak_entity();
        window
            .spawn(cx, async move |cx| {
                let default_branch = default_branch.await??;
                this.update(cx, |project_diff, cx| -> Result<()> {
                    if project_diff.branch_filter_generation != generation {
                        return Ok(());
                    }

                    let default_branch =
                        default_branch.context("Could not determine default branch")?;
                    project_diff.branch_diff.update(cx, |branch_diff, cx| {
                        branch_diff.set_diff_base(
                            DiffBase::Merge {
                                base_ref: default_branch,
                            },
                            cx,
                        );
                    });
                    Ok(())
                })??;
                anyhow::Ok(())
            })
            .detach_and_notify_err(self.workspace.clone(), window, cx);
    }

    fn configure_editor_for_diff_base(&mut self, cx: &mut Context<Self>) {
        let diff_base = self.branch_diff.read(cx).diff_base().clone();
        let workspace = self.workspace.clone();
        let branch_diff = self.branch_diff.clone();

        let rhs_editor = self.editor.update(cx, |editor, cx| {
            if matches!(diff_base, DiffBase::Merge { .. }) {
                editor.disable_diff_hunk_controls(cx);
            } else {
                editor.enable_diff_hunk_controls(cx);
            }

            editor.rhs_editor().clone()
        });

        rhs_editor.update(cx, |editor, _| {
            // The Staged view shows read-only index snapshots. Opening an excerpt
            // should open the editable worktree file rather than the snapshot
            // buffer, so delegate the open to ProjectDiff (story 38).
            editor.set_delegate_open_excerpts(matches!(diff_base, DiffBase::Staged));

            match diff_base {
                DiffBase::Head | DiffBase::Unstaged => {
                    editor.unregister_addon::<BranchDiffAddon>();
                    if editor.addon::<GitPanelAddon>().is_none() {
                        editor.register_addon(GitPanelAddon { workspace });
                    }
                }
                DiffBase::Staged => {
                    // The staged snapshot buffers are synthetic and unregistered,
                    // so `project.status_for_buffer_id` can't supply the header
                    // status badge; BranchDiffAddon resolves it from the snapshot
                    // map. GitPanelAddon still renders the stage/unstage controls.
                    if editor.addon::<GitPanelAddon>().is_none() {
                        editor.register_addon(GitPanelAddon { workspace });
                    }
                    if editor.addon::<BranchDiffAddon>().is_none() {
                        editor.register_addon(BranchDiffAddon { branch_diff });
                    }
                }
                DiffBase::Merge { .. } => {
                    editor.unregister_addon::<GitPanelAddon>();
                    if editor.addon::<BranchDiffAddon>().is_none() {
                        editor.register_addon(BranchDiffAddon { branch_diff });
                    }
                }
            }
        });
    }

    pub fn move_to_entry(
        &mut self,
        entry: GitStatusEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(git_repo) = self.branch_diff.read(cx).repo() else {
            return;
        };
        let repo = git_repo.read(cx);
        let sort_prefix = sort_prefix(repo, &entry.repo_path, entry.status, cx);
        let path_key = PathKey::with_sort_prefix(sort_prefix, entry.repo_path.as_ref().clone());

        self.move_to_path(path_key, window, cx)
    }

    pub fn move_to_project_path(
        &mut self,
        project_path: &ProjectPath,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(git_repo) = self.branch_diff.read(cx).repo() else {
            return;
        };
        let Some(repo_path) = git_repo
            .read(cx)
            .project_path_to_repo_path(project_path, cx)
        else {
            return;
        };
        let status = git_repo
            .read(cx)
            .status_for_path(&repo_path)
            .map(|entry| entry.status)
            .unwrap_or(FileStatus::Untracked);
        let sort_prefix = sort_prefix(&git_repo.read(cx), &repo_path, status, cx);
        let path_key = PathKey::with_sort_prefix(sort_prefix, repo_path.as_ref().clone());
        self.move_to_path(path_key, window, cx)
    }

    pub fn active_path(&self, cx: &App) -> Option<ProjectPath> {
        let editor = self.editor.read(cx).focused_editor().read(cx);
        let multibuffer = editor.buffer().read(cx);
        let position = editor.selections.newest_anchor().head();
        let snapshot = multibuffer.snapshot(cx);
        let (text_anchor, _) = snapshot.anchor_to_buffer_anchor(position)?;
        let buffer = multibuffer.buffer(text_anchor.buffer_id)?;

        // A real worktree file's path is already worktree-relative. The Staged
        // snapshot's synthetic file (`Historic` disk state) instead carries a
        // repo path, which only equals the worktree-relative path when the repo
        // and worktree roots coincide, so resolve it through the git store.
        if let Some(file) = buffer.read(cx).file()
            && !matches!(file.disk_state(), DiskState::Historic { .. })
        {
            return Some(ProjectPath {
                worktree_id: file.worktree_id(cx),
                path: file.path().clone(),
            });
        }

        self.worktree_project_path_for_buffer(text_anchor.buffer_id, cx)
    }

    /// Resolves an excerpt buffer to the worktree `ProjectPath` it represents.
    /// The Staged view's index snapshot buffer is not registered in the project
    /// buffer store and carries a repo path, so resolve it through the git store
    /// (which maps the synthetic buffer to its repository and repo path) instead
    /// of trusting the synthetic file's path as worktree-relative.
    fn worktree_project_path_for_buffer(
        &self,
        buffer_id: BufferId,
        cx: &App,
    ) -> Option<ProjectPath> {
        let (repo, repo_path) = self
            .project
            .read(cx)
            .git_store()
            .read(cx)
            .repository_and_path_for_buffer_id(buffer_id, cx)?;
        repo.read(cx).repo_path_to_project_path(&repo_path, cx)
    }

    fn move_to_beginning(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |editor, cx| {
                editor.change_selections(Default::default(), window, cx, |s| {
                    s.select_ranges(vec![multi_buffer::Anchor::Min..multi_buffer::Anchor::Min]);
                });
            });
        });
    }

    fn move_to_path(&mut self, path_key: PathKey, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(position) = self.multibuffer.read(cx).location_for_path(&path_key, cx) {
            self.editor.update(cx, |editor, cx| {
                editor.rhs_editor().update(cx, |editor, cx| {
                    editor.change_selections(
                        SelectionEffects::scroll(Autoscroll::focused()),
                        window,
                        cx,
                        |s| {
                            s.select_ranges([position..position]);
                        },
                    )
                })
            });
        } else {
            self.pending_scroll = Some(path_key);
        }
    }

    pub fn calculate_changed_lines(&self, cx: &App) -> (u32, u32) {
        self.multibuffer.read(cx).snapshot(cx).total_changed_lines()
    }

    /// Returns the total count of review comments across all hunks/files.
    pub fn total_review_comment_count(&self) -> usize {
        self.review_comment_count
    }

    /// Returns a reference to the splittable editor.
    pub fn editor(&self) -> &Entity<SplittableEditor> {
        &self.editor
    }

    fn button_states(&self, cx: &App) -> ButtonStates {
        let editor = self.editor.read(cx).rhs_editor().read(cx);
        let snapshot = self.multibuffer.read(cx).snapshot(cx);
        let prev_next = snapshot.diff_hunks().nth(1).is_some();
        let mut selection = true;

        let mut ranges = editor
            .selections
            .disjoint_anchor_ranges()
            .collect::<Vec<_>>();
        if !ranges.iter().any(|range| range.start != range.end) {
            selection = false;
            let anchor = editor.selections.newest_anchor().head();
            if let Some((_, excerpt_range)) = snapshot.excerpt_containing(anchor..anchor)
                && let Some(range) = snapshot
                    .anchor_in_buffer(excerpt_range.context.start)
                    .zip(snapshot.anchor_in_buffer(excerpt_range.context.end))
                    .map(|(start, end)| start..end)
            {
                ranges = vec![range];
            } else {
                ranges = Vec::default();
            };
        }
        let mut has_staged_hunks = false;
        let mut has_unstaged_hunks = false;
        for hunk in editor.diff_hunks_in_ranges(&ranges, &snapshot) {
            match hunk.status.secondary {
                DiffHunkSecondaryStatus::HasSecondaryHunk
                | DiffHunkSecondaryStatus::SecondaryHunkAdditionPending => {
                    has_unstaged_hunks = true;
                }
                DiffHunkSecondaryStatus::OverlapsWithSecondaryHunk => {
                    has_staged_hunks = true;
                    has_unstaged_hunks = true;
                }
                DiffHunkSecondaryStatus::NoSecondaryHunk
                | DiffHunkSecondaryStatus::SecondaryHunkRemovalPending => {
                    has_staged_hunks = true;
                }
            }
        }
        let mut stage_all = false;
        let mut unstage_all = false;
        self.workspace
            .read_with(cx, |workspace, cx| {
                if let Some(git_panel) = workspace.panel::<GitPanel>(cx) {
                    let git_panel = git_panel.read(cx);
                    stage_all = git_panel.can_stage_all();
                    unstage_all = git_panel.can_unstage_all();
                }
            })
            .ok();

        ButtonStates {
            stage: has_unstaged_hunks,
            unstage: has_staged_hunks,
            prev_next,
            selection,
            stage_all,
            unstage_all,
        }
    }

    fn handle_editor_event(
        &mut self,
        editor: &Entity<SplittableEditor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            EditorEvent::SelectionsChanged { local: true } => {
                let Some(project_path) = self.active_path(cx) else {
                    return;
                };
                let preferred_section = match self.diff_base(cx) {
                    DiffBase::Staged => Some(Section::Staged),
                    DiffBase::Unstaged => Some(Section::Unstaged),
                    DiffBase::Head | DiffBase::Merge { .. } => None,
                };
                self.workspace
                    .update(cx, |workspace, cx| {
                        if let Some(git_panel) = workspace.panel::<GitPanel>(cx) {
                            git_panel.update(cx, |git_panel, cx| {
                                git_panel.select_entry_by_path(
                                    project_path,
                                    preferred_section,
                                    window,
                                    cx,
                                )
                            })
                        }
                    })
                    .ok();
            }
            EditorEvent::Saved => {
                self._task = cx.spawn_in(window, async move |this, cx| {
                    Self::refresh(this, RefreshReason::EditorSaved, cx).await
                });
            }
            EditorEvent::OpenExcerptsRequested {
                selections_by_buffer,
                ..
            } => {
                // Reached only for the Staged view, whose read-only index
                // snapshot delegates its open. Open the editable worktree file
                // instead of the snapshot buffer (story 38).
                let project_paths: Vec<ProjectPath> = selections_by_buffer
                    .keys()
                    .filter_map(|buffer_id| {
                        self.worktree_project_path_for_buffer(*buffer_id, cx)
                    })
                    .collect();
                self.workspace
                    .update(cx, |workspace, cx| {
                        for project_path in project_paths {
                            workspace
                                .open_path(project_path, None, true, window, cx)
                                .detach_and_log_err(cx);
                        }
                    })
                    .ok();
            }
            _ => {}
        }
        if editor.focus_handle(cx).contains_focused(window, cx)
            && self.multibuffer.read(cx).is_empty()
        {
            self.focus_handle.focus(window, cx)
        }
    }

    #[instrument(skip_all)]
    fn register_buffer(
        &mut self,
        path_key: PathKey,
        file_status: FileStatus,
        buffer: Entity<Buffer>,
        diff: Entity<BufferDiff>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<BufferId> {
        let subscription = cx.subscribe_in(&diff, window, move |this, _, _, window, cx| {
            this._task = window.spawn(cx, {
                let this = cx.weak_entity();
                async |cx| Self::refresh(this, RefreshReason::DiffChanged, cx).await
            })
        });
        self.buffer_diff_subscriptions
            .insert(path_key.path.clone(), (diff.clone(), subscription));

        // TODO(split-diff) we shouldn't have a conflict addon when split
        let conflict_addon = self
            .editor
            .read(cx)
            .rhs_editor()
            .read(cx)
            .addon::<ConflictAddon>()
            .expect("project diff editor should have a conflict addon");

        let snapshot = buffer.read(cx).snapshot();
        let diff_snapshot = diff.read(cx).snapshot(cx);

        let excerpt_ranges = {
            let diff_hunk_ranges = diff_snapshot
                .hunks_intersecting_range(
                    Anchor::min_max_range_for_buffer(snapshot.remote_id()),
                    &snapshot,
                )
                .map(|diff_hunk| diff_hunk.buffer_range.to_point(&snapshot));
            let conflicts = conflict_addon
                .conflict_set(snapshot.remote_id())
                .map(|conflict_set| conflict_set.read(cx).snapshot().conflicts)
                .unwrap_or_default();
            let mut conflicts = conflicts
                .iter()
                .map(|conflict| conflict.range.to_point(&snapshot))
                .peekable();

            if conflicts.peek().is_some() {
                conflicts.collect::<Vec<_>>()
            } else {
                diff_hunk_ranges.collect()
            }
        };

        let mut needs_fold = None;

        let (was_empty, is_excerpt_newly_added) = self.editor.update(cx, |editor, cx| {
            let was_empty = editor.rhs_editor().read(cx).buffer().read(cx).is_empty();
            let is_newly_added = editor.update_excerpts_for_path(
                path_key.clone(),
                buffer,
                excerpt_ranges,
                multibuffer_context_lines(cx),
                diff,
                cx,
            );
            (was_empty, is_newly_added)
        });

        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |editor, cx| {
                if was_empty {
                    editor.change_selections(
                        SelectionEffects::no_scroll(),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges([
                                multi_buffer::Anchor::Min..multi_buffer::Anchor::Min
                            ])
                        },
                    );
                }
                if is_excerpt_newly_added
                    && (file_status.is_deleted()
                        || (file_status.is_untracked()
                            && GitPanelSettings::get_global(cx).collapse_untracked_diff))
                {
                    needs_fold = Some(snapshot.text.remote_id());
                }
            })
        });

        if self.multibuffer.read(cx).is_empty()
            && self
                .editor
                .read(cx)
                .focus_handle(cx)
                .contains_focused(window, cx)
        {
            self.focus_handle.focus(window, cx);
        } else if self.focus_handle.is_focused(window) && !self.multibuffer.read(cx).is_empty() {
            self.editor.update(cx, |editor, cx| {
                editor.focus_handle(cx).focus(window, cx);
            });
        }
        if self.pending_scroll.as_ref() == Some(&path_key) {
            self.move_to_path(path_key, window, cx);
        }

        needs_fold
    }

    #[instrument(skip(this, cx))]
    pub async fn refresh(
        this: WeakEntity<Self>,
        reason: RefreshReason,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let mut path_keys = Vec::new();
        let buffers_to_load = this.update(cx, |this, cx| {
            let (repo, buffers_to_load) = this.branch_diff.update(cx, |branch_diff, cx| {
                let load_buffers = branch_diff.load_buffers(cx);
                (branch_diff.repo().cloned(), load_buffers)
            });
            let mut previous_buffers = this
                .multibuffer
                .read(cx)
                .snapshot(cx)
                .buffers_with_paths()
                .map(|(buffer_snapshot, path_key)| (path_key.clone(), buffer_snapshot.remote_id()))
                .collect::<HashMap<_, _>>();

            if let Some(repo) = repo {
                let repo = repo.read(cx);

                path_keys = Vec::with_capacity(buffers_to_load.len());
                for entry in buffers_to_load.iter() {
                    let sort_prefix = sort_prefix(&repo, &entry.repo_path, entry.file_status, cx);
                    let path_key =
                        PathKey::with_sort_prefix(sort_prefix, entry.repo_path.as_ref().clone());
                    previous_buffers.remove(&path_key);
                    path_keys.push(path_key)
                }
            }

            this.editor.update(cx, |editor, cx| {
                for (path, buffer_id) in previous_buffers {
                    if let Some(buffer) = this.multibuffer.read(cx).buffer(buffer_id) {
                        let skip = match reason {
                            RefreshReason::DiffChanged | RefreshReason::EditorSaved => {
                                buffer.read(cx).is_dirty()
                            }
                            RefreshReason::StatusesChanged => false,
                        };
                        if skip {
                            continue;
                        }
                    }

                    this.buffer_diff_subscriptions.remove(&path.path);
                    let _span = ztracing::info_span!("remove_excerpts_for_path");
                    _span.enter();
                    editor.remove_excerpts_for_path(path, cx);
                }
            });
            buffers_to_load
        })?;

        let mut buffers_to_fold = Vec::new();

        for (entry, path_key) in buffers_to_load.into_iter().zip(path_keys) {
            if let Some((buffer, diff)) = entry.load.await.log_err() {
                // We might be lagging behind enough that all future entry.load futures are no longer pending.
                // If that is the case, this task will never yield, starving the foreground thread of execution time.
                yield_now().await;
                cx.update(|window, cx| {
                    this.update(cx, |this, cx| {
                        let multibuffer = this.multibuffer.read(cx);
                        let skip = multibuffer.buffer(buffer.read(cx).remote_id()).is_some()
                            && multibuffer
                                .diff_for(buffer.read(cx).remote_id())
                                .is_some_and(|prev_diff| prev_diff.entity_id() == diff.entity_id())
                            && match reason {
                                RefreshReason::DiffChanged | RefreshReason::EditorSaved => {
                                    buffer.read(cx).is_dirty()
                                }
                                RefreshReason::StatusesChanged => false,
                            };
                        if !skip {
                            if let Some(buffer_id) = this.register_buffer(
                                path_key,
                                entry.file_status,
                                buffer,
                                diff,
                                window,
                                cx,
                            ) {
                                buffers_to_fold.push(buffer_id);
                            }
                        }
                    })
                    .ok();
                })?;
            }
        }
        this.update(cx, |this, cx| {
            if !buffers_to_fold.is_empty() {
                this.editor.update(cx, |editor, cx| {
                    editor
                        .rhs_editor()
                        .update(cx, |editor, cx| editor.fold_buffers(buffers_to_fold, cx));
                });
            }
            this.pending_scroll.take();
            cx.notify();
        })?;

        Ok(())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn excerpt_paths(&self, cx: &App) -> Vec<std::sync::Arc<util::rel_path::RelPath>> {
        let snapshot = self
            .editor()
            .read(cx)
            .rhs_editor()
            .read(cx)
            .buffer()
            .read(cx)
            .snapshot(cx);
        snapshot
            .excerpts()
            .map(|excerpt| {
                snapshot
                    .path_for_buffer(excerpt.context.start.buffer_id)
                    .unwrap()
                    .path
                    .clone()
            })
            .collect()
    }
}

fn sort_prefix(repo: &Repository, repo_path: &RepoPath, status: FileStatus, cx: &App) -> u64 {
    let settings = GitPanelSettings::get_global(cx);

    if settings.sort_by_path && !settings.tree_view {
        TRACKED_SORT_PREFIX
    } else if repo.had_conflict_on_last_merge_head_change(repo_path) {
        CONFLICT_SORT_PREFIX
    } else if status.is_created() {
        NEW_SORT_PREFIX
    } else {
        TRACKED_SORT_PREFIX
    }
}

impl EventEmitter<EditorEvent> for ProjectDiff {}

impl Focusable for ProjectDiff {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if self.multibuffer.read(cx).is_empty() {
            self.focus_handle.clone()
        } else {
            self.editor.focus_handle(cx)
        }
    }
}

impl Item for ProjectDiff {
    type Event = EditorEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::GitBranch).color(Color::Muted))
    }

    fn to_item_events(event: &EditorEvent, f: &mut dyn FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |primary_editor, cx| {
                primary_editor.deactivated(window, cx);
            })
        });
    }

    fn navigate(
        &mut self,
        data: Arc<dyn Any + Send>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |primary_editor, cx| {
                primary_editor.navigate(data, window, cx)
            })
        })
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        match self.diff_base(cx) {
            DiffBase::Head | DiffBase::Staged | DiffBase::Unstaged => Some("Project Diff".into()),
            DiffBase::Merge { .. } => Some("Branch Diff".into()),
        }
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        Label::new(self.tab_content_text(0, cx))
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        match self.branch_diff.read(cx).diff_base() {
            DiffBase::Head => "Uncommitted Changes".into(),
            DiffBase::Staged => "Staged Changes".into(),
            DiffBase::Unstaged => "Unstaged Changes".into(),
            DiffBase::Merge { base_ref } => format!("Changes since {}", base_ref).into(),
        }
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Project Diff Opened")
    }

    fn as_searchable(&self, _: &Entity<Self>, _cx: &App) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.editor
            .read(cx)
            .rhs_editor()
            .read(cx)
            .for_each_project_item(cx, f)
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |primary_editor, _| {
                primary_editor.set_nav_history(Some(nav_history));
            })
        });
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(None);
        };
        Task::ready(Some(cx.new(|cx| {
            ProjectDiff::new(self.project.clone(), workspace, DiffBase::Head, window, cx)
        })))
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.multibuffer.read(cx).is_dirty(cx)
    }

    fn has_conflict(&self, cx: &App) -> bool {
        self.multibuffer.read(cx).has_conflict(cx)
    }

    fn can_save(&self, _: &App) -> bool {
        true
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |primary_editor, cx| {
                primary_editor.save(options, project, window, cx)
            })
        })
    }

    fn save_as(
        &mut self,
        _: Entity<Project>,
        _: ProjectPath,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> Task<Result<()>> {
        unreachable!()
    }

    fn reload(
        &mut self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |primary_editor, cx| {
                primary_editor.reload(project, window, cx)
            })
        })
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        cx: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.read(cx).rhs_editor().clone().into())
        } else if type_id == TypeId::of::<SplittableEditor>() {
            Some(self.editor.clone().into())
        } else {
            None
        }
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.added_to_workspace(workspace, window, cx)
        });
    }
}

impl Render for ProjectDiff {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.multibuffer.read(cx).is_empty();
        let is_loading = self.branch_diff.read(cx).is_tree_base_loading() || !self._task.is_ready();

        let is_branch_diff_view = matches!(self.diff_base(cx), DiffBase::Merge { .. });

        div()
            .track_focus(&self.focus_handle)
            .key_context(if is_empty { "EmptyPane" } else { "GitDiff" })
            .when(is_branch_diff_view, |this| {
                this.on_action(cx.listener(Self::review_diff))
            })
            .bg(cx.theme().colors().editor_background)
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .when(is_empty && is_loading, |el| {
                let rems = TextSize::Large.rems(cx);
                el.child(
                    Icon::new(IconName::LoadCircle)
                        .size(IconSize::Custom(rems))
                        .color(Color::Accent)
                        .with_rotate_animation(3)
                        .into_any_element(),
                )
            })
            .when(is_empty && !is_loading, |el| {
                let remote_button = if let Some(panel) = self
                    .workspace
                    .upgrade()
                    .and_then(|workspace| workspace.read(cx).panel::<GitPanel>(cx))
                {
                    panel.update(cx, |panel, cx| panel.render_remote_button(cx))
                } else {
                    None
                };
                let keybinding_focus_handle = self.focus_handle(cx);
                el.child(
                    v_flex()
                        .gap_1()
                        .child(
                            h_flex()
                                .justify_around()
                                .child(Label::new(self.empty_state_text(cx))),
                        )
                        .map(|el| match remote_button {
                            Some(button) => el.child(h_flex().justify_around().child(button)),
                            None => el.child(
                                h_flex()
                                    .justify_around()
                                    .child(Label::new("Remote up to date")),
                            ),
                        })
                        .child(
                            h_flex().justify_around().mt_1().child(
                                Button::new("project-diff-close-button", "Close")
                                    // .style(ButtonStyle::Transparent)
                                    .key_binding(KeyBinding::for_action_in(
                                        &CloseActiveItem::default(),
                                        &keybinding_focus_handle,
                                        cx,
                                    ))
                                    .on_click(move |_, window, cx| {
                                        window.focus(&keybinding_focus_handle, cx);
                                        window.dispatch_action(
                                            Box::new(CloseActiveItem::default()),
                                            cx,
                                        );
                                    }),
                            ),
                        ),
                )
            })
            .when(!is_empty, |el| el.child(self.editor.clone()))
    }
}

impl SerializableItem for ProjectDiff {
    fn serialized_item_kind() -> &'static str {
        "ProjectDiff"
    }

    fn cleanup(
        _: workspace::WorkspaceId,
        _: Vec<workspace::ItemId>,
        _: &mut Window,
        _: &mut App,
    ) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn deserialize(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: workspace::WorkspaceId,
        item_id: workspace::ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let db = persistence::ProjectDiffDb::global(cx);
        window.spawn(cx, async move |cx| {
            let diff_base = db.get_diff_base(item_id, workspace_id)?;

            let diff = cx.update(|window, cx| {
                let branch_diff = cx
                    .new(|cx| branch_diff::BranchDiff::new(diff_base, project.clone(), window, cx));
                let workspace = workspace.upgrade().context("workspace gone")?;
                anyhow::Ok(
                    cx.new(|cx| ProjectDiff::new_impl(branch_diff, project, workspace, window, cx)),
                )
            })??;

            Ok(diff)
        })
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: workspace::ItemId,
        _closing: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let diff_base = self.diff_base(cx).clone();

        let db = persistence::ProjectDiffDb::global(cx);
        Some(cx.background_spawn({
            async move {
                db.save_diff_base(item_id, workspace_id, diff_base.clone())
                    .await
            }
        }))
    }

    fn should_serialize(&self, _: &Self::Event) -> bool {
        false
    }
}

mod persistence {

    use anyhow::Context as _;
    use db::{
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };
    use project::git_store::branch_diff::DiffBase;
    use workspace::{ItemId, WorkspaceDb, WorkspaceId};

    pub struct ProjectDiffDb(ThreadSafeConnection);

    impl Domain for ProjectDiffDb {
        const NAME: &str = stringify!(ProjectDiffDb);

        const MIGRATIONS: &[&str] = &[sql!(
                CREATE TABLE project_diffs(
                    workspace_id INTEGER,
                    item_id INTEGER UNIQUE,

                    diff_base TEXT,

                    PRIMARY KEY(workspace_id, item_id),
                    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                ) STRICT;
        )];
    }

    db::static_connection!(ProjectDiffDb, [WorkspaceDb]);

    impl ProjectDiffDb {
        pub async fn save_diff_base(
            &self,
            item_id: ItemId,
            workspace_id: WorkspaceId,
            diff_base: DiffBase,
        ) -> anyhow::Result<()> {
            self.write(move |connection| {
                let sql_stmt = sql!(
                    INSERT OR REPLACE INTO project_diffs(item_id, workspace_id, diff_base) VALUES (?, ?, ?)
                );
                let diff_base_str = serde_json::to_string(&diff_base)?;
                let mut query = connection.exec_bound::<(ItemId, WorkspaceId, String)>(sql_stmt)?;
                query((item_id, workspace_id, diff_base_str)).context(format!(
                    "exec_bound failed to execute or parse for: {}",
                    sql_stmt
                ))
            })
            .await
        }

        pub fn get_diff_base(
            &self,
            item_id: ItemId,
            workspace_id: WorkspaceId,
        ) -> anyhow::Result<DiffBase> {
            let sql_stmt =
                sql!(SELECT diff_base FROM project_diffs WHERE item_id =  ?AND workspace_id =  ?);
            let diff_base_str = self.select_row_bound::<(ItemId, WorkspaceId), String>(sql_stmt)?(
                (item_id, workspace_id),
            )
            .context(::std::format!(
                "Error in get_diff_base, select_row_bound failed to execute or parse for: {}",
                sql_stmt
            ))?;
            let Some(diff_base_str) = diff_base_str else {
                return Ok(DiffBase::Head);
            };
            serde_json::from_str(&diff_base_str).context("deserializing diff base")
        }
    }
}

pub struct ProjectDiffToolbar {
    project_diff: Option<WeakEntity<ProjectDiff>>,
    workspace: WeakEntity<Workspace>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DiffFilter {
    Uncommitted,
    Staged,
    Unstaged,
    Branch,
}

impl DiffFilter {
    const ALL: [Self; 4] = [
        Self::Uncommitted,
        Self::Staged,
        Self::Unstaged,
        Self::Branch,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::Uncommitted => "Uncommitted",
            Self::Staged => "Staged",
            Self::Unstaged => "Unstaged",
            Self::Branch => "Branch",
        }
    }

    fn from_diff_base(diff_base: &DiffBase) -> Self {
        match diff_base {
            DiffBase::Head => Self::Uncommitted,
            DiffBase::Staged => Self::Staged,
            DiffBase::Unstaged => Self::Unstaged,
            DiffBase::Merge { .. } => Self::Branch,
        }
    }
}

impl ProjectDiffToolbar {
    pub fn new(workspace: &Workspace, _: &mut Context<Self>) -> Self {
        Self {
            project_diff: None,
            workspace: workspace.weak_handle(),
        }
    }

    fn project_diff(&self, _: &App) -> Option<Entity<ProjectDiff>> {
        self.project_diff.as_ref()?.upgrade()
    }

    fn selected_diff_filter(&self, cx: &App) -> Option<DiffFilter> {
        let project_diff = self.project_diff(cx)?;
        Some(DiffFilter::from_diff_base(
            project_diff.read(cx).diff_base(cx),
        ))
    }

    fn select_diff_filter(
        &mut self,
        diff_filter: DiffFilter,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(project_diff) = self.project_diff(cx) else {
            return;
        };
        project_diff.update(cx, |project_diff, cx| {
            project_diff.select_diff_filter(diff_filter, window, cx);
        });
        cx.notify();
    }

    fn dispatch_action(&self, action: &dyn Action, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(project_diff) = self.project_diff(cx) {
            project_diff.focus_handle(cx).focus(window, cx);
        }
        let action = action.boxed_clone();
        cx.defer(move |cx| {
            cx.dispatch_action(action.as_ref());
        })
    }

    fn stage_all(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.workspace
            .update(cx, |workspace, cx| {
                if let Some(panel) = workspace.panel::<GitPanel>(cx) {
                    panel.update(cx, |panel, cx| {
                        panel.stage_all(&Default::default(), window, cx);
                    });
                }
            })
            .log_err();
    }

    fn unstage_all(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.workspace
            .update(cx, |workspace, cx| {
                let Some(panel) = workspace.panel::<GitPanel>(cx) else {
                    return;
                };
                panel.update(cx, |panel, cx| {
                    panel.unstage_all(&Default::default(), window, cx);
                });
            })
            .log_err();
    }

    fn render_diff_filter_dropdown(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let selected_diff_filter = self
            .selected_diff_filter(cx)
            .unwrap_or(DiffFilter::Uncommitted);
        let toolbar = cx.entity().downgrade();
        let menu = ContextMenu::build(window, cx, move |mut menu, _, _| {
            for diff_filter in DiffFilter::ALL {
                let toolbar = toolbar.clone();
                menu = menu.toggleable_entry(
                    diff_filter.label(),
                    selected_diff_filter == diff_filter,
                    IconPosition::Start,
                    None,
                    move |window, cx| {
                        toolbar
                            .update(cx, |toolbar, cx| {
                                toolbar.select_diff_filter(diff_filter, window, cx);
                            })
                            .log_err();
                    },
                );
            }
            menu
        });

        DropdownMenu::new("project-diff-filter", selected_diff_filter.label(), menu)
            .style(DropdownStyle::Subtle)
            .into_any_element()
    }
}

impl EventEmitter<ToolbarItemEvent> for ProjectDiffToolbar {}

impl ToolbarItemView for ProjectDiffToolbar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.project_diff = active_pane_item
            .and_then(|item| item.act_as::<ProjectDiff>(cx))
            .map(|entity| entity.downgrade());
        if self.project_diff.is_some() {
            ToolbarItemLocation::PrimaryRight
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn pane_focus_update(
        &mut self,
        _pane_focused: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}

struct ButtonStates {
    stage: bool,
    unstage: bool,
    prev_next: bool,
    selection: bool,
    stage_all: bool,
    unstage_all: bool,
}

impl Render for ProjectDiffToolbar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(project_diff) = self.project_diff(cx) else {
            return div();
        };
        let focus_handle = project_diff.focus_handle(cx);
        let button_states = project_diff.read(cx).button_states(cx);
        let review_count = project_diff.read(cx).total_review_comment_count();
        let is_branch_diff = matches!(project_diff.read(cx).diff_base(cx), DiffBase::Merge { .. });

        h_group_xl()
            .my_neg_1()
            .py_1()
            .items_center()
            .flex_wrap()
            .justify_between()
            .child(self.render_diff_filter_dropdown(window, cx))
            .when(!is_branch_diff, |toolbar| {
                toolbar
                    .child(vertical_divider())
                    .child(
                        h_group_sm()
                            .when(button_states.selection, |el| {
                                el.child(
                                    Button::new("stage", "Toggle Staged")
                                        .tooltip(Tooltip::for_action_title_in(
                                            "Toggle Staged",
                                            &ToggleStaged,
                                            &focus_handle,
                                        ))
                                        .disabled(!button_states.stage && !button_states.unstage)
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.dispatch_action(&ToggleStaged, window, cx)
                                        })),
                                )
                            })
                            .when(!button_states.selection, |el| {
                                el.child(
                                    Button::new("stage", "Stage")
                                        .tooltip(Tooltip::for_action_title_in(
                                            "Stage and go to next hunk",
                                            &StageAndNext,
                                            &focus_handle,
                                        ))
                                        .disabled(
                                            !button_states.prev_next
                                                && !button_states.stage_all
                                                && !button_states.unstage_all,
                                        )
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.dispatch_action(&StageAndNext, window, cx)
                                        })),
                                )
                                .child(
                                    Button::new("unstage", "Unstage")
                                        .tooltip(Tooltip::for_action_title_in(
                                            "Unstage and go to next hunk",
                                            &UnstageAndNext,
                                            &focus_handle,
                                        ))
                                        .disabled(
                                            !button_states.prev_next
                                                && !button_states.stage_all
                                                && !button_states.unstage_all,
                                        )
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.dispatch_action(&UnstageAndNext, window, cx)
                                        })),
                                )
                            }),
                    )
                    // n.b. the only reason these arrows are here is because we don't
                    // support "undo" for staging so we need a way to go back.
                    .child(
                        h_group_sm()
                            .child(
                                IconButton::new("up", IconName::ArrowUp)
                                    .shape(ui::IconButtonShape::Square)
                                    .tooltip(Tooltip::for_action_title_in(
                                        "Go to previous hunk",
                                        &GoToPreviousHunk,
                                        &focus_handle,
                                    ))
                                    .disabled(!button_states.prev_next)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.dispatch_action(&GoToPreviousHunk, window, cx)
                                    })),
                            )
                            .child(
                                IconButton::new("down", IconName::ArrowDown)
                                    .shape(ui::IconButtonShape::Square)
                                    .tooltip(Tooltip::for_action_title_in(
                                        "Go to next hunk",
                                        &GoToHunk,
                                        &focus_handle,
                                    ))
                                    .disabled(!button_states.prev_next)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.dispatch_action(&GoToHunk, window, cx)
                                    })),
                            ),
                    )
                    .child(vertical_divider())
                    .child(
                        h_group_sm()
                            .when(
                                button_states.unstage_all && !button_states.stage_all,
                                |el| {
                                    el.child(
                                        Button::new("unstage-all", "Unstage All")
                                            .tooltip(Tooltip::for_action_title_in(
                                                "Unstage all changes",
                                                &UnstageAll,
                                                &focus_handle,
                                            ))
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.unstage_all(window, cx)
                                            })),
                                    )
                                },
                            )
                            .when(
                                !button_states.unstage_all || button_states.stage_all,
                                |el| {
                                    el.child(
                                        // todo make it so that changing to say "Unstaged"
                                        // doesn't change the position.
                                        div().child(
                                            Button::new("stage-all", "Stage All")
                                                .disabled(!button_states.stage_all)
                                                .tooltip(Tooltip::for_action_title_in(
                                                    "Stage all changes",
                                                    &StageAll,
                                                    &focus_handle,
                                                ))
                                                .on_click(cx.listener(|this, _, window, cx| {
                                                    this.stage_all(window, cx)
                                                })),
                                        ),
                                    )
                                },
                            )
                            .child(
                                Button::new("commit", "Commit")
                                    .tooltip(Tooltip::for_action_title_in(
                                        "Commit",
                                        &Commit,
                                        &focus_handle,
                                    ))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.dispatch_action(&Commit, window, cx);
                                    })),
                            ),
                    )
            })
            .when(!is_branch_diff && review_count > 0, |el| {
                el.child(vertical_divider()).child(
                    render_send_review_to_agent_button(review_count, &focus_handle).on_click(
                        cx.listener(|this, _, window, cx| {
                            this.dispatch_action(&SendReviewToAgent, window, cx)
                        }),
                    ),
                )
            })
    }
}

fn render_send_review_to_agent_button(review_count: usize, focus_handle: &FocusHandle) -> Button {
    Button::new(
        "send-review",
        format!("Send Review to Agent ({})", review_count),
    )
    .start_icon(
        Icon::new(IconName::ZedAssistant)
            .size(IconSize::Small)
            .color(Color::Muted),
    )
    .tooltip(Tooltip::for_action_title_in(
        "Send all review comments to the Agent panel",
        &SendReviewToAgent,
        focus_handle,
    ))
}

pub struct BranchDiffToolbar {
    project_diff: Option<WeakEntity<ProjectDiff>>,
}

impl BranchDiffToolbar {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { project_diff: None }
    }

    fn project_diff(&self, _: &App) -> Option<Entity<ProjectDiff>> {
        self.project_diff.as_ref()?.upgrade()
    }

    fn dispatch_action(&self, action: &dyn Action, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(project_diff) = self.project_diff(cx) {
            project_diff.focus_handle(cx).focus(window, cx);
        }
        let action = action.boxed_clone();
        cx.defer(move |cx| {
            cx.dispatch_action(action.as_ref());
        })
    }
}

impl EventEmitter<ToolbarItemEvent> for BranchDiffToolbar {}

impl ToolbarItemView for BranchDiffToolbar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.project_diff = active_pane_item
            .and_then(|item| item.act_as::<ProjectDiff>(cx))
            .filter(|item| matches!(item.read(cx).diff_base(cx), DiffBase::Merge { .. }))
            .map(|entity| entity.downgrade());
        if self.project_diff.is_some() {
            ToolbarItemLocation::PrimaryRight
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn pane_focus_update(
        &mut self,
        _pane_focused: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}

impl Render for BranchDiffToolbar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(project_diff) = self.project_diff(cx) else {
            return div();
        };
        let focus_handle = project_diff.focus_handle(cx);
        let review_count = project_diff.read(cx).total_review_comment_count();
        let (additions, deletions) = project_diff.read(cx).calculate_changed_lines(cx);
        let diff_base = project_diff.read(cx).diff_base(cx).clone();
        let DiffBase::Merge { base_ref } = diff_base else {
            return div();
        };
        let selected_base_ref = base_ref.clone();
        let base_ref_label = format!("Base: {base_ref}");
        let repository = project_diff.read(cx).branch_diff.read(cx).repo().cloned();
        let workspace = project_diff.read(cx).workspace.clone();
        let project_diff_for_picker = project_diff.downgrade();

        let is_multibuffer_empty = project_diff.read(cx).multibuffer.read(cx).is_empty();
        let is_ai_enabled = AgentSettings::get_global(cx).enabled(cx);

        let show_review_button = !is_multibuffer_empty && is_ai_enabled;

        h_group_xl()
            .my_neg_1()
            .py_1()
            .items_center()
            .flex_wrap()
            .justify_end()
            .gap_2()
            .child(
                PopoverMenu::new("branch-diff-base-branch-picker")
                    .menu(move |window, cx| {
                        let project_diff = project_diff_for_picker.clone();
                        let on_select =
                            Arc::new(move |branch: git::repository::Branch, cx: &mut App| {
                                let base_ref: SharedString = branch.name().to_owned().into();
                                project_diff
                                    .update(cx, |project_diff, cx| {
                                        let branch_diff = &mut project_diff.branch_diff;
                                        branch_diff.update(cx, |branch_diff, cx| {
                                            branch_diff
                                                .set_diff_base(DiffBase::Merge { base_ref }, cx);
                                        });
                                        cx.notify();
                                    })
                                    .ok();
                            });
                        Some(branch_picker::select_popover(
                            workspace.clone(),
                            repository.clone(),
                            Some(selected_base_ref.clone()),
                            on_select,
                            window,
                            cx,
                        ))
                    })
                    .trigger_with_tooltip(
                        Button::new("branch-diff-base-branch", base_ref_label)
                            .color(Color::Muted)
                            .end_icon(
                                Icon::new(IconName::ChevronDown)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted),
                            ),
                        Tooltip::text("Select base branch"),
                    ),
            )
            .when(!is_multibuffer_empty, |this| {
                this.child(DiffStat::new(
                    "branch-diff-stat",
                    additions as usize,
                    deletions as usize,
                ))
            })
            .when(show_review_button, |this| {
                let focus_handle = focus_handle.clone();
                this.child(Divider::vertical()).child(
                    Button::new("review-diff", "Review Diff")
                        .start_icon(
                            Icon::new(IconName::ZedAssistant)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        )
                        .key_binding(KeyBinding::for_action_in(&ReviewDiff, &focus_handle, cx))
                        .tooltip(move |_, cx| {
                            Tooltip::with_meta_in(
                                "Review Diff",
                                Some(&ReviewDiff),
                                "Send this diff for your last agent to review.",
                                &focus_handle,
                                cx,
                            )
                        })
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.dispatch_action(&ReviewDiff, window, cx);
                        })),
                )
            })
            .when(review_count > 0, |this| {
                this.child(vertical_divider()).child(
                    render_send_review_to_agent_button(review_count, &focus_handle).on_click(
                        cx.listener(|this, _, window, cx| {
                            this.dispatch_action(&SendReviewToAgent, window, cx)
                        }),
                    ),
                )
            })
    }
}

struct BranchDiffAddon {
    branch_diff: Entity<branch_diff::BranchDiff>,
}

impl Addon for BranchDiffAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn override_status_for_buffer_id(
        &self,
        buffer_id: language::BufferId,
        cx: &App,
    ) -> Option<FileStatus> {
        self.branch_diff
            .read(cx)
            .status_for_buffer_id(buffer_id, cx)
    }
}

#[cfg(test)]
mod tests {
    use collections::HashMap;
    use db::indoc;
    use editor::test::editor_test_context::{EditorTestContext, assert_state_with_diff};
    use git::status::{FileStatus, StatusCode, TrackedStatus, UnmergedStatus, UnmergedStatusCode};
    use gpui::{TestAppContext, VisualTestContext};
    use project::{FakeFs, Fs};
    use serde_json::json;
    use settings::{DiffViewStyle, SettingsStore};
    use std::{path::Path, sync::Arc};
    use unindent::Unindent as _;
    use util::{
        path,
        rel_path::{RelPath, rel_path},
    };

    use workspace::MultiWorkspace;

    use super::*;

    #[ctor::ctor]
    fn init_logger() {
        zlog::init_test();
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.editor.diff_view_style = Some(DiffViewStyle::Unified);
                });
            });
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
    }

    async fn build_project(cx: &mut TestAppContext) -> (Entity<Project>, Arc<FakeFs>) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "foo.txt": "FOO\n",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

        (project, fs)
    }

    fn project_diff_editor_has_addon<T: Addon>(diff: &Entity<ProjectDiff>, cx: &App) -> bool {
        diff.read(cx)
            .editor
            .read(cx)
            .rhs_editor()
            .read(cx)
            .addon::<T>()
            .is_some()
    }

    fn project_diff_renders_empty_hunk_controls(
        diff: &Entity<ProjectDiff>,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let editor = diff.read(cx).editor.clone();
        SplittableEditor::renders_empty_diff_hunk_controls_for_test(&editor, window, cx)
    }

    #[gpui::test]
    async fn test_project_diff_toolbar_stays_attached_for_every_diff_filter(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let (project, _) = build_project(cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace.clone(), DiffBase::Head, window, cx)
        });
        cx.run_until_parked();
        let toolbar = cx.new(|cx| ProjectDiffToolbar {
            project_diff: None,
            workspace: workspace.read(cx).weak_handle(),
        });

        for diff_base in [
            DiffBase::Head,
            DiffBase::Staged,
            DiffBase::Unstaged,
            DiffBase::Merge {
                base_ref: "main".into(),
            },
        ] {
            diff.update(cx, |diff, cx| {
                diff.branch_diff.update(cx, |branch_diff, cx| {
                    branch_diff.set_diff_base(diff_base.clone(), cx);
                });
            });

            let location = cx.update(|window, cx| {
                toolbar.update(cx, |toolbar, cx| {
                    toolbar.set_active_pane_item(Some(&diff), window, cx)
                })
            });
            assert_eq!(location, ToolbarItemLocation::PrimaryRight);
        }
    }

    #[gpui::test]
    async fn test_project_diff_reapplies_editor_addon_when_diff_filter_changes(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let (project, _) = build_project(cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace.clone(), DiffBase::Head, window, cx)
        });
        cx.run_until_parked();

        assert!(cx.update(|_, cx| project_diff_editor_has_addon::<GitPanelAddon>(&diff, cx)));
        assert!(!cx.update(|_, cx| project_diff_editor_has_addon::<BranchDiffAddon>(&diff, cx)));

        diff.update(cx, |diff, cx| {
            diff.branch_diff.update(cx, |branch_diff, cx| {
                branch_diff.set_diff_base(
                    DiffBase::Merge {
                        base_ref: "main".into(),
                    },
                    cx,
                );
            });
        });
        cx.run_until_parked();

        assert!(!cx.update(|_, cx| project_diff_editor_has_addon::<GitPanelAddon>(&diff, cx)));
        assert!(cx.update(|_, cx| project_diff_editor_has_addon::<BranchDiffAddon>(&diff, cx)));

        diff.update(cx, |diff, cx| {
            diff.branch_diff.update(cx, |branch_diff, cx| {
                branch_diff.set_diff_base(DiffBase::Staged, cx);
            });
        });
        cx.run_until_parked();

        // Staged registers both: GitPanelAddon for the stage/unstage header
        // controls, and BranchDiffAddon to supply the status badge for the
        // synthetic (unregistered) snapshot buffers.
        assert!(cx.update(|_, cx| project_diff_editor_has_addon::<GitPanelAddon>(&diff, cx)));
        assert!(cx.update(|_, cx| project_diff_editor_has_addon::<BranchDiffAddon>(&diff, cx)));

        // Switching away from Staged removes BranchDiffAddon again; Head/Unstaged
        // resolve status from the real worktree buffers via the project.
        diff.update(cx, |diff, cx| {
            diff.branch_diff.update(cx, |branch_diff, cx| {
                branch_diff.set_diff_base(DiffBase::Head, cx);
            });
        });
        cx.run_until_parked();

        assert!(cx.update(|_, cx| project_diff_editor_has_addon::<GitPanelAddon>(&diff, cx)));
        assert!(!cx.update(|_, cx| project_diff_editor_has_addon::<BranchDiffAddon>(&diff, cx)));
    }

    #[gpui::test]
    async fn test_project_diff_reapplies_hunk_controls_when_diff_filter_changes(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let (project, _) = build_project(cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace.clone(), DiffBase::Head, window, cx)
        });
        cx.run_until_parked();

        for (diff_base, hides_hunk_controls) in [
            (DiffBase::Head, false),
            (DiffBase::Staged, false),
            (DiffBase::Unstaged, false),
            (
                DiffBase::Merge {
                    base_ref: "main".into(),
                },
                true,
            ),
            (DiffBase::Head, false),
        ] {
            diff.update(cx, |diff, cx| {
                diff.branch_diff.update(cx, |branch_diff, cx| {
                    branch_diff.set_diff_base(diff_base, cx);
                });
            });
            cx.run_until_parked();

            assert_eq!(
                cx.update(|window, cx| {
                    project_diff_renders_empty_hunk_controls(&diff, window, cx)
                }),
                hides_hunk_controls
            );
        }
    }

    #[gpui::test]
    async fn test_project_diff_empty_state_text_tracks_diff_filter(cx: &mut TestAppContext) {
        init_test(cx);

        let (project, _) = build_project(cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace.clone(), DiffBase::Head, window, cx)
        });
        cx.run_until_parked();

        for (diff_base, expected) in [
            (DiffBase::Head, "No uncommitted changes"),
            (DiffBase::Staged, "No staged changes"),
            (DiffBase::Unstaged, "No unstaged changes"),
            (
                DiffBase::Merge {
                    base_ref: "origin/main".into(),
                },
                "No changes since origin/main",
            ),
        ] {
            diff.update(cx, |diff, cx| {
                diff.branch_diff.update(cx, |branch_diff, cx| {
                    branch_diff.set_diff_base(diff_base, cx);
                });
            });

            assert_eq!(
                diff.read_with(cx, |diff, cx| diff.empty_state_text(cx).to_string()),
                expected
            );
        }
    }

    #[gpui::test]
    async fn test_project_diff_toolbar_selects_uncommitted_staged_and_unstaged_filters(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let (project, _) = build_project(cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace.clone(), DiffBase::Head, window, cx)
        });
        cx.run_until_parked();
        let toolbar = cx.new(|cx| ProjectDiffToolbar {
            project_diff: Some(diff.downgrade()),
            workspace: workspace.read(cx).weak_handle(),
        });

        assert_eq!(
            toolbar.read_with(cx, |toolbar, cx| toolbar.selected_diff_filter(cx)),
            Some(DiffFilter::Uncommitted)
        );

        cx.update(|window, cx| {
            toolbar.update(cx, |toolbar, cx| {
                toolbar.select_diff_filter(DiffFilter::Staged, window, cx)
            });
        });
        assert_eq!(
            diff.read_with(cx, |diff, cx| diff.diff_base(cx).clone()),
            DiffBase::Staged
        );
        assert_eq!(
            toolbar.read_with(cx, |toolbar, cx| toolbar.selected_diff_filter(cx)),
            Some(DiffFilter::Staged)
        );

        cx.update(|window, cx| {
            toolbar.update(cx, |toolbar, cx| {
                toolbar.select_diff_filter(DiffFilter::Unstaged, window, cx)
            });
        });
        assert_eq!(
            diff.read_with(cx, |diff, cx| diff.diff_base(cx).clone()),
            DiffBase::Unstaged
        );
        assert_eq!(
            toolbar.read_with(cx, |toolbar, cx| toolbar.selected_diff_filter(cx)),
            Some(DiffFilter::Unstaged)
        );

        cx.update(|window, cx| {
            toolbar.update(cx, |toolbar, cx| {
                toolbar.select_diff_filter(DiffFilter::Uncommitted, window, cx)
            });
        });
        assert_eq!(
            diff.read_with(cx, |diff, cx| diff.diff_base(cx).clone()),
            DiffBase::Head
        );
        assert_eq!(
            toolbar.read_with(cx, |toolbar, cx| toolbar.selected_diff_filter(cx)),
            Some(DiffFilter::Uncommitted)
        );
    }

    #[gpui::test]
    async fn test_project_diff_toolbar_selects_branch_filter_after_default_branch_resolves(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let (project, _) = build_project(cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace.clone(), DiffBase::Head, window, cx)
        });
        cx.run_until_parked();
        let toolbar = cx.new(|cx| ProjectDiffToolbar {
            project_diff: Some(diff.downgrade()),
            workspace: workspace.read(cx).weak_handle(),
        });

        cx.update(|window, cx| {
            toolbar.update(cx, |toolbar, cx| {
                toolbar.select_diff_filter(DiffFilter::Branch, window, cx)
            });
        });
        assert_eq!(
            toolbar.read_with(cx, |toolbar, cx| toolbar.selected_diff_filter(cx)),
            Some(DiffFilter::Uncommitted)
        );

        cx.run_until_parked();

        assert_eq!(
            diff.read_with(cx, |diff, cx| diff.diff_base(cx).clone()),
            DiffBase::Merge {
                base_ref: "origin/main".into()
            }
        );
        assert_eq!(
            toolbar.read_with(cx, |toolbar, cx| toolbar.selected_diff_filter(cx)),
            Some(DiffFilter::Branch)
        );
    }

    #[gpui::test]
    async fn test_project_diff_toolbar_leaves_previous_filter_active_when_branch_fails(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "foo.txt": "FOO\n",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace.clone(), DiffBase::Head, window, cx)
        });
        cx.run_until_parked();
        let toolbar = cx.new(|cx| ProjectDiffToolbar {
            project_diff: Some(diff.downgrade()),
            workspace: workspace.read(cx).weak_handle(),
        });

        cx.update(|window, cx| {
            toolbar.update(cx, |toolbar, cx| {
                toolbar.select_diff_filter(DiffFilter::Staged, window, cx);
                toolbar.select_diff_filter(DiffFilter::Branch, window, cx);
            });
        });
        cx.run_until_parked();

        assert_eq!(
            diff.read_with(cx, |diff, cx| diff.diff_base(cx).clone()),
            DiffBase::Staged
        );
        assert_eq!(
            toolbar.read_with(cx, |toolbar, cx| toolbar.selected_diff_filter(cx)),
            Some(DiffFilter::Staged)
        );
        assert!(!workspace.read_with(cx, |workspace, _| workspace.notification_ids().is_empty()));
    }

    #[gpui::test]
    async fn test_project_diff_toolbar_ignores_stale_branch_filter_resolution(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let (project, _) = build_project(cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace.clone(), DiffBase::Head, window, cx)
        });
        cx.run_until_parked();
        let toolbar = cx.new(|cx| ProjectDiffToolbar {
            project_diff: Some(diff.downgrade()),
            workspace: workspace.read(cx).weak_handle(),
        });

        cx.update(|window, cx| {
            toolbar.update(cx, |toolbar, cx| {
                toolbar.select_diff_filter(DiffFilter::Branch, window, cx);
                toolbar.select_diff_filter(DiffFilter::Staged, window, cx);
            });
        });
        cx.run_until_parked();

        assert_eq!(
            diff.read_with(cx, |diff, cx| diff.diff_base(cx).clone()),
            DiffBase::Staged
        );
        assert_eq!(
            toolbar.read_with(cx, |toolbar, cx| toolbar.selected_diff_filter(cx)),
            Some(DiffFilter::Staged)
        );
    }

    #[gpui::test]
    async fn test_staged_filter_shows_read_only_index_snapshot_without_worktree_leak(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        // HEAD, index, and worktree all differ. The Staged filter must display
        // the git index as a read-only snapshot, never the worktree's unstaged
        // edit.
        let committed_contents = "one\ntwo\nthree\nfour\nfive\n";
        let staged_contents = "one\nTWO STAGED\nthree\nfour\nfive\n";
        let worktree_contents = "one\nTWO STAGED\nthree\nFOUR UNSTAGED\nfive\n";

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "file.txt": worktree_contents,
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        fs.set_head_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", committed_contents.into())],
            "deadbeef",
        );
        fs.set_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", staged_contents.into())],
        );

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, DiffBase::Staged, window, cx)
        });
        cx.run_until_parked();

        let staged_buffer = diff.read_with(cx, |diff, cx| {
            diff.multibuffer
                .read(cx)
                .all_buffers()
                .into_iter()
                .next()
                .expect("staged diff should display the file")
        });

        // The displayed buffer *is* the index content (read-only) — not the
        // worktree, so the unstaged "FOUR UNSTAGED" edit is absent.
        staged_buffer.read_with(cx, |buffer, _| {
            assert_eq!(buffer.text(), staged_contents);
            assert_eq!(buffer.capability(), Capability::ReadOnly);
        });

        // The editor itself stays writable so the inline stage/unstage hunk
        // controls still render; only the index buffer is read-only.
        diff.read_with(cx, |diff, cx| {
            assert!(!diff.editor.read(cx).rhs_editor().read(cx).read_only(cx));
        });

        // Editing the worktree out of band must not leak into the staged view.
        fs.save(
            path!("/project/file.txt").as_ref(),
            &"one\nTWO STAGED\nthree\nFOUR UNSTAGED\nSIX WORKTREE\n".into(),
            Default::default(),
        )
        .await
        .unwrap();
        cx.run_until_parked();

        let staged_buffer = diff.read_with(cx, |diff, cx| {
            diff.multibuffer
                .read(cx)
                .all_buffers()
                .into_iter()
                .next()
                .expect("staged diff should still display the file")
        });
        staged_buffer.read_with(cx, |buffer, _| {
            assert_eq!(buffer.text(), staged_contents);
        });
    }

    #[gpui::test]
    async fn test_staged_filter_shows_file_status_badge(cx: &mut TestAppContext) {
        init_test(cx);

        // A fully-staged modification: HEAD differs from the index, and the
        // worktree matches the index. The staged header must surface the file's
        // git status (Modified) just like the unstaged view, even though the
        // snapshot buffer is synthetic and unregistered.
        let committed_contents = "one\ntwo\nthree\n";
        let staged_contents = "one\nTWO STAGED\nthree\n";

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "file.txt": staged_contents,
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        fs.set_head_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", committed_contents.into())],
            "deadbeef",
        );
        fs.set_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", staged_contents.into())],
        );

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, DiffBase::Staged, window, cx)
        });
        cx.run_until_parked();

        let staged_buffer = diff.read_with(cx, |diff, cx| {
            diff.multibuffer
                .read(cx)
                .all_buffers()
                .into_iter()
                .next()
                .expect("staged diff should display the file")
        });
        let buffer_id = staged_buffer.read_with(cx, |buffer, _| buffer.remote_id());

        let status = diff.read_with(cx, |diff, cx| {
            diff.editor
                .read(cx)
                .rhs_editor()
                .read(cx)
                .status_for_buffer_id(buffer_id, cx)
        });
        assert_eq!(
            status,
            Some(FileStatus::Tracked(TrackedStatus {
                index_status: StatusCode::Modified,
                worktree_status: StatusCode::Unmodified,
            })),
            "staged snapshot header should show the file's staged git status",
        );
    }

    #[gpui::test]
    async fn test_staged_filter_rejects_inline_text_edits(cx: &mut TestAppContext) {
        init_test(cx);

        // A file with staged changes (index differs from HEAD).
        let committed_contents = "one\ntwo\nthree\n";
        let staged_contents = "one\nTWO STAGED\nthree\n";

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "file.txt": staged_contents,
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        fs.set_head_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", committed_contents.into())],
            "deadbeef",
        );
        fs.set_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", staged_contents.into())],
        );

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, DiffBase::Staged, window, cx)
        });
        cx.run_until_parked();

        let staged_buffer = diff.read_with(cx, |diff, cx| {
            diff.multibuffer
                .read(cx)
                .all_buffers()
                .into_iter()
                .next()
                .expect("staged diff should display the file")
        });

        // Typing into the index excerpt is rejected by the per-buffer read-only
        // capability, even though the editor itself is writable.
        let editor = diff.read_with(cx, |diff, cx| diff.editor.read(cx).rhs_editor().clone());
        editor.update_in(cx, |editor, window, cx| {
            let end = editor.buffer().read(cx).len(cx);
            editor.change_selections(Default::default(), window, cx, |s| {
                s.select_ranges([end..end]);
            });
            editor.handle_input("X", window, cx);
        });

        staged_buffer.read_with(cx, |buffer, _| {
            assert_eq!(buffer.text(), staged_contents);
        });
    }

    #[gpui::test]
    async fn test_unstaged_filter_accepts_inline_text_edits(cx: &mut TestAppContext) {
        init_test(cx);

        // A file with an unstaged worktree edit and nothing staged: it appears
        // in the Unstaged filter as the live, editable worktree buffer.
        let committed_contents = "one\ntwo\nthree\n";
        let worktree_contents = "ONE WORKTREE\ntwo\nthree\n";

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "file.txt": worktree_contents,
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        fs.set_head_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", committed_contents.into())],
            "deadbeef",
        );
        fs.set_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", committed_contents.into())],
        );

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, DiffBase::Unstaged, window, cx)
        });
        cx.run_until_parked();

        let worktree_buffer = diff.read_with(cx, |diff, cx| {
            diff.multibuffer
                .read(cx)
                .all_buffers()
                .into_iter()
                .next()
                .expect("unstaged diff should display the file")
        });

        // The Unstaged filter keeps the live worktree buffer, so it stays
        // editable — the read-only scope is Staged-only.
        worktree_buffer.read_with(cx, |buffer, _| {
            assert_eq!(buffer.capability(), Capability::ReadWrite);
        });

        let editor = diff.read_with(cx, |diff, cx| diff.editor.read(cx).rhs_editor().clone());
        editor.update_in(cx, |editor, window, cx| {
            let end = editor.buffer().read(cx).len(cx);
            editor.change_selections(Default::default(), window, cx, |s| {
                s.select_ranges([end..end]);
            });
            editor.handle_input("X", window, cx);
        });

        worktree_buffer.read_with(cx, |buffer, _| {
            assert!(
                buffer.text().contains('X'),
                "edit should be accepted, got {:?}",
                buffer.text()
            );
        });
    }

    #[gpui::test]
    async fn test_staged_filter_reloads_when_index_changes(cx: &mut TestAppContext) {
        init_test(cx);

        // Partially-staged file: line 1 staged, line 2 only in the worktree.
        let committed_contents = "one\ntwo\nthree\n";
        let staged_contents = "one\nTWO STAGED\nthree\n";
        let restaged_contents = "one\nTWO STAGED\nTHREE STAGED\n";

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "file.txt": restaged_contents,
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        fs.set_head_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", committed_contents.into())],
            "deadbeef",
        );
        fs.set_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", staged_contents.into())],
        );

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, DiffBase::Staged, window, cx)
        });
        cx.run_until_parked();

        let staged_text = |diff: &Entity<ProjectDiff>, cx: &mut VisualTestContext| {
            diff.read_with(cx, |diff, cx| {
                diff.multibuffer
                    .read(cx)
                    .all_buffers()
                    .into_iter()
                    .next()
                    .map(|buffer| buffer.read(cx).text())
            })
        };

        assert_eq!(staged_text(&diff, cx).as_deref(), Some(staged_contents));

        // Simulate `git add` of the second change directly against the index.
        fs.set_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", restaged_contents.into())],
        );
        cx.run_until_parked();
        assert_eq!(
            staged_text(&diff, cx).as_deref(),
            Some(restaged_contents),
            "staged snapshot should reload after the index changes"
        );

        // Simulate `git reset`: the index matches HEAD, so the file leaves the
        // Staged filter entirely.
        fs.set_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", committed_contents.into())],
        );
        cx.run_until_parked();
        assert!(
            diff.read_with(cx, |diff, cx| diff.multibuffer.read(cx).is_empty()),
            "fully-unstaged file should leave the Staged view"
        );
    }

    #[gpui::test]
    async fn test_staged_filter_active_path_resolves_for_index_buffer(cx: &mut TestAppContext) {
        init_test(cx);

        let committed_contents = "one\ntwo\nthree\n";
        let staged_contents = "one\nTWO STAGED\nthree\n";

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "file.txt": staged_contents,
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        fs.set_head_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", committed_contents.into())],
            "deadbeef",
        );
        fs.set_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", staged_contents.into())],
        );

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, DiffBase::Staged, window, cx)
        });
        cx.run_until_parked();

        // The focused index excerpt is file-less; active_path must still resolve
        // it to the worktree path via the excerpt's repo path, rather than
        // returning None.
        let active_path = diff.read_with(cx, |diff, cx| diff.active_path(cx));
        assert_eq!(
            active_path.map(|path| path.path),
            Some(rel_path("file.txt").into_arc()),
        );
    }

    #[gpui::test]
    async fn test_staged_filter_save_and_reload_are_safe_noops(cx: &mut TestAppContext) {
        init_test(cx);

        let committed_contents = "one\ntwo\nthree\n";
        let staged_contents = "one\nTWO STAGED\nthree\n";

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "file.txt": staged_contents,
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        fs.set_head_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", committed_contents.into())],
            "deadbeef",
        );
        fs.set_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", staged_contents.into())],
        );

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, DiffBase::Staged, window, cx)
        });
        cx.run_until_parked();

        let index_buffer = diff.read_with(cx, |diff, cx| {
            diff.multibuffer
                .read(cx)
                .all_buffers()
                .into_iter()
                .next()
                .expect("staged diff should display the file")
        });

        // Saving the diff is a no-op: the read-only index buffer is never dirty,
        // so the file-less save path is not exercised and does not error.
        let editor = diff.read_with(cx, |diff, cx| diff.editor.read(cx).rhs_editor().clone());
        editor
            .update_in(cx, |editor, window, cx| {
                editor.save(SaveOptions::default(), project.clone(), window, cx)
            })
            .await
            .expect("saving the staged diff should be a safe no-op");
        cx.run_until_parked();

        // Reloading the file-less index buffer is a no-op (there is no file to
        // read), and must not error.
        index_buffer
            .update(cx, |buffer, cx| buffer.reload(cx))
            .await
            .ok();
        cx.run_until_parked();

        index_buffer.read_with(cx, |buffer, _| {
            assert_eq!(buffer.text(), staged_contents);
            assert_eq!(buffer.capability(), Capability::ReadOnly);
        });
    }

    #[gpui::test]
    async fn test_staged_filter_inline_unstage_writes_the_index(cx: &mut TestAppContext) {
        init_test(cx);

        // Fully-staged file: HEAD differs from index, worktree matches index.
        let committed_contents = "one\ntwo\nthree\n";
        let staged_contents = "one\nTWO STAGED\nthree\n";

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "file.txt": staged_contents,
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        fs.set_head_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", committed_contents.into())],
            "deadbeef",
        );
        fs.set_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", staged_contents.into())],
        );

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        cx.run_until_parked();

        // Open the Staged diff as the active item in the workspace so its editor
        // paints and registers the staging actions (toggle_staged is registered
        // during element paint, not at construction).
        cx.focus(&workspace);
        workspace.update_in(cx, |workspace, window, cx| {
            ProjectDiff::deploy_at(workspace, None, DiffBase::Staged, window, cx);
        });
        cx.run_until_parked();

        let item = workspace.update(cx, |workspace, cx| {
            workspace.active_item_as::<ProjectDiff>(cx).unwrap()
        });
        cx.focus(&item);
        let editor = item.read_with(cx, |item, cx| item.editor.read(cx).rhs_editor().clone());

        // Unstage the hunk from the in-editor control. This must resolve the
        // file-less index excerpt to its repo path and write the git index,
        // reverting the hunk toward HEAD (stories 37/41).
        let mut cx = EditorTestContext::for_editor_in(editor, cx).await;
        cx.dispatch_action(editor::actions::SelectAll);
        cx.dispatch_action(ToggleStaged);
        cx.run_until_parked();

        let index_contents = fs
            .with_git_state(path!("/project/.git").as_ref(), false, |state| {
                state
                    .index_contents
                    .get(&RepoPath::from_rel_path(rel_path("file.txt")))
                    .cloned()
            })
            .unwrap();
        assert_eq!(
            index_contents.as_deref(),
            Some(committed_contents),
            "inline unstage must write the git index back toward HEAD"
        );
    }

    #[gpui::test]
    async fn test_staged_filter_panel_unstage_writes_the_index(cx: &mut TestAppContext) {
        init_test(cx);

        // Fully-staged file: HEAD differs from index, worktree matches index.
        let committed_contents = "one\ntwo\nthree\n";
        let staged_contents = "one\nTWO STAGED\nthree\n";

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "file.txt": staged_contents,
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        fs.set_head_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", committed_contents.into())],
            "deadbeef",
        );
        fs.set_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", staged_contents.into())],
        );

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, DiffBase::Staged, window, cx)
        });
        cx.run_until_parked();

        // Unstage through the repository's explicit-`RepoPath` path -- the same
        // path the git panel and section-header `-` control reach via
        // `change_file_stage`. It must stay unaffected by the file-less index
        // snapshot machinery (it never touches the override map).
        let repository = project
            .read_with(cx, |project, cx| {
                project.git_store().read(cx).active_repository()
            })
            .expect("active repository");
        repository
            .update(cx, |repository, cx| {
                repository.unstage_entries(vec![RepoPath::from_rel_path(rel_path("file.txt"))], cx)
            })
            .await
            .expect("unstage entries");
        cx.run_until_parked();

        let index_contents = fs
            .with_git_state(path!("/project/.git").as_ref(), false, |state| {
                state
                    .index_contents
                    .get(&RepoPath::from_rel_path(rel_path("file.txt")))
                    .cloned()
            })
            .unwrap();
        assert_eq!(
            index_contents.as_deref(),
            Some(committed_contents),
            "panel/header unstage must write the git index back toward HEAD"
        );

        // The open Staged snapshot stays consistent: once the index matches HEAD
        // the file is fully unstaged and leaves the Staged view (story 39).
        assert!(
            diff.read_with(cx, |diff, cx| diff.multibuffer.read(cx).is_empty()),
            "fully-unstaged file should leave the Staged view after a panel unstage"
        );
    }

    #[gpui::test]
    async fn test_staged_filter_open_to_edit_opens_worktree_file(cx: &mut TestAppContext) {
        init_test(cx);

        // Fully-staged file: HEAD differs from index, worktree matches index.
        let committed_contents = "one\ntwo\nthree\n";
        let staged_contents = "one\nTWO STAGED\nthree\n";

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "file.txt": staged_contents,
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        fs.set_head_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", committed_contents.into())],
            "deadbeef",
        );
        fs.set_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("file.txt", staged_contents.into())],
        );

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        cx.run_until_parked();

        cx.focus(&workspace);
        workspace.update_in(cx, |workspace, window, cx| {
            ProjectDiff::deploy_at(workspace, None, DiffBase::Staged, window, cx);
        });
        cx.run_until_parked();

        let item = workspace.update(cx, |workspace, cx| {
            workspace.active_item_as::<ProjectDiff>(cx).unwrap()
        });
        let editor = item.read_with(cx, |item, cx| item.editor.read(cx).rhs_editor().clone());

        // Activating a hunk in the read-only Staged view must open the actual,
        // editable worktree file (story 38) -- not the file-less index snapshot
        // -- so that subsequent edits land as unstaged changes.
        editor.update_in(cx, |editor, window, cx| {
            editor.select_all(&editor::actions::SelectAll, window, cx);
            editor.open_excerpts(&editor::actions::OpenExcerpts, window, cx);
        });
        cx.run_until_parked();

        let opened = workspace
            .update(cx, |workspace, cx| workspace.active_item_as::<Editor>(cx))
            .expect("activating a staged hunk should open an editable worktree editor");
        opened.read_with(cx, |opened, cx| {
            let buffer = opened
                .buffer()
                .read(cx)
                .as_singleton()
                .expect("worktree file opens as a singleton buffer");
            let buffer = buffer.read(cx);
            assert!(
                buffer.file().is_some(),
                "opened worktree file must be backed by a file"
            );
            assert_eq!(
                buffer.capability(),
                Capability::ReadWrite,
                "opened worktree file must be editable"
            );
        });
    }

    #[gpui::test]
    async fn test_save_after_restore(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "foo.txt": "FOO\n",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

        fs.set_head_for_repo(
            path!("/project/.git").as_ref(),
            &[("foo.txt", "foo\n".into())],
            "deadbeef",
        );
        fs.set_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("foo.txt", "foo\n".into())],
        );

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, DiffBase::Head, window, cx)
        });
        cx.run_until_parked();

        let editor = diff.read_with(cx, |diff, cx| diff.editor.read(cx).rhs_editor().clone());
        assert_state_with_diff(
            &editor,
            cx,
            &"
                - ˇfoo
                + FOO
            "
            .unindent(),
        );

        editor
            .update_in(cx, |editor, window, cx| {
                editor.git_restore(&Default::default(), window, cx);
                editor.save(SaveOptions::default(), project.clone(), window, cx)
            })
            .await
            .unwrap();
        cx.run_until_parked();

        assert_state_with_diff(&editor, cx, &"ˇ".unindent());

        let text = String::from_utf8(fs.read_file_sync("/project/foo.txt").unwrap()).unwrap();
        assert_eq!(text, "foo\n");
    }

    #[gpui::test]
    async fn test_scroll_to_beginning_with_deletion(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "bar": "BAR\n",
                "foo": "FOO\n",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, DiffBase::Head, window, cx)
        });
        cx.run_until_parked();

        fs.set_head_and_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("bar", "bar\n".into()), ("foo", "foo\n".into())],
        );
        cx.run_until_parked();

        let editor = cx.update_window_entity(&diff, |diff, window, cx| {
            diff.move_to_path(
                PathKey::with_sort_prefix(TRACKED_SORT_PREFIX, rel_path("foo").into_arc()),
                window,
                cx,
            );
            diff.editor.read(cx).rhs_editor().clone()
        });
        assert_state_with_diff(
            &editor,
            cx,
            &"
                - bar
                + BAR

                - ˇfoo
                + FOO
            "
            .unindent(),
        );

        let editor = cx.update_window_entity(&diff, |diff, window, cx| {
            diff.move_to_path(
                PathKey::with_sort_prefix(TRACKED_SORT_PREFIX, rel_path("bar").into_arc()),
                window,
                cx,
            );
            diff.editor.read(cx).rhs_editor().clone()
        });
        assert_state_with_diff(
            &editor,
            cx,
            &"
                - ˇbar
                + BAR

                - foo
                + FOO
            "
            .unindent(),
        );
    }

    #[gpui::test]
    async fn test_hunks_after_restore_then_modify(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "foo": "modified\n",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        fs.set_head_for_repo(
            path!("/project/.git").as_ref(),
            &[("foo", "original\n".into())],
            "deadbeef",
        );

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/project/foo"), cx)
            })
            .await
            .unwrap();
        let buffer_editor = cx.new_window_entity(|window, cx| {
            Editor::for_buffer(buffer, Some(project.clone()), window, cx)
        });
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, DiffBase::Head, window, cx)
        });
        cx.run_until_parked();

        let diff_editor = diff.read_with(cx, |diff, cx| diff.editor.read(cx).rhs_editor().clone());

        assert_state_with_diff(
            &diff_editor,
            cx,
            &"
                - ˇoriginal
                + modified
            "
            .unindent(),
        );

        let prev_buffer_hunks =
            cx.update_window_entity(&buffer_editor, |buffer_editor, window, cx| {
                let snapshot = buffer_editor.snapshot(window, cx);
                let snapshot = &snapshot.buffer_snapshot();
                let prev_buffer_hunks = buffer_editor
                    .diff_hunks_in_ranges(&[editor::Anchor::Min..editor::Anchor::Max], snapshot)
                    .collect::<Vec<_>>();
                buffer_editor.git_restore(&Default::default(), window, cx);
                prev_buffer_hunks
            });
        assert_eq!(prev_buffer_hunks.len(), 1);
        cx.run_until_parked();

        let new_buffer_hunks =
            cx.update_window_entity(&buffer_editor, |buffer_editor, window, cx| {
                let snapshot = buffer_editor.snapshot(window, cx);
                let snapshot = &snapshot.buffer_snapshot();
                buffer_editor
                    .diff_hunks_in_ranges(&[editor::Anchor::Min..editor::Anchor::Max], snapshot)
                    .collect::<Vec<_>>()
            });
        assert_eq!(new_buffer_hunks.as_slice(), &[]);

        cx.update_window_entity(&buffer_editor, |buffer_editor, window, cx| {
            buffer_editor.set_text("different\n", window, cx);
            buffer_editor.save(
                SaveOptions {
                    format: false,
                    force_format: false,
                    autosave: false,
                },
                project.clone(),
                window,
                cx,
            )
        })
        .await
        .unwrap();

        cx.run_until_parked();

        cx.update_window_entity(&buffer_editor, |buffer_editor, window, cx| {
            buffer_editor.expand_all_diff_hunks(&Default::default(), window, cx);
        });

        assert_state_with_diff(
            &buffer_editor,
            cx,
            &"
                - original
                + different
                  ˇ"
            .unindent(),
        );

        assert_state_with_diff(
            &diff_editor,
            cx,
            &"
                - ˇoriginal
                + different
            "
            .unindent(),
        );
    }

    use crate::{
        conflict_view::resolve_conflict,
        project_diff::{self, ProjectDiff},
    };

    #[gpui::test]
    async fn test_go_to_prev_hunk_multibuffer(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/a"),
            json!({
                ".git": {},
                "a.txt": "created\n",
                "b.txt": "really changed\n",
                "c.txt": "unchanged\n"
            }),
        )
        .await;

        fs.set_head_and_index_for_repo(
            Path::new(path!("/a/.git")),
            &[
                ("b.txt", "before\n".to_string()),
                ("c.txt", "unchanged\n".to_string()),
                ("d.txt", "deleted\n".to_string()),
            ],
        );

        let project = Project::test(fs, [Path::new(path!("/a"))], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        cx.run_until_parked();

        cx.focus(&workspace);
        cx.update(|window, cx| {
            window.dispatch_action(project_diff::Diff.boxed_clone(), cx);
        });

        cx.run_until_parked();

        let item = workspace.update(cx, |workspace, cx| {
            workspace.active_item_as::<ProjectDiff>(cx).unwrap()
        });
        cx.focus(&item);
        let editor = item.read_with(cx, |item, cx| item.editor.read(cx).rhs_editor().clone());

        let mut cx = EditorTestContext::for_editor_in(editor, cx).await;

        cx.assert_excerpts_with_selections(indoc!(
            "
            [EXCERPT]
            before
            really changed
            [EXCERPT]
            [FOLDED]
            [EXCERPT]
            ˇcreated
        "
        ));

        cx.dispatch_action(editor::actions::GoToPreviousHunk);

        cx.assert_excerpts_with_selections(indoc!(
            "
            [EXCERPT]
            before
            really changed
            [EXCERPT]
            ˇ[FOLDED]
            [EXCERPT]
            created
        "
        ));

        cx.dispatch_action(editor::actions::GoToPreviousHunk);

        cx.assert_excerpts_with_selections(indoc!(
            "
            [EXCERPT]
            ˇbefore
            really changed
            [EXCERPT]
            [FOLDED]
            [EXCERPT]
            created
        "
        ));
    }

    #[gpui::test]
    async fn test_excerpts_splitting_after_restoring_the_middle_excerpt(cx: &mut TestAppContext) {
        init_test(cx);

        let git_contents = indoc! {r#"
            #[rustfmt::skip]
            fn main() {
                let x = 0.0; // this line will be removed
                // 1
                // 2
                // 3
                let y = 0.0; // this line will be removed
                // 1
                // 2
                // 3
                let arr = [
                    0.0, // this line will be removed
                    0.0, // this line will be removed
                    0.0, // this line will be removed
                    0.0, // this line will be removed
                ];
            }
        "#};
        let buffer_contents = indoc! {"
            #[rustfmt::skip]
            fn main() {
                // 1
                // 2
                // 3
                // 1
                // 2
                // 3
                let arr = [
                ];
            }
        "};

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/a"),
            json!({
                ".git": {},
                "main.rs": buffer_contents,
            }),
        )
        .await;

        fs.set_head_and_index_for_repo(
            Path::new(path!("/a/.git")),
            &[("main.rs", git_contents.to_owned())],
        );

        let project = Project::test(fs, [Path::new(path!("/a"))], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        cx.run_until_parked();

        cx.focus(&workspace);
        cx.update(|window, cx| {
            window.dispatch_action(project_diff::Diff.boxed_clone(), cx);
        });

        cx.run_until_parked();

        let item = workspace.update(cx, |workspace, cx| {
            workspace.active_item_as::<ProjectDiff>(cx).unwrap()
        });
        cx.focus(&item);
        let editor = item.read_with(cx, |item, cx| item.editor.read(cx).rhs_editor().clone());

        let mut cx = EditorTestContext::for_editor_in(editor, cx).await;

        cx.assert_excerpts_with_selections(&format!("[EXCERPT]\nˇ{git_contents}"));

        cx.dispatch_action(editor::actions::GoToHunk);
        cx.dispatch_action(editor::actions::GoToHunk);
        cx.dispatch_action(git::Restore);
        cx.dispatch_action(editor::actions::MoveToBeginning);

        cx.assert_excerpts_with_selections(&format!("[EXCERPT]\nˇ{git_contents}"));
    }

    #[gpui::test]
    async fn test_saving_resolved_conflicts(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "foo": "<<<<<<< x\nours\n=======\ntheirs\n>>>>>>> y\n",
            }),
        )
        .await;
        fs.set_status_for_repo(
            Path::new(path!("/project/.git")),
            &[(
                "foo",
                UnmergedStatus {
                    first_head: UnmergedStatusCode::Updated,
                    second_head: UnmergedStatusCode::Updated,
                }
                .into(),
            )],
        );
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, DiffBase::Head, window, cx)
        });
        cx.run_until_parked();

        cx.update(|window, cx| {
            let editor = diff.read(cx).editor.read(cx).rhs_editor().clone();
            let excerpts = editor
                .read(cx)
                .buffer()
                .read(cx)
                .snapshot(cx)
                .excerpts()
                .collect::<Vec<_>>();
            assert_eq!(excerpts.len(), 1);
            let buffer = editor
                .read(cx)
                .buffer()
                .read(cx)
                .all_buffers()
                .into_iter()
                .next()
                .unwrap();
            let buffer_id = buffer.read(cx).remote_id();
            let conflict_set = diff
                .read(cx)
                .editor
                .read(cx)
                .rhs_editor()
                .read(cx)
                .addon::<ConflictAddon>()
                .unwrap()
                .conflict_set(buffer_id)
                .unwrap();
            assert!(conflict_set.read(cx).has_conflict);
            let snapshot = conflict_set.read(cx).snapshot();
            assert_eq!(snapshot.conflicts.len(), 1);

            let ours_range = snapshot.conflicts[0].ours.clone();

            resolve_conflict(
                editor.downgrade(),
                snapshot.conflicts[0].clone(),
                vec![ours_range],
                window,
                cx,
            )
        })
        .await;

        let contents = fs.read_file_sync(path!("/project/foo")).unwrap();
        let contents = String::from_utf8(contents).unwrap();
        assert_eq!(contents, "ours\n");
    }

    #[gpui::test(iterations = 50)]
    async fn test_split_diff_conflict_path_transition_with_dirty_buffer_invalid_anchor_panics(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.editor.diff_view_style = Some(DiffViewStyle::Split);
                });
            });
        });

        let build_conflict_text: fn(usize) -> String = |tag: usize| {
            let mut lines = (0..80)
                .map(|line_index| format!("line {line_index}"))
                .collect::<Vec<_>>();
            for offset in [5usize, 20, 37, 61] {
                lines[offset] = format!("base-{tag}-line-{offset}");
            }
            format!("{}\n", lines.join("\n"))
        };
        let initial_conflict_text = build_conflict_text(0);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "helper.txt": "same\n",
                "conflict.txt": initial_conflict_text,
            }),
        )
        .await;
        fs.with_git_state(path!("/project/.git").as_ref(), true, |state| {
            state
                .refs
                .insert("MERGE_HEAD".into(), "conflict-head".into());
        })
        .unwrap();
        fs.set_status_for_repo(
            path!("/project/.git").as_ref(),
            &[(
                "conflict.txt",
                FileStatus::Unmerged(UnmergedStatus {
                    first_head: UnmergedStatusCode::Updated,
                    second_head: UnmergedStatusCode::Updated,
                }),
            )],
        );
        fs.set_merge_base_content_for_repo(
            path!("/project/.git").as_ref(),
            &[
                ("conflict.txt", build_conflict_text(1)),
                ("helper.txt", "same\n".to_string()),
            ],
        );

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let _project_diff = cx
            .update(|window, cx| {
                ProjectDiff::new_with_default_branch(project.clone(), workspace, window, cx)
            })
            .await
            .unwrap();
        cx.run_until_parked();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/project/conflict.txt"), cx)
            })
            .await
            .unwrap();
        buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "dirty\n")], None, cx));
        assert!(buffer.read_with(cx, |buffer, _| buffer.is_dirty()));
        cx.run_until_parked();

        cx.update(|window, cx| {
            let fs = fs.clone();
            window
                .spawn(cx, async move |cx| {
                    cx.background_executor().simulate_random_delay().await;
                    fs.with_git_state(path!("/project/.git").as_ref(), true, |state| {
                        state.refs.insert("HEAD".into(), "head-1".into());
                        state.refs.remove("MERGE_HEAD");
                    })
                    .unwrap();
                    fs.set_status_for_repo(
                        path!("/project/.git").as_ref(),
                        &[
                            (
                                "conflict.txt",
                                FileStatus::Tracked(TrackedStatus {
                                    index_status: git::status::StatusCode::Modified,
                                    worktree_status: git::status::StatusCode::Modified,
                                }),
                            ),
                            (
                                "helper.txt",
                                FileStatus::Tracked(TrackedStatus {
                                    index_status: git::status::StatusCode::Modified,
                                    worktree_status: git::status::StatusCode::Modified,
                                }),
                            ),
                        ],
                    );
                    // FakeFs assigns deterministic OIDs by entry position; flipping order churns
                    // conflict diff identity without reaching into ProjectDiff internals.
                    fs.set_merge_base_content_for_repo(
                        path!("/project/.git").as_ref(),
                        &[
                            ("helper.txt", "helper-base\n".to_string()),
                            ("conflict.txt", build_conflict_text(2)),
                        ],
                    );
                })
                .detach();
        });

        cx.update(|window, cx| {
            let buffer = buffer.clone();
            window
                .spawn(cx, async move |cx| {
                    cx.background_executor().simulate_random_delay().await;
                    for edit_index in 0..10 {
                        if edit_index > 0 {
                            cx.background_executor().simulate_random_delay().await;
                        }
                        buffer.update(cx, |buffer, cx| {
                            let len = buffer.len();
                            if edit_index % 2 == 0 {
                                buffer.edit(
                                    [(0..0, format!("status-burst-head-{edit_index}\n"))],
                                    None,
                                    cx,
                                );
                            } else {
                                buffer.edit(
                                    [(len..len, format!("status-burst-tail-{edit_index}\n"))],
                                    None,
                                    cx,
                                );
                            }
                        });
                    }
                })
                .detach();
        });

        cx.run_until_parked();
    }

    #[gpui::test]
    async fn test_new_hunk_in_modified_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "foo.txt": "
                    one
                    two
                    three
                    four
                    five
                    six
                    seven
                    eight
                    nine
                    ten
                    ELEVEN
                    twelve
                ".unindent()
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, DiffBase::Head, window, cx)
        });
        cx.run_until_parked();

        fs.set_head_and_index_for_repo(
            Path::new(path!("/project/.git")),
            &[(
                "foo.txt",
                "
                    one
                    two
                    three
                    four
                    five
                    six
                    seven
                    eight
                    nine
                    ten
                    eleven
                    twelve
                "
                .unindent(),
            )],
        );
        cx.run_until_parked();

        let editor = diff.read_with(cx, |diff, cx| diff.editor.read(cx).rhs_editor().clone());

        assert_state_with_diff(
            &editor,
            cx,
            &"
                  ˇnine
                  ten
                - eleven
                + ELEVEN
                  twelve
            "
            .unindent(),
        );

        // The project diff updates its excerpts when a new hunk appears in a buffer that already has a diff.
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/project/foo.txt"), cx)
            })
            .await
            .unwrap();
        buffer.update(cx, |buffer, cx| {
            buffer.edit_via_marked_text(
                &"
                    one
                    «TWO»
                    three
                    four
                    five
                    six
                    seven
                    eight
                    nine
                    ten
                    ELEVEN
                    twelve
                "
                .unindent(),
                None,
                cx,
            );
        });
        project
            .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
            .await
            .unwrap();
        cx.run_until_parked();

        assert_state_with_diff(
            &editor,
            cx,
            &"
                  one
                - two
                + TWO
                  three
                  four
                  five
                  ˇnine
                  ten
                - eleven
                + ELEVEN
                  twelve
            "
            .unindent(),
        );
    }

    #[gpui::test]
    async fn test_branch_diff(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "a.txt": "C",
                "b.txt": "new",
                "c.txt": "in-merge-base-and-work-tree",
                "d.txt": "created-in-head",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let diff = cx
            .update(|window, cx| {
                ProjectDiff::new_with_default_branch(project.clone(), workspace, window, cx)
            })
            .await
            .unwrap();
        cx.run_until_parked();

        fs.set_head_for_repo(
            Path::new(path!("/project/.git")),
            &[("a.txt", "B".into()), ("d.txt", "created-in-head".into())],
            "sha",
        );
        // fs.set_index_for_repo(dot_git, index_state);
        fs.set_merge_base_content_for_repo(
            Path::new(path!("/project/.git")),
            &[
                ("a.txt", "A".into()),
                ("c.txt", "in-merge-base-and-work-tree".into()),
            ],
        );
        cx.run_until_parked();

        let editor = diff.read_with(cx, |diff, cx| diff.editor.read(cx).rhs_editor().clone());

        assert_state_with_diff(
            &editor,
            cx,
            &"
                - A
                + ˇC
                + new
                + created-in-head"
                .unindent(),
        );

        let statuses: HashMap<Arc<RelPath>, Option<FileStatus>> =
            editor.update(cx, |editor, cx| {
                editor
                    .buffer()
                    .read(cx)
                    .all_buffers()
                    .iter()
                    .map(|buffer| {
                        (
                            buffer.read(cx).file().unwrap().path().clone(),
                            editor.status_for_buffer_id(buffer.read(cx).remote_id(), cx),
                        )
                    })
                    .collect()
            });

        assert_eq!(
            statuses,
            HashMap::from_iter([
                (
                    rel_path("a.txt").into_arc(),
                    Some(FileStatus::Tracked(TrackedStatus {
                        index_status: git::status::StatusCode::Modified,
                        worktree_status: git::status::StatusCode::Modified
                    }))
                ),
                (rel_path("b.txt").into_arc(), Some(FileStatus::Untracked)),
                (
                    rel_path("d.txt").into_arc(),
                    Some(FileStatus::Tracked(TrackedStatus {
                        index_status: git::status::StatusCode::Added,
                        worktree_status: git::status::StatusCode::Added
                    }))
                )
            ])
        );
    }

    #[gpui::test]
    async fn test_update_on_uncommit(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "README.md": "# My cool project\n".to_owned()
            }),
        )
        .await;
        fs.set_head_and_index_for_repo(
            Path::new(path!("/project/.git")),
            &[("README.md", "# My cool project\n".to_owned())],
        );
        let project = Project::test(fs.clone(), [Path::new(path!("/project"))], cx).await;
        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        cx.run_until_parked();

        let _editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, rel_path("README.md")), None, true, window, cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        cx.focus(&workspace);
        cx.update(|window, cx| {
            window.dispatch_action(project_diff::Diff.boxed_clone(), cx);
        });
        cx.run_until_parked();
        let item = workspace.update(cx, |workspace, cx| {
            workspace.active_item_as::<ProjectDiff>(cx).unwrap()
        });
        cx.focus(&item);
        let editor = item.read_with(cx, |item, cx| item.editor.read(cx).rhs_editor().clone());

        fs.set_head_and_index_for_repo(
            Path::new(path!("/project/.git")),
            &[(
                "README.md",
                "# My cool project\nDetails to come.\n".to_owned(),
            )],
        );
        cx.run_until_parked();

        let mut cx = EditorTestContext::for_editor_in(editor, cx).await;

        cx.assert_excerpts_with_selections("[EXCERPT]\nˇ# My cool project\nDetails to come.\n");
    }

    #[gpui::test]
    async fn test_deploy_at_respects_active_repository_selection(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project_a"),
            json!({
                ".git": {},
                "a.txt": "CHANGED_A\n",
            }),
        )
        .await;
        fs.insert_tree(
            path!("/project_b"),
            json!({
                ".git": {},
                "b.txt": "CHANGED_B\n",
            }),
        )
        .await;

        fs.set_head_and_index_for_repo(
            Path::new(path!("/project_a/.git")),
            &[("a.txt", "original_a\n".to_string())],
        );
        fs.set_head_and_index_for_repo(
            Path::new(path!("/project_b/.git")),
            &[("b.txt", "original_b\n".to_string())],
        );

        let project = Project::test(
            fs.clone(),
            [
                Path::new(path!("/project_a")),
                Path::new(path!("/project_b")),
            ],
            cx,
        )
        .await;

        let (worktree_a_id, worktree_b_id) = project.read_with(cx, |project, cx| {
            let mut worktrees: Vec<_> = project.worktrees(cx).collect();
            worktrees.sort_by_key(|w| w.read(cx).abs_path());
            (worktrees[0].read(cx).id(), worktrees[1].read(cx).id())
        });

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        cx.run_until_parked();

        // Select project A explicitly and open the diff.
        workspace.update(cx, |workspace, cx| {
            let git_store = workspace.project().read(cx).git_store().clone();
            git_store.update(cx, |git_store, cx| {
                git_store.set_active_repo_for_worktree(worktree_a_id, cx);
            });
        });
        cx.focus(&workspace);
        cx.update(|window, cx| {
            window.dispatch_action(project_diff::Diff.boxed_clone(), cx);
        });
        cx.run_until_parked();

        let diff_item = workspace.update(cx, |workspace, cx| {
            workspace.active_item_as::<ProjectDiff>(cx).unwrap()
        });
        let paths_a = diff_item.read_with(cx, |diff, cx| diff.excerpt_paths(cx));
        assert_eq!(paths_a.len(), 1);
        assert_eq!(*paths_a[0], *"a.txt");

        // Switch the explicit active repository to project B and re-run the diff action.
        workspace.update(cx, |workspace, cx| {
            let git_store = workspace.project().read(cx).git_store().clone();
            git_store.update(cx, |git_store, cx| {
                git_store.set_active_repo_for_worktree(worktree_b_id, cx);
            });
        });
        cx.focus(&workspace);
        cx.update(|window, cx| {
            window.dispatch_action(project_diff::Diff.boxed_clone(), cx);
        });
        cx.run_until_parked();

        let same_diff_item = workspace.update(cx, |workspace, cx| {
            workspace.active_item_as::<ProjectDiff>(cx).unwrap()
        });
        assert_eq!(diff_item.entity_id(), same_diff_item.entity_id());

        let paths_b = diff_item.read_with(cx, |diff, cx| diff.excerpt_paths(cx));
        assert_eq!(paths_b.len(), 1);
        assert_eq!(*paths_b[0], *"b.txt");
    }

    #[gpui::test]
    async fn test_deploy_at_creates_fresh_view_under_target_base(cx: &mut TestAppContext) {
        init_test(cx);

        let (project, _) = build_project(cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        cx.run_until_parked();

        workspace.update_in(cx, |workspace, window, cx| {
            ProjectDiff::deploy_at(workspace, None, DiffBase::Staged, window, cx);
        });
        cx.run_until_parked();

        let staged_view = workspace.update(cx, |workspace, cx| {
            workspace
                .active_item_as::<ProjectDiff>(cx)
                .expect("ProjectDiff should be active after deploy_at")
        });
        assert_eq!(
            staged_view.read_with(cx, |diff, cx| diff.diff_base(cx).clone()),
            DiffBase::Staged,
        );

        workspace.update_in(cx, |workspace, window, cx| {
            ProjectDiff::deploy_at(workspace, None, DiffBase::Unstaged, window, cx);
        });
        cx.run_until_parked();

        let unstaged_view = workspace.update(cx, |workspace, cx| {
            workspace
                .active_item_as::<ProjectDiff>(cx)
                .expect("ProjectDiff should be active after deploy_at")
        });
        assert_eq!(
            unstaged_view.read_with(cx, |diff, cx| diff.diff_base(cx).clone()),
            DiffBase::Unstaged,
        );
        assert_ne!(staged_view.entity_id(), unstaged_view.entity_id());
    }

    #[gpui::test]
    async fn test_deploy_at_does_not_retarget_existing_view_to_new_base(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let (project, _) = build_project(cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace =
            multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        cx.run_until_parked();

        workspace.update_in(cx, |workspace, window, cx| {
            ProjectDiff::deploy_at(workspace, None, DiffBase::Head, window, cx);
        });
        cx.run_until_parked();
        let head_view = workspace.update(cx, |workspace, cx| {
            workspace
                .active_item_as::<ProjectDiff>(cx)
                .expect("ProjectDiff should be active after deploy_at")
        });
        assert_eq!(
            head_view.read_with(cx, |diff, cx| diff.diff_base(cx).clone()),
            DiffBase::Head,
        );

        workspace.update_in(cx, |workspace, window, cx| {
            ProjectDiff::deploy_at(workspace, None, DiffBase::Staged, window, cx);
        });
        cx.run_until_parked();
        let staged_view = workspace.update(cx, |workspace, cx| {
            workspace
                .active_item_as::<ProjectDiff>(cx)
                .expect("ProjectDiff should be active after deploy_at")
        });
        assert_eq!(
            staged_view.read_with(cx, |diff, cx| diff.diff_base(cx).clone()),
            DiffBase::Staged,
        );
        assert_ne!(head_view.entity_id(), staged_view.entity_id());
        assert_eq!(
            head_view.read_with(cx, |diff, cx| diff.diff_base(cx).clone()),
            DiffBase::Head,
        );

        workspace.update_in(cx, |workspace, window, cx| {
            ProjectDiff::deploy_at(workspace, None, DiffBase::Head, window, cx);
        });
        cx.run_until_parked();
        let head_view_again = workspace.update(cx, |workspace, cx| {
            workspace
                .active_item_as::<ProjectDiff>(cx)
                .expect("ProjectDiff should be active after deploy_at")
        });
        assert_eq!(head_view.entity_id(), head_view_again.entity_id());
    }
}
