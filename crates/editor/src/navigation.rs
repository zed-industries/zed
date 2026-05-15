use super::*;

impl Editor {
    pub fn move_left(&mut self, _: &MoveLeft, window: &mut Window, cx: &mut Context<Self>) {
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_with(&mut |map, selection| {
                let cursor = if selection.is_empty() {
                    movement::left(map, selection.start)
                } else {
                    selection.start
                };
                selection.collapse_to(cursor, SelectionGoal::None);
            });
        })
    }

    pub fn select_left(&mut self, _: &SelectLeft, window: &mut Window, cx: &mut Context<Self>) {
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, _| (movement::left(map, head), SelectionGoal::None));
        })
    }

    pub fn move_right(&mut self, _: &MoveRight, window: &mut Window, cx: &mut Context<Self>) {
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_with(&mut |map, selection| {
                let cursor = if selection.is_empty() {
                    movement::right(map, selection.end)
                } else {
                    selection.end
                };
                selection.collapse_to(cursor, SelectionGoal::None)
            });
        })
    }

    pub fn select_right(&mut self, _: &SelectRight, window: &mut Window, cx: &mut Context<Self>) {
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, _| {
                (movement::right(map, head), SelectionGoal::None)
            });
        });
    }

    pub fn move_up(&mut self, _: &MoveUp, window: &mut Window, cx: &mut Context<Self>) {
        if self.take_rename(true, window, cx).is_some() {
            return;
        }

        if self.mode.is_single_line() {
            cx.propagate();
            return;
        }

        let text_layout_details = &self.text_layout_details(window, cx);
        let selection_count = self.selections.count();
        let first_selection = self.selections.first_anchor();

        self.change_selections(Default::default(), window, cx, |s| {
            s.move_with(&mut |map, selection| {
                if !selection.is_empty() {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::up(
                    map,
                    selection.start,
                    selection.goal,
                    false,
                    text_layout_details,
                );
                selection.collapse_to(cursor, goal);
            });
        });

        if selection_count == 1 && first_selection.range() == self.selections.first_anchor().range()
        {
            cx.propagate();
        }
    }

    pub fn move_up_by_lines(
        &mut self,
        action: &MoveUpByLines,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.take_rename(true, window, cx).is_some() {
            return;
        }

        if self.mode.is_single_line() {
            cx.propagate();
            return;
        }

        let text_layout_details = &self.text_layout_details(window, cx);

        self.change_selections(Default::default(), window, cx, |s| {
            s.move_with(&mut |map, selection| {
                if !selection.is_empty() {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::up_by_rows(
                    map,
                    selection.start,
                    action.lines,
                    selection.goal,
                    false,
                    text_layout_details,
                );
                selection.collapse_to(cursor, goal);
            });
        })
    }

    pub fn move_down_by_lines(
        &mut self,
        action: &MoveDownByLines,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.take_rename(true, window, cx).is_some() {
            return;
        }

        if self.mode.is_single_line() {
            cx.propagate();
            return;
        }

        let text_layout_details = &self.text_layout_details(window, cx);

        self.change_selections(Default::default(), window, cx, |s| {
            s.move_with(&mut |map, selection| {
                if !selection.is_empty() {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::down_by_rows(
                    map,
                    selection.start,
                    action.lines,
                    selection.goal,
                    false,
                    text_layout_details,
                );
                selection.collapse_to(cursor, goal);
            });
        })
    }

    pub fn select_down_by_lines(
        &mut self,
        action: &SelectDownByLines,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let text_layout_details = &self.text_layout_details(window, cx);
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, goal| {
                movement::down_by_rows(map, head, action.lines, goal, false, text_layout_details)
            })
        })
    }

    pub fn select_up_by_lines(
        &mut self,
        action: &SelectUpByLines,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let text_layout_details = &self.text_layout_details(window, cx);
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, goal| {
                movement::up_by_rows(map, head, action.lines, goal, false, text_layout_details)
            })
        })
    }

    pub fn select_page_up(
        &mut self,
        _: &SelectPageUp,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(row_count) = self.visible_row_count() else {
            return;
        };

        let text_layout_details = &self.text_layout_details(window, cx);

        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, goal| {
                movement::up_by_rows(map, head, row_count, goal, false, text_layout_details)
            })
        })
    }

    pub fn move_page_up(
        &mut self,
        action: &MovePageUp,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.take_rename(true, window, cx).is_some() {
            return;
        }

        if self
            .context_menu
            .borrow_mut()
            .as_mut()
            .map(|menu| menu.select_first(self.completion_provider.as_deref(), window, cx))
            .unwrap_or(false)
        {
            return;
        }

        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }

        let Some(row_count) = self.visible_row_count() else {
            return;
        };

        let effects = if action.center_cursor {
            SelectionEffects::scroll(Autoscroll::center())
        } else {
            SelectionEffects::default()
        };

        let text_layout_details = &self.text_layout_details(window, cx);

        self.change_selections(effects, window, cx, |s| {
            s.move_with(&mut |map, selection| {
                if !selection.is_empty() {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::up_by_rows(
                    map,
                    selection.end,
                    row_count,
                    selection.goal,
                    false,
                    text_layout_details,
                );
                selection.collapse_to(cursor, goal);
            });
        });
    }

    pub fn select_up(&mut self, _: &SelectUp, window: &mut Window, cx: &mut Context<Self>) {
        let text_layout_details = &self.text_layout_details(window, cx);
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, goal| {
                movement::up(map, head, goal, false, text_layout_details)
            })
        })
    }

    pub fn move_down(&mut self, _: &MoveDown, window: &mut Window, cx: &mut Context<Self>) {
        if self.take_rename(true, window, cx).is_some() {
            return;
        }

        if self.mode.is_single_line() {
            cx.propagate();
            return;
        }

        let text_layout_details = &self.text_layout_details(window, cx);
        let selection_count = self.selections.count();
        let first_selection = self.selections.first_anchor();

        self.change_selections(Default::default(), window, cx, |s| {
            s.move_with(&mut |map, selection| {
                if !selection.is_empty() {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::down(
                    map,
                    selection.end,
                    selection.goal,
                    false,
                    text_layout_details,
                );
                selection.collapse_to(cursor, goal);
            });
        });

        if selection_count == 1 && first_selection.range() == self.selections.first_anchor().range()
        {
            cx.propagate();
        }
    }

    pub fn select_page_down(
        &mut self,
        _: &SelectPageDown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(row_count) = self.visible_row_count() else {
            return;
        };

        let text_layout_details = &self.text_layout_details(window, cx);

        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, goal| {
                movement::down_by_rows(map, head, row_count, goal, false, text_layout_details)
            })
        })
    }

    pub fn move_page_down(
        &mut self,
        action: &MovePageDown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.take_rename(true, window, cx).is_some() {
            return;
        }

        if self
            .context_menu
            .borrow_mut()
            .as_mut()
            .map(|menu| menu.select_last(self.completion_provider.as_deref(), window, cx))
            .unwrap_or(false)
        {
            return;
        }

        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }

        let Some(row_count) = self.visible_row_count() else {
            return;
        };

        let effects = if action.center_cursor {
            SelectionEffects::scroll(Autoscroll::center())
        } else {
            SelectionEffects::default()
        };

        let text_layout_details = &self.text_layout_details(window, cx);
        self.change_selections(effects, window, cx, |s| {
            s.move_with(&mut |map, selection| {
                if !selection.is_empty() {
                    selection.goal = SelectionGoal::None;
                }
                let (cursor, goal) = movement::down_by_rows(
                    map,
                    selection.end,
                    row_count,
                    selection.goal,
                    false,
                    text_layout_details,
                );
                selection.collapse_to(cursor, goal);
            });
        });
    }

    pub fn select_down(&mut self, _: &SelectDown, window: &mut Window, cx: &mut Context<Self>) {
        let text_layout_details = &self.text_layout_details(window, cx);
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, goal| {
                movement::down(map, head, goal, false, text_layout_details)
            })
        });
    }

    pub fn move_to_previous_word_start(
        &mut self,
        _: &MoveToPreviousWordStart,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_cursors_with(&mut |map, head, _| {
                (
                    movement::previous_word_start(map, head),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn move_to_previous_subword_start(
        &mut self,
        _: &MoveToPreviousSubwordStart,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_cursors_with(&mut |map, head, _| {
                (
                    movement::previous_subword_start(map, head),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_previous_word_start(
        &mut self,
        _: &SelectToPreviousWordStart,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, _| {
                (
                    movement::previous_word_start(map, head),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_previous_subword_start(
        &mut self,
        _: &SelectToPreviousSubwordStart,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, _| {
                (
                    movement::previous_subword_start(map, head),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn move_to_next_word_end(
        &mut self,
        _: &MoveToNextWordEnd,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_cursors_with(&mut |map, head, _| {
                (movement::next_word_end(map, head), SelectionGoal::None)
            });
        })
    }

    pub fn move_to_next_subword_end(
        &mut self,
        _: &MoveToNextSubwordEnd,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_cursors_with(&mut |map, head, _| {
                (movement::next_subword_end(map, head), SelectionGoal::None)
            });
        })
    }

    pub fn select_to_next_word_end(
        &mut self,
        _: &SelectToNextWordEnd,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, _| {
                (movement::next_word_end(map, head), SelectionGoal::None)
            });
        })
    }

    pub fn select_to_next_subword_end(
        &mut self,
        _: &SelectToNextSubwordEnd,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, _| {
                (movement::next_subword_end(map, head), SelectionGoal::None)
            });
        })
    }

    pub fn move_to_beginning_of_line(
        &mut self,
        action: &MoveToBeginningOfLine,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let stop_at_indent = action.stop_at_indent && !self.mode.is_single_line();
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_cursors_with(&mut |map, head, _| {
                (
                    movement::indented_line_beginning(
                        map,
                        head,
                        action.stop_at_soft_wraps,
                        stop_at_indent,
                    ),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_beginning_of_line(
        &mut self,
        action: &SelectToBeginningOfLine,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let stop_at_indent = action.stop_at_indent && !self.mode.is_single_line();
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, _| {
                (
                    movement::indented_line_beginning(
                        map,
                        head,
                        action.stop_at_soft_wraps,
                        stop_at_indent,
                    ),
                    SelectionGoal::None,
                )
            });
        });
    }

    pub fn move_to_end_of_line(
        &mut self,
        action: &MoveToEndOfLine,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_cursors_with(&mut |map, head, _| {
                (
                    movement::line_end(map, head, action.stop_at_soft_wraps),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_end_of_line(
        &mut self,
        action: &SelectToEndOfLine,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, _| {
                (
                    movement::line_end(map, head, action.stop_at_soft_wraps),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn move_to_start_of_paragraph(
        &mut self,
        _: &MoveToStartOfParagraph,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_with(&mut |map, selection| {
                selection.collapse_to(
                    movement::start_of_paragraph(map, selection.head(), 1),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn move_to_end_of_paragraph(
        &mut self,
        _: &MoveToEndOfParagraph,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_with(&mut |map, selection| {
                selection.collapse_to(
                    movement::end_of_paragraph(map, selection.head(), 1),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_start_of_paragraph(
        &mut self,
        _: &SelectToStartOfParagraph,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, _| {
                (
                    movement::start_of_paragraph(map, head, 1),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_end_of_paragraph(
        &mut self,
        _: &SelectToEndOfParagraph,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, _| {
                (
                    movement::end_of_paragraph(map, head, 1),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn move_to_start_of_excerpt(
        &mut self,
        _: &MoveToStartOfExcerpt,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_with(&mut |map, selection| {
                selection.collapse_to(
                    movement::start_of_excerpt(
                        map,
                        selection.head(),
                        workspace::searchable::Direction::Prev,
                    ),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn move_to_start_of_next_excerpt(
        &mut self,
        _: &MoveToStartOfNextExcerpt,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }

        self.change_selections(Default::default(), window, cx, |s| {
            s.move_with(&mut |map, selection| {
                selection.collapse_to(
                    movement::start_of_excerpt(
                        map,
                        selection.head(),
                        workspace::searchable::Direction::Next,
                    ),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn move_to_end_of_excerpt(
        &mut self,
        _: &MoveToEndOfExcerpt,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_with(&mut |map, selection| {
                selection.collapse_to(
                    movement::end_of_excerpt(
                        map,
                        selection.head(),
                        workspace::searchable::Direction::Next,
                    ),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn move_to_end_of_previous_excerpt(
        &mut self,
        _: &MoveToEndOfPreviousExcerpt,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_with(&mut |map, selection| {
                selection.collapse_to(
                    movement::end_of_excerpt(
                        map,
                        selection.head(),
                        workspace::searchable::Direction::Prev,
                    ),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_start_of_excerpt(
        &mut self,
        _: &SelectToStartOfExcerpt,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, _| {
                (
                    movement::start_of_excerpt(map, head, workspace::searchable::Direction::Prev),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_start_of_next_excerpt(
        &mut self,
        _: &SelectToStartOfNextExcerpt,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, _| {
                (
                    movement::start_of_excerpt(map, head, workspace::searchable::Direction::Next),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_end_of_excerpt(
        &mut self,
        _: &SelectToEndOfExcerpt,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, _| {
                (
                    movement::end_of_excerpt(map, head, workspace::searchable::Direction::Next),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn select_to_end_of_previous_excerpt(
        &mut self,
        _: &SelectToEndOfPreviousExcerpt,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_heads_with(&mut |map, head, _| {
                (
                    movement::end_of_excerpt(map, head, workspace::searchable::Direction::Prev),
                    SelectionGoal::None,
                )
            });
        })
    }

    pub fn move_to_beginning(
        &mut self,
        _: &MoveToBeginning,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }
        self.change_selections(Default::default(), window, cx, |s| {
            s.select_ranges(vec![Anchor::Min..Anchor::Min]);
        });
    }

    pub fn select_to_beginning(
        &mut self,
        _: &SelectToBeginning,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut selection = self.selections.last::<Point>(&self.display_snapshot(cx));
        selection.set_head(Point::zero(), SelectionGoal::None);
        self.change_selections(Default::default(), window, cx, |s| {
            s.select(vec![selection]);
        });
    }

    pub fn move_to_end(&mut self, _: &MoveToEnd, window: &mut Window, cx: &mut Context<Self>) {
        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }
        let cursor = self.buffer.read(cx).read(cx).len();
        self.change_selections(Default::default(), window, cx, |s| {
            s.select_ranges(vec![cursor..cursor])
        });
    }

    pub fn set_nav_history(&mut self, nav_history: Option<ItemNavHistory>) {
        self.nav_history = nav_history;
    }

    pub fn save_location(
        &mut self,
        _: &SaveLocation,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.create_nav_history_entry(cx);
    }

    pub fn create_nav_history_entry(&mut self, cx: &mut Context<Self>) {
        self.push_to_nav_history(
            self.selections.newest_anchor().head(),
            None,
            false,
            true,
            cx,
        );
    }

    pub fn expand_excerpts(
        &mut self,
        action: &ExpandExcerpts,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.expand_excerpts_for_direction(action.lines, ExpandExcerptDirection::UpAndDown, cx)
    }

    pub fn expand_excerpts_down(
        &mut self,
        action: &ExpandExcerptsDown,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.expand_excerpts_for_direction(action.lines, ExpandExcerptDirection::Down, cx)
    }

    pub fn expand_excerpts_up(
        &mut self,
        action: &ExpandExcerptsUp,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.expand_excerpts_for_direction(action.lines, ExpandExcerptDirection::Up, cx)
    }

    pub fn go_to_singleton_buffer_point(
        &mut self,
        point: Point,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_singleton_buffer_range(point..point, window, cx);
    }

    pub fn go_to_singleton_buffer_range(
        &mut self,
        range: Range<Point>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_singleton_buffer_range_impl(range, true, window, cx);
    }

    /// Like `go_to_singleton_buffer_point`, but does not push a navigation
    /// history entry. Useful when the caller already recorded one (e.g. when
    /// a file was just opened and we only need to move the cursor).
    pub fn go_to_singleton_buffer_point_silently(
        &mut self,
        point: Point,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_singleton_buffer_range_impl(point..point, false, window, cx);
    }

    pub fn go_to_next_document_highlight(
        &mut self,
        _: &GoToNextDocumentHighlight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_document_highlight_before_or_after_position(Direction::Next, window, cx);
    }

    pub fn go_to_prev_document_highlight(
        &mut self,
        _: &GoToPreviousDocumentHighlight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_document_highlight_before_or_after_position(Direction::Prev, window, cx);
    }

    pub fn go_to_definition(
        &mut self,
        _: &GoToDefinition,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Navigated>> {
        let definition =
            self.go_to_definition_of_kind(GotoDefinitionKind::Symbol, false, window, cx);
        let fallback_strategy = EditorSettings::get_global(cx).go_to_definition_fallback;
        cx.spawn_in(window, async move |editor, cx| {
            if definition.await? == Navigated::Yes {
                return Ok(Navigated::Yes);
            }
            match fallback_strategy {
                GoToDefinitionFallback::None => Ok(Navigated::No),
                GoToDefinitionFallback::FindAllReferences => {
                    match editor.update_in(cx, |editor, window, cx| {
                        editor.find_all_references(&FindAllReferences::default(), window, cx)
                    })? {
                        Some(references) => references.await,
                        None => Ok(Navigated::No),
                    }
                }
            }
        })
    }

    pub fn go_to_declaration(
        &mut self,
        _: &GoToDeclaration,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Navigated>> {
        self.go_to_definition_of_kind(GotoDefinitionKind::Declaration, false, window, cx)
    }

    pub fn go_to_declaration_split(
        &mut self,
        _: &GoToDeclaration,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Navigated>> {
        self.go_to_definition_of_kind(GotoDefinitionKind::Declaration, true, window, cx)
    }

    pub fn go_to_implementation(
        &mut self,
        _: &GoToImplementation,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Navigated>> {
        self.go_to_definition_of_kind(GotoDefinitionKind::Implementation, false, window, cx)
    }

    pub fn go_to_implementation_split(
        &mut self,
        _: &GoToImplementationSplit,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Navigated>> {
        self.go_to_definition_of_kind(GotoDefinitionKind::Implementation, true, window, cx)
    }

    pub fn go_to_type_definition(
        &mut self,
        _: &GoToTypeDefinition,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Navigated>> {
        self.go_to_definition_of_kind(GotoDefinitionKind::Type, false, window, cx)
    }

    pub fn go_to_definition_split(
        &mut self,
        _: &GoToDefinitionSplit,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Navigated>> {
        self.go_to_definition_of_kind(GotoDefinitionKind::Symbol, true, window, cx)
    }

    pub fn go_to_type_definition_split(
        &mut self,
        _: &GoToTypeDefinitionSplit,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Navigated>> {
        self.go_to_definition_of_kind(GotoDefinitionKind::Type, true, window, cx)
    }

    pub fn open_url(&mut self, _: &OpenUrl, window: &mut Window, cx: &mut Context<Self>) {
        let selection = self.selections.newest_anchor();
        let head = selection.head();
        let tail = selection.tail();

        let Some((buffer, start_position)) =
            self.buffer.read(cx).text_anchor_for_position(head, cx)
        else {
            return;
        };

        let end_position = if head != tail {
            let Some((_, pos)) = self.buffer.read(cx).text_anchor_for_position(tail, cx) else {
                return;
            };
            Some(pos)
        } else {
            None
        };

        let url_finder = cx.spawn_in(window, async move |_editor, cx| {
            let url = if let Some(end_pos) = end_position {
                find_url_from_range(&buffer, start_position..end_pos, cx.clone())
            } else {
                find_url(&buffer, start_position, cx.clone()).map(|(_, url)| url)
            };

            if let Some(url) = url {
                cx.update(|window, cx| {
                    if parse_zed_link(&url, cx).is_some() {
                        window.dispatch_action(Box::new(zed_actions::OpenZedUrl { url }), cx);
                    } else {
                        cx.open_url(&url);
                    }
                })?;
            }

            anyhow::Ok(())
        });

        url_finder.detach();
    }

    pub fn open_selected_filename(
        &mut self,
        _: &OpenSelectedFilename,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace() else {
            return;
        };

        let position = self.selections.newest_anchor().head();

        let Some((buffer, buffer_position)) =
            self.buffer.read(cx).text_anchor_for_position(position, cx)
        else {
            return;
        };

        let project = self.project.clone();

        cx.spawn_in(window, async move |_, cx| {
            let result = find_file(&buffer, project, buffer_position, cx).await;

            if let Some((_, file_target)) = result {
                let item = workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace.open_resolved_path(file_target.resolved_path.clone(), window, cx)
                    })?
                    .await?;

                file_target.navigate_item_to_position(item, cx);
            }
            anyhow::Ok(())
        })
        .detach();
    }

    pub fn go_to_reference_before_or_after_position(
        &mut self,
        direction: Direction,
        count: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let selection = self.selections.newest_anchor();
        let head = selection.head();

        let multi_buffer = self.buffer.read(cx);

        let (buffer, text_head) = multi_buffer.text_anchor_for_position(head, cx)?;
        let workspace = self.workspace()?;
        let project = workspace.read(cx).project().clone();
        let references =
            project.update(cx, |project, cx| project.references(&buffer, text_head, cx));
        Some(cx.spawn_in(window, async move |editor, cx| -> Result<()> {
            let Some(locations) = references.await? else {
                return Ok(());
            };

            if locations.is_empty() {
                // totally normal - the cursor may be on something which is not
                // a symbol (e.g. a keyword)
                log::info!("no references found under cursor");
                return Ok(());
            }

            let multi_buffer = editor.read_with(cx, |editor, _| editor.buffer().clone())?;

            let (locations, current_location_index) =
                multi_buffer.update(cx, |multi_buffer, cx| {
                    let multi_buffer_snapshot = multi_buffer.snapshot(cx);
                    let mut locations = locations
                        .into_iter()
                        .filter_map(|loc| {
                            let start = multi_buffer_snapshot.anchor_in_excerpt(loc.range.start)?;
                            let end = multi_buffer_snapshot.anchor_in_excerpt(loc.range.end)?;
                            Some(start..end)
                        })
                        .collect::<Vec<_>>();
                    // There is an O(n) implementation, but given this list will be
                    // small (usually <100 items), the extra O(log(n)) factor isn't
                    // worth the (surprisingly large amount of) extra complexity.
                    locations
                        .sort_unstable_by(|l, r| l.start.cmp(&r.start, &multi_buffer_snapshot));

                    let head_offset = head.to_offset(&multi_buffer_snapshot);

                    let current_location_index = locations.iter().position(|loc| {
                        loc.start.to_offset(&multi_buffer_snapshot) <= head_offset
                            && loc.end.to_offset(&multi_buffer_snapshot) >= head_offset
                    });

                    (locations, current_location_index)
                });

            let Some(current_location_index) = current_location_index else {
                // This indicates something has gone wrong, because we already
                // handle the "no references" case above
                log::error!(
                    "failed to find current reference under cursor. Total references: {}",
                    locations.len()
                );
                return Ok(());
            };

            let destination_location_index = match direction {
                Direction::Next => (current_location_index + count) % locations.len(),
                Direction::Prev => {
                    (current_location_index + locations.len() - count % locations.len())
                        % locations.len()
                }
            };

            // TODO(cameron): is this needed?
            // the thinking is to avoid "jumping to the current location" (avoid
            // polluting "jumplist" in vim terms)
            if current_location_index == destination_location_index {
                return Ok(());
            }

            let Range { start, end } = locations[destination_location_index];

            editor.update_in(cx, |editor, window, cx| {
                let effects = SelectionEffects::scroll(Autoscroll::for_go_to_definition(
                    editor.cursor_top_offset(cx),
                    cx,
                ));

                editor.unfold_ranges(&[start..end], false, false, cx);
                editor.change_selections(effects, window, cx, |s| {
                    s.select_ranges([start..start]);
                });
            })?;

            Ok(())
        }))
    }

    pub fn find_all_references(
        &mut self,
        action: &FindAllReferences,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<Navigated>>> {
        let always_open_multibuffer = action.always_open_multibuffer;
        let selection = self.selections.newest_anchor();
        let multi_buffer = self.buffer.read(cx);
        let multi_buffer_snapshot = multi_buffer.snapshot(cx);
        let selection_offset = selection.map(|anchor| anchor.to_offset(&multi_buffer_snapshot));
        let selection_point = selection.map(|anchor| anchor.to_point(&multi_buffer_snapshot));
        let head = selection_offset.head();

        let head_anchor = multi_buffer_snapshot.anchor_at(
            head,
            if head < selection_offset.tail() {
                Bias::Right
            } else {
                Bias::Left
            },
        );

        match self
            .find_all_references_task_sources
            .binary_search_by(|anchor| anchor.cmp(&head_anchor, &multi_buffer_snapshot))
        {
            Ok(_) => {
                log::info!(
                    "Ignoring repeated FindAllReferences invocation with the position of already running task"
                );
                return None;
            }
            Err(i) => {
                self.find_all_references_task_sources.insert(i, head_anchor);
            }
        }

        let (buffer, head) = multi_buffer.text_anchor_for_position(head, cx)?;
        let workspace = self.workspace()?;
        let project = workspace.read(cx).project().clone();
        let references = project.update(cx, |project, cx| project.references(&buffer, head, cx));
        Some(cx.spawn_in(window, async move |editor, cx| {
            let _cleanup = cx.on_drop(&editor, move |editor, _| {
                if let Ok(i) = editor
                    .find_all_references_task_sources
                    .binary_search_by(|anchor| anchor.cmp(&head_anchor, &multi_buffer_snapshot))
                {
                    editor.find_all_references_task_sources.remove(i);
                }
            });

            let Some(locations) = references.await? else {
                return anyhow::Ok(Navigated::No);
            };
            let mut locations = cx.update(|_, cx| {
                locations
                    .into_iter()
                    .map(|location| {
                        let buffer = location.buffer.read(cx);
                        (location.buffer, location.range.to_point(buffer))
                    })
                    // if special-casing the single-match case, remove ranges
                    // that intersect current selection
                    .filter(|(location_buffer, location)| {
                        if always_open_multibuffer || &buffer != location_buffer {
                            return true;
                        }

                        !location.contains_inclusive(&selection_point.range())
                    })
                    .into_group_map()
            })?;
            if locations.is_empty() {
                return anyhow::Ok(Navigated::No);
            }
            for ranges in locations.values_mut() {
                ranges.sort_by_key(|range| (range.start, Reverse(range.end)));
                ranges.dedup();
            }
            let mut num_locations = 0;
            for ranges in locations.values_mut() {
                ranges.sort_by_key(|range| (range.start, Reverse(range.end)));
                ranges.dedup();
                num_locations += ranges.len();
            }

            if num_locations == 1 && !always_open_multibuffer {
                let Some((target_buffer, target_ranges)) = locations.into_iter().next() else {
                    return anyhow::Ok(Navigated::No);
                };
                let Some(target_range) = target_ranges.first().cloned() else {
                    return anyhow::Ok(Navigated::No);
                };

                return editor.update_in(cx, |editor, window, cx| {
                    let range = target_range.to_point(target_buffer.read(cx));
                    let range = editor.range_for_match(&range);
                    let range = range.start..range.start;

                    if Some(&target_buffer) == editor.buffer.read(cx).as_singleton().as_ref() {
                        editor.go_to_singleton_buffer_range(range, window, cx);
                    } else {
                        let pane = workspace.read(cx).active_pane().clone();
                        window.defer(cx, move |window, cx| {
                            let target_editor: Entity<Self> =
                                workspace.update(cx, |workspace, cx| {
                                    let pane = workspace.active_pane().clone();

                                    let preview_tabs_settings = PreviewTabsSettings::get_global(cx);
                                    let keep_old_preview = preview_tabs_settings
                                        .enable_keep_preview_on_code_navigation;
                                    let allow_new_preview = preview_tabs_settings
                                        .enable_preview_file_from_code_navigation;

                                    workspace.open_project_item(
                                        pane,
                                        target_buffer.clone(),
                                        true,
                                        true,
                                        keep_old_preview,
                                        allow_new_preview,
                                        window,
                                        cx,
                                    )
                                });
                            target_editor.update(cx, |target_editor, cx| {
                                // When selecting a definition in a different buffer, disable the nav history
                                // to avoid creating a history entry at the previous cursor location.
                                pane.update(cx, |pane, _| pane.disable_history());
                                target_editor.go_to_singleton_buffer_range(range, window, cx);
                                pane.update(cx, |pane, _| pane.enable_history());
                            });
                        });
                    }
                    Navigated::No
                });
            }

            workspace.update_in(cx, |workspace, window, cx| {
                let target = locations
                    .iter()
                    .flat_map(|(k, v)| iter::repeat(k.clone()).zip(v))
                    .map(|(buffer, location)| {
                        buffer
                            .read(cx)
                            .text_for_range(location.clone())
                            .collect::<String>()
                    })
                    .filter(|text| !text.contains('\n'))
                    .unique()
                    .take(3)
                    .join(", ");
                let title = if target.is_empty() {
                    "References".to_owned()
                } else {
                    format!("References to {target}")
                };
                let allow_preview = PreviewTabsSettings::get_global(cx)
                    .enable_preview_multibuffer_from_code_navigation;
                Self::open_locations_in_multibuffer(
                    workspace,
                    locations,
                    title,
                    false,
                    allow_preview,
                    MultibufferSelectionMode::First,
                    window,
                    cx,
                );
                Navigated::Yes
            })
        }))
    }

    pub(super) fn navigation_entry(
        &self,
        cursor_anchor: Anchor,
        cx: &mut Context<Self>,
    ) -> Option<NavigationEntry> {
        let Some(history) = self.nav_history.clone() else {
            return None;
        };
        let data = self.navigation_data(cursor_anchor, cx);
        Some(history.navigation_entry(Some(Arc::new(data) as Arc<dyn Any + Send + Sync>)))
    }

    pub(super) fn push_to_nav_history(
        &mut self,
        cursor_anchor: Anchor,
        new_position: Option<Point>,
        is_deactivate: bool,
        always: bool,
        cx: &mut Context<Self>,
    ) {
        let data = self.navigation_data(cursor_anchor, cx);
        if let Some(nav_history) = self.nav_history.as_mut() {
            if let Some(new_position) = new_position {
                let row_delta = (new_position.row as i64 - data.cursor_position.row as i64).abs();
                if row_delta == 0 || (row_delta < MIN_NAVIGATION_HISTORY_ROW_DELTA && !always) {
                    return;
                }
            }

            let cursor_row = data.cursor_position.row;
            nav_history.push(Some(data), Some(cursor_row), cx);
            cx.emit(EditorEvent::PushedToNavHistory {
                anchor: cursor_anchor,
                is_deactivate,
            })
        }
    }

    pub(super) fn expand_excerpt(
        &mut self,
        excerpt_anchor: Anchor,
        direction: ExpandExcerptDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let lines_to_expand = EditorSettings::get_global(cx).expand_excerpt_lines;

        if self.delegate_expand_excerpts {
            cx.emit(EditorEvent::ExpandExcerptsRequested {
                excerpt_anchors: vec![excerpt_anchor],
                lines: lines_to_expand,
                direction,
            });
            return;
        }

        let current_scroll_position = self.scroll_position(cx);
        let mut scroll = None;

        if direction == ExpandExcerptDirection::Down {
            let multi_buffer = self.buffer.read(cx);
            let snapshot = multi_buffer.snapshot(cx);
            if let Some((buffer_snapshot, excerpt_range)) =
                snapshot.excerpt_containing(excerpt_anchor..excerpt_anchor)
            {
                let excerpt_end_row =
                    Point::from_anchor(&excerpt_range.context.end, &buffer_snapshot).row;
                let last_row = buffer_snapshot.max_point().row;
                let lines_below = last_row.saturating_sub(excerpt_end_row);
                if lines_below >= lines_to_expand {
                    scroll = Some(
                        current_scroll_position
                            + gpui::Point::new(0.0, lines_to_expand as ScrollOffset),
                    );
                }
            }
        }
        if direction == ExpandExcerptDirection::Up
            && self
                .buffer
                .read(cx)
                .snapshot(cx)
                .excerpt_before(excerpt_anchor)
                .is_none()
        {
            scroll = Some(current_scroll_position);
        }

        self.buffer.update(cx, |buffer, cx| {
            buffer.expand_excerpts([excerpt_anchor], lines_to_expand, direction, cx)
        });

        if let Some(new_scroll_position) = scroll {
            self.set_scroll_position(new_scroll_position, window, cx);
        }
    }

    pub(super) fn go_to_next_change(
        &mut self,
        _: &GoToNextChange,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selections) = self
            .change_list
            .next_change(1, Direction::Next)
            .map(|s| s.to_vec())
        {
            self.change_selections(Default::default(), window, cx, |s| {
                let map = s.display_snapshot();
                s.select_display_ranges(selections.iter().map(|a| {
                    let point = a.to_display_point(&map);
                    point..point
                }))
            })
        }
    }

    pub(super) fn go_to_previous_change(
        &mut self,
        _: &GoToPreviousChange,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selections) = self
            .change_list
            .next_change(1, Direction::Prev)
            .map(|s| s.to_vec())
        {
            self.change_selections(Default::default(), window, cx, |s| {
                let map = s.display_snapshot();
                s.select_display_ranges(selections.iter().map(|a| {
                    let point = a.to_display_point(&map);
                    point..point
                }))
            })
        }
    }

    pub(super) fn go_to_line<T: 'static>(
        &mut self,
        position: Anchor,
        highlight_color: Option<Hsla>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.snapshot(window, cx).display_snapshot;
        let position = position.to_point(&snapshot.buffer_snapshot());
        let start = snapshot
            .buffer_snapshot()
            .clip_point(Point::new(position.row, 0), Bias::Left);
        let end = start + Point::new(1, 0);
        let start = snapshot.buffer_snapshot().anchor_before(start);
        let end = snapshot.buffer_snapshot().anchor_before(end);

        self.highlight_rows::<T>(
            start..end,
            highlight_color
                .unwrap_or_else(|| cx.theme().colors().editor_highlighted_line_background),
            Default::default(),
            cx,
        );

        if self.buffer.read(cx).is_singleton() {
            self.request_autoscroll(Autoscroll::center().for_anchor(start), cx);
        }
    }

    pub(super) fn navigate_to_hover_links(
        &mut self,
        kind: Option<GotoDefinitionKind>,
        definitions: Vec<HoverLink>,
        origin: Option<NavigationEntry>,
        split: bool,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Navigated>> {
        // Separate out url and file links, we can only handle one of them at most or an arbitrary number of locations
        let mut first_url_or_file = None;
        let definitions: Vec<_> = definitions
            .into_iter()
            .filter_map(|def| match def {
                HoverLink::Text(link) => Some(Task::ready(anyhow::Ok(Some(link.target)))),
                HoverLink::InlayHint(lsp_location, server_id) => {
                    let computation =
                        self.compute_target_location(lsp_location, server_id, window, cx);
                    Some(cx.background_spawn(computation))
                }
                HoverLink::Url(url) => {
                    first_url_or_file = Some(Either::Left(url));
                    None
                }
                HoverLink::File(file_target) => {
                    first_url_or_file = Some(Either::Right(file_target));
                    None
                }
            })
            .collect();

        let workspace = self.workspace();

        let excerpt_context_lines = multi_buffer::excerpt_context_lines(cx);
        cx.spawn_in(window, async move |editor, cx| {
            let locations: Vec<Location> = future::join_all(definitions)
                .await
                .into_iter()
                .filter_map(|location| location.transpose())
                .collect::<Result<_>>()
                .context("location tasks")?;
            let mut locations = cx.update(|_, cx| {
                locations
                    .into_iter()
                    .map(|location| {
                        let buffer = location.buffer.read(cx);
                        (location.buffer, location.range.to_point(buffer))
                    })
                    .into_group_map()
            })?;
            let mut num_locations = 0;
            for ranges in locations.values_mut() {
                ranges.sort_by_key(|range| (range.start, Reverse(range.end)));
                ranges.dedup();
                // Merge overlapping or contained ranges. After sorting by
                // (start, Reverse(end)), we can merge in a single pass:
                // if the next range starts before the current one ends,
                // extend the current range's end if needed.
                let mut i = 0;
                while i + 1 < ranges.len() {
                    if ranges[i + 1].start <= ranges[i].end {
                        let merged_end = ranges[i].end.max(ranges[i + 1].end);
                        ranges[i].end = merged_end;
                        ranges.remove(i + 1);
                    } else {
                        i += 1;
                    }
                }
                let fits_in_one_excerpt = ranges
                    .iter()
                    .tuple_windows()
                    .all(|(a, b)| b.start.row - a.end.row <= 2 * excerpt_context_lines);
                num_locations += if fits_in_one_excerpt { 1 } else { ranges.len() };
            }

            if num_locations > 1 {
                let tab_kind = match kind {
                    Some(GotoDefinitionKind::Implementation) => "Implementations",
                    Some(GotoDefinitionKind::Symbol) | None => "Definitions",
                    Some(GotoDefinitionKind::Declaration) => "Declarations",
                    Some(GotoDefinitionKind::Type) => "Types",
                };
                let title = editor
                    .update_in(cx, |_, _, cx| {
                        let target = locations
                            .iter()
                            .flat_map(|(k, v)| iter::repeat(k.clone()).zip(v))
                            .map(|(buffer, location)| {
                                buffer
                                    .read(cx)
                                    .text_for_range(location.clone())
                                    .collect::<String>()
                            })
                            .filter(|text| !text.contains('\n'))
                            .unique()
                            .take(3)
                            .join(", ");
                        if target.is_empty() {
                            tab_kind.to_owned()
                        } else {
                            format!("{tab_kind} for {target}")
                        }
                    })
                    .context("buffer title")?;

                let Some(workspace) = workspace else {
                    return Ok(Navigated::No);
                };

                let opened = workspace
                    .update_in(cx, |workspace, window, cx| {
                        let allow_preview = PreviewTabsSettings::get_global(cx)
                            .enable_preview_multibuffer_from_code_navigation;
                        if let Some((target_editor, target_pane)) =
                            Self::open_locations_in_multibuffer(
                                workspace,
                                locations,
                                title,
                                split,
                                allow_preview,
                                MultibufferSelectionMode::First,
                                window,
                                cx,
                            )
                        {
                            // We create our own nav history instead of using
                            // `target_editor.nav_history` because `nav_history`
                            // seems to be populated asynchronously when an item
                            // is added to a pane
                            let mut nav_history = target_pane
                                .update(cx, |pane, _| pane.nav_history_for_item(&target_editor));
                            target_editor.update(cx, |editor, cx| {
                                let nav_data = editor
                                    .navigation_data(editor.selections.newest_anchor().head(), cx);
                                let target =
                                    Some(nav_history.navigation_entry(Some(
                                        Arc::new(nav_data) as Arc<dyn Any + Send + Sync>
                                    )));
                                nav_history.push_tag(origin, target);
                            })
                        }
                    })
                    .is_ok();

                anyhow::Ok(Navigated::from_bool(opened))
            } else if num_locations == 0 {
                // If there is one url or file, open it directly
                match first_url_or_file {
                    Some(Either::Left(url)) => {
                        cx.update(|window, cx| {
                            if parse_zed_link(&url, cx).is_some() {
                                window
                                    .dispatch_action(Box::new(zed_actions::OpenZedUrl { url }), cx);
                            } else {
                                cx.open_url(&url);
                            }
                        })?;
                        Ok(Navigated::Yes)
                    }
                    Some(Either::Right(file_target)) => {
                        // TODO(andrew): respect preview tab settings
                        //               `enable_keep_preview_on_code_navigation` and
                        //               `enable_preview_file_from_code_navigation`
                        let Some(workspace) = workspace else {
                            return Ok(Navigated::No);
                        };
                        let item = workspace
                            .update_in(cx, |workspace, window, cx| {
                                workspace.open_resolved_path(
                                    file_target.resolved_path.clone(),
                                    window,
                                    cx,
                                )
                            })?
                            .await?;

                        file_target.navigate_item_to_position(item, cx);

                        Ok(Navigated::Yes)
                    }
                    None => Ok(Navigated::No),
                }
            } else {
                let Some((target_buffer, target_ranges)) = locations.into_iter().next() else {
                    return Ok(Navigated::No);
                };

                editor.update_in(cx, |editor, window, cx| {
                    let target_ranges = target_ranges
                        .into_iter()
                        .map(|r| editor.range_for_match(&r))
                        .map(collapse_multiline_range)
                        .collect::<Vec<_>>();
                    if !split
                        && Some(&target_buffer) == editor.buffer.read(cx).as_singleton().as_ref()
                    {
                        let multibuffer = editor.buffer.read(cx);
                        let target_ranges = target_ranges
                            .into_iter()
                            .filter_map(|r| {
                                let start = multibuffer.buffer_point_to_anchor(
                                    &target_buffer,
                                    r.start,
                                    cx,
                                )?;
                                let end = multibuffer.buffer_point_to_anchor(
                                    &target_buffer,
                                    r.end,
                                    cx,
                                )?;
                                Some(start..end)
                            })
                            .collect::<Vec<_>>();
                        if target_ranges.is_empty() {
                            return Navigated::No;
                        }

                        editor.change_selections(
                            SelectionEffects::scroll(Autoscroll::for_go_to_definition(
                                editor.cursor_top_offset(cx),
                                cx,
                            ))
                            .nav_history(true),
                            window,
                            cx,
                            |s| s.select_anchor_ranges(target_ranges),
                        );

                        let target =
                            editor.navigation_entry(editor.selections.newest_anchor().head(), cx);
                        if let Some(mut nav_history) = editor.nav_history.clone() {
                            nav_history.push_tag(origin, target);
                        }
                    } else {
                        let Some(workspace) = workspace else {
                            return Navigated::No;
                        };
                        let pane = workspace.read(cx).active_pane().clone();
                        let offset = editor.cursor_top_offset(cx);

                        window.defer(cx, move |window, cx| {
                            let (target_editor, target_pane): (Entity<Self>, Entity<Pane>) =
                                workspace.update(cx, |workspace, cx| {
                                    let pane = if split {
                                        workspace.adjacent_pane(window, cx)
                                    } else {
                                        workspace.active_pane().clone()
                                    };

                                    let preview_tabs_settings = PreviewTabsSettings::get_global(cx);
                                    let keep_old_preview = preview_tabs_settings
                                        .enable_keep_preview_on_code_navigation;
                                    let allow_new_preview = preview_tabs_settings
                                        .enable_preview_file_from_code_navigation;

                                    let editor = workspace.open_project_item(
                                        pane.clone(),
                                        target_buffer.clone(),
                                        true,
                                        true,
                                        keep_old_preview,
                                        allow_new_preview,
                                        window,
                                        cx,
                                    );
                                    (editor, pane)
                                });
                            // We create our own nav history instead of using
                            // `target_editor.nav_history` because `nav_history`
                            // seems to be populated asynchronously when an item
                            // is added to a pane
                            let mut nav_history = target_pane
                                .update(cx, |pane, _| pane.nav_history_for_item(&target_editor));
                            target_editor.update(cx, |target_editor, cx| {
                                // When selecting a definition in a different buffer, disable the nav history
                                // to avoid creating a history entry at the previous cursor location.
                                pane.update(cx, |pane, _| pane.disable_history());

                                let multibuffer = target_editor.buffer.read(cx);
                                let Some(target_buffer) = multibuffer.as_singleton() else {
                                    return Navigated::No;
                                };
                                let target_ranges = target_ranges
                                    .into_iter()
                                    .filter_map(|r| {
                                        let start = multibuffer.buffer_point_to_anchor(
                                            &target_buffer,
                                            r.start,
                                            cx,
                                        )?;
                                        let end = multibuffer.buffer_point_to_anchor(
                                            &target_buffer,
                                            r.end,
                                            cx,
                                        )?;
                                        Some(start..end)
                                    })
                                    .collect::<Vec<_>>();
                                if target_ranges.is_empty() {
                                    return Navigated::No;
                                }

                                target_editor.change_selections(
                                    SelectionEffects::scroll(Autoscroll::for_go_to_definition(
                                        offset, cx,
                                    ))
                                    .nav_history(true),
                                    window,
                                    cx,
                                    |s| s.select_anchor_ranges(target_ranges),
                                );

                                let nav_data = target_editor.navigation_data(
                                    target_editor.selections.newest_anchor().head(),
                                    cx,
                                );
                                let target =
                                    Some(nav_history.navigation_entry(Some(
                                        Arc::new(nav_data) as Arc<dyn Any + Send + Sync>
                                    )));
                                nav_history.push_tag(origin, target);
                                pane.update(cx, |pane, _| pane.enable_history());
                                Navigated::Yes
                            });
                        });
                    }
                    Navigated::Yes
                })
            }
        })
    }

    pub(super) fn go_to_next_reference(
        &mut self,
        _: &GoToNextReference,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let task = self.go_to_reference_before_or_after_position(Direction::Next, 1, window, cx);
        if let Some(task) = task {
            task.detach();
        };
    }

    pub(super) fn go_to_prev_reference(
        &mut self,
        _: &GoToPreviousReference,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let task = self.go_to_reference_before_or_after_position(Direction::Prev, 1, window, cx);
        if let Some(task) = task {
            task.detach();
        };
    }

    pub(super) fn go_to_symbol_by_offset(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        offset: i8,
    ) -> Task<Result<()>> {
        let editor_snapshot = self.snapshot(window, cx);

        // We don't care about multi-buffer symbols
        if !editor_snapshot.is_singleton() {
            return Task::ready(Ok(()));
        }

        let cursor_offset = self
            .selections
            .newest::<MultiBufferOffset>(&editor_snapshot.display_snapshot)
            .head();

        cx.spawn_in(window, async move |editor, wcx| -> Result<()> {
            let Ok(Some(remote_id)) = editor.update(wcx, |ed, cx| {
                let buffer = ed.buffer.read(cx).as_singleton()?;
                Some(buffer.read(cx).remote_id())
            }) else {
                return Ok(());
            };

            let task = editor.update(wcx, |ed, cx| ed.buffer_outline_items(remote_id, cx))?;
            let outline_items: Vec<OutlineItem<text::Anchor>> = task.await;

            let multi_snapshot = editor_snapshot.buffer();
            let buffer_range = |range: &Range<_>| {
                Some(
                    multi_snapshot
                        .buffer_anchor_range_to_anchor_range(range.clone())?
                        .to_offset(multi_snapshot),
                )
            };

            wcx.update_window(wcx.window_handle(), |_, window, acx| {
                let current_idx = outline_items
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, item)| {
                        // Find the closest outline item by distance between outline text and cursor location
                        let source_range = buffer_range(&item.source_range_for_text)?;
                        let distance_to_closest_endpoint = cmp::min(
                            (source_range.start.0 as isize - cursor_offset.0 as isize).abs(),
                            (source_range.end.0 as isize - cursor_offset.0 as isize).abs(),
                        );

                        let item_towards_offset =
                            (source_range.start.0 as isize - cursor_offset.0 as isize).signum()
                                == (offset as isize).signum();

                        let source_range_contains_cursor = source_range.contains(&cursor_offset);

                        // To pick the next outline to jump to, we should jump in the direction of the offset, and
                        // we should not already be within the outline's source range. We then pick the closest outline
                        // item.
                        (item_towards_offset && !source_range_contains_cursor)
                            .then_some((distance_to_closest_endpoint, idx))
                    })
                    .min()
                    .map(|(_, idx)| idx);

                let Some(idx) = current_idx else {
                    return;
                };

                let Some(range) = buffer_range(&outline_items[idx].source_range_for_text) else {
                    return;
                };
                let selection = [range.start..range.start];

                editor
                    .update(acx, |editor, ecx| {
                        editor.change_selections(
                            SelectionEffects::scroll(Autoscroll::newest()),
                            window,
                            ecx,
                            |s| s.select_ranges(selection),
                        );
                    })
                    .log_err();
            })?;

            Ok(())
        })
    }

    pub(super) fn go_to_next_symbol(
        &mut self,
        _: &GoToNextSymbol,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_symbol_by_offset(window, cx, 1).detach();
    }

    pub(super) fn go_to_previous_symbol(
        &mut self,
        _: &GoToPreviousSymbol,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_symbol_by_offset(window, cx, -1).detach();
    }

    /// Opens a multibuffer with the given project locations in it.
    pub(super) fn open_locations_in_multibuffer(
        workspace: &mut Workspace,
        locations: std::collections::HashMap<Entity<Buffer>, Vec<Range<Point>>>,
        title: String,
        split: bool,
        allow_preview: bool,
        multibuffer_selection_mode: MultibufferSelectionMode,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<(Entity<Editor>, Entity<Pane>)> {
        if locations.is_empty() {
            log::error!("bug: open_locations_in_multibuffer called with empty list of locations");
            return None;
        }

        let capability = workspace.project().read(cx).capability();
        let mut ranges = <Vec<Range<Anchor>>>::new();

        // a key to find existing multibuffer editors with the same set of locations
        // to prevent us from opening more and more multibuffer tabs for searches and the like
        let mut key = (title.clone(), vec![]);
        let excerpt_buffer = cx.new(|cx| {
            let key = &mut key.1;
            let mut multibuffer = MultiBuffer::new(capability);
            for (buffer, mut ranges_for_buffer) in locations {
                ranges_for_buffer.sort_by_key(|range| (range.start, Reverse(range.end)));
                key.push((buffer.read(cx).remote_id(), ranges_for_buffer.clone()));
                multibuffer.set_excerpts_for_path(
                    PathKey::for_buffer(&buffer, cx),
                    buffer.clone(),
                    ranges_for_buffer.clone(),
                    multibuffer_context_lines(cx),
                    cx,
                );
                let snapshot = multibuffer.snapshot(cx);
                let buffer_snapshot = buffer.read(cx).snapshot();
                ranges.extend(ranges_for_buffer.into_iter().filter_map(|range| {
                    let text_range = buffer_snapshot.anchor_range_inside(range);
                    let start = snapshot.anchor_in_buffer(text_range.start)?;
                    let end = snapshot.anchor_in_buffer(text_range.end)?;
                    Some(start..end)
                }))
            }

            multibuffer.with_title(title)
        });
        let existing = workspace.active_pane().update(cx, |pane, cx| {
            pane.items()
                .filter_map(|item| item.downcast::<Editor>())
                .find(|editor| {
                    editor
                        .read(cx)
                        .lookup_key
                        .as_ref()
                        .and_then(|it| {
                            it.downcast_ref::<(String, Vec<(BufferId, Vec<Range<Point>>)>)>()
                        })
                        .is_some_and(|it| *it == key)
                })
        });
        let was_existing = existing.is_some();
        let editor = existing.unwrap_or_else(|| {
            cx.new(|cx| {
                let mut editor = Editor::for_multibuffer(
                    excerpt_buffer,
                    Some(workspace.project().clone()),
                    window,
                    cx,
                );
                editor.lookup_key = Some(Box::new(key));
                editor
            })
        });
        editor.update(cx, |editor, cx| match multibuffer_selection_mode {
            MultibufferSelectionMode::First => {
                if let Some(first_range) = ranges.first() {
                    editor.change_selections(
                        SelectionEffects::no_scroll(),
                        window,
                        cx,
                        |selections| {
                            selections.clear_disjoint();
                            selections.select_anchor_ranges(std::iter::once(first_range.clone()));
                        },
                    );
                }
                editor.highlight_background(
                    HighlightKey::Editor,
                    &ranges,
                    |_, theme| theme.colors().editor_highlighted_line_background,
                    cx,
                );
            }
            MultibufferSelectionMode::All => {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.clear_disjoint();
                    selections.select_anchor_ranges(ranges);
                });
            }
        });

        let item = Box::new(editor.clone());

        let pane = if split {
            workspace.adjacent_pane(window, cx)
        } else {
            workspace.active_pane().clone()
        };
        let activate_pane = split;

        let mut destination_index = None;
        pane.update(cx, |pane, cx| {
            if allow_preview && !was_existing {
                destination_index = pane.replace_preview_item_id(item.item_id(), window, cx);
            }
            if was_existing && !allow_preview {
                pane.unpreview_item_if_preview(item.item_id());
            }
            pane.add_item(item, activate_pane, true, destination_index, window, cx);
        });

        Some((editor, pane))
    }

    fn navigation_data(&self, cursor_anchor: Anchor, cx: &mut Context<Self>) -> NavigationData {
        let display_snapshot = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).read(cx);
        let cursor_position = cursor_anchor.to_point(&buffer);
        let scroll_anchor = self.scroll_manager.native_anchor(&display_snapshot, cx);
        let scroll_top_row = scroll_anchor.top_row(&buffer);
        drop(buffer);

        NavigationData {
            cursor_anchor,
            cursor_position,
            scroll_anchor,
            scroll_top_row,
        }
    }

    fn expand_excerpts_for_direction(
        &mut self,
        lines: u32,
        direction: ExpandExcerptDirection,
        cx: &mut Context<Self>,
    ) {
        let selections = self.selections.disjoint_anchors_arc();

        let lines = if lines == 0 {
            EditorSettings::get_global(cx).expand_excerpt_lines
        } else {
            lines
        };

        let snapshot = self.buffer.read(cx).snapshot(cx);
        let excerpt_anchors = selections
            .iter()
            .flat_map(|selection| {
                snapshot
                    .range_to_buffer_ranges(selection.range())
                    .into_iter()
                    .filter_map(|(buffer_snapshot, range, _)| {
                        snapshot.anchor_in_excerpt(buffer_snapshot.anchor_after(range.start))
                    })
            })
            .collect::<Vec<_>>();

        if self.delegate_expand_excerpts {
            cx.emit(EditorEvent::ExpandExcerptsRequested {
                excerpt_anchors,
                lines,
                direction,
            });
            return;
        }

        self.buffer.update(cx, |buffer, cx| {
            buffer.expand_excerpts(excerpt_anchors, lines, direction, cx)
        })
    }

    fn go_to_definition_of_kind(
        &mut self,
        kind: GotoDefinitionKind,
        split: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Navigated>> {
        let Some(provider) = self.semantics_provider.clone() else {
            return Task::ready(Ok(Navigated::No));
        };
        let head = self
            .selections
            .newest::<MultiBufferOffset>(&self.display_snapshot(cx))
            .head();
        let buffer = self.buffer.read(cx);
        let Some((buffer, head)) = buffer.text_anchor_for_position(head, cx) else {
            return Task::ready(Ok(Navigated::No));
        };
        let Some(definitions) = provider.definitions(&buffer, head, kind, cx) else {
            return Task::ready(Ok(Navigated::No));
        };

        let nav_entry = self.navigation_entry(self.selections.newest_anchor().head(), cx);

        cx.spawn_in(window, async move |editor, cx| {
            let Some(definitions) = definitions.await? else {
                return Ok(Navigated::No);
            };
            let navigated = editor
                .update_in(cx, |editor, window, cx| {
                    editor.navigate_to_hover_links(
                        Some(kind),
                        definitions
                            .into_iter()
                            .filter(|location| {
                                hover_links::exclude_link_to_position(&buffer, &head, location, cx)
                            })
                            .map(HoverLink::Text)
                            .collect::<Vec<_>>(),
                        nav_entry,
                        split,
                        window,
                        cx,
                    )
                })?
                .await?;
            anyhow::Ok(navigated)
        })
    }

    fn compute_target_location(
        &self,
        lsp_location: lsp::Location,
        server_id: LanguageServerId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Option<Location>>> {
        let Some(project) = self.project.clone() else {
            return Task::ready(Ok(None));
        };

        cx.spawn_in(window, async move |editor, cx| {
            let location_task = editor.update(cx, |_, cx| {
                project.update(cx, |project, cx| {
                    project.open_local_buffer_via_lsp(lsp_location.uri.clone(), server_id, cx)
                })
            })?;
            let location = Some({
                let target_buffer_handle = location_task.await.context("open local buffer")?;
                let range = target_buffer_handle.read_with(cx, |target_buffer, _| {
                    let target_start = target_buffer
                        .clip_point_utf16(point_from_lsp(lsp_location.range.start), Bias::Left);
                    let target_end = target_buffer
                        .clip_point_utf16(point_from_lsp(lsp_location.range.end), Bias::Left);
                    target_buffer.anchor_after(target_start)
                        ..target_buffer.anchor_before(target_end)
                });
                Location {
                    buffer: target_buffer_handle,
                    range,
                }
            });
            Ok(location)
        })
    }

    fn go_to_singleton_buffer_range_impl(
        &mut self,
        range: Range<Point>,
        record_nav_history: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let multibuffer = self.buffer().read(cx);
        if !multibuffer.is_singleton() {
            return;
        };
        let anchor_range = range.to_anchors(&multibuffer.snapshot(cx));
        self.change_selections(
            SelectionEffects::scroll(Autoscroll::for_go_to_definition(
                self.cursor_top_offset(cx),
                cx,
            ))
            .nav_history(record_nav_history),
            window,
            cx,
            |s| s.select_anchor_ranges([anchor_range]),
        );
    }

    fn go_to_document_highlight_before_or_after_position(
        &mut self,
        direction: Direction,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let snapshot = self.snapshot(window, cx);
        let buffer = &snapshot.buffer_snapshot();
        let position = self
            .selections
            .newest::<Point>(&snapshot.display_snapshot)
            .head();
        let anchor_position = buffer.anchor_after(position);

        // Get all document highlights (both read and write)
        let mut all_highlights = Vec::new();

        if let Some((_, read_highlights)) = self
            .background_highlights
            .get(&HighlightKey::DocumentHighlightRead)
        {
            all_highlights.extend(read_highlights.iter());
        }

        if let Some((_, write_highlights)) = self
            .background_highlights
            .get(&HighlightKey::DocumentHighlightWrite)
        {
            all_highlights.extend(write_highlights.iter());
        }

        if all_highlights.is_empty() {
            return;
        }

        // Sort highlights by position
        all_highlights.sort_by(|a, b| a.start.cmp(&b.start, buffer));

        let target_highlight = match direction {
            Direction::Next => {
                // Find the first highlight after the current position
                all_highlights
                    .iter()
                    .find(|highlight| highlight.start.cmp(&anchor_position, buffer).is_gt())
            }
            Direction::Prev => {
                // Find the last highlight before the current position
                all_highlights
                    .iter()
                    .rev()
                    .find(|highlight| highlight.end.cmp(&anchor_position, buffer).is_lt())
            }
        };

        if let Some(highlight) = target_highlight {
            let destination = highlight.start.to_point(buffer);
            let autoscroll = Autoscroll::center();

            self.unfold_ranges(&[destination..destination], false, false, cx);
            self.change_selections(SelectionEffects::scroll(autoscroll), window, cx, |s| {
                s.select_ranges([destination..destination]);
            });
        }
    }
}
