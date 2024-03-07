use crate::{motion::Motion, object::Object, utils::copy_selections_content, Vim};
use collections::{HashMap, HashSet};
use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    scroll::Autoscroll,
    Bias, DisplayPoint,
};
use gpui::WindowContext;
use language::{Point, Selection};

pub fn delete_motion(vim: &mut Vim, motion: Motion, times: Option<usize>, cx: &mut WindowContext) {
    vim.stop_recording();
    vim.update_active_editor(cx, |vim, editor, cx| {
        let text_layout_details = editor.text_layout_details(cx);
        editor.transact(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            let mut original_columns: HashMap<_, _> = Default::default();
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    let original_head = selection.head();
                    original_columns.insert(selection.id, original_head.column());
                    motion.expand_selection(map, selection, times, true, &text_layout_details);

                    // Motion::NextWordStart on an empty line should delete it.
                    if let Motion::NextWordStart {
                        ignore_punctuation: _,
                    } = motion
                    {
                        if selection.is_empty()
                            && map
                                .buffer_snapshot
                                .line_len(selection.start.to_point(&map).row)
                                == 0
                        {
                            selection.end = map
                                .buffer_snapshot
                                .clip_point(
                                    Point::new(selection.start.to_point(&map).row + 1, 0),
                                    Bias::Left,
                                )
                                .to_display_point(map)
                        }
                    }
                });
            });
            copy_selections_content(vim, editor, motion.linewise(), cx);
            editor.insert("", cx);

            // Fixup cursor position after the deletion
            editor.set_clip_at_line_ends(true, cx);
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
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

pub fn delete_object(vim: &mut Vim, object: Object, around: bool, cx: &mut WindowContext) {
    vim.stop_recording();
    vim.update_active_editor(cx, |vim, editor, cx| {
        editor.transact(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            // Emulates behavior in vim where if we expanded backwards to include a newline
            // the cursor gets set back to the start of the line
            let mut should_move_to_start: HashSet<_> = Default::default();
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    object.expand_selection(map, selection, around);
                    let offset_range = selection.map(|p| p.to_offset(map, Bias::Left)).range();
                    let mut move_selection_start_to_previous_line =
                        |map: &DisplaySnapshot, selection: &mut Selection<DisplayPoint>| {
                            let start = selection.start.to_offset(map, Bias::Left);
                            if selection.start.row() > 0 {
                                should_move_to_start.insert(selection.id);
                                selection.start = (start - '\n'.len_utf8()).to_display_point(map);
                            }
                        };
                    let contains_only_newlines = map
                        .chars_at(selection.start)
                        .take_while(|(_, p)| p < &selection.end)
                        .all(|(char, _)| char == '\n')
                        && !offset_range.is_empty();
                    let end_at_newline = map
                        .chars_at(selection.end)
                        .next()
                        .map(|(c, _)| c == '\n')
                        .unwrap_or(false);

                    // If expanded range contains only newlines and
                    // the object is around or sentence, expand to include a newline
                    // at the end or start
                    if (around || object == Object::Sentence) && contains_only_newlines {
                        if end_at_newline {
                            move_selection_end_to_next_line(map, selection);
                        } else {
                            move_selection_start_to_previous_line(map, selection);
                        }
                    }

                    // Does post-processing for the trailing newline and EOF
                    // when not cancelled.
                    let cancelled = around && selection.start == selection.end;
                    if object == Object::Paragraph && !cancelled {
                        // EOF check should be done before including a trailing newline.
                        if ends_at_eof(map, selection) {
                            move_selection_start_to_previous_line(map, selection);
                        }

                        if end_at_newline {
                            move_selection_end_to_next_line(map, selection);
                        }
                    }
                });
            });
            copy_selections_content(vim, editor, false, cx);
            editor.insert("", cx);

            // Fixup cursor position after the deletion
            editor.set_clip_at_line_ends(true, cx);
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    let mut cursor = selection.head();
                    if should_move_to_start.contains(&selection.id) {
                        *cursor.column_mut() = 0;
                    }
                    cursor = map.clip_point(cursor, Bias::Left);
                    selection.collapse_to(cursor, selection.goal)
                });
            });
        });
    });
}

fn move_selection_end_to_next_line(map: &DisplaySnapshot, selection: &mut Selection<DisplayPoint>) {
    let end = selection.end.to_offset(map, Bias::Left);
    selection.end = (end + '\n'.len_utf8()).to_display_point(map);
}

fn ends_at_eof(map: &DisplaySnapshot, selection: &mut Selection<DisplayPoint>) -> bool {
    selection.end.to_point(map) == map.buffer_snapshot.max_point()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{
        state::Mode,
        test::{ExemptionFeatures, NeovimBackedTestContext, VimTestContext},
    };

    #[gpui::test]
    async fn test_delete_h(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["d", "h"]);
        cx.assert("Teˇst").await;
        cx.assert("Tˇest").await;
        cx.assert("ˇTest").await;
        cx.assert(indoc! {"
            Test
            ˇtest"})
            .await;
    }

    #[gpui::test]
    async fn test_delete_l(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["d", "l"]);
        cx.assert("ˇTest").await;
        cx.assert("Teˇst").await;
        cx.assert("Tesˇt").await;
        cx.assert(indoc! {"
                Tesˇt
                test"})
            .await;
    }

    #[gpui::test]
    async fn test_delete_w(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_neovim_compatible(
            indoc! {"
            Test tesˇt
                test"},
            ["d", "w"],
        )
        .await;

        cx.assert_neovim_compatible("Teˇst", ["d", "w"]).await;
        cx.assert_neovim_compatible("Tˇest test", ["d", "w"]).await;
        cx.assert_neovim_compatible(
            indoc! {"
            Test teˇst
            test"},
            ["d", "w"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            Test tesˇt
            test"},
            ["d", "w"],
        )
        .await;

        cx.assert_neovim_compatible(
            indoc! {"
            Test test
            ˇ
            test"},
            ["d", "w"],
        )
        .await;

        let mut cx = cx.binding(["d", "shift-w"]);
        cx.assert_neovim_compatible("Test teˇst-test test", ["d", "shift-w"])
            .await;
    }

    #[gpui::test]
    async fn test_delete_next_word_end(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["d", "e"]);
        // cx.assert("Teˇst Test").await;
        // cx.assert("Tˇest test").await;
        cx.assert(indoc! {"
            Test teˇst
            test"})
            .await;
        cx.assert(indoc! {"
            Test tesˇt
            test"})
            .await;
        cx.assert_exempted(
            indoc! {"
            Test test
            ˇ
            test"},
            ExemptionFeatures::OperatorLastNewlineRemains,
        )
        .await;

        let mut cx = cx.binding(["d", "shift-e"]);
        cx.assert("Test teˇst-test test").await;
    }

    #[gpui::test]
    async fn test_delete_b(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["d", "b"]);
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

        let mut cx = cx.binding(["d", "shift-b"]);
        cx.assert("Test test-test ˇtest").await;
    }

    #[gpui::test]
    async fn test_delete_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["d", "$"]);
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
    async fn test_delete_0(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["d", "0"]);
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
    async fn test_delete_k(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["d", "k"]);
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
        cx.assert(indoc! {"
            The qˇuick
            brown fox
            jumps over"})
            .await;
        cx.assert(indoc! {"
            ˇbrown fox
            jumps over"})
            .await;
    }

    #[gpui::test]
    async fn test_delete_j(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["d", "j"]);
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
        cx.assert(indoc! {"
            The qˇuick
            brown fox
            jumps over"})
            .await;
        cx.assert(indoc! {"
            The quick
            brown fox
            ˇ"})
            .await;
    }

    #[gpui::test]
    async fn test_delete_end_of_document(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"},
            ["d", "shift-g"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"},
            ["d", "shift-g"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brown fox
            jumps over
            the lˇazy"},
            ["d", "shift-g"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brown fox
            jumps over
            ˇ"},
            ["d", "shift-g"],
        )
        .await;
    }

    #[gpui::test]
    async fn test_delete_gg(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx)
            .await
            .binding(["d", "g", "g"]);
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"},
            ["d", "g", "g"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The quick
            brown fox
            jumps over
            the lˇazy"},
            ["d", "g", "g"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            The qˇuick
            brown fox
            jumps over
            the lazy"},
            ["d", "g", "g"],
        )
        .await;
        cx.assert_neovim_compatible(
            indoc! {"
            ˇ
            brown fox
            jumps over
            the lazy"},
            ["d", "g", "g"],
        )
        .await;
    }

    #[gpui::test]
    async fn test_cancel_delete_operator(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.set_state(
            indoc! {"
                The quick brown
                fox juˇmps over
                the lazy dog"},
            Mode::Normal,
        );

        // Canceling operator twice reverts to normal mode with no active operator
        cx.simulate_keystrokes(["d", "escape", "k"]);
        assert_eq!(cx.active_operator(), None);
        assert_eq!(cx.mode(), Mode::Normal);
        cx.assert_editor_state(indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog"});
    }

    #[gpui::test]
    async fn test_unbound_command_cancels_pending_operator(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.set_state(
            indoc! {"
                The quick brown
                fox juˇmps over
                the lazy dog"},
            Mode::Normal,
        );

        // Canceling operator twice reverts to normal mode with no active operator
        cx.simulate_keystrokes(["d", "y"]);
        assert_eq!(cx.active_operator(), None);
        assert_eq!(cx.mode(), Mode::Normal);
    }

    #[gpui::test]
    async fn test_delete_with_counts(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {"
                The ˇquick brown
                fox jumps over
                the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["d", "2", "d"]).await;
        cx.assert_shared_state(indoc! {"
        the ˇlazy dog"})
            .await;

        cx.set_shared_state(indoc! {"
                The ˇquick brown
                fox jumps over
                the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["2", "d", "d"]).await;
        cx.assert_shared_state(indoc! {"
        the ˇlazy dog"})
            .await;

        cx.set_shared_state(indoc! {"
                The ˇquick brown
                fox jumps over
                the moon,
                a star, and
                the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["2", "d", "2", "d"]).await;
        cx.assert_shared_state(indoc! {"
        the ˇlazy dog"})
            .await;
    }

    #[gpui::test]
    async fn test_delete_to_adjacent_character(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_neovim_compatible("ˇax", ["d", "t", "x"]).await;
        cx.assert_neovim_compatible("aˇx", ["d", "t", "x"]).await;
    }
}
