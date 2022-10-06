use crate::{motion::Motion, object::Object, utils::copy_selections_content, Vim};
use collections::{HashMap, HashSet};
use editor::{display_map::ToDisplayPoint, Autoscroll, Bias};
use gpui::MutableAppContext;

pub fn delete_motion(vim: &mut Vim, motion: Motion, cx: &mut MutableAppContext) {
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
            copy_selections_content(editor, motion.linewise(), cx);
            editor.insert("", cx);

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

pub fn delete_object(vim: &mut Vim, object: Object, around: bool, cx: &mut MutableAppContext) {
    vim.update_active_editor(cx, |editor, cx| {
        editor.transact(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            // Emulates behavior in vim where if we expanded backwards to include a newline
            // the cursor gets set back to the start of the line
            let mut should_move_to_start: HashSet<_> = Default::default();
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    object.expand_selection(map, selection, around);
                    let offset_range = selection.map(|p| p.to_offset(map, Bias::Left)).range();
                    let contains_only_newlines = map
                        .chars_at(selection.start)
                        .take_while(|(_, p)| p < &selection.end)
                        .all(|(char, _)| char == '\n')
                        || offset_range.is_empty();
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
                            selection.end =
                                (offset_range.end + '\n'.len_utf8()).to_display_point(map);
                        } else if selection.start.row() > 0 {
                            should_move_to_start.insert(selection.id);
                            selection.start =
                                (offset_range.start - '\n'.len_utf8()).to_display_point(map);
                        }
                    }
                });
            });
            copy_selections_content(editor, false, cx);
            editor.insert("", cx);

            // Fixup cursor position after the deletion
            editor.set_clip_at_line_ends(true, cx);
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
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

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{state::Mode, test_contexts::VimTestContext};

    #[gpui::test]
    async fn test_delete_h(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "h"]);
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
    async fn test_delete_l(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "l"]);
        cx.assert("ˇTest", "ˇest");
        cx.assert("Teˇst", "Teˇt");
        cx.assert("Tesˇt", "Teˇs");
        cx.assert(
            indoc! {"
                Tesˇt
                test"},
            indoc! {"
                Teˇs
                test"},
        );
    }

    #[gpui::test]
    async fn test_delete_w(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "w"]);
        cx.assert("Teˇst", "Tˇe");
        cx.assert("Tˇest test", "Tˇtest");
        cx.assert(
            indoc! {"
                Test teˇst
                test"},
            indoc! {"
                Test tˇe
                test"},
        );
        cx.assert(
            indoc! {"
                Test tesˇt
                test"},
            indoc! {"
                Test teˇs
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

        let mut cx = cx.binding(["d", "shift-w"]);
        cx.assert("Test teˇst-test test", "Test teˇtest");
    }

    #[gpui::test]
    async fn test_delete_e(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "e"]);
        cx.assert("Teˇst Test", "Teˇ Test");
        cx.assert("Tˇest test", "Tˇ test");
        cx.assert(
            indoc! {"
                Test teˇst
                test"},
            indoc! {"
                Test tˇe
                test"},
        );
        cx.assert(
            indoc! {"
                Test tesˇt
                test"},
            "Test teˇs",
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

        let mut cx = cx.binding(["d", "shift-e"]);
        cx.assert("Test teˇst-test test", "Test teˇ test");
    }

    #[gpui::test]
    async fn test_delete_b(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "b"]);
        cx.assert("Teˇst Test", "ˇst Test");
        cx.assert("Test ˇtest", "ˇtest");
        cx.assert("Test1 test2 ˇtest3", "Test1 ˇtest3");
        cx.assert(
            indoc! {"
                Test test
                ˇtest"},
            // Trailing whitespace after cursor
            indoc! {"
                Testˇ 
                test"},
        );
        cx.assert(
            indoc! {"
                Test test
                ˇ
                test"},
            // Trailing whitespace after cursor
            indoc! {"
                Testˇ 
                
                test"},
        );

        let mut cx = cx.binding(["d", "shift-b"]);
        cx.assert("Test test-test ˇtest", "Test ˇtest");
    }

    #[gpui::test]
    async fn test_delete_end_of_line(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "$"]);
        cx.assert(
            indoc! {"
                The qˇuick
                brown fox"},
            indoc! {"
                The ˇq
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
    async fn test_delete_0(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "0"]);
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
    async fn test_delete_k(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "k"]);
        cx.assert(
            indoc! {"
                The quick
                brown ˇfox
                jumps over"},
            "jumps ˇover",
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                jumps ˇover"},
            "The quˇick",
        );
        cx.assert(
            indoc! {"
                The qˇuick
                brown fox
                jumps over"},
            indoc! {"
                brownˇ fox
                jumps over"},
        );
        cx.assert(
            indoc! {"
                ˇbrown fox
                jumps over"},
            "ˇjumps over",
        );
    }

    #[gpui::test]
    async fn test_delete_j(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "j"]);
        cx.assert(
            indoc! {"
                The quick
                brown ˇfox
                jumps over"},
            "The quˇick",
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                jumps ˇover"},
            indoc! {"
                The quick
                brown ˇfox"},
        );
        cx.assert(
            indoc! {"
                The qˇuick
                brown fox
                jumps over"},
            "jumpsˇ over",
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                ˇ"},
            indoc! {"
                The quick
                ˇbrown fox"},
        );
    }

    #[gpui::test]
    async fn test_delete_end_of_document(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "shift-g"]);
        cx.assert(
            indoc! {"
                The quick
                brownˇ fox
                jumps over
                the lazy"},
            "The qˇuick",
        );
        cx.assert(
            indoc! {"
                The quick
                brownˇ fox
                jumps over
                the lazy"},
            "The qˇuick",
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
                jumpsˇ over"},
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
                ˇjumps over"},
        );
    }

    #[gpui::test]
    async fn test_delete_gg(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "g", "g"]);
        cx.assert(
            indoc! {"
                The quick
                brownˇ fox
                jumps over
                the lazy"},
            indoc! {"
                jumpsˇ over
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
                brownˇ fox
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
                ˇbrown fox
                jumps over
                the lazy"},
        );
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
}
