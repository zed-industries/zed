//! DiffView provides a UI for displaying differences between two buffers.

use anyhow::Result;
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{Editor, EditorEvent, MultiBuffer};
use futures::{FutureExt, select_biased};
use gpui::{
    AnyElement, AnyView, App, AppContext as _, AsyncApp, Context, Entity, EventEmitter,
    FocusHandle, Focusable, IntoElement, Render, Task, Window,
};
use language::{self, Buffer};
use project::Project;
use std::{
    any::{Any, TypeId},
    path::PathBuf,
    pin::pin,
    sync::Arc,
    time::Duration,
};
use ui::{Color, Icon, IconName, Label, LabelCommon as _, SharedString};
use util::paths::PathExt;
use workspace::{
    Item, ItemHandle as _, ItemNavHistory, ToolbarItemLocation, Workspace,
    item::{BreadcrumbText, ItemEvent, SaveOptions, TabContentParams},
    searchable::SearchableItemHandle,
};
use zed_actions::{
    self, DiffText, SelectionData, SourceLocation,
    SourceLocation::{Custom, Path},
};

pub struct DiffView {
    editor: Entity<Editor>,
    old_buffer: Entity<Buffer>,
    new_buffer: Entity<Buffer>,
    buffer_changes_tx: watch::Sender<()>,
    tab_content_text: SharedString,
    tab_tooltip_text: SharedString,
    _recalculate_diff_task: Task<Result<()>>,
}

const RECALCULATE_DIFF_DEBOUNCE: Duration = Duration::from_millis(250);

impl DiffView {
    pub fn open_file_diff(
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
                let old_path = old_buffer
                    .read(cx)
                    .file()
                    .map(|file| file.full_path(cx).to_path_buf());
                let new_path = new_buffer
                    .read(cx)
                    .file()
                    .map(|file| file.full_path(cx).to_path_buf());

                let old_filename = filename(&Path(old_path.clone()));
                let old_path = full_path(&Path(old_path));
                let new_filename = filename(&Path(new_path.clone()));
                let new_path = full_path(&Path(new_path));

                let tab_content_text = diff_tab_text(&old_filename, &new_filename);
                let tab_tooltip_text = diff_tab_text(&old_path, &new_path);

                let diff_view = cx.new(|cx| {
                    DiffView::new(
                        old_buffer,
                        new_buffer,
                        buffer_diff,
                        tab_content_text,
                        tab_tooltip_text,
                        false,
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

    pub fn open_text_diff(
        diff_text_data: &DiffText,
        workspace: &Workspace,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let action = diff_text_data.clone();
        let workspace = workspace.weak_handle();
        window.spawn(cx, async move |cx| {
            let project = workspace.update(cx, |workspace, _| workspace.project().clone())?;
            let old_text_data = action.old_text_data;
            let new_text_data = action.new_text_data;

            let old_buffer =
                cx.new(|cx| language::Buffer::local(old_text_data.text.clone(), cx))?;
            let new_buffer =
                cx.new(|cx| language::Buffer::local(new_text_data.text.clone(), cx))?;

            let language_registry =
                project.read_with(cx, |project, _| project.languages().clone())?;

            if let Some(language_name) = &old_text_data.language {
                if let Ok(language) = language_registry.language_for_name(language_name).await {
                    old_buffer.update(cx, |buffer, cx| {
                        buffer.set_language(Some(language.clone()), cx);
                    })?;
                }
            }

            if let Some(language_name) = &new_text_data.language {
                if let Ok(language) = language_registry.language_for_name(language_name).await {
                    new_buffer.update(cx, |buffer, cx| {
                        buffer.set_language(Some(language), cx);
                    })?;
                }
            }

            let buffer_diff = build_buffer_diff(&old_buffer, &new_buffer, cx).await?;

            let mut old_filename = filename(&old_text_data.source_location);
            let mut old_path = full_path(&old_text_data.source_location);
            let mut new_filename = filename(&new_text_data.source_location);
            let mut new_path = full_path(&new_text_data.source_location);

            if let Some(selection_data) = old_text_data.selection_data {
                old_filename = add_line_location(&old_filename, &selection_data);
                old_path = add_line_location(&old_path, &selection_data);
            };

            if let Some(selection_data) = new_text_data.selection_data {
                new_filename = add_line_location(&new_filename, &selection_data);
                new_path = add_line_location(&new_path, &selection_data);
            }

            let tab_content_text = diff_tab_text(&old_filename, &new_filename);
            let tab_tooltip_text = diff_tab_text(&old_path, &new_path);

            workspace.update_in(cx, |workspace, window, cx| {
                let diff_view = cx.new(|cx| {
                    DiffView::new(
                        old_buffer,
                        new_buffer,
                        buffer_diff,
                        tab_content_text,
                        tab_tooltip_text,
                        true,
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
        tab_content_text: SharedString,
        tab_tooltip_text: SharedString,
        is_read_only: bool,
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
            editor.set_read_only(is_read_only);
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
            tab_content_text,
            tab_tooltip_text,
        }
    }
}

fn filename(source_location: &SourceLocation) -> String {
    let untitled = "untitled";

    match source_location {
        Path(Some(p)) => match p.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => untitled.to_string(),
        },
        Path(None) => untitled.to_string(),
        Custom(path) => path.clone(),
    }
}

fn full_path(source_location: &SourceLocation) -> String {
    let untitled = "untitled";

    match source_location {
        Path(Some(p)) => p.compact().to_string_lossy().to_string(),
        Path(None) => untitled.to_string(),
        Custom(path) => path.clone(),
    }
}

fn add_line_location(source: &String, selection_data: &SelectionData) -> String {
    let start_row = selection_data.start_row;
    let end_row = selection_data.end_row;
    let start_col = selection_data.start_column;
    let end_col = selection_data.end_column;

    let range_text = if start_row == end_row {
        format!("L{}:{}-{}", start_row + 1, start_col + 1, end_col + 1)
    } else {
        format!(
            "L{}:{}-L{}:{}",
            start_row + 1,
            start_col + 1,
            end_row + 1,
            end_col + 1
        )
    };

    format!("{} @ {}", source, range_text)
}

fn diff_tab_text(old: &String, new: &String) -> SharedString {
    format!("{old} ↔ {new}").into()
}

pub async fn build_buffer_diff(
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

    fn tab_content_text(&self, _detail: usize, _: &App) -> SharedString {
        self.tab_content_text.clone()
    }

    fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> {
        Some(self.tab_tooltip_text.clone())
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
    use zed_actions::TextData;

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
    async fn test_file_diff_view(cx: &mut TestAppContext) {
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
                DiffView::open_file_diff(
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

        diff_view.read_with(cx, |diff_view, _| {
            assert_eq!(diff_view.tab_content_text, "old_file.txt ↔ new_file.txt");
            assert_eq!(
                diff_view.tab_tooltip_text,
                "test/old_file.txt ↔ test/new_file.txt"
            );
        })
    }

    #[gpui::test]
    async fn test_selection_against_selection_text_diff_view(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;

        let (workspace, mut cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let diff_view = workspace
            .update_in(cx, |workspace, window, cx| {
                DiffView::open_text_diff(
                    &DiffText {
                        old_text_data: TextData {
                            text: "old line 1\nline 2\nold line 3\nline 4\n".to_string(),
                            source_location: Path(Some(PathBuf::from("a/b/text_1.txt"))),
                            language: None,
                            selection_data: Some(SelectionData {
                                start_row: 0,
                                start_column: 0,
                                end_row: 4,
                                end_column: 0,
                            }),
                        },
                        new_text_data: TextData {
                            text: "new line 1\nline 2\nnew line 3\nline 4\n".to_string(),
                            source_location: Path(Some(PathBuf::from("a/b/text_2.txt"))),
                            language: None,
                            selection_data: Some(SelectionData {
                                start_row: 0,
                                start_column: 0,
                                end_row: 4,
                                end_column: 0,
                            }),
                        },
                    },
                    workspace,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();

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

        diff_view.read_with(cx, |diff_view, _| {
            assert_eq!(
                diff_view.tab_content_text,
                "text_1.txt @ L1:1-L5:1 ↔ text_2.txt @ L1:1-L5:1"
            );
            assert_eq!(
                diff_view.tab_tooltip_text,
                "a/b/text_1.txt @ L1:1-L5:1 ↔ a/b/text_2.txt @ L1:1-L5:1"
            );
        })
    }

    #[gpui::test]
    async fn test_clipboard_against_selection_text_diff_view(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;

        let (workspace, mut cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let diff_view = workspace
            .update_in(cx, |workspace, window, cx| {
                DiffView::open_text_diff(
                    &DiffText {
                        old_text_data: TextData {
                            text: "old line 1\nline 2\nold line 3\nline 4\n".to_string(),
                            source_location: Custom("clipboard".to_string()),
                            language: None,
                            selection_data: None,
                        },
                        new_text_data: TextData {
                            text: "new line 1\nline 2\nnew line 3\nline 4\n".to_string(),
                            source_location: Path(Some(PathBuf::from("a/b/text.txt"))),
                            language: None,
                            selection_data: Some(SelectionData {
                                start_row: 0,
                                start_column: 0,
                                end_row: 4,
                                end_column: 0,
                            }),
                        },
                    },
                    workspace,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();

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

        diff_view.read_with(cx, |diff_view, _| {
            assert_eq!(
                diff_view.tab_content_text,
                "clipboard ↔ text.txt @ L1:1-L5:1"
            );
            assert_eq!(
                diff_view.tab_tooltip_text,
                "clipboard ↔ a/b/text.txt @ L1:1-L5:1"
            );
        })
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
                DiffView::open_file_diff(
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
