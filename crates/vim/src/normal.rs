mod change;
mod delete;
mod yank;

use std::borrow::Cow;

use crate::{
    motion::Motion,
    state::{Mode, Operator},
    Vim,
};
use change::init as change_init;
use collections::HashSet;
use editor::{Autoscroll, Bias, ClipboardSelection, DisplayPoint};
use gpui::{actions, MutableAppContext, ViewContext};
use language::{AutoindentMode, Point, SelectionGoal};
use workspace::Workspace;

use self::{change::change_over, delete::delete_over, yank::yank_over};

actions!(
    vim,
    [
        InsertAfter,
        InsertFirstNonWhitespace,
        InsertEndOfLine,
        InsertLineAbove,
        InsertLineBelow,
        DeleteLeft,
        DeleteRight,
        ChangeToEndOfLine,
        DeleteToEndOfLine,
        Paste,
        Yank,
    ]
);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(insert_after);
    cx.add_action(insert_first_non_whitespace);
    cx.add_action(insert_end_of_line);
    cx.add_action(insert_line_above);
    cx.add_action(insert_line_below);
    cx.add_action(|_: &mut Workspace, _: &DeleteLeft, cx| {
        Vim::update(cx, |vim, cx| {
            delete_over(vim, Motion::Left, cx);
        })
    });
    cx.add_action(|_: &mut Workspace, _: &DeleteRight, cx| {
        Vim::update(cx, |vim, cx| {
            delete_over(vim, Motion::Right, cx);
        })
    });
    cx.add_action(|_: &mut Workspace, _: &ChangeToEndOfLine, cx| {
        Vim::update(cx, |vim, cx| {
            change_over(vim, Motion::EndOfLine, cx);
        })
    });
    cx.add_action(|_: &mut Workspace, _: &DeleteToEndOfLine, cx| {
        Vim::update(cx, |vim, cx| {
            delete_over(vim, Motion::EndOfLine, cx);
        })
    });
    cx.add_action(paste);

    change_init(cx);
}

pub fn normal_motion(motion: Motion, cx: &mut MutableAppContext) {
    Vim::update(cx, |vim, cx| {
        match vim.state.operator_stack.pop() {
            None => move_cursor(vim, motion, cx),
            Some(Operator::Namespace(_)) => {
                // Can't do anything for a namespace operator. Ignoring
            }
            Some(Operator::Change) => change_over(vim, motion, cx),
            Some(Operator::Delete) => delete_over(vim, motion, cx),
            Some(Operator::Yank) => yank_over(vim, motion, cx),
        }
        vim.clear_operator(cx);
    });
}

fn move_cursor(vim: &mut Vim, motion: Motion, cx: &mut MutableAppContext) {
    vim.update_active_editor(cx, |editor, cx| {
        editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
            s.move_cursors_with(|map, cursor, goal| motion.move_point(map, cursor, goal))
        })
    });
}

fn insert_after(_: &mut Workspace, _: &InsertAfter, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.switch_mode(Mode::Insert, false, cx);
        vim.update_active_editor(cx, |editor, cx| {
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_cursors_with(|map, cursor, goal| {
                    Motion::Right.move_point(map, cursor, goal)
                });
            });
        });
    });
}

fn insert_first_non_whitespace(
    _: &mut Workspace,
    _: &InsertFirstNonWhitespace,
    cx: &mut ViewContext<Workspace>,
) {
    Vim::update(cx, |vim, cx| {
        vim.switch_mode(Mode::Insert, false, cx);
        vim.update_active_editor(cx, |editor, cx| {
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_cursors_with(|map, cursor, goal| {
                    Motion::FirstNonWhitespace.move_point(map, cursor, goal)
                });
            });
        });
    });
}

fn insert_end_of_line(_: &mut Workspace, _: &InsertEndOfLine, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.switch_mode(Mode::Insert, false, cx);
        vim.update_active_editor(cx, |editor, cx| {
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_cursors_with(|map, cursor, goal| {
                    Motion::EndOfLine.move_point(map, cursor, goal)
                });
            });
        });
    });
}

fn insert_line_above(_: &mut Workspace, _: &InsertLineAbove, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.switch_mode(Mode::Insert, false, cx);
        vim.update_active_editor(cx, |editor, cx| {
            editor.transact(cx, |editor, cx| {
                let (map, old_selections) = editor.selections.all_display(cx);
                let selection_start_rows: HashSet<u32> = old_selections
                    .into_iter()
                    .map(|selection| selection.start.row())
                    .collect();
                let edits = selection_start_rows.into_iter().map(|row| {
                    let (indent, _) = map.line_indent(row);
                    let start_of_line = map
                        .clip_point(DisplayPoint::new(row, 0), Bias::Left)
                        .to_point(&map);
                    let mut new_text = " ".repeat(indent as usize);
                    new_text.push('\n');
                    (start_of_line..start_of_line, new_text)
                });
                editor.edit_with_autoindent(edits, cx);
                editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                    s.move_cursors_with(|map, mut cursor, _| {
                        *cursor.row_mut() -= 1;
                        *cursor.column_mut() = map.line_len(cursor.row());
                        (map.clip_point(cursor, Bias::Left), SelectionGoal::None)
                    });
                });
            });
        });
    });
}

fn insert_line_below(_: &mut Workspace, _: &InsertLineBelow, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.switch_mode(Mode::Insert, false, cx);
        vim.update_active_editor(cx, |editor, cx| {
            editor.transact(cx, |editor, cx| {
                let (map, old_selections) = editor.selections.all_display(cx);
                let selection_end_rows: HashSet<u32> = old_selections
                    .into_iter()
                    .map(|selection| selection.end.row())
                    .collect();
                let edits = selection_end_rows.into_iter().map(|row| {
                    let (indent, _) = map.line_indent(row);
                    let end_of_line = map
                        .clip_point(DisplayPoint::new(row, map.line_len(row)), Bias::Left)
                        .to_point(&map);
                    let mut new_text = "\n".to_string();
                    new_text.push_str(&" ".repeat(indent as usize));
                    (end_of_line..end_of_line, new_text)
                });
                editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                    s.move_cursors_with(|map, cursor, goal| {
                        Motion::EndOfLine.move_point(map, cursor, goal)
                    });
                });
                editor.edit_with_autoindent(edits, cx);
            });
        });
    });
}

fn paste(_: &mut Workspace, _: &Paste, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |editor, cx| {
            editor.transact(cx, |editor, cx| {
                editor.set_clip_at_line_ends(false, cx);
                if let Some(item) = cx.as_mut().read_from_clipboard() {
                    let mut clipboard_text = Cow::Borrowed(item.text());
                    if let Some(mut clipboard_selections) =
                        item.metadata::<Vec<ClipboardSelection>>()
                    {
                        let (display_map, selections) = editor.selections.all_display(cx);
                        let all_selections_were_entire_line =
                            clipboard_selections.iter().all(|s| s.is_entire_line);
                        if clipboard_selections.len() != selections.len() {
                            let mut newline_separated_text = String::new();
                            let mut clipboard_selections =
                                clipboard_selections.drain(..).peekable();
                            let mut ix = 0;
                            while let Some(clipboard_selection) = clipboard_selections.next() {
                                newline_separated_text
                                    .push_str(&clipboard_text[ix..ix + clipboard_selection.len]);
                                ix += clipboard_selection.len;
                                if clipboard_selections.peek().is_some() {
                                    newline_separated_text.push('\n');
                                }
                            }
                            clipboard_text = Cow::Owned(newline_separated_text);
                        }

                        let mut new_selections = Vec::new();
                        editor.buffer().update(cx, |buffer, cx| {
                            let snapshot = buffer.snapshot(cx);
                            let mut start_offset = 0;
                            let mut edits = Vec::new();
                            for (ix, selection) in selections.iter().enumerate() {
                                let to_insert;
                                let linewise;
                                if let Some(clipboard_selection) = clipboard_selections.get(ix) {
                                    let end_offset = start_offset + clipboard_selection.len;
                                    to_insert = &clipboard_text[start_offset..end_offset];
                                    linewise = clipboard_selection.is_entire_line;
                                    start_offset = end_offset;
                                } else {
                                    to_insert = clipboard_text.as_str();
                                    linewise = all_selections_were_entire_line;
                                }

                                // If the clipboard text was copied linewise, and the current selection
                                // is empty, then paste the text after this line and move the selection
                                // to the start of the pasted text
                                let insert_at = if linewise {
                                    let (point, _) = display_map
                                        .next_line_boundary(selection.start.to_point(&display_map));

                                    if !to_insert.starts_with('\n') {
                                        // Add newline before pasted text so that it shows up
                                        edits.push((point..point, "\n"));
                                    }
                                    // Drop selection at the start of the next line
                                    let selection_point = Point::new(point.row + 1, 0);
                                    new_selections.push(selection.map(|_| selection_point.clone()));
                                    point
                                } else {
                                    let mut point = selection.end;
                                    // Paste the text after the current selection
                                    *point.column_mut() = point.column() + 1;
                                    let point = display_map
                                        .clip_point(point, Bias::Right)
                                        .to_point(&display_map);

                                    new_selections.push(selection.map(|_| point));
                                    point
                                };

                                if linewise && to_insert.ends_with('\n') {
                                    edits.push((
                                        insert_at..insert_at,
                                        &to_insert[0..to_insert.len().saturating_sub(1)],
                                    ))
                                } else {
                                    edits.push((insert_at..insert_at, to_insert));
                                }
                            }
                            drop(snapshot);
                            buffer.edit(edits, Some(AutoindentMode::EachLine), cx);
                        });

                        editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                            s.select(new_selections)
                        });
                    } else {
                        editor.insert(&clipboard_text, cx);
                    }
                }
                editor.set_clip_at_line_ends(true, cx);
            });
        });
    });
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use util::test::marked_text_offsets;

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
        cx.assert("The qˇuick", "The ˇquick");
        cx.assert("ˇThe quick", "ˇThe quick");
        cx.assert(
            indoc! {"
                The quick
                ˇbrown"},
            indoc! {"
                The quick
                ˇbrown"},
        );
    }

    #[gpui::test]
    async fn test_backspace(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["backspace"]);
        cx.assert("The qˇuick", "The ˇquick");
        cx.assert("ˇThe quick", "ˇThe quick");
        cx.assert(
            indoc! {"
                The quick
                ˇbrown"},
            indoc! {"
                The quick
                ˇbrown"},
        );
    }

    #[gpui::test]
    async fn test_j(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["j"]);
        cx.assert(
            indoc! {"
                The ˇquick
                brown fox"},
            indoc! {"
                The quick
                browˇn fox"},
        );
        cx.assert(
            indoc! {"
                The quick
                browˇn fox"},
            indoc! {"
                The quick
                browˇn fox"},
        );
        cx.assert(
            indoc! {"
                The quicˇk
                brown"},
            indoc! {"
                The quick
                browˇn"},
        );
        cx.assert(
            indoc! {"
                The quick
                ˇbrown"},
            indoc! {"
                The quick
                ˇbrown"},
        );
    }

    #[gpui::test]
    async fn test_k(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["k"]);
        cx.assert(
            indoc! {"
                The ˇquick
                brown fox"},
            indoc! {"
                The ˇquick
                brown fox"},
        );
        cx.assert(
            indoc! {"
                The quick
                browˇn fox"},
            indoc! {"
                The ˇquick
                brown fox"},
        );
        cx.assert(
            indoc! {"
                The
                quicˇk"},
            indoc! {"
                Thˇe
                quick"},
        );
    }

    #[gpui::test]
    async fn test_l(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["l"]);
        cx.assert("The qˇuick", "The quˇick");
        cx.assert("The quicˇk", "The quicˇk");
        cx.assert(
            indoc! {"
                The quicˇk
                brown"},
            indoc! {"
                The quicˇk
                brown"},
        );
    }

    #[gpui::test]
    async fn test_jump_to_line_boundaries(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["$"]);
        cx.assert("Tˇest test", "Test tesˇt");
        cx.assert("Test tesˇt", "Test tesˇt");
        cx.assert(
            indoc! {"
                The ˇquick
                brown"},
            indoc! {"
                The quicˇk
                brown"},
        );
        cx.assert(
            indoc! {"
                The quicˇk
                brown"},
            indoc! {"
                The quicˇk
                brown"},
        );

        let mut cx = cx.binding(["0"]);
        cx.assert("Test ˇtest", "ˇTest test");
        cx.assert("ˇTest test", "ˇTest test");
        cx.assert(
            indoc! {"
                The ˇquick
                brown"},
            indoc! {"
                ˇThe quick
                brown"},
        );
        cx.assert(
            indoc! {"
                ˇThe quick
                brown"},
            indoc! {"
                ˇThe quick
                brown"},
        );
    }

    #[gpui::test]
    async fn test_jump_to_end(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["shift-g"]);

        cx.assert(
            indoc! {"
                The ˇquick
                
                brown fox jumps
                over the lazy dog"},
            indoc! {"
                The quick
                
                brown fox jumps
                overˇ the lazy dog"},
        );
        cx.assert(
            indoc! {"
                The quick
                
                brown fox jumps
                overˇ the lazy dog"},
            indoc! {"
                The quick
                
                brown fox jumps
                overˇ the lazy dog"},
        );
        cx.assert(
            indoc! {"
            The quiˇck
            
            brown"},
            indoc! {"
            The quick
            
            browˇn"},
        );
        cx.assert(
            indoc! {"
            The quiˇck
            
            "},
            indoc! {"
            The quick
            
            ˇ"},
        );
    }

    #[gpui::test]
    async fn test_w(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        let (_, cursor_offsets) = marked_text_offsets(indoc! {"
            The ˇquickˇ-ˇbrown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthˇˇe"});
        cx.set_state(
            indoc! {"
            ˇThe quick-brown
            
            
            fox_jumps over
            the"},
            Mode::Normal,
        );

        for cursor_offset in cursor_offsets {
            cx.simulate_keystroke("w");
            cx.assert_editor_selections(vec![cursor_offset..cursor_offset]);
        }

        // Reset and test ignoring punctuation
        let (_, cursor_offsets) = marked_text_offsets(indoc! {"
            The ˇquick-brown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthˇˇe"});
        cx.set_state(
            indoc! {"
            ˇThe quick-brown
            
            
            fox_jumps over
            the"},
            Mode::Normal,
        );

        for cursor_offset in cursor_offsets {
            cx.simulate_keystroke("shift-w");
            cx.assert_editor_selections(vec![cursor_offset..cursor_offset]);
        }
    }

    #[gpui::test]
    async fn test_e(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        let (_, cursor_offsets) = marked_text_offsets(indoc! {"
            Thˇe quicˇkˇ-browˇn
            
            
            fox_jumpˇs oveˇr
            thˇe"});
        cx.set_state(
            indoc! {"
            ˇThe quick-brown
            
            
            fox_jumps over
            the"},
            Mode::Normal,
        );

        for cursor_offset in cursor_offsets {
            cx.simulate_keystroke("e");
            cx.assert_editor_selections(vec![cursor_offset..cursor_offset]);
        }

        // Reset and test ignoring punctuation
        let (_, cursor_offsets) = marked_text_offsets(indoc! {"
            Thˇe quick-browˇn
            
            
            fox_jumpˇs oveˇr
            thˇˇe"});
        cx.set_state(
            indoc! {"
            ˇThe quick-brown
            
            
            fox_jumps over
            the"},
            Mode::Normal,
        );
        for cursor_offset in cursor_offsets {
            cx.simulate_keystroke("shift-e");
            cx.assert_editor_selections(vec![cursor_offset..cursor_offset]);
        }
    }

    #[gpui::test]
    async fn test_b(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        let (_, cursor_offsets) = marked_text_offsets(indoc! {"
            ˇˇThe ˇquickˇ-ˇbrown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthe"});
        cx.set_state(
            indoc! {"
            The quick-brown
            
            
            fox_jumps over
            thˇe"},
            Mode::Normal,
        );

        for cursor_offset in cursor_offsets.into_iter().rev() {
            cx.simulate_keystroke("b");
            cx.assert_editor_selections(vec![cursor_offset..cursor_offset]);
        }

        // Reset and test ignoring punctuation
        let (_, cursor_offsets) = marked_text_offsets(indoc! {"
            ˇˇThe ˇquick-brown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthe"});
        cx.set_state(
            indoc! {"
            The quick-brown
            
            
            fox_jumps over
            thˇe"},
            Mode::Normal,
        );
        for cursor_offset in cursor_offsets.into_iter().rev() {
            cx.simulate_keystroke("shift-b");
            cx.assert_editor_selections(vec![cursor_offset..cursor_offset]);
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
    async fn test_gg(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["g", "g"]);
        cx.assert(
            indoc! {"
                The quick
            
                brown fox jumps
                over ˇthe lazy dog"},
            indoc! {"
                The qˇuick
            
                brown fox jumps
                over the lazy dog"},
        );
        cx.assert(
            indoc! {"
                The qˇuick
            
                brown fox jumps
                over the lazy dog"},
            indoc! {"
                The qˇuick
            
                brown fox jumps
                over the lazy dog"},
        );
        cx.assert(
            indoc! {"
                The quick
            
                brown fox jumps
                over the laˇzy dog"},
            indoc! {"
                The quicˇk
            
                brown fox jumps
                over the lazy dog"},
        );
        cx.assert(
            indoc! {"
                
            
                brown fox jumps
                over the laˇzy dog"},
            indoc! {"
                ˇ
            
                brown fox jumps
                over the lazy dog"},
        );
    }

    #[gpui::test]
    async fn test_a(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["a"]).mode_after(Mode::Insert);

        cx.assert("The qˇuick", "The quˇick");
        cx.assert("The quicˇk", "The quickˇ");
    }

    #[gpui::test]
    async fn test_insert_end_of_line(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["shift-a"]).mode_after(Mode::Insert);
        cx.assert("The qˇuick", "The quickˇ");
        cx.assert("The qˇuick ", "The quick ˇ");
        cx.assert("ˇ", "ˇ");
        cx.assert(
            indoc! {"
                The qˇuick
                brown fox"},
            indoc! {"
                The quickˇ
                brown fox"},
        );
        cx.assert(
            indoc! {"
                ˇ
                The quick"},
            indoc! {"
                ˇ
                The quick"},
        );
    }

    #[gpui::test]
    async fn test_jump_to_first_non_whitespace(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["^"]);
        cx.assert("The qˇuick", "ˇThe quick");
        cx.assert(" The qˇuick", " ˇThe quick");
        cx.assert("ˇ", "ˇ");
        cx.assert(
            indoc! {"
                The qˇuick
                brown fox"},
            indoc! {"
                ˇThe quick
                brown fox"},
        );
        cx.assert(
            indoc! {"
                ˇ
                The quick"},
            indoc! {"
                ˇ
                The quick"},
        );
        // Indoc disallows trailing whitspace.
        cx.assert("   ˇ \nThe quick", "   ˇ \nThe quick");
    }

    #[gpui::test]
    async fn test_insert_first_non_whitespace(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["shift-i"]).mode_after(Mode::Insert);
        cx.assert("The qˇuick", "ˇThe quick");
        cx.assert(" The qˇuick", " ˇThe quick");
        cx.assert("ˇ", "ˇ");
        cx.assert(
            indoc! {"
                The qˇuick
                brown fox"},
            indoc! {"
                ˇThe quick
                brown fox"},
        );
        cx.assert(
            indoc! {"
                ˇ
                The quick"},
            indoc! {"
                ˇ
                The quick"},
        );
    }

    #[gpui::test]
    async fn test_delete_to_end_of_line(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["shift-d"]);
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
    async fn test_x(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["x"]);
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
    async fn test_delete_left(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["shift-x"]);
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
    async fn test_o(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["o"]).mode_after(Mode::Insert);

        cx.assert(
            "ˇ",
            indoc! {"
                
                ˇ"},
        );
        cx.assert(
            "The ˇquick",
            indoc! {"
                The quick
                ˇ"},
        );
        cx.assert(
            indoc! {"
                The quick
                brown ˇfox
                jumps over"},
            indoc! {"
                The quick
                brown fox
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
                brown fox
                jumps over
                ˇ"},
        );
        cx.assert(
            indoc! {"
                The qˇuick
                brown fox
                jumps over"},
            indoc! {"
                The quick
                ˇ
                brown fox
                jumps over"},
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
        cx.assert(
            indoc! {"
                fn test() {
                    println!(ˇ);
                }
            "},
            indoc! {"
                fn test() {
                    println!();
                    ˇ
                }
            "},
        );
        cx.assert(
            indoc! {"
                fn test(ˇ) {
                    println!();
                }"},
            indoc! {"
                fn test() {
                ˇ
                    println!();
                }"},
        );
    }

    #[gpui::test]
    async fn test_insert_line_above(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["shift-o"]).mode_after(Mode::Insert);

        cx.assert(
            "ˇ",
            indoc! {"
                ˇ
                "},
        );
        cx.assert(
            "The ˇquick",
            indoc! {"
                ˇ
                The quick"},
        );
        cx.assert(
            indoc! {"
                The quick
                brown ˇfox
                jumps over"},
            indoc! {"
                The quick
                ˇ
                brown fox
                jumps over"},
        );
        cx.assert(
            indoc! {"
                The quick
                brown fox
                jumps ˇover"},
            indoc! {"
                The quick
                brown fox
                ˇ
                jumps over"},
        );
        cx.assert(
            indoc! {"
                The qˇuick
                brown fox
                jumps over"},
            indoc! {"
                ˇ
                The quick
                brown fox
                jumps over"},
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
        cx.assert(
            indoc! {"
                fn test()
                    println!(ˇ);"},
            indoc! {"
                fn test()
                    ˇ
                    println!();"},
        );
        cx.assert(
            indoc! {"
                fn test(ˇ) {
                    println!();
                }"},
            indoc! {"
                ˇ
                fn test() {
                    println!();
                }"},
        );
    }

    #[gpui::test]
    async fn test_dd(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["d", "d"]);

        cx.assert("ˇ", "ˇ");
        cx.assert("The ˇquick", "ˇ");
        cx.assert(
            indoc! {"
                The quick
                brown ˇfox
                jumps over"},
            indoc! {"
                The quick
                jumps ˇover"},
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
            indoc! {"
                brownˇ fox
                jumps over"},
        );
        cx.assert(
            indoc! {"
                The quick
                ˇ
                brown fox"},
            indoc! {"
                The quick
                ˇbrown fox"},
        );
    }

    #[gpui::test]
    async fn test_cc(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["c", "c"]).mode_after(Mode::Insert);

        cx.assert("ˇ", "ˇ");
        cx.assert("The ˇquick", "ˇ");
        cx.assert(
            indoc! {"
                The quick
                brown ˇfox
                jumps over"},
            indoc! {"
                The quick
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
                brown fox
                jumps over"},
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
    async fn test_p(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.set_state(
            indoc! {"
                The quick brown
                fox juˇmps over
                the lazy dog"},
            Mode::Normal,
        );

        cx.simulate_keystrokes(["d", "d"]);
        cx.assert_editor_state(indoc! {"
            The quick brown
            the laˇzy dog"});

        cx.simulate_keystroke("p");
        cx.assert_state(
            indoc! {"
                The quick brown
                the lazy dog
                ˇfox jumps over"},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
                The quick brown
                fox «jumpˇ»s over
                the lazy dog"},
            Mode::Visual { line: false },
        );
        cx.simulate_keystroke("y");
        cx.set_state(
            indoc! {"
                The quick brown
                fox jumps oveˇr
                the lazy dog"},
            Mode::Normal,
        );
        cx.simulate_keystroke("p");
        cx.assert_state(
            indoc! {"
                The quick brown
                fox jumps overˇjumps
                the lazy dog"},
            Mode::Normal,
        );
    }
}
