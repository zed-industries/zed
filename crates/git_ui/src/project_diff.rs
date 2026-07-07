use crate::{
    diff_multibuffer::DiffMultibuffer,
    git_panel::{GitPanel, GitPanelAddon, GitStatusEntry},
    staged_diff::StagedDiff,
    unstaged_diff::UnstagedDiff,
};
use anyhow::{Context as _, Result};
use buffer_diff::DiffHunkSecondaryStatus;
use editor::{
    Editor, EditorEvent, SplittableEditor, UncommittedDiffHunkDelegate,
    actions::{GoToHunk, GoToPreviousHunk, SendReviewToAgent},
};
use git::{Commit, StageAll, StageAndNext, ToggleStaged, UnstageAll, UnstageAndNext};
use gpui::{
    Action, AnyElement, App, AppContext as _, Entity, EventEmitter, FocusHandle, Focusable, Render,
    Subscription, Task, WeakEntity, actions,
};
use language::Capability;
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
use ui::{DiffStat, Divider, Tooltip, prelude::*};
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
    _diff_observation: Subscription,
}

impl ProjectDiff {
    pub(crate) fn register(workspace: &mut Workspace, cx: &mut Context<Workspace>) {
        workspace.register_action(Self::deploy);
        workspace.register_action(
            |workspace, _: &git_actions::ViewUncommittedChanges, window, cx| {
                Self::deploy_at(workspace, None, window, cx);
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
        Self::deploy_at(workspace, None, window, cx)
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

    fn new(
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let branch_diff = cx.new(|cx| {
            diff_buffer_list::DiffBufferList::new(DiffBase::Head, project.clone(), window, cx)
        });
        Self::new_impl(branch_diff, project, workspace, window, cx)
    }

    fn new_impl(
        branch_diff: Entity<diff_buffer_list::DiffBufferList>,
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let workspace_handle = workspace.downgrade();
        let diff = cx.new(|cx| {
            DiffMultibuffer::new(
                branch_diff,
                Capability::ReadWrite,
                "No uncommitted changes",
                move |editor, cx| {
                    editor.set_diff_hunk_delegate(Some(Arc::new(UncommittedDiffHunkDelegate)), cx);
                    editor.rhs_editor().update(cx, |rhs_editor, _cx| {
                        rhs_editor.set_read_only(false);
                        rhs_editor.register_addon(GitPanelAddon {
                            workspace: workspace_handle,
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
        let observation = cx.observe(&diff, |_, _, cx| cx.notify());
        Self {
            project,
            workspace: workspace.downgrade(),
            diff,
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
        let diff = self.diff.read(cx);
        let editor = diff.editor().read(cx).rhs_editor().clone();
        let editor = editor.read(cx);
        let snapshot = diff.multibuffer().read(cx).snapshot(cx);
        let prev_next = snapshot.diff_hunks().nth(1).is_some();
        let (selection, ranges) = diff.selected_ranges(cx);
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

    #[cfg(any(test, feature = "test-support"))]
    pub fn excerpt_paths(&self, cx: &App) -> Vec<std::sync::Arc<util::rel_path::RelPath>> {
        self.diff.read(cx).excerpt_paths(cx)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn excerpt_file_paths(&self, cx: &App) -> Vec<String> {
        self.diff.read(cx).excerpt_file_paths(cx)
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

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Uncommitted Changes".into()
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
        _workspace_id: workspace::WorkspaceId,
        _item_id: workspace::ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        window.spawn(cx, async move |cx| {
            cx.update(|window, cx| {
                let branch_diff = cx.new(|cx| {
                    diff_buffer_list::DiffBufferList::new(
                        DiffBase::Head,
                        project.clone(),
                        window,
                        cx,
                    )
                });
                let workspace = workspace.upgrade().context("workspace gone")?;
                anyhow::Ok(
                    cx.new(|cx| ProjectDiff::new_impl(branch_diff, project, workspace, window, cx)),
                )
            })?
        })
    }

    fn serialize(
        &mut self,
        _: &mut Workspace,
        _: workspace::ItemId,
        _: bool,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        Some(Task::ready(Ok(())))
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
    use project::git_store::diff_buffer_list::DiffBase;
    use workspace::{ItemId, WorkspaceDb, WorkspaceId};

    pub struct ProjectDiffDb(ThreadSafeConnection);

    impl Domain for ProjectDiffDb {
        const NAME: &str = stringify!(ProjectDiffDb);

        // Legacy databases stored branch diffs under the "ProjectDiff" item
        // kind, disambiguated by the `diff_base` column. Step 1 rewrites those
        // item kinds so that each diff view owns its serialized kind.
        const MIGRATIONS: &[&str] = &[
            sql!(
                CREATE TABLE project_diffs(
                    workspace_id INTEGER,
                    item_id INTEGER UNIQUE,

                    diff_base TEXT,

                    PRIMARY KEY(workspace_id, item_id),
                    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                ) STRICT;
            ),
            r#"
                UPDATE items SET kind = 'BranchDiff'
                WHERE kind = 'ProjectDiff' AND EXISTS (
                    SELECT 1 FROM project_diffs
                    WHERE project_diffs.item_id = items.item_id
                    AND project_diffs.workspace_id = items.workspace_id
                    AND project_diffs.diff_base LIKE '{"Merge"%'
                );
            "#,
        ];
    }

    db::static_connection!(ProjectDiffDb, [WorkspaceDb]);

    impl ProjectDiffDb {
        pub async fn save_project_diff_base(
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

        let (additions, deletions) = project_diff.read(cx).calculate_changed_lines(cx);
        let is_multibuffer_empty = project_diff.read(cx).multibuffer(cx).read(cx).is_empty();

        h_flex()
            .my_neg_1()
            .py_1()
            .gap_1p5()
            .flex_wrap()
            .justify_between()
            .when(!is_multibuffer_empty, |this| {
                this.child(DiffStat::new(
                    "project-diff-stat",
                    additions as usize,
                    deletions as usize,
                ))
                .child(Divider::vertical().ml_1())
            })
            // n.b. the only reason these arrows are here is because we don't
            // support "undo" for staging so we need a way to go back.
            .child(
                h_group_sm()
                    .child(
                        IconButton::new("up", IconName::ArrowUp)
                            .icon_size(IconSize::Small)
                            .disabled(!button_states.prev_next)
                            .tooltip(Tooltip::for_action_title_in(
                                "Go to Previous Hunk",
                                &GoToPreviousHunk,
                                &focus_handle,
                            ))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.dispatch_action(&GoToPreviousHunk, window, cx)
                            })),
                    )
                    .child(
                        IconButton::new("down", IconName::ArrowDown)
                            .icon_size(IconSize::Small)
                            .disabled(!button_states.prev_next)
                            .tooltip(Tooltip::for_action_title_in(
                                "Go to Next Hunk",
                                &GoToHunk,
                                &focus_handle,
                            ))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.dispatch_action(&GoToHunk, window, cx)
                            })),
                    ),
            )
            .child(Divider::vertical())
            .child(
                h_group_sm()
                    .when(button_states.selection, |this| {
                        this.child(
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
                    .when(!button_states.selection, |this| {
                        this.child(
                            Button::new("stage", "Stage")
                                .disabled(!button_states.stage)
                                .tooltip(Tooltip::for_action_title_in(
                                    "Stage and Go to Next Hunk",
                                    &StageAndNext,
                                    &focus_handle,
                                ))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.dispatch_action(&StageAndNext, window, cx)
                                })),
                        )
                        .child(
                            Button::new("unstage", "Unstage")
                                .disabled(!button_states.unstage)
                                .tooltip(Tooltip::for_action_title_in(
                                    "Unstage and Go to Next Hunk",
                                    &UnstageAndNext,
                                    &focus_handle,
                                ))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.dispatch_action(&UnstageAndNext, window, cx)
                                })),
                        )
                    }),
            )
            .child(Divider::vertical())
            .when(
                button_states.unstage_all && !button_states.stage_all,
                |this| {
                    this.child(
                        Button::new("unstage-all", "Unstage All")
                            .width(rems_from_px(80.))
                            .tooltip(Tooltip::for_action_title_in(
                                "Unstage All Changes",
                                &UnstageAll,
                                &focus_handle,
                            ))
                            .on_click(
                                cx.listener(|this, _, window, cx| this.unstage_all(window, cx)),
                            ),
                    )
                },
            )
            .when(
                !button_states.unstage_all || button_states.stage_all,
                |this| {
                    this.child(
                        Button::new("stage-all", "Stage All")
                            .width(rems_from_px(80.))
                            .disabled(!button_states.stage_all)
                            .tooltip(Tooltip::for_action_title_in(
                                "Stage All Changes",
                                &StageAll,
                                &focus_handle,
                            ))
                            .on_click(
                                cx.listener(|this, _, window, cx| this.stage_all(window, cx)),
                            ),
                    )
                },
            )
            .child(Divider::vertical())
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
            )
            .when(review_count > 0, |el| {
                el.child(Divider::vertical()).child(
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
    use editor::test::editor_test_context::{EditorTestContext, assert_state_with_diff};
    use gpui::TestAppContext;
    use multi_buffer::PathKey;
    use project::FakeFs;
    use serde_json::json;
    use settings::{DiffViewStyle, GitPanelGroupBy, GitPanelSortBy, SettingsStore};
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

    #[test]
    fn test_legacy_branch_diff_rows_migrate_to_their_own_kind() {
        use db::sqlez::{
            connection::Connection,
            domain::{Domain as _, Migrator as _},
        };

        let connection = Connection::open_memory(Some(
            "test_legacy_branch_diff_rows_migrate_to_their_own_kind",
        ));
        connection.exec("PRAGMA foreign_keys = OFF").unwrap()().unwrap();
        workspace::WorkspaceDb::migrate(&connection).unwrap();
        connection
            .migrate(
                persistence::ProjectDiffDb::NAME,
                &persistence::ProjectDiffDb::MIGRATIONS[..1],
                &mut |_, _, _| false,
            )
            .unwrap();

        connection
            .exec(
                "INSERT INTO workspaces(workspace_id) VALUES (1);
                INSERT INTO panes(pane_id, workspace_id, active) VALUES (1, 1, 1);
                INSERT INTO items(item_id, workspace_id, pane_id, kind, position, active) VALUES
                    (1, 1, 1, 'ProjectDiff', 0, 1),
                    (2, 1, 1, 'ProjectDiff', 1, 0)",
            )
            .unwrap()()
        .unwrap();
        let head = serde_json::to_string(&DiffBase::Head).unwrap();
        let merge = serde_json::to_string(&DiffBase::Merge {
            base_ref: "main".into(),
        })
        .unwrap();
        connection
            .exec_bound::<(String, String)>(
                "INSERT INTO project_diffs(workspace_id, item_id, diff_base) VALUES (1, 1, ?), (1, 2, ?)",
            )
            .unwrap()((head, merge))
        .unwrap();

        persistence::ProjectDiffDb::migrate(&connection).unwrap();

        let kinds = connection
            .select::<(i64, String)>("SELECT item_id, kind FROM items ORDER BY item_id")
            .unwrap()()
        .unwrap();
        assert_eq!(
            kinds,
            [
                (1, "ProjectDiff".to_string()),
                (2, "BranchDiff".to_string())
            ]
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
            ProjectDiff::new(project.clone(), workspace, window, cx)
        });
        cx.run_until_parked();

        let editor = diff.read_with(cx, |diff, cx| diff.editor(cx).read(cx).rhs_editor().clone());
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
            ProjectDiff::new(project.clone(), workspace, window, cx)
        });
        cx.run_until_parked();

        fs.set_head_and_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("bar", "bar\n".into()), ("foo", "foo\n".into())],
        );
        cx.run_until_parked();

        let editor = cx.update_window_entity(&diff, |diff, window, cx| {
            diff.diff.update(cx, |diff, cx| {
                diff.move_to_path(
                    PathKey::with_sort_prefix(2, rel_path("foo").into_arc()),
                    window,
                    cx,
                )
            });
            diff.editor(cx).read(cx).rhs_editor().clone()
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
            diff.diff.update(cx, |diff, cx| {
                diff.move_to_path(
                    PathKey::with_sort_prefix(2, rel_path("bar").into_arc()),
                    window,
                    cx,
                )
            });
            diff.editor(cx).read(cx).rhs_editor().clone()
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
            ProjectDiff::new(project.clone(), workspace, window, cx)
        });
        cx.run_until_parked();

        let diff_editor =
            diff.read_with(cx, |diff, cx| diff.editor(cx).read(cx).rhs_editor().clone());

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

        let editor = diff.read_with(cx, |diff, cx| diff.editor(cx).read(cx).rhs_editor().clone());

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
            ProjectDiff::new(project.clone(), workspace, window, cx)
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
            ProjectDiff::new(project.clone(), workspace, window, cx)
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
            ProjectDiff::new(project.clone(), workspace, window, cx)
        });
        cx.run_until_parked();
        let editor = diff.read_with(cx, |diff, cx| diff.editor(cx).read(cx).rhs_editor().clone());

        let mut cx = EditorTestContext::for_editor_in(editor, cx).await;

        cx.assert_excerpts_with_selections(&format!("[EXCERPT]\nˇ{git_contents}"));

        cx.dispatch_action(editor::actions::GoToHunk);
        cx.dispatch_action(editor::actions::GoToHunk);
        cx.dispatch_action(git::Restore);
        cx.dispatch_action(editor::actions::MoveToBeginning);

        cx.assert_excerpts_with_selections(&format!("[EXCERPT]\nˇ{git_contents}"));
    }
}
