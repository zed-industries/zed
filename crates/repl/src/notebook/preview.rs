use anyhow::{Context as _, Result};
use editor::items::SourceLocationRequested;
use editor::{Editor, EditorEvent};

use gpui::{App, Entity, EntityId, EventEmitter, FocusHandle, Focusable, Task, WeakEntity};

use project::{MTime, Project, ProjectItem as _, ProjectPath, project_settings::ProjectSettings};
use settings::Settings as _;
use ui::prelude::*;
use workspace::item::{ItemBufferKind, ItemEvent, SaveOptions, SerializableItem, TabContentParams};
use workspace::pane::ItemNavHistory;
use workspace::{Item, ItemHandle, ItemId, Pane, Workspace, WorkspaceId};
use zed_actions::notebook::{EditSource, OpenNotebook};

use super::{NotebookEditor, NotebookItem};

#[cfg(test)]
#[path = "preview_tests.rs"]
pub(super) mod preview_tests;

struct NotebookSourceView {
    notebook: Entity<NotebookEditor>,
    window: gpui::WindowId,
}

impl editor::Addon for NotebookSourceView {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct NotebookPreview {
    workspace: WeakEntity<Workspace>,
    editor: Entity<Editor>,
    focus_handle: FocusHandle,
    nav_history: Option<ItemNavHistory>,
    pub(super) notebook: Option<Entity<NotebookEditor>>,
    restore_source: bool,
}

pub(super) fn init(cx: &mut App) {
    workspace::register_serializable_item::<NotebookPreview>(cx);
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else { return };
        cx.subscribe_in(&cx.entity(), window, |workspace, _, event, window, cx| {
            let workspace::Event::ItemAdded { item } = event else {
                return;
            };
            let Some(editor) = item.downcast::<Editor>() else {
                return;
            };
            NotebookPreview::open(workspace, editor, false, window, cx);
        })
        .detach();
        workspace.register_action(|workspace, _: &OpenNotebook, window, cx| {
            if let Some(editor) = workspace
                .item_for_action(window, cx)
                .and_then(|item| item.downcast::<Editor>())
            {
                NotebookPreview::open(workspace, editor, true, window, cx);
            }
        });
    })
    .detach();
}

impl NotebookPreview {
    fn source_buffer(
        editor: &Editor,
        project: &Project,
        cx: &App,
    ) -> Option<(Entity<language::Buffer>, ProjectPath)> {
        let buffer = editor.buffer().read(cx).as_singleton()?;
        let source = buffer.read(cx);
        if !source.capability().editable() || source.file()?.disk_state().is_deleted() {
            return None;
        }
        let path = source.project_path(cx)?;
        NotebookItem::supports_path(project, &path, cx).then_some((buffer, path))
    }

    pub fn back_to_notebook_button(editor: &Editor, cx: &App) -> Option<Div> {
        Self::source_buffer(editor, editor.project()?.read(cx), cx)?;
        let focus = editor.focus_handle(cx);
        Some(
            div().debug_selector(|| "back-to-notebook".into()).child(
                Button::new("back-to-notebook", "Back to Notebook")
                    .start_icon(Icon::new(IconName::Book))
                    .on_click(move |_, window, cx| {
                        focus.focus(window, cx);
                        window.dispatch_action(Box::new(OpenNotebook), cx);
                    }),
            ),
        )
    }

    fn open(
        workspace: &mut Workspace,
        editor: Entity<Editor>,
        explicit: bool,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some((buffer, path)) =
            Self::source_buffer(editor.read(cx), workspace.project().read(cx), cx)
        else {
            return;
        };
        if !explicit {
            if workspace.is_restoring() || editor.is_dirty(cx) {
                editor.update(cx, |editor, _| editor.preserve_source_view = true);
                return;
            }
            if editor.read(cx).preserve_source_view {
                return;
            }
        }
        let Some(pane) = workspace.pane_for_item_id(editor.entity_id()) else {
            return;
        };
        let Some(parse) = NotebookItem::try_open(workspace.project(), &path, cx) else {
            return;
        };
        let version = buffer.read(cx).version();
        cx.spawn_in(window, async move |workspace, cx| {
            let cells = parse.await;
            workspace.update_in(cx, |workspace, window, cx| {
                let dirty = editor.is_dirty(cx);
                if workspace.pane_for_item_id(editor.entity_id()).as_ref() != Some(&pane)
                    || pane.read(cx).index_for_item(&editor).is_none()
                    || buffer.read(cx).version() != version
                    || editor.read(cx).active_project_path(cx).as_ref() != Some(&path)
                    || (!explicit
                        && (dirty
                            || workspace.is_restoring()
                            || editor.read(cx).preserve_source_view))
                {
                    return;
                }
                let existing = pane.read(cx).items_of_type::<Self>().find(|preview| {
                    preview
                        .read(cx)
                        .editor
                        .read(cx)
                        .buffer()
                        .read(cx)
                        .as_singleton()
                        .as_ref()
                        == Some(&buffer)
                });
                if let Some(existing) = existing {
                    if explicit {
                        workspace.activate_item(&existing, true, true, window, cx);
                    }
                    return;
                }
                let cells = match cells {
                    Ok(cells) => cells,
                    Err(error) => {
                        if explicit {
                            workspace.show_error(
                                format!("Could not open notebook cells: {error:#}. Check the notebook source and try again."),
                                cx,
                            );
                        }
                        return;
                    }
                };
                let owner = cx.weak_entity();
                let preview =
                    cx.new(|cx| Self::new(owner, editor.clone(), Some(cells), window, cx));
                editor.update(cx, |editor, _| editor.set_nav_history(None));
                pane.update(cx, |pane, cx| {
                    Self::replace_tab(pane, &editor, Box::new(preview), explicit, window, cx);
                });
            })
        })
        .detach_and_log_err(cx);
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        editor: Entity<Editor>,
        parsed: Option<Entity<NotebookItem>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let notebook = parsed.and_then(|item| {
            let project = editor.read(cx).project()?.clone();
            let buffer = editor.read(cx).buffer().read(cx).as_singleton()?;
            if !buffer.read(cx).capability().editable() || item.read(cx).buffer != buffer {
                return None;
            }
            let retained = editor.update(cx, |editor, cx| {
                let retained = editor
                    .addon::<NotebookSourceView>()
                    .filter(|view| {
                        view.window == window.window_handle().window_id()
                            && view.notebook.read(cx).source_is_current(cx)
                            && Some(view.notebook.read(cx).project_path(cx))
                                == editor.active_project_path(cx).as_ref()
                    })
                    .map(|view| view.notebook.clone());
                editor.unregister_addon::<NotebookSourceView>();
                retained
            });
            if retained.is_some() {
                return retained;
            }
            Some(cx.new(|cx| NotebookEditor::new(project, item, window, cx)))
        });
        cx.subscribe_in(
            &editor,
            window,
            |this, _, _: &SourceLocationRequested, window, cx| {
                this.show_source(false, window, cx);
            },
        )
        .detach();
        cx.subscribe_in(&editor, window, |this, _, event, window, cx| {
            Editor::to_item_events(event, &mut |event| cx.emit(event));
            let external_edit = matches!(event, EditorEvent::BufferEdited)
                && this
                    .notebook
                    .as_ref()
                    .is_some_and(|notebook| !notebook.read(cx).source_is_current(cx));
            let changed_file = matches!(event, EditorEvent::FileHandleChanged)
                && this.notebook.as_ref().is_some_and(|notebook| {
                    this.editor.read(cx).active_project_path(cx).as_ref()
                        != Some(notebook.read(cx).project_path(cx))
                });
            if !this.restore_source
                && (external_edit
                    || changed_file
                    || matches!(event, EditorEvent::CapabilityChanged))
            {
                this.restore_source = true;
                this.show_source(false, window, cx);
            }
        })
        .detach();
        Self {
            workspace,
            editor,
            focus_handle: cx.focus_handle(),
            nav_history: None,
            restore_source: notebook.is_none(),
            notebook,
        }
    }

    fn replace_tab(
        pane: &mut Pane,
        from: &dyn ItemHandle,
        to: Box<dyn ItemHandle>,
        explicit: bool,
        window: &mut Window,
        cx: &mut Context<Pane>,
    ) {
        let Some(index) = pane.index_for_item(from) else {
            return;
        };
        let previous_active = pane.active_item();
        let was_active = previous_active
            .as_ref()
            .is_some_and(|item| item.item_id() == from.item_id());
        let focus = explicit || from.item_focus_handle(cx).contains_focused(window, cx);
        let temporary = pane.is_active_preview_item(from.item_id());
        let pinned_count = pane.pinned_count();
        let to_id = to.item_id();
        pane.remove_item(from.item_id(), false, false, window, cx);
        pane.add_item_inner(
            to,
            focus,
            focus,
            explicit || was_active,
            Some(index),
            window,
            cx,
        );
        if index < pinned_count {
            pane.set_pinned_count(pinned_count);
        }
        if temporary {
            pane.replace_preview_item_id(to_id, window, cx);
        }
        if !explicit
            && !was_active
            && let Some(previous) = previous_active
            && let Some(index) = pane.index_for_item(previous.as_ref())
        {
            pane.activate_item(index, false, false, window, cx);
        }
    }

    fn show_source(&self, explicit: bool, window: &mut Window, cx: &mut Context<Self>) {
        let preview = cx.entity();
        let editor = self.editor.clone();
        let notebook = self.notebook.clone().filter(|_| !self.restore_source);
        let workspace = self.workspace.clone();
        window.defer(cx, move |window, cx| {
            let Some(workspace) = workspace.upgrade() else {
                return;
            };
            workspace.update(cx, |workspace, cx| {
                let Some(pane) = workspace.pane_for_item_id(preview.entity_id()) else {
                    return;
                };
                preview.update(cx, |preview, _| preview.nav_history = None);
                let source_buffer = editor.read(cx).buffer().read(cx).as_singleton();
                let editor = pane
                    .read(cx)
                    .items_of_type::<Editor>()
                    .find(|item| item.read(cx).buffer().read(cx).as_singleton() == source_buffer)
                    .unwrap_or(editor);
                if let Some(notebook) = notebook {
                    notebook.update(cx, |notebook, cx| notebook.stop_kernel(cx));
                    editor.update(cx, |editor, _| {
                        editor.register_addon(NotebookSourceView {
                            notebook,
                            window: window.window_handle().window_id(),
                        })
                    });
                }
                editor.update(cx, |editor, _| editor.preserve_source_view = true);
                if workspace.pane_for_item_id(editor.entity_id()).is_some() {
                    if explicit {
                        workspace.activate_item(&editor, true, true, window, cx);
                    }
                    pane.update(cx, |pane, cx| {
                        pane.remove_item(preview.entity_id(), false, false, window, cx)
                    });
                } else {
                    pane.update(cx, |pane, cx| {
                        Self::replace_tab(pane, &preview, Box::new(editor), explicit, window, cx)
                    });
                }
            });
        });
    }
}

impl EventEmitter<ItemEvent> for NotebookPreview {}

impl Focusable for NotebookPreview {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.notebook
            .as_ref()
            .map(|view| view.focus_handle(cx))
            .unwrap_or_else(|| self.focus_handle.clone())
    }
}

impl Item for NotebookPreview {
    type Event = ItemEvent;

    fn set_nav_history(&mut self, history: ItemNavHistory, _: &mut Window, _: &mut Context<Self>) {
        self.nav_history = Some(history);
    }

    fn deactivated(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(history) = &mut self.nav_history {
            history.push::<()>(None, None, cx);
        }
    }

    fn reveal_source(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Box<dyn ItemHandle> {
        self.show_source(false, window, cx);
        Box::new(self.editor.clone())
    }

    fn to_item_events(event: &ItemEvent, f: &mut dyn FnMut(ItemEvent)) {
        f(*event);
    }

    fn tab_content_text(&self, detail: usize, cx: &App) -> SharedString {
        self.editor.read(cx).tab_content_text(detail, cx)
    }

    fn tab_content(&self, params: TabContentParams, _: &Window, cx: &App) -> AnyElement {
        Label::new(self.tab_content_text(params.detail.unwrap_or(0), cx))
            .single_line()
            .color(params.text_color())
            .when(params.preview, |label| label.italic())
            .into_any_element()
    }

    fn tab_icon(&self, _: &Window, _: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Book))
    }

    fn active_project_path(&self, cx: &App) -> Option<ProjectPath> {
        self.editor.read(cx).active_project_path(cx)
    }

    fn buffer_kind(&self, _: &App) -> ItemBufferKind {
        ItemBufferKind::Singleton
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        visit: &mut dyn FnMut(EntityId, &dyn project::ProjectItem),
    ) {
        self.editor.read(cx).for_each_project_item(cx, visit);
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn capability(&self, cx: &App) -> language::Capability {
        self.editor.read(cx).capability(cx)
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.editor.is_dirty(cx)
            || self
                .notebook
                .as_ref()
                .is_some_and(|notebook| notebook.read(cx).has_unsynced_edits(cx))
    }

    fn has_conflict(&self, cx: &App) -> bool {
        self.editor.has_conflict(cx)
    }

    fn has_deleted_file(&self, cx: &App) -> bool {
        self.editor.has_deleted_file(cx)
    }

    fn can_save(&self, cx: &App) -> bool {
        self.editor.can_save(cx)
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if let Some(notebook) = &self.notebook {
            notebook.update(cx, |notebook, cx| {
                notebook.save(options, project, window, cx)
            })
        } else {
            self.editor.update(cx, |editor, cx| {
                editor.save(
                    SaveOptions {
                        format: false,
                        ..options
                    },
                    project,
                    window,
                    cx,
                )
            })
        }
    }

    fn can_save_as(&self, cx: &App) -> bool {
        self.editor.can_save_as(cx)
    }

    fn save_as(
        &mut self,
        project: Entity<Project>,
        path: ProjectPath,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if let Some(notebook) = &self.notebook
            && let Err(error) = notebook.update(cx, |notebook, cx| notebook.flush_cell_edits(cx))
        {
            return Task::ready(Err(error));
        }
        self.editor
            .update(cx, |editor, cx| editor.save_as(project, path, window, cx))
    }

    fn reload(
        &mut self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.editor
            .update(cx, |editor, cx| editor.reload(project, window, cx))
    }

    fn discard_before_close(&mut self, _: &mut Context<Self>) {
        self.restore_source = true;
    }

    fn added_to_workspace(
        &mut self,
        _: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.restore_source {
            self.show_source(false, window, cx);
        }
    }
}

impl Render for NotebookPreview {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .track_focus(&self.focus_handle)
            .bg(cx.theme().colors().editor_background)
            .on_action(
                cx.listener(|this, _: &EditSource, window, cx| this.show_source(true, window, cx)),
            )
            .child(
                h_flex()
                    .justify_between()
                    .px_3()
                    .py_1()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(Label::new("Notebook").color(Color::Muted))
                    .child(
                        Button::new("edit-source", "Edit Source")
                            .start_icon(Icon::new(IconName::Code))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.show_source(true, window, cx)
                            })),
                    ),
            )
            .children(self.notebook.clone())
    }
}

impl SerializableItem for NotebookPreview {
    fn serialized_item_kind() -> &'static str {
        "NotebookPreview"
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        closing: bool,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let workspace_id = workspace.database_id()?;
        if let Some(notebook) = &self.notebook
            && let Err(error) = notebook.update(cx, |notebook, cx| notebook.flush_cell_edits(cx))
        {
            return Some(Task::ready(Err(error)));
        }
        let editor = self.editor.read(cx);
        let (buffer, dirty_snapshot) = editor.buffer_for_session(closing, cx)?;
        let path = editor.active_project_path(cx)?;
        let path = workspace.project().read(cx).absolute_path(&path, cx)?;
        let mtime = buffer
            .read(cx)
            .saved_mtime()
            .and_then(MTime::to_seconds_and_nanos_for_persistence);
        let db = persistence::NotebookPreviewDb::global(cx);
        Some(cx.background_spawn(async move {
            db.save_notebook(
                item_id,
                workspace_id,
                path,
                dirty_snapshot.map(|snapshot| snapshot.text()),
                mtime,
            )
            .await
        }))
    }

    fn should_serialize(&self, event: &Self::Event) -> bool {
        matches!(event, ItemEvent::Edit | ItemEvent::UpdateTab)
    }

    fn deserialize(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let db = persistence::NotebookPreviewDb::global(cx);
        window.spawn(cx, async move |cx| {
            let (path, contents, mtime) = db
                .get_notebook(item_id, workspace_id)?
                .context("No saved notebook path")?;
            let (worktree, path) = project
                .update(cx, |project, cx| {
                    project.find_or_create_worktree(path, false, cx)
                })
                .await?;
            let path = ProjectPath {
                worktree_id: worktree.read_with(cx, |worktree, _| worktree.id()),
                path,
            };
            let buffer = project
                .update(cx, |project, cx| project.open_buffer(path.clone(), cx))
                .await?;
            let (version, restored) = buffer.update(cx, |buffer, cx| {
                let contents = contents.filter(|_| {
                    !buffer.is_dirty()
                        && ProjectSettings::get_global(cx)
                            .session
                            .restore_unsaved_buffers
                });
                let restored = contents.is_some();
                if let Some(contents) = contents {
                    editor::items::restore_serialized_buffer_contents(
                        buffer,
                        contents,
                        mtime.map(|(seconds, nanos)| MTime::from_seconds_and_nanos(seconds, nanos)),
                        cx,
                    );
                }
                (buffer.version(), restored)
            });
            let parse = cx.update(|_, cx| NotebookItem::try_open(&project, &path, cx))?;
            let cells = match parse {
                Some(parse) => parse.await.ok(),
                None => None,
            };
            cx.update(|window, cx| {
                let cells = cells.filter(|_| {
                    (restored || !buffer.read(cx).is_dirty())
                        && buffer.read(cx).version() == version
                });
                let editor = cx.new(|cx| Editor::for_buffer(buffer, Some(project), window, cx));
                cx.new(|cx| Self::new(workspace, editor, cells, window, cx))
            })
        })
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        workspace::delete_unloaded_items(
            alive_items,
            workspace_id,
            "notebook_previews",
            &persistence::NotebookPreviewDb::global(cx),
            cx,
        )
    }
}

mod persistence {
    use db::{
        query,
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };
    use std::path::PathBuf;
    use workspace::{ItemId, WorkspaceDb, WorkspaceId};

    pub(super) struct NotebookPreviewDb(ThreadSafeConnection);

    impl Domain for NotebookPreviewDb {
        const NAME: &str = stringify!(NotebookPreviewDb);
        const MIGRATIONS: &[&str] = &[
            sql!(
                CREATE TABLE notebook_previews (
                    workspace_id INTEGER,
                    item_id INTEGER,
                    path BLOB NOT NULL,
                    alongside_source INTEGER NOT NULL,
                    PRIMARY KEY(workspace_id, item_id),
                    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE
                ) STRICT;
            ),
            sql!(
                ALTER TABLE notebook_previews ADD COLUMN contents TEXT;
                ALTER TABLE notebook_previews ADD COLUMN mtime_seconds INTEGER;
                ALTER TABLE notebook_previews ADD COLUMN mtime_nanos INTEGER;
            ),
        ];
    }

    db::static_connection!(NotebookPreviewDb, [WorkspaceDb]);

    impl NotebookPreviewDb {
        query! {
            pub async fn save_notebook(item_id: ItemId, workspace_id: WorkspaceId, path: PathBuf, contents: Option<String>, mtime: Option<(u64, u32)>) -> Result<()> {
                INSERT OR REPLACE INTO notebook_previews(item_id, workspace_id, path, alongside_source, contents, mtime_seconds, mtime_nanos)
                VALUES (?, ?, ?, FALSE, ?, ?, ?)
            }
        }
        query! {
            pub fn get_notebook(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<(PathBuf, Option<String>, Option<(u64, u32)>)>> {
                SELECT path, contents, mtime_seconds, mtime_nanos FROM notebook_previews WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}
