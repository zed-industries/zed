use editor::{DisplayPoint, Editor, movement};
use gpui::{Action, actions};
use gpui::{Context, Window};
use language::{CharClassifier, CharKind};
use text::SelectionGoal;

use crate::{Vim, motion::Motion, state::Mode};

actions!(
    vim,
    [
        /// Switches to normal mode after the cursor (Helix-style).
        HelixNormalAfter
    ]
);

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, Vim::helix_normal_after);
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

    // #[gpui::test]
    // async fn test_delete(cx: &mut gpui::TestAppContext) {
    //     let mut cx = VimTestContext::new(cx, true).await;

    //     // test delete a selection
    //     cx.set_state(
    //         indoc! {"
    //         The qu«ick ˇ»brown
    //         fox jumps over
    //         the lazy dog."},
    //         Mode::HelixNormal,
    //     );

    //     cx.simulate_keystrokes("d");

    //     cx.assert_state(
    //         indoc! {"
    //         The quˇbrown
    //         fox jumps over
    //         the lazy dog."},
    //         Mode::HelixNormal,
    //     );

    //     // test deleting a single character
    //     cx.simulate_keystrokes("d");

    //     cx.assert_state(
    //         indoc! {"
    //         The quˇrown
    //         fox jumps over
    //         the lazy dog."},
    //         Mode::HelixNormal,
    //     );
    // }

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
}
