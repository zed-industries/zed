use crate::{
    motion::{self, Motion},
    object::Object,
    state::Mode,
    utils::copy_selections_content,
    Vim,
};
use editor::{
    display_map::DisplaySnapshot, movement::TextLayoutDetails, scroll::Autoscroll, DisplayPoint,
};
use gpui::WindowContext;
use language::{char_kind, CharKind, Selection};

pub fn change_motion(vim: &mut Vim, motion: Motion, times: Option<usize>, cx: &mut WindowContext) {
    // Some motions ignore failure when switching to normal mode
    let mut motion_succeeded = matches!(
        motion,
        Motion::Left
            | Motion::Right
            | Motion::EndOfLine { .. }
            | Motion::Backspace
            | Motion::StartOfLine { .. }
    );
    vim.update_active_editor(cx, |vim, editor, cx| {
        let text_layout_details = editor.text_layout_details(cx);
        editor.transact(cx, |editor, cx| {
            // We are swapping to insert mode anyway. Just set the line end clipping behavior now
            editor.set_clip_at_line_ends(false, cx);
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    motion_succeeded |= if let Motion::NextWordStart { ignore_punctuation } = motion
                    {
                        expand_changed_word_selection(
                            map,
                            selection,
                            times,
                            ignore_punctuation,
                            &text_layout_details,
                            false,
                        )
                    } else if let Motion::NextSubwordStart { ignore_punctuation } = motion {
                        expand_changed_word_selection(
                            map,
                            selection,
                            times,
                            ignore_punctuation,
                            &text_layout_details,
                            true,
                        )
                    } else {
                        let result = motion.expand_selection(
                            map,
                            selection,
                            times,
                            false,
                            &text_layout_details,
                        );
                        if let Motion::CurrentLine = motion {
                            let scope = map
                                .buffer_snapshot
                                .language_scope_at(selection.start.to_point(&map));
                            for (ch, _) in map.chars_at(selection.start) {
                                if ch == '\n' || char_kind(&scope, ch) != CharKind::Whitespace {
                                    break;
                                }
                                *selection.start.column_mut() += 1;
                            }
                        }
                        result
                    };
                });
            });
            copy_selections_content(vim, editor, motion.linewise(), cx);
            editor.insert("", cx);
        });
    });

    if motion_succeeded {
        vim.switch_mode(Mode::Insert, false, cx)
    } else {
        vim.switch_mode(Mode::Normal, false, cx)
    }
}

pub fn change_object(vim: &mut Vim, object: Object, around: bool, cx: &mut WindowContext) {
    let mut objects_found = false;
    vim.update_active_editor(cx, |vim, editor, cx| {
        // We are swapping to insert mode anyway. Just set the line end clipping behavior now
        editor.set_clip_at_line_ends(false, cx);
        editor.transact(cx, |editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    objects_found |= object.expand_selection(map, selection, around);
                });
            });
            if objects_found {
                copy_selections_content(vim, editor, false, cx);
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
fn expand_changed_word_selection(
    map: &DisplaySnapshot,
    selection: &mut Selection<DisplayPoint>,
    times: Option<usize>,
    ignore_punctuation: bool,
    text_layout_details: &TextLayoutDetails,
    use_subword: bool,
) -> bool {
    if times.is_none() || times.unwrap() == 1 {
        let scope = map
            .buffer_snapshot
            .language_scope_at(selection.start.to_point(map));
        let in_word = map
            .chars_at(selection.head())
            .next()
            .map(|(c, _)| char_kind(&scope, c) != CharKind::Whitespace)
            .unwrap_or_default();

        if in_word {
            if !use_subword {
                selection.end =
                    motion::next_word_end(map, selection.end, ignore_punctuation, 1, false);
            } else {
                selection.end =
                    motion::next_subword_end(map, selection.end, ignore_punctuation, 1, false);
            }
            selection.end = motion::next_char(map, selection.end, false);
            true
        } else {
            let motion = if use_subword {
                Motion::NextSubwordStart { ignore_punctuation }
            } else {
                Motion::NextWordStart { ignore_punctuation }
            };
            motion.expand_selection(map, selection, None, false, &text_layout_details)
        }
    } else {
        let motion = if use_subword {
            Motion::NextSubwordStart { ignore_punctuation }
        } else {
            Motion::NextWordStart { ignore_punctuation }
        };
        motion.expand_selection(map, selection, times, false, &text_layout_details)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::test::NeovimBackedTestContext;

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
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.assert_neovim_compatible(
            indoc! {"
            The qˇuick
            brown fox"},
            ["c", "0"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            ˇ
            brown fox"},
            ["c", "0"],
        )
        .await;
    }

    #[gpui::test]
    async fn test_change_k(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brown ˇfox
            jumps over"},
            ["c", "k"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brown fox
            jumps ˇover"},
            ["c", "k"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The qˇuick
            brown fox
            jumps over"},
            ["c", "k"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            ˇ
            brown fox
            jumps over"},
            ["c", "k"],
        )
        .await;
    }

    #[gpui::test]
    async fn test_change_j(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brown ˇfox
            jumps over"},
            ["c", "j"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brown fox
            jumps ˇover"},
            ["c", "j"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The qˇuick
            brown fox
            jumps over"},
            ["c", "j"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brown fox
            ˇ"},
            ["c", "j"],
        )
        .await;
    }

    #[gpui::test]
    async fn test_change_end_of_document(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"},
            ["c", "shift-g"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"},
            ["c", "shift-g"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brown fox
            jumps over
            the lˇazy"},
            ["c", "shift-g"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brown fox
            jumps over
            ˇ"},
            ["c", "shift-g"],
        )
        .await;
    }

    #[gpui::test]
    async fn test_change_cc(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_neovim_compatible(
            indoc! {"
           The quick
             brownˇ fox
           jumps over
           the lazy"},
            ["c", "c"],
        )
        .await;

        cx.assert_neovim_compatible(
            indoc! {"
           ˇThe quick
           brown fox
           jumps over
           the lazy"},
            ["c", "c"],
        )
        .await;

        cx.assert_neovim_compatible(
            indoc! {"
           The quick
             broˇwn fox
           jumˇps over
           the lazy"},
            ["c", "c"],
        )
        .await;
    }

    #[gpui::test]
    async fn test_change_gg(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"},
            ["c", "g", "g"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brown fox
            jumps over
            the lˇazy"},
            ["c", "g", "g"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The qˇuick
            brown fox
            jumps over
            the lazy"},
            ["c", "g", "g"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            ˇ
            brown fox
            jumps over
            the lazy"},
            ["c", "g", "g"],
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

        for count in 1..=5 {
            for marked_text in cx.each_marked_position(indoc! {"
                ˇThe quˇickˇ browˇn
                ˇ
                ˇfox ˇjumpsˇ-ˇoˇver
                ˇthe lazy dog
                "})
            {
                cx.assert_neovim_compatible(&marked_text, ["c", &count.to_string(), "b"])
                    .await;
            }
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
