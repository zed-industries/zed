use crate::{motion::Motion, object::Object, state::Mode, utils::copy_selections_content, Vim};
use editor::{
    char_kind, display_map::DisplaySnapshot, movement, Autoscroll, CharKind, DisplayPoint,
};
use gpui::MutableAppContext;
use language::Selection;

pub fn change_motion(vim: &mut Vim, motion: Motion, times: usize, cx: &mut MutableAppContext) {
    // Some motions ignore failure when switching to normal mode
    let mut motion_succeeded = matches!(
        motion,
        Motion::Left | Motion::Right | Motion::EndOfLine | Motion::Backspace | Motion::StartOfLine
    );
    vim.update_active_editor(cx, |editor, cx| {
        editor.transact(cx, |editor, cx| {
            // We are swapping to insert mode anyway. Just set the line end clipping behavior now
            editor.set_clip_at_line_ends(false, cx);
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    motion_succeeded |= if let Motion::NextWordStart { ignore_punctuation } = motion
                    {
                        expand_changed_word_selection(map, selection, times, ignore_punctuation)
                    } else {
                        motion.expand_selection(map, selection, times, false)
                    };
                });
            });
            copy_selections_content(editor, motion.linewise(), cx);
            editor.insert("", cx);
        });
    });

    if motion_succeeded {
        vim.switch_mode(Mode::Insert, false, cx)
    } else {
        vim.switch_mode(Mode::Normal, false, cx)
    }
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

// From the docs https://vimdoc.sourceforge.net/htmldoc/motion.html
// Special case: "cw" and "cW" are treated like "ce" and "cE" if the cursor is
// on a non-blank.  This is because "cw" is interpreted as change-word, and a
// word does not include the following white space.  {Vi: "cw" when on a blank
//     followed by other blanks changes only the first blank; this is probably a
//     bug, because "dw" deletes all the blanks}
//
// NOT HANDLED YET
// Another special case: When using the "w" motion in combination with an
// operator and the last word moved over is at the end of a line, the end of
// that word becomes the end of the operated text, not the first word in the
// next line.
fn expand_changed_word_selection(
    map: &DisplaySnapshot,
    selection: &mut Selection<DisplayPoint>,
    times: usize,
    ignore_punctuation: bool,
) -> bool {
    if times == 1 {
        let in_word = map
            .chars_at(selection.head())
            .next()
            .map(|(c, _)| char_kind(c) != CharKind::Whitespace)
            .unwrap_or_default();

        if in_word {
            selection.end = movement::find_boundary(map, selection.end, |left, right| {
                let left_kind = char_kind(left).coerce_punctuation(ignore_punctuation);
                let right_kind = char_kind(right).coerce_punctuation(ignore_punctuation);

                left_kind != right_kind && left_kind != CharKind::Whitespace
            });
            true
        } else {
            Motion::NextWordStart { ignore_punctuation }.expand_selection(map, selection, 1, false)
        }
    } else {
        Motion::NextWordStart { ignore_punctuation }.expand_selection(map, selection, times, false)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::test::{ExemptionFeatures, NeovimBackedTestContext};

    #[gpui::test]
    async fn test_change_h(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["c", "h"]);
        cx.assert("Teˇst").await;
        cx.assert("Tˇest").await;
        cx.assert("ˇTest").await;
        cx.assert(indoc! {"
            Test
            ˇtest"})
            .await;
    }

    #[gpui::test]
    async fn test_change_backspace(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx)
            .await
            .binding(["c", "backspace"]);
        cx.assert("Teˇst").await;
        cx.assert("Tˇest").await;
        cx.assert("ˇTest").await;
        cx.assert(indoc! {"
            Test
            ˇtest"})
            .await;
    }

    #[gpui::test]
    async fn test_change_l(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["c", "l"]);
        cx.assert("Teˇst").await;
        cx.assert("Tesˇt").await;
    }

    #[gpui::test]
    async fn test_change_w(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["c", "w"]);
        cx.assert("Teˇst").await;
        cx.assert("Tˇest test").await;
        cx.assert("Testˇ  test").await;
        cx.assert(indoc! {"
                Test teˇst
                test"})
            .await;
        cx.assert(indoc! {"
                Test tesˇt
                test"})
            .await;
        cx.assert(indoc! {"
                Test test
                ˇ
                test"})
            .await;

        let mut cx = cx.binding(["c", "shift-w"]);
        cx.assert("Test teˇst-test test").await;
    }

    #[gpui::test]
    async fn test_change_e(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["c", "e"]);
        cx.assert("Teˇst Test").await;
        cx.assert("Tˇest test").await;
        cx.assert(indoc! {"
                Test teˇst
                test"})
            .await;
        cx.assert(indoc! {"
                Test tesˇt
                test"})
            .await;
        cx.assert(indoc! {"
                Test test
                ˇ
                test"})
            .await;

        let mut cx = cx.binding(["c", "shift-e"]);
        cx.assert("Test teˇst-test test").await;
    }

    #[gpui::test]
    async fn test_change_b(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["c", "b"]);
        cx.assert("Teˇst Test").await;
        cx.assert("Test ˇtest").await;
        cx.assert("Test1 test2 ˇtest3").await;
        cx.assert(indoc! {"
                Test test
                ˇtest"})
            .await;
        println!("Marker");
        cx.assert(indoc! {"
                Test test
                ˇ
                test"})
            .await;

        let mut cx = cx.binding(["c", "shift-b"]);
        cx.assert("Test test-test ˇtest").await;
    }

    #[gpui::test]
    async fn test_change_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["c", "$"]);
        cx.assert(indoc! {"
            The qˇuick
            brown fox"})
            .await;
        cx.assert(indoc! {"
            The quick
            ˇ
            brown fox"})
            .await;
    }

    #[gpui::test]
    async fn test_change_0(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["c", "0"]);
        cx.assert(indoc! {"
            The qˇuick
            brown fox"})
            .await;
        cx.assert(indoc! {"
            The quick
            ˇ
            brown fox"})
            .await;
    }

    #[gpui::test]
    async fn test_change_k(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["c", "k"]);
        cx.assert(indoc! {"
            The quick
            brown ˇfox
            jumps over"})
            .await;
        cx.assert(indoc! {"
            The quick
            brown fox
            jumps ˇover"})
            .await;
        cx.assert_exempted(
            indoc! {"
            The qˇuick
            brown fox
            jumps over"},
            ExemptionFeatures::OperatorAbortsOnFailedMotion,
        )
        .await;
        cx.assert_exempted(
            indoc! {"
            ˇ
            brown fox
            jumps over"},
            ExemptionFeatures::OperatorAbortsOnFailedMotion,
        )
        .await;
    }

    #[gpui::test]
    async fn test_change_j(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["c", "j"]);
        cx.assert(indoc! {"
            The quick
            brown ˇfox
            jumps over"})
            .await;
        cx.assert_exempted(
            indoc! {"
            The quick
            brown fox
            jumps ˇover"},
            ExemptionFeatures::OperatorAbortsOnFailedMotion,
        )
        .await;
        cx.assert(indoc! {"
            The qˇuick
            brown fox
            jumps over"})
            .await;
        cx.assert_exempted(
            indoc! {"
            The quick
            brown fox
            ˇ"},
            ExemptionFeatures::OperatorAbortsOnFailedMotion,
        )
        .await;
    }

    #[gpui::test]
    async fn test_change_end_of_document(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx)
            .await
            .binding(["c", "shift-g"]);
        cx.assert(indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"})
            .await;
        cx.assert(indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"})
            .await;
        cx.assert_exempted(
            indoc! {"
            The quick
            brown fox
            jumps over
            the lˇazy"},
            ExemptionFeatures::OperatorAbortsOnFailedMotion,
        )
        .await;
        cx.assert_exempted(
            indoc! {"
            The quick
            brown fox
            jumps over
            ˇ"},
            ExemptionFeatures::OperatorAbortsOnFailedMotion,
        )
        .await;
    }

    #[gpui::test]
    async fn test_change_gg(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx)
            .await
            .binding(["c", "g", "g"]);
        cx.assert(indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"})
            .await;
        cx.assert(indoc! {"
            The quick
            brown fox
            jumps over
            the lˇazy"})
            .await;
        cx.assert_exempted(
            indoc! {"
            The qˇuick
            brown fox
            jumps over
            the lazy"},
            ExemptionFeatures::OperatorAbortsOnFailedMotion,
        )
        .await;
        cx.assert_exempted(
            indoc! {"
            ˇ
            brown fox
            jumps over
            the lazy"},
            ExemptionFeatures::OperatorAbortsOnFailedMotion,
        )
        .await;
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

        cx.add_initial_state_exemptions(
            indoc! {"
            ˇThe quick brown

            fox jumps-over
            the lazy dog
            "},
            ExemptionFeatures::OperatorAbortsOnFailedMotion,
        );

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
