use crate::{motion::Motion, Vim};
use collections::HashMap;
use editor::{Autoscroll, Bias};
use gpui::MutableAppContext;

pub fn delete_over(vim: &mut Vim, motion: Motion, cx: &mut MutableAppContext) {
    vim.update_active_editor(cx, |editor, cx| {
        editor.transact(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            let mut original_columns: HashMap<_, _> = Default::default();
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    let original_head = selection.head();
                    motion.expand_selection(map, selection, true);
                    original_columns.insert(selection.id, original_head.column());
                });
            });
            editor.insert(&"", cx);

            // Fixup cursor position after the deletion
            editor.set_clip_at_line_ends(true, cx);
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    let mut cursor = selection.head();
                    if motion.linewise() {
                        if let Some(column) = original_columns.get(&selection.id) {
                            *cursor.column_mut() = *column
                        }
                    }
                    cursor = map.clip_point(cursor, Bias::Left);
                    selection.collapse_to(cursor, selection.goal)
                });
            });
        });
    });
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::vim_test_context::VimTestContext;

    #[gpui::test]
    async fn test_delete_h(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "h"]);
        cx.assert("Te|st", "T|st");
        cx.assert("T|est", "|est");
        cx.assert("|Test", "|Test");
        cx.assert(
            indoc! {"
                Test
                |test"},
            indoc! {"
                Test
                |test"},
        );
    }

    #[gpui::test]
    async fn test_delete_l(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "l"]);
        cx.assert("|Test", "|est");
        cx.assert("Te|st", "Te|t");
        cx.assert("Tes|t", "Te|s");
        cx.assert(
            indoc! {"
                Tes|t
                test"},
            indoc! {"
                Te|s
                test"},
        );
    }

    #[gpui::test]
    async fn test_delete_w(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "w"]);
        cx.assert("Te|st", "T|e");
        cx.assert("T|est test", "T|test");
        cx.assert(
            indoc! {"
                Test te|st
                test"},
            indoc! {"
                Test t|e
                test"},
        );
        cx.assert(
            indoc! {"
                Test tes|t
                test"},
            indoc! {"
                Test te|s
                test"},
        );
        cx.assert(
            indoc! {"
                Test test
                |
                test"},
            indoc! {"
                Test test
                |
                test"},
        );

        let mut cx = cx.binding(["d", "shift-W"]);
        cx.assert("Test te|st-test test", "Test te|test");
    }

    #[gpui::test]
    async fn test_delete_e(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "e"]);
        cx.assert("Te|st Test", "Te| Test");
        cx.assert("T|est test", "T| test");
        cx.assert(
            indoc! {"
                Test te|st
                test"},
            indoc! {"
                Test t|e
                test"},
        );
        cx.assert(
            indoc! {"
                Test tes|t
                test"},
            "Test te|s",
        );
        cx.assert(
            indoc! {"
                Test test
                |
                test"},
            indoc! {"
                Test test
                |
                test"},
        );

        let mut cx = cx.binding(["d", "shift-E"]);
        cx.assert("Test te|st-test test", "Test te| test");
    }

    #[gpui::test]
    async fn test_delete_b(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "b"]);
        cx.assert("Te|st Test", "|st Test");
        cx.assert("Test |test", "|test");
        cx.assert("Test1 test2 |test3", "Test1 |test3");
        cx.assert(
            indoc! {"
                Test test
                |test"},
            // Trailing whitespace after cursor
            indoc! {"
                Test| 
                test"},
        );
        cx.assert(
            indoc! {"
                Test test
                |
                test"},
            // Trailing whitespace after cursor
            indoc! {"
                Test| 
                
                test"},
        );

        let mut cx = cx.binding(["d", "shift-B"]);
        cx.assert("Test test-test |test", "Test |test");
    }

    #[gpui::test]
    async fn test_delete_end_of_line(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "shift-$"]);
        cx.assert(
            indoc! {"
                The q|uick
                brown fox"},
            indoc! {"
                The |q
                brown fox"},
        );
        cx.assert(
            indoc! {"
                The quick
                |
                brown fox"},
            indoc! {"
                The quick
                |
                brown fox"},
        );
    }

    #[gpui::test]
    async fn test_delete_0(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "0"]);
        cx.assert(
            indoc! {"
                The q|uick
                brown fox"},
            indoc! {"
                |uick
                brown fox"},
        );
        cx.assert(
            indoc! {"
                The quick
                |
                brown fox"},
            indoc! {"
                The quick
                |
                brown fox"},
        );
    }

    #[gpui::test]
    async fn test_delete_k(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "k"]);
        cx.assert(
            indoc! {"
                The quick
                brown |fox
                jumps over"},
            "jumps |over",
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                jumps |over"},
            "The qu|ick",
        );
        cx.assert(
            indoc! {"
                The q|uick
                brown fox
                jumps over"},
            indoc! {"
                brown| fox
                jumps over"},
        );
        cx.assert(
            indoc! {"
                |brown fox
                jumps over"},
            "|jumps over",
        );
    }

    #[gpui::test]
    async fn test_delete_j(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "j"]);
        cx.assert(
            indoc! {"
                The quick
                brown |fox
                jumps over"},
            "The qu|ick",
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                jumps |over"},
            indoc! {"
                The quick
                brown |fox"},
        );
        cx.assert(
            indoc! {"
                The q|uick
                brown fox
                jumps over"},
            "jumps| over",
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                |"},
            indoc! {"
                The quick
                |brown fox"},
        );
    }

    #[gpui::test]
    async fn test_delete_end_of_document(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "shift-G"]);
        cx.assert(
            indoc! {"
                The quick
                brown| fox
                jumps over
                the lazy"},
            "The q|uick",
        );
        cx.assert(
            indoc! {"
                The quick
                brown| fox
                jumps over
                the lazy"},
            "The q|uick",
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                jumps over
                the l|azy"},
            indoc! {"
                The quick
                brown fox
                jumps| over"},
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                jumps over
                |"},
            indoc! {"
                The quick
                brown fox
                |jumps over"},
        );
    }

    #[gpui::test]
    async fn test_delete_gg(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "g", "g"]);
        cx.assert(
            indoc! {"
                The quick
                brown| fox
                jumps over
                the lazy"},
            indoc! {"
                jumps| over
                the lazy"},
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                jumps over
                the l|azy"},
            "|",
        );
        cx.assert(
            indoc! {"
                The q|uick
                brown fox
                jumps over
                the lazy"},
            indoc! {"
                brown| fox
                jumps over
                the lazy"},
        );
        cx.assert(
            indoc! {"
                |
                brown fox
                jumps over
                the lazy"},
            indoc! {"
                |brown fox
                jumps over
                the lazy"},
        );
    }
}
