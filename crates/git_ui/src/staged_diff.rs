use crate::{git_panel::GitStatusEntry, project_diff::ProjectDiff};
use anyhow::{Context as _, Result};
use editor::{Editor, EditorEvent};
use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, FocusHandle, Focusable, Render, SharedString,
    Subscription, Task, WeakEntity,
};
use project::{
    Project, ProjectPath,
    git_store::branch_diff::{BranchDiff, DiffBase},
};
use std::{
    any::{Any, TypeId},
    sync::Arc,
};
use ui::{Icon, Window, prelude::*};
use workspace::{
    ItemNavHistory, SerializableItem, Workspace,
    item::{Item, ItemEvent, SaveOptions, TabContentParams},
    searchable::SearchableItemHandle,
};

pub struct StagedDiff {
    project_diff: Entity<ProjectDiff>,
    add_inner_to_workspace: bool,
    _project_diff_event_subscription: Subscription,
}

impl StagedDiff {
    pub(crate) fn register(workspace: &mut Workspace, cx: &mut Context<Workspace>) {
        let _ = workspace;
        workspace::register_serializable_item::<Self>(cx);
    }

    pub fn deploy_at(
        workspace: &mut Workspace,
        entry: Option<GitStatusEntry>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        telemetry::event!(
            "Git Staged Diff Opened",
            source = if entry.is_some() {
                "Git Panel"
            } else {
                "Action"
            }
        );
        let intended_repo = workspace.project().read(cx).active_repository(cx);
        let existing = workspace.items_of_type::<Self>(cx).next();
        let staged_diff = if let Some(existing) = existing {
            workspace.activate_item(&existing, true, true, window, cx);
            existing
        } else {
            let workspace_handle = cx.entity();
            let staged_diff =
                cx.new(|cx| Self::new(workspace.project().clone(), workspace_handle, window, cx));
            workspace.add_item_to_active_pane(
                Box::new(staged_diff.clone()),
                None,
                true,
                window,
                cx,
            );
            staged_diff
        };

        if let Some(intended) = &intended_repo {
            let needs_switch = staged_diff
                .read(cx)
                .project_diff
                .read(cx)
                .repo(cx)
                .map_or(true, |current| current.entity_id() != intended.entity_id());
            if needs_switch {
                staged_diff.update(cx, |staged_diff, cx| {
                    staged_diff.project_diff.update(cx, |project_diff, cx| {
                        project_diff.set_repo(Some(intended.clone()), cx);
                    });
                });
            }
        }

        if let Some(entry) = entry {
            staged_diff.update(cx, |staged_diff, cx| {
                staged_diff.project_diff.update(cx, |project_diff, cx| {
                    project_diff.move_to_entry(entry, window, cx)
                });
            });
        }
    }

    pub(crate) fn new(
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let project_diff = cx.new(|cx| {
            ProjectDiff::new_with_diff_base(DiffBase::Staged, project, workspace, window, cx)
        });
        Self::from_project_diff(project_diff, true, cx)
    }

    pub(crate) fn from_project_diff(
        project_diff: Entity<ProjectDiff>,
        add_inner_to_workspace: bool,
        cx: &mut Context<Self>,
    ) -> Self {
        let project_diff_event_subscription = cx
            .subscribe(&project_diff, |_, _, event: &EditorEvent, cx| {
                cx.emit(event.clone())
            });

        Self {
            project_diff,
            add_inner_to_workspace,
            _project_diff_event_subscription: project_diff_event_subscription,
        }
    }
}

impl EventEmitter<EditorEvent> for StagedDiff {}

impl Focusable for StagedDiff {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.project_diff.read(cx).focus_handle(cx)
    }
}

impl Item for StagedDiff {
    type Event = EditorEvent;

    fn tab_icon(&self, window: &Window, cx: &App) -> Option<Icon> {
        self.project_diff.read(cx).tab_icon(window, cx)
    }

    fn to_item_events(event: &EditorEvent, f: &mut dyn FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.project_diff
            .update(cx, |project_diff, cx| project_diff.deactivated(window, cx));
    }

    fn navigate(
        &mut self,
        data: Arc<dyn Any + Send>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.project_diff.update(cx, |project_diff, cx| {
            project_diff.navigate(data, window, cx)
        })
    }

    fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> {
        Some("Staged Changes".into())
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, _cx: &App) -> AnyElement {
        Label::new(self.tab_content_text(0, _cx))
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Staged Changes".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Git Staged Diff Opened")
    }

    fn as_searchable(&self, _: &Entity<Self>, cx: &App) -> Option<Box<dyn SearchableItemHandle>> {
        self.project_diff
            .read(cx)
            .as_searchable(&self.project_diff, cx)
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.project_diff.read(cx).for_each_project_item(cx, f);
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.project_diff.update(cx, |project_diff, cx| {
            project_diff.set_nav_history(nav_history, window, cx);
        });
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        workspace_id: Option<workspace::WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        let project_diff_task = self.project_diff.update(cx, |project_diff, cx| {
            project_diff.clone_on_split(workspace_id, window, cx)
        });

        cx.spawn(async move |_, cx| {
            let Some(project_diff) = project_diff_task.await else {
                return None;
            };
            Some(cx.new(|cx| Self::from_project_diff(project_diff, true, cx)))
        })
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.project_diff.read(cx).is_dirty(cx)
    }

    fn has_conflict(&self, cx: &App) -> bool {
        self.project_diff.read(cx).has_conflict(cx)
    }

    fn can_save(&self, _: &App) -> bool {
        false
    }

    fn save(
        &mut self,
        _: SaveOptions,
        _: Entity<Project>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Task<Result<()>> {
        Task::ready(Ok(()))
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
        self.project_diff.update(cx, |project_diff, cx| {
            project_diff.reload(project, window, cx)
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
        } else if type_id == TypeId::of::<ProjectDiff>()
            || type_id == TypeId::of::<Editor>()
            || type_id == TypeId::of::<editor::SplittableEditor>()
            || type_id == TypeId::of::<BranchDiff>()
        {
            self.project_diff
                .read(cx)
                .act_as_type(type_id, &self.project_diff, cx)
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
        if self.add_inner_to_workspace {
            self.project_diff.update(cx, |project_diff, cx| {
                project_diff.added_to_workspace(workspace, window, cx)
            });
        }
    }
}

impl SerializableItem for StagedDiff {
    fn serialized_item_kind() -> &'static str {
        "StagedDiff"
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
        _: workspace::WorkspaceId,
        _: workspace::ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        window.spawn(cx, async move |cx| {
            let workspace = workspace.upgrade().context("workspace gone")?;
            cx.update(|window, cx| Ok(cx.new(|cx| Self::new(project, workspace, window, cx))))?
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

impl Render for StagedDiff {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.project_diff.clone()
    }
}
