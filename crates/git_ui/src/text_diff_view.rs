//! TextDiffView provides a UI for displaying differences between two buffers.

use anyhow::{Result, bail};
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{
    Editor, EditorEvent, MultiBuffer, ToPoint,
    actions::{DiffText, TextSource},
};
use futures::{FutureExt, select_biased};
use gpui::{
    AnyElement, AnyView, App, AppContext as _, AsyncApp, Context, Entity, EventEmitter,
    FocusHandle, Focusable, IntoElement, Render, Task, Window,
};
use language::{self, Buffer};
use project::Project;
use std::{
    any::{Any, TypeId},
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
use zed_actions;

pub struct TextDiffView {
    editor: Entity<Editor>,
    old_text_source: TextSource,
    new_text_source: TextSource,
    buffer_changes_tx: watch::Sender<()>,
    _recalculate_diff_task: Task<Result<()>>,
}

const RECALCULATE_DIFF_DEBOUNCE: Duration = Duration::from_millis(250);

impl TextDiffView {
    pub fn open(
        diff_text_data: &DiffText,
        workspace: &Workspace,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Task<Result<Entity<Self>>>> {
        let workspace = workspace.weak_handle();
        let old_text_source = diff_text_data.old_text_source.clone();
        let new_text_source = diff_text_data.new_text_source.clone();

        let mut buffer = |text_source: &TextSource| match text_source {
            TextSource::Clipboard(text) => Some(cx.new(|cx| language::Buffer::local(text, cx))),
            TextSource::MultiBuffer(multibuffer) => {
                multibuffer.read_with(cx, |mb, _| mb.as_singleton())
            }
        };
        let old_buffer = buffer(&old_text_source)?;
        let new_buffer = buffer(&new_text_source)?;

        let mut old_language = old_buffer.read_with(cx, |buffer, _| buffer.language().cloned());
        let mut new_language = new_buffer.read_with(cx, |buffer, _| buffer.language().cloned());

        if old_language.is_none() {
            old_language = new_language.clone();
        }

        if new_language.is_none() {
            new_language = old_language.clone();
        }

        old_buffer.update(cx, |buffer, cx| {
            buffer.set_language(old_language.clone(), cx);
        });

        new_buffer.update(cx, |buffer, cx| {
            buffer.set_language(new_language.clone(), cx);
        });

        let task = window.spawn(cx, async move |cx| {
            let project = workspace.update(cx, |workspace, _| workspace.project().clone())?;
            let buffer_diff = build_buffer_diff(old_buffer.clone(), new_buffer.clone(), cx).await?;

            workspace.update_in(cx, |workspace, window, cx| {
                let diff_view = cx.new(|cx| {
                    TextDiffView::new(
                        old_text_source,
                        new_text_source,
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
        });

        Some(task)
    }

    // TODO - diff
    // pub fn text(text_source: TextSource, cx: &mut App) -> Option<String> {
    //     match text_source {
    //         Clipboard(text) => Some(text),
    //         Editor(editor) => editor.read_with(cx, |editor, mut cx| {
    //             let selections = editor.selections.all::<usize>(cx);

    //             let Some(first_selection) = selections.first() else {
    //                 return None;
    //             };

    //             let buffer = editor.buffer().read(cx).snapshot(cx);

    //             let selection_range = if first_selection.is_empty() {
    //                 0..buffer.len()
    //             } else {
    //                 first_selection.range()
    //             };

    //             let mut selected_text = String::new();

    //             for chunk in buffer.text_for_range(selection_range.clone()) {
    //                 selected_text.push_str(chunk);
    //             }

    //             let (full_path, language_name) =
    //                 buffer
    //                     .as_singleton()
    //                     .map_or((None, None), |(_, _, buffer)| {
    //                         let file = buffer.file();
    //                         let full_path = file.map(|f| f.full_path(cx).to_path_buf());
    //                         let language_name = buffer
    //                             .language()
    //                             .map(|language| language.name().to_string());
    //                         (full_path, language_name)
    //                     });

    //             let selection_start = selection_range.start.to_point(&buffer);
    //             let selection_end = selection_range.end.to_point(&buffer);

    //             Some(selected_text)
    //         }),
    //     }
    // }

    // TODO - diff - match selections
    // TODO - diff - allow to be bidirectionally edited
    // TODO - diff - no selection = full buffer, or take first

    // TODO - diff - passing in both of these things feels bad
    pub fn new(
        old_text_source: TextSource,
        new_text_source: TextSource,
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
            old_text_source,
            new_text_source,
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
                    let (old_snapshot, new_snapshot) = this.update(cx, |_, cx| {
                        (
                            old_buffer.read(cx).snapshot(),
                            new_buffer.read(cx).snapshot(),
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

pub async fn build_buffer_diff(
    old_buffer: Entity<Buffer>,
    new_buffer: Entity<Buffer>,
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

impl EventEmitter<EditorEvent> for TextDiffView {}

impl Focusable for TextDiffView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Item for TextDiffView {
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
        // TODO - diff - line location
        let title_text = |text_source: &TextSource| match text_source {
            TextSource::Clipboard(_) => "Clipboard".to_string(),
            TextSource::MultiBuffer(multibuffer) => multibuffer.read(cx).title(cx).to_string(),
        };
        let old_name = title_text(&self.old_text_source);
        let new_name = title_text(&self.new_text_source);

        format!("{old_name} ↔ {new_name}").into()
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        // TODO - diff - line location
        let tooltip_text = |text_source: &TextSource| match text_source {
            TextSource::Clipboard(_) => "Clipboard".to_string(),
            TextSource::MultiBuffer(multibuffer) => multibuffer
                .read(cx)
                .as_singleton()
                .map(|b| {
                    b.read(cx)
                        .file()
                        .map(|f| f.full_path(cx).compact().to_string_lossy().to_string())
                })
                .flatten()
                .unwrap_or("untitled".into()),
        };
        let old_tooltip = tooltip_text(&self.old_text_source);
        let new_tooltip = tooltip_text(&self.new_text_source);

        Some(format!("{old_tooltip} ↔ {new_tooltip}").into())
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

impl Render for TextDiffView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.editor.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::test::editor_test_context::assert_state_with_diff;
    use gpui::TestAppContext;
    use project::{FakeFs, Project};
    use settings::{Settings, SettingsStore};
    use std::path::PathBuf;
    use unindent::unindent;
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

    // TODO - diff
    // #[gpui::test]
    // async fn test_selection_against_selection_text_diff_view(cx: &mut TestAppContext) {
    //     init_test(cx);

    //     let fs = FakeFs::new(cx.executor());

    //     let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;

    //     let (workspace, mut cx) =
    //         cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    //     let diff_view = workspace
    //         .update_in(cx, |workspace, window, cx| {
    //             TextDiffView::open(
    //                 &DiffText {
    //                     old_text_source: TextData {
    //                         text: "old line 1\nline 2\nold line 3\nline 4\n".to_string(),
    //                         source_location: Path(Some(PathBuf::from("a/b/text_1.txt"))),
    //                         language: None,
    //                         selection_data: Some(SelectionData {
    //                             start_row: 0,
    //                             start_column: 0,
    //                             end_row: 4,
    //                             end_column: 0,
    //                         }),
    //                     },
    //                     new_text_source: TextData {
    //                         text: "new line 1\nline 2\nnew line 3\nline 4\n".to_string(),
    //                         source_location: Path(Some(PathBuf::from("a/b/text_2.txt"))),
    //                         language: None,
    //                         selection_data: Some(SelectionData {
    //                             start_row: 0,
    //                             start_column: 0,
    //                             end_row: 4,
    //                             end_column: 0,
    //                         }),
    //                     },
    //                 },
    //                 workspace,
    //                 window,
    //                 cx,
    //             )
    //         })
    //         .await
    //         .unwrap();

    //     assert_state_with_diff(
    //         &diff_view.read_with(cx, |diff_view, _| diff_view.editor.clone()),
    //         &mut cx,
    //         &unindent(
    //             "
    //             - old line 1
    //             + ˇnew line 1
    //               line 2
    //             - old line 3
    //             + new line 3
    //               line 4
    //             ",
    //         ),
    //     );

    //     diff_view.read_with(cx, |diff_view, _| {
    //         assert_eq!(
    //             diff_view.tab_content_text,
    //             "text_1.txt @ L1:1-L5:1 ↔ text_2.txt @ L1:1-L5:1"
    //         );
    //         assert_eq!(
    //             diff_view.tab_tooltip_text,
    //             "a/b/text_1.txt @ L1:1-L5:1 ↔ a/b/text_2.txt @ L1:1-L5:1"
    //         );
    //     })
    // }

    // #[gpui::test]
    // async fn test_clipboard_against_selection_text_diff_view(cx: &mut TestAppContext) {
    //     init_test(cx);

    //     let fs = FakeFs::new(cx.executor());

    //     let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;

    //     let (workspace, mut cx) =
    //         cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    //     let diff_view = workspace
    //         .update_in(cx, |workspace, window, cx| {
    //             TextDiffView::open(
    //                 &DiffText {
    //                     old_text_source: TextData {
    //                         text: "old line 1\nline 2\nold line 3\nline 4\n".to_string(),
    //                         source_location: Custom("clipboard".to_string()),
    //                         language: None,
    //                         selection_data: None,
    //                     },
    //                     new_text_source: TextData {
    //                         text: "new line 1\nline 2\nnew line 3\nline 4\n".to_string(),
    //                         source_location: Path(Some(PathBuf::from("a/b/text.txt"))),
    //                         language: None,
    //                         selection_data: Some(SelectionData {
    //                             start_row: 0,
    //                             start_column: 0,
    //                             end_row: 4,
    //                             end_column: 0,
    //                         }),
    //                     },
    //                 },
    //                 workspace,
    //                 window,
    //                 cx,
    //             )
    //         })
    //         .await
    //         .unwrap();

    //     assert_state_with_diff(
    //         &diff_view.read_with(cx, |diff_view, _| diff_view.editor.clone()),
    //         &mut cx,
    //         &unindent(
    //             "
    //             - old line 1
    //             + ˇnew line 1
    //               line 2
    //             - old line 3
    //             + new line 3
    //               line 4
    //             ",
    //         ),
    //     );

    //     diff_view.read_with(cx, |diff_view, _| {
    //         assert_eq!(
    //             diff_view.tab_content_text,
    //             "clipboard ↔ text.txt @ L1:1-L5:1"
    //         );
    //         assert_eq!(
    //             diff_view.tab_tooltip_text,
    //             "clipboard ↔ a/b/text.txt @ L1:1-L5:1"
    //         );
    //     })
    // }
}
