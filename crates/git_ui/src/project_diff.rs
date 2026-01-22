use crate::{
    conflict_view::ConflictAddon,
    git_panel::{GitPanel, GitPanelAddon, GitStatusEntry},
    git_panel_settings::GitPanelSettings,
    remote_button::{render_publish_button, render_push_button},
};
use anyhow::{Context as _, Result, anyhow};
use buffer_diff::{BufferDiff, DiffHunkSecondaryStatus};
use collections::{HashMap, HashSet};
use editor::{
    Addon, Editor, EditorEvent, SelectionEffects, SplittableEditor,
    actions::{GoToHunk, GoToPreviousHunk, SendReviewToAgent},
    multibuffer_context_lines,
    scroll::Autoscroll,
};
use git::{
    Commit, StageAll, StageAndNext, ToggleStaged, UnstageAll, UnstageAndNext,
    repository::{Branch, RepoPath, Upstream, UpstreamTracking, UpstreamTrackingStatus},
    status::FileStatus,
};
use gpui::{
    Action, AnyElement, App, AppContext as _, AsyncWindowContext, Entity, EventEmitter,
    FocusHandle, Focusable, Render, Subscription, Task, WeakEntity, actions,
};
use language::{Anchor, Buffer, Capability, OffsetRangeExt};
use multi_buffer::{MultiBuffer, PathKey};
use project::{
    Project, ProjectPath,
    git_store::{
        Repository,
        branch_diff::{self, BranchDiffEvent, DiffBase},
    },
};
use settings::{Settings, SettingsStore};
use smol::future::yield_now;
use std::any::{Any, TypeId};
use std::sync::Arc;
use theme::ActiveTheme;
use ui::{KeyBinding, Tooltip, prelude::*, vertical_divider};
use util::{ResultExt as _, rel_path::RelPath};
use workspace::{
    CloseActiveItem, ItemNavHistory, SerializableItem, ToolbarItemEvent, ToolbarItemLocation,
    ToolbarItemView, Workspace,
    item::{Item, ItemEvent, ItemHandle, SaveOptions, TabContentParams},
    notifications::NotifyTaskExt,
    searchable::SearchableItemHandle,
};
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
        Self::deploy_at(workspace, None, window, cx)
    }

    fn deploy_branch_diff(
        workspace: &mut Workspace,
        _: &BranchDiff,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        telemetry::event!("Git Branch Diff Opened");
        let project = workspace.project().clone();

        let existing = workspace
            .items_of_type::<Self>(cx)
            .find(|item| matches!(item.read(cx).diff_base(cx), DiffBase::Merge { .. }));
        if let Some(existing) = existing {
            workspace.activate_item(&existing, true, true, window, cx);
            return;
        }
        let workspace = cx.entity();
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
            .detach_and_notify_err(window, cx);
    }

    pub fn deploy_at(
        workspace: &mut Workspace,
        entry: Option<GitStatusEntry>,
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
        let existing = workspace
            .items_of_type::<Self>(cx)
            .find(|item| matches!(item.read(cx).diff_base(cx), DiffBase::Head));
        let project_diff = if let Some(existing) = existing {
            existing.update(cx, |project_diff, cx| {
                project_diff.move_to_beginning(window, cx);
            });

            workspace.activate_item(&existing, true, true, window, cx);
            existing
        } else {
            let workspace_handle = cx.entity();
            let project_diff =
                cx.new(|cx| Self::new(workspace.project().clone(), workspace_handle, window, cx));
            workspace.add_item_to_active_pane(
                Box::new(project_diff.clone()),
                None,
                true,
                window,
                cx,
            );
            project_diff
        };
        if let Some(entry) = entry {
            project_diff.update(cx, |project_diff, cx| {
                project_diff.move_to_entry(entry, window, cx);
            })
        }
    }

    pub fn deploy_at_project_path(
        workspace: &mut Workspace,
        project_path: ProjectPath,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        telemetry::event!("Git Diff Opened", source = "Agent Panel");
        let existing = workspace
            .items_of_type::<Self>(cx)
            .find(|item| matches!(item.read(cx).diff_base(cx), DiffBase::Head));
        let project_diff = if let Some(existing) = existing {
            workspace.activate_item(&existing, true, true, window, cx);
            existing
        } else {
            let workspace_handle = cx.entity();
            let project_diff =
                cx.new(|cx| Self::new(workspace.project().clone(), workspace_handle, window, cx));
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
            editor.primary_editor().update(cx, |editor, cx| {
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let branch_diff =
            cx.new(|cx| branch_diff::BranchDiff::new(DiffBase::Head, project.clone(), window, cx));
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
            let diff_display_editor = SplittableEditor::new_unsplit(
                multibuffer.clone(),
                project.clone(),
                workspace.clone(),
                window,
                cx,
            );
            diff_display_editor
                .primary_editor()
                .update(cx, |editor, cx| {
                    editor.disable_diagnostics(cx);
                    editor.set_show_diff_review_button(true, cx);

                    match branch_diff.read(cx).diff_base() {
                        DiffBase::Head => {
                            editor.register_addon(GitPanelAddon {
                                workspace: workspace.downgrade(),
                            });
                        }
                        DiffBase::Merge { .. } => {
                            editor.register_addon(BranchDiffAddon {
                                branch_diff: branch_diff.clone(),
                            });
                            editor.start_temporary_diff_override();
                            editor.set_render_diff_hunk_controls(
                                Arc::new(|_, _, _, _, _, _, _, _| gpui::Empty.into_any_element()),
                                cx,
                            );
                        }
                    }
                });
            diff_display_editor
        });
        let editor_subscription = cx.subscribe_in(&editor, window, Self::handle_editor_event);

        let primary_editor = editor.read(cx).primary_editor().clone();
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

        Self {
            project,
            workspace: workspace.downgrade(),
            branch_diff,
            focus_handle,
            editor,
            multibuffer,
            buffer_diff_subscriptions: Default::default(),
            pending_scroll: None,
            review_comment_count: 0,
            _task: task,
            _subscription: Subscription::join(
                branch_diff_subscription,
                Subscription::join(editor_subscription, review_comment_subscription),
            ),
        }
    }

    pub fn diff_base<'a>(&'a self, cx: &'a App) -> &'a DiffBase {
        self.branch_diff.read(cx).diff_base()
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
        let editor = self.editor.read(cx).last_selected_editor().read(cx);
        let position = editor.selections.newest_anchor().head();
        let multi_buffer = editor.buffer().read(cx);
        let (_, buffer, _) = multi_buffer.excerpt_containing(position, cx)?;

        let file = buffer.read(cx).file()?;
        Some(ProjectPath {
            worktree_id: file.worktree_id(cx),
            path: file.path().clone(),
        })
    }

    fn move_to_beginning(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.primary_editor().update(cx, |editor, cx| {
                editor.move_to_beginning(&Default::default(), window, cx);
            });
        });
    }

    fn move_to_path(&mut self, path_key: PathKey, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(position) = self.multibuffer.read(cx).location_for_path(&path_key, cx) {
            self.editor.update(cx, |editor, cx| {
                editor.primary_editor().update(cx, |editor, cx| {
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

    /// Returns the total count of review comments across all hunks/files.
    pub fn total_review_comment_count(&self) -> usize {
        self.review_comment_count
    }

    /// Returns a reference to the splittable editor.
    pub fn editor(&self) -> &Entity<SplittableEditor> {
        &self.editor
    }

    fn button_states(&self, cx: &App) -> ButtonStates {
        let editor = self.editor.read(cx).primary_editor().read(cx);
        let snapshot = self.multibuffer.read(cx).snapshot(cx);
        let prev_next = snapshot.diff_hunks().nth(1).is_some();
        let mut selection = true;

        let mut ranges = editor
            .selections
            .disjoint_anchor_ranges()
            .collect::<Vec<_>>();
        if !ranges.iter().any(|range| range.start != range.end) {
            selection = false;
            if let Some((excerpt_id, _, range)) = self
                .editor
                .read(cx)
                .primary_editor()
                .read(cx)
                .active_excerpt(cx)
            {
                ranges = vec![multi_buffer::Anchor::range_in_buffer(excerpt_id, range)];
            } else {
                ranges = Vec::default();
            }
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
                self.workspace
                    .update(cx, |workspace, cx| {
                        if let Some(git_panel) = workspace.panel::<GitPanel>(cx) {
                            git_panel.update(cx, |git_panel, cx| {
                                git_panel.select_entry_by_path(project_path, window, cx)
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
    ) {
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
            .primary_editor()
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

        let (was_empty, is_excerpt_newly_added) = self.editor.update(cx, |editor, cx| {
            let was_empty = editor
                .primary_editor()
                .read(cx)
                .buffer()
                .read(cx)
                .is_empty();
            let (_, is_newly_added) = editor.set_excerpts_for_path(
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
            editor.primary_editor().update(cx, |editor, cx| {
                if was_empty {
                    editor.change_selections(
                        SelectionEffects::no_scroll(),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges([
                                multi_buffer::Anchor::min()..multi_buffer::Anchor::min()
                            ])
                        },
                    );
                }
                if is_excerpt_newly_added
                    && (file_status.is_deleted()
                        || (file_status.is_untracked()
                            && GitPanelSettings::get_global(cx).collapse_untracked_diff))
                {
                    editor.fold_buffer(snapshot.text.remote_id(), cx)
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
    }

    #[instrument(skip_all)]
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
            let mut previous_paths = this
                .multibuffer
                .read(cx)
                .paths()
                .cloned()
                .collect::<HashSet<_>>();

            if let Some(repo) = repo {
                let repo = repo.read(cx);

                path_keys = Vec::with_capacity(buffers_to_load.len());
                for entry in buffers_to_load.iter() {
                    let sort_prefix = sort_prefix(&repo, &entry.repo_path, entry.file_status, cx);
                    let path_key =
                        PathKey::with_sort_prefix(sort_prefix, entry.repo_path.as_ref().clone());
                    previous_paths.remove(&path_key);
                    path_keys.push(path_key)
                }
            }

            this.editor.update(cx, |editor, cx| {
                for path in previous_paths {
                    if let Some(buffer) = this.multibuffer.read(cx).buffer_for_path(&path, cx) {
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
                    editor.remove_excerpts_for_path(path, cx);
                }
            });
            buffers_to_load
        })?;

        for (entry, path_key) in buffers_to_load.into_iter().zip(path_keys.into_iter()) {
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
                            this.register_buffer(
                                path_key,
                                entry.file_status,
                                buffer,
                                diff,
                                window,
                                cx,
                            )
                        }
                    })
                    .ok();
                })?;
            }
        }
        this.update(cx, |this, cx| {
            this.pending_scroll.take();
            cx.notify();
        })?;

        Ok(())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn excerpt_paths(&self, cx: &App) -> Vec<std::sync::Arc<util::rel_path::RelPath>> {
        self.multibuffer
            .read(cx)
            .paths()
            .map(|key| key.path.clone())
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

    fn to_item_events(event: &EditorEvent, f: impl FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.primary_editor().update(cx, |primary_editor, cx| {
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
            editor.primary_editor().update(cx, |primary_editor, cx| {
                primary_editor.navigate(data, window, cx)
            })
        })
    }

    fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> {
        Some("Project Diff".into())
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
            DiffBase::Merge { base_ref } => format!("Changes since {}", base_ref).into(),
        }
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Project Diff Opened")
    }

    fn as_searchable(&self, _: &Entity<Self>, cx: &App) -> Option<Box<dyn SearchableItemHandle>> {
        // TODO(split-diff) SplitEditor should be searchable
        Some(Box::new(self.editor.read(cx).primary_editor().clone()))
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.editor
            .read(cx)
            .primary_editor()
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
            editor.primary_editor().update(cx, |primary_editor, _| {
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
            ProjectDiff::new(self.project.clone(), workspace, window, cx)
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
            editor.primary_editor().update(cx, |primary_editor, cx| {
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
            editor.primary_editor().update(cx, |primary_editor, cx| {
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
            Some(self.editor.read(cx).primary_editor().clone().into())
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

        div()
            .track_focus(&self.focus_handle)
            .key_context(if is_empty { "EmptyPane" } else { "GitDiff" })
            .bg(cx.theme().colors().editor_background)
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .when(is_empty, |el| {
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
                                .child(Label::new("No uncommitted changes")),
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
        window.spawn(cx, async move |cx| {
            let diff_base = persistence::PROJECT_DIFF_DB.get_diff_base(item_id, workspace_id)?;

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

        Some(cx.background_spawn({
            async move {
                persistence::PROJECT_DIFF_DB
                    .save_diff_base(item_id, workspace_id, diff_base.clone())
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

    db::static_connection!(PROJECT_DIFF_DB, ProjectDiffDb, [WorkspaceDb]);

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
            .ok();
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
            .ok();
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
            .filter(|item| item.read(cx).diff_base(cx) == &DiffBase::Head)
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
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(project_diff) = self.project_diff(cx) else {
            return div();
        };
        let focus_handle = project_diff.focus_handle(cx);
        let button_states = project_diff.read(cx).button_states(cx);
        let review_count = project_diff.read(cx).total_review_comment_count();

        h_group_xl()
            .my_neg_1()
            .py_1()
            .items_center()
            .flex_wrap()
            .justify_between()
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
            // "Send Review to Agent" button (only shown when there are review comments)
            .when(review_count > 0, |el| {
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
    .icon(IconName::ZedAssistant)
    .icon_position(IconPosition::Start)
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
    pub fn new(_: &mut Context<Self>) -> Self {
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

        h_group_xl()
            .my_neg_1()
            .py_1()
            .items_center()
            .flex_wrap()
            .justify_end()
            .when(review_count > 0, |el| {
                el.child(
                    render_send_review_to_agent_button(review_count, &focus_handle).on_click(
                        cx.listener(|this, _, window, cx| {
                            this.dispatch_action(&SendReviewToAgent, window, cx)
                        }),
                    ),
                )
            })
    }
}

#[derive(IntoElement, RegisterComponent)]
pub struct ProjectDiffEmptyState {
    pub no_repo: bool,
    pub can_push_and_pull: bool,
    pub focus_handle: Option<FocusHandle>,
    pub current_branch: Option<Branch>,
    // has_pending_commits: bool,
    // ahead_of_remote: bool,
    // no_git_repository: bool,
}

impl RenderOnce for ProjectDiffEmptyState {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let status_against_remote = |ahead_by: usize, behind_by: usize| -> bool {
            matches!(self.current_branch, Some(Branch {
                    upstream:
                        Some(Upstream {
                            tracking:
                                UpstreamTracking::Tracked(UpstreamTrackingStatus {
                                    ahead, behind, ..
                                }),
                            ..
                        }),
                    ..
                }) if (ahead > 0) == (ahead_by > 0) && (behind > 0) == (behind_by > 0))
        };

        let change_count = |current_branch: &Branch| -> (usize, usize) {
            match current_branch {
                Branch {
                    upstream:
                        Some(Upstream {
                            tracking:
                                UpstreamTracking::Tracked(UpstreamTrackingStatus {
                                    ahead, behind, ..
                                }),
                            ..
                        }),
                    ..
                } => (*ahead as usize, *behind as usize),
                _ => (0, 0),
            }
        };

        let not_ahead_or_behind = status_against_remote(0, 0);
        let ahead_of_remote = status_against_remote(1, 0);
        let branch_not_on_remote = if let Some(branch) = self.current_branch.as_ref() {
            branch.upstream.is_none()
        } else {
            false
        };

        let has_branch_container = |branch: &Branch| {
            h_flex()
                .max_w(px(420.))
                .bg(cx.theme().colors().text.opacity(0.05))
                .border_1()
                .border_color(cx.theme().colors().border)
                .rounded_sm()
                .gap_8()
                .px_6()
                .py_4()
                .map(|this| {
                    if ahead_of_remote {
                        let ahead_count = change_count(branch).0;
                        let ahead_string = format!("{} Commits Ahead", ahead_count);
                        this.child(
                            v_flex()
                                .child(Headline::new(ahead_string).size(HeadlineSize::Small))
                                .child(
                                    Label::new(format!("Push your changes to {}", branch.name()))
                                        .color(Color::Muted),
                                ),
                        )
                        .child(div().child(render_push_button(
                            self.focus_handle,
                            "push".into(),
                            ahead_count as u32,
                        )))
                    } else if branch_not_on_remote {
                        this.child(
                            v_flex()
                                .child(Headline::new("Publish Branch").size(HeadlineSize::Small))
                                .child(
                                    Label::new(format!("Create {} on remote", branch.name()))
                                        .color(Color::Muted),
                                ),
                        )
                        .child(
                            div().child(render_publish_button(self.focus_handle, "publish".into())),
                        )
                    } else {
                        this.child(Label::new("Remote status unknown").color(Color::Muted))
                    }
                })
        };

        v_flex().size_full().items_center().justify_center().child(
            v_flex()
                .gap_1()
                .when(self.no_repo, |this| {
                    // TODO: add git init
                    this.text_center()
                        .child(Label::new("No Repository").color(Color::Muted))
                })
                .map(|this| {
                    if not_ahead_or_behind && self.current_branch.is_some() {
                        this.text_center()
                            .child(Label::new("No Changes").color(Color::Muted))
                    } else {
                        this.when_some(self.current_branch.as_ref(), |this, branch| {
                            this.child(has_branch_container(branch))
                        })
                    }
                }),
        )
    }
}

mod preview {
    use git::repository::{
        Branch, CommitSummary, Upstream, UpstreamTracking, UpstreamTrackingStatus,
    };
    use ui::prelude::*;

    use super::ProjectDiffEmptyState;

    // View this component preview using `workspace: open component-preview`
    impl Component for ProjectDiffEmptyState {
        fn scope() -> ComponentScope {
            ComponentScope::VersionControl
        }

        fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
            let unknown_upstream: Option<UpstreamTracking> = None;
            let ahead_of_upstream: Option<UpstreamTracking> = Some(
                UpstreamTrackingStatus {
                    ahead: 2,
                    behind: 0,
                }
                .into(),
            );

            let not_ahead_or_behind_upstream: Option<UpstreamTracking> = Some(
                UpstreamTrackingStatus {
                    ahead: 0,
                    behind: 0,
                }
                .into(),
            );

            fn branch(upstream: Option<UpstreamTracking>) -> Branch {
                Branch {
                    is_head: true,
                    ref_name: "some-branch".into(),
                    upstream: upstream.map(|tracking| Upstream {
                        ref_name: "origin/some-branch".into(),
                        tracking,
                    }),
                    most_recent_commit: Some(CommitSummary {
                        sha: "abc123".into(),
                        subject: "Modify stuff".into(),
                        commit_timestamp: 1710932954,
                        author_name: "John Doe".into(),
                        has_parent: true,
                    }),
                }
            }

            let no_repo_state = ProjectDiffEmptyState {
                no_repo: true,
                can_push_and_pull: false,
                focus_handle: None,
                current_branch: None,
            };

            let no_changes_state = ProjectDiffEmptyState {
                no_repo: false,
                can_push_and_pull: true,
                focus_handle: None,
                current_branch: Some(branch(not_ahead_or_behind_upstream)),
            };

            let ahead_of_upstream_state = ProjectDiffEmptyState {
                no_repo: false,
                can_push_and_pull: true,
                focus_handle: None,
                current_branch: Some(branch(ahead_of_upstream)),
            };

            let unknown_upstream_state = ProjectDiffEmptyState {
                no_repo: false,
                can_push_and_pull: true,
                focus_handle: None,
                current_branch: Some(branch(unknown_upstream)),
            };

            let (width, height) = (px(480.), px(320.));

            Some(
                v_flex()
                    .gap_6()
                    .children(vec![
                        example_group(vec![
                            single_example(
                                "No Repo",
                                div()
                                    .w(width)
                                    .h(height)
                                    .child(no_repo_state)
                                    .into_any_element(),
                            ),
                            single_example(
                                "No Changes",
                                div()
                                    .w(width)
                                    .h(height)
                                    .child(no_changes_state)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Unknown Upstream",
                                div()
                                    .w(width)
                                    .h(height)
                                    .child(unknown_upstream_state)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Ahead of Remote",
                                div()
                                    .w(width)
                                    .h(height)
                                    .child(ahead_of_upstream_state)
                                    .into_any_element(),
                            ),
                        ])
                        .vertical(),
                    ])
                    .into_any_element(),
            )
        }
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
    use git::status::{TrackedStatus, UnmergedStatus, UnmergedStatusCode};
    use gpui::TestAppContext;
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::Path;
    use unindent::Unindent as _;
    use util::{
        path,
        rel_path::{RelPath, rel_path},
    };

    use super::*;

    #[ctor::ctor]
    fn init_logger() {
        zlog::init_test();
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
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

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, window, cx)
        });
        cx.run_until_parked();

        let editor = diff.read_with(cx, |diff, cx| diff.editor.read(cx).primary_editor().clone());
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
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, window, cx)
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
            diff.editor.read(cx).primary_editor().clone()
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
            diff.editor.read(cx).primary_editor().clone()
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
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
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
            ProjectDiff::new(project.clone(), workspace, window, cx)
        });
        cx.run_until_parked();

        let diff_editor =
            diff.read_with(cx, |diff, cx| diff.editor.read(cx).primary_editor().clone());

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
                    .diff_hunks_in_ranges(&[editor::Anchor::min()..editor::Anchor::max()], snapshot)
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
                    .diff_hunks_in_ranges(&[editor::Anchor::min()..editor::Anchor::max()], snapshot)
                    .collect::<Vec<_>>()
            });
        assert_eq!(new_buffer_hunks.as_slice(), &[]);

        cx.update_window_entity(&buffer_editor, |buffer_editor, window, cx| {
            buffer_editor.set_text("different\n", window, cx);
            buffer_editor.save(
                SaveOptions {
                    format: false,
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
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

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
        let editor = item.read_with(cx, |item, cx| item.editor.read(cx).primary_editor().clone());

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
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

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
        let editor = item.read_with(cx, |item, cx| item.editor.read(cx).primary_editor().clone());

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
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, window, cx)
        });
        cx.run_until_parked();

        cx.update(|window, cx| {
            let editor = diff.read(cx).editor.read(cx).primary_editor().clone();
            let excerpt_ids = editor.read(cx).buffer().read(cx).excerpt_ids();
            assert_eq!(excerpt_ids.len(), 1);
            let excerpt_id = excerpt_ids[0];
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
                .primary_editor()
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
                excerpt_id,
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
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let diff = cx.new_window_entity(|window, cx| {
            ProjectDiff::new(project.clone(), workspace, window, cx)
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

        let editor = diff.read_with(cx, |diff, cx| diff.editor.read(cx).primary_editor().clone());

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
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
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

        let editor = diff.read_with(cx, |diff, cx| diff.editor.read(cx).primary_editor().clone());

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
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
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
        let editor = item.read_with(cx, |item, cx| item.editor.read(cx).primary_editor().clone());

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
}
