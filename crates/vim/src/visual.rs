use std::sync::Arc;

use collections::HashMap;
use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement,
    scroll::Autoscroll,
    Bias, DisplayPoint, Editor, ToOffset,
};
use gpui::{actions, ViewContext};
use language::{Point, Selection, SelectionGoal};
use multi_buffer::MultiBufferRow;
use search::BufferSearchBar;
use util::ResultExt;
use workspace::searchable::Direction;

use crate::{
    motion::{first_non_whitespace, next_line_end, start_of_line, Motion},
    object::Object,
    state::{Mode, Operator},
    Vim,
};

actions!(
    vim,
    [
        ToggleVisual,
        ToggleVisualLine,
        ToggleVisualBlock,
        VisualDelete,
        VisualDeleteLine,
        VisualYank,
        VisualYankLine,
        OtherEnd,
        SelectNext,
        SelectPrevious,
        SelectNextMatch,
        SelectPreviousMatch,
        RestoreVisualSelection,
        VisualInsertEndOfLine,
        VisualInsertFirstNonWhiteSpace,
    ]
);

pub fn register(editor: &mut Editor, cx: &mut ViewContext<Vim>) {
    Vim::action(editor, cx, |vim, _: &ToggleVisual, cx| {
        vim.toggle_mode(Mode::Visual, cx)
    });
    Vim::action(editor, cx, |vim, _: &ToggleVisualLine, cx| {
        vim.toggle_mode(Mode::VisualLine, cx)
    });
    Vim::action(editor, cx, |vim, _: &ToggleVisualBlock, cx| {
        vim.toggle_mode(Mode::VisualBlock, cx)
    });
    Vim::action(editor, cx, Vim::other_end);
    Vim::action(editor, cx, Vim::visual_insert_end_of_line);
    Vim::action(editor, cx, Vim::visual_insert_first_non_white_space);
    Vim::action(editor, cx, |vim, _: &VisualDelete, cx| {
        vim.record_current_action(cx);
        vim.visual_delete(false, cx);
    });
    Vim::action(editor, cx, |vim, _: &VisualDeleteLine, cx| {
        vim.record_current_action(cx);
        vim.visual_delete(true, cx);
    });
    Vim::action(editor, cx, |vim, _: &VisualYank, cx| vim.visual_yank(cx));

    Vim::action(editor, cx, Vim::select_next);
    Vim::action(editor, cx, Vim::select_previous);
    Vim::action(editor, cx, |vim, _: &SelectNextMatch, cx| {
        vim.select_match(Direction::Next, cx);
    });
    Vim::action(editor, cx, |vim, _: &SelectPreviousMatch, cx| {
        vim.select_match(Direction::Prev, cx);
    });

    Vim::action(editor, cx, |vim, _: &RestoreVisualSelection, cx| {
        let Some((stored_mode, reversed)) = vim.stored_visual_mode.take() else {
            return;
        };
        let Some((start, end)) = vim.marks.get("<").zip(vim.marks.get(">")) else {
            return;
        };
        let ranges = start
            .iter()
            .zip(end)
            .zip(reversed)
            .map(|((start, end), reversed)| (*start, *end, reversed))
            .collect::<Vec<_>>();

        if vim.mode.is_visual() {
            vim.create_visual_marks(vim.mode, cx);
        }

        vim.update_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                let map = s.display_map();
                let ranges = ranges
                    .into_iter()
                    .map(|(start, end, reversed)| {
                        let new_end = movement::saturating_right(&map, end.to_display_point(&map));
                        Selection {
                            id: s.new_selection_id(),
                            start: start.to_offset(&map.buffer_snapshot),
                            end: new_end.to_offset(&map, Bias::Left),
                            reversed,
                            goal: SelectionGoal::None,
                        }
                    })
                    .collect();
                s.select(ranges);
            })
        });
        vim.switch_mode(stored_mode, true, cx)
    });
}

impl Vim {
    pub fn visual_motion(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        cx: &mut ViewContext<Self>,
    ) {
        self.update_editor(cx, |vim, editor, cx| {
            let text_layout_details = editor.text_layout_details(cx);
            if vim.mode == Mode::VisualBlock
                && !matches!(
                    motion,
                    Motion::EndOfLine {
                        display_lines: false
                    }
                )
            {
                let is_up_or_down = matches!(motion, Motion::Up { .. } | Motion::Down { .. });
                vim.visual_block_motion(is_up_or_down, editor, cx, |map, point, goal| {
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
                            let next_point = if vim.mode == Mode::VisualBlock {
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
    }

    pub fn visual_block_motion(
        &mut self,
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

            let Some((new_head, _)) = move_selection(map, head, goal) else {
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
                    row.0 -= 1
                } else {
                    row.0 += 1
                }
            }

            s.select(selections);
        })
    }

    pub fn visual_object(&mut self, object: Object, cx: &mut ViewContext<Vim>) {
        if let Some(Operator::Object { around }) = self.active_operator() {
            self.pop_operator(cx);
            let current_mode = self.mode;
            let target_mode = object.target_visual_mode(current_mode, around);
            if target_mode != current_mode {
                self.switch_mode(target_mode, true, cx);
            }

            self.update_editor(cx, |_, editor, cx| {
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_with(|map, selection| {
                        let mut mut_selection = selection.clone();

                        // all our motions assume that the current character is
                        // after the cursor; however in the case of a visual selection
                        // the current character is before the cursor.
                        // But this will affect the judgment of the html tag
                        // so the html tag needs to skip this logic.
                        if !selection.reversed && object != Object::Tag {
                            mut_selection.set_head(
                                movement::left(map, mut_selection.head()),
                                mut_selection.goal,
                            );
                        }

                        if let Some(range) = object.range(map, mut_selection, around) {
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
                                let new_selection_end = if map
                                    .buffer_snapshot
                                    .line_len(MultiBufferRow(row_of_selection_end_line))
                                    == 0
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
    }

    fn visual_insert_end_of_line(&mut self, _: &VisualInsertEndOfLine, cx: &mut ViewContext<Self>) {
        self.update_editor(cx, |_, editor, cx| {
            editor.split_selection_into_lines(&Default::default(), cx);
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_cursors_with(|map, cursor, _| {
                    (next_line_end(map, cursor, 1), SelectionGoal::None)
                });
            });
        });

        self.switch_mode(Mode::Insert, false, cx);
    }

    fn visual_insert_first_non_white_space(
        &mut self,
        _: &VisualInsertFirstNonWhiteSpace,
        cx: &mut ViewContext<Self>,
    ) {
        self.update_editor(cx, |_, editor, cx| {
            editor.split_selection_into_lines(&Default::default(), cx);
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_cursors_with(|map, cursor, _| {
                    (
                        first_non_whitespace(map, false, cursor),
                        SelectionGoal::None,
                    )
                });
            });
        });

        self.switch_mode(Mode::Insert, false, cx);
    }

    fn toggle_mode(&mut self, mode: Mode, cx: &mut ViewContext<Self>) {
        if self.mode == mode {
            self.switch_mode(Mode::Normal, false, cx);
        } else {
            self.switch_mode(mode, false, cx);
        }
    }

    pub fn other_end(&mut self, _: &OtherEnd, cx: &mut ViewContext<Self>) {
        self.update_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|_, selection| {
                    selection.reversed = !selection.reversed;
                })
            })
        });
    }

    pub fn visual_delete(&mut self, line_mode: bool, cx: &mut ViewContext<Self>) {
        self.store_visual_marks(cx);
        self.update_editor(cx, |vim, editor, cx| {
            let mut original_columns: HashMap<_, _> = Default::default();
            let line_mode = line_mode || editor.selections.line_mode;

            editor.transact(cx, |editor, cx| {
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_with(|map, selection| {
                        if line_mode {
                            let mut position = selection.head();
                            if !selection.reversed {
                                position = movement::left(map, position);
                            }
                            original_columns.insert(selection.id, position.to_point(map).column);
                            if vim.mode == Mode::VisualBlock {
                                *selection.end.column_mut() = map.line_len(selection.end.row())
                            } else if vim.mode != Mode::VisualLine {
                                selection.start = DisplayPoint::new(selection.start.row(), 0);
                                if selection.end.row() == map.max_point().row() {
                                    selection.end = map.max_point()
                                } else {
                                    *selection.end.row_mut() += 1;
                                    *selection.end.column_mut() = 0;
                                }
                            }
                        }
                        selection.goal = SelectionGoal::None;
                    });
                });
                vim.copy_selections_content(editor, line_mode, cx);
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
                    if vim.mode == Mode::VisualBlock {
                        s.select_anchors(vec![s.first_anchor()])
                    }
                });
            })
        });
        self.switch_mode(Mode::Normal, true, cx);
    }

    pub fn visual_yank(&mut self, cx: &mut ViewContext<Self>) {
        self.store_visual_marks(cx);
        self.update_editor(cx, |vim, editor, cx| {
            let line_mode = editor.selections.line_mode;
            vim.yank_selections_content(editor, line_mode, cx);
            editor.change_selections(None, cx, |s| {
                s.move_with(|map, selection| {
                    if line_mode {
                        selection.start = start_of_line(map, false, selection.start);
                    };
                    selection.collapse_to(selection.start, SelectionGoal::None)
                });
                if vim.mode == Mode::VisualBlock {
                    s.select_anchors(vec![s.first_anchor()])
                }
            });
        });
        self.switch_mode(Mode::Normal, true, cx);
    }

    pub(crate) fn visual_replace(&mut self, text: Arc<str>, cx: &mut ViewContext<Self>) {
        self.stop_recording(cx);
        self.update_editor(cx, |_, editor, cx| {
            editor.transact(cx, |editor, cx| {
                let (display_map, selections) = editor.selections.all_adjusted_display(cx);

                // Selections are biased right at the start. So we need to store
                // anchors that are biased left so that we can restore the selections
                // after the change
                let stable_anchors = editor
                    .selections
                    .disjoint_anchors()
                    .iter()
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

                editor.edit(edits, cx);
                editor.change_selections(None, cx, |s| s.select_ranges(stable_anchors));
            });
        });
        self.switch_mode(Mode::Normal, false, cx);
    }

    pub fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        let count =
            Vim::take_count(cx).unwrap_or_else(|| if self.mode.is_visual() { 1 } else { 2 });
        self.update_editor(cx, |_, editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            for _ in 0..count {
                if editor
                    .select_next(&Default::default(), cx)
                    .log_err()
                    .is_none()
                {
                    break;
                }
            }
        });
    }

    pub fn select_previous(&mut self, _: &SelectPrevious, cx: &mut ViewContext<Self>) {
        let count =
            Vim::take_count(cx).unwrap_or_else(|| if self.mode.is_visual() { 1 } else { 2 });
        self.update_editor(cx, |_, editor, cx| {
            for _ in 0..count {
                if editor
                    .select_previous(&Default::default(), cx)
                    .log_err()
                    .is_none()
                {
                    break;
                }
            }
        });
    }

    pub fn select_match(&mut self, direction: Direction, cx: &mut ViewContext<Self>) {
        let count = Vim::take_count(cx).unwrap_or(1);
        let Some(pane) = self.pane(cx) else {
            return;
        };
        let vim_is_normal = self.mode == Mode::Normal;
        let mut start_selection = 0usize;
        let mut end_selection = 0usize;

        self.update_editor(cx, |_, editor, _| {
            editor.set_collapse_matches(false);
        });
        if vim_is_normal {
            pane.update(cx, |pane, cx| {
                if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>()
                {
                    search_bar.update(cx, |search_bar, cx| {
                        if !search_bar.has_active_match() || !search_bar.show(cx) {
                            return;
                        }
                        // without update_match_index there is a bug when the cursor is before the first match
                        search_bar.update_match_index(cx);
                        search_bar.select_match(direction.opposite(), 1, cx);
                    });
                }
            });
        }
        self.update_editor(cx, |_, editor, cx| {
            let latest = editor.selections.newest::<usize>(cx);
            start_selection = latest.start;
            end_selection = latest.end;
        });

        let mut match_exists = false;
        pane.update(cx, |pane, cx| {
            if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.update_match_index(cx);
                    search_bar.select_match(direction, count, cx);
                    match_exists = search_bar.match_exists(cx);
                });
            }
        });
        if !match_exists {
            self.clear_operator(cx);
            self.stop_replaying(cx);
            return;
        }
        self.update_editor(cx, |_, editor, cx| {
            let latest = editor.selections.newest::<usize>(cx);
            if vim_is_normal {
                start_selection = latest.start;
                end_selection = latest.end;
            } else {
                start_selection = start_selection.min(latest.start);
                end_selection = end_selection.max(latest.end);
            }
            if direction == Direction::Prev {
                std::mem::swap(&mut start_selection, &mut end_selection);
            }
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select_ranges([start_selection..end_selection]);
            });
            editor.set_collapse_matches(true);
        });

        match self.maybe_pop_operator() {
            Some(Operator::Change) => self.substitute(None, false, cx),
            Some(Operator::Delete) => {
                self.stop_recording(cx);
                self.visual_delete(false, cx)
            }
            Some(Operator::Yank) => self.visual_yank(cx),
            _ => {} // Ignoring other operators
        }
    }
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
        cx.simulate_shared_keystrokes("v").await;
        cx.shared_state()
            .await
            .assert_eq(indoc! { "The «qˇ»uick brown
            fox jumps over
            the lazy dog"});
        cx.update_editor(|editor, cx| assert_eq!(cursor, editor.pixel_position_of_cursor(cx)));

        // forwards motions should extend the selection
        cx.simulate_shared_keystrokes("w j").await;
        cx.shared_state().await.assert_eq(indoc! { "The «quick brown
            fox jumps oˇ»ver
            the lazy dog"});

        cx.simulate_shared_keystrokes("escape").await;
        cx.shared_state().await.assert_eq(indoc! { "The quick brown
            fox jumps ˇover
            the lazy dog"});

        // motions work backwards
        cx.simulate_shared_keystrokes("v k b").await;
        cx.shared_state()
            .await
            .assert_eq(indoc! { "The «ˇquick brown
            fox jumps o»ver
            the lazy dog"});

        // works on empty lines
        cx.set_shared_state(indoc! {"
            a
            ˇ
            b
            "})
            .await;
        let cursor = cx.update_editor(|editor, cx| editor.pixel_position_of_cursor(cx));
        cx.simulate_shared_keystrokes("v").await;
        cx.shared_state().await.assert_eq(indoc! {"
            a
            «
            ˇ»b
        "});
        cx.update_editor(|editor, cx| assert_eq!(cursor, editor.pixel_position_of_cursor(cx)));

        // toggles off again
        cx.simulate_shared_keystrokes("v").await;
        cx.shared_state().await.assert_eq(indoc! {"
            a
            ˇ
            b
            "});

        // works at the end of a document
        cx.set_shared_state(indoc! {"
            a
            b
            ˇ"})
            .await;

        cx.simulate_shared_keystrokes("v").await;
        cx.shared_state().await.assert_eq(indoc! {"
            a
            b
            ˇ"});
    }

    #[gpui::test]
    async fn test_visual_insert_first_non_whitespace(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state(
            indoc! {
                "«The quick brown
                fox jumps over
                the lazy dogˇ»"
            },
            Mode::Visual,
        );
        cx.simulate_keystrokes("g shift-i");
        cx.assert_state(
            indoc! {
                "ˇThe quick brown
                ˇfox jumps over
                ˇthe lazy dog"
            },
            Mode::Insert,
        );
    }

    #[gpui::test]
    async fn test_visual_insert_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state(
            indoc! {
                "«The quick brown
                fox jumps over
                the lazy dogˇ»"
            },
            Mode::Visual,
        );
        cx.simulate_keystrokes("g shift-a");
        cx.assert_state(
            indoc! {
                "The quick brownˇ
                fox jumps overˇ
                the lazy dogˇ"
            },
            Mode::Insert,
        );
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
        cx.simulate_shared_keystrokes("shift-v").await;
        cx.shared_state()
            .await
            .assert_eq(indoc! { "The «qˇ»uick brown
            fox jumps over
            the lazy dog"});
        cx.simulate_shared_keystrokes("x").await;
        cx.shared_state().await.assert_eq(indoc! { "fox ˇjumps over
        the lazy dog"});

        // it should work on empty lines
        cx.set_shared_state(indoc! {"
            a
            ˇ
            b"})
            .await;
        cx.simulate_shared_keystrokes("shift-v").await;
        cx.shared_state().await.assert_eq(indoc! {"
            a
            «
            ˇ»b"});
        cx.simulate_shared_keystrokes("x").await;
        cx.shared_state().await.assert_eq(indoc! {"
            a
            ˇb"});

        // it should work at the end of the document
        cx.set_shared_state(indoc! {"
            a
            b
            ˇ"})
            .await;
        let cursor = cx.update_editor(|editor, cx| editor.pixel_position_of_cursor(cx));
        cx.simulate_shared_keystrokes("shift-v").await;
        cx.shared_state().await.assert_eq(indoc! {"
            a
            b
            ˇ"});
        cx.update_editor(|editor, cx| assert_eq!(cursor, editor.pixel_position_of_cursor(cx)));
        cx.simulate_shared_keystrokes("x").await;
        cx.shared_state().await.assert_eq(indoc! {"
            a
            ˇb"});
    }

    #[gpui::test]
    async fn test_visual_delete(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.simulate("v w", "The quick ˇbrown")
            .await
            .assert_matches();

        cx.simulate("v w x", "The quick ˇbrown")
            .await
            .assert_matches();
        cx.simulate(
            "v w j x",
            indoc! {"
                The ˇquick brown
                fox jumps over
                the lazy dog"},
        )
        .await
        .assert_matches();
        // Test pasting code copied on delete
        cx.simulate_shared_keystrokes("j p").await;
        cx.shared_state().await.assert_matches();

        cx.simulate_at_each_offset(
            "v w j x",
            indoc! {"
                The ˇquick brown
                fox jumps over
                the ˇlazy dog"},
        )
        .await
        .assert_matches();
        cx.simulate_at_each_offset(
            "v b k x",
            indoc! {"
                The ˇquick brown
                fox jumps ˇover
                the ˇlazy dog"},
        )
        .await
        .assert_matches();
    }

    #[gpui::test]
    async fn test_visual_line_delete(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
                The quˇick brown
                fox jumps over
                the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("shift-v x").await;
        cx.shared_state().await.assert_matches();

        // Test pasting code copied on delete
        cx.simulate_shared_keystrokes("p").await;
        cx.shared_state().await.assert_matches();

        cx.set_shared_state(indoc! {"
                The quick brown
                fox jumps over
                the laˇzy dog"})
            .await;
        cx.simulate_shared_keystrokes("shift-v x").await;
        cx.shared_state().await.assert_matches();
        cx.shared_clipboard().await.assert_eq("the lazy dog\n");

        cx.set_shared_state(indoc! {"
                                The quˇick brown
                                fox jumps over
                                the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("shift-v j x").await;
        cx.shared_state().await.assert_matches();
        // Test pasting code copied on delete
        cx.simulate_shared_keystrokes("p").await;
        cx.shared_state().await.assert_matches();

        cx.set_shared_state(indoc! {"
            The ˇlong line
            should not
            crash
            "})
            .await;
        cx.simulate_shared_keystrokes("shift-v $ x").await;
        cx.shared_state().await.assert_matches();
    }

    #[gpui::test]
    async fn test_visual_yank(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("The quick ˇbrown").await;
        cx.simulate_shared_keystrokes("v w y").await;
        cx.shared_state().await.assert_eq("The quick ˇbrown");
        cx.shared_clipboard().await.assert_eq("brown");

        cx.set_shared_state(indoc! {"
                The ˇquick brown
                fox jumps over
                the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("v w j y").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    The ˇquick brown
                    fox jumps over
                    the lazy dog"});
        cx.shared_clipboard().await.assert_eq(indoc! {"
                quick brown
                fox jumps o"});

        cx.set_shared_state(indoc! {"
                    The quick brown
                    fox jumps over
                    the ˇlazy dog"})
            .await;
        cx.simulate_shared_keystrokes("v w j y").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    The quick brown
                    fox jumps over
                    the ˇlazy dog"});
        cx.shared_clipboard().await.assert_eq("lazy d");
        cx.simulate_shared_keystrokes("shift-v y").await;
        cx.shared_clipboard().await.assert_eq("the lazy dog\n");

        cx.set_shared_state(indoc! {"
                    The ˇquick brown
                    fox jumps over
                    the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("v b k y").await;
        cx.shared_state().await.assert_eq(indoc! {"
                    ˇThe quick brown
                    fox jumps over
                    the lazy dog"});
        assert_eq!(
            cx.read_from_clipboard()
                .map(|item| item.text().unwrap().to_string())
                .unwrap(),
            "The q"
        );

        cx.set_shared_state(indoc! {"
                    The quick brown
                    fox ˇjumps over
                    the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("shift-v shift-g shift-y")
            .await;
        cx.shared_state().await.assert_eq(indoc! {"
                    The quick brown
                    ˇfox jumps over
                    the lazy dog"});
        cx.shared_clipboard()
            .await
            .assert_eq("fox jumps over\nthe lazy dog\n");
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
        cx.simulate_shared_keystrokes("ctrl-v").await;
        cx.shared_state().await.assert_eq(indoc! {
            "The «qˇ»uick brown
            fox jumps over
            the lazy dog"
        });
        cx.simulate_shared_keystrokes("2 down").await;
        cx.shared_state().await.assert_eq(indoc! {
            "The «qˇ»uick brown
            fox «jˇ»umps over
            the «lˇ»azy dog"
        });
        cx.simulate_shared_keystrokes("e").await;
        cx.shared_state().await.assert_eq(indoc! {
            "The «quicˇ»k brown
            fox «jumpˇ»s over
            the «lazyˇ» dog"
        });
        cx.simulate_shared_keystrokes("^").await;
        cx.shared_state().await.assert_eq(indoc! {
            "«ˇThe q»uick brown
            «ˇfox j»umps over
            «ˇthe l»azy dog"
        });
        cx.simulate_shared_keystrokes("$").await;
        cx.shared_state().await.assert_eq(indoc! {
            "The «quick brownˇ»
            fox «jumps overˇ»
            the «lazy dogˇ»"
        });
        cx.simulate_shared_keystrokes("shift-f space").await;
        cx.shared_state().await.assert_eq(indoc! {
            "The «quickˇ» brown
            fox «jumpsˇ» over
            the «lazy ˇ»dog"
        });

        // toggling through visual mode works as expected
        cx.simulate_shared_keystrokes("v").await;
        cx.shared_state().await.assert_eq(indoc! {
            "The «quick brown
            fox jumps over
            the lazy ˇ»dog"
        });
        cx.simulate_shared_keystrokes("ctrl-v").await;
        cx.shared_state().await.assert_eq(indoc! {
            "The «quickˇ» brown
            fox «jumpsˇ» over
            the «lazy ˇ»dog"
        });

        cx.set_shared_state(indoc! {
            "The ˇquick
             brown
             fox
             jumps over the

             lazy dog
            "
        })
        .await;
        cx.simulate_shared_keystrokes("ctrl-v down down").await;
        cx.shared_state().await.assert_eq(indoc! {
            "The«ˇ q»uick
            bro«ˇwn»
            foxˇ
            jumps over the

            lazy dog
            "
        });
        cx.simulate_shared_keystrokes("down").await;
        cx.shared_state().await.assert_eq(indoc! {
            "The «qˇ»uick
            brow«nˇ»
            fox
            jump«sˇ» over the

            lazy dog
            "
        });
        cx.simulate_shared_keystrokes("left").await;
        cx.shared_state().await.assert_eq(indoc! {
            "The«ˇ q»uick
            bro«ˇwn»
            foxˇ
            jum«ˇps» over the

            lazy dog
            "
        });
        cx.simulate_shared_keystrokes("s o escape").await;
        cx.shared_state().await.assert_eq(indoc! {
            "Theˇouick
            broo
            foxo
            jumo over the

            lazy dog
            "
        });

        // https://github.com/zed-industries/zed/issues/6274
        cx.set_shared_state(indoc! {
            "Theˇ quick brown

            fox jumps over
            the lazy dog
            "
        })
        .await;
        cx.simulate_shared_keystrokes("l ctrl-v j j").await;
        cx.shared_state().await.assert_eq(indoc! {
            "The «qˇ»uick brown

            fox «jˇ»umps over
            the lazy dog
            "
        });
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
        cx.simulate_shared_keystrokes("ctrl-v right down").await;
        cx.shared_state().await.assert_eq(indoc! {
            "The «quˇ»ick brown
            fox «juˇ»mps over
            the lazy dog
            "
        });
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
        cx.simulate_shared_keystrokes("ctrl-v 9 down").await;
        cx.shared_state().await.assert_eq(indoc! {
            "«Tˇ»he quick brown
            «fˇ»ox jumps over
            «tˇ»he lazy dog
            ˇ"
        });

        cx.simulate_shared_keystrokes("shift-i k escape").await;
        cx.shared_state().await.assert_eq(indoc! {
            "ˇkThe quick brown
            kfox jumps over
            kthe lazy dog
            k"
        });

        cx.set_shared_state(indoc! {
            "ˇThe quick brown
            fox jumps over
            the lazy dog
            "
        })
        .await;
        cx.simulate_shared_keystrokes("ctrl-v 9 down").await;
        cx.shared_state().await.assert_eq(indoc! {
            "«Tˇ»he quick brown
            «fˇ»ox jumps over
            «tˇ»he lazy dog
            ˇ"
        });
        cx.simulate_shared_keystrokes("c k escape").await;
        cx.shared_state().await.assert_eq(indoc! {
            "ˇkhe quick brown
            kox jumps over
            khe lazy dog
            k"
        });
    }

    #[gpui::test]
    async fn test_visual_object(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("hello (in [parˇens] o)").await;
        cx.simulate_shared_keystrokes("ctrl-v l").await;
        cx.simulate_shared_keystrokes("a ]").await;
        cx.shared_state()
            .await
            .assert_eq("hello (in «[parens]ˇ» o)");
        cx.simulate_shared_keystrokes("i (").await;
        cx.shared_state()
            .await
            .assert_eq("hello («in [parens] oˇ»)");

        cx.set_shared_state("hello in a wˇord again.").await;
        cx.simulate_shared_keystrokes("ctrl-v l i w").await;
        cx.shared_state()
            .await
            .assert_eq("hello in a w«ordˇ» again.");
        assert_eq!(cx.mode(), Mode::VisualBlock);
        cx.simulate_shared_keystrokes("o a s").await;
        cx.shared_state()
            .await
            .assert_eq("«ˇhello in a word» again.");
    }

    #[gpui::test]
    async fn test_mode_across_command(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("aˇbc", Mode::Normal);
        cx.simulate_keystrokes("ctrl-v");
        assert_eq!(cx.mode(), Mode::VisualBlock);
        cx.simulate_keystrokes("cmd-shift-p escape");
        assert_eq!(cx.mode(), Mode::VisualBlock);
    }

    #[gpui::test]
    async fn test_gn(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("aaˇ aa aa aa aa").await;
        cx.simulate_shared_keystrokes("/ a a enter").await;
        cx.shared_state().await.assert_eq("aa ˇaa aa aa aa");
        cx.simulate_shared_keystrokes("g n").await;
        cx.shared_state().await.assert_eq("aa «aaˇ» aa aa aa");
        cx.simulate_shared_keystrokes("g n").await;
        cx.shared_state().await.assert_eq("aa «aa aaˇ» aa aa");
        cx.simulate_shared_keystrokes("escape d g n").await;
        cx.shared_state().await.assert_eq("aa aa ˇ aa aa");

        cx.set_shared_state("aaˇ aa aa aa aa").await;
        cx.simulate_shared_keystrokes("/ a a enter").await;
        cx.shared_state().await.assert_eq("aa ˇaa aa aa aa");
        cx.simulate_shared_keystrokes("3 g n").await;
        cx.shared_state().await.assert_eq("aa aa aa «aaˇ» aa");

        cx.set_shared_state("aaˇ aa aa aa aa").await;
        cx.simulate_shared_keystrokes("/ a a enter").await;
        cx.shared_state().await.assert_eq("aa ˇaa aa aa aa");
        cx.simulate_shared_keystrokes("g shift-n").await;
        cx.shared_state().await.assert_eq("aa «ˇaa» aa aa aa");
        cx.simulate_shared_keystrokes("g shift-n").await;
        cx.shared_state().await.assert_eq("«ˇaa aa» aa aa aa");
    }

    #[gpui::test]
    async fn test_gl(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("aaˇ aa\naa", Mode::Normal);
        cx.simulate_keystrokes("g l");
        cx.assert_state("«aaˇ» «aaˇ»\naa", Mode::Visual);
        cx.simulate_keystrokes("g >");
        cx.assert_state("«aaˇ» aa\n«aaˇ»", Mode::Visual);
    }

    #[gpui::test]
    async fn test_dgn_repeat(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("aaˇ aa aa aa aa").await;
        cx.simulate_shared_keystrokes("/ a a enter").await;
        cx.shared_state().await.assert_eq("aa ˇaa aa aa aa");
        cx.simulate_shared_keystrokes("d g n").await;

        cx.shared_state().await.assert_eq("aa ˇ aa aa aa");
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq("aa  ˇ aa aa");
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq("aa   ˇ aa");
    }

    #[gpui::test]
    async fn test_cgn_repeat(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("aaˇ aa aa aa aa").await;
        cx.simulate_shared_keystrokes("/ a a enter").await;
        cx.shared_state().await.assert_eq("aa ˇaa aa aa aa");
        cx.simulate_shared_keystrokes("c g n x escape").await;
        cx.shared_state().await.assert_eq("aa ˇx aa aa aa");
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq("aa x ˇx aa aa");
    }

    #[gpui::test]
    async fn test_cgn_nomatch(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("aaˇ aa aa aa aa").await;
        cx.simulate_shared_keystrokes("/ b b enter").await;
        cx.shared_state().await.assert_eq("aaˇ aa aa aa aa");
        cx.simulate_shared_keystrokes("c g n x escape").await;
        cx.shared_state().await.assert_eq("aaˇaa aa aa aa");
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq("aaˇa aa aa aa");

        cx.set_shared_state("aaˇ bb aa aa aa").await;
        cx.simulate_shared_keystrokes("/ b b enter").await;
        cx.shared_state().await.assert_eq("aa ˇbb aa aa aa");
        cx.simulate_shared_keystrokes("c g n x escape").await;
        cx.shared_state().await.assert_eq("aa ˇx aa aa aa");
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq("aa ˇx aa aa aa");
    }

    #[gpui::test]
    async fn test_visual_shift_d(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {
            "The ˇquick brown
            fox jumps over
            the lazy dog
            "
        })
        .await;
        cx.simulate_shared_keystrokes("v down shift-d").await;
        cx.shared_state().await.assert_eq(indoc! {
            "the ˇlazy dog\n"
        });

        cx.set_shared_state(indoc! {
            "The ˇquick brown
            fox jumps over
            the lazy dog
            "
        })
        .await;
        cx.simulate_shared_keystrokes("ctrl-v down shift-d").await;
        cx.shared_state().await.assert_eq(indoc! {
            "Theˇ•
            fox•
            the lazy dog
            "
        });
    }

    #[gpui::test]
    async fn test_gv(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {
            "The ˇquick brown"
        })
        .await;
        cx.simulate_shared_keystrokes("v i w escape g v").await;
        cx.shared_state().await.assert_eq(indoc! {
            "The «quickˇ» brown"
        });

        cx.simulate_shared_keystrokes("o escape g v").await;
        cx.shared_state().await.assert_eq(indoc! {
            "The «ˇquick» brown"
        });

        cx.simulate_shared_keystrokes("escape ^ ctrl-v l").await;
        cx.shared_state().await.assert_eq(indoc! {
            "«Thˇ»e quick brown"
        });
        cx.simulate_shared_keystrokes("g v").await;
        cx.shared_state().await.assert_eq(indoc! {
            "The «ˇquick» brown"
        });
        cx.simulate_shared_keystrokes("g v").await;
        cx.shared_state().await.assert_eq(indoc! {
            "«Thˇ»e quick brown"
        });

        cx.set_state(
            indoc! {"
            fiˇsh one
            fish two
            fish red
            fish blue
        "},
            Mode::Normal,
        );
        cx.simulate_keystrokes("4 g l escape escape g v");
        cx.assert_state(
            indoc! {"
                «fishˇ» one
                «fishˇ» two
                «fishˇ» red
                «fishˇ» blue
            "},
            Mode::Visual,
        );
        cx.simulate_keystrokes("y g v");
        cx.assert_state(
            indoc! {"
                «fishˇ» one
                «fishˇ» two
                «fishˇ» red
                «fishˇ» blue
            "},
            Mode::Visual,
        );
    }
}
