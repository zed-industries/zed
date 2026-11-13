use crate::{
    branch_picker,
    diff_multibuffer::DiffMultibuffer,
    git_panel::GitStatusEntry,
    project_diff::{
        self, CompareWithBranch, DeployBranchDiff, ProjectDiff, ReviewDiff,
        render_send_review_to_agent_button,
    },
};
use agent_settings::AgentSettings;
use anyhow::{Context as _, Result, anyhow};
use editor::{
    Addon, Editor, EditorEvent, HiddenDiffHunkRenderer, SplittableEditor,
    actions::SendReviewToAgent,
};
use git::{repository::DiffType, status::FileStatus};
use gpui::{
    Action, App, AppContext as _, Entity, EventEmitter, FocusHandle, Focusable, Render,
    SharedString, Subscription, Task, WeakEntity,
};
use language::{BufferId, Capability};
use project::{
    Project, ProjectPath,
    git_store::{
        Repository,
        diff_buffer_list::{self, DiffBase},
    },
};
use settings::{GitDiffBaseSetting, Settings};
use std::{
    any::{Any, TypeId},
    sync::Arc,
};
use ui::{DiffStat, Divider, PopoverMenu, Tooltip, prelude::*};
use workspace::{
    ItemHandle, ItemNavHistory, SerializableItem, ToolbarItemEvent, ToolbarItemLocation,
    ToolbarItemView, Workspace,
    item::{Item, ItemEvent, SaveOptions},
    notifications::NotifyTaskExt,
    searchable::SearchableItemHandle,
};
use zed_actions::agent::ReviewBranchDiff;

/// The workspace item for a branch (merge-base) diff: "Changes since {branch}".
/// It wraps a single [`DiffMultibuffer`] over [`DiffBase::Merge`] and delegates
/// the [`Item`] surface to it. The merge base can be changed in place via the
/// [`BranchDiffToolbar`]'s branch picker, which reloads without reconfiguring
/// the editor (the merge styling is identical for every base ref).
pub struct BranchDiff {
    diff: Entity<DiffMultibuffer>,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    _diff_event_subscription: Subscription,
}

struct BranchDiffAddon {
    branch_diff: Entity<diff_buffer_list::DiffBufferList>,
}

impl Addon for BranchDiffAddon {
    fn git_diff_base_for_open_excerpts(
        &self,
        _: &App,
    ) -> Option<Entity<diff_buffer_list::DiffBufferList>> {
        Some(self.branch_diff.clone())
    }

    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn override_status_for_buffer_id(&self, buffer_id: BufferId, cx: &App) -> Option<FileStatus> {
        self.branch_diff
            .read(cx)
            .status_for_buffer_id(buffer_id, cx)
    }
}

impl BranchDiff {
    pub(crate) fn register(workspace: &mut Workspace, cx: &mut Context<Workspace>) {
        workspace.register_action(|workspace, _: &DeployBranchDiff, window, cx| {
            Self::deploy_branch_diff(workspace, window, cx)
        });
        workspace.register_action(Self::compare_with_branch);
        workspace::register_serializable_item::<Self>(cx);
    }

    pub(crate) fn deploy_branch_diff(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        telemetry::event!("Git Branch Diff Opened");
        let project = workspace.project().clone();
        let Some(intended_repo) = project.read(cx).active_repository(cx) else {
            let workspace = cx.entity().downgrade();
            window
                .spawn(cx, async |_cx| {
                    let result: Result<()> = Err(anyhow!("No active repository"));
                    result
                })
                .detach_and_notify_err(workspace, window, cx);
            return;
        };

        let default_branch = intended_repo.update(cx, |repo, _| repo.default_branch(true));
        let workspace = cx.entity();
        let workspace_weak = workspace.downgrade();
        window
            .spawn(cx, async move |cx| {
                let base_ref = default_branch.await??;
                workspace.update_in(cx, |workspace, window, cx| {
                    let git_store = project.read(cx).git_store().clone();
                    let Some(base_ref) = base_ref else {
                        ProjectDiff::deploy_at(workspace, None, window, cx);
                        return;
                    };
                    let branch_diff =
                        if git_store.read(cx).diff_base() == GitDiffBaseSetting::DefaultBranch {
                            Some(git_store.update(cx, |git_store, cx| {
                                git_store.ensure_display_diff(
                                    intended_repo.clone(),
                                    base_ref.clone(),
                                    cx,
                                )
                            }))
                        } else {
                            None
                        };
                    Self::deploy_branch_diff_with_base_ref(
                        workspace,
                        project,
                        intended_repo,
                        base_ref,
                        branch_diff,
                        None,
                        window,
                        cx,
                    );
                })?;

                anyhow::Ok(())
            })
            .detach_and_notify_err(workspace_weak, window, cx);
    }

    fn compare_with_branch(
        workspace: &mut Workspace,
        _: &CompareWithBranch,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let project = workspace.project().clone();
        let Some(repository) = project.read(cx).active_repository(cx) else {
            let workspace = cx.entity().downgrade();
            window
                .spawn(cx, async |_cx| {
                    let result: Result<()> = Err(anyhow!("No active repository"));
                    result
                })
                .detach_and_notify_err(workspace, window, cx);
            return;
        };
        let selected_branch = workspace.active_item_as::<Self>(cx).and_then(|item| {
            match item.read(cx).diff_base(cx) {
                DiffBase::Merge { base_ref } => Some(base_ref.clone()),
                DiffBase::Head | DiffBase::Index | DiffBase::Staged => None,
            }
        });
        let workspace_handle = workspace.weak_handle();
        let on_select = Arc::new({
            let repository = repository.clone();
            let workspace = workspace_handle.clone();
            move |branch: git::repository::Branch, window: &mut Window, cx: &mut App| {
                let base_ref: SharedString = branch.name().to_owned().into();
                workspace
                    .update(cx, |workspace, cx| {
                        Self::deploy_branch_diff_with_base_ref(
                            workspace,
                            project.clone(),
                            repository.clone(),
                            base_ref,
                            None,
                            None,
                            window,
                            cx,
                        );
                    })
                    .ok();
            }
        });

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
    }

    pub(crate) fn deploy_branch_diff_with_base_ref(
        workspace: &mut Workspace,
        project: Entity<Project>,
        intended_repo: Entity<Repository>,
        base_ref: SharedString,
        branch_diff: Option<Entity<diff_buffer_list::DiffBufferList>>,
        entry: Option<GitStatusEntry>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let existing = workspace.items_of_type::<Self>(cx).find(|item| {
            let item = item.read(cx);
            matches!(
                item.diff_base(cx),
                DiffBase::Merge { base_ref: existing_base_ref } if existing_base_ref == &base_ref
            ) && item
                .repo(cx)
                .is_some_and(|repo| repo.read(cx).id == intended_repo.read(cx).id)
                && branch_diff.as_ref().is_none_or(|expected| {
                    item.diff.read(cx).branch_diff().entity_id() == expected.entity_id()
                })
        });
        if let Some(existing) = existing {
            workspace.activate_item(&existing, true, true, window, cx);
            if let Some(entry) = entry {
                existing.update(cx, |branch_diff, cx| {
                    branch_diff.move_to_entry(entry, window, cx);
                });
            }
            return;
        }

        let workspace_handle = cx.entity();
        let branch_diff = cx.new(|cx| {
            Self::new_with_base_ref(
                project,
                workspace_handle,
                base_ref,
                Some(intended_repo),
                branch_diff,
                window,
                cx,
            )
        });
        workspace.add_item_to_active_pane(Box::new(branch_diff.clone()), None, true, window, cx);
        if let Some(entry) = entry {
            branch_diff.update(cx, |branch_diff, cx| {
                branch_diff.move_to_entry(entry, window, cx);
            });
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn new_with_default_branch(
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
            let base_ref = main_branch
                .await??
                .context("Could not determine default branch")?;
            cx.update(|window, cx| {
                let git_store = project.read(cx).git_store().clone();
                let branch_diff =
                    if git_store.read(cx).diff_base() == GitDiffBaseSetting::DefaultBranch {
                        Some(git_store.update(cx, |git_store, cx| {
                            git_store.ensure_display_diff(repo.clone(), base_ref.clone(), cx)
                        }))
                    } else {
                        None
                    };
                cx.new(|cx| {
                    Self::new_with_base_ref(
                        project,
                        workspace,
                        base_ref,
                        Some(repo),
                        branch_diff,
                        window,
                        cx,
                    )
                })
            })
        })
    }

    pub(crate) fn new_with_base_ref(
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        base_ref: SharedString,
        repo: Option<Entity<Repository>>,
        branch_diff: Option<Entity<diff_buffer_list::DiffBufferList>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let branch_diff = branch_diff.unwrap_or_else(|| {
            let git_store = project.read(cx).git_store().clone();
            cx.new(|cx| {
                diff_buffer_list::DiffBufferList::new(
                    DiffBase::Merge { base_ref },
                    git_store,
                    repo,
                    cx,
                )
            })
        });
        let branch_diff_for_addon = branch_diff.clone();
        let diff = cx.new(|cx| {
            DiffMultibuffer::new(
                branch_diff,
                Capability::ReadWrite,
                "No changes",
                move |editor, cx| {
                    editor.set_diff_hunk_renderer(Some(Arc::new(HiddenDiffHunkRenderer)), cx);
                    editor.rhs_editor().update(cx, move |rhs_editor, _cx| {
                        rhs_editor.set_read_only(false);
                        rhs_editor.register_addon(BranchDiffAddon {
                            branch_diff: branch_diff_for_addon,
                        });
                    });
                },
                project.clone(),
                workspace.clone(),
                window,
                cx,
            )
        });
        Self::from_diff(diff, project, workspace, cx)
    }

    fn from_diff(
        diff: Entity<DiffMultibuffer>,
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let diff_event_subscription = cx.subscribe(&diff, |_, _, event: &EditorEvent, cx| {
            cx.emit(event.clone())
        });
        Self {
            diff,
            project,
            workspace: workspace.downgrade(),
            _diff_event_subscription: diff_event_subscription,
        }
    }

    pub(crate) fn diff_base<'a>(&'a self, cx: &'a App) -> &'a DiffBase {
        self.diff.read(cx).diff_base(cx)
    }

    pub(crate) fn repo(&self, cx: &App) -> Option<Entity<Repository>> {
        self.diff.read(cx).repo(cx)
    }

    pub(crate) fn branch_diff_list(&self, cx: &App) -> Entity<diff_buffer_list::DiffBufferList> {
        self.diff.read(cx).branch_diff().clone()
    }

    pub(crate) fn move_to_entry(
        &mut self,
        entry: GitStatusEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.diff
            .update(cx, |diff, cx| diff.move_to_entry(entry, window, cx));
    }

    pub(crate) fn autoscroll(&self, cx: &mut Context<Self>) {
        self.diff.update(cx, |diff, cx| diff.autoscroll(cx));
    }

    fn set_merge_base(&mut self, base_ref: SharedString, cx: &mut Context<Self>) {
        self.diff.update(cx, |diff, cx| {
            diff.branch_diff().update(cx, |branch_diff, cx| {
                branch_diff.set_diff_base(DiffBase::Merge { base_ref }, cx);
            });
        });
    }

    fn review_diff(&mut self, _: &ReviewDiff, window: &mut Window, cx: &mut Context<Self>) {
        let DiffBase::Merge { base_ref } = self.diff_base(cx).clone() else {
            return;
        };
        let Some(repo) = self.repo(cx) else {
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
                                    base_ref,
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

    #[cfg(any(test, feature = "test-support"))]
    pub fn editor(&self, cx: &App) -> Entity<SplittableEditor> {
        self.diff.read(cx).editor().clone()
    }
}

impl EventEmitter<EditorEvent> for BranchDiff {}

impl Focusable for BranchDiff {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.diff.read(cx).focus_handle(cx)
    }
}

impl Item for BranchDiff {
    type Event = EditorEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::GitBranch).color(Color::Muted))
    }

    fn to_item_events(event: &EditorEvent, f: &mut dyn FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.diff
            .update(cx, |diff, cx| diff.deactivated(window, cx));
    }

    fn navigate(
        &mut self,
        data: Arc<dyn Any + Send>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.diff
            .update(cx, |diff, cx| diff.navigate(data, window, cx))
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        Some(self.tab_content_text(0, cx))
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        match self.diff_base(cx) {
            DiffBase::Merge { base_ref } => format!("Changes since {}", base_ref).into(),
            DiffBase::Head | DiffBase::Index | DiffBase::Staged => "Changes".into(),
        }
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Branch Diff Opened")
    }

    fn as_searchable(&self, _: &Entity<Self>, cx: &App) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.diff.read(cx).editor().clone()))
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.diff.read(cx).for_each_project_item(cx, f);
    }

    fn active_project_path(&self, cx: &App) -> Option<ProjectPath> {
        self.diff.read(cx).active_project_path(cx)
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.diff
            .update(cx, |diff, cx| diff.set_nav_history(nav_history, cx));
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
        let DiffBase::Merge { base_ref } = self.diff_base(cx).clone() else {
            return Task::ready(None);
        };
        let repo = self.repo(cx);
        let project = self.project.clone();
        let branch_diff = self.diff.read(cx).branch_diff().clone();
        Task::ready(Some(cx.new(|cx| {
            Self::new_with_base_ref(
                project,
                workspace,
                base_ref,
                repo,
                Some(branch_diff),
                window,
                cx,
            )
        })))
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.diff.read(cx).is_dirty(cx)
    }

    fn has_conflict(&self, cx: &App) -> bool {
        self.diff.read(cx).has_conflict(cx)
    }

    fn can_save(&self, _cx: &App) -> bool {
        true
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.diff
            .update(cx, |diff, cx| diff.save(options, project, window, cx))
    }

    fn save_as(
        &mut self,
        _: Entity<Project>,
        _: ProjectPath,
        _: &mut Window,
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
        self.diff
            .update(cx, |diff, cx| diff.reload(project, window, cx))
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        cx: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<DiffMultibuffer>() {
            Some(self.diff.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            Some(
                self.diff
                    .read(cx)
                    .editor()
                    .read(cx)
                    .rhs_editor()
                    .clone()
                    .into(),
            )
        } else if type_id == TypeId::of::<SplittableEditor>() {
            Some(self.diff.read(cx).editor().clone().into())
        } else if type_id == TypeId::of::<diff_buffer_list::DiffBufferList>() {
            Some(self.diff.read(cx).branch_diff().clone().into())
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
        self.diff.update(cx, |diff, cx| {
            diff.added_to_workspace(workspace, window, cx)
        });
    }
}

impl Render for BranchDiff {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .on_action(cx.listener(Self::review_diff))
            .child(self.diff.clone())
    }
}

impl SerializableItem for BranchDiff {
    fn serialized_item_kind() -> &'static str {
        "BranchDiff"
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
        let db = project_diff::persistence::ProjectDiffDb::global(cx);
        window.spawn(cx, async move |cx| {
            let diff_base = db.get_project_diff_base(item_id, workspace_id)?;
            let DiffBase::Merge { base_ref } = diff_base else {
                anyhow::bail!("expected a merge base for a branch diff");
            };
            let workspace = workspace.upgrade().context("workspace gone")?;
            cx.update(|window, cx| {
                cx.new(|cx| {
                    Self::new_with_base_ref(project, workspace, base_ref, None, None, window, cx)
                })
            })
        })
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: workspace::ItemId,
        _closing: bool,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let DiffBase::Merge { base_ref } = self.diff_base(cx).clone() else {
            return None;
        };
        let diff_base = DiffBase::Merge { base_ref };
        let db = project_diff::persistence::ProjectDiffDb::global(cx);
        Some(cx.background_spawn(async move {
            db.save_project_diff_base(item_id, workspace_id, diff_base)
                .await
        }))
    }

    fn should_serialize(&self, _: &Self::Event) -> bool {
        false
    }
}

pub struct BranchDiffToolbar {
    branch_diff: Option<WeakEntity<BranchDiff>>,
}

impl BranchDiffToolbar {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { branch_diff: None }
    }

    fn branch_diff(&self, _: &App) -> Option<Entity<BranchDiff>> {
        self.branch_diff.as_ref()?.upgrade()
    }

    fn dispatch_action(&self, action: &dyn Action, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(branch_diff) = self.branch_diff(cx) {
            branch_diff.focus_handle(cx).focus(window, cx);
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
        self.branch_diff = active_pane_item
            .and_then(|item| item.act_as::<BranchDiff>(cx))
            .map(|entity| entity.downgrade());
        if self.branch_diff.is_some() {
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
        let Some(branch_diff) = self.branch_diff(cx) else {
            return div();
        };
        let focus_handle = branch_diff.focus_handle(cx);
        let review_count = branch_diff
            .read(cx)
            .diff
            .read(cx)
            .total_review_comment_count();
        let (additions, deletions) = branch_diff
            .read(cx)
            .diff
            .read(cx)
            .calculate_changed_lines(cx);
        let diff_base = branch_diff.read(cx).diff_base(cx).clone();
        let DiffBase::Merge { base_ref } = diff_base else {
            return div();
        };
        let selected_base_ref = base_ref.clone();
        let base_ref_label = format!("Base: {base_ref}");
        let repository = branch_diff.read(cx).repo(cx);
        let workspace = branch_diff.read(cx).workspace.clone();
        let project = branch_diff.read(cx).project.clone();
        let buffers = branch_diff.read(cx).diff.read(cx).branch_diff().clone();
        let uses_display_diff = repository.as_ref().is_some_and(|repository| {
            project
                .read(cx)
                .git_store()
                .read(cx)
                .display_diff_for_repo(repository.read(cx).id)
                .is_some_and(|display_diff| display_diff.entity_id() == buffers.entity_id())
        });
        let view_for_picker = branch_diff.downgrade();

        let is_multibuffer_empty = branch_diff
            .read(cx)
            .diff
            .read(cx)
            .multibuffer()
            .read(cx)
            .is_empty();
        let is_ai_enabled = AgentSettings::get_global(cx).enabled(cx);

        let show_review_button = !is_multibuffer_empty && is_ai_enabled;

        h_flex()
            .my_neg_1()
            .py_1()
            .gap_1p5()
            .flex_wrap()
            .justify_between()
            .when(!is_multibuffer_empty, |this| {
                this.child(DiffStat::new(
                    "branch-diff-stat",
                    additions as usize,
                    deletions as usize,
                ))
            })
            .child(Divider::vertical().ml_1())
            .child(
                PopoverMenu::new("branch-diff-base-branch-picker")
                    .menu(move |window, cx| {
                        let view_for_picker = view_for_picker.clone();
                        let workspace_for_select = workspace.clone();
                        let repository_for_select = repository.clone();
                        let project = project.clone();
                        let on_select = Arc::new(
                            move |branch: git::repository::Branch,
                                  window: &mut Window,
                                  cx: &mut App| {
                                let base_ref: SharedString = branch.name().to_owned().into();
                                if uses_display_diff {
                                    let Some(repository) = repository_for_select.clone() else {
                                        return;
                                    };
                                    workspace_for_select
                                        .update(cx, |workspace, cx| {
                                            BranchDiff::deploy_branch_diff_with_base_ref(
                                                workspace,
                                                project.clone(),
                                                repository,
                                                base_ref,
                                                None,
                                                None,
                                                window,
                                                cx,
                                            );
                                        })
                                        .ok();
                                } else {
                                    view_for_picker
                                        .update(cx, |branch_diff, cx| {
                                            branch_diff.set_merge_base(base_ref, cx);
                                            cx.notify();
                                        })
                                        .ok();
                                }
                            },
                        );

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
                        Button::new("branch-diff-base-branch", base_ref_label).end_icon(
                            Icon::new(IconName::ChevronDown)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        ),
                        Tooltip::text("Select Base Branch"),
                    ),
            )
            .when(show_review_button, |this| {
                let focus_handle = focus_handle.clone();
                this.child(Divider::vertical()).child(
                    Button::new("review-diff", "Review Diff")
                        .start_icon(
                            Icon::new(IconName::ZedAssistant)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        )
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
                this.child(Divider::vertical()).child(
                    render_send_review_to_agent_button(review_count, &focus_handle).on_click(
                        cx.listener(|this, _, window, cx| {
                            this.dispatch_action(&SendReviewToAgent, window, cx)
                        }),
                    ),
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use collections::HashMap;
    use editor::test::editor_test_context::assert_state_with_diff;
    use git::status::{FileStatus, TrackedStatus, UnmergedStatus, UnmergedStatusCode};
    use gpui::{TestAppContext, VisualTestContext};
    use project::{FakeFs, project_settings::ProjectSettings};
    use serde_json::json;
    use settings::{DiffViewStyle, SettingsStore};
    use std::path::Path;
    use std::sync::Arc;
    use unindent::Unindent as _;
    use util::{
        path,
        rel_path::{RelPath, rel_path},
    };
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
        let _branch_diff = cx
            .update(|window, cx| {
                BranchDiff::new_with_default_branch(project.clone(), workspace, window, cx)
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
                    // conflict diff identity without reaching into view internals.
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
                BranchDiff::new_with_default_branch(project.clone(), workspace, window, cx)
            })
            .await
            .unwrap();
        cx.run_until_parked();

        fs.set_head_for_repo(
            Path::new(path!("/project/.git")),
            &[("a.txt", "B".into()), ("d.txt", "created-in-head".into())],
            "sha",
        );
        fs.set_merge_base_content_for_repo(
            Path::new(path!("/project/.git")),
            &[
                ("a.txt", "A".into()),
                ("c.txt", "in-merge-base-and-work-tree".into()),
            ],
        );
        cx.run_until_parked();

        let editor = diff.read_with(cx, |diff, cx| diff.editor(cx).read(cx).rhs_editor().clone());

        let expected_diff = "
                - A
                + C
                + ˇnew
                + created-in-head"
            .unindent();
        assert_state_with_diff(&editor, cx, &expected_diff);

        cx.update(|window, cx| {
            editor.focus_handle(cx).focus(window, cx);
            window.dispatch_action(git::Restore.boxed_clone(), cx);
        });
        cx.run_until_parked();
        assert_state_with_diff(&editor, cx, &expected_diff);

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

    const BRANCH_HEAD: &str = "one\ncommitted\nlast";
    const BRANCH_BASE: &str = "one\nbase\nlast";

    struct BranchComparisonFixture {
        fs: Arc<FakeFs>,
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        branch: Entity<BranchDiff>,
        file_editor: Entity<Editor>,
        source_editor: Entity<Editor>,
    }

    async fn branch_comparison_fixture<'a>(
        cx: &'a mut TestAppContext,
        style: DiffViewStyle,
        working: &str,
    ) -> (BranchComparisonFixture, &'a mut VisualTestContext) {
        init_test(cx);
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.editor.diff_view_style = Some(style);
                    settings.git.get_or_insert_default().diff_base =
                        Some(settings::GitDiffBaseSetting::DefaultBranch);
                });
            });
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), json!({".git": {}, "a.txt": working}))
            .await;
        let dot_git = Path::new(path!("/project/.git"));
        fs.set_head_and_index_for_repo(dot_git, &[("a.txt", BRANCH_HEAD.into())]);
        fs.set_merge_base_content_for_repo(dot_git, &[("a.txt", BRANCH_BASE.into())]);
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |workspace, _| workspace.workspace().clone());
        let file_editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, rel_path("a.txt")), None, true, window, cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        let repository = project
            .read_with(cx, |project, cx| project.active_repository(cx))
            .unwrap();
        workspace.update_in(cx, |workspace, window, cx| {
            BranchDiff::deploy_branch_diff_with_base_ref(
                workspace,
                project.clone(),
                repository,
                "review-base".into(),
                None,
                None,
                window,
                cx,
            );
        });
        cx.run_until_parked();
        let branch = workspace
            .read_with(cx, |workspace, cx| {
                workspace.active_item_as::<BranchDiff>(cx)
            })
            .unwrap();
        let source_editor = branch.read_with(cx, |branch, cx| {
            branch.editor(cx).read(cx).rhs_editor().clone()
        });
        cx.update(|window, cx| {
            source_editor.update(cx, |editor, cx| {
                editor.change_selections(
                    editor::SelectionEffects::no_scroll(),
                    window,
                    cx,
                    |selections| {
                        selections
                            .select_ranges([language::Point::zero()..language::Point::zero()]);
                    },
                );
                editor.focus_handle(cx).focus(window, cx);
            });
        });
        (
            BranchComparisonFixture {
                fs,
                project,
                workspace,
                branch,
                file_editor,
                source_editor,
            },
            cx,
        )
    }

    fn editor_diff_base_text(editor: &Editor, cx: &App) -> Option<String> {
        let multibuffer = editor.buffer().read(cx);
        let buffer = multibuffer.as_singleton()?;
        let diff = multibuffer.diff_for(buffer.read(cx).remote_id())?;
        diff.read(cx).snapshot(cx).base_text_string()
    }

    #[gpui::test]
    async fn test_open_branch_excerpt_keeps_comparison(cx: &mut TestAppContext) {
        let (fixture, cx) =
            branch_comparison_fixture(cx, DiffViewStyle::Unified, BRANCH_HEAD).await;
        cx.dispatch_action(editor::actions::OpenExcerpts);
        cx.run_until_parked();
        let opened = fixture
            .workspace
            .read_with(cx, |workspace, cx| workspace.active_item_as::<Editor>(cx))
            .unwrap();
        assert_eq!(opened, fixture.file_editor);
        opened.read_with(cx, |editor, cx| {
            assert!(!editor.read_only(cx));
            assert_eq!(
                editor.git_diff_base_name(cx).as_deref(),
                Some("review-base")
            );
            assert_eq!(
                editor_diff_base_text(editor, cx).as_deref(),
                Some(BRANCH_BASE)
            );
            let buffer = editor.buffer().read(cx).as_singleton().unwrap();
            assert_eq!(buffer.read(cx).text(), BRANCH_HEAD);
            assert!(project::File::from_dyn(buffer.read(cx).file()).is_some());
        });
    }

    #[gpui::test]
    async fn test_open_branch_excerpt_from_split_keeps_comparison(cx: &mut TestAppContext) {
        let (fixture, cx) = branch_comparison_fixture(cx, DiffViewStyle::Split, BRANCH_HEAD).await;
        let source = fixture.branch.read_with(cx, |branch, cx| {
            branch.editor(cx).read(cx).lhs_editor().unwrap().clone()
        });
        cx.update(|window, cx| source.read(cx).focus_handle(cx).focus(window, cx));
        cx.dispatch_action(editor::actions::OpenExcerptsSplit);
        cx.run_until_parked();
        let opened = fixture
            .workspace
            .read_with(cx, |workspace, cx| workspace.active_item_as::<Editor>(cx))
            .unwrap();
        opened.read_with(cx, |editor, cx| {
            assert!(!editor.read_only(cx));
            assert_eq!(
                editor.git_diff_base_name(cx).as_deref(),
                Some("review-base")
            );
            assert_eq!(
                editor_diff_base_text(editor, cx).as_deref(),
                Some(BRANCH_BASE)
            );
            let buffer = editor.buffer().read(cx).as_singleton().unwrap();
            let working_buffer = fixture
                .file_editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .unwrap();
            assert_eq!(buffer, working_buffer);
            assert_eq!(buffer.read(cx).text(), BRANCH_HEAD);
        });
    }

    #[gpui::test]
    async fn test_open_historical_branch_excerpt_opens_worktree(cx: &mut TestAppContext) {
        let working = "local prefix\none\nlocal edit\nlast";
        let (fixture, cx) = branch_comparison_fixture(cx, DiffViewStyle::Unified, working).await;
        let working_buffer = fixture.file_editor.read_with(cx, |editor, cx| {
            editor.buffer().read(cx).as_singleton().unwrap()
        });
        let git_store = fixture
            .project
            .read_with(cx, |project, _| project.git_store().clone());
        let (_, index) = git_store
            .update(cx, |store, cx| {
                store.open_staged_diff(working_buffer.clone(), cx)
            })
            .await
            .unwrap();
        cx.run_until_parked();
        let split_editor = fixture.branch.read_with(cx, |branch, cx| branch.editor(cx));
        let path = fixture.source_editor.read_with(cx, |editor, cx| {
            editor
                .buffer()
                .read(cx)
                .snapshot(cx)
                .buffers_with_paths()
                .next()
                .unwrap()
                .1
                .clone()
        });
        split_editor.update(cx, |editor, cx| {
            editor.remove_excerpts_for_path(path.clone(), cx)
        });
        cx.run_until_parked();
        let snapshot = index.read_with(cx, |buffer, _| buffer.snapshot());
        let diff = cx.new(|cx| buffer_diff::BufferDiff::new(&snapshot, None, None, cx));
        diff.update(cx, |diff, cx| {
            diff.set_base_text(Some(BRANCH_BASE.into()), snapshot.text, cx)
        })
        .await;
        split_editor.update(cx, |editor, cx| {
            let end = index.read(cx).max_point();
            editor.update_excerpts_for_path(
                path,
                index.clone(),
                vec![language::Point::zero()..end],
                3,
                diff,
                cx,
            );
            editor
                .rhs_editor()
                .update(cx, |editor, _| editor.set_read_only(true));
        });
        fixture
            .branch
            .update(cx, |branch, cx| branch.diff.update(cx, |_, cx| cx.notify()));
        cx.run_until_parked();
        cx.update(|window, cx| {
            fixture.source_editor.update(cx, |editor, cx| {
                editor.change_selections(
                    editor::SelectionEffects::no_scroll(),
                    window,
                    cx,
                    |selections| {
                        selections
                            .select_ranges([language::Point::zero()..language::Point::zero()]);
                    },
                );
                editor.focus_handle(cx).focus(window, cx);
            });
        });
        cx.dispatch_action(editor::actions::OpenExcerpts);
        cx.run_until_parked();
        let opened = fixture
            .workspace
            .read_with(cx, |workspace, cx| workspace.active_item_as::<Editor>(cx))
            .unwrap();
        opened.read_with(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).as_singleton().unwrap();
            assert_eq!(buffer, working_buffer);
            assert_ne!(buffer, index);
            assert!(!editor.read_only(cx));
            assert_eq!(buffer.read(cx).text(), working);
            assert_eq!(
                editor.git_diff_base_name(cx).as_deref(),
                Some("review-base")
            );
            assert_eq!(
                editor_diff_base_text(editor, cx).as_deref(),
                Some(BRANCH_BASE)
            );
        });
    }

    #[gpui::test]
    async fn test_branch_comparison_toggle_refreshes_gutters(cx: &mut TestAppContext) {
        let (fixture, cx) =
            branch_comparison_fixture(cx, DiffViewStyle::Unified, BRANCH_HEAD).await;
        cx.dispatch_action(editor::actions::OpenExcerpts);
        cx.run_until_parked();
        let editor = &fixture.file_editor;
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor
                    .display_snapshot(cx)
                    .buffer_snapshot()
                    .diff_hunks()
                    .count(),
                1
            );
        });
        let observed_base = std::rc::Rc::new(std::cell::RefCell::new(None));
        let _subscription = cx.update(|_, cx| {
            let buffer = editor.read(cx).buffer().clone();
            cx.observe(&buffer, {
                let editor = editor.clone();
                let observed_base = observed_base.clone();
                move |_, cx| {
                    *observed_base.borrow_mut() = editor_diff_base_text(editor.read(cx), cx);
                }
            })
        });
        editor.update(cx, |editor, cx| {
            editor.set_branch_comparison_enabled(false, cx)
        });
        cx.run_until_parked();
        assert_eq!(observed_base.borrow().as_deref(), Some(BRANCH_HEAD));
        editor.update(cx, |editor, cx| {
            assert!(editor.git_diff_base_name(cx).is_none());
            assert_eq!(
                editor.git_diff_comparison_branch_name(cx).as_deref(),
                Some("review-base")
            );
            assert_eq!(
                editor
                    .display_snapshot(cx)
                    .buffer_snapshot()
                    .diff_hunks()
                    .count(),
                0
            );
        });
        editor.update(cx, |editor, cx| {
            editor.set_branch_comparison_enabled(true, cx)
        });
        cx.run_until_parked();
        assert_eq!(observed_base.borrow().as_deref(), Some(BRANCH_BASE));
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.git_diff_base_name(cx).as_deref(),
                Some("review-base")
            );
            assert_eq!(
                editor
                    .display_snapshot(cx)
                    .buffer_snapshot()
                    .diff_hunks()
                    .count(),
                1
            );
        });
    }

    #[gpui::test]
    async fn test_branch_comparison_ignores_outdated_diff_loads(cx: &mut TestAppContext) {
        let (fixture, cx) =
            branch_comparison_fixture(cx, DiffViewStyle::Unified, BRANCH_HEAD).await;
        cx.dispatch_action(editor::actions::OpenExcerpts);
        cx.run_until_parked();
        fixture.file_editor.update(cx, |editor, cx| {
            editor.set_branch_comparison_enabled(false, cx);
            editor.set_branch_comparison_enabled(true, cx);
            editor.set_branch_comparison_enabled(false, cx);
        });
        cx.run_until_parked();
        fixture.file_editor.update(cx, |editor, cx| {
            assert!(editor.git_diff_base_name(cx).is_none());
            assert_eq!(
                editor_diff_base_text(editor, cx).as_deref(),
                Some(BRANCH_HEAD)
            );
            assert_eq!(
                editor
                    .display_snapshot(cx)
                    .buffer_snapshot()
                    .diff_hunks()
                    .count(),
                0
            );
        });
    }

    #[gpui::test]
    async fn test_branch_comparison_preserves_edits_and_git_state(cx: &mut TestAppContext) {
        let (fixture, cx) =
            branch_comparison_fixture(cx, DiffViewStyle::Unified, BRANCH_HEAD).await;
        cx.dispatch_action(editor::actions::OpenExcerpts);
        cx.run_until_parked();
        let editor = &fixture.file_editor;
        cx.update(|window, cx| {
            editor.update(cx, |editor, cx| {
                editor.handle_input("local edit ", window, cx);
            })
        });
        cx.run_until_parked();
        let edited_text = editor.read_with(cx, |editor, cx| {
            assert_eq!(
                editor.git_diff_base_name(cx).as_deref(),
                Some("review-base")
            );
            editor
                .buffer()
                .read(cx)
                .as_singleton()
                .unwrap()
                .read(cx)
                .text()
        });
        assert!(edited_text.contains("local edit "));
        editor.update(cx, |editor, cx| {
            editor.set_branch_comparison_enabled(false, cx)
        });
        cx.run_until_parked();
        editor.read_with(cx, |editor, cx| {
            assert_eq!(
                editor_diff_base_text(editor, cx).as_deref(),
                Some(BRANCH_HEAD)
            );
            let buffer = editor.buffer().read(cx).as_singleton().unwrap();
            assert!(buffer.read(cx).is_dirty());
            assert_eq!(buffer.read(cx).text(), edited_text);
        });
        cx.update(|window, cx| {
            editor.update(cx, |editor, cx| {
                editor.undo(&editor::actions::Undo, window, cx);
            })
        });
        cx.run_until_parked();
        editor.update(cx, |editor, cx| {
            assert!(editor.git_diff_base_name(cx).is_none());
            assert_eq!(
                editor
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .unwrap()
                    .read(cx)
                    .text(),
                BRANCH_HEAD
            );
            assert_eq!(
                editor
                    .display_snapshot(cx)
                    .buffer_snapshot()
                    .diff_hunks()
                    .count(),
                0
            );
        });
        fixture
            .fs
            .with_git_state(Path::new(path!("/project/.git")), false, |state| {
                assert_eq!(
                    state.index_contents.values().next().unwrap().as_slice(),
                    BRANCH_HEAD.as_bytes()
                );
            })
            .unwrap();
        cx.read(|cx| {
            assert_eq!(
                ProjectSettings::get_global(cx).git.diff_base,
                GitDiffBaseSetting::DefaultBranch
            )
        });
    }

    #[gpui::test]
    async fn test_branch_comparison_is_independent_between_editors(cx: &mut TestAppContext) {
        let (fixture, cx) =
            branch_comparison_fixture(cx, DiffViewStyle::Unified, BRANCH_HEAD).await;
        let ordinary = cx.update(|window, cx| {
            fixture
                .file_editor
                .update(cx, |editor, cx| cx.new(|cx| editor.clone(window, cx)))
        });
        cx.dispatch_action(editor::actions::OpenExcerpts);
        cx.run_until_parked();
        let inherited = cx.update(|window, cx| {
            fixture
                .file_editor
                .update(cx, |editor, cx| cx.new(|cx| editor.clone(window, cx)))
        });
        fixture.file_editor.update(cx, |editor, cx| {
            editor.set_branch_comparison_enabled(false, cx)
        });
        cx.run_until_parked();
        inherited.read_with(cx, |editor, cx| {
            assert_eq!(
                editor.git_diff_base_name(cx).as_deref(),
                Some("review-base")
            );
            assert_eq!(
                editor_diff_base_text(editor, cx).as_deref(),
                Some(BRANCH_BASE)
            );
        });
        let uncommitted = cx.update(|window, cx| {
            fixture
                .file_editor
                .update(cx, |editor, cx| cx.new(|cx| editor.clone(window, cx)))
        });
        cx.run_until_parked();
        uncommitted.read_with(cx, |editor, cx| {
            assert!(editor.git_diff_base_name(cx).is_none());
            assert_eq!(
                editor_diff_base_text(editor, cx).as_deref(),
                Some(BRANCH_HEAD)
            );
        });
        uncommitted.update(cx, |editor, cx| {
            editor.set_branch_comparison_enabled(true, cx)
        });
        cx.run_until_parked();
        uncommitted.read_with(cx, |editor, cx| {
            assert_eq!(
                editor.git_diff_base_name(cx).as_deref(),
                Some("review-base")
            );
            assert_eq!(
                editor_diff_base_text(editor, cx).as_deref(),
                Some(BRANCH_BASE)
            );
        });
        fixture.file_editor.read_with(cx, |editor, cx| {
            assert!(editor.git_diff_base_name(cx).is_none());
            assert_eq!(
                editor_diff_base_text(editor, cx).as_deref(),
                Some(BRANCH_HEAD)
            );
        });
        ordinary.read_with(cx, |editor, cx| {
            assert!(editor.git_diff_comparison_branch_name(cx).is_none());
            assert_eq!(
                editor_diff_base_text(editor, cx).as_deref(),
                Some(BRANCH_BASE)
            );
        });
    }

    #[gpui::test]
    async fn test_branch_comparison_follows_source_base_changes(cx: &mut TestAppContext) {
        let (fixture, cx) =
            branch_comparison_fixture(cx, DiffViewStyle::Unified, BRANCH_HEAD).await;
        cx.dispatch_action(editor::actions::OpenExcerpts);
        cx.run_until_parked();
        let uncommitted = cx.update(|window, cx| {
            fixture
                .file_editor
                .update(cx, |editor, cx| cx.new(|cx| editor.clone(window, cx)))
        });
        uncommitted.update(cx, |editor, cx| {
            editor.set_branch_comparison_enabled(false, cx)
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            fixture.file_editor.update(cx, |editor, cx| {
                editor.expand_all_diff_hunks(&editor::actions::ExpandAllDiffHunks, window, cx);
                editor.change_selections(
                    editor::SelectionEffects::no_scroll(),
                    window,
                    cx,
                    |selections| {
                        let point = language::Point::new(1, 0);
                        selections.select_ranges([point..point]);
                    },
                );
            });
        });
        cx.run_until_parked();
        fixture
            .fs
            .with_git_state(Path::new(path!("/project/.git")), true, |state| {
                let oid = git::Oid::from_bytes(&[42; 20]).unwrap();
                state
                    .merge_base_contents
                    .insert(git::repository::RepoPath::new("a.txt").unwrap(), oid);
                state.oids.insert(oid, b"one\nother base\nlast".to_vec());
            })
            .unwrap();
        fixture.branch.update(cx, |branch, cx| {
            branch.set_merge_base("other-base".into(), cx)
        });
        cx.run_until_parked();
        fixture.file_editor.read_with(cx, |editor, cx| {
            assert_eq!(editor.git_diff_base_name(cx).as_deref(), Some("other-base"));
            assert_eq!(
                editor_diff_base_text(editor, cx).as_deref(),
                Some("one\nother base\nlast")
            );
        });
        uncommitted.read_with(cx, |editor, cx| {
            assert!(editor.git_diff_base_name(cx).is_none());
            assert_eq!(
                editor.git_diff_comparison_branch_name(cx).as_deref(),
                Some("other-base")
            );
            assert_eq!(
                editor_diff_base_text(editor, cx).as_deref(),
                Some(BRANCH_HEAD)
            );
        });
        uncommitted.update(cx, |editor, cx| {
            editor.set_branch_comparison_enabled(true, cx)
        });
        cx.run_until_parked();
        uncommitted.read_with(cx, |editor, cx| {
            assert_eq!(editor.git_diff_base_name(cx).as_deref(), Some("other-base"));
            assert_eq!(
                editor_diff_base_text(editor, cx).as_deref(),
                Some("one\nother base\nlast")
            );
        });
    }

    #[gpui::test]
    async fn test_branch_comparison_hunks_cannot_be_restored(cx: &mut TestAppContext) {
        let (fixture, cx) =
            branch_comparison_fixture(cx, DiffViewStyle::Unified, BRANCH_HEAD).await;
        cx.dispatch_action(editor::actions::OpenExcerpts);
        cx.run_until_parked();
        fixture.file_editor.read_with(cx, |editor, cx| {
            let multibuffer = editor.buffer().read(cx);
            let buffer = multibuffer.as_singleton().unwrap();
            assert!(
                multibuffer
                    .diff_for(buffer.read(cx).remote_id())
                    .unwrap()
                    .read(cx)
                    .operations()
                    .is_none()
            );
        });
        cx.update(|window, cx| {
            fixture.file_editor.update(cx, |editor, cx| {
                editor.expand_all_diff_hunks(&editor::actions::ExpandAllDiffHunks, window, cx);
            })
        });
        cx.run_until_parked();
        fixture.file_editor.read_with(cx, |editor, cx| {
            assert!(editor.buffer().read(cx).has_expanded_diff_hunks_in_ranges(
                &[multi_buffer::Anchor::Min..multi_buffer::Anchor::Max],
                cx,
            ));
        });
        cx.update(|window, cx| {
            fixture.file_editor.update(cx, |editor, cx| {
                editor.restore_diff_hunks_in_ranges(
                    vec![multi_buffer::Anchor::Min..multi_buffer::Anchor::Max],
                    window,
                    cx,
                );
            })
        });
        cx.run_until_parked();
        fixture.file_editor.read_with(cx, |editor, cx| {
            assert_eq!(
                editor
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .unwrap()
                    .read(cx)
                    .text(),
                BRANCH_HEAD
            );
        });
        fixture
            .fs
            .with_git_state(Path::new(path!("/project/.git")), false, |state| {
                assert_eq!(
                    state.index_contents.values().next().unwrap().as_slice(),
                    BRANCH_HEAD.as_bytes()
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_open_branch_excerpt_reenables_comparison(cx: &mut TestAppContext) {
        let (fixture, cx) =
            branch_comparison_fixture(cx, DiffViewStyle::Unified, BRANCH_HEAD).await;
        cx.dispatch_action(editor::actions::OpenExcerpts);
        cx.run_until_parked();
        fixture.file_editor.update(cx, |editor, cx| {
            editor.set_branch_comparison_enabled(false, cx)
        });
        cx.run_until_parked();
        fixture.workspace.update_in(cx, |workspace, window, cx| {
            workspace.activate_item(&fixture.branch, true, true, window, cx);
        });
        cx.run_until_parked();
        cx.update(|window, cx| {
            fixture
                .source_editor
                .read(cx)
                .focus_handle(cx)
                .focus(window, cx)
        });
        cx.dispatch_action(editor::actions::OpenExcerpts);
        cx.run_until_parked();
        let reopened = fixture
            .workspace
            .read_with(cx, |workspace, cx| workspace.active_item_as::<Editor>(cx))
            .unwrap();
        assert_eq!(reopened, fixture.file_editor);
        reopened.read_with(cx, |editor, cx| {
            assert_eq!(
                editor.git_diff_base_name(cx).as_deref(),
                Some("review-base")
            );
            assert_eq!(
                editor_diff_base_text(editor, cx).as_deref(),
                Some(BRANCH_BASE)
            );
        });
    }

    #[gpui::test]
    async fn test_branch_diff_action_matches_existing_item_by_base_ref(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "a.txt": "changed",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let target_branch_diff = cx.update(|window, cx| {
            let repository = project.read(cx).active_repository(cx);
            cx.new(|cx| {
                BranchDiff::new_with_base_ref(
                    project.clone(),
                    workspace.clone(),
                    "topic".into(),
                    repository,
                    None,
                    window,
                    cx,
                )
            })
        });
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(
                Box::new(target_branch_diff.clone()),
                None,
                true,
                window,
                cx,
            );
        });
        cx.run_until_parked();

        cx.focus(&workspace);
        cx.update(|window, cx| {
            window.dispatch_action(DeployBranchDiff.boxed_clone(), cx);
        });
        cx.run_until_parked();

        let (active_base_ref, mut base_refs) = workspace.update(cx, |workspace, cx| {
            let active_item = workspace.active_item_as::<BranchDiff>(cx).unwrap();
            let active_base_ref = match active_item.read(cx).diff_base(cx) {
                DiffBase::Merge { base_ref } => base_ref.to_string(),
                DiffBase::Head | DiffBase::Index | DiffBase::Staged => {
                    panic!("expected active item to be a branch diff")
                }
            };
            let base_refs = workspace
                .items_of_type::<BranchDiff>(cx)
                .filter_map(|item| match item.read(cx).diff_base(cx) {
                    DiffBase::Merge { base_ref } => Some(base_ref.to_string()),
                    DiffBase::Head | DiffBase::Index | DiffBase::Staged => None,
                })
                .collect::<Vec<_>>();
            (active_base_ref, base_refs)
        });
        base_refs.sort();

        assert_eq!(active_base_ref, "origin/main");
        assert_eq!(base_refs, vec!["origin/main", "topic"]);
    }
}
