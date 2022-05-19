use crate::{motion::Motion, state::Mode, Vim};
use editor::{char_kind, movement, Autoscroll, ClipboardSelection};
use gpui::{impl_actions, ClipboardItem, MutableAppContext, ViewContext};
use serde::Deserialize;
use workspace::Workspace;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChangeWord {
    #[serde(default)]
    ignore_punctuation: bool,
}

impl_actions!(vim, [ChangeWord]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(change_word);
}

pub fn change_over(vim: &mut Vim, motion: Motion, cx: &mut MutableAppContext) {
    vim.update_active_editor(cx, |editor, cx| {
        editor.transact(cx, |editor, cx| {
            // We are swapping to insert mode anyway. Just set the line end clipping behavior now
            editor.set_clip_at_line_ends(false, cx);
            let mut text = String::new();
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let mut clipboard_selections = Vec::with_capacity(editor.selections.count());
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    motion.expand_selection(map, selection, false);
                    let mut len = 0;
                    let range = selection.start.to_point(map)..selection.end.to_point(map);
                    for chunk in buffer.text_for_range(range) {
                        text.push_str(chunk);
                        len += chunk.len();
                    }
                    clipboard_selections.push(ClipboardSelection {
                        len,
                        is_entire_line: motion.linewise(),
                    });
                });
            });
            editor.insert(&"", cx);
            cx.write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
        });
    });
    vim.switch_mode(Mode::Insert, cx)
}

// From the docs https://vimhelp.org/change.txt.html#cw
// Special case: When the cursor is in a word, "cw" and "cW" do not include the
// white space after a word, they only change up to the end of the word. This is
// because Vim interprets "cw" as change-word, and a word does not include the
// following white space.
fn change_word(
    _: &mut Workspace,
    &ChangeWord { ignore_punctuation }: &ChangeWord,
    cx: &mut ViewContext<Workspace>,
) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |editor, cx| {
            editor.transact(cx, |editor, cx| {
                // We are swapping to insert mode anyway. Just set the line end clipping behavior now
                editor.set_clip_at_line_ends(false, cx);
                editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                    s.move_with(|map, selection| {
                        if selection.end.column() == map.line_len(selection.end.row()) {
                            return;
                        }

                        selection.end =
                            movement::find_boundary(map, selection.end, |left, right| {
                                let left_kind =
                                    char_kind(left).coerce_punctuation(ignore_punctuation);
                                let right_kind =
                                    char_kind(right).coerce_punctuation(ignore_punctuation);

                                left_kind != right_kind || left == '\n' || right == '\n'
                            });
                    });
                });
                editor.insert(&"", cx);
            });
        });
        vim.switch_mode(Mode::Insert, cx);
    });
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{state::Mode, vim_test_context::VimTestContext};

    #[gpui::test]
    async fn test_change_h(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "h"]).mode_after(Mode::Insert);
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
    async fn test_change_l(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "l"]).mode_after(Mode::Insert);
        cx.assert("Te|st", "Te|t");
        cx.assert("Tes|t", "Tes|");
    }

    #[gpui::test]
    async fn test_change_w(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "w"]).mode_after(Mode::Insert);
        cx.assert("Te|st", "Te|");
        cx.assert("T|est test", "T| test");
        cx.assert("Test|  test", "Test|test");
        cx.assert(
            indoc! {"
                Test te|st
                test"},
            indoc! {"
                Test te|
                test"},
        );
        cx.assert(
            indoc! {"
                Test tes|t
                test"},
            indoc! {"
                Test tes|
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

        let mut cx = cx.binding(["c", "shift-W"]);
        cx.assert("Test te|st-test test", "Test te| test");
    }

    #[gpui::test]
    async fn test_change_e(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "e"]).mode_after(Mode::Insert);
        cx.assert("Te|st Test", "Te| Test");
        cx.assert("T|est test", "T| test");
        cx.assert(
            indoc! {"
                Test te|st
                test"},
            indoc! {"
                Test te|
                test"},
        );
        cx.assert(
            indoc! {"
                Test tes|t
                test"},
            "Test tes|",
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

        let mut cx = cx.binding(["c", "shift-E"]);
        cx.assert("Test te|st-test test", "Test te| test");
    }

    #[gpui::test]
    async fn test_change_b(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "b"]).mode_after(Mode::Insert);
        cx.assert("Te|st Test", "|st Test");
        cx.assert("Test |test", "|test");
        cx.assert("Test1 test2 |test3", "Test1 |test3");
        cx.assert(
            indoc! {"
                Test test
                |test"},
            indoc! {"
                Test |
                test"},
        );
        cx.assert(
            indoc! {"
                Test test
                |
                test"},
            indoc! {"
                Test |
                
                test"},
        );

        let mut cx = cx.binding(["c", "shift-B"]);
        cx.assert("Test test-test |test", "Test |test");
    }

    #[gpui::test]
    async fn test_change_end_of_line(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "shift-$"]).mode_after(Mode::Insert);
        cx.assert(
            indoc! {"
                The q|uick
                brown fox"},
            indoc! {"
                The q|
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
    async fn test_change_0(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "0"]).mode_after(Mode::Insert);
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
    async fn test_change_k(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "k"]).mode_after(Mode::Insert);
        cx.assert(
            indoc! {"
                The quick
                brown |fox
                jumps over"},
            indoc! {"
                |
                jumps over"},
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                jumps |over"},
            indoc! {"
                The quick
                |"},
        );
        cx.assert(
            indoc! {"
                The q|uick
                brown fox
                jumps over"},
            indoc! {"
                |
                brown fox
                jumps over"},
        );
        cx.assert(
            indoc! {"
                |
                brown fox
                jumps over"},
            indoc! {"
                |
                brown fox
                jumps over"},
        );
    }

    #[gpui::test]
    async fn test_change_j(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "j"]).mode_after(Mode::Insert);
        cx.assert(
            indoc! {"
                The quick
                brown |fox
                jumps over"},
            indoc! {"
                The quick
                |"},
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                jumps |over"},
            indoc! {"
                The quick
                brown fox
                |"},
        );
        cx.assert(
            indoc! {"
                The q|uick
                brown fox
                jumps over"},
            indoc! {"
                |
                jumps over"},
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                |"},
            indoc! {"
                The quick
                brown fox
                |"},
        );
    }

    #[gpui::test]
    async fn test_change_end_of_document(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "shift-G"]).mode_after(Mode::Insert);
        cx.assert(
            indoc! {"
                The quick
                brown| fox
                jumps over
                the lazy"},
            indoc! {"
                The quick
                |"},
        );
        cx.assert(
            indoc! {"
                The quick
                brown| fox
                jumps over
                the lazy"},
            indoc! {"
                The quick
                |"},
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
                jumps over
                |"},
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
                jumps over
                |"},
        );
    }

    #[gpui::test]
    async fn test_change_gg(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "g", "g"]).mode_after(Mode::Insert);
        cx.assert(
            indoc! {"
                The quick
                brown| fox
                jumps over
                the lazy"},
            indoc! {"
                |
                jumps over
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
                |
                brown fox
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
                |
                brown fox
                jumps over
                the lazy"},
        );
    }
}
