use crate::{motion::Motion, object::Object, utils::copy_selections_content, Vim};
use collections::{HashMap, HashSet};
use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    scroll::Autoscroll,
    Bias, DisplayPoint,
};
use gpui::WindowContext;
use language::{Point, Selection};
use multi_buffer::MultiBufferRow;

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
                                .line_len(MultiBufferRow(selection.start.to_point(&map).row))
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
                            if selection.start.row().0 > 0 {
                                should_move_to_start.insert(selection.id);
                                selection.start = (start - '\n'.len_utf8()).to_display_point(map);
                            }
                        };
                    let range = selection.start.to_offset(map, Bias::Left)
                        ..selection.end.to_offset(map, Bias::Right);
                    let contains_only_newlines = map
                        .buffer_chars_at(range.start)
                        .take_while(|(_, p)| p < &range.end)
                        .all(|(char, _)| char == '\n')
                        && !offset_range.is_empty();
                    let end_at_newline = map
                        .buffer_chars_at(range.end)
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
        test::{NeovimBackedTestContext, VimTestContext},
    };

    #[gpui::test]
    async fn test_delete_h(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches("d h", "Teˇst").await;
        cx.assert_binding_matches("d h", "Tˇest").await;
        cx.assert_binding_matches("d h", "ˇTest").await;
        cx.assert_binding_matches(
            "d h",
            indoc! {"
            Test
            ˇtest"},
        )
        .await;
    }

    #[gpui::test]
    async fn test_delete_l(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches("d l", "ˇTest").await;
        cx.assert_binding_matches("d l", "Teˇst").await;
        cx.assert_binding_matches("d l", "Tesˇt").await;
        cx.assert_binding_matches(
            "d l",
            indoc! {"
                Tesˇt
                test"},
        )
        .await;
    }

    #[gpui::test]
    async fn test_delete_w(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches(
            "d w",
            indoc! {"
            Test tesˇt
                test"},
        )
        .await;

        cx.assert_binding_matches("d w", "Teˇst").await;
        cx.assert_binding_matches("d w", "Tˇest test").await;
        cx.assert_binding_matches(
            "d w",
            indoc! {"
            Test teˇst
            test"},
        )
        .await;
        cx.assert_binding_matches(
            "d w",
            indoc! {"
            Test tesˇt
            test"},
        )
        .await;

        cx.assert_binding_matches(
            "d w",
            indoc! {"
            Test test
            ˇ
            test"},
        )
        .await;

        cx.assert_binding_matches("d shift-w", "Test teˇst-test test")
            .await;
    }

    #[gpui::test]
    async fn test_delete_next_word_end(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches("d e", "Teˇst Test\n").await;
        cx.assert_binding_matches("d e", "Tˇest test\n").await;
        cx.assert_binding_matches(
            "d e",
            indoc! {"
            Test teˇst
            test"},
        )
        .await;
        cx.assert_binding_matches(
            "d e",
            indoc! {"
            Test tesˇt
            test"},
        )
        .await;

        cx.assert_binding_matches("d e", "Test teˇst-test test")
            .await;
    }

    #[gpui::test]
    async fn test_delete_b(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches("d b", "Teˇst Test").await;
        cx.assert_binding_matches("d b", "Test ˇtest").await;
        cx.assert_binding_matches("d b", "Test1 test2 ˇtest3").await;
        cx.assert_binding_matches(
            "d b",
            indoc! {"
            Test test
            ˇtest"},
        )
        .await;
        cx.assert_binding_matches(
            "d b",
            indoc! {"
            Test test
            ˇ
            test"},
        )
        .await;

        cx.assert_binding_matches("d shift-b", "Test test-test ˇtest")
            .await;
    }

    #[gpui::test]
    async fn test_delete_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches(
            "d $",
            indoc! {"
            The qˇuick
            brown fox"},
        )
        .await;
        cx.assert_binding_matches(
            "d $",
            indoc! {"
            The quick
            ˇ
            brown fox"},
        )
        .await;
    }

    #[gpui::test]
    async fn test_delete_0(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches(
            "d 0",
            indoc! {"
            The qˇuick
            brown fox"},
        )
        .await;
        cx.assert_binding_matches(
            "d 0",
            indoc! {"
            The quick
            ˇ
            brown fox"},
        )
        .await;
    }

    #[gpui::test]
    async fn test_delete_k(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches(
            "d k",
            indoc! {"
            The quick
            brown ˇfox
            jumps over"},
        )
        .await;
        cx.assert_binding_matches(
            "d k",
            indoc! {"
            The quick
            brown fox
            jumps ˇover"},
        )
        .await;
        cx.assert_binding_matches(
            "d k",
            indoc! {"
            The qˇuick
            brown fox
            jumps over"},
        )
        .await;
        cx.assert_binding_matches(
            "d k",
            indoc! {"
            ˇbrown fox
            jumps over"},
        )
        .await;
    }

    #[gpui::test]
    async fn test_delete_j(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches(
            "d j",
            indoc! {"
            The quick
            brown ˇfox
            jumps over"},
        )
        .await;
        cx.assert_binding_matches(
            "d j",
            indoc! {"
            The quick
            brown fox
            jumps ˇover"},
        )
        .await;
        cx.assert_binding_matches(
            "d j",
            indoc! {"
            The qˇuick
            brown fox
            jumps over"},
        )
        .await;
        cx.assert_binding_matches(
            "d j",
            indoc! {"
            The quick
            brown fox
            ˇ"},
        )
        .await;
    }

    #[gpui::test]
    async fn test_delete_end_of_document(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches(
            "d shift-g",
            indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"},
        )
        .await;
        cx.assert_binding_matches(
            "d shift-g",
            indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"},
        )
        .await;
        cx.assert_binding_matches(
            "d shift-g",
            indoc! {"
            The quick
            brown fox
            jumps over
            the lˇazy"},
        )
        .await;
        cx.assert_binding_matches(
            "d shift-g",
            indoc! {"
            The quick
            brown fox
            jumps over
            ˇ"},
        )
        .await;
    }

    #[gpui::test]
    async fn test_delete_gg(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches(
            "d g g",
            indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"},
        )
        .await;
        cx.assert_binding_matches(
            "d g g",
            indoc! {"
            The quick
            brown fox
            jumps over
            the lˇazy"},
        )
        .await;
        cx.assert_binding_matches(
            "d g g",
            indoc! {"
            The qˇuick
            brown fox
            jumps over
            the lazy"},
        )
        .await;
        cx.assert_binding_matches(
            "d g g",
            indoc! {"
            ˇ
            brown fox
            jumps over
            the lazy"},
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
        cx.simulate_keystrokes("d escape k");
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
        cx.simulate_keystrokes("d y");
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
        cx.simulate_shared_keystrokes("d 2 d").await;
        cx.assert_shared_state(indoc! {"
        the ˇlazy dog"})
            .await;

        cx.set_shared_state(indoc! {"
                The ˇquick brown
                fox jumps over
                the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("2 d d").await;
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
        cx.simulate_shared_keystrokes("2 d 2 d").await;
        cx.assert_shared_state(indoc! {"
        the ˇlazy dog"})
            .await;
    }

    #[gpui::test]
    async fn test_delete_to_adjacent_character(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches("d t x", "ˇax").await;
        cx.assert_binding_matches("d t x", "aˇx").await;
    }
}
