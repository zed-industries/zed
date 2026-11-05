use super::*;
use futures::future::{FutureExt as _, Shared};
use settings::GitDiffBaseSetting;

pub(super) enum ChangesMode {
    Uncommitted,
    /// `None` until a base is known. The list only exists in branch mode, so
    /// the panel runs no tree diffs otherwise.
    Branch(Option<BranchList>),
}

pub(super) struct BranchList {
    list: Entity<DiffBufferList>,
    _subscription: Subscription,
}

impl BranchList {
    /// A failed reload keeps the files from the last base that loaded, so
    /// they're hidden to show the error instead.
    fn shown_files(&self, cx: &App) -> Option<Vec<GitStatusEntry>> {
        let list = self.list.read(cx);
        if list.tree_diff_error().is_some() {
            return None;
        }
        listed_files(list)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum DefaultBranch {
    NotLoaded,
    Loading,
    Found(SharedString),
    Missing,
}

pub(super) struct BranchBase {
    /// Picked in the panel, or adopted from the active branch diff tab while
    /// in branch mode. Cleared on leaving branch mode.
    picked: Option<SharedString>,
    /// Looked up only when needed, because the branch list changes on every
    /// commit, fetch and push.
    default: DefaultBranch,
    default_is_stale: bool,
    /// Requests waiting on the default branch await clones of this, so
    /// dropping one of them leaves the lookup running.
    default_lookup: Shared<Task<()>>,
    /// Bumped by every lookup and reset. A lookup only writes while the count
    /// is still its own, since a cancelled request's clone can keep it running.
    default_lookup_count: usize,
    pending_show: Option<PendingShow>,
    followed: Option<FollowedBranchDiff>,
}

/// A request to show branch changes that waits for the default branch to
/// know between switching modes and opening the base picker.
struct PendingShow {
    request: ShowBranchChangesRequest,
    wait: Task<()>,
}

/// The active branch diff tab. Branch mode adopts its base and follows base
/// changes made in the tab's own picker.
struct FollowedBranchDiff {
    list: WeakEntity<DiffBufferList>,
    _subscription: Subscription,
}

#[derive(Clone, Copy)]
struct ShowBranchChangesRequest {
    /// The empty state's button, which also opens the diff. It uses the
    /// default branch, or the picked base when there is none.
    from_empty_state: bool,
}

impl BranchBase {
    pub(super) fn new() -> Self {
        Self {
            picked: None,
            default: DefaultBranch::NotLoaded,
            default_is_stale: false,
            default_lookup: Task::ready(()).shared(),
            default_lookup_count: 0,
            pending_show: None,
            followed: None,
        }
    }

    pub(super) fn effective(&self) -> Option<SharedString> {
        self.picked.clone().or_else(|| match &self.default {
            DefaultBranch::Found(default_branch) => Some(default_branch.clone()),
            DefaultBranch::NotLoaded | DefaultBranch::Loading | DefaultBranch::Missing => None,
        })
    }

    fn is_resolving(&self) -> bool {
        self.picked.is_none()
            && matches!(
                self.default,
                DefaultBranch::NotLoaded | DefaultBranch::Loading
            )
    }

    fn is_unavailable(&self) -> bool {
        self.picked.is_none() && self.default == DefaultBranch::Missing
    }

    /// Whether showing branch changes has to wait for a lookup to know
    /// between switching modes and opening the base picker.
    fn needs_lookup_to_decide(&self) -> bool {
        self.picked.is_none()
            && match self.default {
                DefaultBranch::NotLoaded | DefaultBranch::Loading => true,
                DefaultBranch::Missing => self.default_is_stale,
                DefaultBranch::Found(_) => false,
            }
    }

    fn needs_lookup(&self) -> bool {
        self.picked.is_none()
            && match self.default {
                DefaultBranch::NotLoaded => true,
                DefaultBranch::Loading => false,
                DefaultBranch::Found(_) | DefaultBranch::Missing => self.default_is_stale,
            }
    }
}

impl GitPanel {
    pub(super) fn is_branch_mode(&self) -> bool {
        matches!(self.changes_mode, ChangesMode::Branch(_))
    }

    pub(super) fn branch_list(&self) -> Option<&Entity<DiffBufferList>> {
        match &self.changes_mode {
            ChangesMode::Branch(Some(branch_list)) => Some(&branch_list.list),
            ChangesMode::Uncommitted | ChangesMode::Branch(None) => None,
        }
    }

    pub(super) fn enter_branch_mode(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_branch_mode() {
            return;
        }
        self.ensure_default_branch(window, cx);
        self.changes_mode = ChangesMode::Branch(None);
        self.changes_mode_changed(window, cx);
    }

    pub(super) fn enter_uncommitted_mode(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // A request waiting on the lookup usually leaves the panel in
        // uncommitted mode, so this drops it before the early return.
        self.cancel_pending_branch_changes();
        if !self.is_branch_mode() {
            return;
        }
        self.changes_mode = ChangesMode::Uncommitted;
        // The next switch to branch mode starts again from the active tab or
        // the default branch, rather than from a base picked long ago.
        self.branch_base.picked = None;
        self.changes_mode_changed(window, cx);
    }

    fn changes_mode_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.clear_marks();
        self.bulk_staging.take();
        self.context_menu.take();
        self.selected_entry = None;
        self.update_visible_entries(window, cx);
    }

    pub(super) fn set_branch_base(
        &mut self,
        base_ref: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.branch_base.picked.as_ref() == Some(&base_ref) {
            return;
        }
        self.branch_base.picked = Some(base_ref);
        if self.is_branch_mode() {
            self.schedule_visible_entries_update(window, cx);
        }
        cx.notify();
    }

    /// Rows from the previous base would open the new base's diff with their
    /// old statuses, so they can't wait for the update debounce.
    pub(super) fn update_entries_if_base_changed(
        &mut self,
        previous_base: Option<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_branch_mode() && self.branch_base.effective() != previous_base {
            self.update_visible_entries(window, cx);
        }
    }

    pub(super) fn show_branch_changes(
        &mut self,
        _: &ShowBranchChanges,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_branch_changes_for(
            ShowBranchChangesRequest {
                from_empty_state: false,
            },
            window,
            cx,
        );
    }

    pub(super) fn show_branch_changes_and_open_diff(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_branch_changes_for(
            ShowBranchChangesRequest {
                from_empty_state: true,
            },
            window,
            cx,
        );
    }

    fn show_branch_changes_for(
        &mut self,
        request: ShowBranchChangesRequest,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_active_tab(GitPanelTab::Changes, window, cx);
        if self.active_repository.is_none() {
            return;
        }
        // The empty state's button opens the default branch's diff, as it
        // did before branch mode existed.
        if !request.from_empty_state {
            let previous_base = self.branch_base.effective();
            self.adopt_followed_branch_diff_base(window, cx);
            self.update_entries_if_base_changed(previous_base, window, cx);
        }
        // A pending request means a lookup is already running, so this one
        // replaces it rather than acting on a default that's being refreshed
        // and then having the stale request run on top of it.
        if self.branch_base.pending_show.is_some() || self.branch_base.needs_lookup_to_decide() {
            self.ensure_default_branch(window, cx);
            self.show_after_lookup(request, window, cx);
            return;
        }
        if self.branch_base.is_unavailable() {
            self.choose_branch_base_for(request, window, cx);
            return;
        }
        self.enter_branch_mode(window, cx);
        if request.from_empty_state {
            self.open_branch_diff(None, window, cx);
        }
    }

    pub(super) fn show_branch_changes_since(
        &mut self,
        base_ref: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.cancel_pending_branch_changes();
        self.set_active_tab(GitPanelTab::Changes, window, cx);
        let was_branch_mode = self.is_branch_mode();
        self.set_branch_base(base_ref, window, cx);
        if was_branch_mode {
            // Sync now rather than after the update debounce, so the new base
            // starts loading right away.
            self.update_visible_entries(window, cx);
        } else {
            self.enter_branch_mode(window, cx);
        }
    }

    fn on_base_picked(
        panel: WeakEntity<GitPanel>,
        request: ShowBranchChangesRequest,
    ) -> impl Fn(Branch, &mut Window, &mut App) + 'static {
        move |branch, window, cx| {
            let base_ref: SharedString = branch.name().to_owned().into();
            panel
                .update(cx, |panel, cx| {
                    panel.show_branch_changes_since(base_ref, window, cx);
                    if request.from_empty_state {
                        panel.open_branch_diff(None, window, cx);
                    }
                })
                .ok();
        }
    }

    pub(super) fn choose_branch_base(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Otherwise the pending request could open its own picker once the
        // lookup finishes, which toggles this one closed.
        self.cancel_pending_branch_changes();
        self.choose_branch_base_for(
            ShowBranchChangesRequest {
                from_empty_state: false,
            },
            window,
            cx,
        );
    }

    fn choose_branch_base_for(
        &mut self,
        request: ShowBranchChangesRequest,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(repository) = self.active_repository.clone() else {
            return;
        };
        let selected_branch = self.branch_base.effective();
        let on_select: branch_picker::SelectBranchCallback =
            Arc::new(Self::on_base_picked(cx.weak_entity(), request));
        let workspace = self.workspace.clone();
        // Workspace actions call this while the workspace is being updated, so
        // the modal opens once that update has finished.
        window.defer(cx, move |window, cx| {
            let workspace_handle = workspace.clone();
            workspace
                .update(cx, |workspace, cx| {
                    workspace.toggle_modal(window, cx, |window, cx| {
                        branch_picker::select_modal(
                            workspace_handle,
                            Some(repository),
                            selected_branch,
                            on_select,
                            window,
                            cx,
                        )
                    });
                })
                .ok();
        });
    }

    /// A request waiting on the default branch lookup runs once the lookup
    /// finishes, so anything the user does in the meantime that it would undo
    /// has to drop it. The lookup itself keeps running.
    pub(super) fn cancel_pending_branch_changes(&mut self) {
        // A request can adopt the active tab's base before it waits, and
        // outside branch mode that base would outlive it.
        if self.branch_base.pending_show.take().is_some() && !self.is_branch_mode() {
            self.branch_base.picked = None;
        }
    }

    fn show_after_lookup(
        &mut self,
        request: ShowBranchChangesRequest,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let lookup = self.branch_base.default_lookup.clone();
        let wait = cx.spawn_in(window, async move |this, cx| {
            lookup.await;
            this.update_in(cx, |this, window, cx| {
                // This task is the pending one, and dropping a task's handle
                // while it runs would cancel it.
                if let Some(pending) = this.branch_base.pending_show.take() {
                    pending.wait.detach();
                }
                this.show_branch_changes_for(request, window, cx);
            })
            .ok();
        });
        self.branch_base.pending_show = Some(PendingShow { request, wait });
    }

    pub(super) fn show_uncommitted_changes(
        &mut self,
        _: &ShowUncommittedChanges,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_active_tab(GitPanelTab::Changes, window, cx);
        self.enter_uncommitted_mode(window, cx);
    }

    /// Branch names don't carry over between repositories, so switching
    /// repositories starts again from the new one's default branch.
    pub(super) fn reset_branch_state(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let default_lookup_count = self.branch_base.default_lookup_count + 1;
        self.branch_base = BranchBase::new();
        self.branch_base.default_lookup_count = default_lookup_count;
        // The active tab may have become active before the repository switch,
        // when it didn't belong to the active repository yet.
        self.follow_active_branch_diff(window, cx);
        if self.is_branch_mode() {
            self.ensure_default_branch(window, cx);
        }
    }

    pub(super) fn default_branch_may_have_changed(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.branch_base.default_is_stale = true;
        if self.is_branch_mode() {
            self.ensure_default_branch(window, cx);
        }
    }

    fn ensure_default_branch(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.branch_base.needs_lookup() {
            self.resolve_default_branch(window, cx);
        }
    }

    fn resolve_default_branch(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.branch_base.default_lookup_count += 1;
        let lookup_id = self.branch_base.default_lookup_count;
        self.branch_base.default_is_stale = false;
        let Some(repo) = self.active_repository.clone() else {
            self.branch_base.default = DefaultBranch::Missing;
            return;
        };
        if self.branch_base.default == DefaultBranch::NotLoaded {
            self.branch_base.default = DefaultBranch::Loading;
        }
        let default_branch = repo.update(cx, |repo, _| repo.default_branch(true));
        let lookup = cx.spawn_in(window, async move |this, cx| {
            let default_branch = match default_branch.await.ok().and_then(ResultExt::log_err) {
                Some(Some(default_branch)) => DefaultBranch::Found(default_branch),
                Some(None) | None => DefaultBranch::Missing,
            };
            this.update_in(cx, |this, window, cx| {
                if this.branch_base.default_lookup_count != lookup_id {
                    return;
                }
                if this.branch_base.default != default_branch {
                    let previous_base = this.branch_base.effective();
                    this.branch_base.default = default_branch;
                    this.schedule_visible_entries_update(window, cx);
                    this.update_entries_if_base_changed(previous_base, window, cx);
                    cx.notify();
                }
            })
            .ok();
        });
        self.branch_base.default_lookup = lookup.shared();
        // A request left on the old lookup would decide before this one answers.
        if let Some(pending) = self.branch_base.pending_show.take() {
            self.show_after_lookup(pending.request, window, cx);
        }
    }

    /// Runs at the start of every entries update, so changes to the mode,
    /// repository or base only need to schedule one.
    pub(super) fn sync_branch_list(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let target = self
            .active_repository
            .clone()
            .zip(self.branch_base.effective());
        let git_store = self.project.read(cx).git_store().clone();
        let ChangesMode::Branch(branch_list) = &mut self.changes_mode else {
            return;
        };
        let Some((repo, base_ref)) = target else {
            *branch_list = None;
            return;
        };
        let diff_base = DiffBase::Merge { base_ref };

        if let Some(branch_list) = branch_list {
            branch_list.list.update(cx, |list, cx| {
                list.set_repo(Some(repo), cx);
                if list.diff_base() != &diff_base {
                    list.set_diff_base(diff_base, cx);
                }
            });
            return;
        }

        let list = cx.new(|cx| DiffBufferList::new(diff_base, git_store, Some(repo), cx));
        let subscription = cx.subscribe_in(
            &list,
            window,
            |this, _, event: &BranchDiffEvent, window, cx| match event {
                BranchDiffEvent::FileListChanged | BranchDiffEvent::DiffBaseChanged => {
                    this.schedule_visible_entries_update(window, cx);
                }
            },
        );
        *branch_list = Some(BranchList {
            list,
            _subscription: subscription,
        });
    }

    pub(super) fn branch_status_entries(&self, cx: &App) -> Vec<GitStatusEntry> {
        let ChangesMode::Branch(Some(branch_list)) = &self.changes_mode else {
            return Vec::new();
        };
        branch_list.shown_files(cx).unwrap_or_default()
    }

    pub(super) fn follow_active_branch_diff(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let Some(branch_diff) = workspace.read(cx).active_item_as::<BranchDiff>(cx) else {
            self.branch_base.followed = None;
            return;
        };
        let list = branch_diff.read(cx).branch_diff_list(cx);
        if self
            .branch_base
            .followed
            .as_ref()
            .is_some_and(|followed| followed.list.entity_id() == list.entity_id())
        {
            return;
        }

        // Uncommitted mode only watches the tab, so a tab looked at long ago
        // doesn't decide the base the next time branch mode opens.
        let subscription = cx.subscribe_in(
            &list,
            window,
            |this, list, event: &BranchDiffEvent, window, cx| {
                if matches!(event, BranchDiffEvent::DiffBaseChanged) && this.is_branch_mode() {
                    let previous_base = this.branch_base.effective();
                    this.adopt_branch_diff_base(list, window, cx);
                    this.update_entries_if_base_changed(previous_base, window, cx);
                }
            },
        );
        self.branch_base.followed = Some(FollowedBranchDiff {
            list: list.downgrade(),
            _subscription: subscription,
        });
        if self.is_branch_mode() {
            self.adopt_branch_diff_base(&list, window, cx);
        }
    }

    /// Reads the watched tab rather than the workspace, because the panel's
    /// actions run while the workspace is being updated.
    fn adopt_followed_branch_diff_base(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(list) = self
            .branch_base
            .followed
            .as_ref()
            .and_then(|followed| followed.list.upgrade())
        {
            self.adopt_branch_diff_base(&list, window, cx);
        }
    }

    fn adopt_branch_diff_base(
        &mut self,
        list: &Entity<DiffBufferList>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let list = list.read(cx);
        let DiffBase::Merge { base_ref } = list.diff_base() else {
            return;
        };
        let is_active_repository = list
            .repo()
            .zip(self.active_repository.as_ref())
            .is_some_and(|(repo, active_repo)| repo.read(cx).id == active_repo.read(cx).id);
        if !is_active_repository {
            return;
        }
        let base_ref = base_ref.clone();
        self.set_branch_base(base_ref, window, cx);
    }

    pub(super) fn open_branch_diff(
        &mut self,
        entry: Option<GitStatusEntry>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let (Some(repo), Some(base_ref)) =
            (self.active_repository.clone(), self.branch_base.effective())
        else {
            return;
        };
        let list = self
            .branch_diff_tab(&base_ref, cx)
            .map(|branch_diff| branch_diff.read(cx).branch_diff_list(cx))
            .or_else(|| self.display_diff_at(&repo, &base_ref, cx));
        let project = self.project.clone();
        self.workspace
            .update(cx, |workspace, cx| {
                BranchDiff::deploy_branch_diff_with_base_ref(
                    workspace, project, repo, base_ref, list, entry, window, cx,
                );
            })
            .ok();
    }

    /// The active branch diff tab, if it's at the panel's base and already
    /// showing this entry.
    pub(super) fn branch_diff_showing(
        &self,
        entry: &GitStatusEntry,
        cx: &App,
    ) -> Option<Entity<BranchDiff>> {
        let base_ref = self.branch_base.effective()?;
        let active = self
            .workspace
            .upgrade()?
            .read(cx)
            .active_item_as::<BranchDiff>(cx)?;
        let project_path = active.read(cx).active_project_path(cx)?;
        let repo_path = self
            .active_repository
            .as_ref()?
            .read(cx)
            .project_path_to_repo_path(&project_path, cx)?;
        (self.is_branch_diff_at(&active, &base_ref, cx) && repo_path == entry.repo_path)
            .then_some(active)
    }

    fn branch_diff_tab(&self, base_ref: &SharedString, cx: &App) -> Option<Entity<BranchDiff>> {
        self.workspace
            .upgrade()?
            .read(cx)
            .items_of_type::<BranchDiff>(cx)
            .find(|branch_diff| self.is_branch_diff_at(branch_diff, base_ref, cx))
    }

    fn is_branch_diff_at(
        &self,
        branch_diff: &Entity<BranchDiff>,
        base_ref: &SharedString,
        cx: &App,
    ) -> bool {
        let Some(repo) = self.active_repository.as_ref() else {
            return false;
        };
        let branch_diff = branch_diff.read(cx);
        matches!(
            branch_diff.diff_base(cx),
            DiffBase::Merge { base_ref: existing } if existing == base_ref
        ) && branch_diff
            .repo(cx)
            .is_some_and(|branch_diff_repo| branch_diff_repo.read(cx).id == repo.read(cx).id)
    }

    /// `DeployBranchDiff` only reuses a tab that holds the display diff's
    /// list, so a new tab at the same base shares it. It's only read here:
    /// `ensure_display_diff` would move the gutter to the panel's base.
    fn display_diff_at(
        &self,
        repo: &Entity<Repository>,
        base_ref: &SharedString,
        cx: &App,
    ) -> Option<Entity<DiffBufferList>> {
        let git_store = self.project.read(cx).git_store().read(cx);
        if git_store.diff_base() != GitDiffBaseSetting::DefaultBranch {
            return None;
        }
        git_store
            .display_diff_for_repo(repo.read(cx).id)
            .filter(|list| {
                matches!(
                    list.read(cx).diff_base(),
                    DiffBase::Merge { base_ref: display_base } if display_base == base_ref
                )
            })
    }

    pub(super) fn move_branch_diff_to_entry(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        maybe!({
            let entry = self
                .entries
                .get(self.selected_entry?)?
                .status_entry()?
                .clone();
            let base_ref = self.branch_base.effective()?;
            let branch_diff = self.branch_diff_tab(&base_ref, cx)?;
            branch_diff.update(cx, |branch_diff, cx| {
                branch_diff.move_to_entry(entry, window, cx);
            });
            Some(())
        });
    }

    pub(super) fn render_changes_mode_menu(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let label: SharedString = match (self.is_branch_mode(), self.branch_base.effective()) {
            (true, Some(base_ref)) => format!("Since {base_ref}").into(),
            (true, None) => "Branch Changes".into(),
            (false, _) => "Uncommitted".into(),
        };
        let panel = cx.weak_entity();

        PopoverMenu::new("git-panel-changes-mode")
            .with_handle(self.changes_mode_menu_handle.clone())
            .trigger_with_tooltip(
                Button::new("git-panel-changes-mode-trigger", label)
                    .label_size(LabelSize::Small)
                    .end_icon(
                        Icon::new(IconName::ChevronDown)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    ),
                Tooltip::text("Choose Which Changes to Show"),
            )
            .menu(move |window, cx| Some(Self::changes_mode_menu(panel.clone(), window, cx)))
            .anchor(Anchor::TopLeft)
    }

    pub(super) fn changes_mode_menu(
        panel: WeakEntity<GitPanel>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<ContextMenu> {
        let is_branch_mode = panel
            .read_with(cx, |panel, _| panel.is_branch_mode())
            .unwrap_or(false);
        ContextMenu::build(window, cx, move |menu, _, _| {
            // The check goes at the end so "Uncommitted" lines up with the
            // submenu trigger, which has no check slot.
            menu.toggleable_entry(
                "Uncommitted",
                !is_branch_mode,
                IconPosition::End,
                Some(ShowUncommittedChanges.boxed_clone()),
                {
                    let panel = panel.clone();
                    move |window, cx| {
                        panel
                            .update(cx, |panel, cx| {
                                panel.enter_uncommitted_mode(window, cx);
                            })
                            .ok();
                    }
                },
            )
            .submenu("Compare Against Base", move |menu, window, cx| {
                Self::compare_against_base_submenu(menu, panel.clone(), window, cx)
            })
        })
    }

    fn compare_against_base_submenu(
        menu: ContextMenu,
        panel: WeakEntity<GitPanel>,
        window: &mut Window,
        cx: &mut Context<ContextMenu>,
    ) -> ContextMenu {
        let Ok((workspace, repository, selected_branch, menu_handle)) =
            panel.update(cx, |panel, _| {
                // The user is choosing the base now, so a waiting request shouldn't.
                panel.cancel_pending_branch_changes();
                (
                    panel.workspace.clone(),
                    panel.active_repository.clone(),
                    panel.branch_base.effective(),
                    panel.changes_mode_menu_handle.clone(),
                )
            })
        else {
            return menu;
        };
        let picked = Arc::new(AtomicBool::new(false));
        let on_select: branch_picker::SelectBranchCallback = Arc::new({
            let picked = picked.clone();
            let show_branch_changes_since = Self::on_base_picked(
                panel,
                ShowBranchChangesRequest {
                    from_empty_state: false,
                },
            );
            move |branch: Branch, window: &mut Window, cx: &mut App| {
                picked.store(true, Ordering::Relaxed);
                show_branch_changes_since(branch, window, cx);
            }
        });
        let branch_list = cx.new(|cx| {
            branch_picker::select_embedded(
                workspace,
                repository,
                selected_branch,
                on_select,
                window,
                cx,
            )
        });

        cx.subscribe_in(
            &branch_list,
            window,
            move |menu, _, _: &DismissEvent, window, cx| {
                if picked.load(Ordering::Relaxed) {
                    menu_handle.hide(cx);
                } else {
                    menu.select_submenu_parent(&menu::SelectParent, window, cx);
                }
            },
        )
        .detach();

        let picker_focus_handle = branch_list.focus_handle(cx);
        menu.embedded_view(branch_list)
            .focus_content(picker_focus_handle)
    }

    pub(super) fn render_no_branch_changes_ui(&self, cx: &Context<Self>) -> AnyElement {
        let branch_list = self.branch_list().map(|branch_list| branch_list.read(cx));
        let message: SharedString = if branch_list
            .is_some_and(|branch_list| branch_list.tree_diff_error().is_some())
        {
            "Couldn't load branch changes".into()
        } else if branch_list.is_some_and(|branch_list| branch_list.statuses_by_path().is_none())
            || self.branch_base.is_resolving()
        {
            // Only the first load for a base shows this, so refreshes after
            // every status change don't flicker.
            "Loading branch changes…".into()
        } else if let Some(base_ref) = self.branch_base.effective() {
            format!("No changes since {base_ref}").into()
        } else {
            "No default branch found. Pick a base branch.".into()
        };

        h_flex()
            .px_4()
            .child(Label::new(message).color(Color::Muted))
            .into_any_element()
    }

    pub(super) fn build_branch_context_menu(
        &self,
        file_count: usize,
        target_kind: SelectionTargetKind,
        has_history: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let is_file_or_bulk = !matches!(target_kind, SelectionTargetKind::Directory);
        let titles = EntryMenuTitles::new(file_count);
        ContextMenu::build(window, cx, |context_menu, _, _| {
            context_menu
                .context(self.focus_handle.clone())
                .when(is_file_or_bulk, |context_menu| {
                    context_menu
                        .action("Open Diff", menu::Confirm.boxed_clone())
                        .action(titles.view_file, ViewFile.boxed_clone())
                        .separator()
                })
                .action(titles.copy_path, CopyPath.boxed_clone())
                .action(titles.copy_relative_path, CopyRelativePath.boxed_clone())
                .when(is_file_or_bulk && has_history, |context_menu| {
                    context_menu
                        .separator()
                        .action(titles.view_file_history, Box::new(git::FileHistory))
                })
        })
    }
}

fn listed_files(list: &DiffBufferList) -> Option<Vec<GitStatusEntry>> {
    let statuses = list.statuses_by_path()?;
    Some(
        statuses
            .iter()
            .map(|status_entry| GitStatusEntry {
                repo_path: status_entry.repo_path.clone(),
                status: status_entry.status,
                staging: status_entry.status.staging(),
                // Per-file stats against the merge base aren't computed yet.
                diff_stat: None,
            })
            .collect(),
    )
}
