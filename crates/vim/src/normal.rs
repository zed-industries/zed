mod g_prefix;

use editor::{char_kind, movement, Bias};
use gpui::{action, keymap::Binding, MutableAppContext, ViewContext};
use language::SelectionGoal;
use workspace::Workspace;

use crate::{mode::NormalState, Mode, SwitchMode, VimState};

action!(GPrefix);
action!(MoveLeft);
action!(MoveDown);
action!(MoveUp);
action!(MoveRight);
action!(MoveToStartOfLine);
action!(MoveToEndOfLine);
action!(MoveToEnd);
action!(MoveToNextWordStart, bool);
action!(MoveToNextWordEnd, bool);
action!(MoveToPreviousWordStart, bool);

pub fn init(cx: &mut MutableAppContext) {
    let context = Some("Editor && vim_mode == normal");
    cx.add_bindings(vec![
        Binding::new("i", SwitchMode(Mode::Insert), context),
        Binding::new("g", SwitchMode(Mode::Normal(NormalState::GPrefix)), context),
        Binding::new("h", MoveLeft, context),
        Binding::new("j", MoveDown, context),
        Binding::new("k", MoveUp, context),
        Binding::new("l", MoveRight, context),
        Binding::new("0", MoveToStartOfLine, context),
        Binding::new("shift-$", MoveToEndOfLine, context),
        Binding::new("shift-G", MoveToEnd, context),
        Binding::new("w", MoveToNextWordStart(false), context),
        Binding::new("shift-W", MoveToNextWordStart(true), context),
        Binding::new("e", MoveToNextWordEnd(false), context),
        Binding::new("shift-E", MoveToNextWordEnd(true), context),
        Binding::new("b", MoveToPreviousWordStart(false), context),
        Binding::new("shift-B", MoveToPreviousWordStart(true), context),
    ]);
    g_prefix::init(cx);

    cx.add_action(move_left);
    cx.add_action(move_down);
    cx.add_action(move_up);
    cx.add_action(move_right);
    cx.add_action(move_to_start_of_line);
    cx.add_action(move_to_end_of_line);
    cx.add_action(move_to_end);
    cx.add_action(move_to_next_word_start);
    cx.add_action(move_to_next_word_end);
    cx.add_action(move_to_previous_word_start);
}

fn move_left(_: &mut Workspace, _: &MoveLeft, cx: &mut ViewContext<Workspace>) {
    VimState::update_global(cx, |state, cx| {
        state.update_active_editor(cx, |editor, cx| {
            editor.move_cursors(cx, |map, mut cursor, _| {
                *cursor.column_mut() = cursor.column().saturating_sub(1);
                (map.clip_point(cursor, Bias::Left), SelectionGoal::None)
            });
        });
    })
}

fn move_down(_: &mut Workspace, _: &MoveDown, cx: &mut ViewContext<Workspace>) {
    VimState::update_global(cx, |state, cx| {
        state.update_active_editor(cx, |editor, cx| {
            editor.move_cursors(cx, movement::down);
        });
    });
}

fn move_up(_: &mut Workspace, _: &MoveUp, cx: &mut ViewContext<Workspace>) {
    VimState::update_global(cx, |state, cx| {
        state.update_active_editor(cx, |editor, cx| {
            editor.move_cursors(cx, movement::up);
        });
    });
}

fn move_right(_: &mut Workspace, _: &MoveRight, cx: &mut ViewContext<Workspace>) {
    VimState::update_global(cx, |state, cx| {
        state.update_active_editor(cx, |editor, cx| {
            editor.move_cursors(cx, |map, mut cursor, _| {
                *cursor.column_mut() += 1;
                (map.clip_point(cursor, Bias::Right), SelectionGoal::None)
            });
        });
    });
}

fn move_to_start_of_line(
    _: &mut Workspace,
    _: &MoveToStartOfLine,
    cx: &mut ViewContext<Workspace>,
) {
    VimState::update_global(cx, |state, cx| {
        state.update_active_editor(cx, |editor, cx| {
            editor.move_cursors(cx, |map, cursor, _| {
                (
                    movement::line_beginning(map, cursor, false),
                    SelectionGoal::None,
                )
            });
        });
    });
}

fn move_to_end_of_line(_: &mut Workspace, _: &MoveToEndOfLine, cx: &mut ViewContext<Workspace>) {
    VimState::update_global(cx, |state, cx| {
        state.update_active_editor(cx, |editor, cx| {
            editor.move_cursors(cx, |map, cursor, _| {
                (
                    map.clip_point(movement::line_end(map, cursor, false), Bias::Left),
                    SelectionGoal::None,
                )
            });
        });
    });
}

fn move_to_end(_: &mut Workspace, _: &MoveToEnd, cx: &mut ViewContext<Workspace>) {
    VimState::update_global(cx, |state, cx| {
        state.update_active_editor(cx, |editor, cx| {
            editor.replace_selections_with(cx, |map| map.clip_point(map.max_point(), Bias::Left));
        });
    });
}

fn move_to_next_word_start(
    _: &mut Workspace,
    &MoveToNextWordStart(treat_punctuation_as_word): &MoveToNextWordStart,
    cx: &mut ViewContext<Workspace>,
) {
    VimState::update_global(cx, |state, cx| {
        state.update_active_editor(cx, |editor, cx| {
            editor.move_cursors(cx, |map, mut cursor, _| {
                let mut crossed_newline = false;
                cursor = movement::find_boundary(map, cursor, |left, right| {
                    let left_kind = char_kind(left).coerce_punctuation(treat_punctuation_as_word);
                    let right_kind = char_kind(right).coerce_punctuation(treat_punctuation_as_word);
                    let at_newline = right == '\n';

                    let found = (left_kind != right_kind && !right.is_whitespace())
                        || (at_newline && crossed_newline)
                        || (at_newline && left == '\n'); // Prevents skipping repeated empty lines

                    if at_newline {
                        crossed_newline = true;
                    }
                    found
                });
                (cursor, SelectionGoal::None)
            });
        });
    });
}

fn move_to_next_word_end(
    _: &mut Workspace,
    &MoveToNextWordEnd(treat_punctuation_as_word): &MoveToNextWordEnd,
    cx: &mut ViewContext<Workspace>,
) {
    VimState::update_global(cx, |state, cx| {
        state.update_active_editor(cx, |editor, cx| {
            editor.move_cursors(cx, |map, mut cursor, _| {
                *cursor.column_mut() += 1;
                cursor = movement::find_boundary(map, cursor, |left, right| {
                    let left_kind = char_kind(left).coerce_punctuation(treat_punctuation_as_word);
                    let right_kind = char_kind(right).coerce_punctuation(treat_punctuation_as_word);

                    left_kind != right_kind && !left.is_whitespace()
                });
                // find_boundary clips, so if the character after the next character is a newline or at the end of the document, we know
                // we have backtraced already
                if !map
                    .chars_at(cursor)
                    .skip(1)
                    .next()
                    .map(|c| c == '\n')
                    .unwrap_or(true)
                {
                    *cursor.column_mut() = cursor.column().saturating_sub(1);
                }
                (map.clip_point(cursor, Bias::Left), SelectionGoal::None)
            });
        });
    });
}

fn move_to_previous_word_start(
    _: &mut Workspace,
    &MoveToPreviousWordStart(treat_punctuation_as_word): &MoveToPreviousWordStart,
    cx: &mut ViewContext<Workspace>,
) {
    VimState::update_global(cx, |state, cx| {
        state.update_active_editor(cx, |editor, cx| {
            editor.move_cursors(cx, |map, mut cursor, _| {
                // This works even though find_preceding_boundary is called for every character in the line containing
                // cursor because the newline is checked only once.
                cursor = movement::find_preceding_boundary(map, cursor, |left, right| {
                    let left_kind = char_kind(left).coerce_punctuation(treat_punctuation_as_word);
                    let right_kind = char_kind(right).coerce_punctuation(treat_punctuation_as_word);

                    (left_kind != right_kind && !right.is_whitespace()) || left == '\n'
                });
                (cursor, SelectionGoal::None)
            });
        });
    });
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use util::test::marked_text;

    use crate::vim_test_context::VimTestContext;

    #[gpui::test]
    async fn test_hjkl(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true, "Test\nTestTest\nTest").await;
        cx.simulate_keystroke("l");
        cx.assert_editor_state(indoc! {"
            T|est
            TestTest
            Test"});
        cx.simulate_keystroke("h");
        cx.assert_editor_state(indoc! {"
            |Test
            TestTest
            Test"});
        cx.simulate_keystroke("j");
        cx.assert_editor_state(indoc! {"
            Test
            |TestTest
            Test"});
        cx.simulate_keystroke("k");
        cx.assert_editor_state(indoc! {"
            |Test
            TestTest
            Test"});
        cx.simulate_keystroke("j");
        cx.assert_editor_state(indoc! {"
            Test
            |TestTest
            Test"});

        // When moving left, cursor does not wrap to the previous line
        cx.simulate_keystroke("h");
        cx.assert_editor_state(indoc! {"
            Test
            |TestTest
            Test"});

        // When moving right, cursor does not reach the line end or wrap to the next line
        for _ in 0..9 {
            cx.simulate_keystroke("l");
        }
        cx.assert_editor_state(indoc! {"
            Test
            TestTes|t
            Test"});

        // Goal column respects the inability to reach the end of the line
        cx.simulate_keystroke("k");
        cx.assert_editor_state(indoc! {"
            Tes|t
            TestTest
            Test"});
        cx.simulate_keystroke("j");
        cx.assert_editor_state(indoc! {"
            Test
            TestTes|t
            Test"});
    }

    #[gpui::test]
    async fn test_jump_to_line_boundaries(cx: &mut gpui::TestAppContext) {
        let initial_content = indoc! {"
            Test Test
            
            T"};
        let mut cx = VimTestContext::new(cx, true, initial_content).await;

        cx.simulate_keystroke("shift-$");
        cx.assert_editor_state(indoc! {"
            Test Tes|t
            
            T"});
        cx.simulate_keystroke("0");
        cx.assert_editor_state(indoc! {"
            |Test Test
            
            T"});

        cx.simulate_keystroke("j");
        cx.simulate_keystroke("shift-$");
        cx.assert_editor_state(indoc! {"
            Test Test
            |
            T"});
        cx.simulate_keystroke("0");
        cx.assert_editor_state(indoc! {"
            Test Test
            |
            T"});

        cx.simulate_keystroke("j");
        cx.simulate_keystroke("shift-$");
        cx.assert_editor_state(indoc! {"
            Test Test
            
            |T"});
        cx.simulate_keystroke("0");
        cx.assert_editor_state(indoc! {"
            Test Test
            
            |T"});
    }

    #[gpui::test]
    async fn test_jump_to_end(cx: &mut gpui::TestAppContext) {
        let initial_content = indoc! {"
            The quick
            
            brown fox jumps
            over the lazy dog"};
        let mut cx = VimTestContext::new(cx, true, initial_content).await;

        cx.simulate_keystroke("shift-G");
        cx.assert_editor_state(indoc! {"
            The quick
            
            brown fox jumps
            over the lazy do|g"});

        // Repeat the action doesn't move
        cx.simulate_keystroke("shift-G");
        cx.assert_editor_state(indoc! {"
            The quick
            
            brown fox jumps
            over the lazy do|g"});
    }

    #[gpui::test]
    async fn test_next_word_start(cx: &mut gpui::TestAppContext) {
        let (initial_content, cursor_offsets) = marked_text(indoc! {"
            The |quick|-|brown
            |
            |
            |fox_jumps |over
            |th||e"});
        let mut cx = VimTestContext::new(cx, true, &initial_content).await;

        for cursor_offset in cursor_offsets {
            cx.simulate_keystroke("w");
            cx.assert_newest_selection_head_offset(cursor_offset);
        }

        // Reset and test ignoring punctuation
        cx.simulate_keystrokes(&["g", "g"]);
        let (_, cursor_offsets) = marked_text(indoc! {"
            The |quick-brown
            |
            |
            |fox_jumps |over
            |th||e"});

        for cursor_offset in cursor_offsets {
            cx.simulate_keystroke("shift-W");
            cx.assert_newest_selection_head_offset(cursor_offset);
        }
    }

    #[gpui::test]
    async fn test_next_word_end(cx: &mut gpui::TestAppContext) {
        let (initial_content, cursor_offsets) = marked_text(indoc! {"
            Th|e quic|k|-brow|n
            
            
            fox_jump|s ove|r
            th|e"});
        let mut cx = VimTestContext::new(cx, true, &initial_content).await;

        for cursor_offset in cursor_offsets {
            cx.simulate_keystroke("e");
            cx.assert_newest_selection_head_offset(cursor_offset);
        }

        // Reset and test ignoring punctuation
        cx.simulate_keystrokes(&["g", "g"]);
        let (_, cursor_offsets) = marked_text(indoc! {"
            Th|e quick-brow|n
            
            
            fox_jump|s ove|r
            th||e"});
        for cursor_offset in cursor_offsets {
            cx.simulate_keystroke("shift-E");
            cx.assert_newest_selection_head_offset(cursor_offset);
        }
    }

    #[gpui::test]
    async fn test_previous_word_start(cx: &mut gpui::TestAppContext) {
        let (initial_content, cursor_offsets) = marked_text(indoc! {"
            ||The |quick|-|brown
            |
            |
            |fox_jumps |over
            |the"});
        let mut cx = VimTestContext::new(cx, true, &initial_content).await;
        cx.simulate_keystroke("shift-G");

        for cursor_offset in cursor_offsets.into_iter().rev() {
            cx.simulate_keystroke("b");
            cx.assert_newest_selection_head_offset(cursor_offset);
        }

        // Reset and test ignoring punctuation
        cx.simulate_keystroke("shift-G");
        let (_, cursor_offsets) = marked_text(indoc! {"
            ||The |quick-brown
            |
            |
            |fox_jumps |over
            |the"});
        for cursor_offset in cursor_offsets.into_iter().rev() {
            cx.simulate_keystroke("shift-B");
            cx.assert_newest_selection_head_offset(cursor_offset);
        }
    }
}
