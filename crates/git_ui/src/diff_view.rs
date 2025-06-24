//! DiffView provides a UI for displaying differences between two buffers.

use anyhow::Result;
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{Editor, EditorEvent, MultiBuffer};
use futures::{FutureExt, select_biased};
use gpui::{
    AnyElement, AnyView, App, AppContext as _, AsyncApp, Context, Entity, EventEmitter,
    FocusHandle, Focusable, IntoElement, Render, Task, Window,
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
use util::paths::PathExt as _;
use workspace::{
    Item, ItemHandle as _, ItemNavHistory, ToolbarItemLocation, Workspace,
    item::{BreadcrumbText, ItemEvent, SaveOptions, TabContentParams},
    searchable::SearchableItemHandle,
};

pub struct DiffView {
    editor: Entity<Editor>,
    old_buffer: Entity<Buffer>,
    new_buffer: Entity<Buffer>,
    buffer_changes_tx: watch::Sender<()>,
    _recalculate_diff_task: Task<Result<()>>,
}

const RECALCULATE_DIFF_DEBOUNCE: Duration = Duration::from_millis(250);

impl DiffView {
    pub fn open(
        old_path: PathBuf,
        new_path: PathBuf,
        workspace: &Workspace,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let workspace = workspace.weak_handle();
        window.spawn(cx, async move |cx| {
            let project = workspace.update(cx, |workspace, _| workspace.project().clone())?;
            let old_buffer = project
                .update(cx, |project, cx| project.open_local_buffer(&old_path, cx))?
                .await?;
            let new_buffer = project
                .update(cx, |project, cx| project.open_local_buffer(&new_path, cx))?
                .await?;

            let buffer_diff = build_buffer_diff(&old_buffer, &new_buffer, cx).await?;

            workspace.update_in(cx, |workspace, window, cx| {
                let diff_view = cx.new(|cx| {
                    DiffView::new(
                        old_buffer,
                        new_buffer,
                        buffer_diff,
                        project.clone(),
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
        })
    }

    pub fn new(
        old_buffer: Entity<Buffer>,
        new_buffer: Entity<Buffer>,
        diff: Entity<BufferDiff>,
        project: Entity<Project>,
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
            buffer_changes_tx,
            old_buffer,
            new_buffer,
            _recalculate_diff_task: cx.spawn(async move |this, cx| {
                while let Ok(_) = buffer_changes_rx.recv().await {
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
}

async fn build_buffer_diff(
    old_buffer: &Entity<Buffer>,
    new_buffer: &Entity<Buffer>,
    cx: &mut AsyncApp,
) -> Result<Entity<BufferDiff>> {
    let old_buffer_snapshot = old_buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
    let new_buffer_snapshot = new_buffer.read_with(cx, |buffer, _| buffer.snapshot())?;

    let diff_snapshot = cx
        .update(|cx| {
            BufferDiffSnapshot::new_with_base_buffer(
                new_buffer_snapshot.text.clone(),
                Some(old_buffer_snapshot.text().into()),
                old_buffer_snapshot,
                cx,
            )
        })?
        .await;

    cx.new(|cx| {
        let mut diff = BufferDiff::new(&new_buffer_snapshot.text, cx);
        diff.set_snapshot(diff_snapshot, &new_buffer_snapshot.text, cx);
        diff
    })
}

impl EventEmitter<EditorEvent> for DiffView {}

impl Focusable for DiffView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Item for DiffView {
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

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        let old_filename = self
            .old_buffer
            .read(cx)
            .file()
            .and_then(|file| {
                Some(
                    file.full_path(cx)
                        .file_name()?
                        .to_string_lossy()
                        .to_string(),
                )
            })
            .unwrap_or_else(|| "untitled".into());
        let new_filename = self
            .new_buffer
            .read(cx)
            .file()
            .and_then(|file| {
                Some(
                    file.full_path(cx)
                        .file_name()?
                        .to_string_lossy()
                        .to_string(),
                )
            })
            .unwrap_or_else(|| "untitled".into());
        format!("{old_filename} ↔ {new_filename}").into()
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<ui::SharedString> {
        let old_path = self
            .old_buffer
            .read(cx)
            .file()
            .map(|file| file.full_path(cx).compact().to_string_lossy().to_string())
            .unwrap_or_else(|| "untitled".into());
        let new_path = self
            .new_buffer
            .read(cx)
            .file()
            .map(|file| file.full_path(cx).compact().to_string_lossy().to_string())
            .unwrap_or_else(|| "untitled".into());
        Some(format!("{old_path} ↔ {new_path}").into())
    }

    fn to_item_events(event: &EditorEvent, f: impl FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Diff View Opened")
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
    }

    fn is_singleton(&self, _: &App) -> bool {
        false
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

    fn can_save(&self, cx: &App) -> bool {
        // The editor handles the new buffer, so delegate to it
        self.editor.read(cx).can_save(cx)
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        // Delegate saving to the editor, which manages the new buffer
        self.editor
            .update(cx, |editor, cx| editor.save(options, project, window, cx))
    }
}

impl Render for DiffView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.editor.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::test::editor_test_context::assert_state_with_diff;
    use gpui::TestAppContext;
    use project::{FakeFs, Fs, Project};
    use settings::{Settings, SettingsStore};
    use std::path::PathBuf;
    use unindent::unindent;
    use util::path;
    use workspace::Workspace;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
            workspace::init_settings(cx);
            editor::init_settings(cx);
            theme::ThemeSettings::register(cx)
        });
    }

    #[gpui::test]
    async fn test_diff_view(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            serde_json::json!({
                "old_file.txt": "old line 1\nline 2\nold line 3\nline 4\n",
                "new_file.txt": "new line 1\nline 2\nnew line 3\nline 4\n"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;

        let (workspace, mut cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let diff_view = workspace
            .update_in(cx, |workspace, window, cx| {
                DiffView::open(
                    PathBuf::from(path!("/test/old_file.txt")),
                    PathBuf::from(path!("/test/new_file.txt")),
                    workspace,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();

        // Verify initial diff
        assert_state_with_diff(
            &diff_view.read_with(cx, |diff_view, _| diff_view.editor.clone()),
            &mut cx,
            &unindent(
                "
                - old line 1
                + ˇnew line 1
                  line 2
                - old line 3
                + new line 3
                  line 4
                ",
            ),
        );

        // Modify the new file on disk
        fs.save(
            path!("/test/new_file.txt").as_ref(),
            &unindent(
                "
                new line 1
                line 2
                new line 3
                line 4
                new line 5
                ",
            )
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        // The diff now reflects the changes to the new file
        cx.executor().advance_clock(RECALCULATE_DIFF_DEBOUNCE);
        assert_state_with_diff(
            &diff_view.read_with(cx, |diff_view, _| diff_view.editor.clone()),
            &mut cx,
            &unindent(
                "
                - old line 1
                + ˇnew line 1
                  line 2
                - old line 3
                + new line 3
                  line 4
                + new line 5
                ",
            ),
        );

        // Modify the old file on disk
        fs.save(
            path!("/test/old_file.txt").as_ref(),
            &unindent(
                "
                new line 1
                line 2
                old line 3
                line 4
                ",
            )
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        // The diff now reflects the changes to the new file
        cx.executor().advance_clock(RECALCULATE_DIFF_DEBOUNCE);
        assert_state_with_diff(
            &diff_view.read_with(cx, |diff_view, _| diff_view.editor.clone()),
            &mut cx,
            &unindent(
                "
                  ˇnew line 1
                  line 2
                - old line 3
                + new line 3
                  line 4
                + new line 5
                ",
            ),
        );
    }

    #[gpui::test]
    async fn test_save_changes_in_diff_view(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            serde_json::json!({
                "old_file.txt": "old line 1\nline 2\nold line 3\nline 4\n",
                "new_file.txt": "new line 1\nline 2\nnew line 3\nline 4\n"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let diff_view = workspace
            .update_in(cx, |workspace, window, cx| {
                DiffView::open(
                    PathBuf::from(path!("/test/old_file.txt")),
                    PathBuf::from(path!("/test/new_file.txt")),
                    workspace,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();

        diff_view.update_in(cx, |diff_view, window, cx| {
            diff_view.editor.update(cx, |editor, cx| {
                editor.insert("modified ", window, cx);
            });
        });

        diff_view.update_in(cx, |diff_view, _, cx| {
            let buffer = diff_view.new_buffer.read(cx);
            assert!(buffer.is_dirty(), "Buffer should be dirty after edits");
        });

        let save_task = diff_view.update_in(cx, |diff_view, window, cx| {
            workspace::Item::save(
                diff_view,
                workspace::item::SaveOptions::default(),
                project.clone(),
                window,
                cx,
            )
        });

        save_task.await.expect("Save should succeed");

        let saved_content = fs.load(path!("/test/new_file.txt").as_ref()).await.unwrap();
        assert_eq!(
            saved_content,
            "modified new line 1\nline 2\nnew line 3\nline 4\n"
        );

        diff_view.update_in(cx, |diff_view, _, cx| {
            let buffer = diff_view.new_buffer.read(cx);
            assert!(!buffer.is_dirty(), "Buffer should not be dirty after save");
        });
    }
}
