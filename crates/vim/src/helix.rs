use editor::{Editor, SelectionEffects, ToOffset, ToPoint, movement};
use gpui::{Action, actions};
use gpui::{Context, Window};
use text::{Bias, SelectionGoal};

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
        /// Yanks the current selection or character if no selection.
        HelixYank,
        /// Goes to the location of the last modification.
        HelixGotoLastModification,
    ]
);

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, Vim::helix_normal_after);
    Vim::action(editor, cx, Vim::helix_insert);
    Vim::action(editor, cx, Vim::helix_append);
    Vim::action(editor, cx, Vim::helix_yank);
    Vim::action(editor, cx, Vim::helix_goto_last_modification);
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
        self.switch_mode(Mode::HelixNormal, true, window, cx);
        return;
    }

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
            Motion::NextWordStart { .. }
            | Motion::NextWordEnd { .. }
            | Motion::PreviousWordStart { .. }
            | Motion::PreviousWordEnd { .. }
            | Motion::FindForward { .. }
            | Motion::FindBackward { .. } => {
                return self.helix_move_and_select(times, window, cx, motion);
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

    pub fn helix_goto_last_modification(
        &mut self,
        _: &HelixGotoLastModification,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.jump(".".into(), false, false, window, cx);
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
    async fn test_select_motion(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // vim goes to start of next word
        cx.set_state("ˇone two three four five", Mode::HelixNormal);
        cx.simulate_keystrokes("v l l l");
        cx.assert_state("«one ˇ»two three four five", Mode::HelixSelect);
    }

    #[gpui::test]
    async fn test_word_select_motion(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // vim goes to start of next word
        cx.set_state("ˇone two three four five", Mode::Normal);
        cx.simulate_keystrokes("w");
        cx.assert_state("one ˇtwo three four five", Mode::Normal);

        // visual mode does not affect logic
        cx.set_state("ˇone two three four five", Mode::Normal);
        cx.simulate_keystrokes("v w");
        cx.assert_state("«one tˇ»wo three four five", Mode::Visual);
        cx.simulate_keystrokes("w");
        cx.assert_state("«one two tˇ»hree four five", Mode::Visual);

        cx.enable_helix();

        // helix selects up to the first letter of next word, not including it
        cx.set_state("ˇone two three four five", Mode::HelixNormal);
        cx.simulate_keystrokes("w");
        cx.assert_state("«one ˇ»two three four five", Mode::HelixNormal);

        // helix select mode should not affect this
        cx.set_state("ˇone two three four five", Mode::HelixNormal);
        cx.simulate_keystrokes("v w");
        cx.assert_state("«one ˇ»two three four five", Mode::HelixSelect);
        cx.simulate_keystrokes("w");
        cx.assert_state("«one two ˇ»three four five", Mode::HelixSelect);
    }

    #[gpui::test]
    async fn test_back_select_motion(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // vim goes to start of next word
        cx.set_state("ˇone two three four five", Mode::HelixNormal);
        cx.simulate_keystrokes("w w v b b");
        cx.assert_state("«ˇone t»wo three four five", Mode::HelixSelect);
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
}
