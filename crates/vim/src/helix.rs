use editor::{DisplayPoint, Editor, SelectionEffects, ToOffset, ToPoint, movement};
use gpui::{Action, actions};
use gpui::{Context, Window};
use language::{CharClassifier, CharKind};
use text::{Bias, Point, SelectionGoal};

use crate::{
    Vim,
    motion::{Motion, right},
    state::Mode,
};

actions!(
    vim,
    [
        /// Switches to normal mode after the cursor (Helix-style).
        HelixNormalAfter,
        /// Inserts at the beginning of the selection.
        HelixInsert,
        /// Appends at the end of the selection.
        HelixAppend,
        /// Selects the current line.
        HelixSelectLine,
    ]
);

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, Vim::helix_normal_after);
    Vim::action(editor, cx, Vim::helix_insert);
    Vim::action(editor, cx, Vim::helix_append);
    Vim::action(editor, cx, Vim::helix_select_line);
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
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(|map, selection| {
                    let times = times.unwrap_or(1);
                    let new_goal = SelectionGoal::None;
                    let mut head = selection.head();
                    let mut tail = selection.tail();

                    if head == map.max_point() {
                        return;
                    }

                    // collapse to block cursor
                    if tail < head {
                        tail = movement::left(map, head);
                    } else {
                        tail = head;
                        head = movement::right(map, head);
                    }

                    // create a classifier
                    let classifier = map.buffer_snapshot.char_classifier_at(head.to_point(map));

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

                    selection.set_tail(tail, new_goal);
                    selection.set_head(head, new_goal);
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
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(|map, selection| {
                    let times = times.unwrap_or(1);
                    let new_goal = SelectionGoal::None;
                    let mut head = selection.head();
                    let mut tail = selection.tail();

                    if head == DisplayPoint::zero() {
                        return;
                    }

                    // collapse to block cursor
                    if tail < head {
                        tail = movement::left(map, head);
                    } else {
                        tail = head;
                        head = movement::right(map, head);
                    }

                    selection.set_head(head, new_goal);
                    selection.set_tail(tail, new_goal);
                    // flip the selection
                    selection.swap_head_tail();
                    head = selection.head();
                    tail = selection.tail();

                    // create a classifier
                    let classifier = map.buffer_snapshot.char_classifier_at(head.to_point(map));

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

                    selection.set_tail(tail, new_goal);
                    selection.set_head(head, new_goal);
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

                    let found = (left_kind != right_kind && right_kind != CharKind::Whitespace)
                        || at_newline;

                    found
                })
            }
            Motion::NextWordEnd { ignore_punctuation } => {
                self.helix_find_range_forward(times, window, cx, |left, right, classifier| {
                    let left_kind = classifier.kind_with(left, ignore_punctuation);
                    let right_kind = classifier.kind_with(right, ignore_punctuation);
                    let at_newline = (left == '\n') ^ (right == '\n');

                    let found = (left_kind != right_kind && left_kind != CharKind::Whitespace)
                        || at_newline;

                    found
                })
            }
            Motion::PreviousWordStart { ignore_punctuation } => {
                self.helix_find_range_backward(times, window, cx, |left, right, classifier| {
                    let left_kind = classifier.kind_with(left, ignore_punctuation);
                    let right_kind = classifier.kind_with(right, ignore_punctuation);
                    let at_newline = (left == '\n') ^ (right == '\n');

                    let found = (left_kind != right_kind && left_kind != CharKind::Whitespace)
                        || at_newline;

                    found
                })
            }
            Motion::PreviousWordEnd { ignore_punctuation } => {
                self.helix_find_range_backward(times, window, cx, |left, right, classifier| {
                    let left_kind = classifier.kind_with(left, ignore_punctuation);
                    let right_kind = classifier.kind_with(right, ignore_punctuation);
                    let at_newline = (left == '\n') ^ (right == '\n');

                    let found = (left_kind != right_kind && right_kind != CharKind::Whitespace)
                        || at_newline;

                    found
                })
            }
            Motion::FindForward { .. } => {
                self.update_editor(window, cx, |_, editor, window, cx| {
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
                                .move_point(
                                    map,
                                    cursor,
                                    selection.goal,
                                    times,
                                    &text_layout_details,
                                )
                                .unwrap_or((cursor, goal));
                            selection.set_tail(selection.head(), goal);
                            selection.set_head(movement::right(map, point), goal);
                        })
                    });
                });
            }
            Motion::FindBackward { .. } => {
                self.update_editor(window, cx, |_, editor, window, cx| {
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
                                .move_point(
                                    map,
                                    cursor,
                                    selection.goal,
                                    times,
                                    &text_layout_details,
                                )
                                .unwrap_or((cursor, goal));
                            selection.set_tail(selection.head(), goal);
                            selection.set_head(point, goal);
                        })
                    });
                });
            }
            _ => self.helix_move_and_collapse(motion, times, window, cx),
        }
    }

    fn helix_insert(&mut self, _: &HelixInsert, window: &mut Window, cx: &mut Context<Self>) {
        self.start_recording(cx);
        self.update_editor(window, cx, |_, editor, window, cx| {
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
        self.update_editor(window, cx, |_, editor, window, cx| {
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

    fn helix_select_line(
        &mut self,
        _: &HelixSelectLine,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = Vim::take_count(cx).unwrap_or(1);
        Vim::take_forced_motion(cx);

        self.update_editor(window, cx, |_, editor, window, cx| {
            let display_map = editor.display_map.update(cx, |map, cx| map.snapshot(cx));
            let mut selections = editor.selections.all::<Point>(cx);
            let max_point = display_map.buffer_snapshot.max_point();

            for selection in &mut selections {
                // Check if this is already a complete line selection
                let is_line_selection = selection.start.column == 0
                    && selection.end.column == selection.head().column
                    && selection.end.row > selection.start.row;

                let (start_row, end_row) = if is_line_selection {
                    // Extend existing line selection by count more lines
                    let end_row = selection.end.row + count as u32;
                    (selection.start.row, std::cmp::min(max_point.row, end_row))
                } else {
                    // Start new line selection from cursor position
                    let cursor_row = selection.head().row;
                    let end_row = cursor_row + count as u32;
                    (cursor_row, std::cmp::min(max_point.row, end_row))
                };

                selection.start = Point::new(start_row, 0);
                selection.end = std::cmp::min(max_point, Point::new(end_row, 0));
            }
            editor.change_selections(Default::default(), window, cx, |s| {
                s.select(selections);
            });
        });
    }

    pub fn helix_replace(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.update_editor(window, cx, |_, editor, window, cx| {
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
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{state::Mode, test::VimTestContext};

    #[gpui::test]
    async fn test_word_motions(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
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

    // #[gpui::test]
    // async fn test_delete_character_end_of_line(cx: &mut gpui::TestAppContext) {
    //     let mut cx = VimTestContext::new(cx, true).await;

    //     cx.set_state(
    //         indoc! {"
    //         The quick brownˇ
    //         fox jumps over
    //         the lazy dog."},
    //         Mode::HelixNormal,
    //     );

    //     cx.simulate_keystrokes("d");

    //     cx.assert_state(
    //         indoc! {"
    //         The quick brownˇfox jumps over
    //         the lazy dog."},
    //         Mode::HelixNormal,
    //     );
    // }

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

        cx.simulate_keystrokes("2 T r");

        cx.assert_state(
            indoc! {"
                The quick br«ˇown
                fox jumps over
                the laz»y dog."},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_newline_char(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

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
    async fn test_helix_select_line_basic(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Test basic single line selection
        cx.set_state(
            indoc! {"
                The quick brown
                fox ˇjumps over
                the lazy dog
            "},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("x");

        cx.assert_state(
            indoc! {"
                The quick brown
                «fox jumps over
                ˇ»the lazy dog
            "},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_select_line_basic_beginning(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Test basic single line selection
        cx.set_state(
            indoc! {"
                ˇThe quick brown
                fox jumps over
                the lazy dog
            "},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("x");

        cx.assert_state(
            indoc! {"
                «The quick brown
                ˇ»fox jumps over
                the lazy dog
            "},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_select_line_with_count(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Test count-based selection (3x should select 3 lines)
        cx.set_state(
            indoc! {"
                   The ˇquick brown
                   fox jumps over
                   the lazy dog
                   another line
                   final line
               "},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("3 x");

        cx.assert_state(
            indoc! {"
                   «The quick brown
                   fox jumps over
                   the lazy dog
                   ˇ»another line
                   final line
               "},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_select_line_extend_existing(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Test extending existing line selection
        cx.set_state(
            indoc! {"
                   «The quick brown
                   fox jumps over
                   ˇ»the lazy dog
                   another line
               "},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("x");

        cx.assert_state(
            indoc! {"
                   «The quick brown
                   fox jumps over
                   the lazy dog
                   ˇ»another line
               "},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_select_line_extend_with_count(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Test extending with count
        cx.set_state(
            indoc! {"
                   «The quick brown
                   fox jumps overˇ»
                   the lazy dog
                   another line
                   final line
               "},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("2 x");

        cx.assert_state(
            indoc! {"
                   «The quick brown
                   fox jumps over
                   the lazy dog
                   ˇ»another line
                   final line
               "},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_select_line_end_of_file(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Test selection at end of file
        cx.set_state(
            indoc! {"
                   first line
                   second line
                   third ˇline
               "},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("5 x"); // Try to select more lines than available

        cx.assert_state(
            indoc! {"
                   first line
                   second line
                   «third line
                   ˇ»"},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_select_line_empty_lines(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Test selection with empty lines
        cx.set_state(
            indoc! {"
                   first line
                   ˇ
                   third line
               "},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("2 x");

        cx.assert_state(
            indoc! {"
                   first line
                   «
                   third line
                   ˇ»"},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_select_line_multiple_cursors(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Set up multiple cursors on different lines
        cx.set_state(
            indoc! {"
                first ˇline
                second line
                third ˇline
                fourth line
            "},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("x");

        cx.assert_state(
            indoc! {"
                «first line
                ˇ»second line
                «third line
                ˇ»fourth line
            "},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_select_line_multiple_cursors_with_count(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Test multiple cursors with count
        cx.set_state(
            indoc! {"
                first ˇline
                second line
                third line
                fourth ˇline
                fifth line
                sixth line
                seventh line
            "},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("2 x");

        cx.assert_state(
            indoc! {"
                «first line
                second line
                ˇ»third line
                «fourth line
                fifth line
                ˇ»sixth line
                seventh line
            "},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_select_line_overlapping_selections(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Test multiple cursors with count
        cx.set_state(
            indoc! {"
                        first ˇline
                        second line
                        third line
                        fourth ˇline
                        fifth line
                        sixth line
                        seventh line
                    "},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("3 x");

        cx.assert_state(
            indoc! {"
                        «first line
                        second line
                        third line
                        fourth line
                        fifth line
                        sixth line
                        ˇ»seventh line
                    "},
            Mode::HelixNormal,
        );
    }
}
