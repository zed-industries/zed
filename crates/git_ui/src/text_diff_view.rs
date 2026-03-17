//! TextDiffView currently provides a UI for displaying differences between the clipboard and selected text.

use anyhow::Result;
use buffer_diff::BufferDiff;
use editor::{Editor, EditorEvent, MultiBuffer, ToPoint, actions::DiffClipboardWithSelectionData};
use futures::{FutureExt, select_biased};
use gpui::{
    AnyElement, App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, FocusHandle,
    Focusable, IntoElement, Render, Task, Window,
};
use language::{self, Buffer, Language, Point};
use project::Project;
use std::{
    any::{Any, TypeId},
    ops::Range,
    pin::pin,
    sync::Arc,
    time::Duration,
};
use ui::{Color, Icon, IconName, Label, LabelCommon as _, SharedString};
use util::paths::PathExt;

use workspace::{
    Item, ItemHandle as _, ItemNavHistory, Workspace,
    item::{ItemEvent, SaveOptions, TabContentParams},
    searchable::SearchableItemHandle,
};

pub struct TextDiffView {
    diff_editor: Entity<Editor>,
    title: SharedString,
    path: Option<SharedString>,
    buffer_changes_tx: watch::Sender<()>,
    _recalculate_diff_task: Task<Result<()>>,
}

const RECALCULATE_DIFF_DEBOUNCE: Duration = Duration::from_millis(250);

impl TextDiffView {
    pub fn open(
        diff_data: &DiffClipboardWithSelectionData,
        workspace: &Workspace,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Task<Result<Entity<Self>>>> {
        let source_editor = diff_data.editor.clone();

        let selection_data = source_editor.update(cx, |editor, cx| {
            let multibuffer = editor.buffer().read(cx);
            let source_buffer = multibuffer.as_singleton()?;
            let selections = editor.selections.all::<Point>(&editor.display_snapshot(cx));
            let buffer_snapshot = source_buffer.read(cx);
            let first_selection = selections.first()?;
            let max_point = buffer_snapshot.max_point();

            if first_selection.is_empty() {
                let full_range = Point::new(0, 0)..max_point;
                return Some((source_buffer, full_range));
            }

            let start = first_selection.start;
            let end = first_selection.end;
            Some((source_buffer, start..end))
        });

        let Some((source_buffer, selection_range)) = selection_data else {
            log::warn!("There should always be at least one selection in Zed. This is a bug.");
            return None;
        };
        let language = source_buffer.read(cx).language().cloned();

        let clipboard_text = diff_data.clipboard_text.clone();
        let clipboard_buffer = build_buffer_from_text(clipboard_text, language.clone(), cx);

        let selection_text = source_buffer.read_with(cx, |buffer, _| {
            buffer
                .text_for_range(selection_range.clone())
                .collect::<String>()
        });
        let selection_buffer = build_buffer_from_text(selection_text, language.clone(), cx);
        let selection_buffer_snapshot = selection_buffer.read(cx).snapshot();
        let diff_buffer = cx.new(|cx| BufferDiff::new(&selection_buffer_snapshot.text, cx));

        let workspace = workspace.weak_handle();
        let task = window.spawn(cx, async move |cx| {
            let project = workspace.update(cx, |workspace, _| workspace.project().clone())?;

            update_diff_buffer(
                &diff_buffer,
                &source_buffer,
                &selection_buffer,
                &clipboard_buffer,
                cx,
            )
            .await?;

            workspace.update_in(cx, |workspace, window, cx| {
                let diff_view = cx.new(|cx| {
                    TextDiffView::new(
                        clipboard_buffer,
                        selection_buffer,
                        source_editor,
                        source_buffer,
                        selection_range,
                        diff_buffer,
                        project,
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

    pub fn new(
        clipboard_buffer: Entity<Buffer>,
        selection_buffer: Entity<Buffer>,
        source_editor: Entity<Editor>,
        source_buffer: Entity<Buffer>,
        source_range: Range<Point>,
        diff_buffer: Entity<BufferDiff>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(language::Capability::ReadWrite);
            let selection_snapshot = selection_buffer.read(cx).snapshot();
            let full_range_in_selection_buffer = Point::new(0, 0)..selection_snapshot.max_point();
            multibuffer.set_excerpts_for_buffer(
                selection_buffer.clone(),
                [full_range_in_selection_buffer],
                0,
                cx,
            );
            multibuffer.add_diff(diff_buffer.clone(), cx);
            multibuffer
        });
        let diff_editor = cx.new(|cx| {
            let mut editor = Editor::for_multibuffer(multibuffer, Some(project), window, cx);
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

        cx.subscribe(&source_buffer, move |this, _, event, _| match event {
            language::BufferEvent::Edited { .. }
            | language::BufferEvent::LanguageChanged(_)
            | language::BufferEvent::Reparsed => {
                this.buffer_changes_tx.send(()).ok();
            }
            _ => {}
        })
        .detach();

        let editor = source_editor.read(cx);
        let title = editor.buffer().read(cx).title(cx).to_string();
        let selection_location_text = selection_location_text(editor, cx);
        let selection_location_title = selection_location_text
            .as_ref()
            .map(|text| format!("{} @ {}", title, text))
            .unwrap_or(title);

        let path = editor
            .buffer()
            .read(cx)
            .as_singleton()
            .and_then(|b| {
                b.read(cx)
                    .file()
                    .map(|f| f.full_path(cx).compact().to_string_lossy().into_owned())
            })
            .unwrap_or("untitled".into());

        let selection_location_path = selection_location_text
            .map(|text| format!("{} @ {}", path, text))
            .unwrap_or(path);

        Self {
            diff_editor,
            title: format!("Clipboard ↔ {selection_location_title}").into(),
            path: Some(format!("Clipboard ↔ {selection_location_path}").into()),
            buffer_changes_tx,
            _recalculate_diff_task: cx.spawn(async move |_, cx| {
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

                    let latest_text = source_buffer.read_with(cx, |buffer, _| {
                        buffer
                            .text_for_range(source_range.clone())
                            .collect::<String>()
                    });
                    selection_buffer.update(cx, |buffer, cx| buffer.set_text(latest_text, cx));

                    update_diff_buffer(
                        &diff_buffer,
                        &source_buffer,
                        &selection_buffer,
                        &clipboard_buffer,
                        cx,
                    )
                    .await?;
                    log::trace!("finish recalculating");
                }
                Ok(())
            }),
        }
    }
}

fn build_buffer_from_text(
    text: String,
    language: Option<Arc<Language>>,
    cx: &mut App,
) -> Entity<Buffer> {
    cx.new(|cx| {
        let mut buffer = language::Buffer::local(text, cx);
        buffer.set_language(language, cx);
        buffer
    })
}

async fn update_diff_buffer(
    diff: &Entity<BufferDiff>,
    source_buffer: &Entity<Buffer>,
    selection_buffer: &Entity<Buffer>,
    clipboard_buffer: &Entity<Buffer>,
    cx: &mut AsyncApp,
) -> Result<()> {
    let source_buffer_snapshot = source_buffer.read_with(cx, |buffer, _| buffer.snapshot());
    let language = source_buffer_snapshot.language().cloned();
    let language_registry = source_buffer.read_with(cx, |buffer, _| buffer.language_registry());

    let selection_buffer_snapshot = selection_buffer.read_with(cx, |buffer, _| buffer.snapshot());
    let base_buffer_snapshot = clipboard_buffer.read_with(cx, |buffer, _| buffer.snapshot());
    let base_text = base_buffer_snapshot.text();

    let update = diff
        .update(cx, |diff, cx| {
            diff.update_diff(
                selection_buffer_snapshot.text.clone(),
                Some(Arc::from(base_text.as_str())),
                Some(true),
                language.clone(),
                cx,
            )
        })
        .await;

    diff.update(cx, |diff, cx| {
        diff.language_changed(language, language_registry, cx);
        diff.set_snapshot(update, &selection_buffer_snapshot.text, cx)
    })
    .await;
    Ok(())
}

impl EventEmitter<EditorEvent> for TextDiffView {}

impl Focusable for TextDiffView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.diff_editor.focus_handle(cx)
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

    fn tab_content_text(&self, _detail: usize, _: &App) -> SharedString {
        self.title.clone()
    }

    fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> {
        self.path.clone()
    }

    fn to_item_events(event: &EditorEvent, f: &mut dyn FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Selection Diff View Opened")
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.diff_editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.diff_editor.clone().into())
        } else {
            None
        }
    }

    fn as_searchable(&self, _: &Entity<Self>, _: &App) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.diff_editor.clone()))
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.diff_editor.for_each_project_item(cx, f)
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.diff_editor.update(cx, |editor, _| {
            editor.set_nav_history(Some(nav_history));
        });
    }

    fn navigate(
        &mut self,
        data: Arc<dyn Any + Send>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.diff_editor
            .update(cx, |editor, cx| editor.navigate(data, window, cx))
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.diff_editor.update(cx, |editor, cx| {
            editor.added_to_workspace(workspace, window, cx)
        });
    }

    fn can_save(&self, cx: &App) -> bool {
        // The editor handles the new buffer, so delegate to it
        self.diff_editor.read(cx).can_save(cx)
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        // Delegate saving to the editor, which manages the new buffer
        self.diff_editor
            .update(cx, |editor, cx| editor.save(options, project, window, cx))
    }
}

pub fn selection_location_text(editor: &Editor, cx: &App) -> Option<String> {
    let buffer = editor.buffer().read(cx);
    let buffer_snapshot = buffer.snapshot(cx);
    let first_selection = editor.selections.disjoint_anchors().first()?;

    let selection_start = first_selection.start.to_point(&buffer_snapshot);
    let selection_end = first_selection.end.to_point(&buffer_snapshot);

    let start_row = selection_start.row;
    let start_column = selection_start.column;
    let end_row = selection_end.row;
    let end_column = selection_end.column;

    let range_text = if start_row == end_row {
        format!("L{}:{}-{}", start_row + 1, start_column + 1, end_column + 1)
    } else {
        format!(
            "L{}:{}-L{}:{}",
            start_row + 1,
            start_column + 1,
            end_row + 1,
            end_column + 1
        )
    };

    Some(range_text)
}

impl Render for TextDiffView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.diff_editor.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::{MultiBufferOffset, test::editor_test_context::assert_state_with_diff};
    use gpui::{TestAppContext, VisualContext};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use unindent::unindent;
    use util::{path, test::marked_text_ranges};
    use workspace::MultiWorkspace;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
        });
    }

    #[gpui::test]
    async fn test_diffing_clipboard_against_partial_multiline_selection(cx: &mut TestAppContext) {
        base_test(
            path!("/test"),
            path!("/test/text.txt"),
            "line 2\nline 3",
            "line 1\nli«ne 2\nliˇ»ne 3",
            &unindent(
                "
                - line 2
                - line 3
                + ˇne 2
                + li",
            ),
            "Clipboard ↔ text.txt @ L2:3-L3:3",
            &format!("Clipboard ↔ {} @ L2:3-L3:3", path!("test/text.txt")),
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_diffing_clipboard_partial_clipboard_against_full_line(cx: &mut TestAppContext) {
        base_test(
            path!("/test"),
            path!("/test/text.txt"),
            "ne 2\nli",
            "line 1\n«line 2\nline 3ˇ»",
            &unindent(
                "
                - ne 2
                - li
                + ˇline 2
                + line 3",
            ),
            "Clipboard ↔ text.txt @ L2:1-L3:7",
            &format!("Clipboard ↔ {} @ L2:1-L3:7", path!("test/text.txt")),
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_diffing_clipboard_with_selection_at_file_end_without_trailing_newline(
        cx: &mut TestAppContext,
    ) {
        base_test(
            path!("/test"),
            path!("/test/text.txt"),
            "line 1\nline 2\n√√√√√",
            "«line 1\nline 2\n√√√√√ˇ»",
            &unindent(
                "
                  ˇline 1
                  line 2
                  √√√√√",
            ),
            "Clipboard ↔ text.txt @ L1:1-L3:16",
            &format!("Clipboard ↔ {} @ L1:1-L3:16", path!("test/text.txt")),
            cx,
        )
        .await;
    }

    async fn base_test(
        project_root: &str,
        file_path: &str,
        clipboard_text: &str,
        editor_text: &str,
        expected_diff: &str,
        expected_tab_title: &str,
        expected_tab_tooltip: &str,
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let file_name = std::path::Path::new(file_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            project_root,
            json!({
                file_name: editor_text
            }),
        )
        .await;

        let project = Project::test(fs, [project_root.as_ref()], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let buffer = project
            .update(cx, |project, cx| project.open_local_buffer(file_path, cx))
            .await
            .unwrap();

        let editor = cx.new_window_entity(|window, cx| {
            let mut editor = Editor::for_buffer(buffer, None, window, cx);
            let (unmarked_text, selection_ranges) = marked_text_ranges(editor_text, false);
            editor.set_text(unmarked_text, window, cx);
            editor.change_selections(Default::default(), window, cx, |s| {
                s.select_ranges(
                    selection_ranges
                        .into_iter()
                        .map(|range| MultiBufferOffset(range.start)..MultiBufferOffset(range.end)),
                )
            });

            editor
        });

        let diff_view = workspace
            .update_in(cx, |workspace, window, cx| {
                TextDiffView::open(
                    &DiffClipboardWithSelectionData {
                        clipboard_text: clipboard_text.to_string(),
                        editor,
                    },
                    workspace,
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();

        cx.executor().run_until_parked();

        assert_state_with_diff(
            &diff_view.read_with(cx, |diff_view, _| diff_view.diff_editor.clone()),
            cx,
            expected_diff,
        );

        diff_view.read_with(cx, |diff_view, cx| {
            assert_eq!(diff_view.tab_content_text(0, cx), expected_tab_title);
            assert_eq!(
                diff_view.tab_tooltip_text(cx).unwrap(),
                expected_tab_tooltip
            );
        });
    }
}
