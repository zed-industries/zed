use crate::{
    motion::Motion,
    state::{Mode, Operator},
    Vim,
};
use editor::Bias;
use gpui::MutableAppContext;
use language::SelectionGoal;

pub fn normal_motion(motion: Motion, cx: &mut MutableAppContext) {
    Vim::update(cx, |vim, cx| {
        match vim.state.operator_stack.pop() {
            None => move_cursor(vim, motion, cx),
            Some(Operator::Change) => change_over(vim, motion, cx),
            Some(Operator::Delete) => delete_over(vim, motion, cx),
            Some(Operator::Namespace(_)) => panic!(
                "Normal mode recieved motion with namespaced operator. Likely this means an invalid keymap was used"),
        }
        vim.clear_operator(cx);
    });
}

fn move_cursor(vim: &mut Vim, motion: Motion, cx: &mut MutableAppContext) {
    vim.update_active_editor(cx, |editor, cx| {
        editor.move_cursors(cx, |map, cursor, goal| motion.move_point(map, cursor, goal))
    });
}

fn change_over(vim: &mut Vim, motion: Motion, cx: &mut MutableAppContext) {
    vim.update_active_editor(cx, |editor, cx| {
        editor.transact(cx, |editor, cx| {
            // Don't clip at line ends during change operation
            editor.set_clip_at_line_ends(false, cx);
            editor.move_selections(cx, |map, selection| motion.expand_selection(map, selection));
            editor.set_clip_at_line_ends(true, cx);
            editor.insert(&"", cx);
        });
    });
    vim.switch_mode(Mode::Insert, cx)
}

fn delete_over(vim: &mut Vim, motion: Motion, cx: &mut MutableAppContext) {
    vim.update_active_editor(cx, |editor, cx| {
        editor.transact(cx, |editor, cx| {
            // Don't clip at line ends during delete operation
            editor.set_clip_at_line_ends(false, cx);
            editor.move_selections(cx, |map, selection| motion.expand_selection(map, selection));
            editor.insert(&"", cx);

            // Fixup cursor position after the deletion
            editor.set_clip_at_line_ends(true, cx);
            editor.move_selection_heads(cx, |map, head, _| {
                (map.clip_point(head, Bias::Left), SelectionGoal::None)
            });
        });
    });
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use util::test::marked_text;

    use crate::{
        state::{
            Mode::{self, *},
            Namespace, Operator,
        },
        vim_test_context::VimTestContext,
    };

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
        cx.simulate_keystrokes(["g", "g"]);
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
        cx.simulate_keystrokes(["g", "g"]);
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

    #[gpui::test]
    async fn test_g_prefix_and_abort(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true, "").await;

        // Can abort with escape to get back to normal mode
        cx.simulate_keystroke("g");
        assert_eq!(cx.mode(), Normal);
        assert_eq!(
            cx.active_operator(),
            Some(Operator::Namespace(Namespace::G))
        );
        cx.simulate_keystroke("escape");
        assert_eq!(cx.mode(), Normal);
        assert_eq!(cx.active_operator(), None);
    }

    #[gpui::test]
    async fn test_move_to_start(cx: &mut gpui::TestAppContext) {
        let initial_content = indoc! {"
            The quick
            
            brown fox jumps
            over the lazy dog"};
        let mut cx = VimTestContext::new(cx, true, initial_content).await;

        // Jump to the end to
        cx.simulate_keystroke("shift-G");
        cx.assert_editor_state(indoc! {"
            The quick
            
            brown fox jumps
            over the lazy do|g"});

        // Jump to the start
        cx.simulate_keystrokes(["g", "g"]);
        cx.assert_editor_state(indoc! {"
            |The quick
            
            brown fox jumps
            over the lazy dog"});
        assert_eq!(cx.mode(), Normal);
        assert_eq!(cx.active_operator(), None);

        // Repeat action doesn't change
        cx.simulate_keystrokes(["g", "g"]);
        cx.assert_editor_state(indoc! {"
            |The quick
            
            brown fox jumps
            over the lazy dog"});
        assert_eq!(cx.mode(), Normal);
        assert_eq!(cx.active_operator(), None);
    }

    #[gpui::test]
    async fn test_change(cx: &mut gpui::TestAppContext) {
        fn assert(motion: &str, initial_state: &str, state_after: &str, cx: &mut VimTestContext) {
            cx.assert_binding(
                ["c", motion],
                initial_state,
                Mode::Normal,
                state_after,
                Mode::Insert,
            );
        }
        let cx = &mut VimTestContext::new(cx, true, "").await;
        assert("h", "Te|st", "T|st", cx);
        assert("l", "Te|st", "Te|t", cx);
        assert("w", "|Test", "|", cx);
        assert("w", "Te|st", "Te|", cx);
        assert("w", "Te|st Test", "Te| Test", cx);
        assert("e", "Te|st Test", "Te| Test", cx);
        assert("b", "Te|st", "|st", cx);
        assert("b", "Test Te|st", "Test |st", cx);
        assert(
            "w",
            indoc! {"
            The quick
            brown |fox
            jumps over"},
            indoc! {"
            The quick
            brown |
            jumps over"},
            cx,
        );
        assert(
            "shift-W",
            indoc! {"
            The quick
            brown |fox-fox
            jumps over"},
            indoc! {"
            The quick
            brown |
            jumps over"},
            cx,
        );
        assert(
            "k",
            indoc! {"
            The quick
            brown |fox"},
            indoc! {"
            |"},
            cx,
        );
        assert(
            "j",
            indoc! {"
            The q|uick
            brown fox"},
            indoc! {"
            |"},
            cx,
        );
        assert(
            "shift-$",
            indoc! {"
            The q|uick
            brown fox"},
            indoc! {"
            The q|
            brown fox"},
            cx,
        );
        assert(
            "0",
            indoc! {"
            The q|uick
            brown fox"},
            indoc! {"
            |uick
            brown fox"},
            cx,
        );
    }

    #[gpui::test]
    async fn test_delete(cx: &mut gpui::TestAppContext) {
        fn assert(motion: &str, initial_state: &str, state_after: &str, cx: &mut VimTestContext) {
            cx.assert_binding(
                ["d", motion],
                initial_state,
                Mode::Normal,
                state_after,
                Mode::Normal,
            );
        }
        let cx = &mut VimTestContext::new(cx, true, "").await;
        assert("h", "Te|st", "T|st", cx);
        assert("l", "Te|st", "Te|t", cx);
        assert("w", "|Test", "|", cx);
        assert("w", "Te|st", "T|e", cx);
        assert("w", "Te|st Test", "Te|Test", cx);
        assert("e", "Te|st Test", "Te| Test", cx);
        assert("b", "Te|st", "|st", cx);
        assert("b", "Test Te|st", "Test |st", cx);
        assert(
            "w",
            indoc! {"
            The quick
            brown |fox
            jumps over"},
            // Trailing space after cursor
            indoc! {"
            The quick
            brown| 
            jumps over"},
            cx,
        );
        assert(
            "shift-W",
            indoc! {"
            The quick
            brown |fox-fox
            jumps over"},
            // Trailing space after cursor
            indoc! {"
            The quick
            brown| 
            jumps over"},
            cx,
        );
        assert(
            "k",
            indoc! {"
            The quick
            brown |fox"},
            indoc! {"
            |"},
            cx,
        );
        assert(
            "j",
            indoc! {"
            The q|uick
            brown fox"},
            indoc! {"
            |"},
            cx,
        );
        assert(
            "shift-$",
            indoc! {"
            The q|uick
            brown fox"},
            indoc! {"
            The |q
            brown fox"},
            cx,
        );
        assert(
            "0",
            indoc! {"
            The q|uick
            brown fox"},
            indoc! {"
            |uick
            brown fox"},
            cx,
        );
    }
}
