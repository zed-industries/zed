use editor::display_map::ToDisplayPoint;
use editor::{DisplayPoint, Editor, movement, scroll::Autoscroll};
use gpui::{Action, actions, impl_actions};
use gpui::{Context, Window};
use language::{CharClassifier, CharKind};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::motion::MotionKind;
use crate::{Vim, motion::Motion, state::Mode};

actions!(vim, [HelixNormalAfter, HelixDelete, HelixYank]);

#[derive(Clone, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct HelixPaste {
    #[serde(default)]
    before: bool,
    #[serde(default)]
    preserve_clipboard: bool,
}

impl_actions!(vim, [HelixPaste]);

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, Vim::helix_normal_after);
    Vim::action(editor, cx, Vim::helix_delete);
    Vim::action(editor, cx, Vim::helix_yank);
    Vim::action(editor, cx, Vim::helix_paste);
}

impl Vim {
    pub fn helix_normal_after(
        &mut self,
        action: &HelixNormalAfter,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.active_operator().is_some() {
            self.operator_stack.clear();
            self.sync_vim_settings(window, cx);
            return;
        }
        self.stop_recording_immediately(action.boxed_clone(), cx);
        self.switch_mode(Mode::HelixNormal, false, window, cx);
        return;
    }

    pub fn helix_normal_motion(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.helix_move_cursor(motion, times, window, cx);
    }

    fn helix_find_range_forward(
        &mut self,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
        mut is_boundary: impl FnMut(char, char, &CharClassifier) -> bool,
    ) {
        self.update_editor(window, cx, |_, editor, window, cx| {
            editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                s.move_with(|map, selection| {
                    let times = times.unwrap_or(1);

                    if selection.head() == map.max_point() {
                        return;
                    }

                    // collapse to block cursor
                    if selection.tail() < selection.head() {
                        selection.set_tail(movement::left(map, selection.head()), selection.goal);
                    } else {
                        selection.set_tail(selection.head(), selection.goal);
                        selection.set_head(movement::right(map, selection.head()), selection.goal);
                    }

                    // create a classifier
                    let classifier = map
                        .buffer_snapshot
                        .char_classifier_at(selection.head().to_point(map));

                    let mut last_selection = selection.clone();
                    for _ in 0..times {
                        let (new_tail, new_head) =
                            movement::find_boundary_trail(map, selection.head(), |left, right| {
                                is_boundary(left, right, &classifier)
                            });

                        selection.set_head(new_head, selection.goal);
                        if let Some(new_tail) = new_tail {
                            selection.set_tail(new_tail, selection.goal);
                        }

                        if selection.head() == last_selection.head()
                            && selection.tail() == last_selection.tail()
                        {
                            break;
                        }
                        last_selection = selection.clone();
                    }
                });
            });
        });
    }

    fn helix_find_range_backward(
        &mut self,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
        mut is_boundary: impl FnMut(char, char, &CharClassifier) -> bool,
    ) {
        self.update_editor(window, cx, |_, editor, window, cx| {
            editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                s.move_with(|map, selection| {
                    let times = times.unwrap_or(1);

                    if selection.head() == DisplayPoint::zero() {
                        return;
                    }

                    // collapse to block cursor
                    if selection.tail() < selection.head() {
                        selection.set_tail(movement::left(map, selection.head()), selection.goal);
                    } else {
                        selection.set_tail(selection.head(), selection.goal);
                        selection.set_head(movement::right(map, selection.head()), selection.goal);
                    }

                    // flip the selection
                    selection.swap_head_tail();

                    // create a classifier
                    let classifier = map
                        .buffer_snapshot
                        .char_classifier_at(selection.head().to_point(map));

                    let mut last_selection = selection.clone();
                    for _ in 0..times {
                        let (new_tail, new_head) = movement::find_preceding_boundary_trail(
                            map,
                            selection.head(),
                            |left, right| is_boundary(left, right, &classifier),
                        );

                        selection.set_head(new_head, selection.goal);
                        if let Some(new_tail) = new_tail {
                            selection.set_tail(new_tail, selection.goal);
                        }

                        if selection.head() == last_selection.head()
                            && selection.tail() == last_selection.tail()
                        {
                            break;
                        }
                        last_selection = selection.clone();
                    }
                });
            })
        });
    }

    pub fn helix_move_and_collapse(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_editor(window, cx, |_, editor, window, cx| {
            let text_layout_details = editor.text_layout_details(window);
            editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                s.move_with(|map, selection| {
                    let goal = selection.goal;
                    let cursor = if selection.is_empty() || selection.reversed {
                        selection.head()
                    } else {
                        movement::left(map, selection.head())
                    };

                    let (point, goal) = motion
                        .move_point(map, cursor, selection.goal, times, &text_layout_details)
                        .unwrap_or((cursor, goal));

                    selection.collapse_to(point, goal)
                })
            });
        });
    }

    pub fn helix_move_cursor(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match motion {
            Motion::NextWordStart { ignore_punctuation } => {
                self.helix_find_range_forward(times, window, cx, |left, right, classifier| {
                    let left_kind = classifier.kind_with(left, ignore_punctuation);
                    let right_kind = classifier.kind_with(right, ignore_punctuation);
                    let at_newline = right == '\n';

                    let found =
                        left_kind != right_kind && right_kind != CharKind::Whitespace || at_newline;

                    found
                })
            }
            Motion::NextWordEnd { ignore_punctuation } => {
                self.helix_find_range_forward(times, window, cx, |left, right, classifier| {
                    let left_kind = classifier.kind_with(left, ignore_punctuation);
                    let right_kind = classifier.kind_with(right, ignore_punctuation);
                    let at_newline = right == '\n';

                    let found = left_kind != right_kind
                        && (left_kind != CharKind::Whitespace || at_newline);

                    found
                })
            }
            Motion::PreviousWordStart { ignore_punctuation } => {
                self.helix_find_range_backward(times, window, cx, |left, right, classifier| {
                    let left_kind = classifier.kind_with(left, ignore_punctuation);
                    let right_kind = classifier.kind_with(right, ignore_punctuation);
                    let at_newline = right == '\n';

                    let found = left_kind != right_kind
                        && (left_kind != CharKind::Whitespace || at_newline);

                    found
                })
            }
            Motion::PreviousWordEnd { ignore_punctuation } => {
                self.helix_find_range_backward(times, window, cx, |left, right, classifier| {
                    let left_kind = classifier.kind_with(left, ignore_punctuation);
                    let right_kind = classifier.kind_with(right, ignore_punctuation);
                    let at_newline = right == '\n';

                    let found = left_kind != right_kind
                        && right_kind != CharKind::Whitespace
                        && !at_newline;

                    found
                })
            }
            Motion::CurrentLine => {
                self.update_editor(window, cx, |vim, editor, window, cx| {
                    let text_layout_details = editor.text_layout_details(window);
                    editor.transact(window, cx, |editor, window, cx| {
                        editor.set_clip_at_line_ends(false, cx);
                        editor.change_selections(None, window, cx, |s| {
                            s.move_with(|map, selection| {
                                motion.expand_selection(
                                    map,
                                    selection,
                                    times,
                                    &text_layout_details,
                                );
                            })
                        });
                        editor.selections.line_mode = true;
                    });
                });
            }
            _ => self.helix_move_and_collapse(motion, times, window, cx),
        }
    }

    pub fn helix_delete(&mut self, _: &HelixDelete, window: &mut Window, cx: &mut Context<Self>) {
        self.store_visual_marks(window, cx);
        self.update_editor(window, cx, |vim, editor, window, cx| {
            fixup_selections(editor, window, cx);
            vim.copy_selections_content(editor, MotionKind::Exclusive, window, cx);
            editor.insert("", window, cx);
        });
    }

    pub fn helix_yank(&mut self, _: &HelixYank, window: &mut Window, cx: &mut Context<Self>) {
        self.store_visual_marks(window, cx);
        self.update_editor(window, cx, |vim, editor, window, cx| {
            fixup_selections(editor, window, cx);
            let motion_kind = if editor.selections.line_mode {
                MotionKind::Linewise
            } else {
                MotionKind::Exclusive
            };
            vim.copy_ranges(
                editor,
                motion_kind,
                true,
                editor
                    .selections
                    .all_adjusted(cx)
                    .iter()
                    .map(|s| s.range())
                    .collect(),
                window,
                cx,
            );
        });
    }

    pub fn helix_paste(
        &mut self,
        action: &HelixPaste,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.mode != Mode::HelixNormal {
            return;
        }
        self.record_current_action(cx);
        self.store_visual_marks(window, cx);
        let count = Vim::take_count(cx).unwrap_or(1);

        self.update_editor(window, cx, |vim, editor, window, cx| {
            let text_layout_details = editor.text_layout_details(window);
            _ = editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);

                let selected_register = vim.selected_register.take();

                let Some(crate::state::Register {
                    text,
                    clipboard_selections,
                }) = Vim::update_globals(cx, |globals, cx| {
                    globals.read_register(selected_register, Some(editor), cx)
                })
                .filter(|reg| !reg.text.is_empty())
                else {
                    return;
                };
                let clipboard_selections = clipboard_selections
                    .filter(|sel| sel.len() > 1 && vim.mode != Mode::VisualLine);

                if !action.preserve_clipboard && vim.mode.is_visual() {
                    vim.copy_selections_content(editor, MotionKind::for_mode(vim.mode), window, cx);
                }

                let (display_map, current_selections) = editor.selections.all_adjusted_display(cx);

                // unlike zed, if you have a multi-cursor selection from vim block mode,
                // pasting it will paste it on subsequent lines, even if you don't yet
                // have a cursor there.
                let mut selections_to_process = Vec::new();
                let mut i = 0;
                while i < current_selections.len() {
                    selections_to_process
                        .push((current_selections[i].start..current_selections[i].end, true));
                    i += 1;
                }
                if let Some(clipboard_selections) = clipboard_selections.as_ref() {
                    let left = current_selections
                        .iter()
                        .map(|selection| {
                            std::cmp::min(selection.start.column(), selection.end.column())
                        })
                        .min()
                        .unwrap();
                    let mut row =
                        editor::RowExt::next_row(&current_selections.last().unwrap().end.row());
                    while i < clipboard_selections.len() {
                        let cursor =
                            display_map.clip_point(DisplayPoint::new(row, left), text::Bias::Left);
                        selections_to_process.push((cursor..cursor, false));
                        i += 1;
                        row.0 += 1;
                    }
                }

                let first_selection_indent_column =
                    clipboard_selections.as_ref().and_then(|zed_selections| {
                        zed_selections
                            .first()
                            .map(|selection| selection.first_line_indent)});
                let before = action.before || vim.mode == Mode::VisualLine;

                let mut edits = Vec::new();
                let mut new_selections = Vec::new();
                let mut original_indent_columns = Vec::new();
                let mut start_offset = 0;

                for (ix, (selection, preserve)) in selections_to_process.iter().enumerate() {
                    let (mut to_insert, original_indent_column) =
                        if let Some(clipboard_selections) = &clipboard_selections {
                            if let Some(clipboard_selection) = clipboard_selections.get(ix) {
                                let end_offset = start_offset + clipboard_selection.len;
                                let text = text[start_offset..end_offset].to_string();
                                start_offset = end_offset + 1;
                                dbg!((text, Some(clipboard_selection.first_line_indent)))
                            } else {
                                dbg!(("".to_string(), first_selection_indent_column))
                            }
                        } else {
                            dbg!((text.to_string(), first_selection_indent_column))
                        };
                    let line_mode = to_insert.ends_with('\n');

                    if line_mode && !before {
                        if selection.is_empty() {
                            to_insert =
                                "\n".to_owned() + &to_insert[..to_insert.len() - "\n".len()];
                        } else {
                            to_insert = "\n".to_owned() + &to_insert;
                        }
                    } else if line_mode && vim.mode == Mode::VisualLine {
                        to_insert.pop();
                    }

                    let display_range = if line_mode {
                        let point = if before {
                            movement::line_beginning(&display_map, selection.start, false)
                        } else {
                            movement::line_end(&display_map, selection.end, false)
                        };
                        point..point
                    } else {
                        let point = if before {
                            selection.start
                        } else {
                            movement::saturating_right(&display_map, selection.end)
                        };
                        point..point
                    };

                    let point_range = display_range.start.to_point(&display_map)
                        ..display_range.end.to_point(&display_map);

                    let selection_beg =
                        display_map.buffer_snapshot.anchor_before(point_range.start);
                    let selection_end = display_map.buffer_snapshot.anchor_after(point_range.end);
                    edits.push((point_range, to_insert.repeat(count)));
                    new_selections.push((selection_beg, selection_end));
                    original_indent_columns.push(original_indent_column);
                }

                let cursor_offset = editor.selections.last::<usize>(cx).head();
                if editor
                    .buffer()
                    .read(cx)
                    .snapshot(cx)
                    .language_settings_at(cursor_offset, cx)
                    .auto_indent_on_paste
                {
                    editor.edit_with_block_indent(edits, original_indent_columns, cx);
                } else {
                    editor.edit(edits, cx);
                }

                // in line_mode vim will insert the new text on the next (or previous if before) line
                // and put the cursor on the first non-blank character of the first inserted line (or at the end if the first line is blank).
                // otherwise vim will insert the next text at (or before) the current cursor position,
                // the cursor will go to the last (or first, if is_multiline) inserted character.
                editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                    let display_map = s.display_map();
                    s.select_display_ranges(new_selections.into_iter().map(|(s, e)| {
                        s.to_display_point(&display_map)..e.to_display_point(&display_map)
                    }));
                })
            });
        });
    }
}

/// Fixup selections so they have helix's semantics.
/// Specifically:
///  - Make sure that each cursor acts as a 1 character wide selection
fn fixup_selections(editor: &mut Editor, window: &mut Window, cx: &mut Context<Editor>) {
    editor.transact(window, cx, |editor, window, cx| {
        editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
            s.move_with(|map, selection| {
                if selection.is_empty() && !selection.reversed {
                    selection.end = movement::right(map, selection.end);
                }
            });
        });
    });
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{state::Mode, test::VimTestContext};

    #[gpui::test]
    async fn test_next_word_start(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        // «
        // ˇ
        // »
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("w");

        cx.assert_state(
            indoc! {"
            The qu«ick ˇ»brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("w");

        cx.assert_state(
            indoc! {"
            The quick «brownˇ»
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_delete(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // test delete a selection
        cx.set_state(
            indoc! {"
            The qu«ick ˇ»brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("d");

        cx.assert_state(
            indoc! {"
            The quˇbrown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        // test deleting a single character
        cx.simulate_keystrokes("d");

        cx.assert_state(
            indoc! {"
            The quˇrown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_delete_character_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state(
            indoc! {"
            The quick brownˇ
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("d");

        cx.assert_state(
            indoc! {"
            The quick brownˇfox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_delete_character_end_of_buffer(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state(
            indoc! {"
            The quick brown
            fox jumps over
            the lazy dog.ˇ"},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("d");

        cx.assert_state(
            indoc! {"
            The quick brown
            fox jumps over
            the lazy dog.ˇ"},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_yank_and_paste(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state(
            indoc! {"
            The quick brown
            fox jumps over
            the lazy dog.ˇ"},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("x");

        cx.assert_state(
            indoc! {"
            The quick brown
            fox jumps over
            «the lazy dog.ˇ»"},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("y");

        cx.assert_state(
            indoc! {"
            The quick brown
            fox jumps over
            «the lazy dog.ˇ»"},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("p");

        cx.assert_state(
            indoc! {"
            The quick brown
            fox jumps over
            the lazy dog.
            «the lazy dog.ˇ»"},
            Mode::HelixNormal,
        );
    }
}
