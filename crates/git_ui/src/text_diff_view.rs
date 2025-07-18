//! TextDiffView provides a UI for displaying differences between two buffers.

use anyhow::Result;
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{Editor, EditorEvent, MultiBuffer, ToPoint, actions::DiffClipboardWithSelectionData};
use futures::{FutureExt, select_biased};
use gpui::{
    AnyElement, AnyView, App, AppContext as _, AsyncApp, Context, Entity, EventEmitter,
    FocusHandle, Focusable, IntoElement, Render, Task, Window,
};
use language::{self, Buffer, Point};
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
    Item, ItemHandle as _, ItemNavHistory, ToolbarItemLocation, Workspace,
    item::{BreadcrumbText, ItemEvent, SaveOptions, TabContentParams},
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

        let source_editor_buffer_and_range = source_editor.update(cx, |editor, cx| {
            let multibuffer = editor.buffer().read(cx);
            let buffer = multibuffer.as_singleton()?.clone();
            let selections = editor.selections.all::<Point>(cx);
            let buffer_snapshot = buffer.read(cx);
            let Some(first_selection) = selections.first() else {
                return None;
            };
            let selection_range = if first_selection.is_empty() {
                Point::new(0, 0)..buffer_snapshot.max_point()
            } else {
                first_selection.start..first_selection.end
            };

            Some((buffer, selection_range))
        });

        let Some((source_buffer, source_range)) = source_editor_buffer_and_range else {
            log::warn!("There should always be at least one selection in Zed. This is a bug.");
            return None;
        };

        let clipboard_buffer = cx.new(|cx| {
            let clipboard_text = diff_data.clipboard_text.clone();
            let mut buffer = language::Buffer::local(clipboard_text, cx);
            let source_language = source_buffer.read(cx).language().cloned();
            buffer.set_language(source_language, cx);
            buffer
        });

        let workspace = workspace.weak_handle();

        let task = window.spawn(cx, async move |cx| {
            let project = workspace.update(cx, |workspace, _| workspace.project().clone())?;

            let buffer_diff = build_range_based_diff(
                clipboard_buffer.clone(),
                source_buffer.clone(),
                source_range.clone(),
                cx,
            )
            .await?;

            workspace.update_in(cx, |workspace, window, cx| {
                let diff_view = cx.new(|cx| {
                    TextDiffView::new(
                        clipboard_buffer,
                        source_editor,
                        source_buffer,
                        source_range,
                        buffer_diff,
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
        source_editor: Entity<Editor>,
        source_buffer: Entity<Buffer>,
        source_range: Range<Point>,
        diff: Entity<BufferDiff>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(language::Capability::ReadWrite);

            multibuffer.push_excerpts(
                source_buffer.clone(),
                [editor::ExcerptRange::new(source_range)],
                cx,
            );

            multibuffer.add_diff(diff.clone(), cx);
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
            language::BufferEvent::Edited
            | language::BufferEvent::LanguageChanged
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
            .map(|b| {
                b.read(cx)
                    .file()
                    .map(|f| f.full_path(cx).compact().to_string_lossy().to_string())
            })
            .flatten()
            .unwrap_or("untitled".into());

        let selection_location_path = selection_location_text
            .map(|text| format!("{} @ {}", path, text))
            .unwrap_or(path);

        Self {
            diff_editor,
            title: format!("Clipboard ↔ {selection_location_title}").into(),
            path: Some(format!("Clipboard ↔ {selection_location_path}").into()),
            buffer_changes_tx,
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
                            clipboard_buffer.read(cx).snapshot(),
                            source_buffer.read(cx).snapshot(),
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

async fn build_range_based_diff(
    old_buffer: Entity<Buffer>,
    new_buffer: Entity<Buffer>,
    new_range: Range<Point>,
    cx: &mut AsyncApp,
) -> Result<Entity<BufferDiff>> {
    let old_text = old_buffer.read_with(cx, |buffer, _| buffer.text())?;

    let new_buffer_snapshot = new_buffer.read_with(cx, |buffer, _| buffer.snapshot())?;

    let base_buffer = cx.update(|cx| {
        cx.new(|cx| {
            let mut buffer = language::Buffer::local(new_buffer_snapshot.text().to_string(), cx);
            let language = new_buffer.read(cx).language().cloned();
            buffer.set_language(language, cx);

            let range_start = new_buffer_snapshot.point_to_offset(new_range.start);
            let range_end = new_buffer_snapshot.point_to_offset(new_range.end);
            buffer.edit([(range_start..range_end, old_text)], None, cx);

            buffer
        })
    })?;

    let base_buffer_snapshot = base_buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
    let base_text = base_buffer_snapshot.text().to_string();

    let diff_snapshot = cx
        .update(|cx| {
            BufferDiffSnapshot::new_with_base_buffer(
                new_buffer_snapshot.text.clone(),
                Some(Arc::new(base_text)),
                base_buffer_snapshot,
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

    fn to_item_events(event: &EditorEvent, f: impl FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Diff View Opened")
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.diff_editor
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
            Some(self.diff_editor.to_any())
        } else {
            None
        }
    }

    fn as_searchable(&self, _: &Entity<Self>) -> Option<Box<dyn SearchableItemHandle>> {
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
        data: Box<dyn Any>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.diff_editor
            .update(cx, |editor, cx| editor.navigate(data, window, cx))
    }

    fn breadcrumb_location(&self, _: &App) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft
    }

    fn breadcrumbs(&self, theme: &theme::Theme, cx: &App) -> Option<Vec<BreadcrumbText>> {
        self.diff_editor.breadcrumbs(theme, cx)
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
    let buffer = editor.buffer().read(cx).snapshot(cx);
    let Some(first_selection) = editor.selections.disjoint.first() else {
        return None;
    };

    let selection_start = first_selection.start.to_point(&buffer);
    let selection_end = first_selection.end.to_point(&buffer);

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
    use std::path::Path;

    use super::*;

    use editor::{actions, test::editor_test_context::assert_state_with_diff};
    use gpui::{TestAppContext, VisualContext};
    use language::{Language, LanguageConfig, LanguageMatcher, LanguageRegistry};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::{Settings, SettingsStore};
    use std::sync::Arc;
    use unindent::unindent;

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
    async fn test_clipboard_against_selection_text_diff_view(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/test",
            json!({
                "a": {
                    "b": {
                        "text.txt": "new line 1\nline 2\nnew line 3\nline 4"
                    }
                }
            }),
        )
        .await;

        let project = Project::test(fs, ["/test".as_ref()], cx).await;

        let (workspace, mut cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(Path::new("/test/a/b/text.txt"), cx)
            })
            .await
            .unwrap();

        let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
        // language_registry.add(Arc::new(Language::new(
        //     LanguageConfig {
        //         name: "Markdown".into(),
        //         matcher: LanguageMatcher {
        //             path_suffixes: vec!["md".into()],
        //             ..Default::default()
        //         },
        //         ..Default::default()
        //     },
        //     Some(tree_sitter_md::LANGUAGE.into()),
        // )));

        // let markdown = language_registry
        //     .language_for_name("Markdown")
        //     .await
        //     .unwrap();

        // buffer.update(cx, |buffer, cx| {
        //     buffer.set_language(Some(markdown.clone()), cx);
        // });

        let editor = cx.new_window_entity(|window, cx| {
            let mut editor = Editor::for_buffer(buffer, None, window, cx);
            editor.set_text("new line 1\nline 2\nnew line 3\nline 4\n", window, cx);
            editor.select_all(&actions::SelectAll, window, cx);
            editor
        });

        let diff_view = workspace
            .update_in(cx, |workspace, window, cx| {
                TextDiffView::open(
                    &DiffClipboardWithSelectionData {
                        clipboard_text: "old line 1\nline 2\nold line 3\nline 4\n".to_string(),
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

        diff_view.read_with(cx, |diff_view, cx| {
            assert_eq!(
                diff_view.tab_content_text(0, cx),
                "Clipboard ↔ text.txt @ L1:1-L5:1"
            );
            assert_eq!(
                diff_view.tab_tooltip_text(cx).unwrap(),
                "Clipboard ↔ test/a/b/text.txt @ L1:1-L5:1"
            );
        });

        // let diff_buffer = diff_view.read_with(cx, |view, _| view.diff_editor.clone());
        // let snapshot =
        //     diff_buffer.read_with(cx, |editor, cx| editor.buffer().read(cx).snapshot(cx));

        // let deletion_line = 0; // "- old line 1"
        // let deletion_point = language::Point::new(deletion_line, 2); // Skip "- " prefix
        // let language_at_deletion = snapshot.language_at(deletion_point);
        // assert_eq!(language_at_deletion, Some(&markdown.clone()),);

        // let addition_line = 1; // "+ new line 1"
        // let addition_point = language::Point::new(addition_line, 2); // Skip "+ " prefix
        // let language_at_addition = snapshot.language_at(addition_point);
        // assert_eq!(language_at_addition, Some(&markdown),);
    }
}

// TODO - diff - single line diffs should work, do we need to adjust indenting when not selecting the entire line?
// TODO - diff - adjusting highlight in original file should adjust what is shown in the diff view?
// TODO - diff - editing the source should keep the diff in tact, but it currently loses the diff when editing
// TODO - diff - language isn't being set in diff deletion hunks
