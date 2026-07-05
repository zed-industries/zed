use crate::{
    branch_diff::BranchDiff,
    diff_multibuffer::{ButtonStates, DiffMultibuffer},
    git_panel::{GitPanel, GitStatusEntry},
    staged_diff::StagedDiff,
    unstaged_diff::UnstagedDiff,
};
use anyhow::{Context as _, Result};
use editor::{
    Editor, EditorEvent, SplittableEditor,
    actions::{GoToHunk, GoToPreviousHunk, SendReviewToAgent},
};
use git::{Commit, StageAll, StageAndNext, ToggleStaged, UnstageAll, UnstageAndNext};
use gpui::{
    Action, AnyElement, App, AppContext as _, Entity, EventEmitter, FocusHandle, Focusable, Render,
    Subscription, Task, WeakEntity, actions,
};
use multi_buffer::MultiBuffer;
use project::{
    Project, ProjectPath,
    git_store::{
        Repository,
        diff_buffer_list::{self, DiffBase},
    },
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::any::{Any, TypeId};
use std::sync::Arc;
use ui::{Tooltip, prelude::*, vertical_divider};
use workspace::{
    ItemNavHistory, SerializableItem, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView,
    Workspace,
    item::{Item, ItemEvent, ItemHandle, SaveOptions, TabContentParams},
    searchable::SearchableItemHandle,
};
use zed_actions::git as git_actions;

actions!(
    git,
    [
        /// Shows the diff between the working directory and the index.
        Diff,
        /// Adds files to the git staging area.
        Add,
        /// Opens a new agent thread with the branch diff for review.
        ReviewDiff,
        LeaderAndFollower,
        /// Compare with a specific branch
        CompareWithBranch,
    ]
);

/// Shows the diff between the working directory and your default
/// branch (typically main or master).
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = git, name = "BranchDiff")]
pub(crate) struct DeployBranchDiff;

pub struct ProjectDiff {
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    diff: Entity<DiffMultibuffer>,

    pending_item_swap: Option<PendingItemSwap>,
    _diff_observation: Subscription,
}

enum PendingItemSwap {
    UnstagedDiff,
    StagedDiff,
    BranchDiff { base_ref: SharedString },
}

impl ProjectDiff {
    pub(crate) fn register(workspace: &mut Workspace, cx: &mut Context<Workspace>) {
        workspace.register_action(Self::deploy);
        workspace.register_action(
            |workspace, _: &git_actions::ViewUncommittedChanges, window, cx| {
                Self::deploy_head_diff_at(workspace, None, window, cx);
            },
        );
        workspace.register_action(
            |workspace, _: &git_actions::ViewUnstagedChanges, window, cx| {
                UnstagedDiff::deploy_at(workspace, None, window, cx);
            },
        );
        workspace.register_action(
            |workspace, _: &git_actions::ViewStagedChanges, window, cx| {
                StagedDiff::deploy_at(workspace, None, window, cx);
            },
        );
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
        Self::deploy_head_diff_at(workspace, None, window, cx)
    }

    pub fn deploy_at(
        workspace: &mut Workspace,
        entry: Option<GitStatusEntry>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        Self::deploy_head_diff_at(workspace, entry, window, cx)
    }

    fn deploy_head_diff_at(
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
        let intended_repo = workspace.project().read(cx).active_repository(cx);

        let existing = workspace.items_of_type::<Self>(cx).next();
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

        if let Some(intended) = &intended_repo {
            let needs_switch = project_diff
                .read(cx)
                .repo(cx)
                .map_or(true, |current| current.read(cx).id != intended.read(cx).id);
            if needs_switch {
                project_diff.update(cx, |project_diff, cx| {
                    project_diff.set_repo(Some(intended.clone()), cx);
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
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        telemetry::event!("Git Diff Opened", source = "Agent Panel");
        let existing = workspace.items_of_type::<Self>(cx).next();
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
        self.diff.update(cx, |diff, cx| diff.autoscroll(cx));
    }

    pub(crate) fn new(
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let diff = cx.new(|cx| {
            DiffMultibuffer::new_with_diff_base(
                DiffBase::Head,
                project.clone(),
                workspace.clone(),
                window,
                cx,
            )
        });
        Self::from_diff(diff, project, workspace, cx)
    }

    fn new_impl(
        branch_diff: Entity<diff_buffer_list::DiffBufferList>,
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let diff = cx.new(|cx| {
            DiffMultibuffer::new_impl(branch_diff, project.clone(), workspace.clone(), window, cx)
        });
        Self::from_diff(diff, project, workspace, cx)
    }

    fn from_diff(
        diff: Entity<DiffMultibuffer>,
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let observation = cx.observe(&diff, |_, _, cx| cx.notify());
        Self {
            project,
            workspace: workspace.downgrade(),
            diff,
            pending_item_swap: None,
            _diff_observation: observation,
        }
    }

    pub fn diff_base<'a>(&'a self, cx: &'a App) -> &'a DiffBase {
        self.diff.read(cx).diff_base(cx)
    }

    pub(crate) fn repo(&self, cx: &App) -> Option<Entity<Repository>> {
        self.diff.read(cx).repo(cx)
    }

    pub(crate) fn set_repo(&mut self, repo: Option<Entity<Repository>>, cx: &mut Context<Self>) {
        self.diff
            .update(cx, |diff, cx| diff.set_repo(repo.clone(), cx));
    }

    pub fn move_to_entry(
        &mut self,
        entry: GitStatusEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.diff
            .update(cx, |diff, cx| diff.move_to_entry(entry, window, cx));
    }

    pub fn move_to_project_path(
        &mut self,
        project_path: &ProjectPath,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.diff.update(cx, |diff, cx| {
            diff.move_to_project_path(project_path, window, cx)
        });
    }

    fn move_to_beginning(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.diff
            .update(cx, |diff, cx| diff.move_to_beginning(window, cx));
    }

    pub fn calculate_changed_lines(&self, cx: &App) -> (u32, u32) {
        self.diff.read(cx).calculate_changed_lines(cx)
    }

    /// Returns the total count of review comments across all hunks/files.
    pub fn total_review_comment_count(&self, cx: &App) -> usize {
        self.diff.read(cx).total_review_comment_count()
    }

    /// Returns the splittable editor of the currently-shown diff view.
    pub fn editor(&self, cx: &App) -> Entity<SplittableEditor> {
        self.diff.read(cx).editor().clone()
    }

    /// Returns the multibuffer of the currently-shown diff view.
    pub fn multibuffer(&self, cx: &App) -> Entity<MultiBuffer> {
        self.diff.read(cx).multibuffer().clone()
    }

    fn button_states(&self, cx: &App) -> ButtonStates {
        self.diff.read(cx).button_states(cx)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn excerpt_paths(&self, cx: &App) -> Vec<std::sync::Arc<util::rel_path::RelPath>> {
        self.diff.read(cx).excerpt_paths(cx)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn excerpt_file_paths(&self, cx: &App) -> Vec<String> {
        self.diff.read(cx).excerpt_file_paths(cx)
    }
}

impl EventEmitter<EditorEvent> for ProjectDiff {}

impl Focusable for ProjectDiff {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.diff.read(cx).focus_handle(cx)
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
        self.diff.read(cx).tab_content_text(cx)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Project Diff Opened")
    }

    fn as_searchable(&self, _: &Entity<Self>, cx: &App) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.diff.read(cx).editor().clone()))
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.diff.read(cx).for_each_project_item(cx, f)
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
        Task::ready(Some(cx.new(|cx| {
            ProjectDiff::new(self.project.clone(), workspace, window, cx)
        })))
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.diff.read(cx).is_dirty(cx)
    }

    fn has_conflict(&self, cx: &App) -> bool {
        self.diff.read(cx).has_conflict(cx)
    }

    fn can_save(&self, cx: &App) -> bool {
        self.diff.read(cx).can_save(cx)
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

        let Some(swap) = self.pending_item_swap.take() else {
            return;
        };
        let project_diff = cx.entity();
        let project = self.project.clone();
        let workspace = self.workspace.clone();
        let repo = self.diff.read(cx).repo(cx);
        window
            .spawn(cx, async move |cx| {
                cx.update(|window, cx| {
                    let Some(workspace_handle) = workspace.upgrade() else {
                        return anyhow::Ok(());
                    };
                    workspace_handle.update(cx, |workspace, cx| {
                        let Some(pane) = workspace.pane_for(&project_diff) else {
                            return;
                        };
                        let Some(index) = pane.read(cx).index_for_item(&project_diff) else {
                            return;
                        };
                        match &swap {
                            PendingItemSwap::UnstagedDiff => {
                                let item = cx.new(|cx| {
                                    UnstagedDiff::new(
                                        project.clone(),
                                        workspace_handle.clone(),
                                        window,
                                        cx,
                                    )
                                });
                                workspace.add_item(
                                    pane.clone(),
                                    Box::new(item),
                                    Some(index),
                                    true,
                                    true,
                                    window,
                                    cx,
                                );
                            }
                            PendingItemSwap::StagedDiff => {
                                let item = cx.new(|cx| {
                                    StagedDiff::new(
                                        project.clone(),
                                        workspace_handle.clone(),
                                        window,
                                        cx,
                                    )
                                });
                                workspace.add_item(
                                    pane.clone(),
                                    Box::new(item),
                                    Some(index),
                                    true,
                                    true,
                                    window,
                                    cx,
                                );
                            }
                            PendingItemSwap::BranchDiff { base_ref } => {
                                let item = cx.new(|cx| {
                                    BranchDiff::new_with_base_ref(
                                        project.clone(),
                                        workspace_handle.clone(),
                                        base_ref.clone(),
                                        repo.clone(),
                                        window,
                                        cx,
                                    )
                                });
                                workspace.add_item(
                                    pane.clone(),
                                    Box::new(item),
                                    Some(index),
                                    true,
                                    true,
                                    window,
                                    cx,
                                );
                            }
                        }
                        pane.update(cx, |pane, cx| {
                            pane.remove_item(project_diff.item_id(), false, false, window, cx);
                        });
                    });
                    anyhow::Ok(())
                })??;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
    }
}

impl Render for ProjectDiff {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.diff.clone())
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
            let diff_base = db.get_project_diff_base(item_id, workspace_id)?;

            let diff = cx.update(|window, cx| {
                let pending_item_swap = match &diff_base {
                    DiffBase::Head => None,
                    DiffBase::Index => Some(PendingItemSwap::UnstagedDiff),
                    DiffBase::Staged => Some(PendingItemSwap::StagedDiff),
                    DiffBase::Merge { base_ref } => Some(PendingItemSwap::BranchDiff {
                        base_ref: base_ref.clone(),
                    }),
                };
                let branch_diff = cx.new(|cx| {
                    diff_buffer_list::DiffBufferList::new(diff_base, project.clone(), window, cx)
                });
                let workspace = workspace.upgrade().context("workspace gone")?;
                let project_diff =
                    cx.new(|cx| ProjectDiff::new_impl(branch_diff, project, workspace, window, cx));
                if pending_item_swap.is_some() {
                    project_diff.update(cx, |project_diff, _| {
                        project_diff.pending_item_swap = pending_item_swap;
                    });
                }
                anyhow::Ok(project_diff)
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
        let diff_base =
            match persistence::PersistedProjectDiffBase::from_diff_base(self.diff_base(cx)) {
                Ok(diff_base) => diff_base,
                Err(error) => {
                    log::error!("failed to serialize project diff base: {error:#}");
                    return None;
                }
            };

        let db = persistence::ProjectDiffDb::global(cx);
        Some(cx.background_spawn({
            async move {
                db.save_project_diff_base(item_id, workspace_id, diff_base.clone())
                    .await
            }
        }))
    }

    fn should_serialize(&self, _: &Self::Event) -> bool {
        false
    }
}

pub(crate) mod persistence {

    use anyhow::Context as _;
    use db::{
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };
    use gpui::SharedString;
    use project::git_store::diff_buffer_list::DiffBase;
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

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub enum PersistedProjectDiffBase {
        Head,
        Index,
        Staged,
        Merge { base_ref: SharedString },
    }

    impl PersistedProjectDiffBase {
        pub fn into_diff_base(self) -> DiffBase {
            match self {
                Self::Head => DiffBase::Head,
                Self::Index => DiffBase::Index,
                Self::Staged => DiffBase::Staged,
                Self::Merge { base_ref } => DiffBase::Merge { base_ref },
            }
        }

        pub fn from_diff_base(diff_base: &DiffBase) -> anyhow::Result<Self> {
            match diff_base {
                DiffBase::Head => Ok(Self::Head),
                DiffBase::Index => Ok(Self::Index),
                DiffBase::Staged => Ok(Self::Staged),
                DiffBase::Merge { base_ref } => Ok(Self::Merge {
                    base_ref: base_ref.clone(),
                }),
            }
        }
    }

    impl ProjectDiffDb {
        pub async fn save_project_diff_base(
            &self,
            item_id: ItemId,
            workspace_id: WorkspaceId,
            diff_base: PersistedProjectDiffBase,
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

        pub fn get_project_diff_base(
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
            if let Ok(diff_base) = serde_json::from_str::<PersistedProjectDiffBase>(&diff_base_str)
            {
                return Ok(diff_base.into_diff_base());
            }

            let diff_base: DiffBase =
                serde_json::from_str(&diff_base_str).context("deserializing diff base")?;
            PersistedProjectDiffBase::from_diff_base(&diff_base)
                .map(|diff_base| diff_base.into_diff_base())
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

impl Render for ProjectDiffToolbar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(project_diff) = self.project_diff(cx) else {
            return div();
        };
        let focus_handle = project_diff.focus_handle(cx);
        let button_states = project_diff.read(cx).button_states(cx);
        let review_count = project_diff.read(cx).total_review_comment_count(cx);

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

pub(crate) fn render_send_review_to_agent_button(
    review_count: usize,
    focus_handle: &FocusHandle,
) -> Button {
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

#[cfg(test)]
mod tests {
    use buffer_diff::DiffHunkSecondaryStatus;
    use db::indoc;
    use editor::test::editor_test_context::EditorTestContext;
    use gpui::TestAppContext;
    use project::FakeFs;
    use serde_json::json;
    use settings::{DiffViewStyle, SettingsStore};
    use std::path::Path;
    use unindent::Unindent as _;
    use util::{path, rel_path::rel_path};

    use workspace::MultiWorkspace;

    use super::*;

    #[ctor::ctor(unsafe)]
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

    use zed_actions::git as git_actions;

    use crate::project_diff::{self, ProjectDiff};

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
        let editor = item.read_with(cx, |item, cx| item.editor(cx).read(cx).rhs_editor().clone());

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
    async fn test_project_diff_actions_filter_mixed_staged_and_unstaged_hunks(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let committed_contents = r#"
            fn main() {
                println!("hello world");
            }
        "#
        .unindent();
        let staged_contents = r#"
            fn main() {
                println!("goodbye world");
            }
        "#
        .unindent();
        let file_contents = r#"
            // print goodbye
            fn main() {
                println!("goodbye world");
            }
        "#
        .unindent();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "src": {
                    "main.rs": file_contents,
                }
            }),
        )
        .await;

        fs.set_head_for_repo(
            Path::new(path!("/project/.git")),
            &[("src/main.rs", committed_contents)],
            "deadbeef",
        );
        fs.set_index_for_repo(
            Path::new(path!("/project/.git")),
            &[("src/main.rs", staged_contents)],
        );

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        cx.run_until_parked();

        cx.focus(&workspace);
        cx.update(|window, cx| {
            window.dispatch_action(project_diff::Diff.boxed_clone(), cx);
        });
        cx.run_until_parked();

        let diff_item = workspace.update(cx, |workspace, cx| {
            workspace.active_item_as::<ProjectDiff>(cx).unwrap()
        });
        let diff_editor =
            diff_item.read_with(cx, |diff, cx| diff.editor(cx).read(cx).rhs_editor().clone());
        assert_eq!(
            diff_editor.read_with(cx, |editor, cx| {
                let snapshot = editor.buffer().read(cx).snapshot(cx);
                editor
                    .diff_hunks_in_ranges(&[editor::Anchor::Min..editor::Anchor::Max], &snapshot)
                    .map(|hunk| hunk.status.secondary)
                    .collect::<Vec<_>>()
            }),
            vec![
                DiffHunkSecondaryStatus::HasSecondaryHunk,
                DiffHunkSecondaryStatus::NoSecondaryHunk,
            ]
        );

        cx.focus(&workspace);
        cx.update(|window, cx| {
            window.dispatch_action(git_actions::ViewUnstagedChanges.boxed_clone(), cx);
        });
        cx.run_until_parked();

        let unstaged_item = workspace.update(cx, |workspace, cx| {
            workspace.active_item_as::<UnstagedDiff>(cx).unwrap()
        });
        assert_ne!(diff_item.entity_id(), unstaged_item.entity_id());
        let unstaged_editor = workspace.update(cx, |workspace, cx| {
            let active_item = workspace.active_item(cx).unwrap();
            assert_eq!(active_item.tab_content_text(0, cx), "Unstaged Changes");
            active_item
                .act_as::<DiffMultibuffer>(cx)
                .unwrap()
                .read(cx)
                .editor()
                .read(cx)
                .rhs_editor()
                .clone()
        });
        assert_eq!(
            unstaged_editor.read_with(cx, |editor, cx| {
                let snapshot = editor.buffer().read(cx).snapshot(cx);
                editor
                    .diff_hunks_in_ranges(&[editor::Anchor::Min..editor::Anchor::Max], &snapshot)
                    .map(|hunk| hunk.status.secondary)
                    .collect::<Vec<_>>()
            }),
            vec![DiffHunkSecondaryStatus::NoSecondaryHunk]
        );

        cx.focus(&workspace);
        cx.update(|window, cx| {
            window.dispatch_action(git_actions::ViewUncommittedChanges.boxed_clone(), cx);
        });
        cx.run_until_parked();

        let uncommitted_item = workspace.update(cx, |workspace, cx| {
            workspace.active_item_as::<ProjectDiff>(cx).unwrap()
        });
        assert_eq!(diff_item.entity_id(), uncommitted_item.entity_id());
        assert_eq!(
            uncommitted_item.read_with(cx, |diff, cx| diff.tab_content_text(0, cx)),
            "Uncommitted Changes"
        );
        let uncommitted_editor = uncommitted_item
            .read_with(cx, |diff, cx| diff.editor(cx).read(cx).rhs_editor().clone());
        assert_eq!(
            uncommitted_editor.read_with(cx, |editor, cx| {
                let snapshot = editor.buffer().read(cx).snapshot(cx);
                editor
                    .diff_hunks_in_ranges(&[editor::Anchor::Min..editor::Anchor::Max], &snapshot)
                    .map(|hunk| hunk.status.secondary)
                    .collect::<Vec<_>>()
            }),
            vec![
                DiffHunkSecondaryStatus::HasSecondaryHunk,
                DiffHunkSecondaryStatus::NoSecondaryHunk,
            ]
        );

        cx.focus(&workspace);
        cx.update(|window, cx| {
            window.dispatch_action(git_actions::ViewStagedChanges.boxed_clone(), cx);
        });
        cx.run_until_parked();

        let staged_editor = workspace.update(cx, |workspace, cx| {
            workspace.active_item_as::<StagedDiff>(cx).unwrap();
            let active_item = workspace.active_item(cx).unwrap();
            assert_eq!(active_item.tab_content_text(0, cx), "Staged Changes");
            active_item
                .act_as::<DiffMultibuffer>(cx)
                .unwrap()
                .read(cx)
                .editor()
                .read(cx)
                .rhs_editor()
                .clone()
        });
        assert_eq!(
            staged_editor.read_with(cx, |editor, cx| {
                let snapshot = editor.buffer().read(cx).snapshot(cx);
                editor
                    .diff_hunks_in_ranges(&[editor::Anchor::Min..editor::Anchor::Max], &snapshot)
                    .map(|hunk| hunk.status.secondary)
                    .collect::<Vec<_>>()
            }),
            vec![DiffHunkSecondaryStatus::NoSecondaryHunk]
        );
    }

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
        let editor = item.read_with(cx, |item, cx| item.editor(cx).read(cx).rhs_editor().clone());

        let mut cx = EditorTestContext::for_editor_in(editor, cx).await;

        cx.set_selections_state(indoc!(
            "
            before
            really changed

            deleted

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
}
