use crate::{motion::Motion, object::Object, state::Mode, utils::copy_selections_content, Vim};
use editor::{char_kind, display_map::DisplaySnapshot, movement, Autoscroll, Bias, DisplayPoint};
use gpui::MutableAppContext;
use language::Selection;

pub fn change_motion(vim: &mut Vim, motion: Motion, times: usize, cx: &mut MutableAppContext) {
    vim.update_active_editor(cx, |editor, cx| {
        editor.transact(cx, |editor, cx| {
            // We are swapping to insert mode anyway. Just set the line end clipping behavior now
            editor.set_clip_at_line_ends(false, cx);
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    if let Motion::NextWordStart { ignore_punctuation } = motion {
                        expand_changed_word_selection(map, selection, times, ignore_punctuation);
                    } else {
                        motion.expand_selection(map, selection, times, false);
                    }
                });
            });
            copy_selections_content(editor, motion.linewise(), cx);
            editor.insert("", cx);
        });
    });
    vim.switch_mode(Mode::Insert, false, cx)
}

pub fn change_object(vim: &mut Vim, object: Object, around: bool, cx: &mut MutableAppContext) {
    let mut objects_found = false;
    vim.update_active_editor(cx, |editor, cx| {
        // We are swapping to insert mode anyway. Just set the line end clipping behavior now
        editor.set_clip_at_line_ends(false, cx);
        editor.transact(cx, |editor, cx| {
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    objects_found |= object.expand_selection(map, selection, around);
                });
            });
            if objects_found {
                copy_selections_content(editor, false, cx);
                editor.insert("", cx);
            }
        });
    });

    if objects_found {
        vim.switch_mode(Mode::Insert, false, cx);
    } else {
        vim.switch_mode(Mode::Normal, false, cx);
    }
}

// From the docs https://vimhelp.org/change.txt.html#cw
// Special case: When the cursor is in a word, "cw" and "cW" do not include the
// white space after a word, they only change up to the end of the word. This is
// because Vim interprets "cw" as change-word, and a word does not include the
// following white space.
fn expand_changed_word_selection(
    map: &DisplaySnapshot,
    selection: &mut Selection<DisplayPoint>,
    times: usize,
    ignore_punctuation: bool,
) {
    if times > 1 {
        Motion::NextWordStart { ignore_punctuation }.expand_selection(
            map,
            selection,
            times - 1,
            false,
        );
    }

    if times == 1 && selection.end.column() == map.line_len(selection.end.row()) {
        return;
    }

    selection.end = movement::find_boundary(map, selection.end, |left, right| {
        let left_kind = char_kind(left).coerce_punctuation(ignore_punctuation);
        let right_kind = char_kind(right).coerce_punctuation(ignore_punctuation);

        left_kind != right_kind || left == '\n' || right == '\n'
    });
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };

    #[gpui::test]
    async fn test_change_h(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "h"]).mode_after(Mode::Insert);
        cx.assert("Teˇst", "Tˇst");
        cx.assert("Tˇest", "ˇest");
        cx.assert("ˇTest", "ˇTest");
        cx.assert(
            indoc! {"
                Test
                ˇtest"},
            indoc! {"
                Test
                ˇtest"},
        );
    }

    #[gpui::test]
    async fn test_change_l(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "l"]).mode_after(Mode::Insert);
        cx.assert("Teˇst", "Teˇt");
        cx.assert("Tesˇt", "Tesˇ");
    }

    #[gpui::test]
    async fn test_change_w(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "w"]).mode_after(Mode::Insert);
        cx.assert("Teˇst", "Teˇ");
        cx.assert("Tˇest test", "Tˇ test");
        cx.assert("Testˇ  test", "Testˇtest");
        cx.assert(
            indoc! {"
                Test teˇst
                test"},
            indoc! {"
                Test teˇ
                test"},
        );
        cx.assert(
            indoc! {"
                Test tesˇt
                test"},
            indoc! {"
                Test tesˇ
                test"},
        );
        cx.assert(
            indoc! {"
                Test test
                ˇ
                test"},
            indoc! {"
                Test test
                ˇ
                test"},
        );

        let mut cx = cx.binding(["c", "shift-w"]);
        cx.assert("Test teˇst-test test", "Test teˇ test");
    }

    #[gpui::test]
    async fn test_change_e(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "e"]).mode_after(Mode::Insert);
        cx.assert("Teˇst Test", "Teˇ Test");
        cx.assert("Tˇest test", "Tˇ test");
        cx.assert(
            indoc! {"
                Test teˇst
                test"},
            indoc! {"
                Test teˇ
                test"},
        );
        cx.assert(
            indoc! {"
                Test tesˇt
                test"},
            "Test tesˇ",
        );
        cx.assert(
            indoc! {"
                Test test
                ˇ
                test"},
            indoc! {"
                Test test
                ˇ"},
        );

        let mut cx = cx.binding(["c", "shift-e"]);
        cx.assert("Test teˇst-test test", "Test teˇ test");
    }

    #[gpui::test]
    async fn test_change_b(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "b"]).mode_after(Mode::Insert);
        cx.assert("Teˇst Test", "ˇst Test");
        cx.assert("Test ˇtest", "ˇtest");
        cx.assert("Test1 test2 ˇtest3", "Test1 ˇtest3");
        cx.assert(
            indoc! {"
                Test test
                ˇtest"},
            indoc! {"
                Test ˇ
                test"},
        );
        println!("Marker");
        cx.assert(
            indoc! {"
                Test test
                ˇ
                test"},
            indoc! {"
                Test ˇ
                
                test"},
        );

        let mut cx = cx.binding(["c", "shift-b"]);
        cx.assert("Test test-test ˇtest", "Test ˇtest");
    }

    #[gpui::test]
    async fn test_change_end_of_line(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "$"]).mode_after(Mode::Insert);
        cx.assert(
            indoc! {"
                The qˇuick
                brown fox"},
            indoc! {"
                The qˇ
                brown fox"},
        );
        cx.assert(
            indoc! {"
                The quick
                ˇ
                brown fox"},
            indoc! {"
                The quick
                ˇ
                brown fox"},
        );
    }

    #[gpui::test]
    async fn test_change_0(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "0"]).mode_after(Mode::Insert);
        cx.assert(
            indoc! {"
                The qˇuick
                brown fox"},
            indoc! {"
                ˇuick
                brown fox"},
        );
        cx.assert(
            indoc! {"
                The quick
                ˇ
                brown fox"},
            indoc! {"
                The quick
                ˇ
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
                brown ˇfox
                jumps over"},
            indoc! {"
                ˇ
                jumps over"},
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                jumps ˇover"},
            indoc! {"
                The quick
                ˇ"},
        );
        cx.assert(
            indoc! {"
                The qˇuick
                brown fox
                jumps over"},
            indoc! {"
                ˇ
                brown fox
                jumps over"},
        );
        cx.assert(
            indoc! {"
                ˇ
                brown fox
                jumps over"},
            indoc! {"
                ˇ
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
                brown ˇfox
                jumps over"},
            indoc! {"
                The quick
                ˇ"},
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                jumps ˇover"},
            indoc! {"
                The quick
                brown fox
                ˇ"},
        );
        cx.assert(
            indoc! {"
                The qˇuick
                brown fox
                jumps over"},
            indoc! {"
                ˇ
                jumps over"},
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                ˇ"},
            indoc! {"
                The quick
                brown fox
                ˇ"},
        );
    }

    #[gpui::test]
    async fn test_change_end_of_document(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "shift-g"]).mode_after(Mode::Insert);
        cx.assert(
            indoc! {"
                The quick
                brownˇ fox
                jumps over
                the lazy"},
            indoc! {"
                The quick
                ˇ"},
        );
        cx.assert(
            indoc! {"
                The quick
                brownˇ fox
                jumps over
                the lazy"},
            indoc! {"
                The quick
                ˇ"},
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                jumps over
                the lˇazy"},
            indoc! {"
                The quick
                brown fox
                jumps over
                ˇ"},
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                jumps over
                ˇ"},
            indoc! {"
                The quick
                brown fox
                jumps over
                ˇ"},
        );
    }

    #[gpui::test]
    async fn test_change_gg(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "g", "g"]).mode_after(Mode::Insert);
        cx.assert(
            indoc! {"
                The quick
                brownˇ fox
                jumps over
                the lazy"},
            indoc! {"
                ˇ
                jumps over
                the lazy"},
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                jumps over
                the lˇazy"},
            "ˇ",
        );
        cx.assert(
            indoc! {"
                The qˇuick
                brown fox
                jumps over
                the lazy"},
            indoc! {"
                ˇ
                brown fox
                jumps over
                the lazy"},
        );
        cx.assert(
            indoc! {"
                ˇ
                brown fox
                jumps over
                the lazy"},
            indoc! {"
                ˇ
                brown fox
                jumps over
                the lazy"},
        );
    }

    #[gpui::test]
    async fn test_repeated_cj(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        for count in 1..=5 {
            cx.assert_binding_matches_all(
                ["c", &count.to_string(), "j"],
                indoc! {"
                    ˇThe quˇickˇ browˇn
                    ˇ
                    ˇfox ˇjumpsˇ-ˇoˇver
                    ˇthe lazy dog
                    "},
            )
            .await;
        }
    }

    #[gpui::test]
    async fn test_repeated_cl(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        for count in 1..=5 {
            cx.assert_binding_matches_all(
                ["c", &count.to_string(), "l"],
                indoc! {"
                    ˇThe quˇickˇ browˇn
                    ˇ
                    ˇfox ˇjumpsˇ-ˇoˇver
                    ˇthe lazy dog
                    "},
            )
            .await;
        }
    }

    #[gpui::test]
    async fn test_repeated_cb(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        // Changing back any number of times from the start of the file doesn't
        // switch to insert mode in vim. This is weird and painful to implement
        cx.add_initial_state_exemption(indoc! {"
            ˇThe quick brown
            
            fox jumps-over
            the lazy dog
            "});

        for count in 1..=5 {
            cx.assert_binding_matches_all(
                ["c", &count.to_string(), "b"],
                indoc! {"
                    ˇThe quˇickˇ browˇn
                    ˇ
                    ˇfox ˇjumpsˇ-ˇoˇver
                    ˇthe lazy dog
                    "},
            )
            .await;
        }
    }

    #[gpui::test]
    async fn test_repeated_ce(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        for count in 1..=5 {
            cx.assert_binding_matches_all(
                ["c", &count.to_string(), "e"],
                indoc! {"
                    ˇThe quˇickˇ browˇn
                    ˇ
                    ˇfox ˇjumpsˇ-ˇoˇver
                    ˇthe lazy dog
                    "},
            )
            .await;
        }
    }
}
