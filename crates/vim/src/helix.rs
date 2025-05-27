use editor::{DisplayPoint, Editor, movement, scroll::Autoscroll};
use gpui::{Action, actions};
use gpui::{Context, Window};
use language::{CharClassifier, CharKind};

use crate::motion::MotionKind;
use crate::{Vim, motion::Motion, state::Mode};

actions!(vim, [HelixNormalAfter, HelixDelete]);

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, Vim::helix_normal_after);
    Vim::action(editor, cx, Vim::helix_delete);
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
            _ => self.helix_move_and_collapse(motion, times, window, cx),
        }
    }

    pub fn helix_delete(&mut self, _: &HelixDelete, window: &mut Window, cx: &mut Context<Self>) {
        self.store_visual_marks(window, cx);
        self.update_editor(window, cx, |vim, editor, window, cx| {
            // Fixup selections so they have helix's semantics.
            // Specifically:
            //  - Make sure that each cursor acts as a 1 character wide selection
            editor.transact(window, cx, |editor, window, cx| {
                editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                    s.move_with(|map, selection| {
                        if selection.is_empty() && !selection.reversed {
                            selection.end = movement::right(map, selection.end);
                        }
                    });
                });
            });

            vim.copy_selections_content(editor, MotionKind::Exclusive, window, cx);
            editor.insert("", window, cx);
        });
    }
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
}
