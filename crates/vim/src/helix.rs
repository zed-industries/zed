mod boundary;
mod object;
mod select;

use editor::display_map::DisplaySnapshot;
use editor::{
    DisplayPoint, Editor, HideMouseCursorOrigin, SelectionEffects, ToOffset, ToPoint, movement,
};
use gpui::actions;
use gpui::{Context, Window};
use language::{CharClassifier, CharKind, Point};
use text::{Bias, SelectionGoal};

use crate::motion;
use crate::{
    Vim,
    motion::{Motion, right},
    state::Mode,
};

actions!(
    vim,
    [
        /// Yanks the current selection or character if no selection.
        HelixYank,
        /// Inserts at the beginning of the selection.
        HelixInsert,
        /// Appends at the end of the selection.
        HelixAppend,
        /// Goes to the location of the last modification.
        HelixGotoLastModification,
        /// Select entire line or multiple lines, extending downwards.
        HelixSelectLine,
    ]
);

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, Vim::helix_select_lines);
    Vim::action(editor, cx, Vim::helix_insert);
    Vim::action(editor, cx, Vim::helix_append);
    Vim::action(editor, cx, Vim::helix_yank);
    Vim::action(editor, cx, Vim::helix_goto_last_modification);
}

impl Vim {
    pub fn helix_normal_motion(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.helix_move_cursor(motion, times, window, cx);
    }

    pub fn helix_select_motion(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_editor(cx, |_, editor, cx| {
            let text_layout_details = editor.text_layout_details(window);
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(|map, selection| {
                    let current_head = selection.head();

                    let Some((new_head, goal)) = motion.move_point(
                        map,
                        current_head,
                        selection.goal,
                        times,
                        &text_layout_details,
                    ) else {
                        return;
                    };

                    selection.set_head(new_head, goal);
                })
            });
        });
    }

    /// Updates all selections based on where the cursors are.
    fn helix_new_selections(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        mut change: impl FnMut(
            // the start of the cursor
            DisplayPoint,
            &DisplaySnapshot,
        ) -> Option<(DisplayPoint, DisplayPoint)>,
    ) {
        self.update_editor(cx, |_, editor, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(|map, selection| {
                    let cursor_start = if selection.reversed || selection.is_empty() {
                        selection.head()
                    } else {
                        movement::left(map, selection.head())
                    };
                    let Some((head, tail)) = change(cursor_start, map) else {
                        return;
                    };

                    selection.set_head_tail(head, tail, SelectionGoal::None);
                });
            });
        });
    }

    fn helix_find_range_forward(
        &mut self,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
        mut is_boundary: impl FnMut(char, char, &CharClassifier) -> bool,
    ) {
        let times = times.unwrap_or(1);
        self.helix_new_selections(window, cx, |cursor, map| {
            let mut head = movement::right(map, cursor);
            let mut tail = cursor;
            let classifier = map.buffer_snapshot.char_classifier_at(head.to_point(map));
            if head == map.max_point() {
                return None;
            }
            for _ in 0..times {
                let (maybe_next_tail, next_head) =
                    movement::find_boundary_trail(map, head, |left, right| {
                        is_boundary(left, right, &classifier)
                    });

                if next_head == head && maybe_next_tail.unwrap_or(next_head) == tail {
                    break;
                }

                head = next_head;
                if let Some(next_tail) = maybe_next_tail {
                    tail = next_tail;
                }
            }
            Some((head, tail))
        });
    }

    fn helix_find_range_backward(
        &mut self,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
        mut is_boundary: impl FnMut(char, char, &CharClassifier) -> bool,
    ) {
        let times = times.unwrap_or(1);
        self.helix_new_selections(window, cx, |cursor, map| {
            let mut head = cursor;
            // The original cursor was one character wide,
            // but the search starts from the left side of it,
            // so to include that space the selection must end one character to the right.
            let mut tail = movement::right(map, cursor);
            let classifier = map.buffer_snapshot.char_classifier_at(head.to_point(map));
            if head == DisplayPoint::zero() {
                return None;
            }
            for _ in 0..times {
                let (maybe_next_tail, next_head) =
                    movement::find_preceding_boundary_trail(map, head, |left, right| {
                        is_boundary(left, right, &classifier)
                    });

                if next_head == head && maybe_next_tail.unwrap_or(next_head) == tail {
                    break;
                }

                head = next_head;
                if let Some(next_tail) = maybe_next_tail {
                    tail = next_tail;
                }
            }
            Some((head, tail))
        });
    }

    pub fn helix_move_and_collapse(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_editor(cx, |_, editor, cx| {
            let text_layout_details = editor.text_layout_details(window);
            editor.change_selections(Default::default(), window, cx, |s| {
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
                    let at_newline = (left == '\n') ^ (right == '\n');

                    (left_kind != right_kind && right_kind != CharKind::Whitespace) || at_newline
                })
            }
            Motion::NextWordEnd { ignore_punctuation } => {
                self.helix_find_range_forward(times, window, cx, |left, right, classifier| {
                    let left_kind = classifier.kind_with(left, ignore_punctuation);
                    let right_kind = classifier.kind_with(right, ignore_punctuation);
                    let at_newline = (left == '\n') ^ (right == '\n');

                    (left_kind != right_kind && left_kind != CharKind::Whitespace) || at_newline
                })
            }
            Motion::PreviousWordStart { ignore_punctuation } => {
                self.helix_find_range_backward(times, window, cx, |left, right, classifier| {
                    let left_kind = classifier.kind_with(left, ignore_punctuation);
                    let right_kind = classifier.kind_with(right, ignore_punctuation);
                    let at_newline = (left == '\n') ^ (right == '\n');

                    (left_kind != right_kind && left_kind != CharKind::Whitespace) || at_newline
                })
            }
            Motion::PreviousWordEnd { ignore_punctuation } => {
                self.helix_find_range_backward(times, window, cx, |left, right, classifier| {
                    let left_kind = classifier.kind_with(left, ignore_punctuation);
                    let right_kind = classifier.kind_with(right, ignore_punctuation);
                    let at_newline = (left == '\n') ^ (right == '\n');

                    (left_kind != right_kind && right_kind != CharKind::Whitespace) || at_newline
                })
            }
            Motion::FindForward {
                before,
                char,
                mode,
                smartcase,
            } => {
                self.helix_new_selections(window, cx, |cursor, map| {
                    let start = cursor;
                    let mut last_boundary = start;
                    for _ in 0..times.unwrap_or(1) {
                        last_boundary = movement::find_boundary(
                            map,
                            movement::right(map, last_boundary),
                            mode,
                            |left, right| {
                                let current_char = if before { right } else { left };
                                motion::is_character_match(char, current_char, smartcase)
                            },
                        );
                    }
                    Some((last_boundary, start))
                });
            }
            Motion::FindBackward {
                after,
                char,
                mode,
                smartcase,
            } => {
                self.helix_new_selections(window, cx, |cursor, map| {
                    let start = cursor;
                    let mut last_boundary = start;
                    for _ in 0..times.unwrap_or(1) {
                        last_boundary = movement::find_preceding_boundary_display_point(
                            map,
                            last_boundary,
                            mode,
                            |left, right| {
                                let current_char = if after { left } else { right };
                                motion::is_character_match(char, current_char, smartcase)
                            },
                        );
                    }
                    // The original cursor was one character wide,
                    // but the search started from the left side of it,
                    // so to include that space the selection must end one character to the right.
                    Some((last_boundary, movement::right(map, start)))
                });
            }
            _ => self.helix_move_and_collapse(motion, times, window, cx),
        }
    }

    pub fn helix_yank(&mut self, _: &HelixYank, window: &mut Window, cx: &mut Context<Self>) {
        self.update_editor(cx, |vim, editor, cx| {
            let has_selection = editor
                .selections
                .all_adjusted(cx)
                .iter()
                .any(|selection| !selection.is_empty());

            if !has_selection {
                // If no selection, expand to current character (like 'v' does)
                editor.change_selections(Default::default(), window, cx, |s| {
                    s.move_with(|map, selection| {
                        let head = selection.head();
                        let new_head = movement::saturating_right(map, head);
                        selection.set_tail(head, SelectionGoal::None);
                        selection.set_head(new_head, SelectionGoal::None);
                    });
                });
                vim.yank_selections_content(
                    editor,
                    crate::motion::MotionKind::Exclusive,
                    window,
                    cx,
                );
                editor.change_selections(Default::default(), window, cx, |s| {
                    s.move_with(|_map, selection| {
                        selection.collapse_to(selection.start, SelectionGoal::None);
                    });
                });
            } else {
                // Yank the selection(s)
                vim.yank_selections_content(
                    editor,
                    crate::motion::MotionKind::Exclusive,
                    window,
                    cx,
                );
            }
        });

        // Drop back to normal mode after yanking
        self.switch_mode(Mode::HelixNormal, true, window, cx);
    }

    fn helix_insert(&mut self, _: &HelixInsert, window: &mut Window, cx: &mut Context<Self>) {
        self.start_recording(cx);
        self.update_editor(cx, |_, editor, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(|_map, selection| {
                    // In helix normal mode, move cursor to start of selection and collapse
                    if !selection.is_empty() {
                        selection.collapse_to(selection.start, SelectionGoal::None);
                    }
                });
            });
        });
        self.switch_mode(Mode::Insert, false, window, cx);
    }

    fn helix_append(&mut self, _: &HelixAppend, window: &mut Window, cx: &mut Context<Self>) {
        self.start_recording(cx);
        self.switch_mode(Mode::Insert, false, window, cx);
        self.update_editor(cx, |_, editor, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(|map, selection| {
                    let point = if selection.is_empty() {
                        right(map, selection.head(), 1)
                    } else {
                        selection.end
                    };
                    selection.collapse_to(point, SelectionGoal::None);
                });
            });
        });
    }

    pub fn helix_replace(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.update_editor(cx, |_, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                let (map, selections) = editor.selections.all_display(cx);

                // Store selection info for positioning after edit
                let selection_info: Vec<_> = selections
                    .iter()
                    .map(|selection| {
                        let range = selection.range();
                        let start_offset = range.start.to_offset(&map, Bias::Left);
                        let end_offset = range.end.to_offset(&map, Bias::Left);
                        let was_empty = range.is_empty();
                        let was_reversed = selection.reversed;
                        (
                            map.buffer_snapshot.anchor_at(start_offset, Bias::Left),
                            end_offset - start_offset,
                            was_empty,
                            was_reversed,
                        )
                    })
                    .collect();

                let mut edits = Vec::new();
                for selection in &selections {
                    let mut range = selection.range();

                    // For empty selections, extend to replace one character
                    if range.is_empty() {
                        range.end = movement::saturating_right(&map, range.start);
                    }

                    let byte_range = range.start.to_offset(&map, Bias::Left)
                        ..range.end.to_offset(&map, Bias::Left);

                    if !byte_range.is_empty() {
                        let replacement_text = text.repeat(byte_range.len());
                        edits.push((byte_range, replacement_text));
                    }
                }

                editor.edit(edits, cx);

                // Restore selections based on original info
                let snapshot = editor.buffer().read(cx).snapshot(cx);
                let ranges: Vec<_> = selection_info
                    .into_iter()
                    .map(|(start_anchor, original_len, was_empty, was_reversed)| {
                        let start_point = start_anchor.to_point(&snapshot);
                        if was_empty {
                            // For cursor-only, collapse to start
                            start_point..start_point
                        } else {
                            // For selections, span the replaced text
                            let replacement_len = text.len() * original_len;
                            let end_offset = start_anchor.to_offset(&snapshot) + replacement_len;
                            let end_point = snapshot.offset_to_point(end_offset);
                            if was_reversed {
                                end_point..start_point
                            } else {
                                start_point..end_point
                            }
                        }
                    })
                    .collect();

                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges(ranges);
                });
            });
        });
        self.switch_mode(Mode::HelixNormal, true, window, cx);
    }

    pub fn helix_goto_last_modification(
        &mut self,
        _: &HelixGotoLastModification,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.jump(".".into(), false, false, window, cx);
    }

    pub fn helix_select_lines(
        &mut self,
        _: &HelixSelectLine,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = Vim::take_count(cx).unwrap_or(1);
        self.update_editor(cx, |_, editor, cx| {
            editor.hide_mouse_cursor(HideMouseCursorOrigin::MovementAction, cx);
            let display_map = editor.display_map.update(cx, |map, cx| map.snapshot(cx));
            let mut selections = editor.selections.all::<Point>(cx);
            let max_point = display_map.buffer_snapshot.max_point();
            let buffer_snapshot = &display_map.buffer_snapshot;

            for selection in &mut selections {
                // Start always goes to column 0 of the first selected line
                let start_row = selection.start.row;
                let current_end_row = selection.end.row;

                // Check if cursor is on empty line by checking first character
                let line_start_offset = buffer_snapshot.point_to_offset(Point::new(start_row, 0));
                let first_char = buffer_snapshot.chars_at(line_start_offset).next();
                let extra_line = if first_char == Some('\n') { 1 } else { 0 };

                let end_row = current_end_row + count as u32 + extra_line;

                selection.start = Point::new(start_row, 0);
                selection.end = if end_row > max_point.row {
                    max_point
                } else {
                    Point::new(end_row, 0)
                };
                selection.reversed = false;
            }

            editor.change_selections(Default::default(), window, cx, |s| {
                s.select(selections);
            });
        });
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{state::Mode, test::VimTestContext};

    #[gpui::test]
    async fn test_word_motions(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        // «
        // ˇ
        // »
        cx.set_state(
            indoc! {"
            Th«e quiˇ»ck brown
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

        cx.simulate_keystrokes("2 b");

        cx.assert_state(
            indoc! {"
            The «ˇquick »brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("down e up");

        cx.assert_state(
            indoc! {"
            The quicˇk brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.set_state("aa\n  «ˇbb»", Mode::HelixNormal);

        cx.simulate_keystroke("b");

        cx.assert_state("aa\n«ˇ  »bb", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_delete(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

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

    // #[gpui::test]
    // async fn test_delete_character_end_of_buffer(cx: &mut gpui::TestAppContext) {
    //     let mut cx = VimTestContext::new(cx, true).await;

    //     cx.set_state(
    //         indoc! {"
    //         The quick brown
    //         fox jumps over
    //         the lazy dog.ˇ"},
    //         Mode::HelixNormal,
    //     );

    //     cx.simulate_keystrokes("d");

    //     cx.assert_state(
    //         indoc! {"
    //         The quick brown
    //         fox jumps over
    //         the lazy dog.ˇ"},
    //         Mode::HelixNormal,
    //     );
    // }

    #[gpui::test]
    async fn test_f_and_t(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("f z");

        cx.assert_state(
            indoc! {"
                The qu«ick brown
                fox jumps over
                the lazˇ»y dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("F e F e");

        cx.assert_state(
            indoc! {"
                The quick brown
                fox jumps ov«ˇer
                the» lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("e 2 F e");

        cx.assert_state(
            indoc! {"
                Th«ˇe quick brown
                fox jumps over»
                the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("t r t r");

        cx.assert_state(
            indoc! {"
                The quick «brown
                fox jumps oveˇ»r
                the lazy dog."},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_newline_char(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state("aa«\nˇ»bb cc", Mode::HelixNormal);

        cx.simulate_keystroke("w");

        cx.assert_state("aa\n«bb ˇ»cc", Mode::HelixNormal);

        cx.set_state("aa«\nˇ»", Mode::HelixNormal);

        cx.simulate_keystroke("b");

        cx.assert_state("«ˇaa»\n", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_insert_selected(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.set_state(
            indoc! {"
            «The ˇ»quick brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("i");

        cx.assert_state(
            indoc! {"
            ˇThe quick brown
            fox jumps over
            the lazy dog."},
            Mode::Insert,
        );
    }

    #[gpui::test]
    async fn test_append(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        // test from the end of the selection
        cx.set_state(
            indoc! {"
            «Theˇ» quick brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("a");

        cx.assert_state(
            indoc! {"
            Theˇ quick brown
            fox jumps over
            the lazy dog."},
            Mode::Insert,
        );

        // test from the beginning of the selection
        cx.set_state(
            indoc! {"
            «ˇThe» quick brown
            fox jumps over
            the lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("a");

        cx.assert_state(
            indoc! {"
            Theˇ quick brown
            fox jumps over
            the lazy dog."},
            Mode::Insert,
        );
    }

    #[gpui::test]
    async fn test_replace(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // No selection (single character)
        cx.set_state("ˇaa", Mode::HelixNormal);

        cx.simulate_keystrokes("r x");

        cx.assert_state("ˇxa", Mode::HelixNormal);

        // Cursor at the beginning
        cx.set_state("«ˇaa»", Mode::HelixNormal);

        cx.simulate_keystrokes("r x");

        cx.assert_state("«ˇxx»", Mode::HelixNormal);

        // Cursor at the end
        cx.set_state("«aaˇ»", Mode::HelixNormal);

        cx.simulate_keystrokes("r x");

        cx.assert_state("«xxˇ»", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_yank(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Test yanking current character with no selection
        cx.set_state("hello ˇworld", Mode::HelixNormal);
        cx.simulate_keystrokes("y");

        // Test cursor remains at the same position after yanking single character
        cx.assert_state("hello ˇworld", Mode::HelixNormal);
        cx.shared_clipboard().assert_eq("w");

        // Move cursor and yank another character
        cx.simulate_keystrokes("l");
        cx.simulate_keystrokes("y");
        cx.shared_clipboard().assert_eq("o");

        // Test yanking with existing selection
        cx.set_state("hello «worlˇ»d", Mode::HelixNormal);
        cx.simulate_keystrokes("y");
        cx.shared_clipboard().assert_eq("worl");
        cx.assert_state("hello «worlˇ»d", Mode::HelixNormal);

        // Test yanking in select mode character by character
        cx.set_state("hello ˇworld", Mode::HelixNormal);
        cx.simulate_keystroke("v");
        cx.assert_state("hello «wˇ»orld", Mode::HelixSelect);
        cx.simulate_keystroke("y");
        cx.assert_state("hello «wˇ»orld", Mode::HelixNormal);
        cx.shared_clipboard().assert_eq("w");
    }

    #[gpui::test]
    async fn test_shift_r_paste(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // First copy some text to clipboard
        cx.set_state("«hello worldˇ»", Mode::HelixNormal);
        cx.simulate_keystrokes("y");

        // Test paste with shift-r on single cursor
        cx.set_state("foo ˇbar", Mode::HelixNormal);
        cx.simulate_keystrokes("shift-r");

        cx.assert_state("foo hello worldˇbar", Mode::HelixNormal);

        // Test paste with shift-r on selection
        cx.set_state("foo «barˇ» baz", Mode::HelixNormal);
        cx.simulate_keystrokes("shift-r");

        cx.assert_state("foo hello worldˇ baz", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_select_mode(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        assert_eq!(cx.mode(), Mode::Normal);
        cx.enable_helix();

        cx.simulate_keystrokes("v");
        assert_eq!(cx.mode(), Mode::HelixSelect);
        cx.simulate_keystrokes("escape");
        assert_eq!(cx.mode(), Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_insert_mode_stickiness(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Make a modification at a specific location
        cx.set_state("ˇhello", Mode::HelixNormal);
        assert_eq!(cx.mode(), Mode::HelixNormal);
        cx.simulate_keystrokes("i");
        assert_eq!(cx.mode(), Mode::Insert);
        cx.simulate_keystrokes("escape");
        assert_eq!(cx.mode(), Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_goto_last_modification(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Make a modification at a specific location
        cx.set_state("line one\nline ˇtwo\nline three", Mode::HelixNormal);
        cx.assert_state("line one\nline ˇtwo\nline three", Mode::HelixNormal);
        cx.simulate_keystrokes("i");
        cx.simulate_keystrokes("escape");
        cx.simulate_keystrokes("i");
        cx.simulate_keystrokes("m o d i f i e d space");
        cx.simulate_keystrokes("escape");

        // TODO: this fails, because state is no longer helix
        cx.assert_state(
            "line one\nline modified ˇtwo\nline three",
            Mode::HelixNormal,
        );

        // Move cursor away from the modification
        cx.simulate_keystrokes("up");

        // Use "g ." to go back to last modification
        cx.simulate_keystrokes("g .");

        // Verify we're back at the modification location and still in HelixNormal mode
        cx.assert_state(
            "line one\nline modifiedˇ two\nline three",
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_select_lines(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.set_state(
            "line one\nline ˇtwo\nline three\nline four",
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("2 x");
        cx.assert_state(
            "line one\n«line two\nline three\nˇ»line four",
            Mode::HelixNormal,
        );

        // Test extending existing line selection
        cx.set_state(
            indoc! {"
            li«ˇne one
            li»ne two
            line three
            line four"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            «line one
            line two
            ˇ»line three
            line four"},
            Mode::HelixNormal,
        );

        // Pressing x in empty line, select next line (because helix considers cursor a selection)
        cx.set_state(
            indoc! {"
            line one
            ˇ
            line three
            line four"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            line one
            «
            line three
            ˇ»line four"},
            Mode::HelixNormal,
        );

        // Empty line with count selects extra + count lines
        cx.set_state(
            indoc! {"
            line one
            ˇ
            line three
            line four
            line five"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("2 x");
        cx.assert_state(
            indoc! {"
            line one
            «
            line three
            line four
            ˇ»line five"},
            Mode::HelixNormal,
        );

        // Compare empty vs non-empty line behavior
        cx.set_state(
            indoc! {"
            ˇnon-empty line
            line two
            line three"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            «non-empty line
            ˇ»line two
            line three"},
            Mode::HelixNormal,
        );

        // Same test but with empty line - should select one extra
        cx.set_state(
            indoc! {"
            ˇ
            line two
            line three"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            «
            line two
            ˇ»line three"},
            Mode::HelixNormal,
        );

        // Test selecting multiple lines with count
        cx.set_state(
            indoc! {"
            ˇline one
            line two
            line threeˇ
            line four
            line five"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            «line one
            ˇ»line two
            «line three
            ˇ»line four
            line five"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("x");
        cx.assert_state(
            indoc! {"
            «line one
            line two
            line three
            line four
            ˇ»line five"},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_select_mode_motion(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        assert_eq!(cx.mode(), Mode::Normal);
        cx.enable_helix();

        cx.set_state("ˇhello", Mode::HelixNormal);
        cx.simulate_keystrokes("l v l l");
        cx.assert_state("h«ellˇ»o", Mode::HelixSelect);
    }

    #[gpui::test]
    async fn test_helix_select_mode_motion_multiple_cursors(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        assert_eq!(cx.mode(), Mode::Normal);
        cx.enable_helix();

        // Start with multiple cursors (no selections)
        cx.set_state("ˇhello\nˇworld", Mode::HelixNormal);

        // Enter select mode and move right twice
        cx.simulate_keystrokes("v l l");

        // Each cursor should independently create and extend its own selection
        cx.assert_state("«helˇ»lo\n«worˇ»ld", Mode::HelixSelect);
    }

    #[gpui::test]
    async fn test_helix_select_word_motions(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("ˇone two", Mode::Normal);
        cx.simulate_keystrokes("v w");
        cx.assert_state("«one tˇ»wo", Mode::Visual);

        // In Vim, this selects "t". In helix selections stops just before "t"

        cx.enable_helix();
        cx.set_state("ˇone two", Mode::HelixNormal);
        cx.simulate_keystrokes("v w");
        cx.assert_state("«one ˇ»two", Mode::HelixSelect);
    }
}
