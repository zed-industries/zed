use crate::{
    conflict_view,
    git_panel::{GitPanel, GitPanelAddon, GitStatusEntry},
    git_panel_settings::GitPanelSettings,
    staged_diff::StagedDiffDelegate,
    unstaged_diff::UnstagedDiffDelegate,
};
use anyhow::Result;
use buffer_diff::{BufferDiff, DiffHunkSecondaryStatus};
use collections::{HashMap, HashSet};
#[cfg(test)]
use editor::Editor;
use editor::{
    Addon, EditorEvent, EditorSettings, RestoreOnlyDiffHunkDelegate, SelectionEffects,
    SplittableEditor, UncommittedDiffHunkDelegate, actions::GoToHunk, multibuffer_context_lines,
    scroll::Autoscroll,
};
use futures_lite::future::yield_now;
use git::{repository::RepoPath, status::FileStatus};
use gpui::{
    App, AppContext as _, AsyncWindowContext, Entity, EventEmitter, FocusHandle, Focusable, Render,
    Subscription, Task, WeakEntity,
};
use language::{Anchor, Buffer, BufferId, Capability, OffsetRangeExt};
use multi_buffer::{MultiBuffer, PathKey};
use project::{
    ConflictSet, Project, ProjectPath,
    git_store::{
        Repository,
        diff_buffer_list::{self, BranchDiffEvent, DiffBase},
    },
};
use settings::{GitPanelGroupBy, GitPanelSortBy, Settings, SettingsStore};
use std::{collections::BTreeMap, sync::Arc};
use theme::ActiveTheme;
use ui::{CommonAnimationExt as _, KeyBinding, prelude::*};
use util::{ResultExt as _, rel_path::RelPath};
use workspace::{
    CloseActiveItem, ItemNavHistory, Workspace,
    item::{Item, SaveOptions},
};
use ztracing::instrument;

struct BufferSubscriptions {
    _diff: Entity<BufferDiff>,
    display_buffer: Entity<Buffer>,
    _diff_subscription: Subscription,
    _conflict_set: Option<Entity<ConflictSet>>,
    _conflict_set_subscription: Option<Subscription>,
}

pub struct DiffMultibuffer {
    multibuffer: Entity<MultiBuffer>,
    branch_diff: Entity<diff_buffer_list::DiffBufferList>,
    editor: Entity<SplittableEditor>,
    buffer_subscriptions: HashMap<RepoPath, BufferSubscriptions>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    pending_scroll: Option<PathKey>,
    review_comment_count: usize,
    _task: Task<Result<()>>,
    _subscription: Subscription,
}

impl DiffMultibuffer {
    pub(crate) fn new_with_diff_base(
        diff_base: DiffBase,
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let branch_diff = cx.new(|cx| {
            diff_buffer_list::DiffBufferList::new(diff_base, project.clone(), window, cx)
        });
        Self::new_impl(branch_diff, project, workspace, window, cx)
    }

    pub(crate) fn new_impl(
        branch_diff: Entity<diff_buffer_list::DiffBufferList>,
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let multibuffer_capability = match branch_diff.read(cx).diff_base() {
            DiffBase::Staged => Capability::ReadOnly,
            _ => Capability::ReadWrite,
        };
        let focus_handle = cx.focus_handle();
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(multibuffer_capability);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });
        let editor = cx.new(|cx| {
            let mut diff_display_editor = SplittableEditor::new(
                EditorSettings::get_global(cx).diff_view_style,
                multibuffer.clone(),
                project.clone(),
                workspace.clone(),
                window,
                cx,
            );
            let diff_base = branch_diff.read(cx).diff_base().clone();
            Self::configure_editor_for_diff_base(
                &diff_base,
                &mut diff_display_editor,
                workspace.downgrade(),
                branch_diff.clone(),
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
                        async |cx| Self::refresh(this, cx).await
                    })
                }
                BranchDiffEvent::DiffBaseChanged => {
                    this.pending_scroll.take();
                    this._task = window.spawn(cx, {
                        let this = cx.weak_entity();
                        async |cx| Self::refresh(this, cx).await
                    })
                }
            },
        );

        let mut was_sort_by = GitPanelSettings::get_global(cx).sort_by;
        let mut was_group_by = GitPanelSettings::get_global(cx).group_by;
        let mut was_tree_view = GitPanelSettings::get_global(cx).tree_view;
        let mut was_collapse_untracked_diff =
            GitPanelSettings::get_global(cx).collapse_untracked_diff;
        cx.observe_global_in::<SettingsStore>(window, move |this, window, cx| {
            let settings = GitPanelSettings::get_global(cx);
            let sort_by = settings.sort_by;
            let group_by = settings.group_by;
            let tree_view = settings.tree_view;
            let is_collapse_untracked_diff = settings.collapse_untracked_diff;
            if sort_by != was_sort_by
                || group_by != was_group_by
                || tree_view != was_tree_view
                || is_collapse_untracked_diff != was_collapse_untracked_diff
            {
                this._task = {
                    window.spawn(cx, {
                        let this = cx.weak_entity();
                        async |cx| Self::refresh(this, cx).await
                    })
                }
            }
            was_sort_by = sort_by;
            was_group_by = group_by;
            was_tree_view = tree_view;
            was_collapse_untracked_diff = is_collapse_untracked_diff;
        })
        .detach();

        let task = window.spawn(cx, {
            let this = cx.weak_entity();
            async |cx| Self::refresh(this, cx).await
        });

        Self {
            workspace: workspace.downgrade(),
            branch_diff,
            focus_handle,
            editor,
            multibuffer,
            buffer_subscriptions: Default::default(),
            pending_scroll: None,
            review_comment_count: 0,
            _task: task,
            _subscription: Subscription::join(
                branch_diff_subscription,
                Subscription::join(editor_subscription, review_comment_subscription),
            ),
        }
    }

    fn configure_editor_for_diff_base(
        diff_base: &DiffBase,
        editor: &mut SplittableEditor,
        workspace: WeakEntity<Workspace>,
        branch_diff: Entity<diff_buffer_list::DiffBufferList>,
        cx: &mut Context<SplittableEditor>,
    ) {
        match diff_base {
            DiffBase::Head => {
                editor.set_diff_hunk_delegate(Some(Arc::new(UncommittedDiffHunkDelegate)), cx);
                editor.rhs_editor().update(cx, |rhs_editor, _cx| {
                    rhs_editor.set_read_only(false);
                    rhs_editor.unregister_addon::<BranchDiffAddon>();
                    if rhs_editor.addon::<GitPanelAddon>().is_none() {
                        rhs_editor.register_addon(GitPanelAddon { workspace });
                    }
                });
            }
            DiffBase::Index => {
                editor.set_diff_hunk_delegate(Some(Arc::new(UnstagedDiffDelegate)), cx);
                editor.rhs_editor().update(cx, |rhs_editor, _cx| {
                    rhs_editor.set_read_only(false);
                    rhs_editor.unregister_addon::<BranchDiffAddon>();
                    if rhs_editor.addon::<GitPanelAddon>().is_none() {
                        rhs_editor.register_addon(GitPanelAddon { workspace });
                    }
                });
            }
            DiffBase::Staged => {
                editor.set_diff_hunk_delegate(Some(Arc::new(StagedDiffDelegate)), cx);
                editor.rhs_editor().update(cx, |rhs_editor, _cx| {
                    rhs_editor.set_read_only(true);
                    rhs_editor.unregister_addon::<BranchDiffAddon>();
                    if rhs_editor.addon::<GitPanelAddon>().is_none() {
                        rhs_editor.register_addon(GitPanelAddon { workspace });
                    }
                });
            }
            DiffBase::Merge { .. } => {
                editor.set_diff_hunk_delegate(Some(Arc::new(RestoreOnlyDiffHunkDelegate)), cx);
                editor.rhs_editor().update(cx, |rhs_editor, _cx| {
                    rhs_editor.set_read_only(false);
                    rhs_editor.unregister_addon::<GitPanelAddon>();
                    if rhs_editor.addon::<BranchDiffAddon>().is_none() {
                        rhs_editor.register_addon(BranchDiffAddon {
                            branch_diff: branch_diff.clone(),
                        });
                    }
                });
            }
        }
    }

    pub(crate) fn diff_base<'a>(&'a self, cx: &'a App) -> &'a DiffBase {
        self.branch_diff.read(cx).diff_base()
    }

    pub(crate) fn branch_diff(&self) -> &Entity<diff_buffer_list::DiffBufferList> {
        &self.branch_diff
    }

    pub(crate) fn repo(&self, cx: &App) -> Option<Entity<Repository>> {
        self.branch_diff.read(cx).repo().cloned()
    }

    pub(crate) fn set_repo(&mut self, repo: Option<Entity<Repository>>, cx: &mut Context<Self>) {
        self.branch_diff.update(cx, |branch_diff, cx| {
            branch_diff.set_repo(repo, cx);
        });
    }

    pub(crate) fn is_unstaged_mode(&self, cx: &App) -> bool {
        self.diff_base(cx) == &DiffBase::Index
    }

    pub(crate) fn is_staged_mode(&self, cx: &App) -> bool {
        self.diff_base(cx) == &DiffBase::Staged
    }

    pub(crate) fn is_dirty(&self, cx: &App) -> bool {
        self.multibuffer.read(cx).is_dirty(cx)
    }

    pub(crate) fn has_conflict(&self, cx: &App) -> bool {
        self.multibuffer.read(cx).has_conflict(cx)
    }

    pub(crate) fn can_save(&self, cx: &App) -> bool {
        !self.is_staged_mode(cx)
    }

    pub(crate) fn multibuffer(&self) -> &Entity<MultiBuffer> {
        &self.multibuffer
    }

    pub(crate) fn move_to_entry(
        &mut self,
        entry: GitStatusEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(git_repo) = self.branch_diff.read(cx).repo() else {
            return;
        };
        let repo = git_repo.read(cx);
        let path_key = project_diff_path_key(repo, &entry.repo_path, entry.status, cx);

        self.move_to_path(path_key, window, cx)
    }

    pub(crate) fn move_to_project_path(
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
        let path_key = project_diff_path_key(&git_repo.read(cx), &repo_path, status, cx);
        self.move_to_path(path_key, window, cx)
    }

    pub(crate) fn move_to_beginning(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |editor, cx| {
                editor.change_selections(Default::default(), window, cx, |s| {
                    s.select_ranges(vec![multi_buffer::Anchor::Min..multi_buffer::Anchor::Min]);
                });
            });
        });
    }

    pub(crate) fn move_to_path(
        &mut self,
        path_key: PathKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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

    pub(crate) fn autoscroll(&self, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |editor, cx| {
                editor.request_autoscroll(Autoscroll::fit(), cx);
            })
        })
    }

    pub(crate) fn calculate_changed_lines(&self, cx: &App) -> (u32, u32) {
        self.multibuffer.read(cx).snapshot(cx).total_changed_lines()
    }

    /// Returns the total count of review comments across all hunks/files.
    pub(crate) fn total_review_comment_count(&self) -> usize {
        self.review_comment_count
    }

    /// Returns a reference to the splittable editor.
    pub(crate) fn editor(&self) -> &Entity<SplittableEditor> {
        &self.editor
    }

    fn selected_ranges(&self, cx: &App) -> (bool, Vec<std::ops::Range<multi_buffer::Anchor>>) {
        let editor = self.editor.read(cx).rhs_editor().read(cx);
        let snapshot = self.multibuffer.read(cx).snapshot(cx);
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

        (selection, ranges)
    }

    /// Ranges for a toolbar stage/unstage action: the selection, or the cursor
    /// (a zero-width range that resolves to the single hunk under it) when
    /// there is no selection. Unlike [`Self::selected_ranges`], this never
    /// widens to the whole excerpt, so actions affect one hunk at a time.
    fn hunk_action_ranges(&self, cx: &App) -> Vec<std::ops::Range<multi_buffer::Anchor>> {
        self.editor
            .read(cx)
            .rhs_editor()
            .read(cx)
            .selections
            .disjoint_anchor_ranges()
            .collect()
    }

    pub(crate) fn button_states(&self, cx: &App) -> ButtonStates {
        let editor = self.editor.read(cx).rhs_editor().read(cx);
        let snapshot = self.multibuffer.read(cx).snapshot(cx);
        let prev_next = snapshot.diff_hunks().nth(1).is_some();
        let (selection, ranges) = self.selected_ranges(cx);
        let mut has_staged_hunks = false;
        let mut has_unstaged_hunks = false;
        if self.is_unstaged_mode(cx) {
            has_unstaged_hunks = editor
                .diff_hunks_in_ranges(&ranges, &snapshot)
                .next()
                .is_some();
        } else {
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

    pub(crate) fn stage_selected_unstaged_hunks(
        &mut self,
        move_to_next: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_unstaged_mode(cx) {
            return;
        }

        let editor = self.editor.read(cx).rhs_editor().clone();
        let ranges = self.hunk_action_ranges(cx);
        // Route through the editor's shared stage path so dirty buffers are
        // saved first, exactly as they are when staging from the uncommitted
        // diff or a normal editor.
        editor.update(cx, |editor, cx| {
            editor.stage_or_unstage_diff_hunks(true, ranges, window, cx);
        });
        if move_to_next {
            editor
                .focus_handle(cx)
                .dispatch_action(&GoToHunk, window, cx);
        }
    }

    pub(crate) fn unstage_selected_staged_hunks(
        &mut self,
        move_to_next: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_staged_mode(cx) {
            return;
        }

        let editor = self.editor.read(cx).rhs_editor().clone();
        let ranges = self.hunk_action_ranges(cx);
        // Route through the editor's delegated stage path, the same path taken
        // by the hunk buttons (on either side of a split) and the keyboard.
        editor.update(cx, |editor, cx| {
            editor.stage_or_unstage_diff_hunks(false, ranges, window, cx);
        });
        if move_to_next {
            editor
                .focus_handle(cx)
                .dispatch_action(&GoToHunk, window, cx);
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
                // Only follow the git panel selection from the view the user is
                // actually interacting with. Background (non-active) diff views
                // refresh on their own and must not hijack the panel selection.
                if !editor.focus_handle(cx).contains_focused(window, cx) {
                    return;
                }
                let Some(project_path) = self.active_project_path(cx) else {
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
                self._task =
                    cx.spawn_in(window, async move |this, cx| Self::refresh(this, cx).await);
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
        repo_path: RepoPath,
        path_key: PathKey,
        file_status: FileStatus,
        display_buffer: Entity<Buffer>,
        main_buffer: Entity<Buffer>,
        diff: Entity<BufferDiff>,
        conflict_set: Option<Entity<ConflictSet>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<BufferId> {
        let diff_subscription = cx.subscribe_in(&diff, window, {
            let repo_path = repo_path.clone();
            let path_key = path_key.clone();
            let display_buffer = display_buffer.clone();
            let main_buffer = main_buffer.clone();
            let diff = diff.clone();
            let conflict_set = conflict_set.clone();
            move |this, _, event, window, cx| match event {
                buffer_diff::BufferDiffEvent::DiffChanged(_) => {
                    this.buffer_ranges_changed(
                        repo_path.clone(),
                        path_key.clone(),
                        file_status,
                        display_buffer.clone(),
                        main_buffer.clone(),
                        diff.clone(),
                        conflict_set.clone(),
                        window,
                        cx,
                    );
                }
                buffer_diff::BufferDiffEvent::BaseTextChanged => {}
            }
        });
        let conflict_set_subscription = conflict_set.as_ref().map(|conflict_set| {
            cx.subscribe_in(conflict_set, window, {
                let repo_path = repo_path.clone();
                let path_key = path_key.clone();
                let display_buffer = display_buffer.clone();
                let main_buffer = main_buffer.clone();
                let diff = diff.clone();
                let conflict_set = Some(conflict_set.clone());
                move |this, _, _, window, cx| {
                    this.buffer_ranges_changed(
                        repo_path.clone(),
                        path_key.clone(),
                        file_status,
                        display_buffer.clone(),
                        main_buffer.clone(),
                        diff.clone(),
                        conflict_set.clone(),
                        window,
                        cx,
                    )
                }
            })
        });
        self.buffer_subscriptions.insert(
            repo_path,
            BufferSubscriptions {
                _diff: diff.clone(),
                display_buffer: display_buffer.clone(),
                _diff_subscription: diff_subscription,
                _conflict_set: conflict_set.clone(),
                _conflict_set_subscription: conflict_set_subscription,
            },
        );

        let snapshot = display_buffer.read(cx).snapshot();
        let diff_snapshot = diff.read(cx).snapshot(cx);

        let excerpt_ranges = {
            let diff_hunk_ranges = diff_snapshot
                .hunks_intersecting_range(
                    Anchor::min_max_range_for_buffer(snapshot.remote_id()),
                    &snapshot,
                )
                .map(|diff_hunk| diff_hunk.buffer_range.to_point(&snapshot));
            let conflict_ranges = conflict_set.as_ref().and_then(|conflict_set| {
                let conflicts = conflict_set.read(cx).snapshot();
                let conflicts = conflicts
                    .conflicts
                    .iter()
                    .map(|conflict| conflict.range.to_point(&snapshot))
                    .collect::<Vec<_>>();
                (!conflicts.is_empty()).then_some(conflicts)
            });

            conflict_ranges.unwrap_or_else(|| diff_hunk_ranges.collect())
        };

        let buffer_id = snapshot.text.remote_id();
        let mut needs_fold = false;

        let (was_empty, is_excerpt_newly_added) = self.editor.update(cx, |editor, cx| {
            let was_empty = editor.rhs_editor().read(cx).buffer().read(cx).is_empty();
            let is_newly_added = editor.update_excerpts_for_path(
                path_key.clone(),
                display_buffer,
                excerpt_ranges,
                multibuffer_context_lines(cx),
                diff,
                cx,
            );
            if let Some(conflict_set) = conflict_set {
                editor.rhs_editor().update(cx, |editor, cx| {
                    conflict_view::buffer_ranges_updated(editor, conflict_set, cx);
                });
            }
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
                    needs_fold = true;
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

        needs_fold.then_some(buffer_id)
    }

    fn buffer_ranges_changed(
        &mut self,
        repo_path: RepoPath,
        path_key: PathKey,
        file_status: FileStatus,
        display_buffer: Entity<Buffer>,
        main_buffer: Entity<Buffer>,
        diff: Entity<BufferDiff>,
        conflict_set: Option<Entity<ConflictSet>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if display_buffer.read(cx).is_dirty() {
            return;
        }
        self.register_buffer(
            repo_path,
            path_key,
            file_status,
            display_buffer,
            main_buffer,
            diff,
            conflict_set,
            window,
            cx,
        );
    }

    #[instrument(skip(this, cx))]
    pub(crate) async fn refresh(this: WeakEntity<Self>, cx: &mut AsyncWindowContext) -> Result<()> {
        let entries = this.update(cx, |this, cx| {
            let (repo, buffers_to_load) = this.branch_diff.update(cx, |branch_diff, cx| {
                let load_buffers = branch_diff.load_buffers(cx);
                (branch_diff.repo().cloned(), load_buffers)
            });
            let mut previous_paths = this
                .multibuffer
                .read(cx)
                .snapshot(cx)
                .buffers_with_paths()
                .map(|(buffer_snapshot, path_key)| (path_key.clone(), buffer_snapshot.remote_id()))
                .collect::<HashMap<_, _>>();

            let mut entries = BTreeMap::new();
            let mut live_repo_paths = HashSet::default();
            if let Some(repo) = repo {
                let repo = repo.read(cx);
                for diff_buffer in buffers_to_load {
                    live_repo_paths.insert(diff_buffer.repo_path.clone());
                    let path_key = project_diff_path_key(
                        &repo,
                        &diff_buffer.repo_path,
                        diff_buffer.file_status,
                        cx,
                    );
                    previous_paths.remove(&path_key);
                    entries.insert(path_key, diff_buffer);
                }
            }

            let repo_path_by_display_id = this
                .buffer_subscriptions
                .iter()
                .map(|(repo_path, sub)| {
                    (sub.display_buffer.read(cx).remote_id(), repo_path.clone())
                })
                .collect::<HashMap<_, _>>();

            this.editor.update(cx, |editor, cx| {
                for (path, buffer_id) in previous_paths {
                    if let Some(repo_path) = repo_path_by_display_id.get(&buffer_id) {
                        this.buffer_subscriptions.remove(repo_path);
                    }
                    editor.rhs_editor().update(cx, |editor, cx| {
                        conflict_view::buffers_removed(editor, &[buffer_id], cx);
                    });
                    let _span = ztracing::info_span!("remove_excerpts_for_path");
                    _span.enter();
                    editor.remove_excerpts_for_path(path, cx);
                }
            });

            this.buffer_subscriptions
                .retain(|repo_path, _| live_repo_paths.contains(repo_path));

            entries
        })?;

        let mut buffers_to_fold = Vec::new();

        for (path_key, entry) in entries {
            if let Some(loaded_buffer) = entry.load.await.log_err() {
                // We might be lagging behind enough that all future entry.load futures are no longer pending.
                // If that is the case, this task will never yield, starving the foreground thread of execution time.
                yield_now().await;
                cx.update(|window, cx| {
                    this.update(cx, |this, cx| {
                        if let Some(buffer_id) = this.register_buffer(
                            entry.repo_path,
                            path_key,
                            entry.file_status,
                            loaded_buffer.display_buffer,
                            loaded_buffer.main_buffer,
                            loaded_buffer.diff,
                            loaded_buffer.conflict_set,
                            window,
                            cx,
                        ) {
                            buffers_to_fold.push(buffer_id);
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

    pub(crate) fn active_project_path(&self, cx: &App) -> Option<ProjectPath> {
        let editor = self.editor.read(cx).focused_editor().read(cx);
        let multibuffer = editor.buffer().read(cx);
        let position = editor.selections.newest_anchor().head();
        let snapshot = multibuffer.snapshot(cx);
        let (text_anchor, _) = snapshot.anchor_to_buffer_anchor(position)?;
        let buffer = multibuffer.buffer(text_anchor.buffer_id)?;

        let file = buffer.read(cx).file()?;
        Some(ProjectPath {
            worktree_id: file.worktree_id(cx),
            path: file.path().clone(),
        })
    }

    pub(crate) fn tab_content_text(&self, cx: &App) -> SharedString {
        match self.branch_diff.read(cx).diff_base() {
            DiffBase::Head => "Uncommitted Changes".into(),
            DiffBase::Index => "Unstaged Changes".into(),
            DiffBase::Staged => "Staged Changes".into(),
            DiffBase::Merge { base_ref } => format!("Changes since {}", base_ref).into(),
        }
    }

    pub(crate) fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.added_to_workspace(workspace, window, cx)
        });
    }

    pub(crate) fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |primary_editor, cx| {
                primary_editor.deactivated(window, cx);
            })
        });
    }

    pub(crate) fn navigate(
        &mut self,
        data: Arc<dyn std::any::Any + Send>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |primary_editor, cx| {
                primary_editor.navigate(data, window, cx)
            })
        })
    }

    pub(crate) fn set_nav_history(&mut self, nav_history: ItemNavHistory, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |primary_editor, _| {
                primary_editor.set_nav_history(Some(nav_history));
            })
        });
    }

    pub(crate) fn for_each_project_item(
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

    pub(crate) fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if self.is_staged_mode(cx) {
            return Task::ready(Ok(()));
        }

        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |primary_editor, cx| {
                primary_editor.save(options, project, window, cx)
            })
        })
    }

    pub(crate) fn reload(
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

    /// Returns the real (worktree-relative) path of each excerpted buffer, in
    /// the order the excerpts appear in the multibuffer. Unlike
    /// [`Self::excerpt_paths`], this resolves the buffer's actual `File` rather
    /// than the (possibly synthetic) `PathKey` path used for sorting.
    #[cfg(any(test, feature = "test-support"))]
    pub fn excerpt_file_paths(&self, cx: &App) -> Vec<String> {
        let multibuffer = self
            .editor()
            .read(cx)
            .rhs_editor()
            .read(cx)
            .buffer()
            .clone();
        let snapshot = multibuffer.read(cx).snapshot(cx);
        let mut result = Vec::new();
        let mut last_buffer_id = None;
        for excerpt in snapshot.excerpts() {
            let buffer_id = excerpt.context.start.buffer_id;
            if last_buffer_id == Some(buffer_id) {
                continue;
            }
            last_buffer_id = Some(buffer_id);
            if let Some(buffer) = multibuffer.read(cx).buffer(buffer_id)
                && let Some(file) = buffer.read(cx).file()
            {
                result.push(file.path().as_unix_str().to_string());
            }
        }
        result
    }
}

pub(crate) struct ButtonStates {
    pub stage: bool,
    pub unstage: bool,
    pub prev_next: bool,
    pub selection: bool,
    pub stage_all: bool,
    pub unstage_all: bool,
}

impl EventEmitter<EditorEvent> for DiffMultibuffer {}

impl Focusable for DiffMultibuffer {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if self.multibuffer.read(cx).is_empty() {
            self.focus_handle.clone()
        } else {
            self.editor.focus_handle(cx)
        }
    }
}

impl Render for DiffMultibuffer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.multibuffer.read(cx).is_empty();
        let is_loading = self.branch_diff.read(cx).is_tree_base_loading() || !self._task.is_ready();
        let empty_label = match self.diff_base(cx) {
            DiffBase::Head => "No uncommitted changes",
            DiffBase::Index => "No unstaged changes",
            DiffBase::Staged => "No staged changes",
            DiffBase::Merge { .. } => "No changes",
        };

        div()
            .track_focus(&self.focus_handle)
            .key_context(if is_empty { "EmptyPane" } else { "GitDiff" })
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
                        .child(h_flex().justify_around().child(Label::new(empty_label)))
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

const CONFLICT_SORT_PREFIX: u64 = 1;
const TRACKED_SORT_PREFIX: u64 = 2;
const NEW_SORT_PREFIX: u64 = 3;

/// Computes a stable [`PathKey`] for a buffer in the project diff.
///
/// The key is an intrinsic function of the file's own repo path and status; it
/// never depends on which other buffers happen to be present in the
/// multibuffer. This is required because the multibuffer uses the path key both
/// to order excerpts and to identify which excerpts belong to a given buffer, so
/// a key that shifted as files were added or removed would break that identity.
///
/// Status grouping is encoded in the `sort_prefix`, and the within-group order
/// is encoded in the (possibly synthetic) path so that `PathKey`'s natural
/// ordering reproduces the git panel's order. The path here is only ever used
/// for sorting and multibuffer identity; the path shown in the UI comes from the
/// buffer's own `File`.
pub(crate) fn project_diff_path_key(
    repo: &Repository,
    repo_path: &RepoPath,
    status: FileStatus,
    cx: &App,
) -> PathKey {
    let settings = GitPanelSettings::get_global(cx);
    let sort_prefix = if settings.group_by != GitPanelGroupBy::Status {
        TRACKED_SORT_PREFIX
    } else if repo.had_conflict_on_last_merge_head_change(repo_path) {
        CONFLICT_SORT_PREFIX
    } else if status.is_created() {
        NEW_SORT_PREFIX
    } else {
        TRACKED_SORT_PREFIX
    };
    let path = project_diff_sort_path(repo_path, settings.tree_view, settings.sort_by);
    PathKey::with_sort_prefix(sort_prefix, path)
}

fn project_diff_sort_path(
    repo_path: &RelPath,
    tree_view: bool,
    sort_by: GitPanelSortBy,
) -> Arc<RelPath> {
    if tree_view {
        tree_sort_path(repo_path)
    } else {
        match sort_by {
            GitPanelSortBy::Path => repo_path.into_arc(),
            GitPanelSortBy::Name => name_sort_path(repo_path),
        }
    }
}

/// Builds a synthetic path that sorts by file name first, falling back to the
/// full path to keep the key unique per file.
fn name_sort_path(repo_path: &RelPath) -> Arc<RelPath> {
    let Some(file_name) = repo_path.file_name() else {
        return repo_path.into_arc();
    };
    let synthetic = format!("{}/{}", file_name, repo_path.as_unix_str());
    RelPath::unix(&synthetic)
        .map(|path| path.into_arc())
        .unwrap_or_else(|_| repo_path.into_arc())
}

/// Builds a synthetic path whose natural component-wise ordering reproduces a
/// folder-first tree order. Each directory component is prefixed with a NUL
/// byte, which can never appear in a real path component and sorts before every
/// printable character, so at each level directories sort before files.
fn tree_sort_path(repo_path: &RelPath) -> Arc<RelPath> {
    let components: Vec<&str> = repo_path.components().collect();
    if components.len() <= 1 {
        return repo_path.into_arc();
    }
    let last = components.len() - 1;
    let mut synthetic = String::new();
    for (index, component) in components.into_iter().enumerate() {
        if index > 0 {
            synthetic.push('/');
        }
        if index < last {
            synthetic.push('\0');
        }
        synthetic.push_str(component);
    }
    RelPath::unix(&synthetic)
        .map(|path| path.into_arc())
        .unwrap_or_else(|_| repo_path.into_arc())
}

struct BranchDiffAddon {
    branch_diff: Entity<diff_buffer_list::DiffBufferList>,
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
    use db::indoc;
    use editor::test::editor_test_context::{EditorTestContext, assert_state_with_diff};
    use gpui::TestAppContext;
    use project::FakeFs;
    use serde_json::json;
    use settings::{DiffViewStyle, GitPanelGroupBy, GitPanelSortBy, SettingsStore};
    use std::path::Path;
    use unindent::Unindent as _;
    use util::{path, rel_path::rel_path};
    use workspace::MultiWorkspace;

    use super::*;

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
            DiffMultibuffer::new_with_diff_base(
                DiffBase::Head,
                project.clone(),
                workspace,
                window,
                cx,
            )
        });
        cx.run_until_parked();

        let editor = diff.read_with(cx, |diff, cx| diff.editor().read(cx).rhs_editor().clone());
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
            DiffMultibuffer::new_with_diff_base(
                DiffBase::Head,
                project.clone(),
                workspace,
                window,
                cx,
            )
        });
        cx.run_until_parked();

        fs.set_head_and_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("bar", "bar\n".into()), ("foo", "foo\n".into())],
        );
        cx.run_until_parked();

        let editor = cx.update_window_entity(&diff, |diff, window, cx| {
            diff.move_to_path(
                PathKey::with_sort_prefix(2, rel_path("foo").into_arc()),
                window,
                cx,
            );
            diff.editor().read(cx).rhs_editor().clone()
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
                PathKey::with_sort_prefix(2, rel_path("bar").into_arc()),
                window,
                cx,
            );
            diff.editor().read(cx).rhs_editor().clone()
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
            DiffMultibuffer::new_with_diff_base(
                DiffBase::Head,
                project.clone(),
                workspace,
                window,
                cx,
            )
        });
        cx.run_until_parked();

        let diff_editor =
            diff.read_with(cx, |diff, cx| diff.editor().read(cx).rhs_editor().clone());

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
            DiffMultibuffer::new_with_diff_base(
                DiffBase::Head,
                project.clone(),
                workspace,
                window,
                cx,
            )
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

        let editor = diff.read_with(cx, |diff, cx| diff.editor().read(cx).rhs_editor().clone());

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
    async fn test_sort_by_name_tie_breaks_on_path(cx: &mut TestAppContext) {
        init_test(cx);

        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    let git_panel = settings.git_panel.get_or_insert_default();
                    git_panel.sort_by = Some(GitPanelSortBy::Name);
                    git_panel.group_by = Some(GitPanelGroupBy::None);
                });
            });
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "lib": { "foo.rs": "LIB FOO\n" },
                "src": { "foo.rs": "SRC FOO\n" },
                "m.rs": "M\n",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            DiffMultibuffer::new_with_diff_base(
                DiffBase::Head,
                project.clone(),
                workspace,
                window,
                cx,
            )
        });
        cx.run_until_parked();

        fs.set_head_and_index_for_repo(
            path!("/project/.git").as_ref(),
            &[
                ("lib/foo.rs", "lib foo\n".into()),
                ("src/foo.rs", "src foo\n".into()),
                ("m.rs", "m\n".into()),
            ],
        );
        cx.run_until_parked();

        // Sorted by file name, the two `foo.rs` files come before `m.rs`, and the
        // tie between them is broken by the full path (`lib/` before `src/`).
        // A plain path sort would instead order them `lib/foo.rs`, `m.rs`,
        // `src/foo.rs`.
        let paths = diff.read_with(cx, |diff, cx| diff.excerpt_file_paths(cx));
        assert_eq!(paths, vec!["lib/foo.rs", "src/foo.rs", "m.rs"]);
    }

    #[gpui::test]
    async fn test_tree_view_orders_directories_before_files(cx: &mut TestAppContext) {
        init_test(cx);

        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    let git_panel = settings.git_panel.get_or_insert_default();
                    git_panel.tree_view = Some(true);
                    git_panel.group_by = Some(GitPanelGroupBy::None);
                });
            });
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "src": {
                    "a.rs": "A\n",
                    "m.rs": "M\n",
                    "sub": { "b.rs": "B\n" },
                },
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let diff = cx.new_window_entity(|window, cx| {
            DiffMultibuffer::new_with_diff_base(
                DiffBase::Head,
                project.clone(),
                workspace,
                window,
                cx,
            )
        });
        cx.run_until_parked();

        fs.set_head_and_index_for_repo(
            path!("/project/.git").as_ref(),
            &[
                ("src/a.rs", "a\n".into()),
                ("src/m.rs", "m\n".into()),
                ("src/sub/b.rs", "b\n".into()),
            ],
        );
        cx.run_until_parked();

        // In tree view the `src/sub/` directory sorts before the files directly
        // in `src/`. A plain path sort would interleave them as `src/a.rs`,
        // `src/m.rs`, `src/sub/b.rs`.
        let paths = diff.read_with(cx, |diff, cx| diff.excerpt_file_paths(cx));
        assert_eq!(paths, vec!["src/sub/b.rs", "src/a.rs", "src/m.rs"]);
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
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        cx.run_until_parked();

        let diff = cx.new_window_entity(|window, cx| {
            DiffMultibuffer::new_with_diff_base(
                DiffBase::Head,
                project.clone(),
                workspace,
                window,
                cx,
            )
        });
        cx.run_until_parked();
        let editor = diff.read_with(cx, |diff, cx| diff.editor().read(cx).rhs_editor().clone());

        let mut cx = EditorTestContext::for_editor_in(editor, cx).await;

        cx.assert_excerpts_with_selections(&format!("[EXCERPT]\nˇ{git_contents}"));

        cx.dispatch_action(editor::actions::GoToHunk);
        cx.dispatch_action(editor::actions::GoToHunk);
        cx.dispatch_action(git::Restore);
        cx.dispatch_action(editor::actions::MoveToBeginning);

        cx.assert_excerpts_with_selections(&format!("[EXCERPT]\nˇ{git_contents}"));
    }
}
