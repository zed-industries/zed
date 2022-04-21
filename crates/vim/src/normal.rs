mod change;
mod delete;

use crate::{motion::Motion, state::Operator, Vim};
use change::init as change_init;
use gpui::{actions, MutableAppContext};

use self::{change::change_over, delete::delete_over};

actions!(vim, [InsertLineAbove, InsertLineBelow, InsertAfter]);

pub fn init(cx: &mut MutableAppContext) {
    change_init(cx);
}

pub fn normal_motion(motion: Motion, cx: &mut MutableAppContext) {
    Vim::update(cx, |vim, cx| {
        match vim.state.operator_stack.pop() {
            None => move_cursor(vim, motion, cx),
            Some(Operator::Change) => change_over(vim, motion, cx),
            Some(Operator::Delete) => delete_over(vim, motion, cx),
            Some(Operator::Namespace(_)) => {
                // Can't do anything for a namespace operator. Ignoring
            }
        }
        vim.clear_operator(cx);
    });
}

fn move_cursor(vim: &mut Vim, motion: Motion, cx: &mut MutableAppContext) {
    vim.update_active_editor(cx, |editor, cx| {
        editor.move_cursors(cx, |map, cursor, goal| motion.move_point(map, cursor, goal))
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
    async fn test_h(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["h"]);
        cx.assert("The q|uick", "The |quick");
        cx.assert("|The quick", "|The quick");
        cx.assert(
            indoc! {"
                The quick
                |brown"},
            indoc! {"
                The quick
                |brown"},
        );
    }

    #[gpui::test]
    async fn test_l(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["l"]);
        cx.assert("The q|uick", "The qu|ick");
        cx.assert("The quic|k", "The quic|k");
        cx.assert(
            indoc! {"
                The quic|k
                brown"},
            indoc! {"
                The quic|k
                brown"},
        );
    }

    #[gpui::test]
    async fn test_j(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["j"]);
        cx.assert(
            indoc! {"
                The |quick
                brown fox"},
            indoc! {"
                The quick
                brow|n fox"},
        );
        cx.assert(
            indoc! {"
                The quick
                brow|n fox"},
            indoc! {"
                The quick
                brow|n fox"},
        );
        cx.assert(
            indoc! {"
                The quic|k
                brown"},
            indoc! {"
                The quick
                brow|n"},
        );
        cx.assert(
            indoc! {"
                The quick
                |brown"},
            indoc! {"
                The quick
                |brown"},
        );
    }

    #[gpui::test]
    async fn test_k(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["k"]);
        cx.assert(
            indoc! {"
                The |quick
                brown fox"},
            indoc! {"
                The |quick
                brown fox"},
        );
        cx.assert(
            indoc! {"
                The quick
                brow|n fox"},
            indoc! {"
                The |quick
                brown fox"},
        );
        cx.assert(
            indoc! {"
                The
                quic|k"},
            indoc! {"
                Th|e
                quick"},
        );
    }

    #[gpui::test]
    async fn test_jump_to_line_boundaries(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["shift-$"]);
        cx.assert("T|est test", "Test tes|t");
        cx.assert("Test tes|t", "Test tes|t");
        cx.assert(
            indoc! {"
                The |quick
                brown"},
            indoc! {"
                The quic|k
                brown"},
        );
        cx.assert(
            indoc! {"
                The quic|k
                brown"},
            indoc! {"
                The quic|k
                brown"},
        );

        let mut cx = cx.binding(["0"]);
        cx.assert("Test |test", "|Test test");
        cx.assert("|Test test", "|Test test");
        cx.assert(
            indoc! {"
                The |quick
                brown"},
            indoc! {"
                |The quick
                brown"},
        );
        cx.assert(
            indoc! {"
                |The quick
                brown"},
            indoc! {"
                |The quick
                brown"},
        );
    }

    #[gpui::test]
    async fn test_jump_to_end(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["shift-G"]);

        cx.assert(
            indoc! {"
                The |quick
                
                brown fox jumps
                over the lazy dog"},
            indoc! {"
                The quick
                
                brown fox jumps
                over| the lazy dog"},
        );
        cx.assert(
            indoc! {"
                The quick
                
                brown fox jumps
                over| the lazy dog"},
            indoc! {"
                The quick
                
                brown fox jumps
                over| the lazy dog"},
        );
        cx.assert(
            indoc! {"
            The qui|ck
            
            brown"},
            indoc! {"
            The quick
            
            brow|n"},
        );
        cx.assert(
            indoc! {"
            The qui|ck
            
            "},
            indoc! {"
            The quick
            
            |"},
        );
    }

    #[gpui::test]
    async fn test_next_word_start(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        let (_, cursor_offsets) = marked_text(indoc! {"
            The |quick|-|brown
            |
            |
            |fox_jumps |over
            |th||e"});
        cx.set_state(
            indoc! {"
            |The quick-brown
            
            
            fox_jumps over
            the"},
            Mode::Normal,
        );

        for cursor_offset in cursor_offsets {
            cx.simulate_keystroke("w");
            cx.assert_newest_selection_head_offset(cursor_offset);
        }

        // Reset and test ignoring punctuation
        let (_, cursor_offsets) = marked_text(indoc! {"
            The |quick-brown
            |
            |
            |fox_jumps |over
            |th||e"});
        cx.set_state(
            indoc! {"
            |The quick-brown
            
            
            fox_jumps over
            the"},
            Mode::Normal,
        );

        for cursor_offset in cursor_offsets {
            cx.simulate_keystroke("shift-W");
            cx.assert_newest_selection_head_offset(cursor_offset);
        }
    }

    #[gpui::test]
    async fn test_next_word_end(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        let (_, cursor_offsets) = marked_text(indoc! {"
            Th|e quic|k|-brow|n
            
            
            fox_jump|s ove|r
            th|e"});
        cx.set_state(
            indoc! {"
            |The quick-brown
            
            
            fox_jumps over
            the"},
            Mode::Normal,
        );

        for cursor_offset in cursor_offsets {
            cx.simulate_keystroke("e");
            cx.assert_newest_selection_head_offset(cursor_offset);
        }

        // Reset and test ignoring punctuation
        let (_, cursor_offsets) = marked_text(indoc! {"
            Th|e quick-brow|n
            
            
            fox_jump|s ove|r
            th||e"});
        cx.set_state(
            indoc! {"
            |The quick-brown
            
            
            fox_jumps over
            the"},
            Mode::Normal,
        );
        for cursor_offset in cursor_offsets {
            cx.simulate_keystroke("shift-E");
            cx.assert_newest_selection_head_offset(cursor_offset);
        }
    }

    #[gpui::test]
    async fn test_previous_word_start(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        let (_, cursor_offsets) = marked_text(indoc! {"
            ||The |quick|-|brown
            |
            |
            |fox_jumps |over
            |the"});
        cx.set_state(
            indoc! {"
            The quick-brown
            
            
            fox_jumps over
            th|e"},
            Mode::Normal,
        );

        for cursor_offset in cursor_offsets.into_iter().rev() {
            cx.simulate_keystroke("b");
            cx.assert_newest_selection_head_offset(cursor_offset);
        }

        // Reset and test ignoring punctuation
        let (_, cursor_offsets) = marked_text(indoc! {"
            ||The |quick-brown
            |
            |
            |fox_jumps |over
            |the"});
        cx.set_state(
            indoc! {"
            The quick-brown
            
            
            fox_jumps over
            th|e"},
            Mode::Normal,
        );
        for cursor_offset in cursor_offsets.into_iter().rev() {
            cx.simulate_keystroke("shift-B");
            cx.assert_newest_selection_head_offset(cursor_offset);
        }
    }

    #[gpui::test]
    async fn test_g_prefix_and_abort(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

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
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["g", "g"]);
        cx.assert(
            indoc! {"
                The quick
            
                brown fox jumps
                over |the lazy dog"},
            indoc! {"
                The q|uick
            
                brown fox jumps
                over the lazy dog"},
        );
        cx.assert(
            indoc! {"
                The q|uick
            
                brown fox jumps
                over the lazy dog"},
            indoc! {"
                The q|uick
            
                brown fox jumps
                over the lazy dog"},
        );
        cx.assert(
            indoc! {"
                The quick
            
                brown fox jumps
                over the la|zy dog"},
            indoc! {"
                The quic|k
            
                brown fox jumps
                over the lazy dog"},
        );
        cx.assert(
            indoc! {"
                
            
                brown fox jumps
                over the la|zy dog"},
            indoc! {"
                |
            
                brown fox jumps
                over the lazy dog"},
        );
    }
}
