mod case;
mod change;
mod delete;
mod scroll;
mod search;
mod substitute;
mod yank;

use std::{borrow::Cow, sync::Arc};

use crate::{
    motion::Motion,
    object::Object,
    state::{Mode, Operator},
    Vim,
};
use collections::{HashMap, HashSet};
use editor::{
    display_map::{Clip, ToDisplayPoint},
    scroll::autoscroll::Autoscroll,
    Anchor, Bias, ClipboardSelection, DisplayPoint,
};
use gpui::{actions, AppContext, ViewContext, WindowContext};
use language::{AutoindentMode, Point, SelectionGoal};
use log::error;
use workspace::Workspace;

use self::{
    case::change_case,
    change::{change_motion, change_object},
    delete::{delete_motion, delete_object},
    substitute::substitute,
    yank::{yank_motion, yank_object},
};

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
        Substitute,
        ChangeCase,
    ]
);

pub fn init(cx: &mut AppContext) {
    cx.add_action(insert_after);
    cx.add_action(insert_first_non_whitespace);
    cx.add_action(insert_end_of_line);
    cx.add_action(insert_line_above);
    cx.add_action(insert_line_below);
    cx.add_action(change_case);
    search::init(cx);
    cx.add_action(|_: &mut Workspace, _: &Substitute, cx| {
        Vim::update(cx, |vim, cx| {
            let times = vim.pop_number_operator(cx);
            substitute(vim, times, cx);
        })
    });
    cx.add_action(|_: &mut Workspace, _: &DeleteLeft, cx| {
        Vim::update(cx, |vim, cx| {
            let times = vim.pop_number_operator(cx);
            delete_motion(vim, Motion::Left, times, cx);
        })
    });
    cx.add_action(|_: &mut Workspace, _: &DeleteRight, cx| {
        Vim::update(cx, |vim, cx| {
            let times = vim.pop_number_operator(cx);
            delete_motion(vim, Motion::Right, times, cx);
        })
    });
    cx.add_action(|_: &mut Workspace, _: &ChangeToEndOfLine, cx| {
        Vim::update(cx, |vim, cx| {
            let times = vim.pop_number_operator(cx);
            change_motion(vim, Motion::EndOfLine, times, cx);
        })
    });
    cx.add_action(|_: &mut Workspace, _: &DeleteToEndOfLine, cx| {
        Vim::update(cx, |vim, cx| {
            let times = vim.pop_number_operator(cx);
            delete_motion(vim, Motion::EndOfLine, times, cx);
        })
    });
    cx.add_action(paste);

    scroll::init(cx);
}

pub fn normal_motion(
    motion: Motion,
    operator: Option<Operator>,
    times: Option<usize>,
    cx: &mut WindowContext,
) {
    Vim::update(cx, |vim, cx| {
        match operator {
            None => move_cursor(vim, motion, times, cx),
            Some(Operator::Change) => change_motion(vim, motion, times, cx),
            Some(Operator::Delete) => delete_motion(vim, motion, times, cx),
            Some(Operator::Yank) => yank_motion(vim, motion, times, cx),
            Some(operator) => {
                // Can't do anything for text objects, Ignoring
                error!("Unexpected normal mode motion operator: {:?}", operator)
            }
        }
    });
}

pub fn normal_object(object: Object, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        match vim.state.operator_stack.pop() {
            Some(Operator::Object { around }) => match vim.state.operator_stack.pop() {
                Some(Operator::Change) => change_object(vim, object, around, cx),
                Some(Operator::Delete) => delete_object(vim, object, around, cx),
                Some(Operator::Yank) => yank_object(vim, object, around, cx),
                _ => {
                    // Can't do anything for namespace operators. Ignoring
                }
            },
            _ => {
                // Can't do anything with change/delete/yank and text objects. Ignoring
            }
        }
        vim.clear_operator(cx);
    })
}

fn move_cursor(vim: &mut Vim, motion: Motion, times: Option<usize>, cx: &mut WindowContext) {
    vim.update_active_editor(cx, |editor, cx| {
        editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.move_cursors_with(|map, cursor, goal| {
                motion
                    .move_point(map, cursor, goal, times)
                    .unwrap_or((cursor, goal))
            })
        })
    });
}

fn insert_after(_: &mut Workspace, _: &InsertAfter, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.switch_mode(Mode::Insert, false, cx);
        vim.update_active_editor(cx, |editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.maybe_move_cursors_with(|map, cursor, goal| {
                    Motion::Right.move_point(map, cursor, goal, None)
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
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.maybe_move_cursors_with(|map, cursor, goal| {
                    Motion::FirstNonWhitespace.move_point(map, cursor, goal, None)
                });
            });
        });
    });
}

fn insert_end_of_line(_: &mut Workspace, _: &InsertEndOfLine, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.switch_mode(Mode::Insert, false, cx);
        vim.update_active_editor(cx, |editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.maybe_move_cursors_with(|map, cursor, goal| {
                    Motion::EndOfLine.move_point(map, cursor, goal, None)
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
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
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
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.maybe_move_cursors_with(|map, cursor, goal| {
                        Motion::EndOfLine.move_point(map, cursor, goal, None)
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
                editor.set_default_clip(Clip::None, cx);
                if let Some(item) = cx.read_from_clipboard() {
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

                        // If the pasted text is a single line, the cursor should be placed after
                        // the newly pasted text. This is easiest done with an anchor after the
                        // insertion, and then with a fixup to move the selection back one position.
                        // However if the pasted text is linewise, the cursor should be placed at the start
                        // of the new text on the following line. This is easiest done with a manually adjusted
                        // point.
                        // This enum lets us represent both cases
                        enum NewPosition {
                            Inside(Point),
                            After(Anchor),
                        }
                        let mut new_selections: HashMap<usize, NewPosition> = Default::default();
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
                                    new_selections.insert(
                                        selection.id,
                                        NewPosition::Inside(Point::new(point.row + 1, 0)),
                                    );
                                    point
                                } else {
                                    let mut point = selection.end;
                                    // Paste the text after the current selection
                                    *point.column_mut() = point.column() + 1;
                                    let point = display_map
                                        .clip_point(point, Bias::Right)
                                        .to_point(&display_map);

                                    new_selections.insert(
                                        selection.id,
                                        if to_insert.contains('\n') {
                                            NewPosition::Inside(point)
                                        } else {
                                            NewPosition::After(snapshot.anchor_after(point))
                                        },
                                    );
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

                        editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                            s.move_with(|map, selection| {
                                if let Some(new_position) = new_selections.get(&selection.id) {
                                    match new_position {
                                        NewPosition::Inside(new_point) => {
                                            selection.collapse_to(
                                                new_point.to_display_point(map),
                                                SelectionGoal::None,
                                            );
                                        }
                                        NewPosition::After(after_point) => {
                                            let mut new_point = after_point.to_display_point(map);
                                            *new_point.column_mut() =
                                                new_point.column().saturating_sub(1);
                                            new_point = map.clip_point(new_point, Bias::Left);
                                            selection.collapse_to(new_point, SelectionGoal::None);
                                        }
                                    }
                                }
                            });
                        });
                    } else {
                        editor.insert(&clipboard_text, cx);
                    }
                }
                editor.set_default_clip(Clip::EndOfLine, cx);
            });
        });
    });
}

pub(crate) fn normal_replace(text: Arc<str>, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |editor, cx| {
            editor.transact(cx, |editor, cx| {
                editor.set_default_clip(Clip::None, cx);
                let (map, display_selections) = editor.selections.all_display(cx);
                // Selections are biased right at the start. So we need to store
                // anchors that are biased left so that we can restore the selections
                // after the change
                let stable_anchors = editor
                    .selections
                    .disjoint_anchors()
                    .into_iter()
                    .map(|selection| {
                        let start = selection.start.bias_left(&map.buffer_snapshot);
                        start..start
                    })
                    .collect::<Vec<_>>();

                let edits = display_selections
                    .into_iter()
                    .map(|selection| {
                        let mut range = selection.range();
                        *range.end.column_mut() += 1;
                        range.end = map.clip_point(range.end, Bias::Right);

                        (
                            range.start.to_offset(&map, Bias::Left)
                                ..range.end.to_offset(&map, Bias::Left),
                            text.clone(),
                        )
                    })
                    .collect::<Vec<_>>();

                editor.buffer().update(cx, |buffer, cx| {
                    buffer.edit(edits, None, cx);
                });
                editor.set_default_clip(Clip::EndOfLine, cx);
                editor.change_selections(None, cx, |s| {
                    s.select_anchor_ranges(stable_anchors);
                });
            });
        });
        vim.pop_operator(cx)
    });
}

#[cfg(test)]
mod test {
    use gpui::TestAppContext;
    use indoc::indoc;

    use crate::{
        state::Mode::{self},
        test::{ExemptionFeatures, NeovimBackedTestContext},
    };

    #[gpui::test]
    async fn test_h(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["h"]);
        cx.assert_all(indoc! {"
            ˇThe qˇuick
            ˇbrown"
        })
        .await;
    }

    #[gpui::test]
    async fn test_backspace(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx)
            .await
            .binding(["backspace"]);
        cx.assert_all(indoc! {"
            ˇThe qˇuick
            ˇbrown"
        })
        .await;
    }

    #[gpui::test]
    async fn test_j(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["j"]);
        cx.assert_all(indoc! {"
            ˇThe qˇuick broˇwn
            ˇfox jumps"
        })
        .await;
    }

    #[gpui::test]
    async fn test_enter(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["enter"]);
        cx.assert_all(indoc! {"
            ˇThe qˇuick broˇwn
            ˇfox jumps"
        })
        .await;
    }

    #[gpui::test]
    async fn test_k(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["k"]);
        cx.assert_all(indoc! {"
            ˇThe qˇuick
            ˇbrown fˇox jumˇps"
        })
        .await;
    }

    #[gpui::test]
    async fn test_l(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["l"]);
        cx.assert_all(indoc! {"
            ˇThe qˇuicˇk
            ˇbrowˇn"})
            .await;
    }

    #[gpui::test]
    async fn test_jump_to_line_boundaries(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches_all(
            ["$"],
            indoc! {"
            ˇThe qˇuicˇk
            ˇbrowˇn"},
        )
        .await;
        cx.assert_binding_matches_all(
            ["0"],
            indoc! {"
                ˇThe qˇuicˇk
                ˇbrowˇn"},
        )
        .await;
    }

    #[gpui::test]
    async fn test_jump_to_end(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["shift-g"]);

        cx.assert_all(indoc! {"
                The ˇquick

                brown fox jumps
                overˇ the lazy doˇg"})
            .await;
        cx.assert(indoc! {"
            The quiˇck

            brown"})
            .await;
        cx.assert(indoc! {"
            The quiˇck

            "})
            .await;
    }

    #[gpui::test]
    async fn test_w(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["w"]);
        cx.assert_all(indoc! {"
            The ˇquickˇ-ˇbrown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthˇe"})
            .await;
        let mut cx = cx.binding(["shift-w"]);
        cx.assert_all(indoc! {"
            The ˇquickˇ-ˇbrown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthˇe"})
            .await;
    }

    #[gpui::test]
    async fn test_e(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["e"]);
        cx.assert_all(indoc! {"
            Thˇe quicˇkˇ-browˇn


            fox_jumpˇs oveˇr
            thˇe"})
            .await;
        let mut cx = cx.binding(["shift-e"]);
        cx.assert_all(indoc! {"
            Thˇe quicˇkˇ-browˇn


            fox_jumpˇs oveˇr
            thˇe"})
            .await;
    }

    #[gpui::test]
    async fn test_b(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["b"]);
        cx.assert_all(indoc! {"
            ˇThe ˇquickˇ-ˇbrown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthe"})
            .await;
        let mut cx = cx.binding(["shift-b"]);
        cx.assert_all(indoc! {"
            ˇThe ˇquickˇ-ˇbrown
            ˇ
            ˇ
            ˇfox_jumps ˇover
            ˇthe"})
            .await;
    }

    #[gpui::test]
    async fn test_gg(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches_all(
            ["g", "g"],
            indoc! {"
                The qˇuick

                brown fox jumps
                over ˇthe laˇzy dog"},
        )
        .await;
        cx.assert_binding_matches(
            ["g", "g"],
            indoc! {"


                brown fox jumps
                over the laˇzy dog"},
        )
        .await;
        cx.assert_binding_matches(
            ["2", "g", "g"],
            indoc! {"
                ˇ

                brown fox jumps
                over the lazydog"},
        )
        .await;
    }

    #[gpui::test]
    async fn test_end_of_document(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_binding_matches_all(
            ["shift-g"],
            indoc! {"
                The qˇuick

                brown fox jumps
                over ˇthe laˇzy dog"},
        )
        .await;
        cx.assert_binding_matches(
            ["shift-g"],
            indoc! {"


                brown fox jumps
                over the laˇzy dog"},
        )
        .await;
        cx.assert_binding_matches(
            ["2", "shift-g"],
            indoc! {"
                ˇ

                brown fox jumps
                over the lazydog"},
        )
        .await;
    }

    #[gpui::test]
    async fn test_a(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["a"]);
        cx.assert_all("The qˇuicˇk").await;
    }

    #[gpui::test]
    async fn test_insert_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["shift-a"]);
        cx.assert_all(indoc! {"
            ˇ
            The qˇuick
            brown ˇfox "})
            .await;
    }

    #[gpui::test]
    async fn test_jump_to_first_non_whitespace(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["^"]);
        cx.assert("The qˇuick").await;
        cx.assert(" The qˇuick").await;
        cx.assert("ˇ").await;
        cx.assert(indoc! {"
                The qˇuick
                brown fox"})
            .await;
        cx.assert(indoc! {"
                ˇ
                The quick"})
            .await;
        // Indoc disallows trailing whitespace.
        cx.assert("   ˇ \nThe quick").await;
    }

    #[gpui::test]
    async fn test_insert_first_non_whitespace(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["shift-i"]);
        cx.assert("The qˇuick").await;
        cx.assert(" The qˇuick").await;
        cx.assert("ˇ").await;
        cx.assert(indoc! {"
                The qˇuick
                brown fox"})
            .await;
        cx.assert(indoc! {"
                ˇ
                The quick"})
            .await;
    }

    #[gpui::test]
    async fn test_delete_to_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["shift-d"]);
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
    async fn test_x(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["x"]);
        cx.assert_all("ˇTeˇsˇt").await;
        cx.assert(indoc! {"
                Tesˇt
                test"})
            .await;
    }

    #[gpui::test]
    async fn test_delete_left(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["shift-x"]);
        cx.assert_all("ˇTˇeˇsˇt").await;
        cx.assert(indoc! {"
                Test
                ˇtest"})
            .await;
    }

    #[gpui::test]
    async fn test_o(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["o"]);
        cx.assert("ˇ").await;
        cx.assert("The ˇquick").await;
        cx.assert_all(indoc! {"
                The qˇuick
                brown ˇfox
                jumps ˇover"})
            .await;
        cx.assert(indoc! {"
                The quick
                ˇ
                brown fox"})
            .await;

        cx.assert_manual(
            indoc! {"
                fn test() {
                    println!(ˇ);
                }"},
            Mode::Normal,
            indoc! {"
                fn test() {
                    println!();
                    ˇ
                }"},
            Mode::Insert,
        );

        cx.assert_manual(
            indoc! {"
                fn test(ˇ) {
                    println!();
                }"},
            Mode::Normal,
            indoc! {"
                fn test() {
                    ˇ
                    println!();
                }"},
            Mode::Insert,
        );
    }

    #[gpui::test]
    async fn test_insert_line_above(cx: &mut gpui::TestAppContext) {
        let cx = NeovimBackedTestContext::new(cx).await;
        let mut cx = cx.binding(["shift-o"]);
        cx.assert("ˇ").await;
        cx.assert("The ˇquick").await;
        cx.assert_all(indoc! {"
            The qˇuick
            brown ˇfox
            jumps ˇover"})
            .await;
        cx.assert(indoc! {"
            The quick
            ˇ
            brown fox"})
            .await;

        // Our indentation is smarter than vims. So we don't match here
        cx.assert_manual(
            indoc! {"
                fn test() {
                    println!(ˇ);
                }"},
            Mode::Normal,
            indoc! {"
                fn test() {
                    ˇ
                    println!();
                }"},
            Mode::Insert,
        );
        cx.assert_manual(
            indoc! {"
                fn test(ˇ) {
                    println!();
                }"},
            Mode::Normal,
            indoc! {"
                ˇ
                fn test() {
                    println!();
                }"},
            Mode::Insert,
        );
    }

    #[gpui::test]
    async fn test_dd(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["d", "d"]);
        cx.assert("ˇ").await;
        cx.assert("The ˇquick").await;
        cx.assert_all(indoc! {"
                The qˇuick
                brown ˇfox
                jumps ˇover"})
            .await;
        cx.assert_exempted(
            indoc! {"
                The quick
                ˇ
                brown fox"},
            ExemptionFeatures::DeletionOnEmptyLine,
        )
        .await;
    }

    #[gpui::test]
    async fn test_cc(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["c", "c"]);
        cx.assert("ˇ").await;
        cx.assert("The ˇquick").await;
        cx.assert_all(indoc! {"
                The quˇick
                brown ˇfox
                jumps ˇover"})
            .await;
        cx.assert(indoc! {"
                The quick
                ˇ
                brown fox"})
            .await;
    }

    #[gpui::test]
    async fn test_p(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {"
                The quick brown
                fox juˇmps over
                the lazy dog"})
            .await;

        cx.simulate_shared_keystrokes(["d", "d"]).await;
        cx.assert_state_matches().await;

        cx.simulate_shared_keystroke("p").await;
        cx.assert_state_matches().await;

        cx.set_shared_state(indoc! {"
                The quick brown
                fox ˇjumps over
                the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["v", "w", "y"]).await;
        cx.set_shared_state(indoc! {"
                The quick brown
                fox jumps oveˇr
                the lazy dog"})
            .await;
        cx.simulate_shared_keystroke("p").await;
        cx.assert_state_matches().await;
    }

    #[gpui::test]
    async fn test_repeated_word(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        for count in 1..=5 {
            cx.assert_binding_matches_all(
                [&count.to_string(), "w"],
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
    async fn test_h_through_unicode(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["h"]);
        cx.assert_all("Testˇ├ˇ──ˇ┐ˇTest").await;
    }

    #[gpui::test]
    async fn test_f_and_t(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        for count in 1..=3 {
            let test_case = indoc! {"
                ˇaaaˇbˇ ˇbˇ   ˇbˇbˇ aˇaaˇbaaa
                ˇ    ˇbˇaaˇa ˇbˇbˇb
                ˇ
                ˇb
            "};

            cx.assert_binding_matches_all([&count.to_string(), "f", "b"], test_case)
                .await;

            cx.assert_binding_matches_all([&count.to_string(), "t", "b"], test_case)
                .await;
        }
    }

    #[gpui::test]
    async fn test_capital_f_and_capital_t(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        let test_case = indoc! {"
            ˇaaaˇbˇ ˇbˇ   ˇbˇbˇ aˇaaˇbaaa
            ˇ    ˇbˇaaˇa ˇbˇbˇb
            ˇ•••
            ˇb
            "
        };

        for count in 1..=3 {
            cx.assert_binding_matches_all([&count.to_string(), "shift-f", "b"], test_case)
                .await;

            cx.assert_binding_matches_all([&count.to_string(), "shift-t", "b"], test_case)
                .await;
        }
    }

    #[gpui::test]
    async fn test_percent(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await.binding(["%"]);
        cx.assert_all("ˇconsole.logˇ(ˇvaˇrˇ)ˇ;").await;
        cx.assert_all("ˇconsole.logˇ(ˇ'var', ˇ[ˇ1, ˇ2, 3ˇ]ˇ)ˇ;")
            .await;
        cx.assert_all("let result = curried_funˇ(ˇ)ˇ(ˇ)ˇ;").await;
    }
}
