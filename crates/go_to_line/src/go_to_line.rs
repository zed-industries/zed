pub mod cursor_position;

use cursor_position::{LineIndicatorFormat, UserCaretPosition};
use editor::{
    Anchor, Editor, MultiBufferSnapshot, RowHighlightOptions, SelectionEffects, ToOffset, ToPoint,
    actions::Tab,
    scroll::{Autoscroll, ScrollOffset},
};
use gpui::{
    App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, SharedString, Styled,
    Subscription, div, prelude::*,
};
use language::Buffer;
use settings::Settings;
use text::{Bias, Point};
use theme::ActiveTheme;
use ui::prelude::*;
use util::paths::FILE_ROW_COLUMN_DELIMITER;
use workspace::ModalView;

pub fn init(cx: &mut App) {
    LineIndicatorFormat::register(cx);
    cx.observe_new(GoToLine::register).detach();
}

pub struct GoToLine {
    line_editor: Entity<Editor>,
    active_editor: Entity<Editor>,
    current_text: SharedString,
    prev_scroll_position: Option<gpui::Point<ScrollOffset>>,
    _subscriptions: Vec<Subscription>,
}

impl ModalView for GoToLine {}

impl Focusable for GoToLine {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.line_editor.focus_handle(cx)
    }
}
impl EventEmitter<DismissEvent> for GoToLine {}

enum GoToLineRowHighlights {}

impl GoToLine {
    fn register(editor: &mut Editor, _window: Option<&mut Window>, cx: &mut Context<Editor>) {
        let handle = cx.entity().downgrade();
        editor
            .register_action(move |_: &editor::actions::ToggleGoToLine, window, cx| {
                let Some(editor_handle) = handle.upgrade() else {
                    return;
                };
                let Some(workspace) = editor_handle.read(cx).workspace() else {
                    return;
                };
                let editor = editor_handle.read(cx);
                let Some((_, buffer, _)) = editor.active_excerpt(cx) else {
                    return;
                };
                workspace.update(cx, |workspace, cx| {
                    workspace.toggle_modal(window, cx, move |window, cx| {
                        GoToLine::new(editor_handle, buffer, window, cx)
                    });
                })
            })
            .detach();
    }

    pub fn new(
        active_editor: Entity<Editor>,
        active_buffer: Entity<Buffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let (user_caret, last_line, scroll_position) = active_editor.update(cx, |editor, cx| {
            let user_caret = UserCaretPosition::at_selection_end(
                &editor
                    .selections
                    .last::<Point>(&editor.display_snapshot(cx)),
                &editor.buffer().read(cx).snapshot(cx),
            );

            let snapshot = active_buffer.read(cx).snapshot();
            let last_line = editor
                .buffer()
                .read(cx)
                .excerpts_for_buffer(snapshot.remote_id(), cx)
                .into_iter()
                .map(move |(_, range)| text::ToPoint::to_point(&range.context.end, &snapshot).row)
                .max()
                .unwrap_or(0);

            (user_caret, last_line, editor.scroll_position(cx))
        });

        let line = user_caret.line.get();
        let column = user_caret.character.get();

        let line_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            let editor_handle = cx.entity().downgrade();
            editor
                .register_action::<Tab>({
                    move |_, window, cx| {
                        let Some(editor) = editor_handle.upgrade() else {
                            return;
                        };
                        editor.update(cx, |editor, cx| {
                            if let Some(placeholder_text) = editor.placeholder_text(cx)
                                && editor.text(cx).is_empty()
                            {
                                editor.set_text(placeholder_text, window, cx);
                            }
                        });
                    }
                })
                .detach();
            editor.set_placeholder_text(
                &format!("{line}{FILE_ROW_COLUMN_DELIMITER}{column}"),
                window,
                cx,
            );
            editor
        });
        let line_editor_change = cx.subscribe_in(&line_editor, window, Self::on_line_editor_event);

        let current_text = format!(
            "Current Line: {} of {} (column {})",
            line,
            last_line + 1,
            column
        );

        Self {
            line_editor,
            active_editor,
            current_text: current_text.into(),
            prev_scroll_position: Some(scroll_position),
            _subscriptions: vec![line_editor_change, cx.on_release_in(window, Self::release)],
        }
    }

    fn release(&mut self, window: &mut Window, cx: &mut App) {
        let scroll_position = self.prev_scroll_position.take();
        self.active_editor.update(cx, |editor, cx| {
            editor.clear_row_highlights::<GoToLineRowHighlights>();
            if let Some(scroll_position) = scroll_position {
                editor.set_scroll_position(scroll_position, window, cx);
            }
            cx.notify();
        })
    }

    fn on_line_editor_event(
        &mut self,
        _: &Entity<Editor>,
        event: &editor::EditorEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            editor::EditorEvent::Blurred => {
                self.prev_scroll_position.take();
                cx.emit(DismissEvent)
            }
            editor::EditorEvent::BufferEdited => self.highlight_current_line(cx),
            _ => {}
        }
    }

    fn highlight_current_line(&mut self, cx: &mut Context<Self>) {
        self.active_editor.update(cx, |editor, cx| {
            editor.clear_row_highlights::<GoToLineRowHighlights>();
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let Some(start) = self.anchor_from_query(&snapshot, cx) else {
                return;
            };
            let mut start_point = start.to_point(&snapshot);
            start_point.column = 0;
            // Force non-empty range to ensure the line is highlighted.
            let mut end_point = snapshot.clip_point(start_point + Point::new(0, 1), Bias::Left);
            if start_point == end_point {
                end_point = snapshot.clip_point(start_point + Point::new(1, 0), Bias::Left);
            }

            let end = snapshot.anchor_after(end_point);
            editor.highlight_rows::<GoToLineRowHighlights>(
                start..end,
                cx.theme().colors().editor_highlighted_line_background,
                RowHighlightOptions {
                    autoscroll: true,
                    ..Default::default()
                },
                cx,
            );
            editor.request_autoscroll(Autoscroll::center(), cx);
        });
        cx.notify();
    }

    fn anchor_from_query(
        &self,
        snapshot: &MultiBufferSnapshot,
        cx: &Context<Editor>,
    ) -> Option<Anchor> {
        let (query_row, query_char) = self.line_and_char_from_query(cx)?;
        let row = query_row.saturating_sub(1);
        let character = query_char.unwrap_or(0).saturating_sub(1);

        let start_offset = Point::new(row, 0).to_offset(snapshot);
        const MAX_BYTES_IN_UTF_8: u32 = 4;
        let max_end_offset = snapshot
            .clip_point(
                Point::new(row, character * MAX_BYTES_IN_UTF_8 + 1),
                Bias::Right,
            )
            .to_offset(snapshot);

        let mut chars_to_iterate = character;
        let mut end_offset = start_offset;
        'outer: for text_chunk in snapshot.text_for_range(start_offset..max_end_offset) {
            let mut offset_increment = 0;
            for c in text_chunk.chars() {
                if chars_to_iterate == 0 {
                    end_offset += offset_increment;
                    break 'outer;
                } else {
                    chars_to_iterate -= 1;
                    offset_increment += c.len_utf8();
                }
            }
            end_offset += offset_increment;
        }
        Some(snapshot.anchor_before(snapshot.clip_offset(end_offset, Bias::Left)))
    }

    fn line_and_char_from_query(&self, cx: &App) -> Option<(u32, Option<u32>)> {
        let input = self.line_editor.read(cx).text(cx);
        let mut components = input
            .splitn(2, FILE_ROW_COLUMN_DELIMITER)
            .map(str::trim)
            .fuse();
        let row = components.next().and_then(|row| row.parse::<u32>().ok())?;
        let column = components.next().and_then(|col| col.parse::<u32>().ok());
        Some((row, column))
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.active_editor.update(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let Some(start) = self.anchor_from_query(&snapshot, cx) else {
                return;
            };
            editor.change_selections(
                SelectionEffects::scroll(Autoscroll::center()),
                window,
                cx,
                |s| s.select_anchor_ranges([start..start]),
            );
            editor.focus_handle(cx).focus(window);
            cx.notify()
        });
        self.prev_scroll_position.take();

        cx.emit(DismissEvent);
    }
}

impl Render for GoToLine {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let help_text = match self.line_and_char_from_query(cx) {
            Some((line, Some(character))) => {
                format!("Go to line {line}, character {character}").into()
            }
            Some((line, None)) => format!("Go to line {line}").into(),
            None => self.current_text.clone(),
        };

        v_flex()
            .w(rems(24.))
            .elevation_2(cx)
            .key_context("GoToLine")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .child(
                div()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .px_2()
                    .py_1()
                    .child(self.line_editor.clone()),
            )
            .child(
                h_flex()
                    .px_2()
                    .py_1()
                    .gap_1()
                    .child(Label::new(help_text).color(Color::Muted)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cursor_position::{CursorPosition, SelectionStats, UserCaretPosition};
    use editor::actions::{MoveRight, MoveToBeginning, SelectAll};
    use gpui::{TestAppContext, VisualTestContext};
    use indoc::indoc;
    use project::{FakeFs, Project};
    use serde_json::json;
    use std::{num::NonZeroU32, sync::Arc, time::Duration};
    use util::{path, rel_path::rel_path};
    use workspace::{AppState, Workspace};

    #[gpui::test]
    async fn test_go_to_line_view_row_highlights(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "a.rs": indoc!{"
                    struct SingleLine; // display line 0
                                       // display line 1
                    struct MultiLine { // display line 2
                        field_1: i32,  // display line 3
                        field_2: i32,  // display line 4
                    }                  // display line 5
                                       // display line 6
                    struct Another {   // display line 7
                        field_1: i32,  // display line 8
                        field_2: i32,  // display line 9
                        field_3: i32,  // display line 10
                        field_4: i32,  // display line 11
                    }                  // display line 12
                "}
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let worktree_id = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        });
        let _buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/dir/a.rs"), cx)
            })
            .await
            .unwrap();
        let editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, rel_path("a.rs")), None, true, window, cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        let go_to_line_view = open_go_to_line_view(&workspace, cx);
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            Vec::<u32>::new(),
            "Initially opened go to line modal should not highlight any rows"
        );
        assert_single_caret_at_row(&editor, 0, cx);

        cx.simulate_input("1");
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            vec![0],
            "Go to line modal should highlight a row, corresponding to the query"
        );
        assert_single_caret_at_row(&editor, 0, cx);

        cx.simulate_input("8");
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            vec![13],
            "If the query is too large, the last row should be highlighted"
        );
        assert_single_caret_at_row(&editor, 0, cx);

        cx.dispatch_action(menu::Cancel);
        drop(go_to_line_view);
        editor.update(cx, |_, _| {});
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            Vec::<u32>::new(),
            "After cancelling and closing the modal, no rows should be highlighted"
        );
        assert_single_caret_at_row(&editor, 0, cx);

        let go_to_line_view = open_go_to_line_view(&workspace, cx);
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            Vec::<u32>::new(),
            "Reopened modal should not highlight any rows"
        );
        assert_single_caret_at_row(&editor, 0, cx);

        let expected_highlighted_row = 4;
        cx.simulate_input("5");
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            vec![expected_highlighted_row]
        );
        assert_single_caret_at_row(&editor, 0, cx);
        cx.dispatch_action(menu::Confirm);
        drop(go_to_line_view);
        editor.update(cx, |_, _| {});
        assert_eq!(
            highlighted_display_rows(&editor, cx),
            Vec::<u32>::new(),
            "After confirming and closing the modal, no rows should be highlighted"
        );
        // On confirm, should place the caret on the highlighted row.
        assert_single_caret_at_row(&editor, expected_highlighted_row, cx);
    }

    #[gpui::test]
    async fn test_unicode_characters_selection(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "a.rs": "ēlo"
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        workspace.update_in(cx, |workspace, window, cx| {
            let cursor_position = cx.new(|_| CursorPosition::new(workspace));
            workspace.status_bar().update(cx, |status_bar, cx| {
                status_bar.add_right_item(cursor_position, window, cx);
            });
        });

        let worktree_id = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        });
        let _buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/dir/a.rs"), cx)
            })
            .await
            .unwrap();
        let editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, rel_path("a.rs")), None, true, window, cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        cx.executor().advance_clock(Duration::from_millis(200));
        workspace.update(cx, |workspace, cx| {
            assert_eq!(
                &SelectionStats {
                    lines: 0,
                    characters: 0,
                    selections: 1,
                },
                workspace
                    .status_bar()
                    .read(cx)
                    .item_of_type::<CursorPosition>()
                    .expect("missing cursor position item")
                    .read(cx)
                    .selection_stats(),
                "No selections should be initially"
            );
        });
        editor.update_in(cx, |editor, window, cx| {
            editor.select_all(&SelectAll, window, cx)
        });
        cx.executor().advance_clock(Duration::from_millis(200));
        workspace.update(cx, |workspace, cx| {
            assert_eq!(
                &SelectionStats {
                    lines: 1,
                    characters: 3,
                    selections: 1,
                },
                workspace
                    .status_bar()
                    .read(cx)
                    .item_of_type::<CursorPosition>()
                    .expect("missing cursor position item")
                    .read(cx)
                    .selection_stats(),
                "After selecting a text with multibyte unicode characters, the character count should be correct"
            );
        });
    }

    #[gpui::test]
    async fn test_unicode_line_numbers(cx: &mut TestAppContext) {
        init_test(cx);

        let text = "ēlo你好";
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "a.rs": text
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        workspace.update_in(cx, |workspace, window, cx| {
            let cursor_position = cx.new(|_| CursorPosition::new(workspace));
            workspace.status_bar().update(cx, |status_bar, cx| {
                status_bar.add_right_item(cursor_position, window, cx);
            });
        });

        let worktree_id = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        });
        let _buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/dir/a.rs"), cx)
            })
            .await
            .unwrap();
        let editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, rel_path("a.rs")), None, true, window, cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        editor.update_in(cx, |editor, window, cx| {
            editor.move_to_beginning(&MoveToBeginning, window, cx)
        });
        cx.executor().advance_clock(Duration::from_millis(200));
        assert_eq!(
            user_caret_position(1, 1),
            current_position(&workspace, cx),
            "Beginning of the line should be at first line, before any characters"
        );

        for (i, c) in text.chars().enumerate() {
            let i = i as u32 + 1;
            editor.update_in(cx, |editor, window, cx| {
                editor.move_right(&MoveRight, window, cx)
            });
            cx.executor().advance_clock(Duration::from_millis(200));
            assert_eq!(
                user_caret_position(1, i + 1),
                current_position(&workspace, cx),
                "Wrong position for char '{c}' in string '{text}'",
            );
        }

        editor.update_in(cx, |editor, window, cx| {
            editor.move_right(&MoveRight, window, cx)
        });
        cx.executor().advance_clock(Duration::from_millis(200));
        assert_eq!(
            user_caret_position(1, text.chars().count() as u32 + 1),
            current_position(&workspace, cx),
            "After reaching the end of the text, position should not change when moving right"
        );
    }

    #[gpui::test]
    async fn test_go_into_unicode(cx: &mut TestAppContext) {
        init_test(cx);

        let text = "ēlo你好";
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "a.rs": text
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        workspace.update_in(cx, |workspace, window, cx| {
            let cursor_position = cx.new(|_| CursorPosition::new(workspace));
            workspace.status_bar().update(cx, |status_bar, cx| {
                status_bar.add_right_item(cursor_position, window, cx);
            });
        });

        let worktree_id = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        });
        let _buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/dir/a.rs"), cx)
            })
            .await
            .unwrap();
        let editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, rel_path("a.rs")), None, true, window, cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        editor.update_in(cx, |editor, window, cx| {
            editor.move_to_beginning(&MoveToBeginning, window, cx)
        });
        cx.executor().advance_clock(Duration::from_millis(200));
        assert_eq!(user_caret_position(1, 1), current_position(&workspace, cx));

        for (i, c) in text.chars().enumerate() {
            let i = i as u32 + 1;
            let point = user_caret_position(1, i + 1);
            go_to_point(point, user_caret_position(1, i), &workspace, cx);
            cx.executor().advance_clock(Duration::from_millis(200));
            assert_eq!(
                point,
                current_position(&workspace, cx),
                "When going to {point:?}, expecting the cursor to be at char '{c}' in string '{text}'",
            );
        }

        go_to_point(
            user_caret_position(111, 222),
            user_caret_position(1, text.chars().count() as u32 + 1),
            &workspace,
            cx,
        );
        cx.executor().advance_clock(Duration::from_millis(200));
        assert_eq!(
            user_caret_position(1, text.chars().count() as u32 + 1),
            current_position(&workspace, cx),
            "When going into too large point, should go to the end of the text"
        );
    }

    fn current_position(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> UserCaretPosition {
        workspace.update(cx, |workspace, cx| {
            workspace
                .status_bar()
                .read(cx)
                .item_of_type::<CursorPosition>()
                .expect("missing cursor position item")
                .read(cx)
                .position()
                .expect("No position found")
        })
    }

    fn user_caret_position(line: u32, character: u32) -> UserCaretPosition {
        UserCaretPosition {
            line: NonZeroU32::new(line).unwrap(),
            character: NonZeroU32::new(character).unwrap(),
        }
    }

    fn go_to_point(
        new_point: UserCaretPosition,
        expected_placeholder: UserCaretPosition,
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) {
        let go_to_line_view = open_go_to_line_view(workspace, cx);
        go_to_line_view.update(cx, |go_to_line_view, cx| {
            assert_eq!(
                go_to_line_view.line_editor.update(cx, |line_editor, cx| {
                    line_editor
                        .placeholder_text(cx)
                        .expect("No placeholder text")
                }),
                format!(
                    "{}:{}",
                    expected_placeholder.line, expected_placeholder.character
                )
            );
        });
        cx.simulate_input(&format!("{}:{}", new_point.line, new_point.character));
        cx.dispatch_action(menu::Confirm);
    }

    fn open_go_to_line_view(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<GoToLine> {
        cx.dispatch_action(editor::actions::ToggleGoToLine);
        workspace.update(cx, |workspace, cx| {
            workspace.active_modal::<GoToLine>(cx).unwrap()
        })
    }

    fn highlighted_display_rows(editor: &Entity<Editor>, cx: &mut VisualTestContext) -> Vec<u32> {
        editor.update_in(cx, |editor, window, cx| {
            editor
                .highlighted_display_rows(window, cx)
                .into_keys()
                .map(|r| r.0)
                .collect()
        })
    }

    #[track_caller]
    fn assert_single_caret_at_row(
        editor: &Entity<Editor>,
        buffer_row: u32,
        cx: &mut VisualTestContext,
    ) {
        let selections = editor.update(cx, |editor, cx| {
            editor
                .selections
                .all::<rope::Point>(&editor.display_snapshot(cx))
                .into_iter()
                .map(|s| s.start..s.end)
                .collect::<Vec<_>>()
        });
        assert!(
            selections.len() == 1,
            "Expected one caret selection but got: {selections:?}"
        );
        let selection = &selections[0];
        assert!(
            selection.start == selection.end,
            "Expected a single caret selection, but got: {selection:?}"
        );
        assert_eq!(selection.start.row, buffer_row);
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let state = AppState::test(cx);
            language::init(cx);
            crate::init(cx);
            editor::init(cx);
            workspace::init_settings(cx);
            Project::init_settings(cx);
            state
        })
    }
}
