//! GitDiffView provides a UI for displaying git diffs with proper tab titles and read-only protection.

use anyhow::Result;
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{Editor, EditorEvent, MultiBuffer};
use futures::{FutureExt, select_biased};
use gpui::{
    AnyElement, AnyView, App, AppContext as _, Context, Entity, EventEmitter, FocusHandle,
    Focusable, IntoElement, Render, Task, Window,
};
use language::Buffer;
use project::Project;
use std::{
    any::{Any, TypeId},
    path::PathBuf,
    pin::pin,
    sync::Arc,
    time::Duration,
};
use ui::{Color, Icon, IconName, Label, LabelCommon as _, SharedString};
use workspace::{
    Item, ItemHandle as _, ItemNavHistory, ToolbarItemLocation, Workspace,
    item::{BreadcrumbText, ItemEvent, SaveOptions, TabContentParams},
    searchable::SearchableItemHandle,
};

pub struct GitDiffView {
    editor: Entity<Editor>,
    old_buffer: Entity<Buffer>,
    new_buffer: Entity<Buffer>,
    file_path: String,
    old_commit_hash: String,
    new_commit_hash: String,
    buffer_changes_tx: watch::Sender<()>,
    _recalculate_diff_task: Task<Result<()>>,
}

const RECALCULATE_DIFF_DEBOUNCE: Duration = Duration::from_millis(250);

impl GitDiffView {
    pub fn new(
        old_buffer: Entity<Buffer>,
        new_buffer: Entity<Buffer>,
        diff: Entity<BufferDiff>,
        project: Entity<Project>,
        file_path: String,
        old_commit_hash: String,
        new_commit_hash: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::singleton(new_buffer.clone(), cx);
            multibuffer.add_diff(diff.clone(), cx);
            multibuffer
        });
        let editor = cx.new(|cx| {
            let mut editor =
                Editor::for_multibuffer(multibuffer.clone(), Some(project.clone()), window, cx);
            editor.start_temporary_diff_override();
            editor.disable_diagnostics(cx);
            editor.set_expand_all_diff_hunks(cx);
            editor.set_render_diff_hunk_controls(
                Arc::new(|_, _, _, _, _, _, _, _| gpui::Empty.into_any_element()),
                cx,
            );
            // Set read-only to prevent editing historical diffs
            editor.set_read_only(true);
            editor
        });

        let (buffer_changes_tx, mut buffer_changes_rx) = watch::channel(());

        for buffer in [&old_buffer, &new_buffer] {
            cx.subscribe(buffer, move |this, _, event, _| match event {
                language::BufferEvent::Edited
                | language::BufferEvent::LanguageChanged
                | language::BufferEvent::Reparsed => {
                    this.buffer_changes_tx.send(()).ok();
                }
                _ => {}
            })
            .detach();
        }

        Self {
            editor,
            old_buffer,
            new_buffer,
            file_path,
            old_commit_hash,
            new_commit_hash,
            buffer_changes_tx,
            _recalculate_diff_task: cx.spawn(async move |this, cx| {
                while buffer_changes_rx.recv().await.is_ok() {
                    loop {
                        let mut timer = cx
                            .background_executor()
                            .timer(RECALCULATE_DIFF_DEBOUNCE)
                            .fuse();
                        let mut recv = pin!(buffer_changes_rx.recv().fuse());
                        select_biased! {
                            _ = timer => break,
                            _ = recv => continue,
                        }
                    }

                    log::trace!("start recalculating");
                    let (old_snapshot, new_snapshot) = this.update(cx, |this, cx| {
                        (
                            this.old_buffer.read(cx).snapshot(),
                            this.new_buffer.read(cx).snapshot(),
                        )
                    })?;
                    let diff_snapshot = cx
                        .update(|cx| {
                            BufferDiffSnapshot::new_with_base_buffer(
                                new_snapshot.text.clone(),
                                Some(old_snapshot.text().into()),
                                old_snapshot,
                                cx,
                            )
                        })?
                        .await;
                    diff.update(cx, |diff, cx| {
                        diff.set_snapshot(diff_snapshot, &new_snapshot, cx)
                    })?;
                    log::trace!("finish recalculating");
                }
                Ok(())
            }),
        }
    }

    pub fn open(
        old_text: String,
        new_text: String,
        file_path: String,
        old_commit_hash: String,
        new_commit_hash: String,
        project: Entity<Project>,
        workspace: &Workspace,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Task<Result<Entity<Self>>>> {
        let workspace = workspace.weak_handle();
        Some(window.spawn(cx, async move |cx| {
            // Detect language from file extension
            let path = PathBuf::from(&file_path);
            let languages_registry =
                project.read_with(cx, |project, _| project.languages().clone())?;
            let language = languages_registry
                .load_language_for_file_path(&path)
                .await
                .ok();

            // Create buffers with text content and syntax highlighting
            let old_buffer = cx.new(|cx| {
                let mut buffer = Buffer::local(old_text, cx);
                if let Some(ref lang) = language {
                    buffer.set_language(Some(lang.clone()), cx);
                }
                buffer
            })?;

            let new_buffer = cx.new(|cx| {
                let mut buffer = Buffer::local(new_text, cx);
                if let Some(ref lang) = language {
                    buffer.set_language(Some(lang.clone()), cx);
                }
                buffer
            })?;

            // Build the diff
            let old_buffer_snapshot = old_buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
            let new_buffer_snapshot = new_buffer.read_with(cx, |buffer, _| buffer.snapshot())?;

            let diff_snapshot = cx
                .update(|_, cx| {
                    BufferDiffSnapshot::new_with_base_buffer(
                        new_buffer_snapshot.text.clone(),
                        Some(old_buffer_snapshot.text().into()),
                        old_buffer_snapshot,
                        cx,
                    )
                })?
                .await;

            let diff = cx.new(|cx| {
                let mut diff = BufferDiff::new(&new_buffer_snapshot.text, cx);
                diff.set_snapshot(diff_snapshot, &new_buffer_snapshot.text, cx);
                diff
            })?;

            workspace.update_in(cx, |workspace, window, cx| {
                let diff_view = cx.new(|cx| {
                    GitDiffView::new(
                        old_buffer,
                        new_buffer,
                        diff,
                        project.clone(),
                        file_path,
                        old_commit_hash,
                        new_commit_hash,
                        window,
                        cx,
                    )
                });

                let pane = workspace.active_pane();
                pane.update(cx, |pane, cx| {
                    pane.add_item(Box::new(diff_view.clone()), true, true, None, window, cx);
                });

                diff_view
            })
        }))
    }
}

impl EventEmitter<EditorEvent> for GitDiffView {}

impl Focusable for GitDiffView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Item for GitDiffView {
    type Event = EditorEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Diff).color(Color::Muted))
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        Label::new(self.tab_content_text(params.detail.unwrap_or_default(), cx))
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        let path_buf = PathBuf::from(&self.file_path);
        let filename = path_buf.file_name().unwrap_or_default().to_string_lossy();

        // Shorten commit hashes for display (first 8 characters)
        let old_short_hash = if self.old_commit_hash.len() > 8 {
            &self.old_commit_hash[..8]
        } else {
            &self.old_commit_hash
        };
        let new_short_hash = if self.new_commit_hash.len() > 8 {
            &self.new_commit_hash[..8]
        } else {
            &self.new_commit_hash
        };

        format!(
            "{} ({}) ↔ {} ({})",
            filename, old_short_hash, filename, new_short_hash
        )
        .into()
    }

    fn tab_tooltip_text(&self, _cx: &App) -> Option<ui::SharedString> {
        Some(
            format!(
                "{}: {} ↔ {}",
                self.file_path, self.old_commit_hash, self.new_commit_hash
            )
            .into(),
        )
    }

    fn to_item_events(event: &EditorEvent, f: impl FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Git Diff View Opened")
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
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

    fn navigate(
        &mut self,
        data: Box<dyn Any>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor
            .update(cx, |editor, cx| editor.navigate(data, window, cx))
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

    fn can_save(&self, _cx: &App) -> bool {
        false // Git diffs should not be saveable
    }

    fn save(
        &mut self,
        _options: SaveOptions,
        _project: Entity<Project>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        Task::ready(Ok(())) // No-op save operation
    }
}

impl Render for GitDiffView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.editor.clone()
    }
}
