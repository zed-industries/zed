use editor::display_map::ToDisplayPoint;
use editor::{
    DisplayPoint, Editor, HideMouseCursorOrigin, SelectionEffects, ToOffset, ToPoint, movement,
};
use gpui::actions;
use gpui::{Context, Window};
use language::{CharClassifier, CharKind, Point};
use multi_buffer::MultiBufferRow;
use text::{Bias, SelectionGoal};

use crate::object::Object;
use crate::state::Operator;
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
        /// Select entire line or multiple lines, extending downwards.
        HelixSelectLine,
        /// Select current paragraph
        PushMatch,
    ]
);

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, Vim::helix_insert);
    Vim::action(editor, cx, Vim::helix_append);
    Vim::action(editor, cx, Vim::helix_yank);
    Vim::action(editor, cx, Vim::helix_select_lines);
}

impl Vim {
    // pub fn helix_normal_after(
    //     &mut self,
    //     action: &HelixNormalAfter,
    //     window: &mut Window,
    //     cx: &mut Context<Self>,
    // ) {
    //     if self.active_operator().is_some() {
    //         self.operator_stack.clear();
    //         self.sync_vim_settings(window, cx);
    //         return;
    //     }
    //     self.stop_recording_immediately(action.boxed_clone(), cx);
    //     self.switch_mode(Mode::HelixNormal, false, window, cx);
    //     return;
    // }

    // Helix motion which creates a selection
    fn helix_move_and_select(
        &mut self,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
        motion: Motion, //mut is_boundary: impl FnMut(char, char, &CharClassifier) -> bool,
    ) {
        self.update_editor(window, cx, |_, editor, window, cx| {
            let text_layout_details = editor.text_layout_details(window);
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(|map, selection| {
                    let goal = selection.goal;
                    let old_point = selection.head();
                    let was_empty = selection.is_empty();

                    let (new_point, new_goal) = motion
                        .move_point(
                            map,
                            old_point,
                            selection.goal,
                            times,
                            &text_layout_details,
                            true,
                        )
                        .unwrap_or((old_point, goal));

                    selection.set_tail(old_point, goal);
                    selection.set_head(new_point, new_goal);

                    // include old position only if selection was empty
                    if was_empty && selection.end == old_point {
                        selection.end = movement::right(map, selection.end)
                    }
                    // must include cursor position
                    if selection.end == new_point {
                        selection.end = movement::right(map, selection.end)
                    }
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

    // Helix motion which do not create any selection
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
                        .move_point(
                            map,
                            cursor,
                            selection.goal,
                            times,
                            &text_layout_details,
                            true,
                        )
                        .unwrap_or((cursor, goal));

                    selection.collapse_to(point, goal)
                })
            });
        });
    }

    pub fn helix_normal_motion(
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
            Motion::FindForward { .. } | Motion::FindBackward { .. } => {
                return self.helix_move_and_select(times, window, cx, motion);
            }
            _ => self.helix_move_and_collapse(motion, times, window, cx),
        }
    }

    pub fn helix_yank(&mut self, _: &HelixYank, window: &mut Window, cx: &mut Context<Self>) {
        self.update_editor(window, cx, |vim, editor, window, cx| {
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
    }

    pub fn helix_object(
        &mut self,
        object: Object,
        count: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Vim>,
    ) {
        if let Some(Operator::Object { around }) = self.active_operator() {
            self.pop_operator(window, cx);
            let current_mode = self.mode;
            let target_mode = object.target_visual_mode(current_mode, around);
            if target_mode != current_mode {
                self.switch_mode(target_mode, true, window, cx);
            }

            self.update_editor(window, cx, |_, editor, window, cx| {
                editor.change_selections(Default::default(), window, cx, |s| {
                    s.move_with(|map, selection| {
                        // Selection has no meaning, only current cursor position
                        if selection.reversed {
                            selection.collapse_to(selection.start, SelectionGoal::None);
                        } else {
                            selection.collapse_to(selection.end, SelectionGoal::None);
                        }

                        // Skip newlines between paragraphs
                        let current_line_is_empty = map
                            .buffer_snapshot
                            .is_line_blank(MultiBufferRow(selection.start.to_point(map).row));
                        if object == Object::Paragraph && current_line_is_empty {
                            selection.start = Point::new(selection.start.to_point(map).row + 1, 0)
                                .to_display_point(map);

                            if selection.end < selection.start {
                                selection.end = selection.start
                            }
                        }

                        if let Some(range) = object.range(map, selection.clone(), around, count) {
                            selection.start = range.start;
                            selection.end = range.end;

                            // add newline after paragraph
                            if object == Object::Paragraph && around {
                                selection.end = Point::new(selection.end.to_point(map).row + 1, 0)
                                    .to_display_point(map);
                            }
                        }
                    });
                });
            });
        }
        self.switch_mode(Mode::HelixNormal, true, window, cx);
        self.operator_stack.clear();
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

    pub fn helix_select_lines(
        &mut self,
        _: &HelixSelectLine,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = Vim::take_count(cx).unwrap_or(1);
        self.update_editor(window, cx, |_, editor, window, cx| {
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
    use gpui::TestAppContext;
    use indoc::indoc;

    use crate::{state::Mode, test::VimTestContext};

    #[gpui::test]
    async fn test_word_motions(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

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

    #[gpui::test]
    async fn test_helix_select_around_paragraph_basic(cx: &mut TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Initialize cursor state to ensure a selection exists
        cx.set_state(
            indoc! {"
                ˇone

                two two

                three
            "},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("m a w");
        cx.assert_state(
            indoc! {"
                «oneˇ»

                two two

                three
            "},
            Mode::HelixNormal,
        );

        cx.set_state(
            indoc! {"
                one
                ˇ
                two two

                three
            "},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("m a p");
        cx.assert_state(
            indoc! {"
                one

                «two two

                ˇ»three
            "},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_select_around_paragraph_count(cx: &mut TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Initialize cursor state to ensure a selection exists
        cx.set_state(
            indoc! {"
                ˇone

                two two

                three

                four
            "},
            Mode::HelixNormal,
        );

        cx.assert_binding(
            "3 m a p",
            indoc! {"
                ˇone

                two two

                three

                four
            "},
            Mode::HelixNormal,
            indoc! {"
                «one

                two two

                three

                ˇ»four
            "},
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
    }

    #[gpui::test]
    async fn test_helix_select_lines(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
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
}
