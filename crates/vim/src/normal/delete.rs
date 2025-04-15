use crate::{
    Vim,
    motion::{Motion, MotionKind},
    object::Object,
};
use collections::{HashMap, HashSet};
use editor::{
    Bias, DisplayPoint,
    display_map::{DisplaySnapshot, ToDisplayPoint},
    scroll::Autoscroll,
};
use gpui::{Context, Window};
use language::{Point, Selection};
use multi_buffer::MultiBufferRow;

impl Vim {
    pub fn delete_motion(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        forced_motion: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(window, cx, |vim, editor, window, cx| {
            let text_layout_details = editor.text_layout_details(window);
            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let mut original_columns: HashMap<_, _> = Default::default();
                let mut motion_kind = None;
                let mut ranges_to_copy = Vec::new();
                editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                    s.move_with(|map, selection| {
                        let original_head = selection.head();
                        original_columns.insert(selection.id, original_head.column());
                        let kind = motion.expand_selection(
                            map,
                            selection,
                            times,
                            &text_layout_details,
                            forced_motion,
                        );
                        ranges_to_copy
                            .push(selection.start.to_point(map)..selection.end.to_point(map));

                        // When deleting line-wise, we always want to delete a newline.
                        // If there is one after the current line, it goes; otherwise we
                        // pick the one before.
                        if kind == Some(MotionKind::Linewise) {
                            let start = selection.start.to_point(map);
                            let end = selection.end.to_point(map);
                            if end.row < map.buffer_snapshot.max_point().row {
                                selection.end = Point::new(end.row + 1, 0).to_display_point(map)
                            } else if start.row > 0 {
                                selection.start = Point::new(
                                    start.row - 1,
                                    map.buffer_snapshot.line_len(MultiBufferRow(start.row - 1)),
                                )
                                .to_display_point(map)
                            }
                        }
                        if let Some(kind) = kind {
                            motion_kind.get_or_insert(kind);
                        }
                    });
                });
                let Some(kind) = motion_kind else { return };
                vim.copy_ranges(editor, kind, false, ranges_to_copy, window, cx);
                editor.insert("", window, cx);

                // Fixup cursor position after the deletion
                editor.set_clip_at_line_ends(true, cx);
                editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                    s.move_with(|map, selection| {
                        let mut cursor = selection.head();
                        if kind.linewise() {
                            if let Some(column) = original_columns.get(&selection.id) {
                                *cursor.column_mut() = *column
                            }
                        }
                        cursor = map.clip_point(cursor, Bias::Left);
                        selection.collapse_to(cursor, selection.goal)
                    });
                });
                editor.refresh_inline_completion(true, false, window, cx);
            });
        });
    }

    pub fn delete_object(
        &mut self,
        object: Object,
        around: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(window, cx, |vim, editor, window, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);
                // Emulates behavior in vim where if we expanded backwards to include a newline
                // the cursor gets set back to the start of the line
                let mut should_move_to_start: HashSet<_> = Default::default();
                editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                    s.move_with(|map, selection| {
                        object.expand_selection(map, selection, around);
                        let offset_range = selection.map(|p| p.to_offset(map, Bias::Left)).range();
                        let mut move_selection_start_to_previous_line =
                            |map: &DisplaySnapshot, selection: &mut Selection<DisplayPoint>| {
                                let start = selection.start.to_offset(map, Bias::Left);
                                if selection.start.row().0 > 0 {
                                    should_move_to_start.insert(selection.id);
                                    selection.start =
                                        (start - '\n'.len_utf8()).to_display_point(map);
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
                vim.copy_selections_content(editor, MotionKind::Exclusive, window, cx);
                editor.insert("", window, cx);

                // Fixup cursor position after the deletion
                editor.set_clip_at_line_ends(true, cx);
                editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                    s.move_with(|map, selection| {
                        let mut cursor = selection.head();
                        if should_move_to_start.contains(&selection.id) {
                            *cursor.column_mut() = 0;
                        }
                        cursor = map.clip_point(cursor, Bias::Left);
                        selection.collapse_to(cursor, selection.goal)
                    });
                });
                editor.refresh_inline_completion(true, false, window, cx);
            });
        });
    }
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
        cx.simulate("d h", "Teˇst").await.assert_matches();
        cx.simulate("d h", "Tˇest").await.assert_matches();
        cx.simulate("d h", "ˇTest").await.assert_matches();
        cx.simulate(
            "d h",
            indoc! {"
            Test
            ˇtest"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_delete_l(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate("d l", "ˇTest").await.assert_matches();
        cx.simulate("d l", "Teˇst").await.assert_matches();
        cx.simulate("d l", "Tesˇt").await.assert_matches();
        cx.simulate(
            "d l",
            indoc! {"
                Tesˇt
                test"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_delete_w(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate(
            "d w",
            indoc! {"
            Test tesˇt
                test"},
        )
        .await
        .assert_matches();

        cx.simulate("d w", "Teˇst").await.assert_matches();
        cx.simulate("d w", "Tˇest test").await.assert_matches();
        cx.simulate(
            "d w",
            indoc! {"
            Test teˇst
            test"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d w",
            indoc! {"
            Test tesˇt
            test"},
        )
        .await
        .assert_matches();

        cx.simulate(
            "d w",
            indoc! {"
            Test test
            ˇ
            test"},
        )
        .await
        .assert_matches();

        cx.simulate("d shift-w", "Test teˇst-test test")
            .await
            .assert_matches();
    }

    #[gpui::test]
    async fn test_delete_next_word_end(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate("d e", "Teˇst Test\n").await.assert_matches();
        cx.simulate("d e", "Tˇest test\n").await.assert_matches();
        cx.simulate(
            "d e",
            indoc! {"
            Test teˇst
            test"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d e",
            indoc! {"
            Test tesˇt
            test"},
        )
        .await
        .assert_matches();

        cx.simulate("d e", "Test teˇst-test test")
            .await
            .assert_matches();
    }

    #[gpui::test]
    async fn test_delete_b(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate("d b", "Teˇst Test").await.assert_matches();
        cx.simulate("d b", "Test ˇtest").await.assert_matches();
        cx.simulate("d b", "Test1 test2 ˇtest3")
            .await
            .assert_matches();
        cx.simulate(
            "d b",
            indoc! {"
            Test test
            ˇtest"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d b",
            indoc! {"
            Test test
            ˇ
            test"},
        )
        .await
        .assert_matches();

        cx.simulate("d shift-b", "Test test-test ˇtest")
            .await
            .assert_matches();
    }

    #[gpui::test]
    async fn test_delete_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate(
            "d $",
            indoc! {"
            The qˇuick
            brown fox"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d $",
            indoc! {"
            The quick
            ˇ
            brown fox"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_delete_0(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate(
            "d 0",
            indoc! {"
            The qˇuick
            brown fox"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d 0",
            indoc! {"
            The quick
            ˇ
            brown fox"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_delete_k(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate(
            "d k",
            indoc! {"
            The quick
            brown ˇfox
            jumps over"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d k",
            indoc! {"
            The quick
            brown fox
            jumps ˇover"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d k",
            indoc! {"
            The qˇuick
            brown fox
            jumps over"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d k",
            indoc! {"
            ˇbrown fox
            jumps over"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_delete_j(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate(
            "d j",
            indoc! {"
            The quick
            brown ˇfox
            jumps over"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d j",
            indoc! {"
            The quick
            brown fox
            jumps ˇover"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d j",
            indoc! {"
            The qˇuick
            brown fox
            jumps over"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d j",
            indoc! {"
            The quick
            brown fox
            ˇ"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_delete_end_of_document(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate(
            "d shift-g",
            indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d shift-g",
            indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d shift-g",
            indoc! {"
            The quick
            brown fox
            jumps over
            the lˇazy"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d shift-g",
            indoc! {"
            The quick
            brown fox
            jumps over
            ˇ"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_delete_to_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate(
            "d 3 shift-g",
            indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d 3 shift-g",
            indoc! {"
            The quick
            brown fox
            jumps over
            the lˇazy"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d 2 shift-g",
            indoc! {"
            The quick
            brown fox
            jumps over
            ˇ"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_delete_gg(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate(
            "d g g",
            indoc! {"
            The quick
            brownˇ fox
            jumps over
            the lazy"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d g g",
            indoc! {"
            The quick
            brown fox
            jumps over
            the lˇazy"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d g g",
            indoc! {"
            The qˇuick
            brown fox
            jumps over
            the lazy"},
        )
        .await
        .assert_matches();
        cx.simulate(
            "d g g",
            indoc! {"
            ˇ
            brown fox
            jumps over
            the lazy"},
        )
        .await
        .assert_matches();
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
        cx.shared_state().await.assert_eq(indoc! {"
        the ˇlazy dog"});

        cx.set_shared_state(indoc! {"
                The ˇquick brown
                fox jumps over
                the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("2 d d").await;
        cx.shared_state().await.assert_eq(indoc! {"
        the ˇlazy dog"});

        cx.set_shared_state(indoc! {"
                The ˇquick brown
                fox jumps over
                the moon,
                a star, and
                the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("2 d 2 d").await;
        cx.shared_state().await.assert_eq(indoc! {"
        the ˇlazy dog"});
    }

    #[gpui::test]
    async fn test_delete_to_adjacent_character(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.simulate("d t x", "ˇax").await.assert_matches();
        cx.simulate("d t x", "aˇx").await.assert_matches();
    }

    #[gpui::test]
    async fn test_delete_sentence(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        // cx.simulate(
        //     "d )",
        //     indoc! {"
        //     Fiˇrst. Second. Third.
        //     Fourth.
        //     "},
        // )
        // .await
        // .assert_matches();

        // cx.simulate(
        //     "d )",
        //     indoc! {"
        //     First. Secˇond. Third.
        //     Fourth.
        //     "},
        // )
        // .await
        // .assert_matches();

        // // Two deletes
        // cx.simulate(
        //     "d ) d )",
        //     indoc! {"
        //     First. Second. Thirˇd.
        //     Fourth.
        //     "},
        // )
        // .await
        // .assert_matches();

        // Should delete whole line if done on first column
        cx.simulate(
            "d )",
            indoc! {"
            ˇFirst.
            Fourth.
            "},
        )
        .await
        .assert_matches();

        // Backwards it should also delete the whole first line
        cx.simulate(
            "d (",
            indoc! {"
            First.
            ˇSecond.
            Fourth.
            "},
        )
        .await
        .assert_matches();
    }
}
