use std::any::{Any, TypeId};

use anyhow::Result;
use collections::HashSet;
use editor::{scroll::Autoscroll, Editor, EditorEvent};
use feature_flags::FeatureFlagViewExt;
use futures::StreamExt;
use gpui::{
    actions, AnyElement, AnyView, App, AppContext, AsyncWindowContext, Entity, EventEmitter,
    FocusHandle, Focusable, Render, Subscription, Task, WeakEntity,
};
use language::{Anchor, Buffer, Capability, OffsetRangeExt};
use multi_buffer::{MultiBuffer, PathKey};
use project::{buffer_store::BufferChangeSet, git::GitState, Project, ProjectPath};
use theme::ActiveTheme;
use ui::prelude::*;
use util::ResultExt as _;
use workspace::{
    item::{BreadcrumbText, Item, ItemEvent, ItemHandle, TabContentParams},
    searchable::SearchableItemHandle,
    ItemNavHistory, ToolbarItemLocation, Workspace,
};

use crate::git_panel::{GitPanel, GitStatusEntry};

actions!(git, [Diff]);

pub(crate) struct ProjectDiff {
    multibuffer: Entity<MultiBuffer>,
    editor: Entity<Editor>,
    project: Entity<Project>,
    git_state: Entity<GitState>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    update_needed: postage::watch::Sender<()>,
    pending_scroll: Option<PathKey>,

    _task: Task<Result<()>>,
    _subscription: Subscription,
}

struct DiffBuffer {
    path_key: PathKey,
    buffer: Entity<Buffer>,
    change_set: Entity<BufferChangeSet>,
}

const CHANGED_NAMESPACE: &'static str = "0";
const ADDED_NAMESPACE: &'static str = "1";

impl ProjectDiff {
    pub(crate) fn register(
        _: &mut Workspace,
        window: Option<&mut Window>,
        cx: &mut Context<Workspace>,
    ) {
        let Some(window) = window else { return };
        cx.when_flag_enabled::<feature_flags::GitUiFeatureFlag>(window, |workspace, _, _cx| {
            workspace.register_action(Self::deploy);
        });
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
        let project_diff = if let Some(existing) = workspace.item_of_type::<Self>(cx) {
            workspace.activate_item(&existing, true, true, window, cx);
            existing
        } else {
            let workspace_handle = cx.entity().downgrade();
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
                project_diff.scroll_to(entry, window, cx);
            })
        }
    }

    fn new(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));

        let editor = cx.new(|cx| {
            let mut diff_display_editor = Editor::for_multibuffer(
                multibuffer.clone(),
                Some(project.clone()),
                true,
                window,
                cx,
            );
            diff_display_editor.set_expand_all_diff_hunks(cx);
            diff_display_editor
        });
        cx.subscribe_in(&editor, window, Self::handle_editor_event)
            .detach();

        let git_state = project.read(cx).git_state().clone();
        let git_state_subscription = cx.subscribe_in(
            &git_state,
            window,
            move |this, _git_state, _event, _window, _cx| {
                *this.update_needed.borrow_mut() = ();
            },
        );

        let (mut send, recv) = postage::watch::channel::<()>();
        let worker = window.spawn(cx, {
            let this = cx.weak_entity();
            |cx| Self::handle_status_updates(this, recv, cx)
        });
        // Kick of a refresh immediately
        *send.borrow_mut() = ();

        Self {
            project,
            git_state: git_state.clone(),
            workspace,
            focus_handle,
            editor,
            multibuffer,
            pending_scroll: None,
            update_needed: send,
            _task: worker,
            _subscription: git_state_subscription,
        }
    }

    pub fn scroll_to(
        &mut self,
        entry: GitStatusEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(git_repo) = self.git_state.read(cx).active_repository() else {
            return;
        };

        let Some(path) = git_repo
            .read(cx)
            .repo_path_to_project_path(&entry.repo_path)
            .and_then(|project_path| self.project.read(cx).absolute_path(&project_path, cx))
        else {
            return;
        };
        let path_key = if entry.status.is_created() {
            PathKey::namespaced(ADDED_NAMESPACE, &path)
        } else {
            PathKey::namespaced(CHANGED_NAMESPACE, &path)
        };
        self.scroll_to_path(path_key, window, cx)
    }

    fn scroll_to_path(&mut self, path_key: PathKey, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(position) = self.multibuffer.read(cx).location_for_path(&path_key, cx) {
            self.editor.update(cx, |editor, cx| {
                editor.change_selections(Some(Autoscroll::focused()), window, cx, |s| {
                    s.select_ranges([position..position]);
                })
            })
        } else {
            self.pending_scroll = Some(path_key);
        }
    }

    fn handle_editor_event(
        &mut self,
        editor: &Entity<Editor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            EditorEvent::ScrollPositionChanged { .. } => editor.update(cx, |editor, cx| {
                let anchor = editor.scroll_manager.anchor().anchor;
                let Some((_, buffer, _)) = self.multibuffer.read(cx).excerpt_containing(anchor, cx)
                else {
                    return;
                };
                let Some(project_path) = buffer
                    .read(cx)
                    .file()
                    .map(|file| (file.worktree_id(cx), file.path().clone()))
                else {
                    return;
                };
                self.workspace
                    .update(cx, |workspace, cx| {
                        if let Some(git_panel) = workspace.panel::<GitPanel>(cx) {
                            git_panel.update(cx, |git_panel, cx| {
                                git_panel.set_focused_path(project_path.into(), window, cx)
                            })
                        }
                    })
                    .ok();
            }),
            _ => {}
        }
    }

    fn load_buffers(&mut self, cx: &mut Context<Self>) -> Vec<Task<Result<DiffBuffer>>> {
        let Some(repo) = self.git_state.read(cx).active_repository() else {
            self.multibuffer.update(cx, |multibuffer, cx| {
                multibuffer.clear(cx);
            });
            return vec![];
        };

        let mut previous_paths = self.multibuffer.read(cx).paths().collect::<HashSet<_>>();

        let mut result = vec![];
        repo.update(cx, |repo, cx| {
            for entry in repo.status() {
                if !entry.status.has_changes() {
                    continue;
                }
                let Some(project_path) = repo.repo_path_to_project_path(&entry.repo_path) else {
                    continue;
                };
                let Some(abs_path) = self.project.read(cx).absolute_path(&project_path, cx) else {
                    continue;
                };
                // Craft some artificial paths so that created entries will appear last.
                let path_key = if entry.status.is_created() {
                    PathKey::namespaced(ADDED_NAMESPACE, &abs_path)
                } else {
                    PathKey::namespaced(CHANGED_NAMESPACE, &abs_path)
                };

                previous_paths.remove(&path_key);
                let load_buffer = self
                    .project
                    .update(cx, |project, cx| project.open_buffer(project_path, cx));

                let project = self.project.clone();
                result.push(cx.spawn(|_, mut cx| async move {
                    let buffer = load_buffer.await?;
                    let changes = project
                        .update(&mut cx, |project, cx| {
                            project.open_uncommitted_changes(buffer.clone(), cx)
                        })?
                        .await?;
                    Ok(DiffBuffer {
                        path_key,
                        buffer,
                        change_set: changes,
                    })
                }));
            }
        });
        self.multibuffer.update(cx, |multibuffer, cx| {
            for path in previous_paths {
                multibuffer.remove_excerpts_for_path(path, cx);
            }
        });
        result
    }

    fn register_buffer(
        &mut self,
        diff_buffer: DiffBuffer,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let path_key = diff_buffer.path_key;
        let buffer = diff_buffer.buffer;
        let change_set = diff_buffer.change_set;

        let snapshot = buffer.read(cx).snapshot();
        let diff_hunk_ranges = change_set
            .read(cx)
            .diff_hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &snapshot)
            .map(|diff_hunk| diff_hunk.buffer_range.to_point(&snapshot))
            .collect::<Vec<_>>();

        self.multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.set_excerpts_for_path(
                path_key.clone(),
                buffer,
                diff_hunk_ranges,
                editor::DEFAULT_MULTIBUFFER_CONTEXT,
                cx,
            );
        });
        if self.pending_scroll.as_ref() == Some(&path_key) {
            self.scroll_to_path(path_key, window, cx);
        }
    }

    pub async fn handle_status_updates(
        this: WeakEntity<Self>,
        mut recv: postage::watch::Receiver<()>,
        mut cx: AsyncWindowContext,
    ) -> Result<()> {
        while let Some(_) = recv.next().await {
            let buffers_to_load = this.update(&mut cx, |this, cx| this.load_buffers(cx))?;
            for buffer_to_load in buffers_to_load {
                if let Some(buffer) = buffer_to_load.await.log_err() {
                    cx.update(|window, cx| {
                        this.update(cx, |this, cx| this.register_buffer(buffer, window, cx))
                            .ok();
                    })?;
                }
            }
            this.update(&mut cx, |this, _| this.pending_scroll.take())?;
        }

        Ok(())
    }
}

impl EventEmitter<EditorEvent> for ProjectDiff {}

impl Focusable for ProjectDiff {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ProjectDiff {
    type Event = EditorEvent;

    fn to_item_events(event: &EditorEvent, f: impl FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
    }

    fn navigate(
        &mut self,
        data: Box<dyn Any>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor
            .update(cx, |editor, cx| editor.navigate(data, window, cx))
    }

    fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> {
        Some("Project Diff".into())
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, _: &App) -> AnyElement {
        Label::new("Uncommitted Changes")
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("project diagnostics")
    }

    fn as_searchable(&self, _: &Entity<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.editor.for_each_project_item(cx, f)
    }

    fn is_singleton(&self, _: &App) -> bool {
        false
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, _| {
            editor.set_nav_history(Some(nav_history));
        });
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>>
    where
        Self: Sized,
    {
        Some(
            cx.new(|cx| ProjectDiff::new(self.project.clone(), self.workspace.clone(), window, cx)),
        )
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
        format: bool,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.editor.save(format, project, window, cx)
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
        self.editor.reload(project, window, cx)
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.to_any())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.to_any())
        } else {
            None
        }
    }

    fn breadcrumb_location(&self, _: &App) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft
    }

    fn breadcrumbs(&self, theme: &theme::Theme, cx: &App) -> Option<Vec<BreadcrumbText>> {
        self.editor.breadcrumbs(theme, cx)
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
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.multibuffer.read(cx).is_empty();
        if is_empty {
            div()
                .bg(cx.theme().colors().editor_background)
                .flex()
                .items_center()
                .justify_center()
                .size_full()
                .child(Label::new("No uncommitted changes"))
        } else {
            div()
                .bg(cx.theme().colors().editor_background)
                .flex()
                .items_center()
                .justify_center()
                .size_full()
                .child(self.editor.clone())
        }
    }
}
