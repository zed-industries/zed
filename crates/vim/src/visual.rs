use std::{borrow::Cow, sync::Arc};

use collections::HashMap;
use editor::{
    display_map::ToDisplayPoint, movement, scroll::autoscroll::Autoscroll, Bias, ClipboardSelection,
};
use gpui::{actions, AppContext, ViewContext, WindowContext};
use language::{AutoindentMode, SelectionGoal};
use workspace::Workspace;

use crate::{
    motion::Motion,
    object::Object,
    state::{Mode, Operator},
    utils::copy_selections_content,
    Vim,
};

actions!(vim, [VisualDelete, VisualChange, VisualYank, VisualPaste]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(change);
    cx.add_action(delete);
    cx.add_action(yank);
    cx.add_action(paste);
}

pub fn visual_motion(motion: Motion, times: usize, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    let was_reversed = selection.reversed;

                    if let Some((new_head, goal)) =
                        motion.move_point(map, selection.head(), selection.goal, times)
                    {
                        selection.set_head(new_head, goal);

                        if was_reversed && !selection.reversed {
                            // Head was at the start of the selection, and now is at the end. We need to move the start
                            // back by one if possible in order to compensate for this change.
                            *selection.start.column_mut() =
                                selection.start.column().saturating_sub(1);
                            selection.start = map.clip_point(selection.start, Bias::Left);
                        } else if !was_reversed && selection.reversed {
                            // Head was at the end of the selection, and now is at the start. We need to move the end
                            // forward by one if possible in order to compensate for this change.
                            *selection.end.column_mut() = selection.end.column() + 1;
                            selection.end = map.clip_point(selection.end, Bias::Right);
                        }
                    }
                });
            });
        });
    });
}

pub fn visual_object(object: Object, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        if let Operator::Object { around } = vim.pop_operator(cx) {
            vim.update_active_editor(cx, |editor, cx| {
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_with(|map, selection| {
                        let head = selection.head();
                        if let Some(mut range) = object.range(map, head, around) {
                            if !range.is_empty() {
                                if let Some((_, end)) = map.reverse_chars_at(range.end).next() {
                                    range.end = end;
                                }

                                if selection.is_empty() {
                                    selection.start = range.start;
                                    selection.end = range.end;
                                } else if selection.reversed {
                                    selection.start = range.start;
                                } else {
                                    selection.end = range.end;
                                }
                            }
                        }
                    });
                });
            });
        }
    });
}

pub fn change(_: &mut Workspace, _: &VisualChange, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            // Compute edits and resulting anchor selections. If in line mode, adjust
            // the anchor location and additional newline
            let mut edits = Vec::new();
            let mut new_selections = Vec::new();
            let line_mode = editor.selections.line_mode;
            editor.change_selections(None, cx, |s| {
                s.move_with(|map, selection| {
                    if !selection.reversed {
                        // Head is at the end of the selection. Adjust the end position to
                        // to include the character under the cursor.
                        *selection.end.column_mut() = selection.end.column() + 1;
                        selection.end = map.clip_point(selection.end, Bias::Right);
                    }

                    if line_mode {
                        let range = selection.map(|p| p.to_point(map)).range();
                        let expanded_range = map.expand_to_line(range);
                        // If we are at the last line, the anchor needs to be after the newline so that
                        // it is on a line of its own. Otherwise, the anchor may be after the newline
                        let anchor = if expanded_range.end == map.buffer_snapshot.max_point() {
                            map.buffer_snapshot.anchor_after(expanded_range.end)
                        } else {
                            map.buffer_snapshot.anchor_before(expanded_range.start)
                        };

                        edits.push((expanded_range, "\n"));
                        new_selections.push(selection.map(|_| anchor));
                    } else {
                        let range = selection.map(|p| p.to_point(map)).range();
                        let anchor = map.buffer_snapshot.anchor_after(range.end);
                        edits.push((range, ""));
                        new_selections.push(selection.map(|_| anchor));
                    }
                    selection.goal = SelectionGoal::None;
                });
            });
            copy_selections_content(editor, editor.selections.line_mode, cx);
            editor.edit_with_autoindent(edits, cx);
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select_anchors(new_selections);
            });
        });
        vim.switch_mode(Mode::Insert, false, cx);
    });
}

pub fn delete(_: &mut Workspace, _: &VisualDelete, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            let mut original_columns: HashMap<_, _> = Default::default();
            let line_mode = editor.selections.line_mode;
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    if line_mode {
                        original_columns
                            .insert(selection.id, selection.head().to_point(map).column);
                    } else if !selection.reversed {
                        // Head is at the end of the selection. Adjust the end position to
                        // to include the character under the cursor.
                        *selection.end.column_mut() = selection.end.column() + 1;
                        selection.end = map.clip_point(selection.end, Bias::Right);
                    }
                    selection.goal = SelectionGoal::None;
                });
            });
            copy_selections_content(editor, line_mode, cx);
            editor.insert("", cx);

            // Fixup cursor position after the deletion
            editor.set_clip_at_line_ends(true, cx);
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    let mut cursor = selection.head().to_point(map);

                    if let Some(column) = original_columns.get(&selection.id) {
                        cursor.column = *column
                    }
                    let cursor = map.clip_point(cursor.to_display_point(map), Bias::Left);
                    selection.collapse_to(cursor, selection.goal)
                });
            });
        });
        vim.switch_mode(Mode::Normal, false, cx);
    });
}

pub fn yank(_: &mut Workspace, _: &VisualYank, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            let line_mode = editor.selections.line_mode;
            if !line_mode {
                editor.change_selections(None, cx, |s| {
                    s.move_with(|map, selection| {
                        if !selection.reversed {
                            // Head is at the end of the selection. Adjust the end position to
                            // to include the character under the cursor.
                            *selection.end.column_mut() = selection.end.column() + 1;
                            selection.end = map.clip_point(selection.end, Bias::Right);
                        }
                    });
                });
            }
            copy_selections_content(editor, line_mode, cx);
            editor.change_selections(None, cx, |s| {
                s.move_with(|_, selection| {
                    selection.collapse_to(selection.start, SelectionGoal::None)
                });
            });
        });
        vim.switch_mode(Mode::Normal, false, cx);
    });
}

pub fn paste(_: &mut Workspace, _: &VisualPaste, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |editor, cx| {
            editor.transact(cx, |editor, cx| {
                if let Some(item) = cx.read_from_clipboard() {
                    copy_selections_content(editor, editor.selections.line_mode, cx);
                    let mut clipboard_text = Cow::Borrowed(item.text());
                    if let Some(mut clipboard_selections) =
                        item.metadata::<Vec<ClipboardSelection>>()
                    {
                        let (display_map, selections) = editor.selections.all_adjusted_display(cx);
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

                                let mut selection = selection.clone();
                                if !selection.reversed {
                                    let mut adjusted = selection.end;
                                    // Head is at the end of the selection. Adjust the end position to
                                    // to include the character under the cursor.
                                    *adjusted.column_mut() = adjusted.column() + 1;
                                    adjusted = display_map.clip_point(adjusted, Bias::Right);
                                    // If the selection is empty, move both the start and end forward one
                                    // character
                                    if selection.is_empty() {
                                        selection.start = adjusted;
                                        selection.end = adjusted;
                                    } else {
                                        selection.end = adjusted;
                                    }
                                }

                                let range = selection.map(|p| p.to_point(&display_map)).range();

                                let new_position = if linewise {
                                    edits.push((range.start..range.start, "\n"));
                                    let mut new_position = range.start;
                                    new_position.column = 0;
                                    new_position.row += 1;
                                    new_position
                                } else {
                                    range.start
                                };

                                new_selections.push(selection.map(|_| new_position));

                                if linewise && to_insert.ends_with('\n') {
                                    edits.push((
                                        range.clone(),
                                        &to_insert[0..to_insert.len().saturating_sub(1)],
                                    ))
                                } else {
                                    edits.push((range.clone(), to_insert));
                                }

                                if linewise {
                                    edits.push((range.end..range.end, "\n"));
                                }
                            }
                            drop(snapshot);
                            buffer.edit(edits, Some(AutoindentMode::EachLine), cx);
                        });

                        editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                            s.select(new_selections)
                        });
                    } else {
                        editor.insert(&clipboard_text, cx);
                    }
                }
            });
        });
        vim.switch_mode(Mode::Normal, false, cx);
    });
}

pub(crate) fn visual_replace(text: Arc<str>, line: bool, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |editor, cx| {
            editor.transact(cx, |editor, cx| {
                let (display_map, selections) = editor.selections.all_adjusted_display(cx);

                // Selections are biased right at the start. So we need to store
                // anchors that are biased left so that we can restore the selections
                // after the change
                let stable_anchors = editor
                    .selections
                    .disjoint_anchors()
                    .into_iter()
                    .map(|selection| {
                        let start = selection.start.bias_left(&display_map.buffer_snapshot);
                        start..start
                    })
                    .collect::<Vec<_>>();

                let mut edits = Vec::new();
                for selection in selections.iter() {
                    let mut selection = selection.clone();
                    if !line && !selection.reversed {
                        // Head is at the end of the selection. Adjust the end position to
                        // to include the character under the cursor.
                        *selection.end.column_mut() = selection.end.column() + 1;
                        selection.end = display_map.clip_point(selection.end, Bias::Right);
                    }

                    for row_range in
                        movement::split_display_range_by_lines(&display_map, selection.range())
                    {
                        let range = row_range.start.to_offset(&display_map, Bias::Right)
                            ..row_range.end.to_offset(&display_map, Bias::Right);
                        let text = text.repeat(range.len());
                        edits.push((range, text));
                    }
                }

                editor.buffer().update(cx, |buffer, cx| {
                    buffer.edit(edits, None, cx);
                });
                editor.change_selections(None, cx, |s| s.select_ranges(stable_anchors));
            });
        });
        vim.switch_mode(Mode::Normal, false, cx);
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
    async fn test_enter_visual_mode(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx)
            .await
            .binding(["v", "w", "j"]);
        cx.assert_all(indoc! {"
                The ˇquick brown
                fox jumps ˇover
                the ˇlazy dog"})
            .await;
        let mut cx = cx.binding(["v", "b", "k"]);
        cx.assert_all(indoc! {"
                The ˇquick brown
                fox jumps ˇover
                the ˇlazy dog"})
            .await;
    }

    #[gpui::test]
    async fn test_visual_delete(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.assert_binding_matches(["v", "w", "x"], "The quick ˇbrown")
            .await;
        cx.assert_binding_matches(
            ["v", "w", "j", "x"],
            indoc! {"
                The ˇquick brown
                fox jumps over
                the lazy dog"},
        )
        .await;
        // Test pasting code copied on delete
        cx.simulate_shared_keystrokes(["j", "p"]).await;
        cx.assert_state_matches().await;

        let mut cx = cx.binding(["v", "w", "j", "x"]);
        cx.assert_all(indoc! {"
                The ˇquick brown
                fox jumps over
                the ˇlazy dog"})
            .await;
        let mut cx = cx.binding(["v", "b", "k", "x"]);
        cx.assert_all(indoc! {"
                The ˇquick brown
                fox jumps ˇover
                the ˇlazy dog"})
            .await;
    }

    #[gpui::test]
    async fn test_visual_line_delete(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx)
            .await
            .binding(["shift-v", "x"]);
        cx.assert(indoc! {"
                The quˇick brown
                fox jumps over
                the lazy dog"})
            .await;
        // Test pasting code copied on delete
        cx.simulate_shared_keystroke("p").await;
        cx.assert_state_matches().await;

        cx.assert_all(indoc! {"
                The quick brown
                fox juˇmps over
                the laˇzy dog"})
            .await;
        let mut cx = cx.binding(["shift-v", "j", "x"]);
        cx.assert(indoc! {"
                The quˇick brown
                fox jumps over
                the lazy dog"})
            .await;
        // Test pasting code copied on delete
        cx.simulate_shared_keystroke("p").await;
        cx.assert_state_matches().await;

        cx.assert_all(indoc! {"
                The quick brown
                fox juˇmps over
                the laˇzy dog"})
            .await;
    }

    #[gpui::test]
    async fn test_visual_change(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx)
            .await
            .binding(["v", "w", "c"]);
        cx.assert("The quick ˇbrown").await;
        let mut cx = cx.binding(["v", "w", "j", "c"]);
        cx.assert_all(indoc! {"
                The ˇquick brown
                fox jumps ˇover
                the ˇlazy dog"})
            .await;
        let mut cx = cx.binding(["v", "b", "k", "c"]);
        cx.assert_all(indoc! {"
                The ˇquick brown
                fox jumps ˇover
                the ˇlazy dog"})
            .await;
    }

    #[gpui::test]
    async fn test_visual_line_change(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx)
            .await
            .binding(["shift-v", "c"]);
        cx.assert(indoc! {"
                The quˇick brown
                fox jumps over
                the lazy dog"})
            .await;
        // Test pasting code copied on change
        cx.simulate_shared_keystrokes(["escape", "j", "p"]).await;
        cx.assert_state_matches().await;

        cx.assert_all(indoc! {"
                The quick brown
                fox juˇmps over
                the laˇzy dog"})
            .await;
        let mut cx = cx.binding(["shift-v", "j", "c"]);
        cx.assert(indoc! {"
                The quˇick brown
                fox jumps over
                the lazy dog"})
            .await;
        // Test pasting code copied on delete
        cx.simulate_shared_keystrokes(["escape", "j", "p"]).await;
        cx.assert_state_matches().await;

        cx.assert_all(indoc! {"
                The quick brown
                fox juˇmps over
                the laˇzy dog"})
            .await;
    }

    #[gpui::test]
    async fn test_visual_yank(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["v", "w", "y"]);
        cx.assert("The quick ˇbrown", "The quick ˇbrown");
        cx.assert_clipboard_content(Some("brown"));
        let mut cx = cx.binding(["v", "w", "j", "y"]);
        cx.assert(
            indoc! {"
                The ˇquick brown
                fox jumps over
                the lazy dog"},
            indoc! {"
                The ˇquick brown
                fox jumps over
                the lazy dog"},
        );
        cx.assert_clipboard_content(Some(indoc! {"
            quick brown
            fox jumps o"}));
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps over
                the ˇlazy dog"},
            indoc! {"
                The quick brown
                fox jumps over
                the ˇlazy dog"},
        );
        cx.assert_clipboard_content(Some("lazy d"));
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps ˇover
                the lazy dog"},
            indoc! {"
                The quick brown
                fox jumps ˇover
                the lazy dog"},
        );
        cx.assert_clipboard_content(Some(indoc! {"
                over
                t"}));
        let mut cx = cx.binding(["v", "b", "k", "y"]);
        cx.assert(
            indoc! {"
                The ˇquick brown
                fox jumps over
                the lazy dog"},
            indoc! {"
                ˇThe quick brown
                fox jumps over
                the lazy dog"},
        );
        cx.assert_clipboard_content(Some("The q"));
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps over
                the ˇlazy dog"},
            indoc! {"
                The quick brown
                ˇfox jumps over
                the lazy dog"},
        );
        cx.assert_clipboard_content(Some(indoc! {"
            fox jumps over
            the l"}));
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps ˇover
                the lazy dog"},
            indoc! {"
                The ˇquick brown
                fox jumps over
                the lazy dog"},
        );
        cx.assert_clipboard_content(Some(indoc! {"
            quick brown
            fox jumps o"}));
    }

    #[gpui::test]
    async fn test_visual_paste(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
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
                fox jumpˇs over
                the lazy dog"},
            Mode::Normal,
        );
        cx.simulate_keystroke("p");
        cx.assert_state(
            indoc! {"
                The quick brown
                fox jumpsjumpˇs over
                the lazy dog"},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
                The quick brown
                fox juˇmps over
                the lazy dog"},
            Mode::Visual { line: true },
        );
        cx.simulate_keystroke("d");
        cx.assert_state(
            indoc! {"
                The quick brown
                the laˇzy dog"},
            Mode::Normal,
        );
        cx.set_state(
            indoc! {"
                The quick brown
                the «lazˇ»y dog"},
            Mode::Visual { line: false },
        );
        cx.simulate_keystroke("p");
        cx.assert_state(
            &indoc! {"
                The quick brown
                the_
                ˇfox jumps over
                dog"}
            .replace("_", " "), // Hack for trailing whitespace
            Mode::Normal,
        );
    }
}
