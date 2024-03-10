use anyhow::Result;
use std::sync::Arc;

use collections::HashMap;
use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement,
    scroll::Autoscroll,
    Bias, DisplayPoint, Editor,
};
use gpui::{actions, ViewContext, WindowContext};
use language::{Point, Selection, SelectionGoal};
use workspace::Workspace;

use crate::{
    motion::{start_of_line, Motion},
    object::Object,
    state::{Mode, Operator},
    utils::{copy_selections_content, yank_selections_content},
    Vim,
};

actions!(
    vim,
    [
        ToggleVisual,
        ToggleVisualLine,
        ToggleVisualBlock,
        VisualDelete,
        VisualYank,
        OtherEnd,
        SelectNext,
        SelectPrevious,
    ]
);

pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(|_, _: &ToggleVisual, cx: &mut ViewContext<Workspace>| {
        toggle_mode(Mode::Visual, cx)
    });
    workspace.register_action(|_, _: &ToggleVisualLine, cx: &mut ViewContext<Workspace>| {
        toggle_mode(Mode::VisualLine, cx)
    });
    workspace.register_action(
        |_, _: &ToggleVisualBlock, cx: &mut ViewContext<Workspace>| {
            toggle_mode(Mode::VisualBlock, cx)
        },
    );
    workspace.register_action(other_end);
    workspace.register_action(delete);
    workspace.register_action(yank);

    workspace.register_action(|workspace, action, cx| {
        select_next(workspace, action, cx).ok();
    });
    workspace.register_action(|workspace, action, cx| {
        select_previous(workspace, action, cx).ok();
    });
}

pub fn visual_motion(motion: Motion, times: Option<usize>, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |vim, editor, cx| {
            let text_layout_details = editor.text_layout_details(cx);
            if vim.state().mode == Mode::VisualBlock
                && !matches!(
                    motion,
                    Motion::EndOfLine {
                        display_lines: false
                    }
                )
            {
                let is_up_or_down = matches!(motion, Motion::Up { .. } | Motion::Down { .. });
                visual_block_motion(is_up_or_down, editor, cx, |map, point, goal| {
                    motion.move_point(map, point, goal, times, &text_layout_details)
                })
            } else {
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_with(|map, selection| {
                        let was_reversed = selection.reversed;
                        let mut current_head = selection.head();

                        // our motions assume the current character is after the cursor,
                        // but in (forward) visual mode the current character is just
                        // before the end of the selection.

                        // If the file ends with a newline (which is common) we don't do this.
                        // so that if you go to the end of such a file you can use "up" to go
                        // to the previous line and have it work somewhat as expected.
                        #[allow(clippy::nonminimal_bool)]
                        if !selection.reversed
                            && !selection.is_empty()
                            && !(selection.end.column() == 0 && selection.end == map.max_point())
                        {
                            current_head = movement::left(map, selection.end)
                        }

                        let Some((new_head, goal)) = motion.move_point(
                            map,
                            current_head,
                            selection.goal,
                            times,
                            &text_layout_details,
                        ) else {
                            return;
                        };

                        selection.set_head(new_head, goal);

                        // ensure the current character is included in the selection.
                        if !selection.reversed {
                            let next_point = if vim.state().mode == Mode::VisualBlock {
                                movement::saturating_right(map, selection.end)
                            } else {
                                movement::right(map, selection.end)
                            };

                            if !(next_point.column() == 0 && next_point == map.max_point()) {
                                selection.end = next_point;
                            }
                        }

                        // vim always ensures the anchor character stays selected.
                        // if our selection has reversed, we need to move the opposite end
                        // to ensure the anchor is still selected.
                        if was_reversed && !selection.reversed {
                            selection.start = movement::left(map, selection.start);
                        } else if !was_reversed && selection.reversed {
                            selection.end = movement::right(map, selection.end);
                        }
                    })
                });
            }
        });
    });
}

pub fn visual_block_motion(
    preserve_goal: bool,
    editor: &mut Editor,
    cx: &mut ViewContext<Editor>,
    mut move_selection: impl FnMut(
        &DisplaySnapshot,
        DisplayPoint,
        SelectionGoal,
    ) -> Option<(DisplayPoint, SelectionGoal)>,
) {
    let text_layout_details = editor.text_layout_details(cx);
    editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
        let map = &s.display_map();
        let mut head = s.newest_anchor().head().to_display_point(map);
        let mut tail = s.oldest_anchor().tail().to_display_point(map);

        let mut head_x = map.x_for_display_point(head, &text_layout_details);
        let mut tail_x = map.x_for_display_point(tail, &text_layout_details);

        let (start, end) = match s.newest_anchor().goal {
            SelectionGoal::HorizontalRange { start, end } if preserve_goal => (start, end),
            SelectionGoal::HorizontalPosition(start) if preserve_goal => (start, start),
            _ => (tail_x.0, head_x.0),
        };
        let mut goal = SelectionGoal::HorizontalRange { start, end };

        let was_reversed = tail_x > head_x;
        if !was_reversed && !preserve_goal {
            head = movement::saturating_left(map, head);
        }

        let Some((new_head, _)) = move_selection(&map, head, goal) else {
            return;
        };
        head = new_head;
        head_x = map.x_for_display_point(head, &text_layout_details);

        let is_reversed = tail_x > head_x;
        if was_reversed && !is_reversed {
            tail = movement::saturating_left(map, tail);
            tail_x = map.x_for_display_point(tail, &text_layout_details);
        } else if !was_reversed && is_reversed {
            tail = movement::saturating_right(map, tail);
            tail_x = map.x_for_display_point(tail, &text_layout_details);
        }
        if !is_reversed && !preserve_goal {
            head = movement::saturating_right(map, head);
            head_x = map.x_for_display_point(head, &text_layout_details);
        }

        let positions = if is_reversed {
            head_x..tail_x
        } else {
            tail_x..head_x
        };

        if !preserve_goal {
            goal = SelectionGoal::HorizontalRange {
                start: positions.start.0,
                end: positions.end.0,
            };
        }

        let mut selections = Vec::new();
        let mut row = tail.row();

        loop {
            let laid_out_line = map.layout_row(row, &text_layout_details);
            let start = DisplayPoint::new(
                row,
                laid_out_line.closest_index_for_x(positions.start) as u32,
            );
            let mut end =
                DisplayPoint::new(row, laid_out_line.closest_index_for_x(positions.end) as u32);
            if end <= start {
                if start.column() == map.line_len(start.row()) {
                    end = start;
                } else {
                    end = movement::saturating_right(map, start);
                }
            }

            if positions.start <= laid_out_line.width {
                let selection = Selection {
                    id: s.new_selection_id(),
                    start: start.to_point(map),
                    end: end.to_point(map),
                    reversed: is_reversed,
                    goal,
                };

                selections.push(selection);
            }
            if row == head.row() {
                break;
            }
            if tail.row() > head.row() {
                row -= 1
            } else {
                row += 1
            }
        }

        s.select(selections);
    })
}

pub fn visual_object(object: Object, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        if let Some(Operator::Object { around }) = vim.active_operator() {
            vim.pop_operator(cx);
            let current_mode = vim.state().mode;
            let target_mode = object.target_visual_mode(current_mode);
            if target_mode != current_mode {
                vim.switch_mode(target_mode, true, cx);
            }

            vim.update_active_editor(cx, |_, editor, cx| {
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_with(|map, selection| {
                        let mut head = selection.head();

                        // all our motions assume that the current character is
                        // after the cursor; however in the case of a visual selection
                        // the current character is before the cursor.
                        if !selection.reversed {
                            head = movement::left(map, head);
                        }

                        if let Some(range) = object.range(map, head, around) {
                            if !range.is_empty() {
                                let expand_both_ways = object.always_expands_both_ways()
                                    || selection.is_empty()
                                    || movement::right(map, selection.start) == selection.end;

                                if expand_both_ways {
                                    selection.start = range.start;
                                    selection.end = range.end;
                                } else if selection.reversed {
                                    selection.start = range.start;
                                } else {
                                    selection.end = range.end;
                                }
                            }

                            // In the visual selection result of a paragraph object, the cursor is
                            // placed at the start of the last line. And in the visual mode, the
                            // selection end is located after the end character. So, adjustment of
                            // selection end is needed.
                            //
                            // We don't do this adjustment for a one-line blank paragraph since the
                            // trailing newline is included in its selection from the beginning.
                            if object == Object::Paragraph && range.start != range.end {
                                let row_of_selection_end_line = selection.end.to_point(map).row;
                                let new_selection_end =
                                    if map.buffer_snapshot.line_len(row_of_selection_end_line) == 0
                                    {
                                        Point::new(row_of_selection_end_line + 1, 0)
                                    } else {
                                        Point::new(row_of_selection_end_line, 1)
                                    };
                                selection.end = new_selection_end.to_display_point(map);
                            }
                        }
                    });
                });
            });
        }
    });
}

fn toggle_mode(mode: Mode, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        if vim.state().mode == mode {
            vim.switch_mode(Mode::Normal, false, cx);
        } else {
            vim.switch_mode(mode, false, cx);
        }
    })
}

pub fn other_end(_: &mut Workspace, _: &OtherEnd, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.move_with(|_, selection| {
                    selection.reversed = !selection.reversed;
                })
            })
        })
    });
}

pub fn delete(_: &mut Workspace, _: &VisualDelete, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.record_current_action(cx);
        vim.update_active_editor(cx, |vim, editor, cx| {
            let mut original_columns: HashMap<_, _> = Default::default();
            let line_mode = editor.selections.line_mode;

            editor.transact(cx, |editor, cx| {
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_with(|map, selection| {
                        if line_mode {
                            let mut position = selection.head();
                            if !selection.reversed {
                                position = movement::left(map, position);
                            }
                            original_columns.insert(selection.id, position.to_point(map).column);
                        }
                        selection.goal = SelectionGoal::None;
                    });
                });
                copy_selections_content(vim, editor, line_mode, cx);
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
                    if vim.state().mode == Mode::VisualBlock {
                        s.select_anchors(vec![s.first_anchor()])
                    }
                });
            })
        });
        vim.switch_mode(Mode::Normal, true, cx);
    });
}

pub fn yank(_: &mut Workspace, _: &VisualYank, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |vim, editor, cx| {
            let line_mode = editor.selections.line_mode;
            yank_selections_content(vim, editor, line_mode, cx);
            editor.change_selections(None, cx, |s| {
                s.move_with(|map, selection| {
                    if line_mode {
                        selection.start = start_of_line(map, false, selection.start);
                    };
                    selection.collapse_to(selection.start, SelectionGoal::None)
                });
                if vim.state().mode == Mode::VisualBlock {
                    s.select_anchors(vec![s.first_anchor()])
                }
            });
        });
        vim.switch_mode(Mode::Normal, true, cx);
    });
}

pub(crate) fn visual_replace(text: Arc<str>, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.stop_recording();
        vim.update_active_editor(cx, |_, editor, cx| {
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
                    let selection = selection.clone();
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

pub fn select_next(
    _: &mut Workspace,
    _: &SelectNext,
    cx: &mut ViewContext<Workspace>,
) -> Result<()> {
    Vim::update(cx, |vim, cx| {
        let count =
            vim.take_count(cx)
                .unwrap_or_else(|| if vim.state().mode.is_visual() { 1 } else { 2 });
        vim.update_active_editor(cx, |_, editor, cx| {
            for _ in 0..count {
                match editor.select_next(&Default::default(), cx) {
                    Err(a) => return Err(a),
                    _ => {}
                }
            }
            Ok(())
        })
    })
    .unwrap_or(Ok(()))
}

pub fn select_previous(
    _: &mut Workspace,
    _: &SelectPrevious,
    cx: &mut ViewContext<Workspace>,
) -> Result<()> {
    Vim::update(cx, |vim, cx| {
        let count =
            vim.take_count(cx)
                .unwrap_or_else(|| if vim.state().mode.is_visual() { 1 } else { 2 });
        vim.update_active_editor(cx, |_, editor, cx| {
            for _ in 0..count {
                match editor.select_previous(&Default::default(), cx) {
                    Err(a) => return Err(a),
                    _ => {}
                }
            }
            Ok(())
        })
    })
    .unwrap_or(Ok(()))
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use workspace::item::Item;

    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };

    #[gpui::test]
    async fn test_enter_visual_mode(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {
            "The ˇquick brown
            fox jumps over
            the lazy dog"
        })
        .await;
        let cursor = cx.update_editor(|editor, cx| editor.pixel_position_of_cursor(cx));

        // entering visual mode should select the character
        // under cursor
        cx.simulate_shared_keystrokes(["v"]).await;
        cx.assert_shared_state(indoc! { "The «qˇ»uick brown
            fox jumps over
            the lazy dog"})
            .await;
        cx.update_editor(|editor, cx| assert_eq!(cursor, editor.pixel_position_of_cursor(cx)));

        // forwards motions should extend the selection
        cx.simulate_shared_keystrokes(["w", "j"]).await;
        cx.assert_shared_state(indoc! { "The «quick brown
            fox jumps oˇ»ver
            the lazy dog"})
            .await;

        cx.simulate_shared_keystrokes(["escape"]).await;
        assert_eq!(Mode::Normal, cx.neovim_mode().await);
        cx.assert_shared_state(indoc! { "The quick brown
            fox jumps ˇover
            the lazy dog"})
            .await;

        // motions work backwards
        cx.simulate_shared_keystrokes(["v", "k", "b"]).await;
        cx.assert_shared_state(indoc! { "The «ˇquick brown
            fox jumps o»ver
            the lazy dog"})
            .await;

        // works on empty lines
        cx.set_shared_state(indoc! {"
            a
            ˇ
            b
            "})
            .await;
        let cursor = cx.update_editor(|editor, cx| editor.pixel_position_of_cursor(cx));
        cx.simulate_shared_keystrokes(["v"]).await;
        cx.assert_shared_state(indoc! {"
            a
            «
            ˇ»b
        "})
            .await;
        cx.update_editor(|editor, cx| assert_eq!(cursor, editor.pixel_position_of_cursor(cx)));

        // toggles off again
        cx.simulate_shared_keystrokes(["v"]).await;
        cx.assert_shared_state(indoc! {"
            a
            ˇ
            b
            "})
            .await;

        // works at the end of a document
        cx.set_shared_state(indoc! {"
            a
            b
            ˇ"})
            .await;

        cx.simulate_shared_keystrokes(["v"]).await;
        cx.assert_shared_state(indoc! {"
            a
            b
            ˇ"})
            .await;
        assert_eq!(cx.mode(), cx.neovim_mode().await);
    }

    #[gpui::test]
    async fn test_enter_visual_line_mode(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {
            "The ˇquick brown
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["shift-v"]).await;
        cx.assert_shared_state(indoc! { "The «qˇ»uick brown
            fox jumps over
            the lazy dog"})
            .await;
        assert_eq!(cx.mode(), cx.neovim_mode().await);
        cx.simulate_shared_keystrokes(["x"]).await;
        cx.assert_shared_state(indoc! { "fox ˇjumps over
        the lazy dog"})
            .await;

        // it should work on empty lines
        cx.set_shared_state(indoc! {"
            a
            ˇ
            b"})
            .await;
        cx.simulate_shared_keystrokes(["shift-v"]).await;
        cx.assert_shared_state(indoc! { "
            a
            «
            ˇ»b"})
            .await;
        cx.simulate_shared_keystrokes(["x"]).await;
        cx.assert_shared_state(indoc! { "
            a
            ˇb"})
            .await;

        // it should work at the end of the document
        cx.set_shared_state(indoc! {"
            a
            b
            ˇ"})
            .await;
        let cursor = cx.update_editor(|editor, cx| editor.pixel_position_of_cursor(cx));
        cx.simulate_shared_keystrokes(["shift-v"]).await;
        cx.assert_shared_state(indoc! {"
            a
            b
            ˇ"})
            .await;
        assert_eq!(cx.mode(), cx.neovim_mode().await);
        cx.update_editor(|editor, cx| assert_eq!(cursor, editor.pixel_position_of_cursor(cx)));
        cx.simulate_shared_keystrokes(["x"]).await;
        cx.assert_shared_state(indoc! {"
            a
            ˇb"})
            .await;
    }

    #[gpui::test]
    async fn test_visual_delete(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.assert_binding_matches(["v", "w"], "The quick ˇbrown")
            .await;

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
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
                The quˇick brown
                fox jumps over
                the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["shift-v", "x"]).await;
        cx.assert_state_matches().await;

        // Test pasting code copied on delete
        cx.simulate_shared_keystroke("p").await;
        cx.assert_state_matches().await;

        cx.set_shared_state(indoc! {"
                The quick brown
                fox jumps over
                the laˇzy dog"})
            .await;
        cx.simulate_shared_keystrokes(["shift-v", "x"]).await;
        cx.assert_state_matches().await;
        cx.assert_shared_clipboard("the lazy dog\n").await;

        for marked_text in cx.each_marked_position(indoc! {"
                        The quˇick brown
                        fox jumps over
                        the lazy dog"})
        {
            cx.set_shared_state(&marked_text).await;
            cx.simulate_shared_keystrokes(["shift-v", "j", "x"]).await;
            cx.assert_state_matches().await;
            // Test pasting code copied on delete
            cx.simulate_shared_keystroke("p").await;
            cx.assert_state_matches().await;
        }

        cx.set_shared_state(indoc! {"
            The ˇlong line
            should not
            crash
            "})
            .await;
        cx.simulate_shared_keystrokes(["shift-v", "$", "x"]).await;
        cx.assert_state_matches().await;
    }

    #[gpui::test]
    async fn test_visual_yank(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("The quick ˇbrown").await;
        cx.simulate_shared_keystrokes(["v", "w", "y"]).await;
        cx.assert_shared_state("The quick ˇbrown").await;
        cx.assert_shared_clipboard("brown").await;

        cx.set_shared_state(indoc! {"
                The ˇquick brown
                fox jumps over
                the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["v", "w", "j", "y"]).await;
        cx.assert_shared_state(indoc! {"
                    The ˇquick brown
                    fox jumps over
                    the lazy dog"})
            .await;
        cx.assert_shared_clipboard(indoc! {"
                quick brown
                fox jumps o"})
            .await;

        cx.set_shared_state(indoc! {"
                    The quick brown
                    fox jumps over
                    the ˇlazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["v", "w", "j", "y"]).await;
        cx.assert_shared_state(indoc! {"
                    The quick brown
                    fox jumps over
                    the ˇlazy dog"})
            .await;
        cx.assert_shared_clipboard("lazy d").await;
        cx.simulate_shared_keystrokes(["shift-v", "y"]).await;
        cx.assert_shared_clipboard("the lazy dog\n").await;

        let mut cx = cx.binding(["v", "b", "k", "y"]);
        cx.set_shared_state(indoc! {"
                    The ˇquick brown
                    fox jumps over
                    the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["v", "b", "k", "y"]).await;
        cx.assert_shared_state(indoc! {"
                    ˇThe quick brown
                    fox jumps over
                    the lazy dog"})
            .await;
        assert_eq!(
            cx.read_from_clipboard()
                .map(|item| item.text().clone())
                .unwrap(),
            "The q"
        );

        cx.set_shared_state(indoc! {"
                    The quick brown
                    fox ˇjumps over
                    the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes(["shift-v", "shift-g", "shift-y"])
            .await;
        cx.assert_shared_state(indoc! {"
                    The quick brown
                    ˇfox jumps over
                    the lazy dog"})
            .await;
        cx.assert_shared_clipboard("fox jumps over\nthe lazy dog\n")
            .await;
    }

    #[gpui::test]
    async fn test_visual_block_mode(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {
            "The ˇquick brown
             fox jumps over
             the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["ctrl-v"]).await;
        cx.assert_shared_state(indoc! {
            "The «qˇ»uick brown
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["2", "down"]).await;
        cx.assert_shared_state(indoc! {
            "The «qˇ»uick brown
            fox «jˇ»umps over
            the «lˇ»azy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["e"]).await;
        cx.assert_shared_state(indoc! {
            "The «quicˇ»k brown
            fox «jumpˇ»s over
            the «lazyˇ» dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["^"]).await;
        cx.assert_shared_state(indoc! {
            "«ˇThe q»uick brown
            «ˇfox j»umps over
            «ˇthe l»azy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["$"]).await;
        cx.assert_shared_state(indoc! {
            "The «quick brownˇ»
            fox «jumps overˇ»
            the «lazy dogˇ»"
        })
        .await;
        cx.simulate_shared_keystrokes(["shift-f", " "]).await;
        cx.assert_shared_state(indoc! {
            "The «quickˇ» brown
            fox «jumpsˇ» over
            the «lazy ˇ»dog"
        })
        .await;

        // toggling through visual mode works as expected
        cx.simulate_shared_keystrokes(["v"]).await;
        cx.assert_shared_state(indoc! {
            "The «quick brown
            fox jumps over
            the lazy ˇ»dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["ctrl-v"]).await;
        cx.assert_shared_state(indoc! {
            "The «quickˇ» brown
            fox «jumpsˇ» over
            the «lazy ˇ»dog"
        })
        .await;

        cx.set_shared_state(indoc! {
            "The ˇquick
             brown
             fox
             jumps over the

             lazy dog
            "
        })
        .await;
        cx.simulate_shared_keystrokes(["ctrl-v", "down", "down"])
            .await;
        cx.assert_shared_state(indoc! {
            "The«ˇ q»uick
            bro«ˇwn»
            foxˇ
            jumps over the

            lazy dog
            "
        })
        .await;
        cx.simulate_shared_keystrokes(["down"]).await;
        cx.assert_shared_state(indoc! {
            "The «qˇ»uick
            brow«nˇ»
            fox
            jump«sˇ» over the

            lazy dog
            "
        })
        .await;
        cx.simulate_shared_keystroke("left").await;
        cx.assert_shared_state(indoc! {
            "The«ˇ q»uick
            bro«ˇwn»
            foxˇ
            jum«ˇps» over the

            lazy dog
            "
        })
        .await;
        cx.simulate_shared_keystrokes(["s", "o", "escape"]).await;
        cx.assert_shared_state(indoc! {
            "Theˇouick
            broo
            foxo
            jumo over the

            lazy dog
            "
        })
        .await;

        // https://github.com/zed-industries/zed/issues/6274
        cx.set_shared_state(indoc! {
            "Theˇ quick brown

            fox jumps over
            the lazy dog
            "
        })
        .await;
        cx.simulate_shared_keystrokes(["l", "ctrl-v", "j", "j"])
            .await;
        cx.assert_shared_state(indoc! {
            "The «qˇ»uick brown

            fox «jˇ»umps over
            the lazy dog
            "
        })
        .await;
    }

    #[gpui::test]
    async fn test_visual_block_issue_2123(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {
            "The ˇquick brown
            fox jumps over
            the lazy dog
            "
        })
        .await;
        cx.simulate_shared_keystrokes(["ctrl-v", "right", "down"])
            .await;
        cx.assert_shared_state(indoc! {
            "The «quˇ»ick brown
            fox «juˇ»mps over
            the lazy dog
            "
        })
        .await;
    }

    #[gpui::test]
    async fn test_visual_block_insert(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {
            "ˇThe quick brown
            fox jumps over
            the lazy dog
            "
        })
        .await;
        cx.simulate_shared_keystrokes(["ctrl-v", "9", "down"]).await;
        cx.assert_shared_state(indoc! {
            "«Tˇ»he quick brown
            «fˇ»ox jumps over
            «tˇ»he lazy dog
            ˇ"
        })
        .await;

        cx.simulate_shared_keystrokes(["shift-i", "k", "escape"])
            .await;
        cx.assert_shared_state(indoc! {
            "ˇkThe quick brown
            kfox jumps over
            kthe lazy dog
            k"
        })
        .await;

        cx.set_shared_state(indoc! {
            "ˇThe quick brown
            fox jumps over
            the lazy dog
            "
        })
        .await;
        cx.simulate_shared_keystrokes(["ctrl-v", "9", "down"]).await;
        cx.assert_shared_state(indoc! {
            "«Tˇ»he quick brown
            «fˇ»ox jumps over
            «tˇ»he lazy dog
            ˇ"
        })
        .await;
        cx.simulate_shared_keystrokes(["c", "k", "escape"]).await;
        cx.assert_shared_state(indoc! {
            "ˇkhe quick brown
            kox jumps over
            khe lazy dog
            k"
        })
        .await;
    }

    #[gpui::test]
    async fn test_visual_object(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("hello (in [parˇens] o)").await;
        cx.simulate_shared_keystrokes(["ctrl-v", "l"]).await;
        cx.simulate_shared_keystrokes(["a", "]"]).await;
        cx.assert_shared_state("hello (in «[parens]ˇ» o)").await;
        cx.simulate_shared_keystrokes(["i", "("]).await;
        cx.assert_shared_state("hello («in [parens] oˇ»)").await;

        cx.set_shared_state("hello in a wˇord again.").await;
        cx.simulate_shared_keystrokes(["ctrl-v", "l", "i", "w"])
            .await;
        cx.assert_shared_state("hello in a w«ordˇ» again.").await;
        assert_eq!(cx.mode(), Mode::VisualBlock);
        cx.simulate_shared_keystrokes(["o", "a", "s"]).await;
        cx.assert_shared_state("«ˇhello in a word» again.").await;
    }

    #[gpui::test]
    async fn test_mode_across_command(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("aˇbc", Mode::Normal);
        cx.simulate_keystrokes(["ctrl-v"]);
        assert_eq!(cx.mode(), Mode::VisualBlock);
        cx.simulate_keystrokes(["cmd-shift-p", "escape"]);
        assert_eq!(cx.mode(), Mode::VisualBlock);
    }
}
